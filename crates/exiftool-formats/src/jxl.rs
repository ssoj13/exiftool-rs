//! JPEG XL format parser.
//!
//! JPEG XL has two container types:
//! - Codestream: raw bitstream starting with 0xFF 0x0A
//! - Container: ISOBMFF-based format with boxes
//!
//! Container box types:
//! - `JXL ` (0x4A584C20): File type box (similar to ftyp)
//! - `jxlc`: Codestream box
//! - `jxlp`: Partial codestream box
//! - `Exif`: EXIF metadata
//! - `xml `: XMP metadata
//! - `jumb`: JUMBF metadata
//! - `brob`: Brotli-compressed box

use crate::tag_lookup::{lookup_exif_subifd, lookup_gps, lookup_ifd0};
use crate::{entry_to_attr, makernotes, Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use exiftool_core::{ByteOrder, IfdReader, RawValue};
use std::io::SeekFrom;

/// JPEG XL container magic (12 bytes).
const JXL_CONTAINER_MAGIC: &[u8] = &[
    0x00, 0x00, 0x00, 0x0C, // Box size (12)
    0x4A, 0x58, 0x4C, 0x20, // "JXL "
    0x0D, 0x0A, 0x87, 0x0A, // Signature
];

/// JPEG XL codestream signature (2 bytes).
const JXL_CODESTREAM_MAGIC: &[u8] = &[0xFF, 0x0A];

/// JPEG XL parser.
pub struct JxlParser;

impl FormatParser for JxlParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // Container format
        if header.len() >= 12 && header[..12] == *JXL_CONTAINER_MAGIC {
            return true;
        }
        // Raw codestream
        if header.len() >= 2 && header[..2] == *JXL_CODESTREAM_MAGIC {
            return true;
        }
        false
    }

    fn format_name(&self) -> &'static str {
        "JXL"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["jxl"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        // Read enough for container magic, but handle smaller files
        let mut header = [0u8; 12];
        let bytes_read = reader.read(&mut header)?;
        reader.seek(SeekFrom::Start(0))?;

        let mut metadata = Metadata::new("JXL");
        metadata.set_file_type("JXL", "image/jxl");

        if bytes_read >= 12 && header[..12] == *JXL_CONTAINER_MAGIC {
            metadata.exif.set("JXL:ContainerFormat", AttrValue::Str("ISOBMFF".to_string()));
            self.parse_container(reader, &mut metadata)?;
        } else if bytes_read >= 2 && header[..2] == *JXL_CODESTREAM_MAGIC {
            metadata.exif.set("JXL:ContainerFormat", AttrValue::Str("Codestream".to_string()));
            self.parse_codestream(reader, &mut metadata)?;
        } else {
            return Err(Error::InvalidStructure("Invalid JXL signature".into()));
        }

        Ok(metadata)
    }
}

impl JxlParser {
    /// Parse ISOBMFF-based container format.
    fn parse_container(&self, reader: &mut dyn ReadSeek, metadata: &mut Metadata) -> Result<()> {
        reader.seek(SeekFrom::Start(12))?; // Skip file type box

        let file_size = crate::utils::get_file_size(reader)?;
        reader.seek(SeekFrom::Start(12))?;

        while reader.stream_position()? < file_size {
            let box_start = reader.stream_position()?;
            
            // Read box header
            let mut size_buf = [0u8; 4];
            if reader.read_exact(&mut size_buf).is_err() {
                break;
            }
            let mut box_size = u32::from_be_bytes(size_buf) as u64;

            let mut type_buf = [0u8; 4];
            if reader.read_exact(&mut type_buf).is_err() {
                break;
            }
            let box_type = String::from_utf8_lossy(&type_buf).to_string();

            // Handle extended size
            if box_size == 1 {
                let mut ext_size = [0u8; 8];
                reader.read_exact(&mut ext_size)?;
                box_size = u64::from_be_bytes(ext_size);
            } else if box_size == 0 {
                // Box extends to EOF
                box_size = file_size - box_start;
            }

            let header_size = if box_size == 1 { 16 } else { 8 };
            let data_size = box_size.saturating_sub(header_size);

            match box_type.as_str() {
                "jxlc" => {
                    // Codestream box - parse for image info
                    let codestream_start = reader.stream_position()?;
                    self.parse_codestream(reader, metadata)?;
                    reader.seek(SeekFrom::Start(codestream_start + data_size))?;
                }
                "jxlp" => {
                    // Partial codestream - check if first part (index 0)
                    let mut index_buf = [0u8; 4];
                    reader.read_exact(&mut index_buf)?;
                    let index = u32::from_be_bytes(index_buf);
                    if index == 0 || index == 0x80000000 {
                        // First or only partial - parse codestream header
                        self.parse_codestream(reader, metadata)?;
                    }
                    reader.seek(SeekFrom::Start(box_start + box_size))?;
                }
                "Exif" => {
                    self.parse_exif_box(reader, data_size, metadata)?;
                }
                "xml " => {
                    self.parse_xmp_box(reader, data_size, metadata)?;
                }
                "jumb" => {
                    metadata.exif.set("JXL:HasJUMBF", AttrValue::Bool(true));
                    reader.seek(SeekFrom::Start(box_start + box_size))?;
                }
                "brob" => {
                    // Brotli compressed box - note its presence
                    metadata.exif.set("JXL:HasBrotliBox", AttrValue::Bool(true));
                    reader.seek(SeekFrom::Start(box_start + box_size))?;
                }
                _ => {
                    reader.seek(SeekFrom::Start(box_start + box_size))?;
                }
            }

            // Prevent infinite loop
            if reader.stream_position()? <= box_start {
                break;
            }
        }

        Ok(())
    }

    /// Parse EXIF box.
    fn parse_exif_box(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if !(8..=10 * 1024 * 1024).contains(&size) {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(());
        }

        // JXL EXIF box: 4-byte offset to TIFF header, then TIFF data
        let mut offset_buf = [0u8; 4];
        reader.read_exact(&mut offset_buf)?;
        let tiff_offset = u32::from_be_bytes(offset_buf) as usize;

        let data_size = (size - 4) as usize;
        let mut exif_data = vec![0u8; data_size];
        reader.read_exact(&mut exif_data)?;

        // Find TIFF header
        let tiff_start = if tiff_offset < data_size {
            tiff_offset
        } else {
            self.find_tiff_header(&exif_data)
        };

        if tiff_start >= exif_data.len() || exif_data.len() - tiff_start < 8 {
            return Ok(());
        }

        let tiff_data = &exif_data[tiff_start..];
        self.parse_tiff_exif(tiff_data, metadata)?;

        Ok(())
    }

    /// Parse XMP box.
    fn parse_xmp_box(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if size > 10 * 1024 * 1024 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(());
        }

        let mut xmp_data = vec![0u8; size as usize];
        reader.read_exact(&mut xmp_data)?;

        if let Ok(xmp_str) = std::str::from_utf8(&xmp_data) {
            if let Ok(xmp_attrs) = exiftool_xmp::XmpParser::parse(xmp_str) {
                for (key, value) in xmp_attrs.iter() {
                    metadata.exif.set(format!("XMP:{}", key), value.clone());
                }
            }
            metadata.xmp = Some(xmp_str.to_string());
        }

        Ok(())
    }

    /// Find TIFF header in EXIF data.
    fn find_tiff_header(&self, data: &[u8]) -> usize {
        for i in 0..data.len().saturating_sub(4) {
            let marker = &data[i..i + 2];
            if (marker == b"II" || marker == b"MM") && data.len() > i + 3 {
                let magic = if marker == b"MM" {
                    u16::from_be_bytes([data[i + 2], data[i + 3]])
                } else {
                    u16::from_le_bytes([data[i + 2], data[i + 3]])
                };
                if magic == 42 {
                    return i;
                }
            }
        }
        0
    }

    /// Parse TIFF-format EXIF data.
    fn parse_tiff_exif(&self, tiff_data: &[u8], metadata: &mut Metadata) -> Result<()> {
        let byte_order = match ByteOrder::from_marker([tiff_data[0], tiff_data[1]]) {
            Ok(bo) => bo,
            Err(_) => return Ok(()),
        };

        let reader = IfdReader::new(tiff_data, byte_order);
        let ifd0_offset = match reader.parse_header() {
            Ok(o) => o,
            Err(_) => return Ok(()),
        };

        let (entries, _next_ifd) = match reader.read_ifd(ifd0_offset) {
            Ok(e) => e,
            Err(_) => return Ok(()),
        };

        // Find Make for MakerNotes vendor detection
        let mut vendor = makernotes::Vendor::Unknown;
        for entry in &entries {
            if entry.tag == 0x010F {
                if let RawValue::String(make) = &entry.value {
                    vendor = makernotes::Vendor::from_make(make);
                }
                break;
            }
        }

        // Convert IFD0 entries
        for entry in &entries {
            if let Some(name) = lookup_ifd0(entry.tag) {
                metadata.exif.set(name, entry_to_attr(entry));
            }

            match entry.tag {
                0x8769 => {
                    // ExifIFD pointer
                    if let Some(offset) = entry.value.as_u32() {
                        if let Ok((exif_entries, _)) = reader.read_ifd(offset) {
                            for e in &exif_entries {
                                if e.tag == 0x927C {
                                    // MakerNotes
                                    if let RawValue::Undefined(bytes) = &e.value {
                                        if let Some(mn_data) = makernotes::parse(bytes, vendor, byte_order) {
                                            for (key, val) in mn_data.iter() {
                                                metadata.exif.set(key.clone(), val.clone());
                                            }
                                        }
                                    }
                                } else if let Some(name) = lookup_exif_subifd(e.tag) {
                                    metadata.exif.set(name, entry_to_attr(e));
                                }
                            }
                        }
                    }
                }
                0x8825 => {
                    // GPS IFD pointer
                    if let Some(offset) = entry.value.as_u32() {
                        if let Ok((gps_entries, _)) = reader.read_ifd(offset) {
                            for e in &gps_entries {
                                if let Some(name) = lookup_gps(e.tag) {
                                    metadata.exif.set(name, entry_to_attr(e));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Parse raw codestream header for basic image info.
    fn parse_codestream(&self, reader: &mut dyn ReadSeek, metadata: &mut Metadata) -> Result<()> {
        let mut sig = [0u8; 2];
        if reader.read_exact(&mut sig).is_err() {
            return Ok(());
        }

        if sig != *JXL_CODESTREAM_MAGIC {
            return Ok(()); // Not a codestream, skip
        }

        // Parse SizeHeader (variable length) - best effort
        let mut first = [0u8; 1];
        if reader.read_exact(&mut first).is_err() {
            return Ok(());
        }

        let small = (first[0] & 0x01) != 0;
        
        if small {
            // Small image: ratio and height/width bits encoded
            let ratio = (first[0] >> 1) & 0x07;
            let height_bits = (first[0] >> 4) & 0x0F;
            
            let mut second = [0u8; 1];
            if reader.read_exact(&mut second).is_err() {
                return Ok(());
            }
            let width_bits = ((second[0] as u16) << 4) | ((first[0] >> 4) as u16 & 0x0F);
            
            let (width, height) = self.decode_small_size(ratio, height_bits as u32, width_bits as u32);
            
            if width > 0 && height > 0 {
                metadata.exif.set("File:ImageWidth", AttrValue::Int(width as i32));
                metadata.exif.set("File:ImageHeight", AttrValue::Int(height as i32));
            }
        }
        // For large images, we'd need more complex VarInt parsing - skip for now

        Ok(())
    }

    /// Decode small image dimensions from ratio and bits.
    fn decode_small_size(&self, ratio: u8, h_bits: u32, w_bits: u32) -> (u32, u32) {
        let height = (h_bits + 1) * 8;
        let width = match ratio {
            0 => (w_bits + 1) * 8,
            1 => height,           // 1:1
            2 => height * 12 / 10, // 1.2:1
            3 => height * 4 / 3,   // 4:3
            4 => height * 3 / 2,   // 3:2
            5 => height * 16 / 9,  // 16:9
            6 => height * 5 / 4,   // 5:4
            7 => height * 2,       // 2:1
            _ => height,
        };
        (width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_can_parse_container() {
        let parser = JxlParser;
        let mut header = vec![0u8; 16];
        header[..12].copy_from_slice(JXL_CONTAINER_MAGIC);
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn test_can_parse_codestream() {
        let parser = JxlParser;
        assert!(parser.can_parse(&[0xFF, 0x0A, 0x00, 0x00]));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = JxlParser;
        assert!(!parser.can_parse(b"RIFF"));
        assert!(!parser.can_parse(b"\x89PNG"));
        assert!(!parser.can_parse(&[]));
    }

    #[test]
    fn test_parse_container_minimal() {
        let mut data = Vec::new();
        // File type box (12 bytes)
        data.extend_from_slice(JXL_CONTAINER_MAGIC);
        // jxlc box with minimal codestream
        data.extend_from_slice(&[0, 0, 0, 14]); // size=14 (8 header + 6 data)
        data.extend_from_slice(b"jxlc");
        data.extend_from_slice(&[0xFF, 0x0A]); // codestream signature
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // padding

        let parser = JxlParser;
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("JXL:ContainerFormat"), Some("ISOBMFF"));
    }

    #[test]
    fn test_parse_codestream_minimal() {
        // Minimal codestream: signature + size header (enough bytes)
        let data = vec![0xFF, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let parser = JxlParser;
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("JXL:ContainerFormat"), Some("Codestream"));
    }
}

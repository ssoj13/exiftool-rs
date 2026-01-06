//! JPEG 2000 format parser.
//!
//! Supports JP2 container format and raw codestream:
//! - .jp2: JPEG 2000 Part 1 (JP2 file format with boxes)
//! - .jpx: JPEG 2000 Part 2 (extended)
//! - .j2k/.jpc/.j2c: Raw JPEG 2000 codestream
//!
//! JP2 Box types:
//! - `jP  `: Signature box (must be first)
//! - `ftyp`: File type box
//! - `jp2h`: JP2 header box (contains ihdr, colr)
//! - `ihdr`: Image header (dimensions, components)
//! - `colr`: Color specification
//! - `jp2c`: Codestream (actual image data)
//! - `uuid`: UUID box (can contain XMP)
//! - `xml `: XML box (XMP metadata)

use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// JP2 signature box content.
const JP2_SIGNATURE: &[u8] = &[0x0D, 0x0A, 0x87, 0x0A];

/// Raw codestream marker (SOC - Start of Codestream).
const J2K_SOC_MARKER: &[u8] = &[0xFF, 0x4F];

/// XMP UUID for JPEG 2000.
const XMP_UUID: &[u8] = &[
    0xBE, 0x7A, 0xCF, 0xCB, 0x97, 0xA9, 0x42, 0xE8,
    0x9C, 0x71, 0x99, 0x94, 0x91, 0xE3, 0xAF, 0xAC,
];

/// JPEG 2000 parser.
pub struct Jp2Parser;

impl FormatParser for Jp2Parser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // JP2 container: starts with signature box "jP  " followed by signature
        if header.len() >= 12 {
            // Check for JP2 signature box
            if &header[0..4] == &[0x00, 0x00, 0x00, 0x0C]  // size = 12
                && &header[4..8] == b"jP  "
                && &header[8..12] == JP2_SIGNATURE {
                return true;
            }
        }
        // Raw codestream
        if header.len() >= 2 && &header[0..2] == J2K_SOC_MARKER {
            return true;
        }
        false
    }

    fn format_name(&self) -> &'static str {
        "JP2"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["jp2", "jpx", "jpf", "j2k", "jpc", "j2c"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut header = [0u8; 12];
        let bytes_read = reader.read(&mut header)?;
        reader.seek(SeekFrom::Start(0))?;

        let mut metadata = Metadata::new("JP2");
        metadata.exif.set("File:MIMEType", AttrValue::Str("image/jp2".to_string()));

        // Check format type
        if bytes_read >= 12
            && &header[0..4] == &[0x00, 0x00, 0x00, 0x0C]
            && &header[4..8] == b"jP  "
            && &header[8..12] == JP2_SIGNATURE
        {
            metadata.exif.set("File:FileType", AttrValue::Str("JP2".to_string()));
            self.parse_jp2_container(reader, &mut metadata)?;
        } else if bytes_read >= 2 && &header[0..2] == J2K_SOC_MARKER {
            metadata.exif.set("File:FileType", AttrValue::Str("J2K".to_string()));
            metadata.exif.set("File:MIMEType", AttrValue::Str("image/j2c".to_string()));
            self.parse_codestream(reader, &mut metadata)?;
        } else {
            return Err(Error::InvalidStructure("Invalid JPEG 2000 signature".into()));
        }

        Ok(metadata)
    }
}

impl Jp2Parser {
    /// Parse JP2 container format.
    fn parse_jp2_container(&self, reader: &mut dyn ReadSeek, metadata: &mut Metadata) -> Result<()> {
        let file_size = reader.seek(SeekFrom::End(0))?;
        reader.seek(SeekFrom::Start(0))?;

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

            // Handle extended size (box_size == 1)
            let header_size = if box_size == 1 {
                let mut ext_size = [0u8; 8];
                reader.read_exact(&mut ext_size)?;
                box_size = u64::from_be_bytes(ext_size);
                16
            } else if box_size == 0 {
                box_size = file_size - box_start;
                8
            } else {
                8
            };

            let data_size = box_size.saturating_sub(header_size);

            match box_type.as_str() {
                "ftyp" => {
                    self.parse_ftyp(reader, data_size, metadata)?;
                }
                "jp2h" => {
                    // JP2 header - contains sub-boxes
                    self.parse_jp2h(reader, data_size, metadata)?;
                }
                "jp2c" => {
                    // Codestream - parse for image dimensions
                    let codestream_start = reader.stream_position()?;
                    self.parse_codestream(reader, metadata)?;
                    reader.seek(SeekFrom::Start(codestream_start + data_size))?;
                }
                "uuid" => {
                    self.parse_uuid(reader, data_size, metadata)?;
                }
                "xml " => {
                    self.parse_xml(reader, data_size, metadata)?;
                }
                _ => {
                    reader.seek(SeekFrom::Start(box_start + box_size))?;
                }
            }

            // Ensure forward progress
            if reader.stream_position()? <= box_start {
                reader.seek(SeekFrom::Start(box_start + box_size.max(8)))?;
            }
        }

        Ok(())
    }

    /// Parse file type box (ftyp).
    fn parse_ftyp(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if size < 4 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(());
        }

        let mut brand = [0u8; 4];
        reader.read_exact(&mut brand)?;
        let brand_str = String::from_utf8_lossy(&brand).trim().to_string();

        metadata.exif.set("JP2:Brand", AttrValue::Str(brand_str.clone()));

        // Update format based on brand
        match brand_str.as_str() {
            "jp2 " | "jp2" => {
                metadata.format = "JP2";
            }
            "jpx " | "jpx" => {
                metadata.format = "JPX";
                metadata.exif.set("File:FileType", AttrValue::Str("JPX".to_string()));
            }
            _ => {}
        }

        // Minor version
        if size >= 8 {
            let mut minor = [0u8; 4];
            reader.read_exact(&mut minor)?;
            let minor_version = u32::from_be_bytes(minor);
            metadata.exif.set("JP2:MinorVersion", AttrValue::UInt(minor_version));
        }

        // Skip remaining (compatibility list)
        let remaining = size.saturating_sub(8);
        if remaining > 0 {
            reader.seek(SeekFrom::Current(remaining as i64))?;
        }

        Ok(())
    }

    /// Parse JP2 header super-box (jp2h).
    fn parse_jp2h(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        let end_pos = reader.stream_position()? + size;

        while reader.stream_position()? < end_pos {
            let sub_start = reader.stream_position()?;

            let mut sub_size_buf = [0u8; 4];
            if reader.read_exact(&mut sub_size_buf).is_err() {
                break;
            }
            let sub_size = u32::from_be_bytes(sub_size_buf) as u64;

            let mut sub_type = [0u8; 4];
            if reader.read_exact(&mut sub_type).is_err() {
                break;
            }

            let data_size = sub_size.saturating_sub(8);

            match &sub_type {
                b"ihdr" => {
                    self.parse_ihdr(reader, data_size, metadata)?;
                }
                b"colr" => {
                    self.parse_colr(reader, data_size, metadata)?;
                }
                b"bpcc" => {
                    // Bits per component
                    metadata.exif.set("JP2:HasBPCC", AttrValue::Bool(true));
                    reader.seek(SeekFrom::Current(data_size as i64))?;
                }
                b"pclr" => {
                    // Palette
                    metadata.exif.set("JP2:HasPalette", AttrValue::Bool(true));
                    reader.seek(SeekFrom::Current(data_size as i64))?;
                }
                b"res " => {
                    // Resolution
                    self.parse_res(reader, data_size, metadata)?;
                }
                _ => {
                    reader.seek(SeekFrom::Current(data_size as i64))?;
                }
            }

            if reader.stream_position()? <= sub_start {
                break;
            }
        }

        Ok(())
    }

    /// Parse image header box (ihdr).
    fn parse_ihdr(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if size < 14 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(());
        }

        let mut ihdr = [0u8; 14];
        reader.read_exact(&mut ihdr)?;

        let height = u32::from_be_bytes([ihdr[0], ihdr[1], ihdr[2], ihdr[3]]);
        let width = u32::from_be_bytes([ihdr[4], ihdr[5], ihdr[6], ihdr[7]]);
        let num_components = u16::from_be_bytes([ihdr[8], ihdr[9]]);
        let bits_per_component = ihdr[10];
        let compression_type = ihdr[11];
        let colorspace_unknown = ihdr[12];
        let ipr = ihdr[13]; // Intellectual property

        metadata.exif.set("File:ImageWidth", AttrValue::UInt(width));
        metadata.exif.set("File:ImageHeight", AttrValue::UInt(height));
        metadata.exif.set("JP2:NumComponents", AttrValue::UInt(num_components as u32));
        metadata.exif.set("JP2:BitsPerComponent", AttrValue::UInt(bits_per_component as u32));

        let compression = match compression_type {
            7 => "JPEG 2000",
            _ => "Unknown",
        };
        metadata.exif.set("JP2:Compression", AttrValue::Str(compression.to_string()));

        if colorspace_unknown == 1 {
            metadata.exif.set("JP2:ColorspaceUnknown", AttrValue::Bool(true));
        }
        if ipr == 1 {
            metadata.exif.set("JP2:IPR", AttrValue::Bool(true));
        }

        // Skip remaining
        let remaining = size.saturating_sub(14);
        if remaining > 0 {
            reader.seek(SeekFrom::Current(remaining as i64))?;
        }

        Ok(())
    }

    /// Parse color specification box (colr).
    fn parse_colr(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if size < 3 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(());
        }

        let mut colr = [0u8; 3];
        reader.read_exact(&mut colr)?;

        let method = colr[0];
        let _precedence = colr[1];
        let _approx = colr[2];

        match method {
            1 => {
                // Enumerated colorspace
                if size >= 7 {
                    let mut cs = [0u8; 4];
                    reader.read_exact(&mut cs)?;
                    let colorspace = u32::from_be_bytes(cs);

                    let cs_name = match colorspace {
                        16 => "sRGB",
                        17 => "Greyscale",
                        18 => "sYCC",
                        _ => "Unknown",
                    };
                    metadata.exif.set("JP2:ColorSpace", AttrValue::Str(cs_name.to_string()));

                    let remaining = size.saturating_sub(7);
                    if remaining > 0 {
                        reader.seek(SeekFrom::Current(remaining as i64))?;
                    }
                } else {
                    reader.seek(SeekFrom::Current((size - 3) as i64))?;
                }
            }
            2 => {
                // Restricted ICC profile
                metadata.exif.set("JP2:ColorMethod", AttrValue::Str("RestrictedICC".to_string()));
                reader.seek(SeekFrom::Current((size - 3) as i64))?;
            }
            3 => {
                // Any ICC profile (JPX only)
                metadata.exif.set("JP2:ColorMethod", AttrValue::Str("ICC".to_string()));
                reader.seek(SeekFrom::Current((size - 3) as i64))?;
            }
            _ => {
                reader.seek(SeekFrom::Current((size - 3) as i64))?;
            }
        }

        Ok(())
    }

    /// Parse resolution box (res).
    fn parse_res(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        let end_pos = reader.stream_position()? + size;

        while reader.stream_position()? < end_pos {
            let mut sub_size_buf = [0u8; 4];
            if reader.read_exact(&mut sub_size_buf).is_err() {
                break;
            }
            let sub_size = u32::from_be_bytes(sub_size_buf) as u64;

            let mut sub_type = [0u8; 4];
            if reader.read_exact(&mut sub_type).is_err() {
                break;
            }

            if (&sub_type == b"resc" || &sub_type == b"resd") && sub_size >= 18 {
                // Capture/display resolution
                let mut res = [0u8; 10];
                reader.read_exact(&mut res)?;

                let vr_n = u16::from_be_bytes([res[0], res[1]]);
                let vr_d = u16::from_be_bytes([res[2], res[3]]);
                let hr_n = u16::from_be_bytes([res[4], res[5]]);
                let hr_d = u16::from_be_bytes([res[6], res[7]]);
                let vr_e = res[8] as i8;
                let hr_e = res[9] as i8;

                if vr_d != 0 && hr_d != 0 {
                    let v_res = (vr_n as f64 / vr_d as f64) * 10f64.powi(vr_e as i32);
                    let h_res = (hr_n as f64 / hr_d as f64) * 10f64.powi(hr_e as i32);

                    let prefix = if &sub_type == b"resc" { "Capture" } else { "Display" };
                    metadata.exif.set(
                        &format!("JP2:{}XResolution", prefix),
                        AttrValue::Float(h_res as f32),
                    );
                    metadata.exif.set(
                        &format!("JP2:{}YResolution", prefix),
                        AttrValue::Float(v_res as f32),
                    );
                }

                let remaining = sub_size.saturating_sub(18);
                if remaining > 0 {
                    reader.seek(SeekFrom::Current(remaining as i64))?;
                }
            } else {
                reader.seek(SeekFrom::Current(sub_size.saturating_sub(8) as i64))?;
            }
        }

        Ok(())
    }

    /// Parse UUID box (may contain XMP).
    fn parse_uuid(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if size < 16 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(());
        }

        let mut uuid = [0u8; 16];
        reader.read_exact(&mut uuid)?;

        if uuid == *XMP_UUID {
            // XMP data follows
            let xmp_size = (size - 16) as usize;
            if xmp_size > 0 && xmp_size < 10 * 1024 * 1024 {
                let mut xmp_data = vec![0u8; xmp_size];
                reader.read_exact(&mut xmp_data)?;

                if let Ok(xmp_str) = std::str::from_utf8(&xmp_data) {
                    if let Ok(xmp_attrs) = exiftool_xmp::XmpParser::parse(xmp_str) {
                        for (key, value) in xmp_attrs.iter() {
                            metadata.exif.set(format!("XMP:{}", key), value.clone());
                        }
                    }
                    metadata.xmp = Some(xmp_str.to_string());
                }
            } else {
                reader.seek(SeekFrom::Current(xmp_size as i64))?;
            }
        } else {
            reader.seek(SeekFrom::Current((size - 16) as i64))?;
        }

        Ok(())
    }

    /// Parse XML box (may contain XMP).
    fn parse_xml(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if size > 10 * 1024 * 1024 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(());
        }

        let mut xml_data = vec![0u8; size as usize];
        reader.read_exact(&mut xml_data)?;

        if let Ok(xml_str) = std::str::from_utf8(&xml_data) {
            // Check if it's XMP (starts with <?xpacket or <x:xmpmeta or <rdf:RDF)
            let trimmed = xml_str.trim_start();
            if trimmed.starts_with("<?xpacket") || 
               trimmed.starts_with("<x:xmpmeta") || 
               trimmed.starts_with("<rdf:RDF") {
                if let Ok(xmp_attrs) = exiftool_xmp::XmpParser::parse(xml_str) {
                    for (key, value) in xmp_attrs.iter() {
                        metadata.exif.set(format!("XMP:{}", key), value.clone());
                    }
                }
                metadata.xmp = Some(xml_str.to_string());
            } else {
                // Generic XML, just note it
                metadata.exif.set("JP2:HasXML", AttrValue::Bool(true));
            }
        }

        Ok(())
    }

    /// Parse raw JPEG 2000 codestream.
    fn parse_codestream(&self, reader: &mut dyn ReadSeek, metadata: &mut Metadata) -> Result<()> {
        // Read SOC marker
        let mut marker = [0u8; 2];
        if reader.read_exact(&mut marker).is_err() {
            return Ok(());
        }

        if marker != *J2K_SOC_MARKER {
            return Ok(());
        }

        // Look for SIZ marker (required, contains image dimensions)
        loop {
            if reader.read_exact(&mut marker).is_err() {
                break;
            }

            if marker[0] != 0xFF {
                break;
            }

            match marker[1] {
                0x51 => {
                    // SIZ marker - Image and tile size
                    self.parse_siz(reader, metadata)?;
                    break;
                }
                0x52..=0x5F | 0x61..=0x63 => {
                    // Other markers with length - skip
                    let mut len = [0u8; 2];
                    if reader.read_exact(&mut len).is_err() {
                        break;
                    }
                    let length = u16::from_be_bytes(len) as i64;
                    reader.seek(SeekFrom::Current(length - 2))?;
                }
                0x90 => {
                    // SOT - start of tile, stop looking
                    break;
                }
                0x93 => {
                    // SOD - start of data, stop looking
                    break;
                }
                _ => {
                    // Skip unknown marker
                }
            }
        }

        Ok(())
    }

    /// Parse SIZ marker (image size).
    fn parse_siz(&self, reader: &mut dyn ReadSeek, metadata: &mut Metadata) -> Result<()> {
        let mut len = [0u8; 2];
        reader.read_exact(&mut len)?;
        let length = u16::from_be_bytes(len) as usize;

        if length < 38 {
            reader.seek(SeekFrom::Current((length - 2) as i64))?;
            return Ok(());
        }

        let mut siz = [0u8; 36];
        reader.read_exact(&mut siz)?;

        let _rsiz = u16::from_be_bytes([siz[0], siz[1]]);
        let xsiz = u32::from_be_bytes([siz[2], siz[3], siz[4], siz[5]]);
        let ysiz = u32::from_be_bytes([siz[6], siz[7], siz[8], siz[9]]);
        let xosiz = u32::from_be_bytes([siz[10], siz[11], siz[12], siz[13]]);
        let yosiz = u32::from_be_bytes([siz[14], siz[15], siz[16], siz[17]]);
        let _xtsiz = u32::from_be_bytes([siz[18], siz[19], siz[20], siz[21]]);
        let _ytsiz = u32::from_be_bytes([siz[22], siz[23], siz[24], siz[25]]);
        let _xtosiz = u32::from_be_bytes([siz[26], siz[27], siz[28], siz[29]]);
        let _ytosiz = u32::from_be_bytes([siz[30], siz[31], siz[32], siz[33]]);
        let csiz = u16::from_be_bytes([siz[34], siz[35]]);

        // Image dimensions (reference grid minus offset)
        let width = xsiz - xosiz;
        let height = ysiz - yosiz;

        metadata.exif.set("File:ImageWidth", AttrValue::UInt(width));
        metadata.exif.set("File:ImageHeight", AttrValue::UInt(height));
        metadata.exif.set("JP2:NumComponents", AttrValue::UInt(csiz as u32));

        // Skip component info
        let remaining = length.saturating_sub(38);
        if remaining > 0 {
            reader.seek(SeekFrom::Current(remaining as i64))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_can_parse_jp2() {
        let parser = Jp2Parser;
        let mut header = vec![0u8; 16];
        // JP2 signature box
        header[0..4].copy_from_slice(&[0x00, 0x00, 0x00, 0x0C]); // size
        header[4..8].copy_from_slice(b"jP  "); // type
        header[8..12].copy_from_slice(JP2_SIGNATURE); // signature

        assert!(parser.can_parse(&header));
    }

    #[test]
    fn test_can_parse_j2k() {
        let parser = Jp2Parser;
        assert!(parser.can_parse(&[0xFF, 0x4F, 0x00, 0x00]));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = Jp2Parser;
        assert!(!parser.can_parse(b"RIFF"));
        assert!(!parser.can_parse(b"\x89PNG"));
        assert!(!parser.can_parse(&[]));
    }

    #[test]
    fn test_parse_jp2_minimal() {
        let mut data = Vec::new();

        // Signature box
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x0C]);
        data.extend_from_slice(b"jP  ");
        data.extend_from_slice(JP2_SIGNATURE);

        // File type box
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x14]); // size=20
        data.extend_from_slice(b"ftyp");
        data.extend_from_slice(b"jp2 "); // brand
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // minor version
        data.extend_from_slice(b"jp2 "); // compatibility

        let parser = Jp2Parser;
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("File:FileType"), Some("JP2"));
        assert_eq!(meta.exif.get_str("JP2:Brand"), Some("jp2"));
    }

    #[test]
    fn test_parse_j2k_minimal() {
        // SOC + SIZ marker with image dimensions
        let mut data = vec![0xFF, 0x4F]; // SOC
        data.extend_from_slice(&[0xFF, 0x51]); // SIZ marker
        data.extend_from_slice(&[0x00, 0x2F]); // length = 47

        // SIZ content
        data.extend_from_slice(&[0x00, 0x00]); // Rsiz
        data.extend_from_slice(&[0x00, 0x00, 0x04, 0x00]); // Xsiz = 1024
        data.extend_from_slice(&[0x00, 0x00, 0x03, 0x00]); // Ysiz = 768
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // XOsiz = 0
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // YOsiz = 0
        data.extend_from_slice(&[0x00, 0x00, 0x01, 0x00]); // XTsiz
        data.extend_from_slice(&[0x00, 0x00, 0x01, 0x00]); // YTsiz
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // XTOsiz
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // YTOsiz
        data.extend_from_slice(&[0x00, 0x03]); // Csiz = 3 components
        // Component info (3 components, 3 bytes each)
        data.extend_from_slice(&[0x07, 0x01, 0x01]); // component 0
        data.extend_from_slice(&[0x07, 0x01, 0x01]); // component 1
        data.extend_from_slice(&[0x07, 0x01, 0x01]); // component 2

        let parser = Jp2Parser;
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("File:FileType"), Some("J2K"));
        assert_eq!(meta.exif.get_u32("File:ImageWidth"), Some(1024));
        assert_eq!(meta.exif.get_u32("File:ImageHeight"), Some(768));
        assert_eq!(meta.exif.get_u32("JP2:NumComponents"), Some(3));
    }
}

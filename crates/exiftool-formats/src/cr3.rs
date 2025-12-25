//! Canon CR3 format parser.
//!
//! CR3 is Canon's RAW format based on ISOBMFF (ISO Base Media File Format).
//! Structure:
//! - ftyp box: "crx " brand
//! - moov box: contains Canon-specific metadata (CNCV, CCTP, CMT1-4, THMB)
//! - mdat box: RAW image data
//!
//! Canon-specific boxes in moov/trak/mdia/minf/stbl:
//! - CMT1: IFD0 (main TIFF/EXIF)
//! - CMT2: ExifIFD
//! - CMT3: MakerNotes
//! - CMT4: GPS
//! - CNCV: Canon Compressor Version
//! - CCTP: Canon CCTP box
//! - THMB: Thumbnail

use crate::tag_lookup::{lookup_exif_subifd, lookup_gps, lookup_ifd0};
use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use exiftool_core::{ByteOrder, IfdReader};
use std::io::SeekFrom;

/// CR3 format parser.
pub struct Cr3Parser;

impl FormatParser for Cr3Parser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 12 {
            return false;
        }
        
        // Check for ftyp box
        if &header[4..8] != b"ftyp" {
            return false;
        }
        
        // Check for "crx " brand (CR3)
        &header[8..12] == b"crx "
    }

    fn format_name(&self) -> &'static str {
        "CR3"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["cr3"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("CR3");

        // Parse ftyp box
        reader.seek(SeekFrom::Start(0))?;
        
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        
        let ftyp_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
        
        if &buf[4..8] != b"ftyp" {
            return Err(Error::InvalidStructure("Missing ftyp box".into()));
        }
        
        // Read major brand
        let mut brand = [0u8; 4];
        reader.read_exact(&mut brand)?;
        metadata.exif.set("MajorBrand", AttrValue::Str(String::from_utf8_lossy(&brand).to_string()));
        
        // Read minor version
        let mut version = [0u8; 4];
        reader.read_exact(&mut version)?;
        let minor_version = u32::from_be_bytes(version);
        metadata.exif.set("MinorVersion", AttrValue::UInt(minor_version));
        
        // Parse boxes after ftyp
        reader.seek(SeekFrom::Start(ftyp_size))?;
        
        let file_size = reader.seek(SeekFrom::End(0))?;
        reader.seek(SeekFrom::Start(ftyp_size))?;
        
        while reader.stream_position()? < file_size {
            let pos = reader.stream_position()?;
            
            if reader.read_exact(&mut buf).is_err() {
                break;
            }
            
            let mut box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];
            
            // Handle extended size
            if box_size == 1 {
                let mut ext_size = [0u8; 8];
                reader.read_exact(&mut ext_size)?;
                box_size = u64::from_be_bytes(ext_size);
            } else if box_size == 0 {
                box_size = file_size - pos;
            }
            
            match &box_type {
                b"moov" => {
                    // moov contains Canon metadata boxes
                    self.parse_moov(reader, pos + 8, pos + box_size, &mut metadata)?;
                }
                b"mdat" => {
                    metadata.exif.set("MediaDataSize", AttrValue::UInt((box_size - 8) as u32));
                }
                b"uuid" => {
                    // Canon may store XMP in uuid box
                    self.parse_uuid(reader, pos + 8, box_size - 8, &mut metadata)?;
                }
                _ => {}
            }
            
            if box_size == 0 || pos + box_size > file_size {
                break;
            }
            reader.seek(SeekFrom::Start(pos + box_size))?;
        }
        
        Ok(metadata)
    }
}

impl Cr3Parser {
    /// Parse moov box and its children.
    fn parse_moov(
        &self,
        reader: &mut dyn ReadSeek,
        start: u64,
        end: u64,
        metadata: &mut Metadata,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(start))?;
        
        let mut buf = [0u8; 8];
        
        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            
            if reader.read_exact(&mut buf).is_err() {
                break;
            }
            
            let box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];
            
            if box_size < 8 || pos + box_size > end {
                break;
            }
            
            match &box_type {
                b"trak" => {
                    // Track box - may contain CMT boxes
                    self.parse_trak(reader, pos + 8, pos + box_size, metadata)?;
                }
                b"uuid" => {
                    // Canon-specific UUID boxes
                    self.parse_uuid(reader, pos + 8, box_size - 8, metadata)?;
                }
                b"CNCV" => {
                    // Canon Compressor Version
                    let mut cncv = vec![0u8; (box_size - 8) as usize];
                    reader.read_exact(&mut cncv)?;
                    let version = String::from_utf8_lossy(&cncv).trim_end_matches('\0').to_string();
                    metadata.exif.set("CanonCompressorVersion", AttrValue::Str(version));
                }
                _ => {}
            }
            
            reader.seek(SeekFrom::Start(pos + box_size))?;
        }
        
        Ok(())
    }
    
    /// Parse trak box recursively looking for CMT boxes.
    fn parse_trak(
        &self,
        reader: &mut dyn ReadSeek,
        start: u64,
        end: u64,
        metadata: &mut Metadata,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(start))?;
        
        let mut buf = [0u8; 8];
        
        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            
            if reader.read_exact(&mut buf).is_err() {
                break;
            }
            
            let box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];
            
            if box_size < 8 || pos + box_size > end {
                break;
            }
            
            match &box_type {
                b"mdia" | b"minf" | b"stbl" => {
                    // Container boxes - recurse
                    self.parse_trak(reader, pos + 8, pos + box_size, metadata)?;
                }
                b"CMT1" => {
                    // IFD0 - main EXIF
                    self.parse_cmt_box(reader, pos + 8, box_size - 8, metadata, "IFD0")?;
                }
                b"CMT2" => {
                    // ExifIFD
                    self.parse_cmt_box(reader, pos + 8, box_size - 8, metadata, "EXIF")?;
                }
                b"CMT3" => {
                    // MakerNotes
                    self.parse_cmt_box(reader, pos + 8, box_size - 8, metadata, "MakerNotes")?;
                }
                b"CMT4" => {
                    // GPS
                    self.parse_cmt_box(reader, pos + 8, box_size - 8, metadata, "GPS")?;
                }
                b"THMB" => {
                    // Thumbnail
                    metadata.exif.set("ThumbnailSize", AttrValue::UInt((box_size - 8) as u32));
                }
                _ => {}
            }
            
            reader.seek(SeekFrom::Start(pos + box_size))?;
        }
        
        Ok(())
    }
    
    /// Parse CMT box containing TIFF IFD.
    fn parse_cmt_box(
        &self,
        reader: &mut dyn ReadSeek,
        start: u64,
        size: u64,
        metadata: &mut Metadata,
        prefix: &str,
    ) -> Result<()> {
        if size < 8 {
            return Ok(());
        }
        
        reader.seek(SeekFrom::Start(start))?;
        
        // Read TIFF data
        let mut tiff_data = vec![0u8; size as usize];
        reader.read_exact(&mut tiff_data)?;
        
        // Parse byte order
        let byte_order = match ByteOrder::from_marker([tiff_data[0], tiff_data[1]]) {
            Ok(bo) => bo,
            Err(_) => return Ok(()), // Invalid TIFF header
        };
        
        let ifd_reader = IfdReader::new(&tiff_data, byte_order, 0);
        
        // Parse TIFF header to get IFD offset
        let ifd_offset = match ifd_reader.parse_header() {
            Ok(offset) => offset,
            Err(_) => return Ok(()),
        };
        
        // Read IFD entries
        if let Ok((entries, _)) = ifd_reader.read_ifd(ifd_offset) {
            for entry in entries {
                let tag_name = match prefix {
                    "IFD0" => lookup_ifd0(entry.tag),
                    "EXIF" => lookup_exif_subifd(entry.tag),
                    "GPS" => lookup_gps(entry.tag),
                    _ => None,
                };
                
                if let Some(name) = tag_name {
                    let value = entry_to_attr(&entry);
                    metadata.exif.set(name, value);
                }
            }
        }
        
        Ok(())
    }
    
    /// Parse UUID box (may contain XMP).
    fn parse_uuid(
        &self,
        reader: &mut dyn ReadSeek,
        start: u64,
        size: u64,
        metadata: &mut Metadata,
    ) -> Result<()> {
        if size < 16 {
            return Ok(());
        }
        
        reader.seek(SeekFrom::Start(start))?;
        
        // Read UUID (16 bytes)
        let mut uuid = [0u8; 16];
        reader.read_exact(&mut uuid)?;
        
        // XMP UUID: BE7ACFCB-97A9-42E8-9C71-999491E3AFAC
        const XMP_UUID: [u8; 16] = [
            0xBE, 0x7A, 0xCF, 0xCB, 0x97, 0xA9, 0x42, 0xE8,
            0x9C, 0x71, 0x99, 0x94, 0x91, 0xE3, 0xAF, 0xAC,
        ];
        
        if uuid == XMP_UUID {
            let xmp_size = size - 16;
            if xmp_size > 0 && xmp_size < 10_000_000 {
                let mut xmp_data = vec![0u8; xmp_size as usize];
                reader.read_exact(&mut xmp_data)?;
                
                if let Ok(xmp) = String::from_utf8(xmp_data) {
                    // Parse XMP
                    if let Ok(xmp_attrs) = exiftool_xmp::XmpParser::parse(&xmp) {
                        for (key, value) in xmp_attrs.iter() {
                            metadata.exif.set(format!("XMP:{}", key), value.clone());
                        }
                    }
                    metadata.xmp = Some(xmp);
                }
            }
        }
        
        Ok(())
    }
}

// Use shared entry_to_attr from crate::utils
use crate::utils::entry_to_attr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_cr3() {
        let parser = Cr3Parser;
        let header = [
            0x00, 0x00, 0x00, 0x18,
            b'f', b't', b'y', b'p',
            b'c', b'r', b'x', b' ',
        ];
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn reject_heic() {
        let parser = Cr3Parser;
        let header = [
            0x00, 0x00, 0x00, 0x18,
            b'f', b't', b'y', b'p',
            b'h', b'e', b'i', b'c',
        ];
        assert!(!parser.can_parse(&header));
    }

    #[test]
    fn reject_jpeg() {
        let parser = Cr3Parser;
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
    }
}

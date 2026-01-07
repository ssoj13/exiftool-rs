//! X3F (Sigma/Foveon) RAW format parser.
//!
//! X3F is Sigma's proprietary RAW format for Foveon sensor cameras.
//!
//! # Structure
//!
//! - 4 bytes: Magic "FOVb"
//! - 4 bytes: Version (major.minor)
//! - 8 bytes: Unique ID
//! - 4 bytes: Mark bits
//! - 4 bytes: Columns
//! - 4 bytes: Rows
//! - 4 bytes: Rotation
//! - Variable: White balance string (null-terminated)
//! - Variable: Extended data (v2.1+)
//! - Directory at end of file

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// X3F format magic.
const X3F_MAGIC: &[u8; 4] = b"FOVb";

/// X3F format parser.
pub struct X3fParser;

impl FormatParser for X3fParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 4 && &header[0..4] == X3F_MAGIC
    }

    fn format_name(&self) -> &'static str {
        "X3F"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["x3f"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("X3F");
        meta.set_file_type("X3F", "image/x-sigma-x3f");
        meta.exif.set("Make", AttrValue::Str("SIGMA".to_string()));

        // Read header
        let mut header = [0u8; 40];
        reader.seek(SeekFrom::Start(0))?;
        reader.read_exact(&mut header)?;

        // Validate magic
        if &header[0..4] != X3F_MAGIC {
            return Err(crate::Error::InvalidStructure("Not a valid X3F file".to_string()));
        }

        // Version (little-endian)
        let version_minor = header[4];
        let version_major = header[5];
        meta.exif.set(
            "X3F:Version",
            AttrValue::Str(format!("{}.{}", version_major, version_minor)),
        );

        // Unique ID (8 bytes at offset 8)
        let unique_id = format!(
            "{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            header[8], header[9], header[10], header[11],
            header[12], header[13], header[14], header[15]
        );
        meta.exif.set("X3F:UniqueID", AttrValue::Str(unique_id));

        // Mark bits (offset 16)
        let mark = u32::from_le_bytes([header[16], header[17], header[18], header[19]]);
        if mark & 1 != 0 {
            meta.exif.set("X3F:ColorMode", AttrValue::Str("Color".to_string()));
        } else {
            meta.exif.set("X3F:ColorMode", AttrValue::Str("Monochrome".to_string()));
        }

        // Image dimensions (offsets 20, 24)
        let columns = u32::from_le_bytes([header[20], header[21], header[22], header[23]]);
        let rows = u32::from_le_bytes([header[24], header[25], header[26], header[27]]);
        meta.exif.set("File:ImageWidth", AttrValue::UInt(columns));
        meta.exif.set("File:ImageHeight", AttrValue::UInt(rows));

        // Rotation (offset 28)
        let rotation = u32::from_le_bytes([header[28], header[29], header[30], header[31]]);
        let rotation_str = match rotation {
            0 => "Horizontal",
            90 => "Rotate 90 CW",
            180 => "Rotate 180",
            270 => "Rotate 270 CW",
            _ => "Unknown",
        };
        meta.exif.set("X3F:Rotation", AttrValue::Str(rotation_str.to_string()));

        // White balance string (null-terminated, starts at offset 32)
        let mut wb_bytes = Vec::new();
        reader.seek(SeekFrom::Start(32))?;
        let mut byte = [0u8; 1];
        for _ in 0..64 {
            if reader.read_exact(&mut byte).is_err() {
                break;
            }
            if byte[0] == 0 {
                break;
            }
            wb_bytes.push(byte[0]);
        }
        if !wb_bytes.is_empty() {
            let wb = String::from_utf8_lossy(&wb_bytes).to_string();
            meta.exif.set("X3F:WhiteBalance", AttrValue::Str(wb));
        }

        // File size
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        // Try to read directory at end of file
        self.parse_directory(reader, &mut meta, file_size)?;

        Ok(meta)
    }
}

impl X3fParser {
    /// Parse X3F directory structure at end of file.
    fn parse_directory(
        &self,
        reader: &mut dyn ReadSeek,
        meta: &mut Metadata,
        file_size: u64,
    ) -> Result<()> {
        // Directory offset is at file_size - 4
        if file_size < 12 {
            return Ok(());
        }

        reader.seek(SeekFrom::End(-4))?;
        let mut offset_bytes = [0u8; 4];
        reader.read_exact(&mut offset_bytes)?;
        let dir_offset = u32::from_le_bytes(offset_bytes) as u64;

        if dir_offset >= file_size || dir_offset < 40 {
            return Ok(());
        }

        // Read directory header
        reader.seek(SeekFrom::Start(dir_offset))?;
        let mut dir_header = [0u8; 12];
        if reader.read_exact(&mut dir_header).is_err() {
            return Ok(());
        }

        // Check directory magic "SECd"
        if &dir_header[0..4] != b"SECd" {
            return Ok(());
        }

        // Section version
        let _section_version = u32::from_le_bytes([dir_header[4], dir_header[5], dir_header[6], dir_header[7]]);

        // Number of directory entries
        let num_entries = u32::from_le_bytes([dir_header[8], dir_header[9], dir_header[10], dir_header[11]]) as usize;

        if num_entries > 100 {
            return Ok(());
        }

        meta.exif.set("X3F:DirectoryEntries", AttrValue::UInt(num_entries as u32));

        // Read directory entries (12 bytes each)
        for _ in 0..num_entries {
            let mut entry = [0u8; 12];
            if reader.read_exact(&mut entry).is_err() {
                break;
            }

            let entry_offset = u32::from_le_bytes([entry[0], entry[1], entry[2], entry[3]]) as u64;
            let entry_size = u32::from_le_bytes([entry[4], entry[5], entry[6], entry[7]]);
            let entry_type = &entry[8..12];

            // Parse known section types
            match entry_type {
                b"PROP" => {
                    self.parse_prop_section(reader, meta, entry_offset, entry_size)?;
                }
                b"IMA2" | b"IMAG" => {
                    meta.exif.set("X3F:ImageDataOffset", AttrValue::UInt64(entry_offset));
                    meta.exif.set("X3F:ImageDataSize", AttrValue::UInt(entry_size));
                }
                b"CAMF" => {
                    meta.exif.set("X3F:CameraInfoOffset", AttrValue::UInt64(entry_offset));
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Parse PROP (property) section.
    fn parse_prop_section(
        &self,
        reader: &mut dyn ReadSeek,
        meta: &mut Metadata,
        offset: u64,
        _size: u32,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(offset))?;

        // Section header
        let mut header = [0u8; 24];
        if reader.read_exact(&mut header).is_err() {
            return Ok(());
        }

        // Check magic "SECp"
        if &header[0..4] != b"SECp" {
            return Ok(());
        }

        // Number of properties
        let num_props = u32::from_le_bytes([header[8], header[9], header[10], header[11]]) as usize;
        let _format = u32::from_le_bytes([header[12], header[13], header[14], header[15]]);

        // Property data offset within section
        let prop_offset = u32::from_le_bytes([header[20], header[21], header[22], header[23]]) as u64;

        if num_props > 100 {
            return Ok(());
        }

        // Read property entries (8 bytes each: name_off, value_off)
        let mut entries = Vec::with_capacity(num_props);
        for _ in 0..num_props {
            let mut entry = [0u8; 8];
            if reader.read_exact(&mut entry).is_err() {
                break;
            }
            let name_off = u32::from_le_bytes([entry[0], entry[1], entry[2], entry[3]]) as u64;
            let value_off = u32::from_le_bytes([entry[4], entry[5], entry[6], entry[7]]) as u64;
            entries.push((name_off, value_off));
        }

        // Read property strings
        for (name_off, value_off) in entries {
            let name = self.read_string(reader, offset + prop_offset + name_off)?;
            let value = self.read_string(reader, offset + prop_offset + value_off)?;

            if name.is_empty() {
                continue;
            }

            // Map known properties
            match name.as_str() {
                "CAMMANUF" => {
                    meta.exif.set("Make", AttrValue::Str(value));
                }
                "CAMMODEL" => {
                    meta.exif.set("Model", AttrValue::Str(value));
                }
                "CAMSERIAL" => {
                    meta.exif.set("X3F:SerialNumber", AttrValue::Str(value));
                }
                "EXPTIME" => {
                    if let Ok(v) = value.parse::<f64>() {
                        meta.exif.set("ExposureTime", AttrValue::Float(v as f32));
                    }
                }
                "APERTURE" => {
                    if let Ok(v) = value.parse::<f64>() {
                        meta.exif.set("FNumber", AttrValue::Float(v as f32));
                    }
                }
                "FLENGTH" => {
                    if let Ok(v) = value.parse::<f64>() {
                        meta.exif.set("FocalLength", AttrValue::Float(v as f32));
                    }
                }
                "ISO" => {
                    if let Ok(v) = value.parse::<u32>() {
                        meta.exif.set("ISO", AttrValue::UInt(v));
                    }
                }
                "FLEQ35MM" => {
                    if let Ok(v) = value.parse::<f64>() {
                        meta.exif.set("FocalLengthIn35mmFormat", AttrValue::Float(v as f32));
                    }
                }
                "DATETIME" => {
                    meta.exif.set("DateTimeOriginal", AttrValue::Str(value));
                }
                "LENSAPTS" | "LENSAPTSMIN" => {
                    meta.exif.set("X3F:LensAperture", AttrValue::Str(value));
                }
                "LENSFLMIN" | "LENSFLMAX" => {
                    meta.exif.set(format!("X3F:{}", name), AttrValue::Str(value));
                }
                _ => {
                    // Store other properties with X3F prefix
                    if !value.is_empty() && value.len() < 256 {
                        meta.exif.set(format!("X3F:{}", name), AttrValue::Str(value));
                    }
                }
            }
        }

        Ok(())
    }

    /// Read null-terminated string at offset.
    fn read_string(&self, reader: &mut dyn ReadSeek, offset: u64) -> Result<String> {
        reader.seek(SeekFrom::Start(offset))?;
        let mut bytes = Vec::new();
        let mut byte = [0u8; 1];
        for _ in 0..256 {
            if reader.read_exact(&mut byte).is_err() {
                break;
            }
            if byte[0] == 0 {
                break;
            }
            bytes.push(byte[0]);
        }
        Ok(String::from_utf8_lossy(&bytes).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_x3f_header(width: u32, height: u32) -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // Magic
        data[0..4].copy_from_slice(b"FOVb");
        // Version 2.3
        data[4] = 3; // minor
        data[5] = 2; // major
        // Unique ID
        data[8..16].copy_from_slice(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
        // Mark bits (color)
        data[16..20].copy_from_slice(&1u32.to_le_bytes());
        // Dimensions
        data[20..24].copy_from_slice(&width.to_le_bytes());
        data[24..28].copy_from_slice(&height.to_le_bytes());
        // Rotation
        data[28..32].copy_from_slice(&0u32.to_le_bytes());
        // White balance
        data[32..36].copy_from_slice(b"Auto");
        data[36] = 0;
        data
    }

    #[test]
    fn test_can_parse() {
        let parser = X3fParser;
        let data = make_x3f_header(4640, 3088);
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = X3fParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"JUNK"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = X3fParser;
        let data = make_x3f_header(4640, 3088);
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "X3F");
        assert_eq!(meta.exif.get_str("X3F:Version"), Some("2.3"));
        assert_eq!(meta.exif.get_u32("File:ImageWidth"), Some(4640));
        assert_eq!(meta.exif.get_u32("File:ImageHeight"), Some(3088));
        assert_eq!(meta.exif.get_str("X3F:ColorMode"), Some("Color"));
        assert_eq!(meta.exif.get_str("X3F:WhiteBalance"), Some("Auto"));
    }

    #[test]
    fn test_parse_rotation() {
        let parser = X3fParser;
        let mut data = make_x3f_header(3088, 4640);
        data[28..32].copy_from_slice(&90u32.to_le_bytes());
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("X3F:Rotation"), Some("Rotate 90 CW"));
    }
}

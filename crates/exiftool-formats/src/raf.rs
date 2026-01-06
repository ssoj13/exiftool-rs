//! Fujifilm RAF (RAW) format parser.
//!
//! RAF structure:
//! - Header (16 bytes): "FUJIFILMCCD-RAW " magic
//! - Format version at offset 0x3C (4 bytes): "0201", "0202", etc.
//! - JPEG preview offset at 0x54 (4 bytes, big-endian)
//! - JPEG preview length at 0x58 (4 bytes, big-endian)
//! - CFA data offset at 0x5C (4 bytes)
//! - CFA data length at 0x60 (4 bytes)
//!
//! The JPEG preview contains full EXIF metadata including MakerNotes.
//! We parse the embedded JPEG to extract metadata.

use crate::{Error, FormatParser, JpegParser, Metadata, ReadSeek, Result};
use std::io::SeekFrom;

/// RAF magic signature.
const RAF_MAGIC: &[u8; 16] = b"FUJIFILMCCD-RAW ";

/// Fujifilm RAF format parser.
pub struct RafParser;

impl FormatParser for RafParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 16 && header[..16] == *RAF_MAGIC
    }

    fn format_name(&self) -> &'static str {
        "RAF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["raf"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        // Read RAF header
        let mut header = [0u8; 0x64]; // 100 bytes covers all header fields
        reader.read_exact(&mut header)?;

        // Verify magic
        if &header[..16] != RAF_MAGIC {
            return Err(Error::InvalidStructure("invalid RAF magic".into()));
        }

        // Read format version string (4 bytes at 0x3C)
        let version = String::from_utf8_lossy(&header[0x3C..0x40]).to_string();

        // Read JPEG preview offset and length (big-endian)
        let jpeg_offset = u32::from_be_bytes([header[0x54], header[0x55], header[0x56], header[0x57]]);
        let jpeg_length = u32::from_be_bytes([header[0x58], header[0x59], header[0x5A], header[0x5B]]);

        if jpeg_offset == 0 || jpeg_length == 0 {
            return Err(Error::InvalidStructure("RAF has no JPEG preview".into()));
        }

        // Seek to JPEG preview
        reader.seek(SeekFrom::Start(jpeg_offset as u64))?;

        // Read JPEG preview into buffer
        let mut jpeg_data = vec![0u8; jpeg_length as usize];
        reader.read_exact(&mut jpeg_data)?;

        // Parse embedded JPEG for metadata
        let mut jpeg_reader = std::io::Cursor::new(&jpeg_data);
        let jpeg_parser = JpegParser;

        let mut metadata = jpeg_parser.parse(&mut jpeg_reader)?;

        // Store JPEG preview data
        metadata.preview = Some(jpeg_data);

        // Update format to RAF
        metadata.format = "RAF";

        // Add RAF-specific info
        metadata.exif.set("RAFVersion", exiftool_attrs::AttrValue::Str(version.trim_end_matches('\0').to_string()));

        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_raf() {
        let parser = RafParser;
        let mut header = RAF_MAGIC.to_vec();
        header.extend_from_slice(&[0u8; 16]); // Padding
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn reject_jpeg() {
        let parser = RafParser;
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
    }

    #[test]
    fn reject_tiff() {
        let parser = RafParser;
        assert!(!parser.can_parse(&[b'I', b'I', 0x2A, 0x00]));
    }
}

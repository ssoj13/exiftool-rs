//! NRW (Nikon RAW) format parser.
//!
//! NRW is Nikon's RAW format for Coolpix cameras.
//! It's TIFF-based with Nikon-specific tags.
//!
//! Detection: TIFF header + Nikon make + .nrw extension

use crate::{FormatParser, Metadata, ReadSeek, Result, TiffParser};
use exiftool_attrs::AttrValue;

/// NRW format parser.
pub struct NrwParser {
    tiff: TiffParser,
}

impl NrwParser {
    /// Create new NRW parser.
    pub fn new() -> Self {
        Self {
            tiff: TiffParser::default(),
        }
    }
}

impl Default for NrwParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for NrwParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // NRW is TIFF-based, check for TIFF magic
        if header.len() < 8 {
            return false;
        }

        // TIFF magic: II (little) or MM (big) + 42
        let is_tiff = match &header[0..4] {
            [0x49, 0x49, 0x2A, 0x00] => true, // II + 42 LE
            [0x4D, 0x4D, 0x00, 0x2A] => true, // MM + 42 BE
            _ => false,
        };

        if !is_tiff {
            return false;
        }

        // NRW detection is primarily by extension
        // Magic alone can't distinguish from regular TIFF/NEF
        // Return false here - registry uses by_extension for NRW
        false
    }

    fn format_name(&self) -> &'static str {
        "NRW"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["nrw"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        // Parse as TIFF first
        let mut meta = self.tiff.parse(reader)?;

        // Override format name
        meta.format = "NRW";
        meta.set_file_type("NRW", "image/x-nikon-nrw");

        // Verify this is actually Nikon
        let is_nikon = meta
            .exif
            .get_str("Make")
            .map(|m| m.to_lowercase().contains("nikon"))
            .unwrap_or(false);

        if !is_nikon {
            // Set Make to Nikon if not present (NRW is always Nikon)
            if meta.exif.get_str("Make").is_none() {
                meta.exif.set("Make", AttrValue::Str("NIKON".to_string()));
            }
        }

        Ok(meta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_tiff_header() -> Vec<u8> {
        let mut data = vec![0u8; 256];
        // TIFF header (little-endian)
        data[0..4].copy_from_slice(&[0x49, 0x49, 0x2A, 0x00]);
        // IFD offset: 8
        data[4..8].copy_from_slice(&8u32.to_le_bytes());
        // IFD entry count: 1
        data[8..10].copy_from_slice(&1u16.to_le_bytes());
        // Tag: Make (0x010F), Type: ASCII (2), Count: 6, Value offset: 100
        data[10..12].copy_from_slice(&0x010Fu16.to_le_bytes());
        data[12..14].copy_from_slice(&2u16.to_le_bytes());
        data[14..18].copy_from_slice(&6u32.to_le_bytes());
        data[18..22].copy_from_slice(&100u32.to_le_bytes());
        // Next IFD: 0
        data[22..26].copy_from_slice(&0u32.to_le_bytes());
        // Make value
        data[100..106].copy_from_slice(b"NIKON\0");
        data
    }

    #[test]
    fn test_format_info() {
        let parser = NrwParser::new();
        assert_eq!(parser.format_name(), "NRW");
        assert!(parser.extensions().contains(&"nrw"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = NrwParser::new();
        let data = make_tiff_header();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "NRW");
        assert_eq!(meta.exif.get_str("Make"), Some("NIKON"));
    }

    #[test]
    fn test_can_parse_returns_false() {
        // NRW detection is by extension, not magic
        let parser = NrwParser::new();
        let data = make_tiff_header();
        assert!(!parser.can_parse(&data));
    }
}

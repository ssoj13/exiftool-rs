//! RWL (Leica RAW) format parser.
//!
//! RWL is Leica's RAW format used in some of their cameras.
//! It's based on TIFF structure, similar to DNG but with Leica branding.

use crate::{FormatParser, Metadata, ReadSeek, Result, TiffParser};

/// RWL format parser.
pub struct RwlParser {
    tiff: TiffParser,
}

impl RwlParser {
    pub fn new() -> Self {
        Self {
            tiff: TiffParser::default(),
        }
    }
}

impl Default for RwlParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for RwlParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // RWL uses standard TIFF magic - detected by extension
        self.tiff.can_parse(header)
    }

    fn format_name(&self) -> &'static str {
        "RWL"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["rwl"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = self.tiff.parse(reader)?;
        
        // Check if this is actually a Leica file
        if let Some(make) = meta.exif.get_str("Make") {
            if make.to_uppercase().contains("LEICA") {
                meta.format = "RWL";
            }
        }
        
        Ok(meta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_tiff_header(make: &str) -> Vec<u8> {
        let mut data = vec![0u8; 512];
        data[0..4].copy_from_slice(b"II\x2A\x00");
        data[4..8].copy_from_slice(&8u32.to_le_bytes());
        data[8..10].copy_from_slice(&1u16.to_le_bytes());
        data[10..12].copy_from_slice(&0x010Fu16.to_le_bytes());
        data[12..14].copy_from_slice(&2u16.to_le_bytes());
        data[14..18].copy_from_slice(&(make.len() as u32 + 1).to_le_bytes());
        data[18..22].copy_from_slice(&100u32.to_le_bytes());
        data[22..26].copy_from_slice(&0u32.to_le_bytes());
        data[100..100 + make.len()].copy_from_slice(make.as_bytes());
        data
    }

    #[test]
    fn test_format_info() {
        let parser = RwlParser::new();
        assert_eq!(parser.format_name(), "RWL");
        assert!(parser.extensions().contains(&"rwl"));
    }

    #[test]
    fn test_parse_leica() {
        let parser = RwlParser::new();
        let data = make_tiff_header("LEICA CAMERA AG");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "RWL");
    }

    #[test]
    fn test_parse_leica_lowercase() {
        let parser = RwlParser::new();
        let data = make_tiff_header("Leica Camera AG");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "RWL");
    }

    #[test]
    fn test_parse_non_leica() {
        let parser = RwlParser::new();
        let data = make_tiff_header("Canon");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "TIFF");
    }
}

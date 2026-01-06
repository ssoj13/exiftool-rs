//! SRW (Samsung RAW) format parser.
//!
//! SRW is Samsung's RAW format used in their NX-series cameras.
//! It's based on TIFF structure with Samsung-specific MakerNotes.

use crate::{FormatParser, Metadata, ReadSeek, Result, TiffParser};

/// SRW format parser.
pub struct SrwParser {
    tiff: TiffParser,
}

impl SrwParser {
    pub fn new() -> Self {
        Self {
            tiff: TiffParser::default(),
        }
    }
}

impl Default for SrwParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for SrwParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // SRW uses standard TIFF magic - detected by extension
        self.tiff.can_parse(header)
    }

    fn format_name(&self) -> &'static str {
        "SRW"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["srw"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = self.tiff.parse(reader)?;
        
        // Check if this is actually a Samsung file
        if let Some(make) = meta.exif.get_str("Make") {
            if make.to_uppercase().contains("SAMSUNG") {
                meta.format = "SRW";
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
        let parser = SrwParser::new();
        assert_eq!(parser.format_name(), "SRW");
        assert!(parser.extensions().contains(&"srw"));
    }

    #[test]
    fn test_parse_samsung() {
        let parser = SrwParser::new();
        let data = make_tiff_header("SAMSUNG");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "SRW");
    }

    #[test]
    fn test_parse_samsung_lowercase() {
        let parser = SrwParser::new();
        let data = make_tiff_header("Samsung Techwin");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "SRW");
    }

    #[test]
    fn test_parse_non_samsung() {
        let parser = SrwParser::new();
        let data = make_tiff_header("Canon");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "TIFF");
    }
}

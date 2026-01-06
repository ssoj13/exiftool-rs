//! ERF (Epson RAW) format parser.
//!
//! ERF is Epson's RAW format used in their R-D1 rangefinder camera.
//! It's based on TIFF structure with Epson-specific tags.

use crate::{FormatParser, Metadata, ReadSeek, Result, TiffParser};

/// ERF format parser.
pub struct ErfParser {
    tiff: TiffParser,
}

impl ErfParser {
    pub fn new() -> Self {
        Self {
            tiff: TiffParser::default(),
        }
    }
}

impl Default for ErfParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for ErfParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // ERF uses standard TIFF magic - detected by extension
        self.tiff.can_parse(header)
    }

    fn format_name(&self) -> &'static str {
        "ERF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["erf"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = self.tiff.parse(reader)?;
        
        // Check if this is actually an Epson file
        if let Some(make) = meta.exif.get_str("Make") {
            if make.to_uppercase().contains("EPSON") {
                meta.format = "ERF";
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
        // TIFF little-endian
        data[0..4].copy_from_slice(b"II\x2A\x00");
        // IFD offset at 8
        data[4..8].copy_from_slice(&8u32.to_le_bytes());
        // 1 entry
        data[8..10].copy_from_slice(&1u16.to_le_bytes());
        // Make tag (0x010F), ASCII, length, offset
        data[10..12].copy_from_slice(&0x010Fu16.to_le_bytes());
        data[12..14].copy_from_slice(&2u16.to_le_bytes()); // ASCII
        data[14..18].copy_from_slice(&(make.len() as u32 + 1).to_le_bytes());
        data[18..22].copy_from_slice(&100u32.to_le_bytes()); // offset
        // Next IFD = 0
        data[22..26].copy_from_slice(&0u32.to_le_bytes());
        // Make string at offset 100
        data[100..100 + make.len()].copy_from_slice(make.as_bytes());
        data
    }

    #[test]
    fn test_format_info() {
        let parser = ErfParser::new();
        assert_eq!(parser.format_name(), "ERF");
        assert!(parser.extensions().contains(&"erf"));
    }

    #[test]
    fn test_can_parse_returns_true_for_tiff() {
        let parser = ErfParser::new();
        let data = make_tiff_header("EPSON");
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_parse_epson() {
        let parser = ErfParser::new();
        let data = make_tiff_header("EPSON");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "ERF");
        assert_eq!(meta.exif.get_str("Make"), Some("EPSON"));
    }

    #[test]
    fn test_parse_non_epson_stays_tiff() {
        let parser = ErfParser::new();
        let data = make_tiff_header("Canon");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        // Non-Epson files stay as TIFF
        assert_eq!(meta.format, "TIFF");
    }
}

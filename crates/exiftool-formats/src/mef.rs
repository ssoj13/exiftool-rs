//! MEF (Mamiya RAW) format parser.
//!
//! MEF is Mamiya's RAW format used in their medium format cameras.
//! It's based on TIFF structure.

use crate::{FormatParser, Metadata, ReadSeek, Result, TiffParser};

/// MEF format parser.
pub struct MefParser {
    tiff: TiffParser,
}

impl MefParser {
    pub fn new() -> Self {
        Self {
            tiff: TiffParser::default(),
        }
    }
}

impl Default for MefParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for MefParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // MEF uses standard TIFF magic - detected by extension
        self.tiff.can_parse(header)
    }

    fn format_name(&self) -> &'static str {
        "MEF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["mef"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = self.tiff.parse(reader)?;
        
        // Check if this is actually a Mamiya file
        if let Some(make) = meta.exif.get_str("Make") {
            let make_upper = make.to_uppercase();
            if make_upper.contains("MAMIYA") || make_upper.contains("LEAF") {
                meta.format = "MEF";
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
        let parser = MefParser::new();
        assert_eq!(parser.format_name(), "MEF");
        assert!(parser.extensions().contains(&"mef"));
    }

    #[test]
    fn test_parse_mamiya() {
        let parser = MefParser::new();
        let data = make_tiff_header("Mamiya-OP Co.,Ltd.");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "MEF");
    }

    #[test]
    fn test_parse_leaf() {
        let parser = MefParser::new();
        let data = make_tiff_header("Leaf");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "MEF");
    }

    #[test]
    fn test_parse_non_mamiya() {
        let parser = MefParser::new();
        let data = make_tiff_header("Canon");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "TIFF");
    }
}

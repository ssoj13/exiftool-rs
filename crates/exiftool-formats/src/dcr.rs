//! DCR/KDC/K25 (Kodak RAW) format parsers.
//!
//! Kodak used several RAW formats:
//! - DCR: Kodak Digital Camera Raw (pro cameras)
//! - KDC: Kodak Digital Camera (consumer cameras)
//! - K25: Kodak DC25 format (early digital cameras)
//!
//! All are TIFF-based with Kodak-specific tags.

use crate::{FormatParser, Metadata, ReadSeek, Result, TiffParser};

/// DCR format parser (Kodak pro RAW).
pub struct DcrParser {
    tiff: TiffParser,
}

impl DcrParser {
    pub fn new() -> Self {
        Self {
            tiff: TiffParser::default(),
        }
    }
}

impl Default for DcrParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for DcrParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        self.tiff.can_parse(header)
    }

    fn format_name(&self) -> &'static str {
        "DCR"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["dcr"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = self.tiff.parse(reader)?;
        
        if let Some(make) = meta.exif.get_str("Make") {
            if make.to_uppercase().contains("KODAK") {
                meta.format = "DCR";
            }
        }
        
        Ok(meta)
    }
}

/// KDC format parser (Kodak consumer RAW).
pub struct KdcParser {
    tiff: TiffParser,
}

impl KdcParser {
    pub fn new() -> Self {
        Self {
            tiff: TiffParser::default(),
        }
    }
}

impl Default for KdcParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for KdcParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        self.tiff.can_parse(header)
    }

    fn format_name(&self) -> &'static str {
        "KDC"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["kdc"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = self.tiff.parse(reader)?;
        
        if let Some(make) = meta.exif.get_str("Make") {
            if make.to_uppercase().contains("KODAK") {
                meta.format = "KDC";
            }
        }
        
        Ok(meta)
    }
}

/// K25 format parser (Kodak DC25).
pub struct K25Parser {
    tiff: TiffParser,
}

impl K25Parser {
    pub fn new() -> Self {
        Self {
            tiff: TiffParser::default(),
        }
    }
}

impl Default for K25Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for K25Parser {
    fn can_parse(&self, header: &[u8]) -> bool {
        self.tiff.can_parse(header)
    }

    fn format_name(&self) -> &'static str {
        "K25"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["k25"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = self.tiff.parse(reader)?;
        
        if let Some(make) = meta.exif.get_str("Make") {
            if make.to_uppercase().contains("KODAK") {
                meta.format = "K25";
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
    fn test_dcr_format_info() {
        let parser = DcrParser::new();
        assert_eq!(parser.format_name(), "DCR");
        assert!(parser.extensions().contains(&"dcr"));
    }

    #[test]
    fn test_dcr_parse_kodak() {
        let parser = DcrParser::new();
        let data = make_tiff_header("EASTMAN KODAK COMPANY");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "DCR");
    }

    #[test]
    fn test_kdc_format_info() {
        let parser = KdcParser::new();
        assert_eq!(parser.format_name(), "KDC");
        assert!(parser.extensions().contains(&"kdc"));
    }

    #[test]
    fn test_kdc_parse_kodak() {
        let parser = KdcParser::new();
        let data = make_tiff_header("Kodak");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "KDC");
    }

    #[test]
    fn test_k25_format_info() {
        let parser = K25Parser::new();
        assert_eq!(parser.format_name(), "K25");
        assert!(parser.extensions().contains(&"k25"));
    }

    #[test]
    fn test_k25_parse_kodak() {
        let parser = K25Parser::new();
        let data = make_tiff_header("KODAK");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "K25");
    }

    #[test]
    fn test_non_kodak_stays_tiff() {
        let parser = DcrParser::new();
        let data = make_tiff_header("Canon");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "TIFF");
    }
}

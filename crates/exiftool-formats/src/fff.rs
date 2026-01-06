//! 3FR/FFF (Hasselblad RAW) format parser.
//!
//! 3FR and FFF are Hasselblad's RAW formats.
//! Both are TIFF-based with Hasselblad-specific tags.
//!
//! - 3FR: Modern Hasselblad digital backs
//! - FFF: Older Imacon/Hasselblad format

use crate::{FormatParser, Metadata, ReadSeek, Result, TiffParser};
use exiftool_attrs::AttrValue;

/// 3FR/FFF format parser.
pub struct FffParser {
    tiff: TiffParser,
}

impl FffParser {
    /// Create new 3FR/FFF parser.
    pub fn new() -> Self {
        Self {
            tiff: TiffParser::default(),
        }
    }
}

impl Default for FffParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for FffParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // 3FR/FFF are TIFF-based
        if header.len() < 8 {
            return false;
        }

        // TIFF magic check
        let is_tiff = match &header[0..4] {
            [0x49, 0x49, 0x2A, 0x00] => true, // II + 42 LE
            [0x4D, 0x4D, 0x00, 0x2A] => true, // MM + 42 BE
            _ => false,
        };

        if !is_tiff {
            return false;
        }

        // Detection is primarily by extension
        false
    }

    fn format_name(&self) -> &'static str {
        "3FR"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["3fr", "fff"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        // Parse as TIFF first
        let mut meta = self.tiff.parse(reader)?;

        // Determine format
        let format = detect_hasselblad_format(&meta);
        meta.format = format;

        meta.exif.set("File:FileType", AttrValue::Str(format.to_string()));
        meta.exif.set(
            "File:MIMEType",
            AttrValue::Str(format!("image/x-hasselblad-{}", format.to_lowercase())),
        );

        // Verify/set Make
        let is_hasselblad = meta
            .exif
            .get_str("Make")
            .map(|m| {
                let lower = m.to_lowercase();
                lower.contains("hasselblad") || lower.contains("imacon")
            })
            .unwrap_or(false);

        if !is_hasselblad {
            if meta.exif.get_str("Make").is_none() {
                meta.exif.set("Make", AttrValue::Str("Hasselblad".to_string()));
            }
        }

        Ok(meta)
    }
}

/// Detect whether this is 3FR or FFF.
fn detect_hasselblad_format(meta: &Metadata) -> &'static str {
    if let Some(make) = meta.exif.get_str("Make") {
        let lower = make.to_lowercase();
        if lower.contains("imacon") {
            return "FFF";
        }
    }
    // Default to 3FR (modern format)
    "3FR"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_tiff_header_with_make(make: &str) -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // TIFF header (big-endian, common for Hasselblad)
        data[0..4].copy_from_slice(&[0x4D, 0x4D, 0x00, 0x2A]);
        // IFD offset: 8
        data[4..8].copy_from_slice(&8u32.to_be_bytes());
        // IFD entry count: 1
        data[8..10].copy_from_slice(&1u16.to_be_bytes());
        // Tag: Make (0x010F)
        data[10..12].copy_from_slice(&0x010Fu16.to_be_bytes());
        data[12..14].copy_from_slice(&2u16.to_be_bytes()); // ASCII
        let make_len = make.len() + 1;
        data[14..18].copy_from_slice(&(make_len as u32).to_be_bytes());
        data[18..22].copy_from_slice(&200u32.to_be_bytes()); // offset
        // Next IFD: 0
        data[22..26].copy_from_slice(&0u32.to_be_bytes());
        // Make value
        let make_bytes = make.as_bytes();
        data[200..200 + make_bytes.len()].copy_from_slice(make_bytes);
        data[200 + make_bytes.len()] = 0;
        data
    }

    #[test]
    fn test_format_info() {
        let parser = FffParser::new();
        assert_eq!(parser.format_name(), "3FR");
        assert!(parser.extensions().contains(&"3fr"));
        assert!(parser.extensions().contains(&"fff"));
    }

    #[test]
    fn test_parse_3fr() {
        let parser = FffParser::new();
        let data = make_tiff_header_with_make("Hasselblad");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "3FR");
        assert_eq!(meta.exif.get_str("Make"), Some("Hasselblad"));
    }

    #[test]
    fn test_parse_fff() {
        let parser = FffParser::new();
        let data = make_tiff_header_with_make("Imacon");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "FFF");
    }

    #[test]
    fn test_can_parse_returns_false() {
        let parser = FffParser::new();
        let data = make_tiff_header_with_make("Hasselblad");
        assert!(!parser.can_parse(&data));
    }
}

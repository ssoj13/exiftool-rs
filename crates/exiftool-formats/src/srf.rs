//! SRF/SR2 (Sony RAW) format parser.
//!
//! SRF (Sony RAW Format) and SR2 (Sony RAW 2) are Sony's legacy RAW formats.
//! Both are TIFF-based with Sony-specific tags.
//!
//! - SRF: Used by early Sony DSC cameras
//! - SR2: Used by Sony Alpha cameras before ARW
//!
//! Detection: TIFF header + Sony make + .srf/.sr2 extension

use crate::{FormatParser, Metadata, ReadSeek, Result, TiffParser};
use exiftool_attrs::AttrValue;

/// SRF/SR2 format parser.
pub struct SrfParser {
    tiff: TiffParser,
}

impl SrfParser {
    /// Create new SRF/SR2 parser.
    pub fn new() -> Self {
        Self {
            tiff: TiffParser::default(),
        }
    }
}

impl Default for SrfParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for SrfParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // SRF/SR2 are TIFF-based
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

        // SRF/SR2 detection is primarily by extension
        // Magic alone can't distinguish from regular TIFF/ARW
        false
    }

    fn format_name(&self) -> &'static str {
        "SR2"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["srf", "sr2"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        // Parse as TIFF first
        let mut meta = self.tiff.parse(reader)?;

        // Determine format from actual content or default to SR2
        let format = detect_sony_format(&meta);
        meta.format = format;

        meta.exif.set("File:FileType", AttrValue::Str(format.to_string()));
        meta.exif.set(
            "File:MIMEType",
            AttrValue::Str(format!("image/x-sony-{}", format.to_lowercase())),
        );

        // Verify this is actually Sony
        let is_sony = meta
            .exif
            .get_str("Make")
            .map(|m| m.to_lowercase().contains("sony"))
            .unwrap_or(false);

        if !is_sony {
            if meta.exif.get_str("Make").is_none() {
                meta.exif.set("Make", AttrValue::Str("SONY".to_string()));
            }
        }

        Ok(meta)
    }
}

/// Detect whether this is SRF or SR2 based on metadata.
fn detect_sony_format(meta: &Metadata) -> &'static str {
    // Check for SR2-specific tags
    // SR2 typically has more advanced metadata
    if let Some(model) = meta.exif.get_str("Model") {
        let model_lower = model.to_lowercase();
        // Early DSC models use SRF
        if model_lower.contains("dsc-") {
            // DSC-F828, DSC-R1, DSC-V3 use SRF
            if model_lower.contains("f828") || model_lower.contains("r1") || model_lower.contains("v3") {
                return "SRF";
            }
        }
        // Alpha models typically use SR2 or ARW
        if model_lower.contains("dslr-a") || model_lower.contains("alpha") {
            return "SR2";
        }
    }
    // Default to SR2 (more common)
    "SR2"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_tiff_header_with_model(model: &str) -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // TIFF header (little-endian)
        data[0..4].copy_from_slice(&[0x49, 0x49, 0x2A, 0x00]);
        // IFD offset: 8
        data[4..8].copy_from_slice(&8u32.to_le_bytes());
        // IFD entry count: 2
        data[8..10].copy_from_slice(&2u16.to_le_bytes());

        // Tag 1: Make (0x010F)
        data[10..12].copy_from_slice(&0x010Fu16.to_le_bytes());
        data[12..14].copy_from_slice(&2u16.to_le_bytes()); // ASCII
        data[14..18].copy_from_slice(&5u32.to_le_bytes()); // count
        data[18..22].copy_from_slice(&200u32.to_le_bytes()); // offset

        // Tag 2: Model (0x0110)
        data[22..24].copy_from_slice(&0x0110u16.to_le_bytes());
        data[24..26].copy_from_slice(&2u16.to_le_bytes()); // ASCII
        let model_len = model.len() + 1;
        data[26..30].copy_from_slice(&(model_len as u32).to_le_bytes());
        data[30..34].copy_from_slice(&250u32.to_le_bytes()); // offset

        // Next IFD: 0
        data[34..38].copy_from_slice(&0u32.to_le_bytes());

        // Make value at offset 200
        data[200..205].copy_from_slice(b"SONY\0");

        // Model value at offset 250
        let model_bytes = model.as_bytes();
        data[250..250 + model_bytes.len()].copy_from_slice(model_bytes);
        data[250 + model_bytes.len()] = 0; // null terminator

        data
    }

    #[test]
    fn test_format_info() {
        let parser = SrfParser::new();
        assert_eq!(parser.format_name(), "SR2");
        assert!(parser.extensions().contains(&"srf"));
        assert!(parser.extensions().contains(&"sr2"));
    }

    #[test]
    fn test_parse_sr2() {
        let parser = SrfParser::new();
        let data = make_tiff_header_with_model("DSLR-A100");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "SR2");
        assert_eq!(meta.exif.get_str("Make"), Some("SONY"));
        assert_eq!(meta.exif.get_str("Model"), Some("DSLR-A100"));
    }

    #[test]
    fn test_parse_srf() {
        let parser = SrfParser::new();
        let data = make_tiff_header_with_model("DSC-F828");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "SRF");
    }

    #[test]
    fn test_can_parse_returns_false() {
        // SRF/SR2 detection is by extension
        let parser = SrfParser::new();
        let data = make_tiff_header_with_model("DSC-F828");
        assert!(!parser.can_parse(&data));
    }
}

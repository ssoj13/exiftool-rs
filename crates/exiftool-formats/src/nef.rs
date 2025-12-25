//! Nikon NEF format parser.
//!
//! NEF (Nikon Electronic Format) is Nikon's RAW image format.
//! Structure: TIFF-based with Nikon-specific MakerNotes.
//!
//! Detection: Standard TIFF header, identified by Make="NIKON" in IFD0.
//! Since we can't easily check Make without parsing, we rely on file extension
//! and fall back to TiffParser for actual parsing.
//!
//! Nikon-specific features (STUB - needs implementation):
//! - NikonPreview sub-IFD
//! - Nikon MakerNotes decoding
//! - White balance data
//! - Lens data
//! - Shot info

use crate::{makernotes, FormatParser, Metadata, ReadSeek, Result, TiffConfig, TiffParser};

/// Nikon NEF format parser.
/// 
/// STUB: Currently wraps TiffParser. Full Nikon MakerNotes parsing
/// requires extensive reverse-engineering of Nikon's proprietary format.
pub struct NefParser {
    tiff_parser: TiffParser,
}

impl NefParser {
    pub fn new() -> Self {
        Self {
            tiff_parser: TiffParser::with_config(TiffConfig {
                format_name: "NEF",
                allowed_magic: &[42, 43],
                vendor: Some(makernotes::Vendor::Nikon),
            }),
        }
    }
}

impl Default for NefParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for NefParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // NEF uses standard TIFF header
        // We can't distinguish from TIFF without parsing IFD0 Make tag
        // So we only detect via extension in registry
        // This method is called for auto-detection which won't work for NEF
        // unless we read the Make tag
        
        // Check for TIFF signature first
        if header.len() < 8 {
            return false;
        }
        
        let is_tiff = (header[0] == b'I' && header[1] == b'I' && header[2] == 0x2A && header[3] == 0x00)
            || (header[0] == b'M' && header[1] == b'M' && header[2] == 0x00 && header[3] == 0x2A);
        
        if !is_tiff {
            return false;
        }
        
        // STUB: For proper detection, would need to parse IFD0 and check Make="NIKON"
        // For now, return false - detection happens via extension
        false
    }

    fn format_name(&self) -> &'static str {
        "NEF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["nef", "nrw"] // NRW is Nikon's compact camera RAW
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        // Parse as TIFF - all NEF metadata is in standard TIFF/EXIF tags
        let mut metadata = self.tiff_parser.parse(reader)?;
        
        // Override format name
        metadata.format = "NEF";
        
        // STUB: Additional Nikon-specific parsing would go here:
        // - Parse Nikon MakerNotes (tag 0x927C in EXIF sub-IFD)
        // - Extract Nikon-specific tags like:
        //   - Lens data
        //   - White balance coefficients
        //   - Shot info (shutter count, etc.)
        //   - Active D-Lighting setting
        //   - Picture control
        
        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_info() {
        let parser = NefParser::new();
        assert_eq!(parser.format_name(), "NEF");
        assert!(parser.extensions().contains(&"nef"));
        assert!(parser.extensions().contains(&"nrw"));
    }

    #[test]
    fn reject_jpeg() {
        let parser = NefParser::new();
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
    }

    #[test]
    fn reject_png() {
        let parser = NefParser::new();
        assert!(!parser.can_parse(&[0x89, b'P', b'N', b'G']));
    }
}

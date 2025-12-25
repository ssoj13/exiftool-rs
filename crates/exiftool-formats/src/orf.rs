//! Olympus ORF format parser.
//!
//! ORF (Olympus Raw Format) is Olympus/OM Digital's RAW format.
//! Structure: TIFF-based with Olympus-specific MakerNotes.
//!
//! Magic bytes: standard TIFF (0x2A) or Olympus special (0x4F52="OR", 0x5352="SR")

use crate::{makernotes, FormatParser, Metadata, ReadSeek, Result, TiffConfig, TiffParser};

/// Olympus ORF format parser.
pub struct OrfParser {
    tiff: TiffParser,
}

impl OrfParser {
    pub fn new() -> Self {
        Self {
            tiff: TiffParser::with_config(TiffConfig {
                format_name: "ORF",
                // Standard TIFF + Olympus special magic bytes
                allowed_magic: &[42, 43, 0x4F52, 0x5352],
                vendor: Some(makernotes::Vendor::Olympus),
            }),
        }
    }
}

impl Default for OrfParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for OrfParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 8 {
            return false;
        }
        
        // Check for Olympus special magic: IIRO or IIRS
        let is_orf_special = (header[0] == b'I' && header[1] == b'I' 
            && header[2] == b'R' && header[3] == b'O')
            || (header[0] == b'I' && header[1] == b'I' 
            && header[2] == b'R' && header[3] == b'S');
        
        if is_orf_special {
            return true;
        }
        
        // Standard TIFF magic - detect via extension only
        false
    }

    fn format_name(&self) -> &'static str {
        "ORF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["orf", "ori"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        self.tiff.parse(reader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_info() {
        let parser = OrfParser::new();
        assert_eq!(parser.format_name(), "ORF");
        assert!(parser.extensions().contains(&"orf"));
    }

    #[test]
    fn detect_orf_special() {
        let parser = OrfParser::new();
        assert!(parser.can_parse(b"IIRO\x08\x00\x00\x00"));
        assert!(parser.can_parse(b"IIRS\x08\x00\x00\x00"));
    }
}

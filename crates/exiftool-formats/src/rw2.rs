//! Panasonic RW2 format parser.
//!
//! RW2 is Panasonic/Leica's RAW image format.
//! Structure: TIFF-based with Panasonic-specific MakerNotes.
//!
//! Magic bytes: standard TIFF (0x2A) or Panasonic special (0x55)

use crate::{makernotes, FormatParser, Metadata, ReadSeek, Result, TiffConfig, TiffParser};

/// Panasonic RW2 format parser.
pub struct Rw2Parser {
    tiff: TiffParser,
}

impl Rw2Parser {
    pub fn new() -> Self {
        Self {
            tiff: TiffParser::with_config(TiffConfig {
                format_name: "RW2",
                // Standard TIFF + Panasonic 0x55 magic
                allowed_magic: &[42, 43, 0x55],
                vendor: Some(makernotes::Vendor::Panasonic),
            }),
        }
    }
}

impl Default for Rw2Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for Rw2Parser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 8 {
            return false;
        }
        
        // Panasonic uses 0x55 magic instead of standard 0x2A
        let is_panasonic = header[0] == b'I' && header[1] == b'I' 
            && header[2] == 0x55 && header[3] == 0x00;
        
        if is_panasonic {
            return true;
        }
        
        // Standard TIFF magic - detect via extension only
        false
    }

    fn format_name(&self) -> &'static str {
        "RW2"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["rw2", "raw"]
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
        let parser = Rw2Parser::new();
        assert_eq!(parser.format_name(), "RW2");
        assert!(parser.extensions().contains(&"rw2"));
    }

    #[test]
    fn detect_panasonic_magic() {
        let parser = Rw2Parser::new();
        assert!(parser.can_parse(b"II\x55\x00\x08\x00\x00\x00"));
    }
}

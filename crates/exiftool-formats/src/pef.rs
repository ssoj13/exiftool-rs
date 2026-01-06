//! Pentax PEF format parser.
//!
//! PEF (Pentax Electronic Format) is Pentax/Ricoh's RAW format.
//! Structure: TIFF-based with Pentax-specific MakerNotes.
//!
//! Detection: TIFF header, identified by Make containing "PENTAX" or "RICOH".
//! Modern Pentax cameras also support DNG as an alternative.

use crate::{makernotes, FormatParser, Metadata, ReadSeek, Result, TiffConfig, TiffParser};

/// Pentax PEF format parser.
pub struct PefParser {
    tiff: TiffParser,
}

impl PefParser {
    pub fn new() -> Self {
        Self {
            tiff: TiffParser::with_config(TiffConfig {
                format_name: "PEF",
                allowed_magic: &[42, 43],
                vendor: Some(makernotes::Vendor::Pentax),
            }),
        }
    }
}

impl Default for PefParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for PefParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 8 {
            return false;
        }
        
        // PEF uses standard TIFF header (usually little-endian)
        let is_tiff = (header[0] == b'I' && header[1] == b'I' && header[2] == 0x2A && header[3] == 0x00)
            || (header[0] == b'M' && header[1] == b'M' && header[2] == 0x00 && header[3] == 0x2A);
        
        // Can't distinguish from TIFF without parsing Make
        if is_tiff {
            return false;  // Detect via extension
        }
        
        false
    }

    fn format_name(&self) -> &'static str {
        "PEF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["pef"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = self.tiff.parse(reader)?;
        metadata.format = "PEF";
        
        // Pentax MakerNotes contain:
        // - PentaxModelID
        // - LensInfo
        // - ShotInfo
        // - ColorInfo
        // - FlashInfo
        // - AEInfo
        // - LensData
        // - CustomSettings
        
        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_info() {
        let parser = PefParser::new();
        assert_eq!(parser.format_name(), "PEF");
        assert!(parser.extensions().contains(&"pef"));
    }
}

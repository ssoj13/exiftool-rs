//! Sony ARW format parser.
//!
//! ARW (Alpha RAW) is Sony's RAW image format.
//! Structure: TIFF-based with Sony-specific MakerNotes.
//!
//! Detection: Standard TIFF header, identified by Make containing "SONY".
//! Supports both ARW and SRF (older Sony format) files.

use crate::{makernotes, FormatParser, Metadata, ReadSeek, Result, TiffConfig, TiffParser};

/// Sony ARW format parser.
pub struct ArwParser {
    tiff: TiffParser,
}

impl ArwParser {
    pub fn new() -> Self {
        Self {
            tiff: TiffParser::with_config(TiffConfig {
                format_name: "ARW",
                allowed_magic: &[42, 43],
                vendor: Some(makernotes::Vendor::Sony),
            }),
        }
    }
}

impl Default for ArwParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for ArwParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // ARW uses standard TIFF header
        if header.len() < 8 {
            return false;
        }
        
        // Check TIFF signature
        let is_tiff = (header[0] == b'I' && header[1] == b'I' && header[2] == 0x2A && header[3] == 0x00)
            || (header[0] == b'M' && header[1] == b'M' && header[2] == 0x00 && header[3] == 0x2A);
        
        if !is_tiff {
            return false;
        }
        
        // Detection via extension - can't easily check Make without parsing
        false
    }

    fn format_name(&self) -> &'static str {
        "ARW"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["arw", "srf", "sr2"]  // ARW, SRF (old), SR2 (older)
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = self.tiff.parse(reader)?;
        metadata.format = "ARW";
        
        // Sony MakerNotes will be parsed via the MakerNotes module
        // which handles Sony-specific tags like:
        // - SonyModelID
        // - LensType
        // - AFInfo
        // - ColorTemperature
        // - DynamicRangeOptimizer
        // - CreativeStyle
        
        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_info() {
        let parser = ArwParser::new();
        assert_eq!(parser.format_name(), "ARW");
        assert!(parser.extensions().contains(&"arw"));
        assert!(parser.extensions().contains(&"srf"));
    }
}

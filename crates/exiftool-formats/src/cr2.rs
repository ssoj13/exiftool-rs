//! Canon CR2 (RAW) format parser.
//!
//! CR2 is TIFF-based with Canon-specific extensions:
//! - Standard TIFF header (II/MM + 0x002A)
//! - "CR" marker at offset 8-9 followed by version (0x02, 0x00)
//! - IFD0 -> IFD1 -> IFD2 -> IFD3 (RAW data)
//! - Embedded JPEG preview in IFD0
//! - Canon MakerNotes in EXIF sub-IFD
//!
//! We use TiffParser as base since structure is identical.

use crate::{makernotes, Error, FormatParser, Metadata, ReadSeek, Result, TiffConfig, TiffParser};

/// Canon CR2 format parser.
/// 
/// Extends TiffParser with Canon-specific detection and metadata.
pub struct Cr2Parser {
    tiff_parser: TiffParser,
}

impl Default for Cr2Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Cr2Parser {
    pub fn new() -> Self {
        Self {
            tiff_parser: TiffParser::with_config(TiffConfig {
                format_name: "CR2",
                allowed_magic: &[42, 43],
                vendor: Some(makernotes::Vendor::Canon),
            }),
        }
    }
}

impl FormatParser for Cr2Parser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 12 {
            return false;
        }

        // Must be valid TIFF first
        let is_tiff_le = header[0] == b'I' && header[1] == b'I' && header[2] == 0x2A && header[3] == 0x00;
        let is_tiff_be = header[0] == b'M' && header[1] == b'M' && header[2] == 0x00 && header[3] == 0x2A;

        if !is_tiff_le && !is_tiff_be {
            return false;
        }

        // CR2 has "CR" marker at offset 8 followed by version
        header[8] == b'C' && header[9] == b'R' && header[10] == 0x02
    }

    fn format_name(&self) -> &'static str {
        "CR2"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["cr2"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        // Read header to verify CR2
        let mut header = [0u8; 16];
        reader.read_exact(&mut header)?;

        if !self.can_parse(&header) {
            return Err(Error::InvalidStructure("not a valid CR2 file".into()));
        }

        // Extract CR2 version
        let cr2_major = header[10];
        let cr2_minor = header[11];

        // Reset to start and use TIFF parser
        reader.seek(std::io::SeekFrom::Start(0))?;
        let mut metadata = self.tiff_parser.parse(reader)?;

        // Update format to CR2
        metadata.format = "CR2";

        // Add CR2-specific info
        metadata.exif.set(
            "CR2Version",
            exiftool_attrs::AttrValue::Str(format!("{}.{}", cr2_major, cr2_minor)),
        );

        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_cr2() {
        let parser = Cr2Parser::new();
        // Little-endian TIFF with CR2 marker
        let header = [
            b'I', b'I', 0x2A, 0x00, // TIFF LE header
            0x10, 0x00, 0x00, 0x00, // IFD0 offset
            b'C', b'R', 0x02, 0x00, // CR2 marker + version
            0x00, 0x00, 0x00, 0x00,
        ];
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn reject_plain_tiff() {
        let parser = Cr2Parser::new();
        let header = [
            b'I', b'I', 0x2A, 0x00,
            0x08, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        assert!(!parser.can_parse(&header));
    }

    #[test]
    fn reject_jpeg() {
        let parser = Cr2Parser::new();
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0, 0, 0, 0, 0, 0, 0, 0, 0]));
    }
}

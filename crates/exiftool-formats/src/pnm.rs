//! PNM (Netpbm) format parser.
//!
//! Supports PPM, PGM, PBM, and PAM formats:
//! - P1: PBM ASCII (bitmap, 1-bit)
//! - P2: PGM ASCII (graymap)
//! - P3: PPM ASCII (pixmap/color)
//! - P4: PBM binary
//! - P5: PGM binary
//! - P6: PPM binary
//! - P7: PAM (Portable Arbitrary Map)
//!
//! Header structure:
//! - Magic number (P1-P7)
//! - Width, Height (ASCII decimal)
//! - Maxval (except PBM)
//! - Comments (lines starting with #)

use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::{BufRead, BufReader};

/// PNM format parser (PPM/PGM/PBM/PAM).
pub struct PnmParser;

/// PNM format variant.
#[derive(Debug, Clone, Copy, PartialEq)]
enum PnmType {
    PbmAscii,   // P1
    PgmAscii,   // P2
    PpmAscii,   // P3
    PbmBinary,  // P4
    PgmBinary,  // P5
    PpmBinary,  // P6
    Pam,        // P7
}

impl PnmType {
    fn from_magic(magic: &[u8]) -> Option<Self> {
        if magic.len() < 2 || magic[0] != b'P' {
            return None;
        }
        match magic[1] {
            b'1' => Some(Self::PbmAscii),
            b'2' => Some(Self::PgmAscii),
            b'3' => Some(Self::PpmAscii),
            b'4' => Some(Self::PbmBinary),
            b'5' => Some(Self::PgmBinary),
            b'6' => Some(Self::PpmBinary),
            b'7' => Some(Self::Pam),
            _ => None,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::PbmAscii | Self::PbmBinary => "PBM",
            Self::PgmAscii | Self::PgmBinary => "PGM",
            Self::PpmAscii | Self::PpmBinary => "PPM",
            Self::Pam => "PAM",
        }
    }

    fn color_type(&self) -> &'static str {
        match self {
            Self::PbmAscii | Self::PbmBinary => "Bitmap",
            Self::PgmAscii | Self::PgmBinary => "Grayscale",
            Self::PpmAscii | Self::PpmBinary => "RGB",
            Self::Pam => "Arbitrary",
        }
    }

    fn encoding(&self) -> &'static str {
        match self {
            Self::PbmAscii | Self::PgmAscii | Self::PpmAscii => "ASCII",
            Self::PbmBinary | Self::PgmBinary | Self::PpmBinary | Self::Pam => "Binary",
        }
    }

    fn bits_per_sample(&self, maxval: u32) -> u32 {
        match self {
            Self::PbmAscii | Self::PbmBinary => 1,
            _ => {
                if maxval <= 255 { 8 }
                else if maxval <= 65535 { 16 }
                else { 32 }
            }
        }
    }

    fn samples_per_pixel(&self) -> u32 {
        match self {
            Self::PbmAscii | Self::PbmBinary => 1,
            Self::PgmAscii | Self::PgmBinary => 1,
            Self::PpmAscii | Self::PpmBinary => 3,
            Self::Pam => 0, // Determined by DEPTH in PAM header
        }
    }
}

impl FormatParser for PnmParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        PnmType::from_magic(header).is_some()
    }

    fn format_name(&self) -> &'static str {
        "PNM"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["ppm", "pgm", "pbm", "pnm", "pam"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        // Read magic number
        let mut magic = [0u8; 2];
        reader.read_exact(&mut magic)?;

        let pnm_type = PnmType::from_magic(&magic)
            .ok_or_else(|| Error::InvalidStructure("Invalid PNM magic".into()))?;

        let mut metadata = Metadata::new(pnm_type.name());

        // Set format info
        metadata.set_file_type(pnm_type.name(), "image/x-portable-anymap");
        metadata.exif.set("File:ColorType", AttrValue::Str(pnm_type.color_type().to_string()));
        metadata.exif.set("File:Encoding", AttrValue::Str(pnm_type.encoding().to_string()));

        if pnm_type == PnmType::Pam {
            self.parse_pam(reader, &mut metadata)?;
        } else {
            self.parse_pnm(reader, pnm_type, &mut metadata)?;
        }

        Ok(metadata)
    }
}

impl PnmParser {
    /// Parse standard PNM (P1-P6) header.
    fn parse_pnm(&self, reader: &mut dyn ReadSeek, pnm_type: PnmType, metadata: &mut Metadata) -> Result<()> {
        let mut buf_reader = BufReader::new(reader);
        let mut comments = Vec::new();

        // Read width
        let width = self.read_value(&mut buf_reader, &mut comments)?;
        // Read height
        let height = self.read_value(&mut buf_reader, &mut comments)?;

        metadata.exif.set("File:ImageWidth", AttrValue::Int(width as i32));
        metadata.exif.set("File:ImageHeight", AttrValue::Int(height as i32));

        // Read maxval (not for PBM)
        let maxval = match pnm_type {
            PnmType::PbmAscii | PnmType::PbmBinary => 1,
            _ => self.read_value(&mut buf_reader, &mut comments)?,
        };

        if maxval > 1 {
            metadata.exif.set("File:MaxValue", AttrValue::Int(maxval as i32));
        }

        // Calculate bits
        let bits = pnm_type.bits_per_sample(maxval);
        let samples = pnm_type.samples_per_pixel();

        metadata.exif.set("File:BitsPerSample", AttrValue::Int(bits as i32));
        metadata.exif.set("File:SamplesPerPixel", AttrValue::Int(samples as i32));

        // Store comments
        if !comments.is_empty() {
            let joined = comments.join("\n");
            metadata.exif.set("File:Comment", AttrValue::Str(joined));
        }

        Ok(())
    }

    /// Parse PAM (P7) header.
    fn parse_pam(&self, reader: &mut dyn ReadSeek, metadata: &mut Metadata) -> Result<()> {
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();

        let mut width: Option<u32> = None;
        let mut height: Option<u32> = None;
        let mut depth: Option<u32> = None;
        let mut maxval: Option<u32> = None;
        let mut tupltype: Option<String> = None;
        let mut comments = Vec::new();

        // Read until ENDHDR
        loop {
            line.clear();
            if buf_reader.read_line(&mut line)? == 0 {
                return Err(Error::InvalidStructure("Unexpected EOF in PAM header".into()));
            }

            let trimmed = line.trim();

            if let Some(comment) = trimmed.strip_prefix('#') {
                comments.push(comment.trim().to_string());
                continue;
            }

            if trimmed == "ENDHDR" {
                break;
            }

            // Parse keyword value pairs
            let parts: Vec<&str> = trimmed.splitn(2, char::is_whitespace).collect();
            if parts.len() == 2 {
                let key = parts[0];
                let val = parts[1].trim();

                match key {
                    "WIDTH" => width = val.parse().ok(),
                    "HEIGHT" => height = val.parse().ok(),
                    "DEPTH" => depth = val.parse().ok(),
                    "MAXVAL" => maxval = val.parse().ok(),
                    "TUPLTYPE" => tupltype = Some(val.to_string()),
                    _ => {}
                }
            }
        }

        // Set metadata
        if let Some(w) = width {
            metadata.exif.set("File:ImageWidth", AttrValue::Int(w as i32));
        }
        if let Some(h) = height {
            metadata.exif.set("File:ImageHeight", AttrValue::Int(h as i32));
        }
        if let Some(d) = depth {
            metadata.exif.set("File:SamplesPerPixel", AttrValue::Int(d as i32));
        }
        if let Some(m) = maxval {
            metadata.exif.set("File:MaxValue", AttrValue::Int(m as i32));
            let bits = if m <= 255 { 8 } else if m <= 65535 { 16 } else { 32 };
            metadata.exif.set("File:BitsPerSample", AttrValue::Int(bits));
        }
        if let Some(tt) = tupltype {
            let color_type = match tt.as_str() {
                "BLACKANDWHITE" => "Bitmap",
                "GRAYSCALE" => "Grayscale",
                "GRAYSCALE_ALPHA" => "Grayscale+Alpha",
                "RGB" => "RGB",
                "RGB_ALPHA" => "RGBA",
                _ => &tt,
            };
            metadata.exif.set("File:ColorType", AttrValue::Str(color_type.to_string()));
        }

        if !comments.is_empty() {
            metadata.exif.set("File:Comment", AttrValue::Str(comments.join("\n")));
        }

        Ok(())
    }

    /// Read next ASCII decimal value, skipping whitespace and comments.
    fn read_value<R: BufRead>(&self, reader: &mut R, comments: &mut Vec<String>) -> Result<u32> {
        let mut result = String::new();
        let mut in_comment = false;
        let mut comment_buf = String::new();

        loop {
            let mut byte = [0u8; 1];
            if reader.read(&mut byte)? == 0 {
                break;
            }

            let ch = byte[0] as char;

            if in_comment {
                if ch == '\n' || ch == '\r' {
                    if !comment_buf.is_empty() {
                        comments.push(comment_buf.trim().to_string());
                        comment_buf.clear();
                    }
                    in_comment = false;
                } else {
                    comment_buf.push(ch);
                }
                continue;
            }

            if ch == '#' {
                in_comment = true;
                continue;
            }

            if ch.is_ascii_whitespace() {
                if !result.is_empty() {
                    break;
                }
                continue;
            }

            if ch.is_ascii_digit() {
                result.push(ch);
            } else {
                return Err(Error::InvalidStructure(format!("Invalid character in PNM header: {:?}", ch)));
            }
        }

        result.parse()
            .map_err(|_| Error::InvalidStructure("Failed to parse PNM value".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_can_parse() {
        let parser = PnmParser;
        assert!(parser.can_parse(b"P1"));
        assert!(parser.can_parse(b"P2 test"));
        assert!(parser.can_parse(b"P3"));
        assert!(parser.can_parse(b"P4"));
        assert!(parser.can_parse(b"P5"));
        assert!(parser.can_parse(b"P6"));
        assert!(parser.can_parse(b"P7"));
        assert!(!parser.can_parse(b"P8"));
        assert!(!parser.can_parse(b"BM"));
        assert!(!parser.can_parse(b""));
    }

    #[test]
    fn test_parse_pgm_ascii() {
        let data = b"P2\n# Test comment\n4 3\n255\n";
        let mut cursor = Cursor::new(data.as_slice());
        let parser = PnmParser;

        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.exif.get_i32("File:ImageWidth"), Some(4));
        assert_eq!(meta.exif.get_i32("File:ImageHeight"), Some(3));
        assert_eq!(meta.exif.get_i32("File:MaxValue"), Some(255));
        assert_eq!(meta.exif.get_str("File:ColorType"), Some("Grayscale"));
        assert_eq!(meta.exif.get_str("File:Encoding"), Some("ASCII"));
        assert_eq!(meta.exif.get_str("File:Comment"), Some("Test comment"));
    }

    #[test]
    fn test_parse_ppm_binary() {
        let data = b"P6\n640 480\n255\n";
        let mut cursor = Cursor::new(data.as_slice());
        let parser = PnmParser;

        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.exif.get_i32("File:ImageWidth"), Some(640));
        assert_eq!(meta.exif.get_i32("File:ImageHeight"), Some(480));
        assert_eq!(meta.exif.get_str("File:ColorType"), Some("RGB"));
        assert_eq!(meta.exif.get_str("File:Encoding"), Some("Binary"));
        assert_eq!(meta.exif.get_i32("File:SamplesPerPixel"), Some(3));
    }

    #[test]
    fn test_parse_pbm() {
        let data = b"P1\n8 8\n";
        let mut cursor = Cursor::new(data.as_slice());
        let parser = PnmParser;

        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.exif.get_i32("File:ImageWidth"), Some(8));
        assert_eq!(meta.exif.get_i32("File:ImageHeight"), Some(8));
        assert_eq!(meta.exif.get_str("File:ColorType"), Some("Bitmap"));
        assert_eq!(meta.exif.get_i32("File:BitsPerSample"), Some(1));
    }

    #[test]
    fn test_parse_pam() {
        let data = b"P7\nWIDTH 320\nHEIGHT 240\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n";
        let mut cursor = Cursor::new(data.as_slice());
        let parser = PnmParser;

        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.exif.get_i32("File:ImageWidth"), Some(320));
        assert_eq!(meta.exif.get_i32("File:ImageHeight"), Some(240));
        assert_eq!(meta.exif.get_i32("File:SamplesPerPixel"), Some(4));
        assert_eq!(meta.exif.get_str("File:ColorType"), Some("RGBA"));
    }

    #[test]
    fn test_parse_with_multiple_comments() {
        let data = b"P5\n# Comment 1\n# Comment 2\n100 100\n# Comment 3\n65535\n";
        let mut cursor = Cursor::new(data.as_slice());
        let parser = PnmParser;

        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.exif.get_i32("File:ImageWidth"), Some(100));
        assert_eq!(meta.exif.get_i32("File:MaxValue"), Some(65535));
        assert_eq!(meta.exif.get_i32("File:BitsPerSample"), Some(16));

        let comment = meta.exif.get_str("File:Comment").unwrap();
        assert!(comment.contains("Comment 1"));
        assert!(comment.contains("Comment 2"));
    }
}

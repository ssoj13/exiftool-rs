//! HDR (Radiance RGBE) format parser.
//!
//! Radiance HDR format structure:
//! - Magic: "#?" followed by format identifier (usually "RADIANCE" or "RGBE")
//! - Header: key=value pairs, one per line
//! - Empty line separator
//! - Resolution line: e.g. "-Y 512 +X 1024"
//! - Pixel data (RGBE or XYZE encoded)
//!
//! Common metadata:
//! - FORMAT: radiance format (32-bit_rle_rgbe, 32-bit_rle_xyze)
//! - EXPOSURE: exposure value
//! - SOFTWARE: software that created the file
//! - GAMMA: gamma correction value
//! - PRIMARIES: color primaries
//! - PIXASPECT: pixel aspect ratio
//! - VIEW: view parameters
//! - COLORCORR: color correction

use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::{BufRead, BufReader};

/// Radiance HDR magic signature.
#[allow(dead_code)]
const HDR_MAGIC: &[u8; 2] = b"#?";

/// HDR (Radiance RGBE) format parser.
pub struct HdrParser;

impl FormatParser for HdrParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // Must start with "#?" (Radiance signature)
        header.len() >= 2 && header[0] == b'#' && header[1] == b'?'
    }

    fn format_name(&self) -> &'static str {
        "HDR"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["hdr", "pic", "rgbe"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("HDR");

        let buf_reader = BufReader::new(reader);
        let mut lines = buf_reader.lines();

        // Read magic line
        let magic_line = lines
            .next()
            .ok_or(Error::InvalidStructure("Empty HDR file".into()))??;

        if !magic_line.starts_with("#?") {
            return Err(Error::InvalidStructure("Missing HDR magic".into()));
        }

        // Extract format identifier from magic line (e.g., "#?RADIANCE")
        let format_id = magic_line.trim_start_matches("#?");
        if !format_id.is_empty() {
            metadata.exif.set("FormatIdentifier", AttrValue::Str(format_id.to_string()));
        }

        // Parse header lines until empty line
        let mut found_resolution = false;
        for line_result in lines {
            let line = line_result?;

            // Empty line marks end of header
            if line.is_empty() {
                continue;
            }

            // Resolution line (last header line before pixel data)
            if line.starts_with('-') || line.starts_with('+') {
                if let Some((width, height)) = parse_resolution(&line) {
                    metadata.exif.set("ImageWidth", AttrValue::UInt(width));
                    metadata.exif.set("ImageHeight", AttrValue::UInt(height));
                    found_resolution = true;
                }
                break; // Resolution line is always last
            }

            // Comment lines
            if line.starts_with('#') {
                // Could store as comment, but usually just skip
                continue;
            }

            // Key=value pairs
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key.to_uppercase().as_str() {
                    "FORMAT" => {
                        metadata.exif.set("Format", AttrValue::Str(value.to_string()));
                    }
                    "EXPOSURE" => {
                        if let Ok(exp) = value.parse::<f32>() {
                            metadata.exif.set("Exposure", AttrValue::Float(exp));
                        } else {
                            metadata.exif.set("Exposure", AttrValue::Str(value.to_string()));
                        }
                    }
                    "GAMMA" => {
                        if let Ok(gamma) = value.parse::<f32>() {
                            metadata.exif.set("Gamma", AttrValue::Float(gamma));
                        } else {
                            metadata.exif.set("Gamma", AttrValue::Str(value.to_string()));
                        }
                    }
                    "PIXASPECT" => {
                        if let Ok(aspect) = value.parse::<f32>() {
                            metadata.exif.set("PixelAspectRatio", AttrValue::Float(aspect));
                        } else {
                            metadata.exif.set("PixelAspectRatio", AttrValue::Str(value.to_string()));
                        }
                    }
                    "SOFTWARE" => {
                        metadata.exif.set("Software", AttrValue::Str(value.to_string()));
                    }
                    "PRIMARIES" => {
                        metadata.exif.set("Primaries", AttrValue::Str(value.to_string()));
                    }
                    "COLORCORR" => {
                        metadata.exif.set("ColorCorrection", AttrValue::Str(value.to_string()));
                    }
                    "VIEW" => {
                        metadata.exif.set("View", AttrValue::Str(value.to_string()));
                    }
                    _ => {
                        // Store unknown attributes with HDR: prefix
                        metadata.exif.set(
                            format!("HDR:{}", key),
                            AttrValue::Str(value.to_string()),
                        );
                    }
                }
            }
        }

        if !found_resolution {
            return Err(Error::InvalidStructure("Missing resolution in HDR file".into()));
        }

        Ok(metadata)
    }
}

/// Parse resolution string like "-Y 512 +X 1024" or "+X 1024 -Y 512"
fn parse_resolution(line: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 4 {
        return None;
    }

    let mut width = 0u32;
    let mut height = 0u32;

    // Parse axis pairs
    for i in (0..4).step_by(2) {
        let axis = parts[i];
        let value: u32 = parts.get(i + 1)?.parse().ok()?;

        if axis.ends_with('X') {
            width = value;
        } else if axis.ends_with('Y') {
            height = value;
        }
    }

    if width > 0 && height > 0 {
        Some((width, height))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_hdr() {
        let parser = HdrParser;
        assert!(parser.can_parse(b"#?RADIANCE\n"));
        assert!(parser.can_parse(b"#?RGBE\n"));
    }

    #[test]
    fn reject_jpeg() {
        let parser = HdrParser;
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
    }

    #[test]
    fn reject_exr() {
        let parser = HdrParser;
        assert!(!parser.can_parse(&[0x76, 0x2F, 0x31, 0x01]));
    }

    #[test]
    fn parse_resolution_string() {
        assert_eq!(parse_resolution("-Y 512 +X 1024"), Some((1024, 512)));
        assert_eq!(parse_resolution("+X 800 -Y 600"), Some((800, 600)));
        assert_eq!(parse_resolution("-Y 100 +X 200"), Some((200, 100)));
    }
}

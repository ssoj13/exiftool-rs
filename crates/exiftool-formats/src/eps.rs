//! EPS/PS (Encapsulated PostScript) format parser.
//!
//! EPS files are PostScript with DSC (Document Structuring Conventions) comments.
//! Structure:
//! - %!PS-Adobe header
//! - DSC comments (%%Title, %%Creator, %%BoundingBox, etc.)
//! - Optional XMP packet
//! - PostScript code

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// EPS/PS parser.
pub struct EpsParser;

impl FormatParser for EpsParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // Check for PostScript magic: %!PS or %!PS-Adobe
        if header.len() >= 4 && &header[0..4] == b"%!PS" {
            return true;
        }
        // Check for DOS EPS (binary header + PS)
        if header.len() >= 4 && header[0..4] == [0xC5, 0xD0, 0xD3, 0xC6] {
            return true;
        }
        false
    }

    fn format_name(&self) -> &'static str {
        "EPS"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["eps", "epsf", "epsi", "ps"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("EPS");

        reader.seek(SeekFrom::Start(0))?;
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        // Handle DOS EPS (binary header)
        let ps_data = if data.len() >= 4 && data[0..4] == [0xC5, 0xD0, 0xD3, 0xC6] {
            // DOS EPS header: 4-byte magic + 4-byte PS offset + 4-byte PS length
            if data.len() < 12 {
                return Ok(meta);
            }
            let ps_offset = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
            let ps_length = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;
            
            meta.exif.set("EPSType", AttrValue::Str("DOS EPS".into()));
            
            if ps_offset < data.len() && ps_offset + ps_length <= data.len() {
                &data[ps_offset..ps_offset + ps_length]
            } else if ps_offset < data.len() {
                &data[ps_offset..]
            } else {
                return Ok(meta);
            }
        } else {
            meta.exif.set("EPSType", AttrValue::Str("ASCII EPS".into()));
            &data[..]
        };

        // Parse DSC comments
        parse_dsc_comments(ps_data, &mut meta);

        // Look for XMP packet
        if let Some(xmp) = find_xmp_packet(ps_data) {
            if let Ok(xmp_str) = std::str::from_utf8(xmp) {
                meta.exif.set("XMP", AttrValue::Str(xmp_str.to_string()));
            }
        }

        Ok(meta)
    }
}

/// Parse DSC (Document Structuring Conventions) comments.
fn parse_dsc_comments(data: &[u8], meta: &mut Metadata) {
    // Convert to string for line parsing
    let text = String::from_utf8_lossy(&data[..data.len().min(32768)]);

    // Check PS version in header
    for line in text.lines().take(1) {
        if line.starts_with("%!PS-Adobe-") {
            if let Some(ver) = line.strip_prefix("%!PS-Adobe-") {
                let version = ver.split_whitespace().next().unwrap_or(ver);
                meta.exif.set("PSVersion", AttrValue::Str(version.to_string()));
            }
        }
    }

    // Parse DSC comments (limit to header area, ~8KB)
    let header_text = if text.len() > 8192 {
        &text[..8192]
    } else {
        &text
    };

    for line in header_text.lines() {
        if !line.starts_with("%%") {
            continue;
        }

        // Parse key: value format
        if let Some((key, value)) = parse_dsc_line(line) {
            match key {
                "Title" => {
                    meta.exif.set("Title", AttrValue::Str(clean_ps_string(value)));
                }
                "Creator" => {
                    meta.exif.set("Creator", AttrValue::Str(clean_ps_string(value)));
                }
                "CreationDate" => {
                    meta.exif.set("CreateDate", AttrValue::Str(value.to_string()));
                }
                "For" => {
                    meta.exif.set("For", AttrValue::Str(clean_ps_string(value)));
                }
                "BoundingBox" => {
                    // Format: llx lly urx ury
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    if parts.len() >= 4 {
                        if let (Ok(llx), Ok(lly), Ok(urx), Ok(ury)) = (
                            parts[0].parse::<i32>(),
                            parts[1].parse::<i32>(),
                            parts[2].parse::<i32>(),
                            parts[3].parse::<i32>(),
                        ) {
                            let width = urx - llx;
                            let height = ury - lly;
                            meta.exif.set("ImageWidth", AttrValue::UInt(width as u32));
                            meta.exif.set("ImageHeight", AttrValue::UInt(height as u32));
                            meta.exif.set("BoundingBox", AttrValue::Str(value.to_string()));
                        }
                    }
                }
                "HiResBoundingBox" => {
                    meta.exif.set("HiResBoundingBox", AttrValue::Str(value.to_string()));
                }
                "Pages" => {
                    if let Ok(pages) = value.parse::<u32>() {
                        meta.exif.set("Pages", AttrValue::UInt(pages));
                    }
                }
                "PageOrder" => {
                    meta.exif.set("PageOrder", AttrValue::Str(value.to_string()));
                }
                "DocumentData" => {
                    meta.exif.set("DocumentData", AttrValue::Str(value.to_string()));
                }
                "LanguageLevel" => {
                    if let Ok(level) = value.parse::<u32>() {
                        meta.exif.set("LanguageLevel", AttrValue::UInt(level));
                    }
                }
                "Copyright" => {
                    meta.exif.set("Copyright", AttrValue::Str(clean_ps_string(value)));
                }
                "DocumentNeededResources" | "DocumentSuppliedResources" => {
                    // Skip multi-line resource lists
                }
                _ => {}
            }
        }

        // Stop at EndComments
        if line.starts_with("%%EndComments") {
            break;
        }
    }
}

/// Parse a DSC comment line into key-value pair.
fn parse_dsc_line(line: &str) -> Option<(&str, &str)> {
    let line = line.strip_prefix("%%")?;
    
    // Handle "Key: Value" format
    if let Some(pos) = line.find(':') {
        let key = line[..pos].trim();
        let value = line[pos + 1..].trim();
        if !key.is_empty() {
            return Some((key, value));
        }
    }
    
    None
}

/// Clean PostScript string (remove parentheses).
fn clean_ps_string(s: &str) -> String {
    let s = s.trim();
    if s.starts_with('(') && s.ends_with(')') {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Find XMP packet in PostScript data.
fn find_xmp_packet(data: &[u8]) -> Option<&[u8]> {
    // Look for XMP packet markers
    let xmp_start = b"<?xpacket begin=";
    let xmp_end = b"<?xpacket end=";
    
    // Find start
    let start_pos = data.windows(xmp_start.len())
        .position(|w| w == xmp_start)?;
    
    // Find end
    let search_area = &data[start_pos..];
    let end_pos = search_area.windows(xmp_end.len())
        .position(|w| w == xmp_end)?;
    
    // Find the closing ?>
    let end_area = &search_area[end_pos..];
    let close_pos = end_area.windows(2)
        .position(|w| w == b"?>")?;
    
    Some(&search_area[..end_pos + close_pos + 2])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_can_parse_ps() {
        let parser = EpsParser;
        assert!(parser.can_parse(b"%!PS-Adobe-3.0 EPSF-3.0"));
        assert!(parser.can_parse(b"%!PS\n"));
    }

    #[test]
    fn test_can_parse_dos_eps() {
        let parser = EpsParser;
        assert!(parser.can_parse(&[0xC5, 0xD0, 0xD3, 0xC6, 0x00]));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = EpsParser;
        assert!(!parser.can_parse(b"PDF-1.4"));
        assert!(!parser.can_parse(b"\x89PNG"));
    }

    #[test]
    fn test_format_info() {
        let parser = EpsParser;
        assert_eq!(parser.format_name(), "EPS");
        assert!(parser.extensions().contains(&"eps"));
        assert!(parser.extensions().contains(&"ps"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = EpsParser;

        let eps_data = b"%!PS-Adobe-3.0 EPSF-3.0
%%Title: (Test Document)
%%Creator: (Test App)
%%CreationDate: 2024-01-15
%%BoundingBox: 0 0 612 792
%%Pages: 1
%%EndComments
% PostScript code here
";

        let mut cursor = Cursor::new(&eps_data[..]);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "EPS");
        assert_eq!(meta.exif.get_str("PSVersion"), Some("3.0"));
        assert_eq!(meta.exif.get_str("Title"), Some("Test Document"));
        assert_eq!(meta.exif.get_str("Creator"), Some("Test App"));
        assert_eq!(meta.exif.get_u32("ImageWidth"), Some(612));
        assert_eq!(meta.exif.get_u32("ImageHeight"), Some(792));
        assert_eq!(meta.exif.get_u32("Pages"), Some(1));
    }

    #[test]
    fn test_parse_with_xmp() {
        let parser = EpsParser;

        let eps_data = b"%!PS-Adobe-3.0
%%Title: (XMP Test)
%%EndComments
<?xpacket begin=\"\" id=\"W5M0\"?><x:xmpmeta></x:xmpmeta><?xpacket end=\"w\"?>
";

        let mut cursor = Cursor::new(&eps_data[..]);
        let meta = parser.parse(&mut cursor).unwrap();

        assert!(meta.exif.get_str("XMP").is_some());
    }
}

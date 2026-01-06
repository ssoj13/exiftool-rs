//! AI (Adobe Illustrator) format parser.
//!
//! AI files come in two flavors:
//! - PDF-based (modern, since AI 9+): starts with %PDF
//! - EPS-based (legacy): starts with %!PS-Adobe
//!
//! Both contain Adobe Illustrator specific metadata.

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// Adobe Illustrator parser.
pub struct AiParser;

impl FormatParser for AiParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // Check for PDF-based AI or EPS-based AI
        // We need to look for AI-specific markers, not just PDF/EPS
        if header.len() < 32 {
            return false;
        }

        // PDF-based AI: starts with %PDF and contains AI markers
        if header.starts_with(b"%PDF") {
            // Quick heuristic: look for "Illustrator" in first bytes
            return header.windows(11).any(|w| w == b"Illustrator");
        }

        // EPS-based AI: %!PS-Adobe with AI markers
        if header.starts_with(b"%!PS") {
            return header.windows(11).any(|w| w == b"Illustrator");
        }

        false
    }

    fn format_name(&self) -> &'static str {
        "AI"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["ai", "ait"] // .ait = AI template
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("AI");

        reader.seek(SeekFrom::Start(0))?;
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        if data.len() < 32 {
            return Ok(meta);
        }

        // Detect AI type
        if data.starts_with(b"%PDF") {
            meta.exif.set("AIType", AttrValue::Str("PDF-based".into()));
            parse_pdf_ai(&data, &mut meta);
        } else if data.starts_with(b"%!PS") {
            meta.exif.set("AIType", AttrValue::Str("EPS-based".into()));
            parse_eps_ai(&data, &mut meta);
        }

        // Look for XMP
        if let Some(xmp) = find_xmp(&data) {
            if let Ok(xmp_str) = std::str::from_utf8(xmp) {
                meta.exif.set("XMP", AttrValue::Str(xmp_str.to_string()));
            }
        }

        Ok(meta)
    }
}

/// Parse PDF-based AI metadata.
fn parse_pdf_ai(data: &[u8], meta: &mut Metadata) {
    let text = String::from_utf8_lossy(&data[..data.len().min(65536)]);

    // Look for PDF version
    if let Some(line) = text.lines().next() {
        if let Some(ver) = line.strip_prefix("%PDF-") {
            meta.exif.set("PDFVersion", AttrValue::Str(ver.to_string()));
        }
    }

    // Look for Creator in PDF info dictionary
    // Format: /Creator (Adobe Illustrator...)
    if let Some(pos) = text.find("/Creator") {
        let after = &text[pos + 8..];
        if let Some(value) = extract_pdf_string(after) {
            meta.exif.set("Creator", AttrValue::Str(value));
        }
    }

    // Producer
    if let Some(pos) = text.find("/Producer") {
        let after = &text[pos + 9..];
        if let Some(value) = extract_pdf_string(after) {
            meta.exif.set("Producer", AttrValue::Str(value));
        }
    }

    // Title
    if let Some(pos) = text.find("/Title") {
        let after = &text[pos + 6..];
        if let Some(value) = extract_pdf_string(after) {
            meta.exif.set("Title", AttrValue::Str(value));
        }
    }

    // CreationDate
    if let Some(pos) = text.find("/CreationDate") {
        let after = &text[pos + 13..];
        if let Some(value) = extract_pdf_string(after) {
            meta.exif.set("CreateDate", AttrValue::Str(parse_pdf_date(&value)));
        }
    }

    // ModDate
    if let Some(pos) = text.find("/ModDate") {
        let after = &text[pos + 8..];
        if let Some(value) = extract_pdf_string(after) {
            meta.exif.set("ModifyDate", AttrValue::Str(parse_pdf_date(&value)));
        }
    }

    // Look for Illustrator version in AI private data
    // %%AI8_CreatorVersion: X.X
    for line in text.lines() {
        if line.starts_with("%%AI") && line.contains("CreatorVersion") {
            if let Some(pos) = line.find(':') {
                let version = line[pos + 1..].trim();
                meta.exif.set("AIVersion", AttrValue::Str(version.to_string()));
                break;
            }
        }
    }

    // Look for MediaBox/ArtBox for dimensions
    // /MediaBox [0 0 612 792]
    if let Some(pos) = text.find("/MediaBox") {
        let after = &text[pos + 9..];
        if let Some(bbox) = extract_pdf_array(after) {
            if let Some((w, h)) = parse_bbox(&bbox) {
                meta.exif.set("ImageWidth", AttrValue::UInt(w as u32));
                meta.exif.set("ImageHeight", AttrValue::UInt(h as u32));
            }
        }
    }
}

/// Parse EPS-based AI metadata.
fn parse_eps_ai(data: &[u8], meta: &mut Metadata) {
    let text = String::from_utf8_lossy(&data[..data.len().min(32768)]);

    // Parse DSC comments
    for line in text.lines() {
        if line.starts_with("%%Title:") {
            let value = line[8..].trim();
            meta.exif.set("Title", AttrValue::Str(clean_ps_string(value)));
        } else if line.starts_with("%%Creator:") {
            let value = line[10..].trim();
            meta.exif.set("Creator", AttrValue::Str(clean_ps_string(value)));
        } else if line.starts_with("%%CreationDate:") {
            let value = line[15..].trim();
            meta.exif.set("CreateDate", AttrValue::Str(value.to_string()));
        } else if line.starts_with("%%BoundingBox:") {
            let value = line[14..].trim();
            if let Some((w, h)) = parse_bbox(value) {
                meta.exif.set("ImageWidth", AttrValue::UInt(w as u32));
                meta.exif.set("ImageHeight", AttrValue::UInt(h as u32));
            }
        } else if line.starts_with("%%AI") && line.contains("CreatorVersion") {
            if let Some(pos) = line.find(':') {
                let version = line[pos + 1..].trim();
                meta.exif.set("AIVersion", AttrValue::Str(version.to_string()));
            }
        } else if line.starts_with("%%EndComments") {
            break;
        }
    }
}

/// Extract PDF string value (parentheses or hex).
fn extract_pdf_string(s: &str) -> Option<String> {
    let s = s.trim();
    if s.starts_with('(') {
        // Parenthesized string
        let mut depth = 0;
        let mut end = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = i;
                        break;
                    }
                }
                _ => {}
            }
        }
        if end > 1 {
            return Some(s[1..end].to_string());
        }
    } else if s.starts_with('<') {
        // Hex string
        if let Some(end) = s.find('>') {
            let hex = &s[1..end];
            // Decode hex to string
            let bytes: Vec<u8> = (0..hex.len())
                .step_by(2)
                .filter_map(|i| u8::from_str_radix(&hex[i..i + 2.min(hex.len() - i)], 16).ok())
                .collect();
            return String::from_utf8(bytes).ok();
        }
    }
    None
}

/// Extract PDF array value.
fn extract_pdf_array(s: &str) -> Option<String> {
    let s = s.trim();
    if let Some(start) = s.find('[') {
        if let Some(end) = s[start..].find(']') {
            return Some(s[start + 1..start + end].to_string());
        }
    }
    None
}

/// Parse bounding box "llx lly urx ury" or "0 0 w h".
fn parse_bbox(s: &str) -> Option<(f64, f64)> {
    let parts: Vec<f64> = s
        .split_whitespace()
        .filter_map(|p| p.parse().ok())
        .collect();
    
    if parts.len() >= 4 {
        let width = parts[2] - parts[0];
        let height = parts[3] - parts[1];
        return Some((width.abs(), height.abs()));
    }
    None
}

/// Parse PDF date format D:YYYYMMDDHHmmSS.
fn parse_pdf_date(s: &str) -> String {
    let s = s.trim().trim_start_matches("D:");
    if s.len() >= 8 {
        // YYYYMMDD -> YYYY-MM-DD
        let year = &s[0..4];
        let month = &s[4..6];
        let day = &s[6..8];
        
        if s.len() >= 14 {
            let hour = &s[8..10];
            let min = &s[10..12];
            let sec = &s[12..14];
            return format!("{}-{}-{} {}:{}:{}", year, month, day, hour, min, sec);
        }
        return format!("{}-{}-{}", year, month, day);
    }
    s.to_string()
}

/// Clean PostScript string.
fn clean_ps_string(s: &str) -> String {
    let s = s.trim();
    if s.starts_with('(') && s.ends_with(')') {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Find XMP packet.
fn find_xmp(data: &[u8]) -> Option<&[u8]> {
    let xmp_start = b"<?xpacket begin=";
    let xmp_end = b"<?xpacket end=";

    let start_pos = data.windows(xmp_start.len()).position(|w| w == xmp_start)?;
    let search_area = &data[start_pos..];
    let end_pos = search_area.windows(xmp_end.len()).position(|w| w == xmp_end)?;
    let end_area = &search_area[end_pos..];
    let close_pos = end_area.windows(2).position(|w| w == b"?>")?;

    Some(&search_area[..end_pos + close_pos + 2])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_can_parse_pdf_ai() {
        let parser = AiParser;
        // PDF-based AI needs "Illustrator" marker
        let mut header = b"%PDF-1.5 Adobe Illustrator".to_vec();
        header.resize(64, 0);
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn test_can_parse_eps_ai() {
        let parser = AiParser;
        let mut header = b"%!PS-Adobe-3.0 Illustrator".to_vec();
        header.resize(64, 0);
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn test_cannot_parse_plain_pdf() {
        let parser = AiParser;
        let mut header = b"%PDF-1.5 plain document".to_vec();
        header.resize(64, 0);
        assert!(!parser.can_parse(&header));
    }

    #[test]
    fn test_format_info() {
        let parser = AiParser;
        assert_eq!(parser.format_name(), "AI");
        assert!(parser.extensions().contains(&"ai"));
    }

    #[test]
    fn test_parse_pdf_ai() {
        let parser = AiParser;

        let ai_data = b"%PDF-1.5 Illustrator
1 0 obj
<<
/Creator (Adobe Illustrator 25.0)
/Title (Test Artwork)
/CreationDate (D:20240115120000)
/MediaBox [0 0 612 792]
>>
endobj
%%AI8_CreatorVersion: 25.0
";

        let mut cursor = Cursor::new(&ai_data[..]);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "AI");
        assert_eq!(meta.exif.get_str("AIType"), Some("PDF-based"));
        assert_eq!(meta.exif.get_str("Creator"), Some("Adobe Illustrator 25.0"));
        assert_eq!(meta.exif.get_str("Title"), Some("Test Artwork"));
    }

    #[test]
    fn test_parse_eps_ai() {
        let parser = AiParser;

        let ai_data = b"%!PS-Adobe-3.0 Illustrator
%%Title: (Legacy Artwork)
%%Creator: (Adobe Illustrator 8.0)
%%BoundingBox: 0 0 595 842
%%AI5_CreatorVersion: 8.0
%%EndComments
";

        let mut cursor = Cursor::new(&ai_data[..]);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "AI");
        assert_eq!(meta.exif.get_str("AIType"), Some("EPS-based"));
        assert_eq!(meta.exif.get_str("Title"), Some("Legacy Artwork"));
        assert_eq!(meta.exif.get_str("AIVersion"), Some("8.0"));
        assert_eq!(meta.exif.get_u32("ImageWidth"), Some(595));
        assert_eq!(meta.exif.get_u32("ImageHeight"), Some(842));
    }
}

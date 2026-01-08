//! Adobe PDF format parser.
//!
//! PDF metadata is stored in:
//! - Info dictionary (Title, Author, Subject, Keywords, Creator, Producer, dates)
//! - XMP metadata stream (richer metadata including Dublin Core, XMP Basic)
//!
//! Reference: PDF 1.7 Reference (ISO 32000-1:2008)

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::{AttrValue, Attrs};
use std::io::{Read, Seek};

/// PDF format parser.
pub struct PdfParser;

impl PdfParser {
    /// PDF magic: "%PDF-"
    const MAGIC: &'static [u8] = b"%PDF-";
    
    /// Parse PDF and extract metadata.
    fn parse_pdf<R: Read + Seek + ?Sized>(reader: &mut R) -> Result<(Attrs, Option<String>)> {
        let mut attrs = Attrs::new();
        let xmp;
        
        // Read entire file (PDF requires random access)
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        
        if data.len() < 8 {
            return Err(crate::Error::InvalidStructure("PDF too small".into()));
        }
        
        // Extract PDF version from header
        if let Some(version) = Self::parse_version(&data) {
            attrs.set("PDFVersion", AttrValue::Str(version));
        }
        
        // Find and parse Info dictionary
        Self::parse_info_dict(&data, &mut attrs);
        
        // Find and extract XMP metadata
        xmp = Self::find_xmp(&data);
        
        // Count pages if possible
        if let Some(page_count) = Self::count_pages(&data) {
            attrs.set("PageCount", AttrValue::UInt(page_count));
        }
        
        Ok((attrs, xmp))
    }
    
    /// Parse PDF version from header.
    fn parse_version(data: &[u8]) -> Option<String> {
        // Format: %PDF-1.7
        if data.len() >= 8 && &data[0..5] == b"%PDF-" {
            // Find end of version (newline)
            let end = data[5..].iter().position(|&b| b == b'\n' || b == b'\r')
                .map(|p| p + 5)
                .unwrap_or(8.min(data.len()));
            
            if let Ok(s) = std::str::from_utf8(&data[5..end]) {
                return Some(s.trim().to_string());
            }
        }
        None
    }
    
    /// Find and parse the Info dictionary.
    fn parse_info_dict(data: &[u8], attrs: &mut Attrs) {
        // Look for /Info reference in trailer
        // Format: /Info N 0 R  (where N is object number)
        if let Some(pos) = Self::find_bytes(data, b"/Info") {
            let remaining = &data[pos + 5..];
            
            // Parse object reference: " N 0 R"
            if let Some((obj_num, _gen)) = Self::parse_obj_ref(remaining) {
                // Find the object: "N 0 obj"
                let search = format!("{} 0 obj", obj_num);
                if let Some(obj_pos) = Self::find_bytes(data, search.as_bytes()) {
                    // Find dictionary start
                    if let Some(dict_start) = Self::find_bytes(&data[obj_pos..], b"<<") {
                        let dict_data = &data[obj_pos + dict_start..];
                        Self::parse_dict_entries(dict_data, attrs);
                    }
                }
            }
        }
    }
    
    /// Parse object reference "N G R" returning (obj_num, gen_num).
    fn parse_obj_ref(data: &[u8]) -> Option<(u32, u32)> {
        let s = std::str::from_utf8(data).ok()?;
        let mut parts = s.split_whitespace();
        let obj_num: u32 = parts.next()?.parse().ok()?;
        let gen_num: u32 = parts.next()?.parse().ok()?;
        let r = parts.next()?;
        if r == "R" {
            Some((obj_num, gen_num))
        } else {
            None
        }
    }
    
    /// Parse dictionary entries and extract metadata.
    fn parse_dict_entries(data: &[u8], attrs: &mut Attrs) {
        let mappings = [
            ("/Title", "Title"),
            ("/Author", "Author"),
            ("/Subject", "Subject"),
            ("/Keywords", "Keywords"),
            ("/Creator", "Creator"),
            ("/Producer", "Producer"),
            ("/CreationDate", "CreateDate"),
            ("/ModDate", "ModifyDate"),
        ];
        
        for (pdf_key, attr_name) in mappings {
            if let Some(value) = Self::extract_dict_value(data, pdf_key.as_bytes()) {
                attrs.set(attr_name, AttrValue::Str(value));
            }
        }
    }
    
    /// Extract a string value from a dictionary.
    fn extract_dict_value(data: &[u8], key: &[u8]) -> Option<String> {
        let pos = Self::find_bytes(data, key)?;
        let remaining = &data[pos + key.len()..];
        
        // Skip whitespace
        let start = remaining.iter().position(|&b| !b.is_ascii_whitespace())?;
        let value_data = &remaining[start..];
        
        // Check for string types
        if value_data.starts_with(b"(") {
            // Literal string: (text)
            Self::parse_literal_string(value_data)
        } else if value_data.starts_with(b"<") && !value_data.starts_with(b"<<") {
            // Hex string: <hex>
            Self::parse_hex_string(value_data)
        } else {
            None
        }
    }
    
    /// Parse PDF literal string: (text).
    fn parse_literal_string(data: &[u8]) -> Option<String> {
        if !data.starts_with(b"(") {
            return None;
        }
        
        let mut result = Vec::new();
        let mut depth = 0;
        let mut escape = false;
        
        for &b in &data[1..] {
            if escape {
                match b {
                    b'n' => result.push(b'\n'),
                    b'r' => result.push(b'\r'),
                    b't' => result.push(b'\t'),
                    b'b' => result.push(0x08),
                    b'f' => result.push(0x0C),
                    b'(' => result.push(b'('),
                    b')' => result.push(b')'),
                    b'\\' => result.push(b'\\'),
                    _ => result.push(b),
                }
                escape = false;
            } else if b == b'\\' {
                escape = true;
            } else if b == b'(' {
                depth += 1;
                result.push(b);
            } else if b == b')' {
                if depth == 0 {
                    break;
                }
                depth -= 1;
                result.push(b);
            } else {
                result.push(b);
            }
        }
        
        // Handle BOM for UTF-16BE or assume UTF-8/Latin1
        if result.len() >= 2 && result[0] == 0xFE && result[1] == 0xFF {
            // UTF-16BE with BOM
            Self::decode_utf16be(&result[2..])
        } else {
            // Try UTF-8 first, fall back to Latin1
            String::from_utf8(result.clone())
                .or_else(|_| Ok::<String, ()>(result.iter().map(|&b| b as char).collect()))
                .ok()
        }
    }
    
    /// Decode UTF-16BE bytes to String.
    fn decode_utf16be(data: &[u8]) -> Option<String> {
        if data.len() % 2 != 0 {
            return None;
        }
        
        let chars: Vec<u16> = data.chunks(2)
            .map(|c| u16::from_be_bytes([c[0], c[1]]))
            .collect();
        
        String::from_utf16(&chars).ok()
    }
    
    /// Parse PDF hex string: <hex>.
    fn parse_hex_string(data: &[u8]) -> Option<String> {
        if !data.starts_with(b"<") || data.starts_with(b"<<") {
            return None;
        }
        
        let end = data.iter().position(|&b| b == b'>')?;
        let hex_str: String = data[1..end].iter()
            .filter(|b| !b.is_ascii_whitespace())
            .map(|&b| b as char)
            .collect();
        
        // Decode hex to bytes
        let mut bytes = Vec::new();
        let chars: Vec<char> = hex_str.chars().collect();
        for chunk in chars.chunks(2) {
            let s: String = chunk.iter().collect();
            if let Ok(b) = u8::from_str_radix(&s, 16) {
                bytes.push(b);
            }
        }
        
        // Handle BOM
        if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
            Self::decode_utf16be(&bytes[2..])
        } else {
            String::from_utf8(bytes.clone())
                .or_else(|_| Ok::<String, ()>(bytes.iter().map(|&b| b as char).collect()))
                .ok()
        }
    }
    
    /// Find XMP metadata in PDF.
    fn find_xmp(data: &[u8]) -> Option<String> {
        // XMP is typically in a stream with /Subtype /XML
        // Or we can find the packet directly
        
        // Method 1: Look for xpacket markers
        let begin = b"<?xpacket begin=";
        let end = b"<?xpacket end=";
        
        if let Some(start) = Self::find_bytes(data, begin) {
            // Find the actual start of XMP (skip to <x:xmpmeta or <rdf:RDF)
            let xmp_start = &data[start..];
            if let Some(end_pos) = Self::find_bytes(xmp_start, end) {
                // Include the end packet
                let packet_end = xmp_start[end_pos..].iter()
                    .position(|&b| b == b'>')
                    .map(|p| end_pos + p + 1)
                    .unwrap_or(end_pos + 20);
                
                return std::str::from_utf8(&xmp_start[..packet_end]).ok()
                    .map(|s| s.to_string());
            }
        }
        
        // Method 2: Look for /Metadata reference with XML subtype
        // This is more complex, skip for now
        
        None
    }
    
    /// Count pages in PDF.
    fn count_pages(data: &[u8]) -> Option<u32> {
        // Look for /Count in Pages object
        // Format: /Type /Pages ... /Count N
        
        // Find /Type /Pages
        if let Some(pos) = Self::find_bytes(data, b"/Type /Pages") {
            // Look for /Count nearby (within ~200 bytes)
            let search_area = &data[pos..pos.saturating_add(200).min(data.len())];
            if let Some(count_pos) = Self::find_bytes(search_area, b"/Count") {
                let remaining = &search_area[count_pos + 6..];
                // Skip whitespace and parse number
                let s = std::str::from_utf8(remaining).ok()?;
                let num_str: String = s.chars()
                    .skip_while(|c| c.is_whitespace())
                    .take_while(|c| c.is_ascii_digit())
                    .collect();
                return num_str.parse().ok();
            }
        }
        None
    }
    
    /// Find bytes in data, return position.
    fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack.windows(needle.len())
            .position(|w| w == needle)
    }
}

impl FormatParser for PdfParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 5 && &header[0..5] == Self::MAGIC
    }
    
    fn format_name(&self) -> &'static str {
        "PDF"
    }
    
    fn extensions(&self) -> &'static [&'static str] {
        &["pdf"]
    }
    
    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let (attrs, xmp) = Self::parse_pdf(reader)?;
        
        let mut metadata = Metadata::new("PDF");
        metadata.exif = attrs;
        metadata.xmp = xmp;
        metadata.set_file_type("PDF", "application/pdf");
        
        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_can_parse() {
        let parser = PdfParser;
        assert!(parser.can_parse(b"%PDF-1.4"));
        assert!(parser.can_parse(b"%PDF-1.7"));
        assert!(parser.can_parse(b"%PDF-2.0"));
        assert!(!parser.can_parse(b"8BPS\x00\x01"));
        assert!(!parser.can_parse(b"\xFF\xD8\xFF"));
    }
    
    #[test]
    fn test_parse_version() {
        assert_eq!(PdfParser::parse_version(b"%PDF-1.4\n"), Some("1.4".to_string()));
        assert_eq!(PdfParser::parse_version(b"%PDF-2.0\r\n"), Some("2.0".to_string()));
    }
    
    #[test]
    fn test_parse_literal_string() {
        assert_eq!(
            PdfParser::parse_literal_string(b"(Hello World)"),
            Some("Hello World".to_string())
        );
        assert_eq!(
            PdfParser::parse_literal_string(b"(Hello\\nWorld)"),
            Some("Hello\nWorld".to_string())
        );
        assert_eq!(
            PdfParser::parse_literal_string(b"(Nested (parens) here)"),
            Some("Nested (parens) here".to_string())
        );
    }
    
    #[test]
    fn test_parse_hex_string() {
        assert_eq!(
            PdfParser::parse_hex_string(b"<48656C6C6F>"),
            Some("Hello".to_string())
        );
    }
}

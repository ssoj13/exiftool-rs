//! SVG (Scalable Vector Graphics) metadata parser.
//!
//! Extracts metadata from SVG XML files:
//! - Document dimensions (width, height, viewBox)
//! - Dublin Core metadata (title, creator, description, etc.)
//! - RDF metadata
//! - Custom metadata in <metadata> element
//!
//! # Example
//!
//! ```no_run
//! use exiftool_formats::{SvgParser, FormatParser, Metadata};
//! use std::io::Cursor;
//!
//! let svg_data = br#"<?xml version="1.0"?>
//! <svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
//!   <metadata>
//!     <dc:title>Test Image</dc:title>
//!   </metadata>
//! </svg>"#;
//!
//! let mut cursor = Cursor::new(&svg_data[..]);
//! let metadata = SvgParser.parse(&mut cursor).unwrap();
//! ```

use crate::{FormatParser, Metadata, ReadSeek};
use exiftool_attrs::AttrValue;

/// SVG file signature (XML declaration or <svg)
const SVG_SIGNATURES: &[&[u8]] = &[
    b"<?xml",
    b"<svg",
    b"<!DOCTYPE svg",
    // UTF-8 BOM + <?xml
    b"\xEF\xBB\xBF<?xml",
];

/// SVG parser.
pub struct SvgParser;

impl FormatParser for SvgParser {
    fn format_name(&self) -> &'static str {
        "SVG"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["svg", "svgz"]
    }

    fn can_parse(&self, header: &[u8]) -> bool {
        // Check signatures
        for sig in SVG_SIGNATURES {
            if header.starts_with(sig) {
                return true;
            }
        }
        
        // Also check for <svg anywhere in first 1KB (handles whitespace/comments)
        if header.len() >= 4 {
            let search = &header[..header.len().min(1024)];
            if let Ok(text) = std::str::from_utf8(search) {
                let lower = text.to_lowercase();
                return lower.contains("<svg") || lower.contains("<!doctype svg");
            }
        }
        
        false
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> crate::Result<Metadata> {
        let mut metadata = Metadata::new("SVG");
        
        // Read entire file (SVG files are typically small)
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        
        // Parse XML structure
        parse_svg_content(&content, &mut metadata);
        
        Ok(metadata)
    }
}

/// Parse SVG content and extract metadata.
fn parse_svg_content(content: &str, metadata: &mut Metadata) {
    // Extract <svg> root element attributes
    if let Some(svg_tag) = find_tag(content, "svg") {
        parse_svg_attributes(svg_tag, metadata);
        // Inkscape attributes are on the <svg> tag itself
        parse_inkscape_elements(svg_tag, metadata);
    }
    
    // Extract <metadata> content
    if let Some(meta_content) = extract_tag_content(content, "metadata") {
        parse_metadata_element(&meta_content, metadata);
    }
    
    // Extract <title> at document level
    if let Some(title) = extract_simple_tag_content(content, "title") {
        if !title.trim().is_empty() {
            metadata.exif.set("SVG:Title", AttrValue::Str(title.trim().to_string()));
        }
    }
    
    // Extract <desc> at document level
    if let Some(desc) = extract_simple_tag_content(content, "desc") {
        if !desc.trim().is_empty() {
            metadata.exif.set("SVG:Description", AttrValue::Str(desc.trim().to_string()));
        }
    }
}

/// Parse <svg> element attributes.
fn parse_svg_attributes(svg_tag: &str, metadata: &mut Metadata) {
    // Width
    if let Some(width) = extract_attribute(svg_tag, "width") {
        metadata.exif.set("SVG:Width", AttrValue::Str(width));
    }
    
    // Height
    if let Some(height) = extract_attribute(svg_tag, "height") {
        metadata.exif.set("SVG:Height", AttrValue::Str(height));
    }
    
    // ViewBox
    if let Some(viewbox) = extract_attribute(svg_tag, "viewBox") {
        // Parse viewBox components: minX minY width height
        let parts: Vec<String> = viewbox.split_whitespace().map(|s| s.to_string()).collect();
        metadata.exif.set("SVG:ViewBox", AttrValue::Str(viewbox));
        if parts.len() == 4 {
            if let (Ok(w), Ok(h)) = (parts[2].parse::<f64>(), parts[3].parse::<f64>()) {
                metadata.exif.set("SVG:ViewBoxWidth", AttrValue::Float(w as f32));
                metadata.exif.set("SVG:ViewBoxHeight", AttrValue::Float(h as f32));
            }
        }
    }
    
    // Version
    if let Some(version) = extract_attribute(svg_tag, "version") {
        metadata.exif.set("SVG:Version", AttrValue::Str(version));
    }
    
    // xmlns (namespace)
    if let Some(xmlns) = extract_attribute(svg_tag, "xmlns") {
        metadata.exif.set("SVG:Namespace", AttrValue::Str(xmlns));
    }
    
    // xmlns:xlink
    if let Some(xlink) = extract_attribute(svg_tag, "xmlns:xlink") {
        metadata.exif.set("SVG:XLinkNamespace", AttrValue::Str(xlink));
    }
    
    // id
    if let Some(id) = extract_attribute(svg_tag, "id") {
        metadata.exif.set("SVG:ID", AttrValue::Str(id));
    }
    
    // style
    if let Some(style) = extract_attribute(svg_tag, "style") {
        metadata.exif.set("SVG:Style", AttrValue::Str(style));
    }
    
    // preserveAspectRatio
    if let Some(par) = extract_attribute(svg_tag, "preserveAspectRatio") {
        metadata.exif.set("SVG:PreserveAspectRatio", AttrValue::Str(par));
    }
}

/// Parse <metadata> element content.
fn parse_metadata_element(content: &str, metadata: &mut Metadata) {
    // Dublin Core (dc:) elements
    parse_dc_elements(content, metadata);
    
    // RDF metadata
    parse_rdf_elements(content, metadata);
    
    // CC (Creative Commons) license
    parse_cc_elements(content, metadata);
}

/// Parse Dublin Core elements.
fn parse_dc_elements(content: &str, metadata: &mut Metadata) {
    // Common DC elements
    let dc_tags = [
        ("dc:title", "DC:Title"),
        ("dc:creator", "DC:Creator"),
        ("dc:subject", "DC:Subject"),
        ("dc:description", "DC:Description"),
        ("dc:publisher", "DC:Publisher"),
        ("dc:contributor", "DC:Contributor"),
        ("dc:date", "DC:Date"),
        ("dc:type", "DC:Type"),
        ("dc:format", "DC:Format"),
        ("dc:identifier", "DC:Identifier"),
        ("dc:source", "DC:Source"),
        ("dc:language", "DC:Language"),
        ("dc:relation", "DC:Relation"),
        ("dc:coverage", "DC:Coverage"),
        ("dc:rights", "DC:Rights"),
    ];
    
    for (xml_tag, attr_name) in dc_tags {
        if let Some(value) = extract_dc_value(content, xml_tag) {
            metadata.exif.set(attr_name, AttrValue::Str(value));
        }
    }
}

/// Extract DC element value (handles rdf:Bag, rdf:Seq, rdf:Alt wrappers).
fn extract_dc_value(content: &str, tag: &str) -> Option<String> {
    // Try to find the tag content
    let tag_content = extract_tag_content(content, tag)?;
    
    // Check for RDF containers
    if tag_content.contains("rdf:li") {
        // Collect all rdf:li values
        let mut values = Vec::new();
        let mut search_pos = 0;
        
        while let Some(li_content) = extract_tag_content_from(&tag_content, "rdf:li", search_pos) {
            let clean = strip_tags(&li_content).trim().to_string();
            if !clean.is_empty() {
                values.push(clean);
            }
            search_pos = tag_content.find("rdf:li").map(|p| p + 1).unwrap_or(tag_content.len());
            if let Some(pos) = tag_content[search_pos..].find("rdf:li") {
                search_pos += pos + 1;
            } else {
                break;
            }
        }
        
        if !values.is_empty() {
            return Some(values.join(", "));
        }
    }
    
    // Simple text content
    let clean = strip_tags(&tag_content).trim().to_string();
    if clean.is_empty() { None } else { Some(clean) }
}

/// Parse RDF elements.
fn parse_rdf_elements(content: &str, metadata: &mut Metadata) {
    // rdf:about
    if let Some(rdf_desc) = find_tag(content, "rdf:Description") {
        if let Some(about) = extract_attribute(rdf_desc, "rdf:about") {
            if !about.is_empty() {
                metadata.exif.set("RDF:About", AttrValue::Str(about));
            }
        }
    }
}

/// Parse Creative Commons elements.
fn parse_cc_elements(content: &str, metadata: &mut Metadata) {
    // cc:license
    if let Some(work) = find_tag(content, "cc:Work") {
        if let Some(license) = extract_attribute(work, "rdf:about") {
            metadata.exif.set("CC:Work", AttrValue::Str(license));
        }
    }
    
    // License URI
    if let Some(license_tag) = find_tag(content, "cc:license") {
        if let Some(license_uri) = extract_attribute(license_tag, "rdf:resource") {
            metadata.exif.set("CC:License", AttrValue::Str(license_uri));
        }
    }
}

/// Parse Inkscape-specific elements from svg tag.
fn parse_inkscape_elements(svg_tag: &str, metadata: &mut Metadata) {
    // sodipodi:docname
    if let Some(docname) = extract_attribute(svg_tag, "sodipodi:docname") {
        metadata.exif.set("Inkscape:DocumentName", AttrValue::Str(docname));
    }
    
    if let Some(version) = extract_attribute(svg_tag, "inkscape:version") {
        metadata.exif.set("Inkscape:Version", AttrValue::Str(version));
    }
}

// === XML parsing helpers ===

/// Find opening tag and return its content up to >.
fn find_tag<'a>(content: &'a str, tag_name: &str) -> Option<&'a str> {
    let pattern = format!("<{}", tag_name);
    let start = content.find(&pattern)?;
    let tag_start = start + 1; // Skip <
    
    // Find end of tag (either > or />)
    let remaining = &content[tag_start..];
    let end = remaining.find('>')? + tag_start;
    
    Some(&content[start..=end])
}

/// Extract content between opening and closing tags.
fn extract_tag_content(content: &str, tag_name: &str) -> Option<String> {
    extract_tag_content_from(content, tag_name, 0)
}

/// Extract content between opening and closing tags starting from position.
fn extract_tag_content_from(content: &str, tag_name: &str, from: usize) -> Option<String> {
    let search = &content[from..];
    let open_pattern = format!("<{}", tag_name);
    let close_pattern = format!("</{}", tag_name);
    
    let open_start = search.find(&open_pattern)?;
    let tag_end = search[open_start..].find('>')? + open_start + 1;
    
    // Handle self-closing tags
    if search[open_start..tag_end].ends_with("/>") {
        return None;
    }
    
    let close_start = search[tag_end..].find(&close_pattern)? + tag_end;
    
    Some(search[tag_end..close_start].to_string())
}

/// Extract simple tag content (for <title>, <desc>).
fn extract_simple_tag_content(content: &str, tag_name: &str) -> Option<String> {
    // Look for tag not inside <metadata>
    let metadata_start = content.find("<metadata");
    let metadata_end = content.find("</metadata>");
    
    // Find all occurrences and pick one outside metadata
    let open_pattern = format!("<{}>", tag_name);
    let close_pattern = format!("</{}>", tag_name);
    
    let mut pos = 0;
    while let Some(open) = content[pos..].find(&open_pattern) {
        let abs_open = pos + open;
        let content_start = abs_open + open_pattern.len();
        
        if let Some(close) = content[content_start..].find(&close_pattern) {
            let abs_close = content_start + close;
            
            // Check if this tag is outside <metadata>
            let inside_metadata = metadata_start
                .zip(metadata_end)
                .map(|(s, e)| abs_open > s && abs_close < e)
                .unwrap_or(false);
            
            if !inside_metadata {
                return Some(content[content_start..abs_close].to_string());
            }
        }
        
        pos = abs_open + 1;
    }
    
    None
}

/// Extract attribute value from tag.
fn extract_attribute(tag: &str, attr_name: &str) -> Option<String> {
    // Pattern: attr_name="value" or attr_name='value'
    let patterns = [
        format!("{}=\"", attr_name),
        format!("{}='", attr_name),
    ];
    
    for pattern in &patterns {
        if let Some(start) = tag.find(pattern) {
            let value_start = start + pattern.len();
            let quote_char = if pattern.ends_with('"') { '"' } else { '\'' };
            
            if let Some(end) = tag[value_start..].find(quote_char) {
                return Some(tag[value_start..value_start + end].to_string());
            }
        }
    }
    
    None
}

/// Strip XML tags from content.
fn strip_tags(content: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    
    for c in content.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }
    
    // Decode common XML entities
    result
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn parse_svg(content: &str) -> Metadata {
        let mut cursor = Cursor::new(content.as_bytes());
        SvgParser.parse(&mut cursor).unwrap()
    }

    #[test]
    fn detect_svg_xml_decl() {
        let svg = r#"<?xml version="1.0"?><svg></svg>"#;
        assert!(SvgParser.can_parse(svg.as_bytes()));
    }

    #[test]
    fn detect_svg_direct() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"></svg>"#;
        assert!(SvgParser.can_parse(svg.as_bytes()));
    }

    #[test]
    fn detect_svg_with_whitespace() {
        let svg = "   \n  <svg></svg>";
        assert!(SvgParser.can_parse(svg.as_bytes()));
    }

    #[test]
    fn reject_html() {
        let html = "<!DOCTYPE html><html><body></body></html>";
        assert!(!SvgParser.can_parse(html.as_bytes()));
    }

    #[test]
    fn parse_dimensions() {
        let svg = r#"<svg width="100" height="200" viewBox="0 0 100 200"></svg>"#;
        let meta = parse_svg(svg);

        assert_eq!(meta.exif.get_str("SVG:Width"), Some("100"));
        assert_eq!(meta.exif.get_str("SVG:Height"), Some("200"));
        assert_eq!(meta.exif.get_str("SVG:ViewBox"), Some("0 0 100 200"));
    }

    #[test]
    fn parse_title_desc() {
        let svg = r#"<svg>
            <title>Test Title</title>
            <desc>Test Description</desc>
        </svg>"#;
        let meta = parse_svg(svg);

        assert_eq!(meta.exif.get_str("SVG:Title"), Some("Test Title"));
        assert_eq!(meta.exif.get_str("SVG:Description"), Some("Test Description"));
    }

    #[test]
    fn parse_dublin_core() {
        let svg = r#"<svg>
            <metadata>
                <rdf:RDF>
                    <dc:title>DC Title</dc:title>
                    <dc:creator>Artist Name</dc:creator>
                    <dc:description>A vector graphic</dc:description>
                </rdf:RDF>
            </metadata>
        </svg>"#;
        let meta = parse_svg(svg);

        assert_eq!(meta.exif.get_str("DC:Title"), Some("DC Title"));
        assert_eq!(meta.exif.get_str("DC:Creator"), Some("Artist Name"));
        assert_eq!(meta.exif.get_str("DC:Description"), Some("A vector graphic"));
    }

    #[test]
    fn parse_inkscape_metadata() {
        let svg = r#"<svg 
            sodipodi:docname="test.svg"
            inkscape:version="1.2 (dc2aeda, 2022-05-15)">
        </svg>"#;
        let meta = parse_svg(svg);

        assert_eq!(meta.exif.get_str("Inkscape:DocumentName"), Some("test.svg"));
        assert_eq!(meta.exif.get_str("Inkscape:Version").map(|s| s.contains("1.2")), Some(true));
    }

    #[test]
    fn format_info() {
        assert_eq!(SvgParser.format_name(), "SVG");
        assert!(SvgParser.extensions().contains(&"svg"));
    }
}

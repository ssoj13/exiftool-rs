//! XML output format for exiftool-cli.
//!
//! Generates ExifTool-compatible XML output with proper escaping.

use exiftool_attrs::AttrValue;
use exiftool_formats::Metadata;
use std::fmt::Write;
use std::path::Path;

/// XML escape special characters.
fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            // Control chars (except tab, newline, carriage return)
            c if c.is_control() && c != '\t' && c != '\n' && c != '\r' => {
                write!(out, "&#x{:X};", c as u32).unwrap();
            }
            c => out.push(c),
        }
    }
    out
}

/// Convert AttrValue to XML-safe string.
fn val_to_xml(v: &AttrValue) -> String {
    xml_escape(&v.to_string())
}

/// Format metadata as XML string.
pub fn format_xml(path: &Path, m: &Metadata, filter: &[String], out: &mut String) {
    // Single file header
    let _ = writeln!(out, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let _ = writeln!(out, "<rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"");
    let _ = writeln!(out, "         xmlns:et=\"http://ns.exiftool.org/1.0/\"");
    let _ = writeln!(out, "         xmlns:ExifTool=\"http://ns.exiftool.org/ExifTool/1.0/\"");
    let _ = writeln!(out, "         xmlns:System=\"http://ns.exiftool.org/File/System/1.0/\"");
    let _ = writeln!(out, "         xmlns:File=\"http://ns.exiftool.org/File/1.0/\"");
    let _ = writeln!(out, "         xmlns:EXIF=\"http://ns.exiftool.org/EXIF/1.0/\"");
    let _ = writeln!(out, "         xmlns:XMP=\"http://ns.exiftool.org/XMP/1.0/\"");
    let _ = writeln!(out, "         xmlns:IPTC=\"http://ns.exiftool.org/IPTC/1.0/\"");
    let _ = writeln!(out, "         xmlns:ICC=\"http://ns.exiftool.org/ICC_Profile/1.0/\"");
    let _ = writeln!(out, "         xmlns:Composite=\"http://ns.exiftool.org/Composite/1.0/\">");
    
    let _ = writeln!(out, " <rdf:Description rdf:about=\"{}\"", xml_escape(&path.display().to_string()));
    let _ = writeln!(out, "  et:toolkit=\"exiftool-rs\">");
    
    // File info
    let _ = writeln!(out, "  <File:FileType>{}</File:FileType>", m.format);
    
    // All tags
    let mut entries: Vec<_> = m.exif.iter()
        .filter(|(k, _)| filter.is_empty() || super::tag_matches(k, filter))
        .collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));
    
    for (k, v) in entries {
        // Determine namespace based on tag name patterns
        let ns = determine_namespace(k);
        let _ = writeln!(out, "  <{}:{}>{}</{}:{}>", ns, k, val_to_xml(v), ns, k);
    }
    
    // XMP size if present
    if let Some(ref xmp) = m.xmp {
        let _ = writeln!(out, "  <File:XMPSize>{}</File:XMPSize>", xmp.len());
    }
    
    // Thumbnail size if present
    if let Some(ref thumb) = m.thumbnail {
        let _ = writeln!(out, "  <File:ThumbnailSize>{}</File:ThumbnailSize>", thumb.len());
    }
    
    // Page count for multi-page
    if m.pages.len() > 1 {
        let _ = writeln!(out, "  <File:PageCount>{}</File:PageCount>", m.pages.len());
    }
    
    let _ = writeln!(out, " </rdf:Description>");
    let _ = writeln!(out, "</rdf:RDF>");
}

/// Print XML to stdout.
pub fn print_xml(path: &Path, m: &Metadata, filter: &[String]) {
    let mut out = String::new();
    format_xml(path, m, filter, &mut out);
    print!("{}", out);
}

/// Determine XML namespace for a tag.
fn determine_namespace(tag: &str) -> &'static str {
    // GPS tags
    if tag.starts_with("GPS") {
        return "EXIF";
    }
    // XMP tags
    if tag.starts_with("XMP") || tag.contains("dc:") || tag.contains("xmp:") {
        return "XMP";
    }
    // IPTC tags
    if tag.starts_with("IPTC") || matches!(tag, 
        "Headline" | "Caption" | "Keywords" | "City" | "Province" | 
        "Country" | "Category" | "SupplementalCategories" | "Byline" |
        "BylineTitle" | "Credit" | "Source" | "CopyrightNotice" |
        "ObjectName" | "SpecialInstructions" | "DateCreated" | "TimeCreated"
    ) {
        return "IPTC";
    }
    // ICC profile tags
    if tag.starts_with("ICC") || tag.starts_with("Profile") || 
       matches!(tag, "ColorSpace" | "ColorSpaceData" | "DeviceModel" | 
                "DeviceManufacturer" | "RenderingIntent") {
        return "ICC";
    }
    // Composite/calculated tags
    if matches!(tag, "Megapixels" | "ImageSize" | "LensID" | "ShutterSpeed" |
                "Aperture" | "FocalLength35efl" | "HyperfocalDistance" |
                "CircleOfConfusion" | "DOF" | "FOV" | "LightValue") {
        return "Composite";
    }
    // Default to EXIF
    "EXIF"
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_xml_escape() {
        assert_eq!(xml_escape("hello"), "hello");
        assert_eq!(xml_escape("<>&\"'"), "&lt;&gt;&amp;&quot;&apos;");
        assert_eq!(xml_escape("a & b"), "a &amp; b");
    }
    
    #[test]
    fn test_determine_namespace() {
        assert_eq!(determine_namespace("GPSLatitude"), "EXIF");
        assert_eq!(determine_namespace("Make"), "EXIF");
        assert_eq!(determine_namespace("Headline"), "IPTC");
        assert_eq!(determine_namespace("ProfileDescription"), "ICC");
        assert_eq!(determine_namespace("Megapixels"), "Composite");
    }
}

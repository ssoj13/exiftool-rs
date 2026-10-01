//! XMP XML writer.
//!
//! Serializes Attrs back to XMP format with proper RDF structure.

use crate::{ns, Result};
use exiftool_attrs::{AttrValue, Attrs};
use std::collections::HashMap;

/// XMP writer.
pub struct XmpWriter;

/// XMP tag type for proper serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmpTagType {
    /// Simple string value
    Simple,
    /// rdf:Bag (unordered list)
    Bag,
    /// rdf:Seq (ordered list)
    Seq,
    /// rdf:Alt (language alternatives)
    Alt,
}

impl XmpWriter {
    /// Serialize Attrs to XMP XML string.
    pub fn write(attrs: &Attrs) -> Result<String> {
        let mut output = String::new();
        
        // XMP packet header
        output.push_str("<?xpacket begin=\"\u{FEFF}\" id=\"W5M0MpCehiHzreSzNTczkc9d\"?>\n");
        output.push_str("<x:xmpmeta xmlns:x=\"adobe:ns:meta/\">\n");
        output.push_str("  <rdf:RDF xmlns:rdf=\"");
        output.push_str(ns::RDF);
        output.push_str("\">\n");
        
        // Collect namespaces used
        let namespaces = Self::collect_namespaces(attrs);
        
        // Start Description with namespaces
        output.push_str("    <rdf:Description rdf:about=\"\"\n");
        for (prefix, uri) in &namespaces {
            output.push_str(&format!("        xmlns:{}=\"{}\"\n", prefix, uri));
        }
        output.push_str("        >\n");
        
        // Group attrs by namespace
        let grouped = Self::group_by_namespace(attrs);
        
        // Write properties
        for (ns_prefix, props) in &grouped {
            for (prop_name, value) in props {
                Self::write_property(&mut output, ns_prefix, prop_name, value)?;
            }
        }
        
        // Close Description and RDF
        output.push_str("    </rdf:Description>\n");
        output.push_str("  </rdf:RDF>\n");
        output.push_str("</x:xmpmeta>\n");
        output.push_str("<?xpacket end=\"w\"?>");
        
        Ok(output)
    }

    /// Write a single property with appropriate RDF structure.
    fn write_property(
        output: &mut String,
        ns_prefix: &str,
        prop_name: &str,
        value: &AttrValue,
    ) -> Result<()> {
        let tag_type = Self::infer_tag_type(ns_prefix, prop_name);
        let full_tag = format!("{}:{}", ns_prefix, prop_name);
        
        match value {
            AttrValue::List(items) => {
                // Determine container type
                let container = match tag_type {
                    XmpTagType::Seq => "rdf:Seq",
                    XmpTagType::Alt => "rdf:Alt",
                    _ => "rdf:Bag", // Default to Bag for lists
                };
                
                output.push_str(&format!("      <{}>\n", full_tag));
                output.push_str(&format!("        <{}>\n", container));
                
                for item in items {
                    if let AttrValue::Str(s) = item {
                        let escaped = Self::escape_xml(s);
                        if tag_type == XmpTagType::Alt {
                            output.push_str(&format!("          <rdf:li xml:lang=\"x-default\">{}</rdf:li>\n", escaped));
                        } else {
                            output.push_str(&format!("          <rdf:li>{}</rdf:li>\n", escaped));
                        }
                    }
                }
                
                output.push_str(&format!("        </{}>\n", container));
                output.push_str(&format!("      </{}>\n", full_tag));
            }
            AttrValue::Str(s) => {
                // Check if it's a language-tagged value (e.g., dc:title[en])
                if tag_type == XmpTagType::Alt {
                    output.push_str(&format!("      <{}>\n", full_tag));
                    output.push_str("        <rdf:Alt>\n");
                    let escaped = Self::escape_xml(s);
                    output.push_str(&format!("          <rdf:li xml:lang=\"x-default\">{}</rdf:li>\n", escaped));
                    output.push_str("        </rdf:Alt>\n");
                    output.push_str(&format!("      </{}>\n", full_tag));
                } else {
                    let escaped = Self::escape_xml(s);
                    output.push_str(&format!("      <{}>{}</{}>\n", full_tag, escaped, full_tag));
                }
            }
            AttrValue::Int(n) => {
                output.push_str(&format!("      <{}>{}</{}>\n", full_tag, n, full_tag));
            }
            AttrValue::UInt(n) => {
                output.push_str(&format!("      <{}>{}</{}>\n", full_tag, n, full_tag));
            }
            AttrValue::Float(f) => {
                output.push_str(&format!("      <{}>{}</{}>\n", full_tag, f, full_tag));
            }
            AttrValue::Double(d) => {
                output.push_str(&format!("      <{}>{}</{}>\n", full_tag, d, full_tag));
            }
            AttrValue::Bool(b) => {
                let val = if *b { "True" } else { "False" };
                output.push_str(&format!("      <{}>{}</{}>\n", full_tag, val, full_tag));
            }
            AttrValue::DateTime(dt) => {
                let formatted = dt.format("%Y-%m-%dT%H:%M:%S").to_string();
                output.push_str(&format!("      <{}>{}</{}>\n", full_tag, formatted, full_tag));
            }
            AttrValue::Rational(num, den) => {
                output.push_str(&format!("      <{}>{}/{}</{}>\n", full_tag, num, den, full_tag));
            }
            AttrValue::URational(num, den) => {
                output.push_str(&format!("      <{}>{}/{}</{}>\n", full_tag, num, den, full_tag));
            }
            _ => {
                // Skip unsupported types
            }
        }
        
        Ok(())
    }

    /// Collect all namespaces used in attrs.
    fn collect_namespaces(attrs: &Attrs) -> Vec<(String, String)> {
        let mut ns_set: HashMap<String, String> = HashMap::new();
        
        for (key, _) in attrs.iter() {
            if let Some((prefix, _)) = Self::parse_key(key) {
                let uri = Self::prefix_to_uri(&prefix);
                ns_set.insert(prefix, uri);
            }
        }
        
        let mut result: Vec<_> = ns_set.into_iter().collect();
        result.sort_by(|a, b| a.0.cmp(&b.0));
        result
    }

    /// Group attrs by namespace prefix.
    fn group_by_namespace(attrs: &Attrs) -> HashMap<String, Vec<(String, AttrValue)>> {
        let mut grouped: HashMap<String, Vec<(String, AttrValue)>> = HashMap::new();
        
        for (key, value) in attrs.iter() {
            // Skip language-variant keys (e.g., dc:title[en]) - they're handled with main key
            if key.contains('[') {
                continue;
            }
            
            // Skip struct fields (e.g., xmp:Flash.Mode) for now
            if key.contains('.') {
                continue;
            }
            
            if let Some((prefix, prop_name)) = Self::parse_key(key) {
                grouped
                    .entry(prefix)
                    .or_default()
                    .push((prop_name, value.clone()));
            }
        }
        
        grouped
    }

    /// Parse XMP key into (namespace_prefix, property_name).
    fn parse_key(key: &str) -> Option<(String, String)> {
        // Handle normalized keys like "DC:subject" -> ("dc", "subject")
        let key = Self::denormalize_key(key);
        
        if let Some(pos) = key.find(':') {
            let prefix = &key[..pos];
            let name = &key[pos + 1..];
            Some((prefix.to_string(), name.to_string()))
        } else {
            None
        }
    }

    /// Convert normalized key back to XMP prefix.
    fn denormalize_key(key: &str) -> String {
        key.replace("DC:", "dc:")
            .replace("XMP:", "xmp:")
            .replace("XMP-MM:", "xmpMM:")
            .replace("XMP-Rights:", "xmpRights:")
            .replace("EXIF:", "exif:")
            .replace("EXIF-EX:", "exifEX:")
            .replace("TIFF:", "tiff:")
            .replace("Photoshop:", "photoshop:")
            .replace("IPTC:", "Iptc4xmpCore:")
            .replace("IPTC-Ext:", "Iptc4xmpExt:")
            .replace("CRS:", "crs:")
            .replace("Lightroom:", "lr:")
            .replace("AUX:", "aux:")
    }

    /// Convert namespace prefix to URI.
    fn prefix_to_uri(prefix: &str) -> String {
        match prefix {
            "dc" => ns::DC.to_string(),
            "xmp" => ns::XMP.to_string(),
            "xmpRights" => ns::XMP_RIGHTS.to_string(),
            "exif" => ns::EXIF.to_string(),
            "tiff" => ns::TIFF.to_string(),
            "photoshop" => ns::PHOTOSHOP.to_string(),
            "Iptc4xmpCore" => ns::IPTC.to_string(),
            "xmpMM" => "http://ns.adobe.com/xap/1.0/mm/".to_string(),
            "stRef" => "http://ns.adobe.com/xap/1.0/sType/ResourceRef#".to_string(),
            "stEvt" => "http://ns.adobe.com/xap/1.0/sType/ResourceEvent#".to_string(),
            "crs" => "http://ns.adobe.com/camera-raw-settings/1.0/".to_string(),
            "lr" => "http://ns.adobe.com/lightroom/1.0/".to_string(),
            "aux" => "http://ns.adobe.com/exif/1.0/aux/".to_string(),
            "exifEX" => "http://cipa.jp/exif/1.0/".to_string(),
            _ => format!("http://ns.unknown/{}/", prefix),
        }
    }

    /// Infer tag type for proper RDF serialization.
    fn infer_tag_type(ns_prefix: &str, prop_name: &str) -> XmpTagType {
        // Dublin Core tags that use specific types
        if ns_prefix == "dc" {
            match prop_name {
                "subject" | "type" | "format" => XmpTagType::Bag,
                "creator" | "publisher" | "contributor" | "relation" | "language" => XmpTagType::Seq,
                "title" | "description" | "rights" => XmpTagType::Alt,
                _ => XmpTagType::Simple,
            }
        } else if ns_prefix == "xmp" {
            match prop_name {
                "Identifier" => XmpTagType::Bag,
                _ => XmpTagType::Simple,
            }
        } else if ns_prefix == "Iptc4xmpCore" {
            match prop_name {
                "Scene" | "SubjectCode" => XmpTagType::Bag,
                "CreatorContactInfo" => XmpTagType::Simple, // Struct, but not list
                _ => XmpTagType::Simple,
            }
        } else {
            XmpTagType::Simple
        }
    }

    /// Escape special XML characters.
    fn escape_xml(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_simple() {
        let mut attrs = Attrs::new();
        attrs.set("XMP:Rating", AttrValue::Str("5".to_string()));
        attrs.set("XMP:CreateDate", AttrValue::Str("2024-01-15T10:30:00".to_string()));

        let xmp = XmpWriter::write(&attrs).unwrap();
        
        assert!(xmp.contains("<?xpacket"));
        assert!(xmp.contains("<xmp:Rating>5</xmp:Rating>"));
        assert!(xmp.contains("<xmp:CreateDate>2024-01-15T10:30:00</xmp:CreateDate>"));
    }

    #[test]
    fn test_write_bag() {
        let mut attrs = Attrs::new();
        attrs.set("DC:subject", AttrValue::List(vec![
            AttrValue::Str("keyword1".to_string()),
            AttrValue::Str("keyword2".to_string()),
        ]));

        let xmp = XmpWriter::write(&attrs).unwrap();
        
        assert!(xmp.contains("<rdf:Bag>"));
        assert!(xmp.contains("<rdf:li>keyword1</rdf:li>"));
        assert!(xmp.contains("<rdf:li>keyword2</rdf:li>"));
    }

    #[test]
    fn test_write_seq() {
        let mut attrs = Attrs::new();
        attrs.set("DC:creator", AttrValue::List(vec![
            AttrValue::Str("Author 1".to_string()),
            AttrValue::Str("Author 2".to_string()),
        ]));

        let xmp = XmpWriter::write(&attrs).unwrap();
        
        assert!(xmp.contains("<rdf:Seq>"));
        assert!(xmp.contains("<rdf:li>Author 1</rdf:li>"));
    }

    #[test]
    fn test_write_alt() {
        let mut attrs = Attrs::new();
        attrs.set("DC:title", AttrValue::Str("My Photo Title".to_string()));

        let xmp = XmpWriter::write(&attrs).unwrap();
        
        assert!(xmp.contains("<rdf:Alt>"));
        assert!(xmp.contains("xml:lang=\"x-default\""));
        assert!(xmp.contains("My Photo Title"));
    }

    #[test]
    fn test_roundtrip() {
        use crate::XmpParser;
        
        let mut original = Attrs::new();
        original.set("XMP:Rating", AttrValue::Str("5".to_string()));
        original.set("DC:subject", AttrValue::List(vec![
            AttrValue::Str("nature".to_string()),
            AttrValue::Str("landscape".to_string()),
        ]));

        let xmp = XmpWriter::write(&original).unwrap();
        let parsed = XmpParser::parse(&xmp).unwrap();
        
        assert_eq!(parsed.get_str("XMP:Rating"), Some("5"));
        
        if let Some(AttrValue::List(items)) = parsed.get("DC:subject") {
            assert_eq!(items.len(), 2);
        } else {
            panic!("Expected DC:subject as list");
        }
    }

    #[test]
    fn test_xml_escape() {
        let mut attrs = Attrs::new();
        attrs.set("DC:description", AttrValue::Str("Test <>&\"' chars".to_string()));

        let xmp = XmpWriter::write(&attrs).unwrap();
        
        assert!(xmp.contains("&lt;"));
        assert!(xmp.contains("&gt;"));
        assert!(xmp.contains("&amp;"));
    }
}

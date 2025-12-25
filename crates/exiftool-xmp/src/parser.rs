//! XMP XML parser with full RDF support.
//!
//! Handles:
//! - Simple properties (xmp:Rating="5")
//! - rdf:Bag (unordered lists, e.g., dc:subject keywords)
//! - rdf:Seq (ordered lists, e.g., dc:creator)
//! - rdf:Alt (language alternatives, e.g., dc:title with xml:lang)
//! - Nested structures (XMP structs like exif:Flash)
//! - XMP packet wrapper detection

use crate::{Error, Result};
use exiftool_attrs::{AttrValue, Attrs};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Reader;

/// XMP parser with full RDF collection support.
pub struct XmpParser;

/// State during XMP parsing.
#[derive(Debug, Clone, PartialEq)]
enum ParseState {
    /// Outside of Description element
    Outside,
    /// Inside rdf:Description
    InDescription,
    /// Inside a property element (e.g., dc:subject)
    InProperty { ns: String, name: String },
    /// Inside rdf:Bag - collecting unordered items
    InBag { ns: String, name: String, items: Vec<AttrValue> },
    /// Inside rdf:Seq - collecting ordered items
    InSeq { ns: String, name: String, items: Vec<AttrValue> },
    /// Inside rdf:Alt - collecting language alternatives
    InAlt { ns: String, name: String, items: Vec<(String, String)> },
    /// Inside rdf:li element
    InListItem { lang: Option<String> },
    /// Inside rdf:li with parseType="Resource" - collecting struct fields
    InResourceItem { fields: Vec<(String, String)> },
    /// Inside a field within a resource item
    InResourceField { field_name: String },
    /// Inside nested struct
    InStruct { parent_ns: String, parent_name: String, fields: Vec<(String, String)> },
}

impl XmpParser {
    /// Parse XMP string into attributes.
    pub fn parse(xmp: &str) -> Result<Attrs> {
        let mut attrs = Attrs::new();
        let mut reader = Reader::from_str(xmp);
        reader.config_mut().trim_text(true);

        let mut state_stack: Vec<ParseState> = vec![ParseState::Outside];
        let mut text_buf = String::new();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let name = elem_name(e);
                    text_buf.clear();

                    match state_stack.last().cloned() {
                        Some(ParseState::Outside) => {
                            if name.ends_with(":Description") || name == "Description" {
                                // Extract inline attributes from Description
                                extract_description_attrs(e, &mut attrs);
                                state_stack.push(ParseState::InDescription);
                            }
                        }
                        Some(ParseState::InDescription) => {
                            // Starting a property element
                            let (ns, prop_name) = split_name(&name);
                            state_stack.push(ParseState::InProperty {
                                ns: ns.to_string(),
                                name: prop_name.to_string(),
                            });
                        }
                        Some(ParseState::InProperty { ref ns, ref name }) => {
                            // Check for RDF collection types
                            let elem = elem_name(e);
                            let local = local_name(&elem);
                            match local {
                                "Bag" => {
                                    let new_state = ParseState::InBag {
                                        ns: ns.clone(),
                                        name: name.clone(),
                                        items: Vec::new(),
                                    };
                                    state_stack.pop();
                                    state_stack.push(new_state);
                                }
                                "Seq" => {
                                    let new_state = ParseState::InSeq {
                                        ns: ns.clone(),
                                        name: name.clone(),
                                        items: Vec::new(),
                                    };
                                    state_stack.pop();
                                    state_stack.push(new_state);
                                }
                                "Alt" => {
                                    let new_state = ParseState::InAlt {
                                        ns: ns.clone(),
                                        name: name.clone(),
                                        items: Vec::new(),
                                    };
                                    state_stack.pop();
                                    state_stack.push(new_state);
                                }
                                "Description" => {
                                    // Nested struct
                                    let new_state = ParseState::InStruct {
                                        parent_ns: ns.clone(),
                                        parent_name: name.clone(),
                                        fields: Vec::new(),
                                    };
                                    // Extract struct attributes
                                    // Extract struct inline attributes if any
                                    // (attributes are handled in InStruct state)
                                    state_stack.pop();
                                    state_stack.push(new_state);
                                }
                                _ => {
                                    // Nested property in struct - treat as struct start
                                    let new_state = ParseState::InStruct {
                                        parent_ns: ns.clone(),
                                        parent_name: name.clone(),
                                        fields: Vec::new(),
                                    };
                                    state_stack.pop();
                                    state_stack.push(new_state);
                                }
                            }
                        }
                        Some(ParseState::InBag { .. })
                        | Some(ParseState::InSeq { .. })
                        | Some(ParseState::InAlt { .. }) => {
                            // Starting rdf:li element
                            let lang = get_xml_lang(e);
                            // Check for parseType="Resource"
                            let is_resource = has_parse_type_resource(e);
                            if is_resource {
                                state_stack.push(ParseState::InResourceItem { fields: Vec::new() });
                            } else {
                                state_stack.push(ParseState::InListItem { lang });
                            }
                        }
                        Some(ParseState::InResourceItem { .. }) => {
                            // Field inside resource item
                            let elem = elem_name(e);
                            let (_, field_name) = split_name(&elem);
                            state_stack.push(ParseState::InResourceField { 
                                field_name: field_name.to_string() 
                            });
                        }
                        Some(ParseState::InStruct { .. }) => {
                            // Nested field inside struct - record field name
                            let elem = elem_name(e);
                            let (field_ns, field_name) = split_name(&elem);
                            // For now, just track we're in a nested element
                            state_stack.push(ParseState::InProperty {
                                ns: field_ns.to_string(),
                                name: field_name.to_string(),
                            });
                        }
                        _ => {}
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    let name = elem_name(e);

                    match state_stack.last().cloned() {
                        Some(ParseState::InDescription) => {
                            // Empty element with attributes - treat as struct
                            let (ns, prop_name) = split_name(&name);
                            for attr in e.attributes().flatten() {
                                let attr_key = attr_name(&attr);
                                if !attr_key.starts_with("xmlns") && !attr_key.starts_with("rdf:") {
                                    let (_, attr_local) = split_name(&attr_key);
                                    let full_key = format!("{}:{}.{}", ns, prop_name, attr_local);
                                    let val = String::from_utf8_lossy(&attr.value).to_string();
                                    attrs.set(normalize_key(&full_key), AttrValue::Str(val));
                                }
                            }
                        }
                        Some(ParseState::InBag { ref mut items, .. }) => {
                            // Empty rdf:li - might have parseType="Resource" with attrs
                            for attr in e.attributes().flatten() {
                                let key = attr_name(&attr);
                                if !key.starts_with("rdf:") {
                                    let val = String::from_utf8_lossy(&attr.value).to_string();
                                    items.push(AttrValue::Str(val));
                                }
                            }
                        }
                        Some(ParseState::InSeq { ref mut items, .. }) => {
                            for attr in e.attributes().flatten() {
                                let key = attr_name(&attr);
                                if !key.starts_with("rdf:") {
                                    let val = String::from_utf8_lossy(&attr.value).to_string();
                                    items.push(AttrValue::Str(val));
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Text(ref e)) => {
                    if let Ok(txt) = e.decode() {
                        text_buf.push_str(&txt);
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = end_elem_name(e);

                    match state_stack.last().cloned() {
                        Some(ParseState::InResourceField { field_name }) => {
                            state_stack.pop();
                            let text = text_buf.trim().to_string();
                            text_buf.clear();
                            
                            // Add field to parent resource item
                            if let Some(ParseState::InResourceItem { fields }) = state_stack.last_mut() {
                                if !text.is_empty() {
                                    fields.push((field_name, text));
                                }
                            }
                        }
                        Some(ParseState::InResourceItem { fields }) => {
                            state_stack.pop();
                            
                            // Create a struct representation for the list
                            // For now, store fields as ParentName.FieldName
                            if let Some(parent) = state_stack.last_mut() {
                                match parent {
                                    ParseState::InBag { ns, name, items } => {
                                        // Store struct as formatted string or as Map
                                        for (field_name, field_val) in &fields {
                                            let key = format!("{}:{}.{}", ns, name, field_name);
                                            attrs.set(normalize_key(&key), AttrValue::Str(field_val.clone()));
                                        }
                                        // Also add placeholder to items count
                                        if !fields.is_empty() {
                                            items.push(AttrValue::Str("<struct>".to_string()));
                                        }
                                    }
                                    ParseState::InSeq { ns, name, items } => {
                                        for (field_name, field_val) in &fields {
                                            let key = format!("{}:{}.{}", ns, name, field_name);
                                            attrs.set(normalize_key(&key), AttrValue::Str(field_val.clone()));
                                        }
                                        if !fields.is_empty() {
                                            items.push(AttrValue::Str("<struct>".to_string()));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Some(ParseState::InListItem { lang }) => {
                            state_stack.pop();
                            let text = text_buf.trim().to_string();
                            text_buf.clear();

                            // Add to parent collection
                            if let Some(parent) = state_stack.last_mut() {
                                match parent {
                                    ParseState::InBag { items, .. } => {
                                        if !text.is_empty() {
                                            items.push(AttrValue::Str(text));
                                        }
                                    }
                                    ParseState::InSeq { items, .. } => {
                                        if !text.is_empty() {
                                            items.push(AttrValue::Str(text));
                                        }
                                    }
                                    ParseState::InAlt { items, .. } => {
                                        let lang_key = lang.unwrap_or_else(|| "x-default".to_string());
                                        if !text.is_empty() {
                                            items.push((lang_key, text));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Some(ParseState::InBag { ns, name, items }) => {
                            state_stack.pop();
                            if !items.is_empty() {
                                let key = format!("{}:{}", ns, name);
                                attrs.set(normalize_key(&key), AttrValue::List(items));
                            }
                        }
                        Some(ParseState::InSeq { ns, name, items }) => {
                            state_stack.pop();
                            if !items.is_empty() {
                                let key = format!("{}:{}", ns, name);
                                attrs.set(normalize_key(&key), AttrValue::List(items));
                            }
                        }
                        Some(ParseState::InAlt { ns, name, items }) => {
                            state_stack.pop();
                            if !items.is_empty() {
                                // For Alt, use x-default value or first value
                                let default_val = items
                                    .iter()
                                    .find(|(lang, _)| lang == "x-default")
                                    .or_else(|| items.first())
                                    .map(|(_, v)| v.clone());

                                if let Some(val) = default_val {
                                    let key = format!("{}:{}", ns, name);
                                    attrs.set(normalize_key(&key), AttrValue::Str(val));
                                }

                                // Also store all language variants if multiple
                                if items.len() > 1 {
                                    for (lang, val) in &items {
                                        let key = format!("{}:{}[{}]", ns, name, lang);
                                        attrs.set(normalize_key(&key), AttrValue::Str(val.clone()));
                                    }
                                }
                            }
                        }
                        Some(ParseState::InProperty { ns, name: prop_name }) => {
                            state_stack.pop();
                            let text = text_buf.trim().to_string();
                            text_buf.clear();
                            if !text.is_empty() {
                                let key = format!("{}:{}", ns, prop_name);
                                attrs.set(normalize_key(&key), AttrValue::Str(text));
                            }
                        }
                        Some(ParseState::InStruct { parent_ns, parent_name, fields }) => {
                            state_stack.pop();
                            // Store struct fields with parent.field notation
                            for (field_name, field_val) in fields {
                                let key = format!("{}:{}.{}", parent_ns, parent_name, field_name);
                                attrs.set(normalize_key(&key), AttrValue::Str(field_val));
                            }
                        }
                        Some(ParseState::InDescription) => {
                            if name.ends_with(":Description") || name == "Description" {
                                state_stack.pop();
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(Error::Xml(e)),
                _ => {}
            }
        }

        Ok(attrs)
    }
}

/// Extract element name as string from start tag.
fn elem_name(e: &BytesStart) -> String {
    String::from_utf8_lossy(e.name().as_ref()).to_string()
}

/// Extract element name as string from end tag.
fn end_elem_name(e: &BytesEnd) -> String {
    String::from_utf8_lossy(e.name().as_ref()).to_string()
}

/// Extract attribute name as string.
fn attr_name(attr: &quick_xml::events::attributes::Attribute) -> String {
    String::from_utf8_lossy(attr.key.as_ref()).to_string()
}

/// Split namespace:name into parts.
fn split_name(name: &str) -> (&str, &str) {
    if let Some(pos) = name.find(':') {
        (&name[..pos], &name[pos + 1..])
    } else {
        ("", name)
    }
}

/// Get local name (after colon).
fn local_name(name: &str) -> &str {
    if let Some(pos) = name.find(':') {
        &name[pos + 1..]
    } else {
        name
    }
}

/// Get xml:lang attribute value.
fn get_xml_lang(e: &BytesStart) -> Option<String> {
    for attr in e.attributes().flatten() {
        let key = attr_name(&attr);
        if key == "xml:lang" {
            return Some(String::from_utf8_lossy(&attr.value).to_string());
        }
    }
    None
}

/// Check if element has rdf:parseType="Resource" attribute.
fn has_parse_type_resource(e: &BytesStart) -> bool {
    for attr in e.attributes().flatten() {
        let key = attr_name(&attr);
        if key == "rdf:parseType" {
            let val = String::from_utf8_lossy(&attr.value);
            return val == "Resource";
        }
    }
    false
}

/// Extract Description element attributes as properties.
fn extract_description_attrs(e: &BytesStart, attrs: &mut Attrs) {
    for attr in e.attributes().flatten() {
        let key = attr_name(&attr);
        // Skip namespace declarations and rdf attributes
        if !key.starts_with("xmlns") && !key.starts_with("rdf:") {
            let value = String::from_utf8_lossy(&attr.value).to_string();
            attrs.set(normalize_key(&key), AttrValue::Str(value));
        }
    }
}

/// Normalize XMP key to consistent format.
fn normalize_key(key: &str) -> String {
    key.replace("dc:", "DC:")
        .replace("xmp:", "XMP:")
        .replace("xmpMM:", "XMP-MM:")
        .replace("xmpRights:", "XMP-Rights:")
        .replace("exif:", "EXIF:")
        .replace("exifEX:", "EXIF-EX:")
        .replace("tiff:", "TIFF:")
        .replace("photoshop:", "Photoshop:")
        .replace("Iptc4xmpCore:", "IPTC:")
        .replace("Iptc4xmpExt:", "IPTC-Ext:")
        .replace("crs:", "CRS:")
        .replace("lr:", "Lightroom:")
        .replace("aux:", "AUX:")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_xmp() {
        let xmp = r#"<?xpacket begin="" id="W5M0MpCehiHzreSzNTczkc9d"?>
<x:xmpmeta xmlns:x="adobe:ns:meta/">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
    <rdf:Description rdf:about=""
      xmlns:xmp="http://ns.adobe.com/xap/1.0/"
      xmp:Rating="5">
    </rdf:Description>
  </rdf:RDF>
</x:xmpmeta>
<?xpacket end="w"?>"#;

        let attrs = XmpParser::parse(xmp).unwrap();
        assert_eq!(attrs.get_str("XMP:Rating"), Some("5"));
    }

    #[test]
    fn parse_rdf_bag() {
        let xmp = r#"<x:xmpmeta xmlns:x="adobe:ns:meta/">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
    <rdf:Description xmlns:dc="http://purl.org/dc/elements/1.1/">
      <dc:subject>
        <rdf:Bag>
          <rdf:li>keyword1</rdf:li>
          <rdf:li>keyword2</rdf:li>
          <rdf:li>keyword3</rdf:li>
        </rdf:Bag>
      </dc:subject>
    </rdf:Description>
  </rdf:RDF>
</x:xmpmeta>"#;

        let attrs = XmpParser::parse(xmp).unwrap();
        if let Some(AttrValue::List(items)) = attrs.get("DC:subject") {
            assert_eq!(items.len(), 3);
            assert!(items.contains(&AttrValue::Str("keyword1".to_string())));
            assert!(items.contains(&AttrValue::Str("keyword2".to_string())));
            assert!(items.contains(&AttrValue::Str("keyword3".to_string())));
        } else {
            panic!("Expected List for DC:subject");
        }
    }

    #[test]
    fn parse_rdf_seq() {
        let xmp = r#"<x:xmpmeta xmlns:x="adobe:ns:meta/">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
    <rdf:Description xmlns:dc="http://purl.org/dc/elements/1.1/">
      <dc:creator>
        <rdf:Seq>
          <rdf:li>First Author</rdf:li>
          <rdf:li>Second Author</rdf:li>
        </rdf:Seq>
      </dc:creator>
    </rdf:Description>
  </rdf:RDF>
</x:xmpmeta>"#;

        let attrs = XmpParser::parse(xmp).unwrap();
        if let Some(AttrValue::List(items)) = attrs.get("DC:creator") {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], AttrValue::Str("First Author".to_string()));
            assert_eq!(items[1], AttrValue::Str("Second Author".to_string()));
        } else {
            panic!("Expected List for DC:creator");
        }
    }

    #[test]
    fn parse_rdf_alt_lang() {
        let xmp = r#"<x:xmpmeta xmlns:x="adobe:ns:meta/">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
    <rdf:Description xmlns:dc="http://purl.org/dc/elements/1.1/">
      <dc:title>
        <rdf:Alt>
          <rdf:li xml:lang="x-default">Default Title</rdf:li>
          <rdf:li xml:lang="en">English Title</rdf:li>
          <rdf:li xml:lang="de">Deutscher Titel</rdf:li>
        </rdf:Alt>
      </dc:title>
    </rdf:Description>
  </rdf:RDF>
</x:xmpmeta>"#;

        let attrs = XmpParser::parse(xmp).unwrap();
        // Default value
        assert_eq!(attrs.get_str("DC:title"), Some("Default Title"));
        // Language variants
        assert_eq!(attrs.get_str("DC:title[en]"), Some("English Title"));
        assert_eq!(attrs.get_str("DC:title[de]"), Some("Deutscher Titel"));
    }

    #[test]
    fn parse_nested_property() {
        let xmp = r#"<x:xmpmeta xmlns:x="adobe:ns:meta/">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
    <rdf:Description xmlns:xmp="http://ns.adobe.com/xap/1.0/">
      <xmp:CreateDate>2024-01-15T10:30:00</xmp:CreateDate>
    </rdf:Description>
  </rdf:RDF>
</x:xmpmeta>"#;

        let attrs = XmpParser::parse(xmp).unwrap();
        assert_eq!(attrs.get_str("XMP:CreateDate"), Some("2024-01-15T10:30:00"));
    }

    #[test]
    fn parse_struct_in_bag() {
        // Test parseType="Resource" inside rdf:Bag (like xapBJ:JobRef)
        let xmp = r#"<x:xmpmeta xmlns:x="adobe:ns:meta/">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
    <rdf:Description xmlns:xapBJ="http://ns.adobe.com/xap/1.0/bj/"
                     xmlns:stJob="http://ns.adobe.com/xap/1.0/sType/Job#">
      <xapBJ:JobRef>
        <rdf:Bag>
          <rdf:li rdf:parseType="Resource">
            <stJob:name>My Job</stJob:name>
            <stJob:id>job-123</stJob:id>
          </rdf:li>
        </rdf:Bag>
      </xapBJ:JobRef>
    </rdf:Description>
  </rdf:RDF>
</x:xmpmeta>"#;

        let attrs = XmpParser::parse(xmp).unwrap();
        // Struct fields should be accessible with parent.field notation
        assert_eq!(attrs.get_str("xapBJ:JobRef.name"), Some("My Job"));
        assert_eq!(attrs.get_str("xapBJ:JobRef.id"), Some("job-123"));
    }
}

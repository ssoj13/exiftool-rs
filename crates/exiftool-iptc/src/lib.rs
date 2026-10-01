//! IPTC-IIM metadata parser and writer.
//!
//! IPTC-IIM (International Press Telecommunications Council - Information Interchange Model)
//! is a standard for storing metadata in image files, commonly embedded in:
//! - JPEG APP13 segment (inside Photoshop IRB)
//! - TIFF IFD tag 33723 (0x83BB)
//! - PSD Image Resources
//!
//! Structure: Each dataset is encoded as:
//! ```text
//! 0x1C | record | dataset | size_hi | size_lo | data[size]
//! ```
//!
//! Records:
//! - Record 1: Envelope (transmission info)
//! - Record 2: Application (content metadata) - most commonly used
//! - Record 3: NewsPhoto (image-specific)
//! - Record 7-9: Pre/Object/Post data
//! - Record 240: FotoStation

mod tags;
mod error;

pub use error::{Error, Result};
pub use tags::*;

use exiftool_attrs::{AttrValue, Attrs};
use std::collections::HashMap;

/// IPTC record types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Record {
    Envelope = 1,
    Application = 2,
    NewsPhoto = 3,
    PreObjectData = 7,
    ObjectData = 8,
    PostObjectData = 9,
    FotoStation = 240,
}

impl Record {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Envelope),
            2 => Some(Self::Application),
            3 => Some(Self::NewsPhoto),
            7 => Some(Self::PreObjectData),
            8 => Some(Self::ObjectData),
            9 => Some(Self::PostObjectData),
            240 => Some(Self::FotoStation),
            _ => None,
        }
    }
}

/// Parsed IPTC dataset.
#[derive(Debug, Clone)]
pub struct Dataset {
    pub record: u8,
    pub tag: u8,
    pub data: Vec<u8>,
}

/// IPTC parser.
pub struct IptcParser;

impl IptcParser {
    /// Parse IPTC data block into Attrs.
    pub fn parse(data: &[u8]) -> Result<Attrs> {
        let datasets = Self::parse_datasets(data)?;
        Self::datasets_to_attrs(&datasets)
    }

    /// Parse raw datasets from IPTC block.
    pub fn parse_datasets(data: &[u8]) -> Result<Vec<Dataset>> {
        let mut datasets = Vec::new();
        let mut pos = 0;

        while pos + 5 <= data.len() {
            // Tag marker
            if data[pos] != 0x1C {
                pos += 1;
                continue;
            }

            let record = data[pos + 1];
            let tag = data[pos + 2];

            // Size can be standard (2 bytes) or extended (>32767)
            let size_indicator = u16::from_be_bytes([data[pos + 3], data[pos + 4]]);
            let (size, header_len) = if size_indicator & 0x8000 != 0 {
                // Extended size: size_indicator & 0x7FFF = number of bytes for size
                let size_bytes = (size_indicator & 0x7FFF) as usize;
                if pos + 5 + size_bytes > data.len() {
                    break;
                }
                let mut size: usize = 0;
                for i in 0..size_bytes {
                    size = (size << 8) | data[pos + 5 + i] as usize;
                }
                (size, 5 + size_bytes)
            } else {
                (size_indicator as usize, 5)
            };

            pos += header_len;

            if pos + size > data.len() {
                break;
            }

            datasets.push(Dataset {
                record,
                tag,
                data: data[pos..pos + size].to_vec(),
            });

            pos += size;
        }

        Ok(datasets)
    }

    /// Convert datasets to Attrs with proper grouping and list handling.
    fn datasets_to_attrs(datasets: &[Dataset]) -> Result<Attrs> {
        let mut attrs = Attrs::new();
        let mut lists: HashMap<String, Vec<String>> = HashMap::new();

        for ds in datasets {
            let (group, tag_info) = match ds.record {
                1 => ("IPTC:Envelope", tags::envelope_tag(ds.tag)),
                2 => ("IPTC", tags::application_tag(ds.tag)),
                3 => ("IPTC:NewsPhoto", tags::newsphoto_tag(ds.tag)),
                _ => continue,
            };

            let Some(info) = tag_info else { continue };

            let key = if group == "IPTC" {
                format!("IPTC:{}", info.name)
            } else {
                format!("{}:{}", group, info.name)
            };

            // Convert data to value based on format
            let value = match info.format {
                TagFormat::String | TagFormat::Text => {
                    decode_iptc_string(&ds.data)
                }
                TagFormat::Digits => {
                    String::from_utf8_lossy(&ds.data).to_string()
                }
                TagFormat::Int16u => {
                    if ds.data.len() >= 2 {
                        let v = u16::from_be_bytes([ds.data[0], ds.data[1]]);
                        v.to_string()
                    } else {
                        continue;
                    }
                }
                TagFormat::Binary => {
                    // Store as hex string
                    ds.data.iter().map(|b| format!("{:02x}", b)).collect::<String>()
                }
            };

            // Handle list tags (Keywords, etc.)
            if info.is_list {
                lists.entry(key).or_default().push(value);
            } else {
                attrs.set(&key, AttrValue::Str(value));
            }
        }

        // Convert lists to AttrValue::List
        for (key, values) in lists {
            let list: Vec<AttrValue> = values.into_iter().map(AttrValue::Str).collect();
            attrs.set(&key, AttrValue::List(list));
        }

        Ok(attrs)
    }
}

/// IPTC writer.
pub struct IptcWriter;

impl IptcWriter {
    /// Serialize Attrs to IPTC data block.
    pub fn write(attrs: &Attrs) -> Result<Vec<u8>> {
        let mut data = Vec::new();

        // Collect datasets from attrs
        let mut datasets: Vec<Dataset> = Vec::new();

        for (key, value) in attrs.iter() {
            // Parse key: "IPTC:Keywords" -> record 2, tag 25
            let (record, tag, info) = Self::parse_key(key)?;
            
            match value {
                AttrValue::List(items) if info.is_list => {
                    // Write each list item as separate dataset
                    for item in items {
                        if let AttrValue::Str(s) = item {
                            datasets.push(Dataset {
                                record,
                                tag,
                                data: encode_iptc_string(s),
                            });
                        }
                    }
                }
                AttrValue::Str(s) => {
                    datasets.push(Dataset {
                        record,
                        tag,
                        data: encode_iptc_string(s),
                    });
                }
                AttrValue::UInt(n) => {
                    datasets.push(Dataset {
                        record,
                        tag,
                        data: (*n as u16).to_be_bytes().to_vec(),
                    });
                }
                _ => continue,
            }
        }

        // Sort by record, then by tag
        datasets.sort_by_key(|d| (d.record, d.tag));

        // Serialize
        for ds in datasets {
            data.push(0x1C);
            data.push(ds.record);
            data.push(ds.tag);

            let size = ds.data.len();
            if size > 0x7FFF {
                // Extended size
                let size_bytes = if size > 0xFFFFFF { 4 } else if size > 0xFFFF { 3 } else { 2 };
                data.push(0x80 | size_bytes);
                for i in (0..size_bytes).rev() {
                    data.push((size >> (i * 8)) as u8);
                }
            } else {
                data.extend_from_slice(&(size as u16).to_be_bytes());
            }

            data.extend_from_slice(&ds.data);
        }

        Ok(data)
    }

    /// Parse IPTC key to (record, tag, info).
    fn parse_key(key: &str) -> Result<(u8, u8, &'static TagInfo)> {
        let key = key.strip_prefix("IPTC:").unwrap_or(key);
        
        // Check envelope tags
        if let Some(stripped) = key.strip_prefix("Envelope:") {
            if let Some((tag, info)) = tags::envelope_tag_by_name(stripped) {
                return Ok((1, tag, info));
            }
        }
        
        // Check newsphoto tags
        if let Some(stripped) = key.strip_prefix("NewsPhoto:") {
            if let Some((tag, info)) = tags::newsphoto_tag_by_name(stripped) {
                return Ok((3, tag, info));
            }
        }
        
        // Application record (default)
        if let Some((tag, info)) = tags::application_tag_by_name(key) {
            return Ok((2, tag, info));
        }

        Err(Error::UnknownTag(key.to_string()))
    }
}

/// Decode IPTC string (handles UTF-8 and Latin-1).
fn decode_iptc_string(data: &[u8]) -> String {
    // Try UTF-8 first
    if let Ok(s) = std::str::from_utf8(data) {
        return s.trim_end_matches('\0').to_string();
    }
    
    // Fallback to Latin-1
    data.iter()
        .filter(|&&b| b != 0)
        .map(|&b| b as char)
        .collect()
}

/// Encode string to IPTC (UTF-8).
fn encode_iptc_string(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        // Simple IPTC block: Record 2, Tag 105 (Headline) = "Test"
        let data = [
            0x1C, 0x02, 105, 0x00, 0x04, // Header: record 2, tag 105, size 4
            b'T', b'e', b's', b't',       // Data
        ];
        
        let attrs = IptcParser::parse(&data).unwrap();
        assert_eq!(
            attrs.get("IPTC:Headline"),
            Some(&AttrValue::Str("Test".to_string()))
        );
    }

    #[test]
    fn test_parse_byline_list() {
        // Byline is a list tag - single value becomes list
        let data = [
            0x1C, 0x02, 80, 0x00, 0x04, // Header: record 2, tag 80 (Byline), size 4
            b'J', b'o', b'h', b'n',      // Data
        ];
        
        let attrs = IptcParser::parse(&data).unwrap();
        // Byline is a list tag, so single value becomes a list
        if let Some(AttrValue::List(values)) = attrs.get("IPTC:Byline") {
            assert_eq!(values.len(), 1);
            assert_eq!(values[0], AttrValue::Str("John".to_string()));
        } else {
            panic!("Expected Byline as list");
        }
    }

    #[test]
    fn test_parse_keywords_list() {
        // Multiple Keywords
        let data = [
            0x1C, 0x02, 25, 0x00, 0x03, b'c', b'a', b't',     // "cat"
            0x1C, 0x02, 25, 0x00, 0x03, b'd', b'o', b'g',     // "dog"
            0x1C, 0x02, 25, 0x00, 0x05, b'b', b'i', b'r', b'd', b's', // "birds"
        ];
        
        let attrs = IptcParser::parse(&data).unwrap();
        
        if let Some(AttrValue::List(keywords)) = attrs.get("IPTC:Keywords") {
            assert_eq!(keywords.len(), 3);
        } else {
            panic!("Expected Keywords list");
        }
    }

    #[test]
    fn test_roundtrip() {
        let mut attrs = Attrs::new();
        attrs.set("IPTC:Headline", AttrValue::Str("Test Headline".to_string()));
        attrs.set("IPTC:City", AttrValue::Str("New York".to_string()));
        attrs.set("IPTC:Keywords", AttrValue::List(vec![
            AttrValue::Str("test".to_string()),
            AttrValue::Str("example".to_string()),
        ]));

        let data = IptcWriter::write(&attrs).unwrap();
        let parsed = IptcParser::parse(&data).unwrap();

        assert_eq!(
            parsed.get("IPTC:Headline"),
            Some(&AttrValue::Str("Test Headline".to_string()))
        );
        assert_eq!(
            parsed.get("IPTC:City"),
            Some(&AttrValue::Str("New York".to_string()))
        );
    }
}

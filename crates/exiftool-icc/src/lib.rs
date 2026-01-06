//! ICC Profile parser and writer.
//!
//! ICC (International Color Consortium) profiles define color spaces
//! and color management transforms.
//!
//! # Structure
//!
//! - 128-byte header with profile metadata
//! - Tag table with offset/size entries
//! - Tag data (various types)
//!
//! # Example
//!
//! ```no_run
//! use exiftool_icc::IccParser;
//!
//! let data = std::fs::read("profile.icc").unwrap();
//! let attrs = IccParser::parse(&data).unwrap();
//!
//! if let Some(desc) = attrs.get_str("ICC:ProfileDescription") {
//!     println!("Profile: {}", desc);
//! }
//! ```

mod error;
mod header;
mod tags;

pub use error::{Error, Result};
pub use header::{IccHeader, ProfileClass, RenderingIntent};

use exiftool_attrs::{AttrValue, Attrs};

/// ICC Profile parser.
pub struct IccParser;

impl IccParser {
    /// Parse ICC profile from bytes.
    pub fn parse(data: &[u8]) -> Result<Attrs> {
        if data.len() < 128 {
            return Err(Error::TooShort(data.len()));
        }

        let mut attrs = Attrs::new();

        // Parse header (128 bytes)
        let header = IccHeader::parse(&data[..128])?;
        header.to_attrs(&mut attrs);

        // Parse tag table
        if data.len() < 132 {
            return Ok(attrs);
        }

        let tag_count = u32::from_be_bytes([data[128], data[129], data[130], data[131]]) as usize;

        // Validate tag count
        if tag_count > 1000 || 132 + tag_count * 12 > data.len() {
            return Err(Error::InvalidTagCount(tag_count));
        }

        // Parse tags
        for i in 0..tag_count {
            let entry_offset = 132 + i * 12;
            let sig = &data[entry_offset..entry_offset + 4];
            let offset =
                u32::from_be_bytes([data[entry_offset + 4], data[entry_offset + 5], data[entry_offset + 6], data[entry_offset + 7]])
                    as usize;
            let size =
                u32::from_be_bytes([data[entry_offset + 8], data[entry_offset + 9], data[entry_offset + 10], data[entry_offset + 11]])
                    as usize;

            // Validate offset/size
            if offset + size > data.len() {
                continue;
            }

            let tag_data = &data[offset..offset + size];
            Self::parse_tag(&mut attrs, sig, tag_data);
        }

        Ok(attrs)
    }

    /// Parse a single tag.
    fn parse_tag(attrs: &mut Attrs, sig: &[u8], data: &[u8]) {
        let sig_str = String::from_utf8_lossy(sig).trim().to_string();
        let tag_name = tags::tag_name(&sig_str);

        if data.len() < 8 {
            return;
        }

        // Tag type signature (first 4 bytes)
        let type_sig = &data[0..4];

        match type_sig {
            // Text type
            b"text" => {
                if data.len() > 8 {
                    let text = String::from_utf8_lossy(&data[8..])
                        .trim_end_matches('\0')
                        .to_string();
                    attrs.set(format!("ICC:{}", tag_name), AttrValue::Str(text));
                }
            }
            // Multi-localized Unicode (mluc)
            b"mluc" => {
                if let Some(text) = Self::parse_mluc(data) {
                    attrs.set(format!("ICC:{}", tag_name), AttrValue::Str(text));
                }
            }
            // Description type (desc)
            b"desc" => {
                if let Some(text) = Self::parse_desc(data) {
                    attrs.set(format!("ICC:{}", tag_name), AttrValue::Str(text));
                }
            }
            // XYZ type
            b"XYZ " => {
                if data.len() >= 20 {
                    let x = Self::read_s15fixed16(&data[8..12]);
                    let y = Self::read_s15fixed16(&data[12..16]);
                    let z = Self::read_s15fixed16(&data[16..20]);
                    attrs.set(
                        format!("ICC:{}", tag_name),
                        AttrValue::Str(format!("{:.6} {:.6} {:.6}", x, y, z)),
                    );
                }
            }
            // Signature type (sig)
            b"sig " => {
                if data.len() >= 12 {
                    let sig_val = String::from_utf8_lossy(&data[8..12]).trim().to_string();
                    attrs.set(format!("ICC:{}", tag_name), AttrValue::Str(sig_val));
                }
            }
            // Curve type (curv)
            b"curv" => {
                if data.len() >= 12 {
                    let count = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);
                    if count == 0 {
                        attrs.set(format!("ICC:{}", tag_name), AttrValue::Str("Linear".to_string()));
                    } else if count == 1 && data.len() >= 14 {
                        let gamma = u16::from_be_bytes([data[12], data[13]]) as f64 / 256.0;
                        attrs.set(format!("ICC:{}", tag_name), AttrValue::Str(format!("Gamma {:.2}", gamma)));
                    } else {
                        attrs.set(format!("ICC:{}", tag_name), AttrValue::Str(format!("Curve ({} points)", count)));
                    }
                }
            }
            // Parametric curve (para)
            b"para" => {
                if data.len() >= 12 {
                    let func_type = u16::from_be_bytes([data[8], data[9]]);
                    attrs.set(
                        format!("ICC:{}", tag_name),
                        AttrValue::Str(format!("Parametric (type {})", func_type)),
                    );
                }
            }
            // Date/time type (dtim)
            b"dtim" => {
                if data.len() >= 20 {
                    let year = u16::from_be_bytes([data[8], data[9]]);
                    let month = u16::from_be_bytes([data[10], data[11]]);
                    let day = u16::from_be_bytes([data[12], data[13]]);
                    let hour = u16::from_be_bytes([data[14], data[15]]);
                    let min = u16::from_be_bytes([data[16], data[17]]);
                    let sec = u16::from_be_bytes([data[18], data[19]]);
                    attrs.set(
                        format!("ICC:{}", tag_name),
                        AttrValue::Str(format!(
                            "{:04}:{:02}:{:02} {:02}:{:02}:{:02}",
                            year, month, day, hour, min, sec
                        )),
                    );
                }
            }
            // View/measurement/etc - store type info
            _ => {
                let type_name = String::from_utf8_lossy(type_sig).trim().to_string();
                attrs.set(
                    format!("ICC:{}", tag_name),
                    AttrValue::Str(format!("[{} data, {} bytes]", type_name, data.len())),
                );
            }
        }
    }

    /// Parse multi-localized unicode string.
    fn parse_mluc(data: &[u8]) -> Option<String> {
        if data.len() < 16 {
            return None;
        }

        let record_count = u32::from_be_bytes([data[8], data[9], data[10], data[11]]) as usize;
        let record_size = u32::from_be_bytes([data[12], data[13], data[14], data[15]]) as usize;

        if record_count == 0 || record_size < 12 {
            return None;
        }

        // Get first record (usually en-US)
        let record_offset = 16;
        if data.len() < record_offset + 12 {
            return None;
        }

        let str_len = u32::from_be_bytes([
            data[record_offset + 4],
            data[record_offset + 5],
            data[record_offset + 6],
            data[record_offset + 7],
        ]) as usize;
        let str_offset = u32::from_be_bytes([
            data[record_offset + 8],
            data[record_offset + 9],
            data[record_offset + 10],
            data[record_offset + 11],
        ]) as usize;

        if str_offset + str_len > data.len() {
            return None;
        }

        // UTF-16BE decode
        let utf16_data = &data[str_offset..str_offset + str_len];
        let utf16: Vec<u16> = utf16_data
            .chunks_exact(2)
            .map(|c| u16::from_be_bytes([c[0], c[1]]))
            .collect();

        String::from_utf16(&utf16).ok().map(|s| s.trim_end_matches('\0').to_string())
    }

    /// Parse description type (older format).
    fn parse_desc(data: &[u8]) -> Option<String> {
        if data.len() < 12 {
            return None;
        }

        let ascii_len = u32::from_be_bytes([data[8], data[9], data[10], data[11]]) as usize;
        if ascii_len == 0 || 12 + ascii_len > data.len() {
            return None;
        }

        let text = String::from_utf8_lossy(&data[12..12 + ascii_len])
            .trim_end_matches('\0')
            .to_string();
        Some(text)
    }

    /// Read s15Fixed16Number (signed 15.16 fixed point).
    fn read_s15fixed16(data: &[u8]) -> f64 {
        let raw = i32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        raw as f64 / 65536.0
    }

    /// Extract raw ICC profile bytes from image metadata.
    pub fn extract_profile(attrs: &Attrs) -> Option<&[u8]> {
        attrs.get_bytes("ICC_Profile")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_header() -> Vec<u8> {
        let mut data = vec![0u8; 128];
        // Profile size (128 bytes minimal)
        data[0..4].copy_from_slice(&128u32.to_be_bytes());
        // CMM type
        data[4..8].copy_from_slice(b"ADBE");
        // Version 4.3.0: byte 8 = major (4), byte 9 = minor.bugfix (0x30 = 3.0)
        data[8] = 4;
        data[9] = 0x30;
        // Profile class: mntr (display)
        data[12..16].copy_from_slice(b"mntr");
        // Color space: RGB
        data[16..20].copy_from_slice(b"RGB ");
        // PCS: XYZ
        data[20..24].copy_from_slice(b"XYZ ");
        // Date/time: 2024-01-15 12:30:00
        data[24..26].copy_from_slice(&2024u16.to_be_bytes());
        data[26..28].copy_from_slice(&1u16.to_be_bytes());
        data[28..30].copy_from_slice(&15u16.to_be_bytes());
        data[30..32].copy_from_slice(&12u16.to_be_bytes());
        data[32..34].copy_from_slice(&30u16.to_be_bytes());
        data[34..36].copy_from_slice(&0u16.to_be_bytes());
        // Signature: acsp
        data[36..40].copy_from_slice(b"acsp");
        // Platform: APPL
        data[40..44].copy_from_slice(b"APPL");
        // Rendering intent: perceptual (0)
        data[64..68].copy_from_slice(&0u32.to_be_bytes());
        data
    }

    #[test]
    fn test_parse_header() {
        let data = make_header();
        let attrs = IccParser::parse(&data).unwrap();

        assert_eq!(attrs.get_str("ICC:ProfileCMMType"), Some("ADBE"));
        assert_eq!(attrs.get_str("ICC:ProfileVersion"), Some("4.3.0"));
        assert_eq!(attrs.get_str("ICC:ProfileClass"), Some("Display Device Profile"));
        assert_eq!(attrs.get_str("ICC:ColorSpaceData"), Some("RGB"));
        assert_eq!(attrs.get_str("ICC:RenderingIntent"), Some("Perceptual"));
    }

    #[test]
    fn test_parse_with_tags() {
        let mut data = make_header();
        // Update size
        let total_size = 128 + 4 + 12 + 32; // header + count + 1 entry + tag data
        data[0..4].copy_from_slice(&(total_size as u32).to_be_bytes());

        // Tag count: 1
        data.extend_from_slice(&1u32.to_be_bytes());

        // Tag entry: desc at offset 144, size 32
        data.extend_from_slice(b"desc");
        data.extend_from_slice(&144u32.to_be_bytes());
        data.extend_from_slice(&32u32.to_be_bytes());

        // Tag data: text type with "Test Profile"
        data.extend_from_slice(b"text"); // type
        data.extend_from_slice(&[0, 0, 0, 0]); // reserved
        data.extend_from_slice(b"Test Profile\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");

        let attrs = IccParser::parse(&data).unwrap();
        assert_eq!(attrs.get_str("ICC:ProfileDescription"), Some("Test Profile"));
    }

    #[test]
    fn test_too_short() {
        let data = vec![0u8; 64];
        assert!(matches!(IccParser::parse(&data), Err(Error::TooShort(_))));
    }
}

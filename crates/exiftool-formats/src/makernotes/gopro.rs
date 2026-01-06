//! GoPro MakerNotes/GPMF parser.
//!
//! GoPro uses GPMF (GoPro Metadata Format) - a KLV-style binary format.
//! Found in:
//! - APP6 "GoPro" segment in JPEG files
//! - 'GPMF' box in MP4 files
//!
//! GPMF Structure:
//! - 4-byte FourCC key
//! - 1-byte type (char code)
//! - 1-byte structure size
//! - 2-byte repeat count (big-endian)
//! - Data (padded to 4-byte boundary)
//!
//! Types: 'b'=int8, 'B'=uint8, 's'=int16, 'S'=uint16, 'l'=int32, 'L'=uint32,
//!        'f'=float32, 'd'=float64, 'c'=char, 'U'=UTC date, 'F'=FourCC, etc.

use super::{Vendor, VendorParser};
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::ByteOrder;
use exiftool_tags::generated::gopro;

/// GoPro GPMF parser.
pub struct GoProParser;

impl VendorParser for GoProParser {
    fn vendor(&self) -> Vendor {
        Vendor::GoPro
    }

    fn parse(&self, data: &[u8], _byte_order: ByteOrder) -> Option<Attrs> {
        // Check for "GoPro" header (APP6 segment)
        if data.len() < 6 {
            return None;
        }

        let mut attrs = Attrs::new();
        let start_offset;

        // APP6 starts with "GoPro\0"
        if data.len() >= 6 && &data[0..6] == b"GoPro\0" {
            start_offset = 6;
        } else {
            // Raw GPMF data (from MP4)
            start_offset = 0;
        }

        // Parse GPMF KLV entries
        parse_gpmf(&data[start_offset..], &mut attrs, "");

        if attrs.is_empty() {
            return None;
        }

        Some(attrs)
    }
}

/// Parse GPMF data recursively.
fn parse_gpmf(data: &[u8], attrs: &mut Attrs, prefix: &str) {
    let mut offset = 0;

    while offset + 8 <= data.len() {
        // FourCC key (4 bytes)
        let fourcc = &data[offset..offset + 4];
        let fourcc_str = String::from_utf8_lossy(fourcc).to_string();

        // Type (1 byte)
        let type_char = data[offset + 4] as char;

        // Structure size (1 byte)
        let struct_size = data[offset + 5] as usize;

        // Repeat count (2 bytes, big-endian)
        let repeat = u16::from_be_bytes([data[offset + 6], data[offset + 7]]) as usize;

        // Data size (with 4-byte padding)
        let data_size = struct_size * repeat;
        let padded_size = (data_size + 3) & !3;

        offset += 8;

        if offset + padded_size > data.len() {
            break;
        }

        let value_data = &data[offset..offset + data_size];

        // Nested container (type 0 or null)
        if type_char == '\0' || type_char == '?' {
            let new_prefix = if prefix.is_empty() {
                fourcc_str.clone()
            } else {
                format!("{}:{}", prefix, fourcc_str)
            };
            parse_gpmf(value_data, attrs, &new_prefix);
        } else {
            // Leaf value - extract and store
            let tag_name = if let Some(tag_def) = gopro::lookup(&fourcc_str) {
                tag_def.name.to_string()
            } else {
                fourcc_str.clone()
            };

            let full_name = if prefix.is_empty() {
                tag_name
            } else {
                format!("{}:{}", prefix, tag_name)
            };

            if let Some(value) = extract_value(type_char, struct_size, repeat, value_data) {
                attrs.set(&full_name, value);
            }
        }

        offset += padded_size;
    }
}

/// Extract typed value from GPMF data.
fn extract_value(type_char: char, struct_size: usize, repeat: usize, data: &[u8]) -> Option<AttrValue> {
    match type_char {
        // String types
        'c' | 'U' => {
            let s = String::from_utf8_lossy(data)
                .trim_end_matches('\0')
                .to_string();
            Some(AttrValue::Str(s))
        }

        // FourCC
        'F' => {
            if data.len() >= 4 {
                let s = String::from_utf8_lossy(&data[0..4]).to_string();
                Some(AttrValue::Str(s))
            } else {
                None
            }
        }

        // Signed integers
        'b' => {
            if repeat == 1 {
                Some(AttrValue::Int(data[0] as i8 as i32))
            } else {
                let vals: Vec<String> = data.iter().take(repeat).map(|&v| (v as i8).to_string()).collect();
                Some(AttrValue::Str(vals.join(" ")))
            }
        }

        's' => {
            if struct_size == 2 {
                if repeat == 1 && data.len() >= 2 {
                    let v = i16::from_be_bytes([data[0], data[1]]);
                    Some(AttrValue::Int(v as i32))
                } else {
                    let vals: Vec<String> = data
                        .chunks_exact(2)
                        .take(repeat)
                        .map(|c| i16::from_be_bytes([c[0], c[1]]).to_string())
                        .collect();
                    Some(AttrValue::Str(vals.join(" ")))
                }
            } else {
                None
            }
        }

        'l' => {
            if struct_size == 4 {
                if repeat == 1 && data.len() >= 4 {
                    let v = i32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                    Some(AttrValue::Int(v))
                } else {
                    let vals: Vec<String> = data
                        .chunks_exact(4)
                        .take(repeat)
                        .map(|c| i32::from_be_bytes([c[0], c[1], c[2], c[3]]).to_string())
                        .collect();
                    Some(AttrValue::Str(vals.join(" ")))
                }
            } else {
                None
            }
        }

        // Unsigned integers
        'B' => {
            if repeat == 1 {
                Some(AttrValue::Int(data[0] as i32))
            } else {
                let vals: Vec<String> = data.iter().take(repeat).map(|&v| v.to_string()).collect();
                Some(AttrValue::Str(vals.join(" ")))
            }
        }

        'S' => {
            if struct_size == 2 {
                if repeat == 1 && data.len() >= 2 {
                    let v = u16::from_be_bytes([data[0], data[1]]);
                    Some(AttrValue::Int(v as i32))
                } else {
                    let vals: Vec<String> = data
                        .chunks_exact(2)
                        .take(repeat)
                        .map(|c| u16::from_be_bytes([c[0], c[1]]).to_string())
                        .collect();
                    Some(AttrValue::Str(vals.join(" ")))
                }
            } else {
                None
            }
        }

        'L' => {
            if struct_size == 4 {
                if repeat == 1 && data.len() >= 4 {
                    let v = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                    Some(AttrValue::Int(v as i32))
                } else {
                    let vals: Vec<String> = data
                        .chunks_exact(4)
                        .take(repeat)
                        .map(|c| u32::from_be_bytes([c[0], c[1], c[2], c[3]]).to_string())
                        .collect();
                    Some(AttrValue::Str(vals.join(" ")))
                }
            } else {
                None
            }
        }

        // Float
        'f' => {
            if struct_size == 4 {
                if repeat == 1 && data.len() >= 4 {
                    let v = f32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                    Some(AttrValue::Float(v))
                } else {
                    let vals: Vec<String> = data
                        .chunks_exact(4)
                        .take(repeat)
                        .map(|c| {
                            let v = f32::from_be_bytes([c[0], c[1], c[2], c[3]]);
                            format!("{:.6}", v)
                        })
                        .collect();
                    Some(AttrValue::Str(vals.join(" ")))
                }
            } else {
                None
            }
        }

        // Double
        'd' => {
            if struct_size == 8 {
                if repeat == 1 && data.len() >= 8 {
                    let v = f64::from_be_bytes([
                        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
                    ]);
                    Some(AttrValue::Double(v))
                } else {
                    let vals: Vec<String> = data
                        .chunks_exact(8)
                        .take(repeat)
                        .map(|c| {
                            let v = f64::from_be_bytes([
                                c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7],
                            ]);
                            format!("{:.6}", v)
                        })
                        .collect();
                    Some(AttrValue::Str(vals.join(" ")))
                }
            } else {
                None
            }
        }

        // 64-bit timestamp
        'J' => {
            if struct_size == 8 && data.len() >= 8 {
                let v = u64::from_be_bytes([
                    data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
                ]);
                Some(AttrValue::Str(v.to_string()))
            } else {
                None
            }
        }

        // GUID
        'G' => {
            if data.len() >= 16 {
                let guid = format!(
                    "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                    data[0], data[1], data[2], data[3],
                    data[4], data[5],
                    data[6], data[7],
                    data[8], data[9],
                    data[10], data[11], data[12], data[13], data[14], data[15]
                );
                Some(AttrValue::Str(guid))
            } else {
                None
            }
        }

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_vendor() {
        assert_eq!(GoProParser.vendor(), Vendor::GoPro);
    }

    #[test]
    fn parse_simple_gpmf() {
        // DVNM (DeviceName) - type 'c', size 1, repeat 10
        // "GoPro Max\0" padded to 12 bytes
        let mut data = Vec::new();
        data.extend_from_slice(b"DVNM"); // FourCC
        data.push(b'c'); // type = char
        data.push(1); // struct size = 1
        data.extend_from_slice(&10u16.to_be_bytes()); // repeat = 10
        data.extend_from_slice(b"GoPro Max\0"); // data (10 bytes)
        data.extend_from_slice(&[0, 0]); // padding to 12

        let attrs = GoProParser.parse(&data, ByteOrder::BigEndian).unwrap();
        assert_eq!(attrs.get_str("DeviceName"), Some("GoPro Max"));
    }

    #[test]
    fn parse_with_gopro_header() {
        // APP6 with "GoPro\0" prefix
        let mut data = Vec::new();
        data.extend_from_slice(b"GoPro\0"); // APP6 header

        // DVNM entry
        data.extend_from_slice(b"DVNM");
        data.push(b'c');
        data.push(1);
        data.extend_from_slice(&8u16.to_be_bytes());
        data.extend_from_slice(b"Hero 12\0");

        let attrs = GoProParser.parse(&data, ByteOrder::BigEndian).unwrap();
        assert_eq!(attrs.get_str("DeviceName"), Some("Hero 12"));
    }

    #[test]
    fn parse_numeric_value() {
        // TMPC (Temperature) - type 'f', size 4, repeat 1
        let mut data = Vec::new();
        data.extend_from_slice(b"TMPC");
        data.push(b'f');
        data.push(4);
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&25.5f32.to_be_bytes());

        let attrs = GoProParser.parse(&data, ByteOrder::BigEndian).unwrap();
        if let Some(temp) = attrs.get_f32("Temperature") {
            assert!((temp - 25.5).abs() < 0.01);
        }
    }
}

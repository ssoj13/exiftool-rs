//! MXF (Material eXchange Format) parser.
//!
//! MXF is SMPTE standard for professional video exchange.
//!
//! # Structure
//!
//! - KLV (Key-Length-Value) triplets
//! - Header partition with metadata
//! - Body partitions with essence
//! - Footer partition with index

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

// MXF Partition Pack Key prefix (first 13 bytes)
const PARTITION_PACK_KEY: [u8; 13] = [
    0x06, 0x0E, 0x2B, 0x34, 0x02, 0x05, 0x01, 0x01,
    0x0D, 0x01, 0x02, 0x01, 0x01,
];

// Preface Set Key
const PREFACE_KEY: [u8; 16] = [
    0x06, 0x0E, 0x2B, 0x34, 0x02, 0x53, 0x01, 0x01,
    0x0D, 0x01, 0x01, 0x01, 0x01, 0x01, 0x2F, 0x00,
];

// Identification Set Key  
const IDENTIFICATION_KEY: [u8; 16] = [
    0x06, 0x0E, 0x2B, 0x34, 0x02, 0x53, 0x01, 0x01,
    0x0D, 0x01, 0x01, 0x01, 0x01, 0x01, 0x30, 0x00,
];

/// MXF format parser.
pub struct MxfParser;

impl FormatParser for MxfParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 14 {
            return false;
        }
        // Check partition pack key prefix
        header[0..13] == PARTITION_PACK_KEY
    }

    fn format_name(&self) -> &'static str {
        "MXF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["mxf"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("MXF");
        meta.exif.set("File:FileType", AttrValue::Str("MXF".to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("application/mxf".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // File size
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        reader.seek(SeekFrom::Start(0))?;

        // Parse header partition pack
        let mut key = [0u8; 16];
        reader.read_exact(&mut key)?;

        if key[0..13] != PARTITION_PACK_KEY {
            return Ok(meta);
        }

        // Partition type from byte 13
        let partition_type = match key[13] {
            0x02 => "Header (Open Incomplete)",
            0x03 => "Header (Closed Incomplete)",
            0x04 => "Header (Open Complete)",
            0x05 => "Header (Closed Complete)",
            _ => "Unknown",
        };
        meta.exif.set("MXF:PartitionType", AttrValue::Str(partition_type.to_string()));

        // Read BER length
        let length = read_ber_length(reader)?;
        if length < 64 {
            return Ok(meta);
        }

        // Parse partition pack
        let mut pack = vec![0u8; length.min(256) as usize];
        reader.read_exact(&mut pack)?;

        // Major/Minor version (bytes 0-3)
        let major = u16::from_be_bytes([pack[0], pack[1]]);
        let minor = u16::from_be_bytes([pack[2], pack[3]]);
        meta.exif.set("MXF:Version", AttrValue::Str(format!("{}.{}", major, minor)));

        // KAG size (bytes 4-7)
        let kag_size = u32::from_be_bytes([pack[4], pack[5], pack[6], pack[7]]);
        if kag_size > 0 {
            meta.exif.set("MXF:KAGSize", AttrValue::UInt(kag_size));
        }

        // This partition offset (bytes 8-15)
        // Previous partition offset (bytes 16-23)
        // Footer partition offset (bytes 24-31)
        
        // Header byte count (bytes 32-39)
        let header_size = u64::from_be_bytes([
            pack[32], pack[33], pack[34], pack[35],
            pack[36], pack[37], pack[38], pack[39],
        ]);
        if header_size > 0 {
            meta.exif.set("MXF:HeaderSize", AttrValue::UInt64(header_size));
        }

        // Index byte count (bytes 40-47)
        // Index SID (bytes 48-51)
        // Body offset (bytes 52-59)
        // Body SID (bytes 60-63)

        // Operational pattern (bytes 64-79, UL)
        if pack.len() >= 80 {
            let op = parse_operational_pattern(&pack[64..80]);
            meta.exif.set("MXF:OperationalPattern", AttrValue::Str(op));
        }

        // Now scan for metadata sets
        let header_end = (header_size + 200).min(file_size);
        
        while reader.stream_position()? < header_end {
            let mut set_key = [0u8; 16];
            if reader.read_exact(&mut set_key).is_err() {
                break;
            }

            let set_len = match read_ber_length(reader) {
                Ok(l) => l,
                Err(_) => break,
            };

            if set_len == 0 || set_len > 100000 {
                break;
            }

            let set_start = reader.stream_position()?;

            // Check for known sets
            if set_key[0..15] == PREFACE_KEY[0..15] {
                parse_preface_set(reader, set_len, &mut meta)?;
            } else if set_key[0..15] == IDENTIFICATION_KEY[0..15] {
                parse_identification_set(reader, set_len, &mut meta)?;
            }

            // Skip to next KLV
            reader.seek(SeekFrom::Start(set_start + set_len))?;
        }

        Ok(meta)
    }
}

/// Read BER-encoded length.
fn read_ber_length(reader: &mut dyn ReadSeek) -> Result<u64> {
    let mut first = [0u8; 1];
    reader.read_exact(&mut first)?;

    if first[0] < 0x80 {
        // Short form
        Ok(first[0] as u64)
    } else {
        // Long form
        let num_bytes = (first[0] & 0x7F) as usize;
        if num_bytes > 8 {
            return Ok(0);
        }
        
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes[8 - num_bytes..])?;
        Ok(u64::from_be_bytes(bytes))
    }
}

/// Parse operational pattern UL.
fn parse_operational_pattern(ul: &[u8]) -> String {
    if ul.len() < 16 {
        return "Unknown".to_string();
    }

    // Bytes 12-13 indicate the pattern
    let item_complexity = ul[12];
    let package_complexity = ul[13];

    let item = match item_complexity {
        0x01 => "1",
        0x02 => "2",
        0x03 => "3",
        0x10 => "Atom",
        _ => "?",
    };

    let pkg = match package_complexity {
        0x01 => "a",
        0x02 => "b",
        0x03 => "c",
        _ => "?",
    };

    format!("OP{}{}", item, pkg)
}

/// Parse Preface set for creation date.
fn parse_preface_set(reader: &mut dyn ReadSeek, _len: u64, meta: &mut Metadata) -> Result<()> {
    // Preface contains local tags, simplified parsing
    let mut buf = [0u8; 256];
    let read = reader.read(&mut buf)?;

    // Look for ModificationDate tag (0x3B02)
    for i in 0..read.saturating_sub(10) {
        if buf[i] == 0x3B && buf[i + 1] == 0x02 {
            // Found tag, length follows
            let tag_len = u16::from_be_bytes([buf[i + 2], buf[i + 3]]) as usize;
            if tag_len == 8 && i + 4 + 8 <= read {
                // MXF timestamp is 8 bytes
                let ts = &buf[i + 4..i + 12];
                let year = u16::from_be_bytes([ts[0], ts[1]]);
                let month = ts[2];
                let day = ts[3];
                let hour = ts[4];
                let min = ts[5];
                let sec = ts[6];

                meta.exif.set("MXF:ModificationDate", AttrValue::Str(
                    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hour, min, sec)
                ));
                break;
            }
        }
    }

    Ok(())
}

/// Parse Identification set for product info.
fn parse_identification_set(reader: &mut dyn ReadSeek, _len: u64, meta: &mut Metadata) -> Result<()> {
    let mut buf = [0u8; 512];
    let read = reader.read(&mut buf)?;

    // Look for ProductName tag (0x3C01) - UTF-16BE string
    for i in 0..read.saturating_sub(6) {
        if buf[i] == 0x3C && buf[i + 1] == 0x01 {
            let tag_len = u16::from_be_bytes([buf[i + 2], buf[i + 3]]) as usize;
            if tag_len > 0 && tag_len < 256 && i + 4 + tag_len <= read {
                let str_data = &buf[i + 4..i + 4 + tag_len];
                let s = decode_utf16be(str_data);
                if !s.is_empty() {
                    meta.exif.set("MXF:ProductName", AttrValue::Str(s));
                }
            }
        }
        // CompanyName (0x3C02)
        else if buf[i] == 0x3C && buf[i + 1] == 0x02 {
            let tag_len = u16::from_be_bytes([buf[i + 2], buf[i + 3]]) as usize;
            if tag_len > 0 && tag_len < 256 && i + 4 + tag_len <= read {
                let str_data = &buf[i + 4..i + 4 + tag_len];
                let s = decode_utf16be(str_data);
                if !s.is_empty() {
                    meta.exif.set("MXF:CompanyName", AttrValue::Str(s));
                }
            }
        }
        // ProductVersion (0x3C04)
        else if buf[i] == 0x3C && buf[i + 1] == 0x04 {
            let tag_len = u16::from_be_bytes([buf[i + 2], buf[i + 3]]) as usize;
            if tag_len >= 10 && i + 4 + tag_len <= read {
                let ver = &buf[i + 4..i + 4 + tag_len];
                let major = u16::from_be_bytes([ver[0], ver[1]]);
                let minor = u16::from_be_bytes([ver[2], ver[3]]);
                let patch = u16::from_be_bytes([ver[4], ver[5]]);
                meta.exif.set("MXF:ProductVersion", AttrValue::Str(
                    format!("{}.{}.{}", major, minor, patch)
                ));
            }
        }
    }

    Ok(())
}

/// Decode UTF-16BE string.
fn decode_utf16be(data: &[u8]) -> String {
    let u16_chars: Vec<u16> = data
        .chunks_exact(2)
        .map(|c| u16::from_be_bytes([c[0], c[1]]))
        .take_while(|&c| c != 0)
        .collect();
    String::from_utf16_lossy(&u16_chars)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_mxf_header() -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // Partition pack key
        data[0..13].copy_from_slice(&PARTITION_PACK_KEY);
        data[13] = 0x04; // Header Open Complete
        data[14..16].copy_from_slice(&[0x00, 0x00]);
        // BER length (short form)
        data[16] = 88; // Pack is 88 bytes
        // Pack data
        // Major version
        data[17..19].copy_from_slice(&1u16.to_be_bytes());
        // Minor version
        data[19..21].copy_from_slice(&3u16.to_be_bytes());
        // KAG size
        data[21..25].copy_from_slice(&512u32.to_be_bytes());
        
        data
    }

    #[test]
    fn test_can_parse() {
        let parser = MxfParser;
        let data = make_mxf_header();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = MxfParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"RIFF"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = MxfParser;
        let data = make_mxf_header();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "MXF");
        assert_eq!(meta.exif.get_str("MXF:PartitionType"), Some("Header (Open Complete)"));
    }

    #[test]
    fn test_format_info() {
        let parser = MxfParser;
        assert_eq!(parser.format_name(), "MXF");
        assert!(parser.extensions().contains(&"mxf"));
    }

    #[test]
    fn test_ber_length_short() {
        let data = vec![0x40u8, 0x00, 0x00];
        let mut cursor = Cursor::new(data);
        assert_eq!(read_ber_length(&mut cursor).unwrap(), 64);
    }

    #[test]
    fn test_ber_length_long() {
        let data = vec![0x82u8, 0x01, 0x00]; // 256
        let mut cursor = Cursor::new(data);
        assert_eq!(read_ber_length(&mut cursor).unwrap(), 256);
    }
}

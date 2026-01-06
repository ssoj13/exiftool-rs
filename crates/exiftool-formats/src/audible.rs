//! Audible audiobook format parser.
//!
//! Supports:
//! - AA format (legacy Audible)
//! - AAX format (modern, MPEG-4 based - detected by Mp4Parser)
//!
//! # AA Structure
//!
//! - Magic: varies by version
//! - Header with metadata table
//! - Content is encrypted

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::{Read, Seek, SeekFrom};

/// Audible AA format parser.
pub struct AudibleParser;

impl FormatParser for AudibleParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 12 {
            return false;
        }
        // AA format magic patterns
        // Version 2/3: starts with 57 90 75 36
        if header[0] == 0x57 && header[1] == 0x90 && header[2] == 0x75 && header[3] == 0x36 {
            return true;
        }
        // Version 4+: has ftyp box with "aax " or "aaxc"
        // (handled by Mp4Parser, but we check anyway)
        if &header[4..8] == b"ftyp" {
            if header.len() >= 12 {
                let brand = &header[8..12];
                if brand == b"aax " || brand == b"aaxc" || brand == b"M4A " {
                    return true;
                }
            }
        }
        false
    }

    fn format_name(&self) -> &'static str {
        "AA"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["aa", "aax", "aaxc"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("AA");
        meta.exif.set("File:FileType", AttrValue::Str("AA".to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("audio/vnd.audible.aax".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // Check magic
        let mut header = [0u8; 16];
        reader.read_exact(&mut header)?;

        // File size
        let file_size = reader.seek(SeekFrom::End(0))?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        // Check if MPEG-4 based (AAX)
        if &header[4..8] == b"ftyp" {
            meta.format = "AAX";
            meta.exif.set("File:FileType", AttrValue::Str("AAX".to_string()));
            meta.exif.set("Audible:Format", AttrValue::Str("AAX (MPEG-4)".to_string()));
            // AAX parsing is handled better by Mp4Parser
            return Ok(meta);
        }

        // Legacy AA format
        reader.seek(SeekFrom::Start(0))?;

        // Parse AA header
        parse_aa_header(reader, &mut meta)?;

        Ok(meta)
    }
}

/// Parse legacy AA format header.
fn parse_aa_header(reader: &mut dyn ReadSeek, meta: &mut Metadata) -> Result<()> {
    // Read first chunk to determine version
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;

    // AA file has a table of contents at a fixed offset
    // The format varies by version, but generally:
    // - Magic (4 bytes)
    // - File size (4 bytes)
    // - Magic2 (4 bytes)
    // - TOC offset (4 bytes)

    let mut size_bytes = [0u8; 4];
    reader.read_exact(&mut size_bytes)?;
    let _file_size = u32::from_be_bytes(size_bytes);

    let mut magic2 = [0u8; 4];
    reader.read_exact(&mut magic2)?;

    let mut toc_offset_bytes = [0u8; 4];
    reader.read_exact(&mut toc_offset_bytes)?;
    let toc_offset = u32::from_be_bytes(toc_offset_bytes);

    meta.exif.set("Audible:Format", AttrValue::Str("AA (Legacy)".to_string()));
    meta.exif.set("Audible:TOCOffset", AttrValue::UInt(toc_offset));

    // Try to read TOC if valid
    if toc_offset > 16 && toc_offset < 100000 {
        reader.seek(SeekFrom::Start(toc_offset as u64))?;
        
        // TOC has entries: name (null-terminated) + offset + size
        let mut toc_header = [0u8; 4];
        if reader.read_exact(&mut toc_header).is_ok() {
            let num_entries = u32::from_be_bytes(toc_header);
            meta.exif.set("Audible:TOCEntries", AttrValue::UInt(num_entries.min(100)));

            // Parse metadata entries
            for _ in 0..num_entries.min(50) {
                if parse_aa_toc_entry(reader, meta).is_err() {
                    break;
                }
            }
        }
    }

    Ok(())
}

/// Parse a single TOC entry.
fn parse_aa_toc_entry(reader: &mut dyn ReadSeek, meta: &mut Metadata) -> Result<()> {
    // Entry format: key_len (4) + key + value_len (4) + value
    let mut len_bytes = [0u8; 4];
    reader.read_exact(&mut len_bytes)?;
    let key_len = u32::from_be_bytes(len_bytes) as usize;
    
    if key_len == 0 || key_len > 256 {
        return Ok(());
    }

    let mut key_buf = vec![0u8; key_len];
    reader.read_exact(&mut key_buf)?;
    let key = String::from_utf8_lossy(&key_buf).trim_end_matches('\0').to_string();

    reader.read_exact(&mut len_bytes)?;
    let value_len = u32::from_be_bytes(len_bytes) as usize;

    if value_len == 0 || value_len > 10000 {
        return Ok(());
    }

    let mut value_buf = vec![0u8; value_len];
    reader.read_exact(&mut value_buf)?;
    let value = String::from_utf8_lossy(&value_buf).trim_end_matches('\0').to_string();

    // Map known Audible metadata keys
    let tag_name = match key.as_str() {
        "title" => "Audio:Title",
        "author" => "Audio:Artist",
        "narrator" => "Audible:Narrator",
        "pubdate" => "Audible:PublishDate",
        "publisher" => "Audible:Publisher",
        "codec" => "Audio:Codec",
        "copyright" => "Audible:Copyright",
        "description" | "short_description" => "Audible:Description",
        "long_description" => "Audible:LongDescription",
        "keywords" => "Audible:Keywords",
        "product_id" => "Audible:ProductID",
        "asin" => "Audible:ASIN",
        "duration" => {
            if let Ok(dur) = value.parse::<f64>() {
                meta.exif.set("Audio:Duration", AttrValue::Double(dur / 1000.0));
            }
            return Ok(());
        }
        "content_format" => "Audible:ContentFormat",
        _ => return Ok(()),
    };

    if !value.is_empty() {
        meta.exif.set(tag_name, AttrValue::Str(value));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_aa_header() -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // AA magic
        data[0..4].copy_from_slice(&[0x57, 0x90, 0x75, 0x36]);
        // File size
        data[4..8].copy_from_slice(&1000u32.to_be_bytes());
        // Magic2
        data[8..12].copy_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        // TOC offset (invalid for simple test)
        data[12..16].copy_from_slice(&500u32.to_be_bytes());
        data
    }

    #[test]
    fn test_can_parse_aa() {
        let parser = AudibleParser;
        let data = make_aa_header();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = AudibleParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"RIFF"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = AudibleParser;
        let data = make_aa_header();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "AA");
        assert_eq!(meta.exif.get_str("Audible:Format"), Some("AA (Legacy)"));
    }

    #[test]
    fn test_format_info() {
        let parser = AudibleParser;
        assert_eq!(parser.format_name(), "AA");
        assert!(parser.extensions().contains(&"aa"));
        assert!(parser.extensions().contains(&"aax"));
    }
}

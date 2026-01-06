//! Common RIFF container utilities.
//!
//! RIFF (Resource Interchange File Format) is used by WAV, AVI, and other formats.
//! This module provides common parsing functions for RIFF chunks.

use crate::{Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;

/// Map RIFF INFO chunk tag to human-readable name.
pub fn info_tag_name(tag: &[u8; 4]) -> &'static str {
    match tag {
        b"INAM" => "Title",
        b"IART" => "Artist",
        b"ICMT" => "Comment",
        b"ICOP" => "Copyright",
        b"ICRD" => "DateCreated",
        b"IGNR" => "Genre",
        b"IKEY" => "Keywords",
        b"IMED" => "Medium",
        b"IPRD" => "Product",
        b"ISBJ" => "Subject",
        b"ISFT" => "Software",
        b"ISRC" => "Source",
        b"ISRF" => "SourceForm",
        b"ITCH" => "Technician",
        b"IENG" => "Engineer",
        b"IDIM" => "Dimensions",
        b"IDIT" => "DateTimeOriginal",
        b"ILNG" => "Language",
        b"IPLT" => "Palette",
        b"IPRT" => "Part",
        b"ITRK" => "TrackNumber",
        b"IWRI" => "WrittenBy",
        b"IWMU" => "WatermarkURL",
        _ => "Unknown",
    }
}

/// Parse RIFF INFO chunk containing metadata tags.
///
/// INFO chunks contain null-terminated strings with tags like INAM, IART, etc.
pub fn parse_info(reader: &mut dyn ReadSeek, end_pos: u64, metadata: &mut Metadata) -> Result<()> {
    use std::io::SeekFrom;
    
    while reader.stream_position()? < end_pos {
        let chunk_start = reader.stream_position()?;

        let mut chunk_id = [0u8; 4];
        if reader.read_exact(&mut chunk_id).is_err() {
            break;
        }

        let mut size_buf = [0u8; 4];
        if reader.read_exact(&mut size_buf).is_err() {
            break;
        }
        let chunk_size = u32::from_le_bytes(size_buf) as u64;

        if chunk_size > 0 && chunk_size < 64 * 1024 {
            let mut data = vec![0u8; chunk_size as usize];
            reader.read_exact(&mut data)?;
            let value = String::from_utf8_lossy(&data)
                .trim_end_matches('\0')
                .to_string();

            if !value.is_empty() {
                let tag_name = info_tag_name(&chunk_id);
                metadata.exif.set(format!("RIFF:{}", tag_name), AttrValue::Str(value));
            }
        } else {
            reader.seek(SeekFrom::Current(chunk_size as i64))?;
        }

        // Word alignment
        let current = reader.stream_position()?;
        let expected = chunk_start + 8 + chunk_size;
        if current < expected {
            reader.seek(SeekFrom::Start(expected))?;
        }
        if reader.stream_position()? % 2 != 0 {
            reader.seek(SeekFrom::Current(1))?;
        }
    }

    Ok(())
}

/// Read RIFF chunk header (4-byte ID + 4-byte size).
///
/// Returns None if EOF reached.
pub fn read_chunk_header(reader: &mut dyn ReadSeek) -> Option<([u8; 4], u64)> {
    let mut chunk_id = [0u8; 4];
    if reader.read_exact(&mut chunk_id).is_err() {
        return None;
    }

    let mut size_buf = [0u8; 4];
    if reader.read_exact(&mut size_buf).is_err() {
        return None;
    }
    let chunk_size = u32::from_le_bytes(size_buf) as u64;

    Some((chunk_id, chunk_size))
}

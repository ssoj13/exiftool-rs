//! ASF (Advanced Systems Format) parser for WMA/WMV files.
//!
//! ASF is Microsoft's container format for Windows Media Audio/Video.
//!
//! # Structure
//!
//! - Header Object (GUID + size + sub-objects)
//! - Data Object (media packets)
//! - Index Objects (optional)

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

// ASF GUIDs
const ASF_HEADER_GUID: [u8; 16] = [
    0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11,
    0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C,
];

const FILE_PROPERTIES_GUID: [u8; 16] = [
    0xA1, 0xDC, 0xAB, 0x8C, 0x47, 0xA9, 0xCF, 0x11,
    0x8E, 0xE4, 0x00, 0xC0, 0x0C, 0x20, 0x53, 0x65,
];

const STREAM_PROPERTIES_GUID: [u8; 16] = [
    0x91, 0x07, 0xDC, 0xB7, 0xB7, 0xA9, 0xCF, 0x11,
    0x8E, 0xE6, 0x00, 0xC0, 0x0C, 0x20, 0x53, 0x65,
];

const CONTENT_DESC_GUID: [u8; 16] = [
    0x33, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11,
    0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C,
];

const EXT_CONTENT_DESC_GUID: [u8; 16] = [
    0x40, 0xA4, 0xD0, 0xD2, 0x07, 0xE3, 0xD2, 0x11,
    0x97, 0xF0, 0x00, 0xA0, 0xC9, 0x5E, 0xA8, 0x50,
];

const AUDIO_MEDIA_GUID: [u8; 16] = [
    0x40, 0x9E, 0x69, 0xF8, 0x4D, 0x5B, 0xCF, 0x11,
    0xA8, 0xFD, 0x00, 0x80, 0x5F, 0x5C, 0x44, 0x2B,
];

/// ASF/WMA/WMV format parser.
pub struct AsfParser;

impl FormatParser for AsfParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 16 && header[0..16] == ASF_HEADER_GUID
    }

    fn format_name(&self) -> &'static str {
        "ASF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["wma", "wmv", "asf"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("ASF");
        meta.exif.set("File:FileType", AttrValue::Str("ASF".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // File size
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        reader.seek(SeekFrom::Start(0))?;

        // Read ASF header object
        let mut header = [0u8; 30];
        reader.read_exact(&mut header)?;

        // Verify GUID
        if header[0..16] != ASF_HEADER_GUID {
            return Ok(meta);
        }

        // Header size (8 bytes LE)
        let header_size = u64::from_le_bytes([
            header[16], header[17], header[18], header[19],
            header[20], header[21], header[22], header[23],
        ]);

        // Number of header objects (4 bytes LE)
        let num_objects = u32::from_le_bytes([header[24], header[25], header[26], header[27]]);

        // Parse header objects
        let mut pos = 30u64;
        let header_end = header_size.min(1_000_000); // Safety limit

        for _ in 0..num_objects.min(50) {
            if pos + 24 > header_end {
                break;
            }

            reader.seek(SeekFrom::Start(pos))?;

            let mut obj_header = [0u8; 24];
            if reader.read_exact(&mut obj_header).is_err() {
                break;
            }

            let guid = &obj_header[0..16];
            let obj_size = u64::from_le_bytes([
                obj_header[16], obj_header[17], obj_header[18], obj_header[19],
                obj_header[20], obj_header[21], obj_header[22], obj_header[23],
            ]);

            if obj_size < 24 || obj_size > header_end - pos {
                break;
            }

            // Parse known objects
            if guid == FILE_PROPERTIES_GUID {
                parse_file_properties(reader, &mut meta)?;
            } else if guid == STREAM_PROPERTIES_GUID {
                parse_stream_properties(reader, &mut meta)?;
            } else if guid == CONTENT_DESC_GUID {
                parse_content_description(reader, &mut meta)?;
            } else if guid == EXT_CONTENT_DESC_GUID {
                parse_ext_content_description(reader, obj_size - 24, &mut meta)?;
            }

            pos += obj_size;
        }

        // Determine if audio or video
        if meta.exif.get_str("ASF:VideoCodec").is_some() {
            meta.format = "WMV";
            meta.exif.set("File:FileType", AttrValue::Str("WMV".to_string()));
            meta.exif.set("File:MIMEType", AttrValue::Str("video/x-ms-wmv".to_string()));
        } else {
            meta.format = "WMA";
            meta.exif.set("File:FileType", AttrValue::Str("WMA".to_string()));
            meta.exif.set("File:MIMEType", AttrValue::Str("audio/x-ms-wma".to_string()));
        }

        Ok(meta)
    }
}

/// Parse File Properties Object.
fn parse_file_properties(reader: &mut dyn ReadSeek, meta: &mut Metadata) -> Result<()> {
    let mut data = [0u8; 80];
    reader.read_exact(&mut data)?;

    // File ID GUID (16 bytes) - skip
    // File size (8 bytes)
    let _file_size = u64::from_le_bytes([
        data[16], data[17], data[18], data[19],
        data[20], data[21], data[22], data[23],
    ]);

    // Creation date (8 bytes, FILETIME)
    let creation_time = u64::from_le_bytes([
        data[24], data[25], data[26], data[27],
        data[28], data[29], data[30], data[31],
    ]);
    if creation_time > 0 {
        // Convert FILETIME to Unix timestamp
        // FILETIME is 100ns intervals since 1601-01-01
        let unix_time = (creation_time / 10_000_000).saturating_sub(11644473600);
        meta.exif.set("ASF:CreationDate", AttrValue::UInt64(unix_time));
    }

    // Data packets count (8 bytes)
    // Play duration (8 bytes, 100ns units)
    let play_duration = u64::from_le_bytes([
        data[40], data[41], data[42], data[43],
        data[44], data[45], data[46], data[47],
    ]);

    // Send duration (8 bytes)
    // Preroll (8 bytes, ms)
    let preroll = u64::from_le_bytes([
        data[56], data[57], data[58], data[59],
        data[60], data[61], data[62], data[63],
    ]);

    // Calculate actual duration
    if play_duration > 0 {
        let duration_100ns = play_duration.saturating_sub(preroll * 10_000);
        let duration_secs = duration_100ns as f64 / 10_000_000.0;
        if duration_secs > 0.0 && duration_secs < 86400.0 * 30.0 {
            meta.exif.set("Audio:Duration", AttrValue::Double(duration_secs));
        }
    }

    // Flags (4 bytes)
    // Min/Max packet size (4+4 bytes)
    // Max bitrate (4 bytes)
    let max_bitrate = u32::from_le_bytes([data[76], data[77], data[78], data[79]]);
    if max_bitrate > 0 {
        meta.exif.set("Audio:Bitrate", AttrValue::UInt(max_bitrate / 1000));
    }

    Ok(())
}

/// Parse Stream Properties Object.
fn parse_stream_properties(reader: &mut dyn ReadSeek, meta: &mut Metadata) -> Result<()> {
    let mut data = [0u8; 54];
    reader.read_exact(&mut data)?;

    let stream_type = &data[0..16];

    if stream_type == AUDIO_MEDIA_GUID {
        // Audio stream - read WAVEFORMATEX
        let type_specific_len = u32::from_le_bytes([data[40], data[41], data[42], data[43]]) as usize;
        
        if type_specific_len >= 18 {
            let mut wave = vec![0u8; type_specific_len.min(256)];
            // Skip to type-specific data
            reader.seek(SeekFrom::Current(4))?; // error correction len
            reader.read_exact(&mut wave)?;

            // WAVEFORMATEX structure
            let format_tag = u16::from_le_bytes([wave[0], wave[1]]);
            let channels = u16::from_le_bytes([wave[2], wave[3]]);
            let sample_rate = u32::from_le_bytes([wave[4], wave[5], wave[6], wave[7]]);
            let avg_bytes_per_sec = u32::from_le_bytes([wave[8], wave[9], wave[10], wave[11]]);
            let bits_per_sample = u16::from_le_bytes([wave[14], wave[15]]);

            meta.exif.set("Audio:SampleRate", AttrValue::UInt(sample_rate));
            meta.exif.set("Audio:Channels", AttrValue::UInt(channels as u32));
            
            if bits_per_sample > 0 {
                meta.exif.set("Audio:BitsPerSample", AttrValue::UInt(bits_per_sample as u32));
            }

            if avg_bytes_per_sec > 0 {
                meta.exif.set("Audio:AvgBytesPerSec", AttrValue::UInt(avg_bytes_per_sec));
            }

            let codec = match format_tag {
                0x0161 => "WMA v2",
                0x0162 => "WMA Pro",
                0x0163 => "WMA Lossless",
                0x000A => "WMA Voice",
                _ => "WMA",
            };
            meta.exif.set("Audio:Codec", AttrValue::Str(codec.to_string()));

            let channel_mode = match channels {
                1 => "Mono",
                2 => "Stereo",
                6 => "5.1",
                8 => "7.1",
                _ => "Multi-channel",
            };
            meta.exif.set("Audio:ChannelMode", AttrValue::Str(channel_mode.to_string()));
        }
    } else {
        // Video stream
        meta.exif.set("ASF:VideoCodec", AttrValue::Str("WMV".to_string()));
    }

    Ok(())
}

/// Parse Content Description Object.
fn parse_content_description(reader: &mut dyn ReadSeek, meta: &mut Metadata) -> Result<()> {
    let mut lengths = [0u8; 10];
    reader.read_exact(&mut lengths)?;

    let title_len = u16::from_le_bytes([lengths[0], lengths[1]]) as usize;
    let author_len = u16::from_le_bytes([lengths[2], lengths[3]]) as usize;
    let copyright_len = u16::from_le_bytes([lengths[4], lengths[5]]) as usize;
    let desc_len = u16::from_le_bytes([lengths[6], lengths[7]]) as usize;
    let _rating_len = u16::from_le_bytes([lengths[8], lengths[9]]) as usize;

    // Read strings (UTF-16LE)
    if title_len > 0 && title_len < 2000 {
        let title = read_utf16le_string(reader, title_len)?;
        if !title.is_empty() {
            meta.exif.set("Audio:Title", AttrValue::Str(title));
        }
    }

    if author_len > 0 && author_len < 2000 {
        let author = read_utf16le_string(reader, author_len)?;
        if !author.is_empty() {
            meta.exif.set("Audio:Artist", AttrValue::Str(author));
        }
    }

    if copyright_len > 0 && copyright_len < 2000 {
        let copyright = read_utf16le_string(reader, copyright_len)?;
        if !copyright.is_empty() {
            meta.exif.set("Audio:Copyright", AttrValue::Str(copyright));
        }
    }

    if desc_len > 0 && desc_len < 10000 {
        let desc = read_utf16le_string(reader, desc_len)?;
        if !desc.is_empty() {
            meta.exif.set("Audio:Description", AttrValue::Str(desc));
        }
    }

    Ok(())
}

/// Parse Extended Content Description Object.
fn parse_ext_content_description(reader: &mut dyn ReadSeek, size: u64, meta: &mut Metadata) -> Result<()> {
    let mut count_bytes = [0u8; 2];
    reader.read_exact(&mut count_bytes)?;
    let count = u16::from_le_bytes(count_bytes) as usize;

    let mut bytes_read = 2u64;

    for _ in 0..count.min(50) {
        if bytes_read + 4 > size {
            break;
        }

        // Name length (2 bytes)
        let mut name_len_bytes = [0u8; 2];
        reader.read_exact(&mut name_len_bytes)?;
        let name_len = u16::from_le_bytes(name_len_bytes) as usize;
        bytes_read += 2;

        if name_len == 0 || name_len > 1000 || bytes_read + name_len as u64 > size {
            break;
        }

        // Name (UTF-16LE)
        let name = read_utf16le_string(reader, name_len)?;
        bytes_read += name_len as u64;

        // Value type (2 bytes)
        let mut type_bytes = [0u8; 2];
        reader.read_exact(&mut type_bytes)?;
        let value_type = u16::from_le_bytes(type_bytes);
        bytes_read += 2;

        // Value length (2 bytes)
        let mut value_len_bytes = [0u8; 2];
        reader.read_exact(&mut value_len_bytes)?;
        let value_len = u16::from_le_bytes(value_len_bytes) as usize;
        bytes_read += 2;

        if value_len > 10000 || bytes_read + value_len as u64 > size {
            reader.seek(SeekFrom::Current(value_len as i64))?;
            bytes_read += value_len as u64;
            continue;
        }

        // Map known attributes
        let tag_name = match name.trim_end_matches('\0').to_lowercase().as_str() {
            "wm/albumtitle" => Some("Audio:Album"),
            "wm/year" => Some("Audio:Year"),
            "wm/genre" => Some("Audio:Genre"),
            "wm/tracknumber" => Some("Audio:Track"),
            "wm/albumartist" => Some("Audio:AlbumArtist"),
            "wm/composer" => Some("Audio:Composer"),
            "wm/publisher" => Some("Audio:Publisher"),
            "wm/encodedby" => Some("Audio:EncodedBy"),
            _ => None,
        };

        if let Some(tag) = tag_name {
            match value_type {
                0 => {
                    // UTF-16LE string
                    let value = read_utf16le_string(reader, value_len)?;
                    if !value.is_empty() {
                        meta.exif.set(tag, AttrValue::Str(value.trim_end_matches('\0').to_string()));
                    }
                }
                3 => {
                    // DWORD
                    let mut val = [0u8; 4];
                    reader.read_exact(&mut val)?;
                    let num = u32::from_le_bytes(val);
                    meta.exif.set(tag, AttrValue::UInt(num));
                }
                _ => {
                    reader.seek(SeekFrom::Current(value_len as i64))?;
                }
            }
        } else {
            reader.seek(SeekFrom::Current(value_len as i64))?;
        }
        bytes_read += value_len as u64;
    }

    Ok(())
}

/// Read UTF-16LE string.
fn read_utf16le_string(reader: &mut dyn ReadSeek, len: usize) -> Result<String> {
    let mut data = vec![0u8; len];
    reader.read_exact(&mut data)?;

    let u16_chars: Vec<u16> = data
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .take_while(|&c| c != 0)
        .collect();

    Ok(String::from_utf16_lossy(&u16_chars))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_asf_header() -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // ASF Header GUID
        data[0..16].copy_from_slice(&ASF_HEADER_GUID);
        // Header size (100 bytes)
        data[16..24].copy_from_slice(&100u64.to_le_bytes());
        // Number of objects (0)
        data[24..28].copy_from_slice(&0u32.to_le_bytes());
        // Reserved
        data[28] = 0x01;
        data[29] = 0x02;
        data
    }

    #[test]
    fn test_can_parse() {
        let parser = AsfParser;
        let data = make_asf_header();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = AsfParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"RIFF"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = AsfParser;
        let data = make_asf_header();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        // Without stream properties, defaults to WMA
        assert_eq!(meta.format, "WMA");
    }

    #[test]
    fn test_format_info() {
        let parser = AsfParser;
        assert_eq!(parser.format_name(), "ASF");
        assert!(parser.extensions().contains(&"wma"));
        assert!(parser.extensions().contains(&"wmv"));
    }
}

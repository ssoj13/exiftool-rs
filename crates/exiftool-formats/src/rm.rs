//! Real Media (.rm, .rmvb) format parser.
//!
//! Real Media is a container format from RealNetworks.
//! Structure:
//! - .RMF header (magic 0x2E524D46)
//! - PROP (file properties)
//! - CONT (content description: title, author, copyright)
//! - MDPR (media properties per stream)
//! - DATA (actual media data)
//! - INDX (index)

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// Real Media parser.
pub struct RmParser;

impl FormatParser for RmParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // .RMF magic: 0x2E 0x52 0x4D 0x46 (".RMF")
        header.len() >= 4 && &header[0..4] == b".RMF"
    }

    fn format_name(&self) -> &'static str {
        "RM"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["rm", "rmvb", "ra", "ram", "rv"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("RM");

        // Read entire file into buffer
        reader.seek(SeekFrom::Start(0))?;
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        if data.len() < 18 {
            return Ok(meta);
        }

        // Verify .RMF magic
        if &data[0..4] != b".RMF" {
            return Ok(meta);
        }

        // Parse .RMF header
        // offset 4-7: size (big-endian)
        // offset 8-9: version
        // offset 10-13: file version
        // offset 14-17: num headers
        let rmf_size = u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize;
        let file_version = u32::from_be_bytes([data[10], data[11], data[12], data[13]]);
        let num_headers = u32::from_be_bytes([data[14], data[15], data[16], data[17]]);

        meta.exif.set("FileVersion", AttrValue::UInt(file_version));
        meta.exif.set("NumHeaders", AttrValue::UInt(num_headers));

        // Parse subsequent chunks
        let mut pos = rmf_size.min(data.len());

        while pos + 10 <= data.len() {
            // Chunk header: 4-byte type + 4-byte size + 2-byte version
            let chunk_type = &data[pos..pos + 4];
            let chunk_size =
                u32::from_be_bytes([data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]])
                    as usize;

            if chunk_size < 10 || pos + chunk_size > data.len() {
                break;
            }

            match chunk_type {
                b"PROP" => parse_prop(&data[pos..pos + chunk_size], &mut meta),
                b"CONT" => parse_cont(&data[pos..pos + chunk_size], &mut meta),
                b"MDPR" => parse_mdpr(&data[pos..pos + chunk_size], &mut meta),
                _ => {}
            }

            pos += chunk_size;
        }

        Ok(meta)
    }
}

/// Parse PROP (Properties) chunk.
fn parse_prop(data: &[u8], meta: &mut Metadata) {
    // PROP structure (after common header):
    // offset 10-13: max bit rate
    // offset 14-17: avg bit rate
    // offset 18-21: max packet size
    // offset 22-25: avg packet size
    // offset 26-29: num packets
    // offset 30-33: duration (ms)
    // offset 34-37: preroll (ms)
    // offset 38-41: index offset
    // offset 42-45: data offset
    // offset 46-47: num streams
    // offset 48-49: flags

    if data.len() < 50 {
        return;
    }

    let max_bitrate = u32::from_be_bytes([data[10], data[11], data[12], data[13]]);
    let avg_bitrate = u32::from_be_bytes([data[14], data[15], data[16], data[17]]);
    let duration_ms = u32::from_be_bytes([data[30], data[31], data[32], data[33]]);
    let num_streams = u16::from_be_bytes([data[46], data[47]]);
    let flags = u16::from_be_bytes([data[48], data[49]]);

    meta.exif.set("MaxBitrate", AttrValue::UInt(max_bitrate));
    meta.exif.set("AvgBitrate", AttrValue::UInt(avg_bitrate));
    meta.exif.set("Duration", AttrValue::Double(duration_ms as f64 / 1000.0));
    meta.exif.set("NumStreams", AttrValue::UInt(num_streams as u32));

    // Flags
    if flags & 0x0001 != 0 {
        meta.exif.set("SaveEnabled", AttrValue::Str("Yes".into()));
    }
    if flags & 0x0002 != 0 {
        meta.exif.set("PerfectPlay", AttrValue::Str("Yes".into()));
    }
    if flags & 0x0004 != 0 {
        meta.exif.set("LiveBroadcast", AttrValue::Str("Yes".into()));
    }
}

/// Parse CONT (Content Description) chunk.
fn parse_cont(data: &[u8], meta: &mut Metadata) {
    // CONT structure (after common header):
    // offset 10-11: title length
    // then: title string
    // then: author length (2 bytes)
    // then: author string
    // then: copyright length (2 bytes)
    // then: copyright string
    // then: comment length (2 bytes)
    // then: comment string

    if data.len() < 12 {
        return;
    }

    let mut pos = 10;

    // Title
    if pos + 2 > data.len() {
        return;
    }
    let title_len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
    pos += 2;

    if pos + title_len <= data.len() && title_len > 0 {
        if let Ok(title) = String::from_utf8(data[pos..pos + title_len].to_vec()) {
            if !title.trim().is_empty() {
                meta.exif.set("Title", AttrValue::Str(title.trim().to_string()));
            }
        }
    }
    pos += title_len;

    // Author
    if pos + 2 > data.len() {
        return;
    }
    let author_len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
    pos += 2;

    if pos + author_len <= data.len() && author_len > 0 {
        if let Ok(author) = String::from_utf8(data[pos..pos + author_len].to_vec()) {
            if !author.trim().is_empty() {
                meta.exif.set("Author", AttrValue::Str(author.trim().to_string()));
            }
        }
    }
    pos += author_len;

    // Copyright
    if pos + 2 > data.len() {
        return;
    }
    let copyright_len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
    pos += 2;

    if pos + copyright_len <= data.len() && copyright_len > 0 {
        if let Ok(copyright) = String::from_utf8(data[pos..pos + copyright_len].to_vec()) {
            if !copyright.trim().is_empty() {
                meta.exif.set("Copyright", AttrValue::Str(copyright.trim().to_string()));
            }
        }
    }
    pos += copyright_len;

    // Comment
    if pos + 2 > data.len() {
        return;
    }
    let comment_len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
    pos += 2;

    if pos + comment_len <= data.len() && comment_len > 0 {
        if let Ok(comment) = String::from_utf8(data[pos..pos + comment_len].to_vec()) {
            if !comment.trim().is_empty() {
                meta.exif.set("Comment", AttrValue::Str(comment.trim().to_string()));
            }
        }
    }
}

/// Parse MDPR (Media Properties) chunk.
fn parse_mdpr(data: &[u8], meta: &mut Metadata) {
    // MDPR structure (after common header):
    // offset 10-11: stream number
    // offset 12-15: max bit rate
    // offset 16-19: avg bit rate
    // offset 20-23: max packet size
    // offset 24-27: avg packet size
    // offset 28-31: start time
    // offset 32-35: preroll
    // offset 36-39: duration (ms)
    // offset 40: stream name length
    // then: stream name string
    // then: mime type length (1 byte)
    // then: mime type string
    // then: type specific data length (4 bytes)
    // then: type specific data

    if data.len() < 41 {
        return;
    }

    let stream_num = u16::from_be_bytes([data[10], data[11]]);
    let stream_bitrate = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    let stream_duration_ms = u32::from_be_bytes([data[36], data[37], data[38], data[39]]);

    let stream_key = format!("Stream{}", stream_num);

    // Stream name
    let mut pos = 40;
    if pos >= data.len() {
        return;
    }
    let name_len = data[pos] as usize;
    pos += 1;

    let stream_name = if pos + name_len <= data.len() && name_len > 0 {
        String::from_utf8(data[pos..pos + name_len].to_vec())
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    } else {
        None
    };
    pos += name_len;

    // MIME type
    if pos >= data.len() {
        return;
    }
    let mime_len = data[pos] as usize;
    pos += 1;

    let mime_type = if pos + mime_len <= data.len() && mime_len > 0 {
        String::from_utf8(data[pos..pos + mime_len].to_vec())
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    } else {
        None
    };

    // Build stream info
    let mut stream_info = format!("#{}", stream_num);
    if let Some(name) = stream_name {
        stream_info = format!("{} \"{}\"", stream_info, name);
    }
    if let Some(mime) = &mime_type {
        stream_info = format!("{} {}", stream_info, mime);
    }
    stream_info = format!("{} {}kbps", stream_info, stream_bitrate / 1000);

    meta.exif.set(&stream_key, AttrValue::Str(stream_info));

    // Detect codec from MIME type
    if let Some(mime) = mime_type {
        let codec = match mime.as_str() {
            "audio/x-pn-realaudio" | "audio/x-pn-multirate-realaudio" => "RealAudio",
            "video/x-pn-realvideo" | "video/x-pn-multirate-realvideo" => "RealVideo",
            "audio/x-pn-multirate-realaudio-live" => "RealAudio Live",
            _ => "",
        };

        if !codec.is_empty() {
            let key = if mime.starts_with("audio") {
                "AudioCodec"
            } else {
                "VideoCodec"
            };
            // Only set if not already set
            if meta.exif.get(key).is_none() {
                meta.exif.set(key, AttrValue::Str(codec.into()));
            }

            // Store stream duration if video
            if mime.starts_with("video") && stream_duration_ms > 0
                && meta.exif.get("VideoDuration").is_none() {
                    meta.exif.set(
                        "VideoDuration",
                        AttrValue::Double(stream_duration_ms as f64 / 1000.0),
                    );
                }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_can_parse() {
        let parser = RmParser;
        assert!(parser.can_parse(b".RMF\x00\x00\x00\x12"));
        assert!(parser.can_parse(b".RMF\x00\x00\x00\x12extra"));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = RmParser;
        assert!(!parser.can_parse(b"RIFF"));
        assert!(!parser.can_parse(b"ftyp"));
        assert!(!parser.can_parse(&[0x1A, 0x45, 0xDF, 0xA3])); // MKV
    }

    #[test]
    fn test_format_info() {
        let parser = RmParser;
        assert_eq!(parser.format_name(), "RM");
        assert!(parser.extensions().contains(&"rm"));
        assert!(parser.extensions().contains(&"rmvb"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = RmParser;

        // Build minimal Real Media file
        let mut data = Vec::new();

        // .RMF header (18 bytes)
        data.extend_from_slice(b".RMF"); // magic
        data.extend_from_slice(&18u32.to_be_bytes()); // size
        data.extend_from_slice(&0u16.to_be_bytes()); // version
        data.extend_from_slice(&0u32.to_be_bytes()); // file version
        data.extend_from_slice(&2u32.to_be_bytes()); // num headers (PROP + CONT)

        // PROP chunk (50 bytes)
        data.extend_from_slice(b"PROP"); // type
        data.extend_from_slice(&50u32.to_be_bytes()); // size
        data.extend_from_slice(&0u16.to_be_bytes()); // version
        data.extend_from_slice(&128000u32.to_be_bytes()); // max bitrate
        data.extend_from_slice(&96000u32.to_be_bytes()); // avg bitrate
        data.extend_from_slice(&1000u32.to_be_bytes()); // max packet size
        data.extend_from_slice(&800u32.to_be_bytes()); // avg packet size
        data.extend_from_slice(&1000u32.to_be_bytes()); // num packets
        data.extend_from_slice(&60000u32.to_be_bytes()); // duration (60 sec)
        data.extend_from_slice(&0u32.to_be_bytes()); // preroll
        data.extend_from_slice(&0u32.to_be_bytes()); // index offset
        data.extend_from_slice(&0u32.to_be_bytes()); // data offset
        data.extend_from_slice(&1u16.to_be_bytes()); // num streams
        data.extend_from_slice(&0u16.to_be_bytes()); // flags

        let mut cursor = Cursor::new(&data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "RM");
        assert_eq!(meta.exif.get_u32("MaxBitrate"), Some(128000));
        assert_eq!(meta.exif.get_u32("AvgBitrate"), Some(96000));
        assert_eq!(meta.exif.get_f64("Duration"), Some(60.0));
        assert_eq!(meta.exif.get_u32("NumStreams"), Some(1));
    }

    #[test]
    fn test_parse_with_content() {
        let parser = RmParser;

        // Build Real Media file with CONT chunk
        let mut data = Vec::new();

        // .RMF header (18 bytes)
        data.extend_from_slice(b".RMF");
        data.extend_from_slice(&18u32.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes());
        data.extend_from_slice(&0u32.to_be_bytes());
        data.extend_from_slice(&1u32.to_be_bytes());

        // CONT chunk
        let title = b"Test Title";
        let author = b"Test Author";
        let copyright = b"2024";
        let comment = b"Test Comment";
        let cont_size = 10 + 2 + title.len() + 2 + author.len() + 2 + copyright.len() + 2 + comment.len();

        data.extend_from_slice(b"CONT");
        data.extend_from_slice(&(cont_size as u32).to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes()); // version
        data.extend_from_slice(&(title.len() as u16).to_be_bytes());
        data.extend_from_slice(title);
        data.extend_from_slice(&(author.len() as u16).to_be_bytes());
        data.extend_from_slice(author);
        data.extend_from_slice(&(copyright.len() as u16).to_be_bytes());
        data.extend_from_slice(copyright);
        data.extend_from_slice(&(comment.len() as u16).to_be_bytes());
        data.extend_from_slice(comment);

        let mut cursor = Cursor::new(&data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("Title"), Some("Test Title"));
        assert_eq!(meta.exif.get_str("Author"), Some("Test Author"));
        assert_eq!(meta.exif.get_str("Copyright"), Some("2024"));
        assert_eq!(meta.exif.get_str("Comment"), Some("Test Comment"));
    }
}

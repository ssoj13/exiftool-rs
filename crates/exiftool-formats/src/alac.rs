//! ALAC (Apple Lossless Audio Codec) detection.
//!
//! ALAC is typically stored in M4A/CAF containers.
//! M4A files are handled by Mp4Parser which detects ALAC codec.
//! This module provides a standalone CAF (Core Audio Format) parser
//! which can also contain ALAC audio.
//!
//! # CAF Structure
//!
//! - 4 bytes: "caff" magic
//! - 2 bytes: version (1)
//! - 2 bytes: flags (0)
//! - Chunks: desc, data, pakt, info, etc.

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// CAF (Core Audio Format) parser - can contain ALAC.
pub struct CafParser;

impl FormatParser for CafParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 8 {
            return false;
        }
        // "caff" magic + version 1
        &header[0..4] == b"caff" && header[4] == 0 && header[5] == 1
    }

    fn format_name(&self) -> &'static str {
        "CAF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["caf"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("CAF");
        meta.exif.set("File:FileType", AttrValue::Str("CAF".to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("audio/x-caf".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // Read CAF header (8 bytes)
        let mut header = [0u8; 8];
        reader.read_exact(&mut header)?;

        // Version
        let version = u16::from_be_bytes([header[4], header[5]]);
        meta.exif.set("CAF:Version", AttrValue::UInt(version as u32));

        // Parse chunks
        let file_size = reader.seek(SeekFrom::End(0))?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));
        
        let mut pos = 8u64;
        
        while pos + 12 <= file_size {
            reader.seek(SeekFrom::Start(pos))?;
            
            let mut chunk_header = [0u8; 12];
            if reader.read_exact(&mut chunk_header).is_err() {
                break;
            }

            let chunk_type = &chunk_header[0..4];
            let chunk_size = i64::from_be_bytes([
                chunk_header[4], chunk_header[5], chunk_header[6], chunk_header[7],
                chunk_header[8], chunk_header[9], chunk_header[10], chunk_header[11],
            ]);

            match chunk_type {
                b"desc" => {
                    // Audio description chunk
                    parse_caf_desc(reader, &mut meta)?;
                }
                b"info" => {
                    // Information chunk (metadata)
                    if chunk_size > 0 && chunk_size < 1_000_000 {
                        parse_caf_info(reader, chunk_size as usize, &mut meta)?;
                    }
                }
                b"data" => {
                    // Audio data chunk
                    if chunk_size > 0 {
                        meta.exif.set("CAF:AudioDataSize", AttrValue::UInt64(chunk_size as u64));
                    }
                }
                _ => {}
            }

            // Move to next chunk (-1 means unknown size, extends to EOF)
            if chunk_size < 0 {
                break;
            }
            pos += 12 + chunk_size as u64;
        }

        Ok(meta)
    }
}

/// Parse CAF audio description chunk.
fn parse_caf_desc(reader: &mut dyn ReadSeek, meta: &mut Metadata) -> Result<()> {
    let mut desc = [0u8; 32];
    reader.read_exact(&mut desc)?;

    // Sample rate (8 bytes, float64 BE)
    let sample_rate_bits = u64::from_be_bytes([
        desc[0], desc[1], desc[2], desc[3], desc[4], desc[5], desc[6], desc[7],
    ]);
    let sample_rate = f64::from_bits(sample_rate_bits);
    meta.exif.set("Audio:SampleRate", AttrValue::UInt(sample_rate as u32));

    // Format ID (4 bytes)
    let format_id = &desc[8..12];
    let codec = match format_id {
        b"alac" => "Apple Lossless (ALAC)",
        b"aac " => "AAC",
        b"lpcm" => "Linear PCM",
        b"ima4" => "IMA 4:1 ADPCM",
        b"alaw" => "A-law",
        b"ulaw" => "mu-law",
        b".mp3" => "MP3",
        b"mp4a" => "MPEG-4 Audio",
        _ => {
            let id_str = String::from_utf8_lossy(format_id).to_string();
            meta.exif.set("CAF:FormatID", AttrValue::Str(id_str));
            "Unknown"
        }
    };
    meta.exif.set("Audio:Codec", AttrValue::Str(codec.to_string()));

    // Format flags (4 bytes)
    let format_flags = u32::from_be_bytes([desc[12], desc[13], desc[14], desc[15]]);
    if format_flags != 0 {
        meta.exif.set("CAF:FormatFlags", AttrValue::UInt(format_flags));
    }

    // Bytes per packet (4 bytes)
    let bytes_per_packet = u32::from_be_bytes([desc[16], desc[17], desc[18], desc[19]]);
    if bytes_per_packet > 0 {
        meta.exif.set("CAF:BytesPerPacket", AttrValue::UInt(bytes_per_packet));
    }

    // Frames per packet (4 bytes)
    let frames_per_packet = u32::from_be_bytes([desc[20], desc[21], desc[22], desc[23]]);
    if frames_per_packet > 0 {
        meta.exif.set("CAF:FramesPerPacket", AttrValue::UInt(frames_per_packet));
    }

    // Channels per frame (4 bytes)
    let channels = u32::from_be_bytes([desc[24], desc[25], desc[26], desc[27]]);
    meta.exif.set("Audio:Channels", AttrValue::UInt(channels));
    
    let channel_mode = match channels {
        1 => "Mono",
        2 => "Stereo",
        6 => "5.1 Surround",
        8 => "7.1 Surround",
        _ => "Multi-channel",
    };
    meta.exif.set("Audio:ChannelMode", AttrValue::Str(channel_mode.to_string()));

    // Bits per channel (4 bytes)
    let bits_per_sample = u32::from_be_bytes([desc[28], desc[29], desc[30], desc[31]]);
    if bits_per_sample > 0 {
        meta.exif.set("Audio:BitsPerSample", AttrValue::UInt(bits_per_sample));
    }

    Ok(())
}

/// Parse CAF information chunk.
fn parse_caf_info(reader: &mut dyn ReadSeek, size: usize, meta: &mut Metadata) -> Result<()> {
    if size < 4 {
        return Ok(());
    }

    let mut data = vec![0u8; size];
    reader.read_exact(&mut data)?;

    // Number of entries (4 bytes BE)
    let num_entries = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
    
    let mut pos = 4;
    
    for _ in 0..num_entries.min(50) {
        // Key (null-terminated)
        let key_end = data[pos..].iter().position(|&b| b == 0);
        if key_end.is_none() {
            break;
        }
        let key_end = key_end.unwrap();
        let key = String::from_utf8_lossy(&data[pos..pos + key_end]).to_string();
        pos += key_end + 1;

        if pos >= size {
            break;
        }

        // Value (null-terminated)
        let value_end = data[pos..].iter().position(|&b| b == 0);
        if value_end.is_none() {
            break;
        }
        let value_end = value_end.unwrap();
        let value = String::from_utf8_lossy(&data[pos..pos + value_end]).to_string();
        pos += value_end + 1;

        // Map known keys
        let tag_name = match key.as_str() {
            "title" => "Audio:Title",
            "artist" => "Audio:Artist",
            "album" => "Audio:Album",
            "track number" => "Audio:Track",
            "year" => "Audio:Year",
            "genre" => "Audio:Genre",
            "comments" => "Audio:Comment",
            "encoder" => "Audio:Encoder",
            "source encoder" => "Audio:SourceEncoder",
            "approximate duration in seconds" => {
                if let Ok(dur) = value.parse::<f64>() {
                    meta.exif.set("Audio:Duration", AttrValue::Double(dur));
                }
                continue;
            }
            _ => continue,
        };

        meta.exif.set(tag_name, AttrValue::Str(value));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_caf_header() -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // CAF header
        data[0..4].copy_from_slice(b"caff");
        data[4..6].copy_from_slice(&1u16.to_be_bytes()); // version
        data[6..8].copy_from_slice(&0u16.to_be_bytes()); // flags
        
        // desc chunk
        data[8..12].copy_from_slice(b"desc");
        data[12..20].copy_from_slice(&32i64.to_be_bytes()); // chunk size
        
        // Audio description
        data[20..28].copy_from_slice(&44100.0f64.to_be_bytes()); // sample rate
        data[28..32].copy_from_slice(b"alac"); // format ID
        data[32..36].copy_from_slice(&0u32.to_be_bytes()); // format flags
        data[36..40].copy_from_slice(&0u32.to_be_bytes()); // bytes per packet
        data[40..44].copy_from_slice(&4096u32.to_be_bytes()); // frames per packet
        data[44..48].copy_from_slice(&2u32.to_be_bytes()); // channels
        data[48..52].copy_from_slice(&16u32.to_be_bytes()); // bits per sample
        
        data
    }

    #[test]
    fn test_can_parse() {
        let parser = CafParser;
        let data = make_caf_header();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = CafParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"RIFF"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = CafParser;
        let data = make_caf_header();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "CAF");
        assert_eq!(meta.exif.get_u32("Audio:SampleRate"), Some(44100));
        assert_eq!(meta.exif.get_u32("Audio:Channels"), Some(2));
        assert_eq!(meta.exif.get_str("Audio:Codec"), Some("Apple Lossless (ALAC)"));
    }

    #[test]
    fn test_format_info() {
        let parser = CafParser;
        assert_eq!(parser.format_name(), "CAF");
        assert!(parser.extensions().contains(&"caf"));
    }
}

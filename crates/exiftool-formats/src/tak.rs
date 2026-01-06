//! TAK (Tom's lossless Audio Kompressor) format parser.
//!
//! TAK is a lossless audio codec with high compression ratio.
//!
//! # Structure
//!
//! - "tBaK" magic (4 bytes)
//! - Stream info metadata block
//! - Optional APEv2 tags at end

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// TAK format parser.
pub struct TakParser;

impl FormatParser for TakParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 4 {
            return false;
        }
        // "tBaK" magic
        &header[0..4] == b"tBaK"
    }

    fn format_name(&self) -> &'static str {
        "TAK"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["tak"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("TAK");
        meta.exif.set("File:FileType", AttrValue::Str("TAK".to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("audio/x-tak".to_string()));
        meta.exif.set("Audio:Codec", AttrValue::Str("TAK".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // Read header
        let mut header = [0u8; 4];
        reader.read_exact(&mut header)?;

        // Parse metadata blocks
        loop {
            let mut block_header = [0u8; 4];
            if reader.read_exact(&mut block_header).is_err() {
                break;
            }

            // Block type (5 bits) and size (24 bits) packed
            let type_and_size = u32::from_le_bytes(block_header);
            let block_type = (type_and_size & 0x1F) as u8;
            let block_size = (type_and_size >> 5) as usize;

            // End marker or invalid
            if block_type == 0 || block_size == 0 || block_size > 1_000_000 {
                break;
            }

            let mut block_data = vec![0u8; block_size];
            if reader.read_exact(&mut block_data).is_err() {
                break;
            }

            match block_type {
                1 => {
                    // Stream info
                    parse_tak_stream_info(&block_data, &mut meta);
                }
                2 => {
                    // Encoder info
                    if block_data.len() >= 4 {
                        let encoder_ver = u32::from_le_bytes([
                            block_data[0], block_data[1], block_data[2], block_data[3],
                        ]);
                        let major = encoder_ver >> 16;
                        let minor = (encoder_ver >> 8) & 0xFF;
                        let patch = encoder_ver & 0xFF;
                        meta.exif.set("TAK:EncoderVersion", 
                            AttrValue::Str(format!("{}.{}.{}", major, minor, patch)));
                    }
                }
                3 => {
                    // Metadata end - audio data follows
                    break;
                }
                _ => {}
            }
        }

        // File size
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        // Check for APEv2 tag at end
        if file_size > 32 {
            reader.seek(SeekFrom::Start(file_size - 32))?;
            let mut footer = [0u8; 32];
            if reader.read_exact(&mut footer).is_ok() && &footer[0..8] == b"APETAGEX" {
                meta.exif.set("TAK:HasAPEv2Tag", AttrValue::Str("Yes".to_string()));
            }
        }

        Ok(meta)
    }
}

/// Parse TAK stream info block.
fn parse_tak_stream_info(data: &[u8], meta: &mut Metadata) {
    if data.len() < 10 {
        return;
    }

    // Stream info is bit-packed, simplified parsing
    // First 2 bytes contain encoder profile and sample rate info
    let info = u16::from_le_bytes([data[0], data[1]]);
    
    // Sample rate index (bits 0-3)
    let sr_index = info & 0x0F;
    let sample_rate = match sr_index {
        0 => 6000,
        1 => 8000,
        2 => 11025,
        3 => 12000,
        4 => 16000,
        5 => 22050,
        6 => 24000,
        7 => 32000,
        8 => 44100,
        9 => 48000,
        10 => 88200,
        11 => 96000,
        12 => 176400,
        13 => 192000,
        _ => 44100,
    };
    meta.exif.set("Audio:SampleRate", AttrValue::UInt(sample_rate));

    // Bits per sample (bits 4-5): 0=8, 1=16, 2=24
    let bps_index = (info >> 4) & 0x03;
    let bits_per_sample = match bps_index {
        0 => 8,
        1 => 16,
        2 => 24,
        _ => 16,
    };
    meta.exif.set("Audio:BitsPerSample", AttrValue::UInt(bits_per_sample));

    // Channels (bits 6-9): value + 1
    let channels = ((info >> 6) & 0x0F) + 1;
    meta.exif.set("Audio:Channels", AttrValue::UInt(channels as u32));
    
    let channel_mode = match channels {
        1 => "Mono",
        2 => "Stereo",
        _ => "Multi-channel",
    };
    meta.exif.set("Audio:ChannelMode", AttrValue::Str(channel_mode.to_string()));

    // Total samples (if available in extended info)
    if data.len() >= 10 {
        let total_samples = u64::from_le_bytes([
            data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9],
        ]);
        if total_samples > 0 && sample_rate > 0 {
            let duration = total_samples as f64 / sample_rate as f64;
            meta.exif.set("Audio:Duration", AttrValue::Double(duration));
            
            let mins = (duration / 60.0) as u32;
            let secs = (duration % 60.0) as u32;
            meta.exif.set("Audio:DurationFormatted", 
                AttrValue::Str(format!("{}:{:02}", mins, secs)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_tak_header() -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // TAK magic
        data[0..4].copy_from_slice(b"tBaK");
        
        // Stream info block (type 1, size 10)
        let type_and_size: u32 = 1 | (10 << 5);
        data[4..8].copy_from_slice(&type_and_size.to_le_bytes());
        
        // Stream info: 44100 Hz (8), 16-bit (1), stereo (1 channel = 2-1)
        // sr_index=8, bps=1, channels=1 => 0x0048 + 0x0010 + 0x0040 = 0x0058... simplified
        let info: u16 = 8 | (1 << 4) | (1 << 6); // 44100, 16-bit, stereo
        data[8..10].copy_from_slice(&info.to_le_bytes());
        
        // Sample count (10 seconds at 44100)
        data[10..18].copy_from_slice(&441000u64.to_le_bytes());
        
        // End block (type 3)
        let end_block: u32 = 3 | (0 << 5);
        data[18..22].copy_from_slice(&end_block.to_le_bytes());
        
        data
    }

    #[test]
    fn test_can_parse() {
        let parser = TakParser;
        let data = make_tak_header();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = TakParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"fLaC"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = TakParser;
        let data = make_tak_header();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "TAK");
        assert_eq!(meta.exif.get_u32("Audio:SampleRate"), Some(44100));
        assert_eq!(meta.exif.get_u32("Audio:Channels"), Some(2));
        assert_eq!(meta.exif.get_str("Audio:ChannelMode"), Some("Stereo"));
    }

    #[test]
    fn test_format_info() {
        let parser = TakParser;
        assert_eq!(parser.format_name(), "TAK");
        assert!(parser.extensions().contains(&"tak"));
    }
}

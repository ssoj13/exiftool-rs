//! AAC (Advanced Audio Coding) ADTS format parser.
//!
//! ADTS (Audio Data Transport Stream) is a streaming format for AAC.
//! AAC in M4A/MP4 containers is handled by Mp4Parser.
//!
//! # ADTS Frame Structure
//!
//! - Sync word: 0xFFF (12 bits)
//! - Header (7 bytes fixed + optional 2 bytes CRC)
//! - Audio data

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// AAC ADTS format parser.
pub struct AacParser;

impl FormatParser for AacParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 4 {
            return false;
        }
        // ADTS sync word: 0xFFF (12 bits)
        // First byte = 0xFF, second byte starts with 0xF
        header[0] == 0xFF && (header[1] & 0xF0) == 0xF0
    }

    fn format_name(&self) -> &'static str {
        "AAC"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["aac", "adts"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("AAC");
        meta.exif.set("File:FileType", AttrValue::Str("AAC".to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("audio/aac".to_string()));
        meta.exif.set("Audio:Codec", AttrValue::Str("AAC".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // File size
        let file_size = reader.seek(SeekFrom::End(0))?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        reader.seek(SeekFrom::Start(0))?;

        // Parse first ADTS frame header
        let mut header = [0u8; 7];
        reader.read_exact(&mut header)?;

        // Verify sync word
        if header[0] != 0xFF || (header[1] & 0xF0) != 0xF0 {
            return Ok(meta);
        }

        // MPEG version (bit 3 of byte 1): 0 = MPEG-4, 1 = MPEG-2
        let mpeg_version = if header[1] & 0x08 != 0 { 2 } else { 4 };
        meta.exif.set("AAC:MPEGVersion", AttrValue::UInt(mpeg_version));

        // Layer (bits 1-2 of byte 1): always 0 for AAC
        let layer = (header[1] >> 1) & 0x03;
        if layer != 0 {
            meta.exif.set("AAC:Layer", AttrValue::UInt(layer as u32));
        }

        // Protection absent (bit 0 of byte 1): 1 = no CRC, 0 = CRC present
        let protection_absent = header[1] & 0x01 != 0;
        let header_size = if protection_absent { 7 } else { 9 };
        meta.exif.set("AAC:HeaderSize", AttrValue::UInt(header_size));

        // Profile (bits 6-7 of byte 2): 0=Main, 1=LC, 2=SSR, 3=LTP
        let profile = (header[2] >> 6) & 0x03;
        let profile_name = match profile {
            0 => "Main",
            1 => "LC (Low Complexity)",
            2 => "SSR (Scalable Sample Rate)",
            3 => "LTP (Long Term Prediction)",
            _ => "Unknown",
        };
        meta.exif.set("AAC:Profile", AttrValue::Str(profile_name.to_string()));

        // Sample rate index (bits 2-5 of byte 2)
        let sr_index = (header[2] >> 2) & 0x0F;
        let sample_rate = match sr_index {
            0 => 96000,
            1 => 88200,
            2 => 64000,
            3 => 48000,
            4 => 44100,
            5 => 32000,
            6 => 24000,
            7 => 22050,
            8 => 16000,
            9 => 12000,
            10 => 11025,
            11 => 8000,
            12 => 7350,
            _ => 0,
        };
        if sample_rate > 0 {
            meta.exif.set("Audio:SampleRate", AttrValue::UInt(sample_rate));
        }

        // Channel configuration (bit 0 of byte 2 + bits 6-7 of byte 3)
        let channel_config = ((header[2] & 0x01) << 2) | ((header[3] >> 6) & 0x03);
        let (channels, channel_mode) = match channel_config {
            0 => (0, "Defined in AOT"),
            1 => (1, "Mono"),
            2 => (2, "Stereo"),
            3 => (3, "3.0"),
            4 => (4, "4.0"),
            5 => (5, "5.0"),
            6 => (6, "5.1"),
            7 => (8, "7.1"),
            _ => (0, "Unknown"),
        };
        if channels > 0 {
            meta.exif.set("Audio:Channels", AttrValue::UInt(channels));
        }
        meta.exif.set("Audio:ChannelMode", AttrValue::Str(channel_mode.to_string()));

        // Frame length (13 bits: bits 0-1 of byte 3 + byte 4 + bits 5-7 of byte 5)
        let frame_length = (((header[3] & 0x03) as u32) << 11)
            | ((header[4] as u32) << 3)
            | ((header[5] >> 5) as u32);
        meta.exif.set("AAC:FrameLength", AttrValue::UInt(frame_length));

        // Calculate approximate duration by counting frames
        if sample_rate > 0 && frame_length > 0 {
            reader.seek(SeekFrom::Start(0))?;
            let frame_count = count_adts_frames(reader, file_size, 1000)?;
            
            if frame_count > 0 {
                meta.exif.set("AAC:FrameCount", AttrValue::UInt(frame_count));
                
                // Each AAC frame = 1024 samples
                let total_samples = frame_count as u64 * 1024;
                let duration = total_samples as f64 / sample_rate as f64;
                meta.exif.set("Audio:Duration", AttrValue::Double(duration));

                // Estimate bitrate
                let bitrate = (file_size as f64 * 8.0 / duration / 1000.0) as u32;
                if bitrate > 0 && bitrate < 1000 {
                    meta.exif.set("Audio:Bitrate", AttrValue::UInt(bitrate));
                    meta.exif.set("Audio:BitrateMode", AttrValue::Str("VBR".to_string()));
                }
            }
        }

        Ok(meta)
    }
}

/// Count ADTS frames (up to max_frames for estimation).
fn count_adts_frames(reader: &mut dyn ReadSeek, file_size: u64, max_frames: u32) -> Result<u32> {
    let mut count = 0u32;
    let mut pos = 0u64;

    while pos < file_size && count < max_frames {
        reader.seek(SeekFrom::Start(pos))?;
        
        let mut header = [0u8; 7];
        if reader.read_exact(&mut header).is_err() {
            break;
        }

        // Verify sync word
        if header[0] != 0xFF || (header[1] & 0xF0) != 0xF0 {
            break;
        }

        // Get frame length
        let frame_length = (((header[3] & 0x03) as u64) << 11)
            | ((header[4] as u64) << 3)
            | ((header[5] >> 5) as u64);

        if frame_length < 7 || frame_length > 8192 {
            break;
        }

        count += 1;
        pos += frame_length;
    }

    // Estimate total frames if we hit max
    if count >= max_frames && pos > 0 {
        let avg_frame_size = pos / count as u64;
        if avg_frame_size > 0 {
            count = (file_size / avg_frame_size) as u32;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_adts_frame() -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // ADTS header (7 bytes)
        // Sync word + MPEG-4 + Layer 0 + No CRC
        data[0] = 0xFF;
        data[1] = 0xF1; // 1111 0001: MPEG-4, layer 0, no CRC
        // Profile LC (01) + SR index 4 (44100) + private 0 + channel config start
        data[2] = 0x50; // 0101 0000: LC profile, 44100 Hz
        // Channel config (2=stereo) + frame length start
        data[3] = 0x80; // 1000 0000: stereo
        // Frame length middle
        data[4] = 0x00;
        // Frame length end + buffer fullness start
        data[5] = 0x1F; // frame length = 7 (minimum)
        data[6] = 0xFC;
        data
    }

    #[test]
    fn test_can_parse() {
        let parser = AacParser;
        let data = make_adts_frame();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = AacParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"RIFF"));
        assert!(!parser.can_parse(&[0xFF, 0x00])); // Not sync word
    }

    #[test]
    fn test_parse_basic() {
        let parser = AacParser;
        let data = make_adts_frame();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "AAC");
        assert_eq!(meta.exif.get_u32("Audio:SampleRate"), Some(44100));
        assert_eq!(meta.exif.get_str("AAC:Profile"), Some("LC (Low Complexity)"));
    }

    #[test]
    fn test_format_info() {
        let parser = AacParser;
        assert_eq!(parser.format_name(), "AAC");
        assert!(parser.extensions().contains(&"aac"));
    }
}

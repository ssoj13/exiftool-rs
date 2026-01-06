//! WavPack format parser.
//!
//! WavPack is a lossless/hybrid audio compression format.
//! Files start with "wvpk" magic.

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// WavPack format parser.
pub struct WvParser;

impl FormatParser for WvParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 4 {
            return false;
        }
        // "wvpk" magic
        &header[0..4] == b"wvpk"
    }

    fn format_name(&self) -> &'static str {
        "WV"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["wv"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("WV");
        meta.exif.set("File:FileType", AttrValue::Str("WV".to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("audio/x-wavpack".to_string()));
        meta.exif.set("Audio:Codec", AttrValue::Str("WavPack".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // Read WavPack block header (32 bytes)
        let mut header = [0u8; 32];
        reader.read_exact(&mut header)?;

        // Block size (offset 4, 4 bytes LE) - includes header
        let block_size = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
        meta.exif.set("WV:BlockSize", AttrValue::UInt(block_size));

        // Version (offset 8, 2 bytes LE)
        let version = u16::from_le_bytes([header[8], header[9]]);
        meta.exif.set("WV:Version", AttrValue::UInt(version as u32));
        
        let version_str = format!("{}.{}", version >> 8, version & 0xFF);
        meta.exif.set("WV:VersionString", AttrValue::Str(version_str));

        // Track number (offset 10, 1 byte)
        let track_no = header[10];
        if track_no > 0 {
            meta.exif.set("Audio:Track", AttrValue::UInt(track_no as u32));
        }

        // Index number (offset 11, 1 byte)
        let index_no = header[11];
        if index_no > 0 {
            meta.exif.set("WV:IndexNumber", AttrValue::UInt(index_no as u32));
        }

        // Total samples (offset 12, 4 bytes LE) - 0xFFFFFFFF means unknown
        let total_samples = u32::from_le_bytes([header[12], header[13], header[14], header[15]]);
        if total_samples != 0xFFFFFFFF {
            meta.exif.set("WV:TotalSamples", AttrValue::UInt(total_samples));
        }

        // Block index (offset 16, 4 bytes LE)
        let _block_index = u32::from_le_bytes([header[16], header[17], header[18], header[19]]);

        // Block samples (offset 20, 4 bytes LE)
        let block_samples = u32::from_le_bytes([header[20], header[21], header[22], header[23]]);
        meta.exif.set("WV:BlockSamples", AttrValue::UInt(block_samples));

        // Flags (offset 24, 4 bytes LE)
        let flags = u32::from_le_bytes([header[24], header[25], header[26], header[27]]);
        
        // Bits per sample from flags (bits 0-1)
        let bps_code = flags & 0x03;
        let bits_per_sample = match bps_code {
            0 => 8,
            1 => 16,
            2 => 24,
            3 => 32,
            _ => 16,
        };
        meta.exif.set("Audio:BitsPerSample", AttrValue::UInt(bits_per_sample));

        // Mono/stereo from flags (bit 2)
        let is_mono = (flags & 0x04) != 0;
        let channels = if is_mono { 1 } else { 2 };
        meta.exif.set("Audio:Channels", AttrValue::UInt(channels));
        meta.exif.set("Audio:ChannelMode", AttrValue::Str(
            if is_mono { "Mono" } else { "Stereo" }.to_string()
        ));

        // Hybrid mode (bit 3)
        let is_hybrid = (flags & 0x08) != 0;
        if is_hybrid {
            meta.exif.set("WV:EncodingMode", AttrValue::Str("Hybrid".to_string()));
        } else {
            meta.exif.set("WV:EncodingMode", AttrValue::Str("Lossless".to_string()));
        }

        // Joint stereo (bit 4)
        let is_joint = (flags & 0x10) != 0;
        if is_joint && !is_mono {
            meta.exif.set("WV:JointStereo", AttrValue::Str("Yes".to_string()));
        }

        // Sample rate from flags (bits 23-26)
        let sr_code = (flags >> 23) & 0x0F;
        let sample_rate = match sr_code {
            0 => 6000,
            1 => 8000,
            2 => 9600,
            3 => 11025,
            4 => 12000,
            5 => 16000,
            6 => 22050,
            7 => 24000,
            8 => 32000,
            9 => 44100,
            10 => 48000,
            11 => 64000,
            12 => 88200,
            13 => 96000,
            14 => 192000,
            _ => 44100,
        };
        meta.exif.set("Audio:SampleRate", AttrValue::UInt(sample_rate));

        // Calculate duration
        if total_samples != 0xFFFFFFFF && sample_rate > 0 {
            let duration = total_samples as f64 / sample_rate as f64;
            meta.exif.set("Audio:Duration", AttrValue::Double(duration));
            
            let mins = (duration / 60.0) as u32;
            let secs = (duration % 60.0) as u32;
            meta.exif.set("Audio:DurationFormatted", 
                AttrValue::Str(format!("{}:{:02}", mins, secs)));
        }

        // File size
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        // Try to find APEv2 tag at end
        if file_size > 32 {
            reader.seek(SeekFrom::Start(file_size - 32))?;
            let mut footer = [0u8; 32];
            if reader.read_exact(&mut footer).is_ok() && &footer[0..8] == b"APETAGEX" {
                meta.exif.set("WV:HasAPEv2Tag", AttrValue::Str("Yes".to_string()));
            }
        }

        Ok(meta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_wv_header() -> Vec<u8> {
        let mut data = vec![0u8; 1024];
        // Magic "wvpk"
        data[0..4].copy_from_slice(b"wvpk");
        // Block size
        data[4..8].copy_from_slice(&1000u32.to_le_bytes());
        // Version (4.80 = 0x0480)
        data[8..10].copy_from_slice(&0x0480u16.to_le_bytes());
        // Track/index
        data[10] = 0;
        data[11] = 0;
        // Total samples
        data[12..16].copy_from_slice(&441000u32.to_le_bytes()); // 10 seconds at 44100
        // Block index
        data[16..20].copy_from_slice(&0u32.to_le_bytes());
        // Block samples
        data[20..24].copy_from_slice(&4096u32.to_le_bytes());
        // Flags: 16-bit (1), stereo (0), lossless (0), 44100 Hz (9 << 23)
        let flags: u32 = 0x01 | (9 << 23);
        data[24..28].copy_from_slice(&flags.to_le_bytes());
        data
    }

    #[test]
    fn test_can_parse() {
        let parser = WvParser;
        let data = make_wv_header();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = WvParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"OggS"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = WvParser;
        let data = make_wv_header();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "WV");
        assert_eq!(meta.exif.get_u32("Audio:SampleRate"), Some(44100));
        assert_eq!(meta.exif.get_u32("Audio:Channels"), Some(2));
        assert_eq!(meta.exif.get_str("Audio:ChannelMode"), Some("Stereo"));
        assert_eq!(meta.exif.get_u32("Audio:BitsPerSample"), Some(16));
        assert_eq!(meta.exif.get_str("WV:EncodingMode"), Some("Lossless"));
    }

    #[test]
    fn test_format_info() {
        let parser = WvParser;
        assert_eq!(parser.format_name(), "WV");
        assert!(parser.extensions().contains(&"wv"));
    }
}

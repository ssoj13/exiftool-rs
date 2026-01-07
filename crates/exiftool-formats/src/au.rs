//! AU (Sun/NeXT Audio) format parser.
//!
//! AU is a simple audio format developed by Sun Microsystems.
//! Also known as SND format on NeXT systems.
//!
//! # Structure
//!
//! - 4 bytes: Magic ".snd" (0x2E736E64)
//! - 4 bytes: Data offset
//! - 4 bytes: Data size (0xFFFFFFFF = unknown)
//! - 4 bytes: Encoding format
//! - 4 bytes: Sample rate
//! - 4 bytes: Channels
//! - Variable: Annotation (null-terminated string)
//! - Audio data

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// AU format parser.
pub struct AuParser;

impl FormatParser for AuParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 4 {
            return false;
        }
        // ".snd" magic (big-endian)
        header[0..4] == [0x2E, 0x73, 0x6E, 0x64]
    }

    fn format_name(&self) -> &'static str {
        "AU"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["au", "snd"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("AU");
        meta.set_file_type("AU", "audio/basic");

        // Read header (24 bytes minimum)
        let mut header = [0u8; 24];
        reader.seek(SeekFrom::Start(0))?;
        reader.read_exact(&mut header)?;

        // All values are big-endian
        let data_offset = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);
        let data_size = u32::from_be_bytes([header[8], header[9], header[10], header[11]]);
        let encoding = u32::from_be_bytes([header[12], header[13], header[14], header[15]]);
        let sample_rate = u32::from_be_bytes([header[16], header[17], header[18], header[19]]);
        let channels = u32::from_be_bytes([header[20], header[21], header[22], header[23]]);

        meta.exif.set("AU:DataOffset", AttrValue::UInt(data_offset));
        
        if data_size != 0xFFFFFFFF {
            meta.exif.set("AU:DataSize", AttrValue::UInt(data_size));
        }

        meta.exif.set("Audio:SampleRate", AttrValue::UInt(sample_rate));
        meta.exif.set("Audio:Channels", AttrValue::UInt(channels));

        // Channel description
        let channel_desc = match channels {
            1 => "Mono",
            2 => "Stereo",
            _ => "Multi-channel",
        };
        meta.exif.set("Audio:ChannelMode", AttrValue::Str(channel_desc.to_string()));

        // Encoding format
        let (encoding_name, bits_per_sample) = encoding_info(encoding);
        meta.exif.set("AU:Encoding", AttrValue::Str(encoding_name.to_string()));
        meta.exif.set("AU:EncodingID", AttrValue::UInt(encoding));
        
        if bits_per_sample > 0 {
            meta.exif.set("Audio:BitsPerSample", AttrValue::UInt(bits_per_sample));
        }

        // Calculate duration if we have enough info
        if data_size != 0xFFFFFFFF && sample_rate > 0 && bits_per_sample > 0 && channels > 0 {
            let bytes_per_sample = bits_per_sample / 8;
            let total_samples = data_size / (bytes_per_sample * channels);
            let duration = total_samples as f64 / sample_rate as f64;
            meta.exif.set("Audio:Duration", AttrValue::Double(duration));
            
            // Format duration as MM:SS
            let mins = (duration / 60.0) as u32;
            let secs = (duration % 60.0) as u32;
            meta.exif.set("Audio:DurationFormatted", 
                AttrValue::Str(format!("{}:{:02}", mins, secs)));
        }

        // Read annotation if present (between header and data)
        if data_offset > 24 {
            let anno_len = (data_offset - 24) as usize;
            if anno_len > 0 && anno_len < 65536 {
                let mut anno = vec![0u8; anno_len];
                if reader.read_exact(&mut anno).is_ok() {
                    // Null-terminated string
                    let end = anno.iter().position(|&b| b == 0).unwrap_or(anno.len());
                    let text = String::from_utf8_lossy(&anno[..end]).trim().to_string();
                    if !text.is_empty() {
                        meta.exif.set("AU:Annotation", AttrValue::Str(text));
                    }
                }
            }
        }

        // File size
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        Ok(meta)
    }
}

/// Get encoding name and bits per sample for AU encoding ID.
fn encoding_info(encoding: u32) -> (&'static str, u32) {
    match encoding {
        1 => ("8-bit G.711 mu-law", 8),
        2 => ("8-bit linear PCM", 8),
        3 => ("16-bit linear PCM", 16),
        4 => ("24-bit linear PCM", 24),
        5 => ("32-bit linear PCM", 32),
        6 => ("32-bit IEEE floating point", 32),
        7 => ("64-bit IEEE floating point", 64),
        8 => ("Fragmented sample data", 0),
        9 => ("DSP program", 0),
        10 => ("8-bit fixed point", 8),
        11 => ("16-bit fixed point", 16),
        12 => ("24-bit fixed point", 24),
        13 => ("32-bit fixed point", 32),
        18 => ("16-bit linear PCM with emphasis", 16),
        19 => ("16-bit linear PCM compressed", 16),
        20 => ("16-bit linear PCM with emphasis and compression", 16),
        21 => ("Music Kit DSP commands", 0),
        23 => ("4-bit G.721 ADPCM", 4),
        24 => ("G.722 ADPCM", 0),
        25 => ("3-bit G.723 ADPCM", 3),
        26 => ("5-bit G.723 ADPCM", 5),
        27 => ("8-bit G.711 A-law", 8),
        _ => ("Unknown", 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_au_header(sample_rate: u32, channels: u32, encoding: u32) -> Vec<u8> {
        let mut data = vec![0u8; 1024];
        // Magic ".snd"
        data[0..4].copy_from_slice(&[0x2E, 0x73, 0x6E, 0x64]);
        // Data offset (24 = header only)
        data[4..8].copy_from_slice(&24u32.to_be_bytes());
        // Data size
        data[8..12].copy_from_slice(&1000u32.to_be_bytes());
        // Encoding
        data[12..16].copy_from_slice(&encoding.to_be_bytes());
        // Sample rate
        data[16..20].copy_from_slice(&sample_rate.to_be_bytes());
        // Channels
        data[20..24].copy_from_slice(&channels.to_be_bytes());
        data
    }

    #[test]
    fn test_can_parse() {
        let parser = AuParser;
        let data = make_au_header(44100, 2, 3);
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = AuParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"RIFF"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = AuParser;
        let data = make_au_header(44100, 2, 3); // 16-bit PCM stereo
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "AU");
        assert_eq!(meta.exif.get_u32("Audio:SampleRate"), Some(44100));
        assert_eq!(meta.exif.get_u32("Audio:Channels"), Some(2));
        assert_eq!(meta.exif.get_str("Audio:ChannelMode"), Some("Stereo"));
        assert_eq!(meta.exif.get_str("AU:Encoding"), Some("16-bit linear PCM"));
    }

    #[test]
    fn test_parse_mono_mulaw() {
        let parser = AuParser;
        let data = make_au_header(8000, 1, 1); // mu-law mono (telephone quality)
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_u32("Audio:SampleRate"), Some(8000));
        assert_eq!(meta.exif.get_u32("Audio:Channels"), Some(1));
        assert_eq!(meta.exif.get_str("Audio:ChannelMode"), Some("Mono"));
        assert_eq!(meta.exif.get_str("AU:Encoding"), Some("8-bit G.711 mu-law"));
    }

    #[test]
    fn test_parse_with_annotation() {
        let parser = AuParser;
        let mut data = vec![0u8; 1024];
        data[0..4].copy_from_slice(&[0x2E, 0x73, 0x6E, 0x64]);
        data[4..8].copy_from_slice(&48u32.to_be_bytes()); // offset = 48 (24 bytes for annotation)
        data[8..12].copy_from_slice(&1000u32.to_be_bytes());
        data[12..16].copy_from_slice(&3u32.to_be_bytes());
        data[16..20].copy_from_slice(&22050u32.to_be_bytes());
        data[20..24].copy_from_slice(&1u32.to_be_bytes());
        // Annotation
        data[24..36].copy_from_slice(b"Test Audio\0\0");

        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("AU:Annotation"), Some("Test Audio"));
    }

    #[test]
    fn test_format_info() {
        let parser = AuParser;
        assert_eq!(parser.format_name(), "AU");
        assert!(parser.extensions().contains(&"au"));
        assert!(parser.extensions().contains(&"snd"));
    }
}

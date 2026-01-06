//! AIFF/AIFC (Audio Interchange File Format) parser.
//!
//! AIFF is Apple's uncompressed audio format, similar to WAV but big-endian.
//! AIFC is the compressed variant.
//!
//! # Structure
//!
//! - 4 bytes: "FORM"
//! - 4 bytes: File size - 8 (big-endian)
//! - 4 bytes: "AIFF" or "AIFC"
//! - Chunks: COMM, SSND, ANNO, AUTH, NAME, etc.

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// AIFF format parser.
pub struct AiffParser;

impl FormatParser for AiffParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 12 {
            return false;
        }
        // FORM + size + AIFF/AIFC
        &header[0..4] == b"FORM"
            && (&header[8..12] == b"AIFF" || &header[8..12] == b"AIFC")
    }

    fn format_name(&self) -> &'static str {
        "AIFF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["aiff", "aif", "aifc"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("AIFF");

        // Read header
        let mut header = [0u8; 12];
        reader.seek(SeekFrom::Start(0))?;
        reader.read_exact(&mut header)?;

        // Validate
        if &header[0..4] != b"FORM" {
            return Err(crate::Error::InvalidStructure("Not a valid AIFF file".to_string()));
        }

        // File size
        let chunk_size = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);
        let file_size = chunk_size as u64 + 8;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        // Format type
        let is_aifc = &header[8..12] == b"AIFC";
        let format_name = if is_aifc { "AIFC" } else { "AIFF" };
        meta.format = if is_aifc { "AIFC" } else { "AIFF" };
        meta.exif.set("File:FileType", AttrValue::Str(format_name.to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("audio/aiff".to_string()));

        // Parse chunks
        self.parse_chunks(reader, &mut meta, 12, file_size)?;

        Ok(meta)
    }
}

impl AiffParser {
    /// Parse AIFF chunks.
    fn parse_chunks(
        &self,
        reader: &mut dyn ReadSeek,
        meta: &mut Metadata,
        start: u64,
        end: u64,
    ) -> Result<()> {
        let mut pos = start;

        while pos + 8 <= end {
            reader.seek(SeekFrom::Start(pos))?;

            let mut chunk_header = [0u8; 8];
            if reader.read_exact(&mut chunk_header).is_err() {
                break;
            }

            let chunk_id = &chunk_header[0..4];
            let chunk_size = u32::from_be_bytes([
                chunk_header[4],
                chunk_header[5],
                chunk_header[6],
                chunk_header[7],
            ]) as u64;

            // Chunk data starts at pos + 8
            let data_start = pos + 8;

            match chunk_id {
                b"COMM" => self.parse_comm(reader, meta, data_start, chunk_size)?,
                b"SSND" => {
                    meta.exif.set("AIFF:SoundDataOffset", AttrValue::UInt64(data_start));
                    meta.exif.set("AIFF:SoundDataSize", AttrValue::UInt64(chunk_size));
                }
                b"NAME" => {
                    if let Some(s) = self.read_string(reader, data_start, chunk_size)? {
                        meta.exif.set("AIFF:Name", AttrValue::Str(s));
                    }
                }
                b"AUTH" => {
                    if let Some(s) = self.read_string(reader, data_start, chunk_size)? {
                        meta.exif.set("AIFF:Author", AttrValue::Str(s));
                    }
                }
                b"(c) " => {
                    if let Some(s) = self.read_string(reader, data_start, chunk_size)? {
                        meta.exif.set("AIFF:Copyright", AttrValue::Str(s));
                    }
                }
                b"ANNO" => {
                    if let Some(s) = self.read_string(reader, data_start, chunk_size)? {
                        meta.exif.set("AIFF:Annotation", AttrValue::Str(s));
                    }
                }
                b"COMT" => self.parse_comments(reader, meta, data_start, chunk_size)?,
                b"ID3 " => {
                    meta.exif.set("AIFF:ID3Offset", AttrValue::UInt64(data_start));
                    meta.exif.set("AIFF:ID3Size", AttrValue::UInt64(chunk_size));
                }
                _ => {}
            }

            // Move to next chunk (pad to even boundary)
            pos = data_start + chunk_size;
            if chunk_size % 2 != 0 {
                pos += 1;
            }
        }

        Ok(())
    }

    /// Parse COMM (Common) chunk.
    fn parse_comm(
        &self,
        reader: &mut dyn ReadSeek,
        meta: &mut Metadata,
        offset: u64,
        size: u64,
    ) -> Result<()> {
        if size < 18 {
            return Ok(());
        }

        reader.seek(SeekFrom::Start(offset))?;
        let mut data = [0u8; 18];
        reader.read_exact(&mut data)?;

        // Number of channels
        let num_channels = u16::from_be_bytes([data[0], data[1]]);
        meta.exif.set("AIFF:NumChannels", AttrValue::UInt(num_channels as u32));

        // Number of sample frames
        let num_frames = u32::from_be_bytes([data[2], data[3], data[4], data[5]]);
        meta.exif.set("AIFF:NumSampleFrames", AttrValue::UInt(num_frames));

        // Sample size (bits)
        let sample_size = u16::from_be_bytes([data[6], data[7]]);
        meta.exif.set("AIFF:BitsPerSample", AttrValue::UInt(sample_size as u32));

        // Sample rate (80-bit extended precision float)
        let sample_rate = self.read_extended(&data[8..18]);
        meta.exif.set("AIFF:SampleRate", AttrValue::UInt(sample_rate as u32));

        // Calculate duration
        if sample_rate > 0.0 {
            let duration = num_frames as f64 / sample_rate;
            meta.exif.set("AIFF:Duration", AttrValue::Float(duration as f32));

            // Calculate bitrate
            let bitrate = (sample_rate * num_channels as f64 * sample_size as f64) / 1000.0;
            meta.exif.set("AIFF:Bitrate", AttrValue::Str(format!("{:.0} kbps", bitrate)));
        }

        // Audio channels description
        let channels_desc = match num_channels {
            1 => "Mono",
            2 => "Stereo",
            6 => "5.1 Surround",
            8 => "7.1 Surround",
            _ => "Multi-channel",
        };
        meta.exif.set("AIFF:ChannelMode", AttrValue::Str(channels_desc.to_string()));

        Ok(())
    }

    /// Parse COMT (Comments) chunk.
    fn parse_comments(
        &self,
        reader: &mut dyn ReadSeek,
        meta: &mut Metadata,
        offset: u64,
        _size: u64,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(offset))?;

        let mut count_bytes = [0u8; 2];
        if reader.read_exact(&mut count_bytes).is_err() {
            return Ok(());
        }
        let count = u16::from_be_bytes(count_bytes) as usize;

        if count > 100 {
            return Ok(());
        }

        let mut comments = Vec::new();
        for _ in 0..count {
            // Timestamp (4 bytes) + marker (2 bytes) + count (2 bytes)
            let mut header = [0u8; 8];
            if reader.read_exact(&mut header).is_err() {
                break;
            }
            let text_len = u16::from_be_bytes([header[6], header[7]]) as usize;

            if text_len > 0 && text_len < 10000 {
                let mut text = vec![0u8; text_len];
                if reader.read_exact(&mut text).is_ok() {
                    let s = String::from_utf8_lossy(&text).trim().to_string();
                    if !s.is_empty() {
                        comments.push(s);
                    }
                }
            }

            // Pad to even
            if text_len % 2 != 0 {
                let mut pad = [0u8; 1];
                let _ = reader.read_exact(&mut pad);
            }
        }

        if !comments.is_empty() {
            meta.exif.set("AIFF:Comments", AttrValue::Str(comments.join("; ")));
        }

        Ok(())
    }

    /// Read string chunk.
    fn read_string(
        &self,
        reader: &mut dyn ReadSeek,
        offset: u64,
        size: u64,
    ) -> Result<Option<String>> {
        if size == 0 || size > 10000 {
            return Ok(None);
        }

        reader.seek(SeekFrom::Start(offset))?;
        let mut data = vec![0u8; size as usize];
        reader.read_exact(&mut data)?;

        let s = String::from_utf8_lossy(&data).trim().to_string();
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s))
        }
    }

    /// Read 80-bit extended precision float (IEEE 754).
    fn read_extended(&self, data: &[u8]) -> f64 {
        if data.len() < 10 {
            return 0.0;
        }

        let sign = (data[0] & 0x80) != 0;
        let exponent = (((data[0] & 0x7F) as u16) << 8) | (data[1] as u16);
        let mantissa = u64::from_be_bytes([
            data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9],
        ]);

        if exponent == 0 && mantissa == 0 {
            return 0.0;
        }

        // Convert to f64
        let exp = (exponent as i32) - 16383 - 63;
        let mut value = (mantissa as f64) * 2.0_f64.powi(exp);

        if sign {
            value = -value;
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_aiff_file() -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // FORM header
        data[0..4].copy_from_slice(b"FORM");
        data[4..8].copy_from_slice(&504u32.to_be_bytes()); // size
        data[8..12].copy_from_slice(b"AIFF");

        // COMM chunk at offset 12
        data[12..16].copy_from_slice(b"COMM");
        data[16..20].copy_from_slice(&18u32.to_be_bytes()); // size

        // COMM data
        data[20..22].copy_from_slice(&2u16.to_be_bytes()); // channels
        data[22..26].copy_from_slice(&44100u32.to_be_bytes()); // frames
        data[26..28].copy_from_slice(&16u16.to_be_bytes()); // bits

        // Sample rate as 80-bit extended (44100 Hz)
        // 44100 = 0xAC44, exponent = 16383 + 15 = 16398 = 0x400E
        data[28] = 0x40;
        data[29] = 0x0E;
        data[30] = 0xAC;
        data[31] = 0x44;
        // Rest zeros

        data
    }

    #[test]
    fn test_can_parse() {
        let parser = AiffParser;
        let data = make_aiff_file();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = AiffParser;
        assert!(!parser.can_parse(b"RIFF....WAVE")); // WAV, not AIFF
        assert!(!parser.can_parse(&[0x00; 20]));
    }

    #[test]
    fn test_cannot_parse_wav() {
        let parser = AiffParser;
        let mut wav = vec![0u8; 20];
        wav[0..4].copy_from_slice(b"RIFF");
        wav[8..12].copy_from_slice(b"WAVE");
        assert!(!parser.can_parse(&wav));
    }

    #[test]
    fn test_parse_basic() {
        let parser = AiffParser;
        let data = make_aiff_file();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "AIFF");
        assert_eq!(meta.exif.get_u32("AIFF:NumChannels"), Some(2));
        assert_eq!(meta.exif.get_u32("AIFF:BitsPerSample"), Some(16));
        assert_eq!(meta.exif.get_str("AIFF:ChannelMode"), Some("Stereo"));
    }

    #[test]
    fn test_parse_aifc() {
        let parser = AiffParser;
        let mut data = make_aiff_file();
        data[8..12].copy_from_slice(b"AIFC");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "AIFC");
    }
}

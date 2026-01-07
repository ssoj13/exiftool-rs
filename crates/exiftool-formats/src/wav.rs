//! WAV (Waveform Audio) format parser.
//!
//! WAV uses RIFF container format (same as AVI):
//! - RIFF header: "RIFF" + size + "WAVE"
//! - fmt chunk: audio format parameters
//! - data chunk: audio samples
//! - LIST INFO: metadata (INAM, IART, etc.)
//! - bext: Broadcast Wave extension

use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// WAV parser.
pub struct WavParser;

impl FormatParser for WavParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 12 {
            return false;
        }
        &header[0..4] == b"RIFF" && &header[8..12] == b"WAVE"
    }

    fn format_name(&self) -> &'static str {
        "WAV"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["wav", "wave", "bwf"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut header = [0u8; 12];
        reader.read_exact(&mut header)?;

        if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" {
            return Err(Error::InvalidStructure("Not a valid WAV file".into()));
        }

        let file_size = u32::from_le_bytes([header[4], header[5], header[6], header[7]]) as u64 + 8;

        let mut metadata = Metadata::new("WAV");
        metadata.set_file_type("WAV", "audio/wav");
        metadata.exif.set("File:FileSize", AttrValue::UInt(file_size as u32));

        self.parse_chunks(reader, file_size, &mut metadata)?;

        Ok(metadata)
    }
}

impl WavParser {
    /// Parse RIFF chunks.
    fn parse_chunks(&self, reader: &mut dyn ReadSeek, end_pos: u64, metadata: &mut Metadata) -> Result<()> {
        while reader.stream_position()? < end_pos {
            let chunk_start = reader.stream_position()?;

            let Some((chunk_id, chunk_size)) = crate::riff::read_chunk_header(reader) else {
                break;
            };

            match &chunk_id {
                b"fmt " => {
                    self.parse_fmt(reader, chunk_size, metadata)?;
                }
                b"data" => {
                    metadata.exif.set("WAV:DataSize", AttrValue::UInt(chunk_size as u32));
                    reader.seek(SeekFrom::Current(chunk_size as i64))?;
                }
                b"fact" => {
                    self.parse_fact(reader, chunk_size, metadata)?;
                }
                b"LIST" => {
                    let mut form_type = [0u8; 4];
                    if reader.read_exact(&mut form_type).is_err() {
                        break;
                    }

                    let list_end = chunk_start + 8 + chunk_size;

                    if &form_type == b"INFO" {
                        crate::riff::parse_info(reader, list_end, metadata)?;
                    } else {
                        reader.seek(SeekFrom::Start(list_end))?;
                    }
                }
                b"bext" => {
                    self.parse_bext(reader, chunk_size, metadata)?;
                }
                b"iXML" | b"_PMX" | b"XMP " => {
                    self.parse_xmp(reader, chunk_size, metadata)?;
                }
                b"cue " => {
                    metadata.exif.set("WAV:HasCuePoints", AttrValue::Bool(true));
                    reader.seek(SeekFrom::Current(chunk_size as i64))?;
                }
                b"smpl" => {
                    self.parse_smpl(reader, chunk_size, metadata)?;
                }
                b"JUNK" | b"PAD " => {
                    reader.seek(SeekFrom::Current(chunk_size as i64))?;
                }
                _ => {
                    reader.seek(SeekFrom::Current(chunk_size as i64))?;
                }
            }

            // Word alignment
            let current = reader.stream_position()?;
            if current % 2 != 0 {
                reader.seek(SeekFrom::Current(1))?;
            }

            if reader.stream_position()? <= chunk_start {
                break;
            }
        }

        Ok(())
    }

    /// Parse fmt chunk (audio format).
    fn parse_fmt(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if size < 16 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(());
        }

        let mut fmt = [0u8; 16];
        reader.read_exact(&mut fmt)?;

        let audio_format = u16::from_le_bytes([fmt[0], fmt[1]]);
        let num_channels = u16::from_le_bytes([fmt[2], fmt[3]]);
        let sample_rate = u32::from_le_bytes([fmt[4], fmt[5], fmt[6], fmt[7]]);
        let byte_rate = u32::from_le_bytes([fmt[8], fmt[9], fmt[10], fmt[11]]);
        let _block_align = u16::from_le_bytes([fmt[12], fmt[13]]);
        let bits_per_sample = u16::from_le_bytes([fmt[14], fmt[15]]);

        // Audio format
        let format_name = match audio_format {
            0x0001 => "PCM",
            0x0002 => "ADPCM",
            0x0003 => "IEEE Float",
            0x0006 => "A-Law",
            0x0007 => "Mu-Law",
            0x0011 => "IMA ADPCM",
            0x0055 => "MP3",
            0x00FF => "AAC",
            0xFFFE => "Extensible",
            _ => "Unknown",
        };
        metadata.exif.set("WAV:AudioFormat", AttrValue::Str(format_name.to_string()));
        metadata.exif.set("WAV:AudioFormatID", AttrValue::UInt(audio_format as u32));

        metadata.exif.set("WAV:NumChannels", AttrValue::UInt(num_channels as u32));
        metadata.exif.set("WAV:SampleRate", AttrValue::UInt(sample_rate));
        metadata.exif.set("WAV:ByteRate", AttrValue::UInt(byte_rate));
        metadata.exif.set("WAV:BitsPerSample", AttrValue::UInt(bits_per_sample as u32));

        // Calculate bitrate
        let bitrate = byte_rate * 8 / 1000;
        metadata.exif.set("WAV:Bitrate", AttrValue::UInt(bitrate));

        // Skip remaining (extended format info)
        let remaining = size.saturating_sub(16);
        if remaining > 0 {
            reader.seek(SeekFrom::Current(remaining as i64))?;
        }

        Ok(())
    }

    /// Parse fact chunk (sample count for compressed audio).
    fn parse_fact(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if size >= 4 {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            let sample_count = u32::from_le_bytes(buf);
            metadata.exif.set("WAV:SampleCount", AttrValue::UInt(sample_count));

            let remaining = size.saturating_sub(4);
            if remaining > 0 {
                reader.seek(SeekFrom::Current(remaining as i64))?;
            }
        } else {
            reader.seek(SeekFrom::Current(size as i64))?;
        }

        Ok(())
    }

    /// Parse bext chunk (Broadcast Wave Extension).
    fn parse_bext(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if size < 602 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(());
        }

        metadata.exif.set("WAV:IsBroadcastWave", AttrValue::Bool(true));

        let mut bext = vec![0u8; 602];
        reader.read_exact(&mut bext)?;

        // Description (256 bytes)
        let desc = String::from_utf8_lossy(&bext[0..256])
            .trim_end_matches('\0')
            .to_string();
        if !desc.is_empty() {
            metadata.exif.set("BWF:Description", AttrValue::Str(desc));
        }

        // Originator (32 bytes)
        let originator = String::from_utf8_lossy(&bext[256..288])
            .trim_end_matches('\0')
            .to_string();
        if !originator.is_empty() {
            metadata.exif.set("BWF:Originator", AttrValue::Str(originator));
        }

        // OriginatorReference (32 bytes)
        let orig_ref = String::from_utf8_lossy(&bext[288..320])
            .trim_end_matches('\0')
            .to_string();
        if !orig_ref.is_empty() {
            metadata.exif.set("BWF:OriginatorReference", AttrValue::Str(orig_ref));
        }

        // OriginationDate (10 bytes) YYYY-MM-DD
        let date = String::from_utf8_lossy(&bext[320..330])
            .trim_end_matches('\0')
            .to_string();
        if !date.is_empty() && date.len() == 10 {
            metadata.exif.set("BWF:OriginationDate", AttrValue::Str(date));
        }

        // OriginationTime (8 bytes) HH:MM:SS
        let time = String::from_utf8_lossy(&bext[330..338])
            .trim_end_matches('\0')
            .to_string();
        if !time.is_empty() && time.len() == 8 {
            metadata.exif.set("BWF:OriginationTime", AttrValue::Str(time));
        }

        // TimeReference (8 bytes) - sample count since midnight
        let time_ref = u64::from_le_bytes([
            bext[338], bext[339], bext[340], bext[341],
            bext[342], bext[343], bext[344], bext[345],
        ]);
        if time_ref > 0 {
            metadata.exif.set("BWF:TimeReference", AttrValue::UInt(time_ref as u32));
        }

        // Version (2 bytes)
        let version = u16::from_le_bytes([bext[346], bext[347]]);
        metadata.exif.set("BWF:Version", AttrValue::UInt(version as u32));

        // UMID (64 bytes)
        // SMPTE UMID - complex format, just note presence
        let has_umid = bext[348..412].iter().any(|&b| b != 0);
        if has_umid {
            metadata.exif.set("BWF:HasUMID", AttrValue::Bool(true));
        }

        // CodingHistory (at end, variable length)
        let remaining = size.saturating_sub(602);
        if remaining > 0 && remaining < 64 * 1024 {
            let mut history = vec![0u8; remaining as usize];
            reader.read_exact(&mut history)?;
            let history_str = String::from_utf8_lossy(&history)
                .trim_end_matches('\0')
                .to_string();
            if !history_str.is_empty() {
                metadata.exif.set("BWF:CodingHistory", AttrValue::Str(history_str));
            }
        } else if remaining > 0 {
            reader.seek(SeekFrom::Current(remaining as i64))?;
        }

        Ok(())
    }

    /// Parse smpl chunk (sampler info).
    fn parse_smpl(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if size < 36 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(());
        }

        let mut smpl = [0u8; 36];
        reader.read_exact(&mut smpl)?;

        let _manufacturer = u32::from_le_bytes([smpl[0], smpl[1], smpl[2], smpl[3]]);
        let _product = u32::from_le_bytes([smpl[4], smpl[5], smpl[6], smpl[7]]);
        let sample_period = u32::from_le_bytes([smpl[8], smpl[9], smpl[10], smpl[11]]);
        let midi_unity_note = u32::from_le_bytes([smpl[12], smpl[13], smpl[14], smpl[15]]);
        let _midi_pitch_fraction = u32::from_le_bytes([smpl[16], smpl[17], smpl[18], smpl[19]]);
        let _smpte_format = u32::from_le_bytes([smpl[20], smpl[21], smpl[22], smpl[23]]);
        let _smpte_offset = u32::from_le_bytes([smpl[24], smpl[25], smpl[26], smpl[27]]);
        let num_loops = u32::from_le_bytes([smpl[28], smpl[29], smpl[30], smpl[31]]);
        let _sampler_data = u32::from_le_bytes([smpl[32], smpl[33], smpl[34], smpl[35]]);

        metadata.exif.set("WAV:HasSamplerInfo", AttrValue::Bool(true));

        if sample_period > 0 {
            // Sample period in nanoseconds
            metadata.exif.set("WAV:SamplePeriod", AttrValue::UInt(sample_period));
        }

        if midi_unity_note < 128 {
            metadata.exif.set("WAV:MIDIUnityNote", AttrValue::UInt(midi_unity_note));
        }

        if num_loops > 0 {
            metadata.exif.set("WAV:NumLoops", AttrValue::UInt(num_loops));
        }

        // Skip loop data and remaining
        let remaining = size.saturating_sub(36);
        if remaining > 0 {
            reader.seek(SeekFrom::Current(remaining as i64))?;
        }

        Ok(())
    }

    /// Parse XMP chunk.
    fn parse_xmp(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if size > 10 * 1024 * 1024 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(());
        }

        let mut xmp_data = vec![0u8; size as usize];
        reader.read_exact(&mut xmp_data)?;

        if let Ok(xmp_str) = std::str::from_utf8(&xmp_data) {
            if let Ok(xmp_attrs) = exiftool_xmp::XmpParser::parse(xmp_str) {
                for (key, value) in xmp_attrs.iter() {
                    metadata.exif.set(format!("XMP:{}", key), value.clone());
                }
            }
            metadata.xmp = Some(xmp_str.to_string());
        }

        Ok(())
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_wav_header(sample_rate: u32, channels: u16, bits: u16) -> Vec<u8> {
        let mut data = Vec::new();

        // RIFF header
        data.extend_from_slice(b"RIFF");
        let size_pos = data.len();
        data.extend_from_slice(&0u32.to_le_bytes()); // placeholder
        data.extend_from_slice(b"WAVE");

        // fmt chunk
        data.extend_from_slice(b"fmt ");
        data.extend_from_slice(&16u32.to_le_bytes()); // chunk size
        data.extend_from_slice(&1u16.to_le_bytes()); // audio format (PCM)
        data.extend_from_slice(&channels.to_le_bytes());
        data.extend_from_slice(&sample_rate.to_le_bytes());
        let byte_rate = sample_rate * channels as u32 * bits as u32 / 8;
        data.extend_from_slice(&byte_rate.to_le_bytes());
        let block_align = channels * bits / 8;
        data.extend_from_slice(&block_align.to_le_bytes());
        data.extend_from_slice(&bits.to_le_bytes());

        // data chunk (empty)
        data.extend_from_slice(b"data");
        data.extend_from_slice(&0u32.to_le_bytes());

        // Update RIFF size
        let riff_size = (data.len() - 8) as u32;
        data[size_pos..size_pos + 4].copy_from_slice(&riff_size.to_le_bytes());

        data
    }

    #[test]
    fn test_can_parse() {
        let parser = WavParser;
        let data = make_wav_header(44100, 2, 16);
        assert!(parser.can_parse(&data[..12]));
    }

    #[test]
    fn test_cannot_parse_avi() {
        let parser = WavParser;
        let mut header = vec![0u8; 12];
        header[0..4].copy_from_slice(b"RIFF");
        header[8..12].copy_from_slice(b"AVI ");
        assert!(!parser.can_parse(&header));
    }

    #[test]
    fn test_parse_basic() {
        let parser = WavParser;
        let data = make_wav_header(44100, 2, 16);
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("File:FileType"), Some("WAV"));
        assert_eq!(meta.exif.get_str("WAV:AudioFormat"), Some("PCM"));
        assert_eq!(meta.exif.get_u32("WAV:SampleRate"), Some(44100));
        assert_eq!(meta.exif.get_u32("WAV:NumChannels"), Some(2));
        assert_eq!(meta.exif.get_u32("WAV:BitsPerSample"), Some(16));
    }

    #[test]
    fn test_parse_mono_8bit() {
        let parser = WavParser;
        let data = make_wav_header(22050, 1, 8);
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_u32("WAV:SampleRate"), Some(22050));
        assert_eq!(meta.exif.get_u32("WAV:NumChannels"), Some(1));
        assert_eq!(meta.exif.get_u32("WAV:BitsPerSample"), Some(8));
    }

    #[test]
    fn test_parse_with_info() {
        let parser = WavParser;
        let mut data = make_wav_header(44100, 2, 16);

        // Remove data chunk (last 8 bytes)
        data.truncate(data.len() - 8);

        // Add LIST INFO
        data.extend_from_slice(b"LIST");
        let list_size_pos = data.len();
        data.extend_from_slice(&0u32.to_le_bytes());
        data.extend_from_slice(b"INFO");

        // INAM (title)
        data.extend_from_slice(b"INAM");
        data.extend_from_slice(&10u32.to_le_bytes());
        data.extend_from_slice(b"Test Audio");

        // Update LIST size
        let list_size = data.len() - list_size_pos - 4;
        data[list_size_pos..list_size_pos + 4].copy_from_slice(&(list_size as u32).to_le_bytes());

        // Update RIFF size
        let riff_size = (data.len() - 8) as u32;
        data[4..8].copy_from_slice(&riff_size.to_le_bytes());

        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("RIFF:Title"), Some("Test Audio"));
    }
}

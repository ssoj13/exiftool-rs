//! DSF (DSD Stream File) format parser.
//!
//! DSF is a container format for DSD (Direct Stream Digital) audio.
//! Used for high-resolution audio, originally from SACD.
//!
//! # Structure
//!
//! - DSD chunk: "DSD " magic, file size, metadata offset
//! - fmt chunk: format info (sample rate, channels, bits)
//! - data chunk: audio data
//! - metadata chunk: optional ID3v2 tags

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// DSF format parser.
pub struct DsfParser;

impl FormatParser for DsfParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 4 {
            return false;
        }
        // "DSD " magic
        &header[0..4] == b"DSD "
    }

    fn format_name(&self) -> &'static str {
        "DSF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["dsf"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("DSF");
        meta.exif.set("File:FileType", AttrValue::Str("DSF".to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("audio/x-dsf".to_string()));
        meta.exif.set("Audio:Codec", AttrValue::Str("DSD".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // Read DSD chunk (28 bytes)
        let mut dsd_chunk = [0u8; 28];
        reader.read_exact(&mut dsd_chunk)?;

        // Verify magic
        if &dsd_chunk[0..4] != b"DSD " {
            return Ok(meta);
        }

        // Chunk size (offset 4, 8 bytes LE)
        let dsd_chunk_size = u64::from_le_bytes([
            dsd_chunk[4], dsd_chunk[5], dsd_chunk[6], dsd_chunk[7],
            dsd_chunk[8], dsd_chunk[9], dsd_chunk[10], dsd_chunk[11],
        ]);
        meta.exif.set("DSF:DSDChunkSize", AttrValue::UInt64(dsd_chunk_size));

        // Total file size (offset 12, 8 bytes LE)
        let total_file_size = u64::from_le_bytes([
            dsd_chunk[12], dsd_chunk[13], dsd_chunk[14], dsd_chunk[15],
            dsd_chunk[16], dsd_chunk[17], dsd_chunk[18], dsd_chunk[19],
        ]);
        meta.exif.set("File:FileSize", AttrValue::UInt64(total_file_size));

        // Metadata offset (offset 20, 8 bytes LE)
        let metadata_offset = u64::from_le_bytes([
            dsd_chunk[20], dsd_chunk[21], dsd_chunk[22], dsd_chunk[23],
            dsd_chunk[24], dsd_chunk[25], dsd_chunk[26], dsd_chunk[27],
        ]);
        if metadata_offset > 0 {
            meta.exif.set("DSF:MetadataOffset", AttrValue::UInt64(metadata_offset));
        }

        // Read fmt chunk
        let mut fmt_header = [0u8; 52];
        if reader.read_exact(&mut fmt_header).is_err() {
            return Ok(meta);
        }

        // Verify fmt magic
        if &fmt_header[0..4] != b"fmt " {
            return Ok(meta);
        }

        // Format version (offset 12, 4 bytes LE)
        let format_version = u32::from_le_bytes([fmt_header[12], fmt_header[13], fmt_header[14], fmt_header[15]]);
        meta.exif.set("DSF:FormatVersion", AttrValue::UInt(format_version));

        // Format ID (offset 16, 4 bytes LE) - 0 = DSD raw
        let format_id = u32::from_le_bytes([fmt_header[16], fmt_header[17], fmt_header[18], fmt_header[19]]);
        let format_name = if format_id == 0 { "DSD Raw" } else { "Unknown" };
        meta.exif.set("DSF:FormatID", AttrValue::Str(format_name.to_string()));

        // Channel type (offset 20, 4 bytes LE)
        let channel_type = u32::from_le_bytes([fmt_header[20], fmt_header[21], fmt_header[22], fmt_header[23]]);
        let channel_desc = match channel_type {
            1 => "Mono",
            2 => "Stereo",
            3 => "3 Channels",
            4 => "Quad",
            5 => "4 Channels",
            6 => "5 Channels",
            7 => "5.1 Surround",
            _ => "Unknown",
        };
        meta.exif.set("Audio:ChannelMode", AttrValue::Str(channel_desc.to_string()));

        // Channel count (offset 24, 4 bytes LE)
        let channels = u32::from_le_bytes([fmt_header[24], fmt_header[25], fmt_header[26], fmt_header[27]]);
        meta.exif.set("Audio:Channels", AttrValue::UInt(channels));

        // Sample rate (offset 28, 4 bytes LE) - DSD sample rate (2.8224 MHz for DSD64)
        let sample_rate = u32::from_le_bytes([fmt_header[28], fmt_header[29], fmt_header[30], fmt_header[31]]);
        meta.exif.set("Audio:SampleRate", AttrValue::UInt(sample_rate));

        // DSD rate description
        let dsd_rate = match sample_rate {
            2822400 => "DSD64 (1x)",
            5644800 => "DSD128 (2x)",
            11289600 => "DSD256 (4x)",
            22579200 => "DSD512 (8x)",
            _ => "Unknown DSD rate",
        };
        meta.exif.set("DSF:DSDRate", AttrValue::Str(dsd_rate.to_string()));

        // Bits per sample (offset 32, 4 bytes LE) - always 1 for DSD
        let bits_per_sample = u32::from_le_bytes([fmt_header[32], fmt_header[33], fmt_header[34], fmt_header[35]]);
        meta.exif.set("Audio:BitsPerSample", AttrValue::UInt(bits_per_sample));

        // Sample count (offset 36, 8 bytes LE)
        let sample_count = u64::from_le_bytes([
            fmt_header[36], fmt_header[37], fmt_header[38], fmt_header[39],
            fmt_header[40], fmt_header[41], fmt_header[42], fmt_header[43],
        ]);
        meta.exif.set("DSF:SampleCount", AttrValue::UInt64(sample_count));

        // Calculate duration
        if sample_rate > 0 {
            let duration = sample_count as f64 / sample_rate as f64;
            meta.exif.set("Audio:Duration", AttrValue::Double(duration));
            
            let mins = (duration / 60.0) as u32;
            let secs = (duration % 60.0) as u32;
            meta.exif.set("Audio:DurationFormatted", 
                AttrValue::Str(format!("{}:{:02}", mins, secs)));
        }

        // Block size per channel (offset 44, 4 bytes LE)
        let block_size = u32::from_le_bytes([fmt_header[44], fmt_header[45], fmt_header[46], fmt_header[47]]);
        meta.exif.set("DSF:BlockSize", AttrValue::UInt(block_size));

        Ok(meta)
    }
}

/// DFF (DSDIFF) format parser.
pub struct DffParser;

impl FormatParser for DffParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 12 {
            return false;
        }
        // "FRM8" + size + "DSD " magic
        &header[0..4] == b"FRM8" && &header[12..16] == b"DSD "
    }

    fn format_name(&self) -> &'static str {
        "DFF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["dff"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("DFF");
        meta.exif.set("File:FileType", AttrValue::Str("DFF".to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("audio/x-dff".to_string()));
        meta.exif.set("Audio:Codec", AttrValue::Str("DSD".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // Read FRM8 header (16 bytes)
        let mut frm8 = [0u8; 16];
        reader.read_exact(&mut frm8)?;

        // File size (offset 4, 8 bytes BE)
        let file_size = u64::from_be_bytes([
            frm8[4], frm8[5], frm8[6], frm8[7],
            frm8[8], frm8[9], frm8[10], frm8[11],
        ]);
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size + 12)); // +12 for header

        // Parse chunks
        let mut pos = 16u64;
        let end_pos = file_size + 12;

        while pos < end_pos {
            reader.seek(SeekFrom::Start(pos))?;
            
            let mut chunk_header = [0u8; 12];
            if reader.read_exact(&mut chunk_header).is_err() {
                break;
            }

            let chunk_id = &chunk_header[0..4];
            let chunk_size = u64::from_be_bytes([
                chunk_header[4], chunk_header[5], chunk_header[6], chunk_header[7],
                chunk_header[8], chunk_header[9], chunk_header[10], chunk_header[11],
            ]);

            match chunk_id {
                b"FVER" => {
                    // Format version
                    let mut ver = [0u8; 4];
                    if reader.read_exact(&mut ver).is_ok() {
                        let version = u32::from_be_bytes(ver);
                        meta.exif.set("DFF:FormatVersion", AttrValue::UInt(version));
                    }
                }
                b"PROP" => {
                    // Property chunk - contains SND chunk with audio info
                    parse_dff_prop(reader, chunk_size, &mut meta)?;
                }
                _ => {}
            }

            // Move to next chunk (aligned to 2 bytes)
            pos += 12 + chunk_size;
            if chunk_size % 2 != 0 {
                pos += 1;
            }
        }

        Ok(meta)
    }
}

/// Parse DFF PROP chunk.
fn parse_dff_prop(reader: &mut dyn ReadSeek, size: u64, meta: &mut Metadata) -> Result<()> {
    let start = reader.stream_position()?;
    
    // Property type (4 bytes) - should be "SND "
    let mut prop_type = [0u8; 4];
    reader.read_exact(&mut prop_type)?;
    
    if &prop_type != b"SND " {
        return Ok(());
    }

    let end = start + size;
    let mut pos = start + 4;

    while pos < end {
        reader.seek(SeekFrom::Start(pos))?;
        
        let mut chunk_header = [0u8; 12];
        if reader.read_exact(&mut chunk_header).is_err() {
            break;
        }

        let chunk_id = &chunk_header[0..4];
        let chunk_size = u64::from_be_bytes([
            chunk_header[4], chunk_header[5], chunk_header[6], chunk_header[7],
            chunk_header[8], chunk_header[9], chunk_header[10], chunk_header[11],
        ]);

        match chunk_id {
            b"FS  " => {
                // Sample rate
                let mut sr = [0u8; 4];
                if reader.read_exact(&mut sr).is_ok() {
                    let sample_rate = u32::from_be_bytes(sr);
                    meta.exif.set("Audio:SampleRate", AttrValue::UInt(sample_rate));
                    
                    let dsd_rate = match sample_rate {
                        2822400 => "DSD64 (1x)",
                        5644800 => "DSD128 (2x)",
                        11289600 => "DSD256 (4x)",
                        22579200 => "DSD512 (8x)",
                        _ => "Unknown DSD rate",
                    };
                    meta.exif.set("DFF:DSDRate", AttrValue::Str(dsd_rate.to_string()));
                }
            }
            b"CHNL" => {
                // Channel info
                let mut chnl = [0u8; 2];
                if reader.read_exact(&mut chnl).is_ok() {
                    let channels = u16::from_be_bytes(chnl);
                    meta.exif.set("Audio:Channels", AttrValue::UInt(channels as u32));
                    
                    let channel_mode = match channels {
                        1 => "Mono",
                        2 => "Stereo",
                        6 => "5.1 Surround",
                        _ => "Multi-channel",
                    };
                    meta.exif.set("Audio:ChannelMode", AttrValue::Str(channel_mode.to_string()));
                }
            }
            b"CMPR" => {
                // Compression type
                let mut cmpr = [0u8; 4];
                if reader.read_exact(&mut cmpr).is_ok() {
                    let compression = String::from_utf8_lossy(&cmpr).trim().to_string();
                    meta.exif.set("DFF:Compression", AttrValue::Str(compression));
                }
            }
            _ => {}
        }

        pos += 12 + chunk_size;
        if chunk_size % 2 != 0 {
            pos += 1;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_dsf_header() -> Vec<u8> {
        let mut data = vec![0u8; 1024];
        // DSD chunk
        data[0..4].copy_from_slice(b"DSD ");
        data[4..12].copy_from_slice(&28u64.to_le_bytes()); // chunk size
        data[12..20].copy_from_slice(&1000u64.to_le_bytes()); // file size
        data[20..28].copy_from_slice(&0u64.to_le_bytes()); // metadata offset
        
        // fmt chunk
        data[28..32].copy_from_slice(b"fmt ");
        data[32..40].copy_from_slice(&52u64.to_le_bytes()); // chunk size
        data[40..44].copy_from_slice(&1u32.to_le_bytes()); // format version
        data[44..48].copy_from_slice(&0u32.to_le_bytes()); // format ID (DSD raw)
        data[48..52].copy_from_slice(&2u32.to_le_bytes()); // channel type (stereo)
        data[52..56].copy_from_slice(&2u32.to_le_bytes()); // channel count
        data[56..60].copy_from_slice(&2822400u32.to_le_bytes()); // sample rate (DSD64)
        data[60..64].copy_from_slice(&1u32.to_le_bytes()); // bits per sample
        data[64..72].copy_from_slice(&28224000u64.to_le_bytes()); // sample count (10 sec)
        data[72..76].copy_from_slice(&4096u32.to_le_bytes()); // block size
        data
    }

    fn make_dff_header() -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // FRM8 header
        data[0..4].copy_from_slice(b"FRM8");
        data[4..12].copy_from_slice(&500u64.to_be_bytes()); // size
        data[12..16].copy_from_slice(b"DSD ");
        data
    }

    #[test]
    fn test_dsf_can_parse() {
        let parser = DsfParser;
        let data = make_dsf_header();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_dsf_cannot_parse_invalid() {
        let parser = DsfParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"RIFF"));
    }

    #[test]
    fn test_dsf_parse_basic() {
        let parser = DsfParser;
        let data = make_dsf_header();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "DSF");
        assert_eq!(meta.exif.get_u32("Audio:SampleRate"), Some(2822400));
        assert_eq!(meta.exif.get_u32("Audio:Channels"), Some(2));
        assert_eq!(meta.exif.get_str("Audio:ChannelMode"), Some("Stereo"));
        assert_eq!(meta.exif.get_str("DSF:DSDRate"), Some("DSD64 (1x)"));
    }

    #[test]
    fn test_dsf_format_info() {
        let parser = DsfParser;
        assert_eq!(parser.format_name(), "DSF");
        assert!(parser.extensions().contains(&"dsf"));
    }

    #[test]
    fn test_dff_can_parse() {
        let parser = DffParser;
        let data = make_dff_header();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_dff_format_info() {
        let parser = DffParser;
        assert_eq!(parser.format_name(), "DFF");
        assert!(parser.extensions().contains(&"dff"));
    }
}

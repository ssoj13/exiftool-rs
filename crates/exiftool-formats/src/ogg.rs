//! OGG container format parser.
//!
//! OGG is a container format that can hold Vorbis, Opus, FLAC, Theora, etc.
//! This parser handles OGG Vorbis audio files.
//!
//! # Structure
//!
//! - OGG pages with "OggS" magic
//! - Each page contains segments
//! - First page contains codec identification
//! - Second page contains Vorbis comments (metadata)

use crate::{FormatParser, Metadata, ReadSeek, Result, Error};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// OGG format parser.
pub struct OggParser;

impl FormatParser for OggParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 4 {
            return false;
        }
        // "OggS" magic
        &header[0..4] == b"OggS"
    }

    fn format_name(&self) -> &'static str {
        "OGG"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["ogg", "oga", "ogv", "ogx"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("OGG");
        
        reader.seek(SeekFrom::Start(0))?;

        // Read first OGG page to identify codec
        let first_page = read_ogg_page(reader)?;
        
        // Check codec type from first packet
        if first_page.len() >= 7 {
            if &first_page[1..7] == b"vorbis" {
                meta.format = "OGG";
                meta.set_file_type("OGG", "audio/ogg");
                meta.exif.set("Audio:Codec", AttrValue::Str("Vorbis".to_string()));
                
                // Parse Vorbis identification header
                if first_page.len() >= 30 {
                    parse_vorbis_id(&first_page, &mut meta);
                }
            } else if &first_page[0..8] == b"OpusHead" {
                meta.format = "OPUS";
                meta.set_file_type("OPUS", "audio/opus");
                meta.exif.set("Audio:Codec", AttrValue::Str("Opus".to_string()));
                
                // Parse Opus header
                if first_page.len() >= 19 {
                    parse_opus_header(&first_page, &mut meta);
                }
            } else if &first_page[0..5] == b"\x7fFLAC" {
                meta.format = "OGG";
                meta.set_file_type("OGG", "audio/ogg");
                meta.exif.set("Audio:Codec", AttrValue::Str("FLAC".to_string()));
            } else if &first_page[1..7] == b"theora" {
                meta.format = "OGV";
                meta.set_file_type("OGV", "video/ogg");
                meta.exif.set("Video:Codec", AttrValue::Str("Theora".to_string()));
            }
        }

        // Read second page for Vorbis comments
        if let Ok(comment_page) = read_ogg_page(reader) {
            // Vorbis comment header starts with 0x03 + "vorbis"
            if comment_page.len() > 7 && comment_page[0] == 0x03 && &comment_page[1..7] == b"vorbis" {
                parse_vorbis_comments(&comment_page[7..], &mut meta);
            }
            // Opus tags start with "OpusTags"
            else if comment_page.len() > 8 && &comment_page[0..8] == b"OpusTags" {
                parse_vorbis_comments(&comment_page[8..], &mut meta);
            }
        }

        // File size
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        Ok(meta)
    }
}

/// Read a single OGG page and return its data.
fn read_ogg_page(reader: &mut dyn ReadSeek) -> Result<Vec<u8>> {
    let mut header = [0u8; 27];
    reader.read_exact(&mut header)?;

    // Verify OggS magic
    if &header[0..4] != b"OggS" {
        return Err(Error::InvalidStructure("Not an OGG page".to_string()));
    }

    // Number of segments
    let num_segments = header[26] as usize;
    
    // Read segment table
    let mut segment_table = vec![0u8; num_segments];
    reader.read_exact(&mut segment_table)?;

    // Calculate total data size
    let data_size: usize = segment_table.iter().map(|&s| s as usize).sum();

    // Read page data
    let mut data = vec![0u8; data_size];
    reader.read_exact(&mut data)?;

    Ok(data)
}

/// Parse Vorbis identification header.
fn parse_vorbis_id(data: &[u8], meta: &mut Metadata) {
    if data.len() < 30 || data[0] != 0x01 {
        return;
    }

    // Vorbis version (should be 0)
    let _version = u32::from_le_bytes([data[7], data[8], data[9], data[10]]);
    
    // Audio channels
    let channels = data[11];
    meta.exif.set("Audio:Channels", AttrValue::UInt(channels as u32));
    
    let channel_mode = match channels {
        1 => "Mono",
        2 => "Stereo",
        _ => "Multi-channel",
    };
    meta.exif.set("Audio:ChannelMode", AttrValue::Str(channel_mode.to_string()));

    // Sample rate
    let sample_rate = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
    meta.exif.set("Audio:SampleRate", AttrValue::UInt(sample_rate));

    // Bitrates (may be -1 for VBR)
    let bitrate_max = i32::from_le_bytes([data[16], data[17], data[18], data[19]]);
    let bitrate_nom = i32::from_le_bytes([data[20], data[21], data[22], data[23]]);
    let bitrate_min = i32::from_le_bytes([data[24], data[25], data[26], data[27]]);

    if bitrate_nom > 0 {
        meta.exif.set("Audio:NominalBitrate", AttrValue::Int(bitrate_nom));
    }
    if bitrate_max > 0 {
        meta.exif.set("Audio:MaxBitrate", AttrValue::Int(bitrate_max));
    }
    if bitrate_min > 0 {
        meta.exif.set("Audio:MinBitrate", AttrValue::Int(bitrate_min));
    }
}

/// Parse Opus header.
fn parse_opus_header(data: &[u8], meta: &mut Metadata) {
    if data.len() < 19 {
        return;
    }

    // Version
    let version = data[8];
    meta.exif.set("Opus:Version", AttrValue::UInt(version as u32));

    // Channel count
    let channels = data[9];
    meta.exif.set("Audio:Channels", AttrValue::UInt(channels as u32));
    
    let channel_mode = match channels {
        1 => "Mono",
        2 => "Stereo",
        _ => "Multi-channel",
    };
    meta.exif.set("Audio:ChannelMode", AttrValue::Str(channel_mode.to_string()));

    // Pre-skip
    let pre_skip = u16::from_le_bytes([data[10], data[11]]);
    meta.exif.set("Opus:PreSkip", AttrValue::UInt(pre_skip as u32));

    // Original sample rate (informational, Opus always uses 48kHz internally)
    let orig_sample_rate = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
    meta.exif.set("Audio:SampleRate", AttrValue::UInt(48000)); // Opus native
    if orig_sample_rate > 0 {
        meta.exif.set("Audio:OriginalSampleRate", AttrValue::UInt(orig_sample_rate));
    }

    // Output gain
    let output_gain = i16::from_le_bytes([data[16], data[17]]);
    if output_gain != 0 {
        let gain_db = output_gain as f64 / 256.0;
        meta.exif.set("Opus:OutputGain", AttrValue::Double(gain_db));
    }
}

/// Parse Vorbis comments (also used by Opus, FLAC in OGG).
fn parse_vorbis_comments(data: &[u8], meta: &mut Metadata) {
    if data.len() < 8 {
        return;
    }

    let mut pos = 0;

    // Vendor string length
    if pos + 4 > data.len() {
        return;
    }
    let vendor_len = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
    pos += 4;

    // Vendor string
    if pos + vendor_len > data.len() {
        return;
    }
    let vendor = String::from_utf8_lossy(&data[pos..pos + vendor_len]).to_string();
    if !vendor.is_empty() {
        meta.exif.set("Audio:Encoder", AttrValue::Str(vendor));
    }
    pos += vendor_len;

    // Comment count
    if pos + 4 > data.len() {
        return;
    }
    let comment_count = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
    pos += 4;

    // Parse comments
    for _ in 0..comment_count.min(100) {
        if pos + 4 > data.len() {
            break;
        }
        let comment_len = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        pos += 4;

        if pos + comment_len > data.len() || comment_len > 65536 {
            break;
        }

        let comment = String::from_utf8_lossy(&data[pos..pos + comment_len]);
        pos += comment_len;

        // Parse KEY=VALUE format
        if let Some(eq_pos) = comment.find('=') {
            let key = comment[..eq_pos].to_uppercase();
            let value = &comment[eq_pos + 1..];

            let tag_name = match key.as_str() {
                "TITLE" => "Audio:Title",
                "ARTIST" => "Audio:Artist",
                "ALBUM" => "Audio:Album",
                "DATE" | "YEAR" => "Audio:Year",
                "TRACKNUMBER" | "TRACK" => "Audio:Track",
                "GENRE" => "Audio:Genre",
                "COMMENT" | "DESCRIPTION" => "Audio:Comment",
                "ALBUMARTIST" => "Audio:AlbumArtist",
                "COMPOSER" => "Audio:Composer",
                "PERFORMER" => "Audio:Performer",
                "COPYRIGHT" => "Audio:Copyright",
                "LICENSE" => "Audio:License",
                "ORGANIZATION" => "Audio:Organization",
                "DISCNUMBER" => "Audio:DiscNumber",
                "ISRC" => "Audio:ISRC",
                "ENCODER" => "Audio:EncoderSettings",
                _ => continue,
            };

            meta.exif.set(tag_name, AttrValue::Str(value.to_string()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_ogg_vorbis() -> Vec<u8> {
        let mut data = vec![0u8; 512];
        
        // First page - Vorbis ID header
        // OGG page header
        data[0..4].copy_from_slice(b"OggS");
        data[4] = 0; // version
        data[5] = 0x02; // BOS flag
        data[6..14].copy_from_slice(&0u64.to_le_bytes()); // granule
        data[14..18].copy_from_slice(&0u32.to_le_bytes()); // serial
        data[18..22].copy_from_slice(&0u32.to_le_bytes()); // page seq
        data[22..26].copy_from_slice(&0u32.to_le_bytes()); // CRC (ignored)
        data[26] = 1; // 1 segment
        data[27] = 30; // segment size
        
        // Vorbis ID packet
        data[28] = 0x01; // packet type
        data[29..35].copy_from_slice(b"vorbis");
        data[35..39].copy_from_slice(&0u32.to_le_bytes()); // version
        data[39] = 2; // channels
        data[40..44].copy_from_slice(&44100u32.to_le_bytes()); // sample rate
        data[44..48].copy_from_slice(&(-1i32).to_le_bytes()); // max bitrate
        data[48..52].copy_from_slice(&128000i32.to_le_bytes()); // nom bitrate
        data[52..56].copy_from_slice(&(-1i32).to_le_bytes()); // min bitrate

        // Second page - Vorbis comments
        let page2_start = 58;
        data[page2_start..page2_start + 4].copy_from_slice(b"OggS");
        data[page2_start + 4] = 0;
        data[page2_start + 5] = 0;
        data[page2_start + 26] = 1;
        data[page2_start + 27] = 40;
        
        let comment_start = page2_start + 28;
        data[comment_start] = 0x03;
        data[comment_start + 1..comment_start + 7].copy_from_slice(b"vorbis");
        // Vendor length = 4, vendor = "Test"
        data[comment_start + 7..comment_start + 11].copy_from_slice(&4u32.to_le_bytes());
        data[comment_start + 11..comment_start + 15].copy_from_slice(b"Test");
        // 1 comment
        data[comment_start + 15..comment_start + 19].copy_from_slice(&1u32.to_le_bytes());
        // Comment: "TITLE=Test Song" (15 chars)
        data[comment_start + 19..comment_start + 23].copy_from_slice(&15u32.to_le_bytes());
        data[comment_start + 23..comment_start + 38].copy_from_slice(b"TITLE=Test Song");

        data
    }

    #[test]
    fn test_can_parse() {
        let parser = OggParser;
        let data = make_ogg_vorbis();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = OggParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"RIFF"));
    }

    #[test]
    fn test_parse_vorbis() {
        let parser = OggParser;
        let data = make_ogg_vorbis();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "OGG");
        assert_eq!(meta.exif.get_str("Audio:Codec"), Some("Vorbis"));
        assert_eq!(meta.exif.get_u32("Audio:SampleRate"), Some(44100));
        assert_eq!(meta.exif.get_u32("Audio:Channels"), Some(2));
        assert_eq!(meta.exif.get_str("Audio:ChannelMode"), Some("Stereo"));
    }

    #[test]
    fn test_format_info() {
        let parser = OggParser;
        assert_eq!(parser.format_name(), "OGG");
        assert!(parser.extensions().contains(&"ogg"));
        assert!(parser.extensions().contains(&"oga"));
    }
}

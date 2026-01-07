//! APE (Monkey's Audio) format parser.
//!
//! APE is a lossless audio compression format.
//! Files start with "MAC " magic followed by version info.
//!
//! Also parses APEv2 tags which can appear at the end of the file.

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// APE format parser.
pub struct ApeParser;

impl FormatParser for ApeParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 4 {
            return false;
        }
        // "MAC " magic
        &header[0..4] == b"MAC "
    }

    fn format_name(&self) -> &'static str {
        "APE"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["ape"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("APE");
        meta.set_file_type("APE", "audio/x-ape");
        meta.exif.set("Audio:Codec", AttrValue::Str("Monkey's Audio".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // Read APE header
        let mut header = [0u8; 76];
        if reader.read_exact(&mut header).is_ok() {
            // Version (offset 4, 2 bytes LE)
            let version = u16::from_le_bytes([header[4], header[5]]);
            meta.exif.set("APE:Version", AttrValue::UInt(version as u32));
            
            // Format version string
            let version_str = format!("{}.{}", version / 1000, (version % 1000) / 10);
            meta.exif.set("APE:VersionString", AttrValue::Str(version_str));

            // For version >= 3980, header structure changed
            if version >= 3980 {
                // Compression type (offset 52)
                let compression = u16::from_le_bytes([header[52], header[53]]);
                let compression_name = match compression {
                    1000 => "Fast",
                    2000 => "Normal", 
                    3000 => "High",
                    4000 => "Extra High",
                    5000 => "Insane",
                    _ => "Unknown",
                };
                meta.exif.set("APE:CompressionType", AttrValue::Str(compression_name.to_string()));
                meta.exif.set("APE:CompressionLevel", AttrValue::UInt(compression as u32));

                // Format flags (offset 54)
                let format_flags = u16::from_le_bytes([header[54], header[55]]);
                meta.exif.set("APE:FormatFlags", AttrValue::UInt(format_flags as u32));

                // Blocks per frame (offset 56)
                let blocks_per_frame = u32::from_le_bytes([header[56], header[57], header[58], header[59]]);
                meta.exif.set("APE:BlocksPerFrame", AttrValue::UInt(blocks_per_frame));

                // Final frame blocks (offset 60)
                let final_frame_blocks = u32::from_le_bytes([header[60], header[61], header[62], header[63]]);
                meta.exif.set("APE:FinalFrameBlocks", AttrValue::UInt(final_frame_blocks));

                // Total frames (offset 64)
                let total_frames = u32::from_le_bytes([header[64], header[65], header[66], header[67]]);
                meta.exif.set("APE:TotalFrames", AttrValue::UInt(total_frames));

                // Bits per sample (offset 68)
                let bits_per_sample = u16::from_le_bytes([header[68], header[69]]);
                meta.exif.set("Audio:BitsPerSample", AttrValue::UInt(bits_per_sample as u32));

                // Channels (offset 70)
                let channels = u16::from_le_bytes([header[70], header[71]]);
                meta.exif.set("Audio:Channels", AttrValue::UInt(channels as u32));
                
                let channel_mode = match channels {
                    1 => "Mono",
                    2 => "Stereo",
                    _ => "Multi-channel",
                };
                meta.exif.set("Audio:ChannelMode", AttrValue::Str(channel_mode.to_string()));

                // Sample rate (offset 72)
                let sample_rate = u32::from_le_bytes([header[72], header[73], header[74], header[75]]);
                meta.exif.set("Audio:SampleRate", AttrValue::UInt(sample_rate));

                // Calculate duration
                if total_frames > 0 && sample_rate > 0 {
                    let total_blocks = if total_frames > 1 {
                        (total_frames - 1) as u64 * blocks_per_frame as u64 + final_frame_blocks as u64
                    } else {
                        final_frame_blocks as u64
                    };
                    let duration = total_blocks as f64 / sample_rate as f64;
                    meta.exif.set("Audio:Duration", AttrValue::Double(duration));
                    
                    let mins = (duration / 60.0) as u32;
                    let secs = (duration % 60.0) as u32;
                    meta.exif.set("Audio:DurationFormatted", 
                        AttrValue::Str(format!("{}:{:02}", mins, secs)));
                }
            }
        }

        // Try to read APEv2 tag at end of file
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        // APEv2 tag footer is 32 bytes before end (or before ID3v1 if present)
        if file_size > 32 {
            // Check for ID3v1 tag
            let mut check_pos = file_size - 32;
            reader.seek(SeekFrom::Start(file_size - 128))?;
            let mut id3_check = [0u8; 3];
            if reader.read_exact(&mut id3_check).is_ok() && &id3_check == b"TAG" {
                check_pos = file_size - 128 - 32;
            }

            // Read APEv2 footer
            reader.seek(SeekFrom::Start(check_pos))?;
            let mut footer = [0u8; 32];
            if reader.read_exact(&mut footer).is_ok() && &footer[0..8] == b"APETAGEX" {
                let tag_size = u32::from_le_bytes([footer[12], footer[13], footer[14], footer[15]]);
                let item_count = u32::from_le_bytes([footer[16], footer[17], footer[18], footer[19]]);

                // Read tag data
                if tag_size > 0 && tag_size < 1_000_000 {
                    let tag_start = check_pos - tag_size as u64 + 32;
                    reader.seek(SeekFrom::Start(tag_start))?;
                    let mut tag_data = vec![0u8; tag_size as usize];
                    if reader.read_exact(&mut tag_data).is_ok() {
                        parse_apev2_items(&tag_data, item_count, &mut meta);
                    }
                }
            }
        }

        Ok(meta)
    }
}

/// Parse APEv2 tag items.
fn parse_apev2_items(data: &[u8], item_count: u32, meta: &mut Metadata) {
    let mut pos = 0;

    for _ in 0..item_count.min(100) {
        if pos + 8 > data.len() {
            break;
        }

        // Value size (4 bytes LE)
        let value_size = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        pos += 4;

        // Flags (4 bytes LE)
        let _flags = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;

        // Key (null-terminated string)
        let key_end = data[pos..].iter().position(|&b| b == 0);
        if key_end.is_none() {
            break;
        }
        let key_end = key_end.unwrap();
        let key = String::from_utf8_lossy(&data[pos..pos + key_end]).to_uppercase();
        pos += key_end + 1;

        // Value
        if pos + value_size > data.len() || value_size > 65536 {
            break;
        }
        let value = String::from_utf8_lossy(&data[pos..pos + value_size]).to_string();
        pos += value_size;

        // Map to standard tags
        let tag_name = match key.as_str() {
            "TITLE" => "Audio:Title",
            "ARTIST" => "Audio:Artist",
            "ALBUM" => "Audio:Album",
            "YEAR" => "Audio:Year",
            "TRACK" => "Audio:Track",
            "GENRE" => "Audio:Genre",
            "COMMENT" => "Audio:Comment",
            "ALBUMARTIST" | "ALBUM ARTIST" => "Audio:AlbumArtist",
            "COMPOSER" => "Audio:Composer",
            "DISCNUMBER" | "DISC" => "Audio:DiscNumber",
            "COPYRIGHT" => "Audio:Copyright",
            "ENCODER" => "Audio:Encoder",
            _ => continue,
        };

        meta.exif.set(tag_name, AttrValue::Str(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_ape_header() -> Vec<u8> {
        let mut data = vec![0u8; 1024];
        // Magic "MAC "
        data[0..4].copy_from_slice(b"MAC ");
        // Version 3990
        data[4..6].copy_from_slice(&3990u16.to_le_bytes());
        // Padding to offset 52
        // Compression type (Normal = 2000)
        data[52..54].copy_from_slice(&2000u16.to_le_bytes());
        // Format flags
        data[54..56].copy_from_slice(&0u16.to_le_bytes());
        // Blocks per frame
        data[56..60].copy_from_slice(&73728u32.to_le_bytes());
        // Final frame blocks
        data[60..64].copy_from_slice(&36864u32.to_le_bytes());
        // Total frames
        data[64..68].copy_from_slice(&100u32.to_le_bytes());
        // Bits per sample
        data[68..70].copy_from_slice(&16u16.to_le_bytes());
        // Channels
        data[70..72].copy_from_slice(&2u16.to_le_bytes());
        // Sample rate
        data[72..76].copy_from_slice(&44100u32.to_le_bytes());
        data
    }

    #[test]
    fn test_can_parse() {
        let parser = ApeParser;
        let data = make_ape_header();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = ApeParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"OggS"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = ApeParser;
        let data = make_ape_header();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "APE");
        assert_eq!(meta.exif.get_u32("APE:Version"), Some(3990));
        assert_eq!(meta.exif.get_u32("Audio:SampleRate"), Some(44100));
        assert_eq!(meta.exif.get_u32("Audio:Channels"), Some(2));
        assert_eq!(meta.exif.get_str("Audio:ChannelMode"), Some("Stereo"));
        assert_eq!(meta.exif.get_str("APE:CompressionType"), Some("Normal"));
    }

    #[test]
    fn test_format_info() {
        let parser = ApeParser;
        assert_eq!(parser.format_name(), "APE");
        assert!(parser.extensions().contains(&"ape"));
    }
}

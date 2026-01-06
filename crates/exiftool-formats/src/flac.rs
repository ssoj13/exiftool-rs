//! FLAC format parser.
//!
//! FLAC (Free Lossless Audio Codec) is a lossless audio format.
//! Structure:
//! - 4-byte magic: "fLaC"
//! - Metadata blocks (STREAMINFO required first)
//! - Audio frames
//!
//! Metadata block types:
//! - 0: STREAMINFO (required, sample rate, channels, bits, etc.)
//! - 1: PADDING
//! - 2: APPLICATION
//! - 3: SEEKTABLE
//! - 4: VORBIS_COMMENT (Vorbis comment block - main metadata)
//! - 5: CUESHEET
//! - 6: PICTURE (album art)
//!
//! Vorbis comments use "FIELD=value" format with UTF-8 encoding.

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// FLAC format parser.
pub struct FlacParser;

impl FormatParser for FlacParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 4 && &header[0..4] == b"fLaC"
    }

    fn format_name(&self) -> &'static str {
        "FLAC"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["flac", "fla"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("FLAC");

        // Read and verify magic
        reader.seek(SeekFrom::Start(0))?;
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;

        if &magic != b"fLaC" {
            return Err(crate::Error::InvalidStructure("Invalid FLAC magic".into()));
        }

        metadata.exif.set("FileType", AttrValue::Str("FLAC".to_string()));

        // Parse metadata blocks
        let mut is_last = false;
        while !is_last {
            let mut block_header = [0u8; 4];
            reader.read_exact(&mut block_header)?;

            is_last = block_header[0] & 0x80 != 0;
            let block_type = block_header[0] & 0x7F;
            let block_size = ((block_header[1] as u32) << 16)
                | ((block_header[2] as u32) << 8)
                | (block_header[3] as u32);

            if block_size > 16 * 1024 * 1024 {
                break; // Sanity check
            }

            match block_type {
                0 => {
                    // STREAMINFO
                    self.parse_streaminfo(reader, &mut metadata)?;
                }
                4 => {
                    // VORBIS_COMMENT
                    self.parse_vorbis_comment(reader, block_size, &mut metadata)?;
                }
                6 => {
                    // PICTURE
                    self.parse_picture(reader, block_size, &mut metadata)?;
                }
                _ => {
                    // Skip other blocks
                    reader.seek(SeekFrom::Current(block_size as i64))?;
                }
            }
        }

        Ok(metadata)
    }
}

impl FlacParser {
    /// Parse STREAMINFO block (34 bytes).
    fn parse_streaminfo(&self, reader: &mut dyn ReadSeek, metadata: &mut Metadata) -> Result<()> {
        let mut data = [0u8; 34];
        reader.read_exact(&mut data)?;

        // Minimum block size (2 bytes)
        let min_block_size = u16::from_be_bytes([data[0], data[1]]);
        // Maximum block size (2 bytes)
        let max_block_size = u16::from_be_bytes([data[2], data[3]]);

        // Minimum frame size (3 bytes)
        let min_frame_size = ((data[4] as u32) << 16) | ((data[5] as u32) << 8) | (data[6] as u32);
        // Maximum frame size (3 bytes)
        let max_frame_size = ((data[7] as u32) << 16) | ((data[8] as u32) << 8) | (data[9] as u32);

        // Sample rate (20 bits), channels (3 bits), bits per sample (5 bits), total samples (36 bits)
        // Bytes 10-17 contain these packed values
        let sample_rate = ((data[10] as u32) << 12) | ((data[11] as u32) << 4) | ((data[12] as u32) >> 4);
        let channels = ((data[12] >> 1) & 0x07) + 1;
        let bits_per_sample = (((data[12] & 0x01) << 4) | ((data[13] >> 4) & 0x0F)) + 1;
        let total_samples = (((data[13] & 0x0F) as u64) << 32)
            | ((data[14] as u64) << 24)
            | ((data[15] as u64) << 16)
            | ((data[16] as u64) << 8)
            | (data[17] as u64);

        // MD5 signature (16 bytes) at data[18..34]
        let md5: String = data[18..34].iter().map(|b| format!("{:02x}", b)).collect();

        metadata.exif.set("SampleRate", AttrValue::UInt(sample_rate));
        metadata.exif.set("AudioChannels", AttrValue::UInt(channels as u32));
        metadata.exif.set("BitsPerSample", AttrValue::UInt(bits_per_sample as u32));

        if total_samples > 0 {
            metadata.exif.set("TotalSamples", AttrValue::UInt64(total_samples));

            // Calculate duration
            if sample_rate > 0 {
                let duration_secs = total_samples as f64 / sample_rate as f64;
                metadata.exif.set("Duration", AttrValue::Str(format_duration(duration_secs)));
                metadata.exif.set("DurationSeconds", AttrValue::Double(duration_secs));
            }
        }

        if min_block_size > 0 {
            metadata.exif.set("MinBlockSize", AttrValue::UInt(min_block_size as u32));
        }
        if max_block_size > 0 {
            metadata.exif.set("MaxBlockSize", AttrValue::UInt(max_block_size as u32));
        }
        if min_frame_size > 0 {
            metadata.exif.set("MinFrameSize", AttrValue::UInt(min_frame_size));
        }
        if max_frame_size > 0 {
            metadata.exif.set("MaxFrameSize", AttrValue::UInt(max_frame_size));
        }

        // Only store non-zero MD5
        if !md5.chars().all(|c| c == '0') {
            metadata.exif.set("AudioMD5", AttrValue::Str(md5));
        }

        Ok(())
    }

    /// Parse VORBIS_COMMENT block.
    fn parse_vorbis_comment(
        &self,
        reader: &mut dyn ReadSeek,
        block_size: u32,
        metadata: &mut Metadata,
    ) -> Result<()> {
        let start_pos = reader.stream_position()?;
        let end_pos = start_pos + block_size as u64;

        // Vendor string length (4 bytes, little-endian)
        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf)?;
        let vendor_len = u32::from_le_bytes(len_buf) as usize;

        if vendor_len > 0 && vendor_len < 1024 {
            let mut vendor = vec![0u8; vendor_len];
            reader.read_exact(&mut vendor)?;
            if let Ok(vendor_str) = String::from_utf8(vendor) {
                metadata.exif.set("Encoder", AttrValue::Str(vendor_str));
            }
        } else {
            reader.seek(SeekFrom::Current(vendor_len as i64))?;
        }

        // Number of comments (4 bytes, little-endian)
        reader.read_exact(&mut len_buf)?;
        let comment_count = u32::from_le_bytes(len_buf);

        if comment_count > 1000 {
            return Ok(()); // Sanity check
        }

        for _ in 0..comment_count {
            if reader.stream_position()? >= end_pos {
                break;
            }

            // Comment length (4 bytes, little-endian)
            reader.read_exact(&mut len_buf)?;
            let comment_len = u32::from_le_bytes(len_buf) as usize;

            if comment_len == 0 || comment_len > 1024 * 1024 {
                continue;
            }

            let mut comment = vec![0u8; comment_len];
            reader.read_exact(&mut comment)?;

            if let Ok(comment_str) = String::from_utf8(comment) {
                if let Some(eq_pos) = comment_str.find('=') {
                    let field = &comment_str[..eq_pos];
                    let value = &comment_str[eq_pos + 1..];

                    // Map Vorbis comment fields to metadata
                    let tag_name = match field.to_uppercase().as_str() {
                        "TITLE" => "Title",
                        "ARTIST" => "Artist",
                        "ALBUM" => "Album",
                        "DATE" | "YEAR" => "Year",
                        "TRACKNUMBER" | "TRACK" => "Track",
                        "DISCNUMBER" | "DISC" => "DiscNumber",
                        "GENRE" => "Genre",
                        "COMMENT" | "DESCRIPTION" => "Comment",
                        "ALBUMARTIST" | "ALBUM ARTIST" => "AlbumArtist",
                        "COMPOSER" => "Composer",
                        "PERFORMER" => "Performer",
                        "COPYRIGHT" => "Copyright",
                        "LICENSE" => "License",
                        "ORGANIZATION" | "LABEL" => "Publisher",
                        "ISRC" => "ISRC",
                        "LYRICS" => "Lyrics",
                        "BPM" | "TEMPO" => "BPM",
                        "REPLAYGAIN_TRACK_GAIN" => "ReplayGainTrack",
                        "REPLAYGAIN_ALBUM_GAIN" => "ReplayGainAlbum",
                        "REPLAYGAIN_TRACK_PEAK" => "ReplayGainTrackPeak",
                        "REPLAYGAIN_ALBUM_PEAK" => "ReplayGainAlbumPeak",
                        "ENCODER" | "ENCODED-BY" => "EncodedBy",
                        _ => {
                            // Store custom fields with "Vorbis:" prefix
                            let custom_key = format!("Vorbis:{}", field);
                            metadata.exif.set(&custom_key, AttrValue::Str(value.to_string()));
                            continue;
                        }
                    };

                    metadata.exif.set(tag_name, AttrValue::Str(value.to_string()));
                }
            }
        }

        // Seek to end of block in case we didn't read everything
        reader.seek(SeekFrom::Start(end_pos))?;

        Ok(())
    }

    /// Parse PICTURE block.
    fn parse_picture(
        &self,
        reader: &mut dyn ReadSeek,
        block_size: u32,
        metadata: &mut Metadata,
    ) -> Result<()> {
        let start_pos = reader.stream_position()?;
        let end_pos = start_pos + block_size as u64;

        // Picture type (4 bytes)
        let mut type_buf = [0u8; 4];
        reader.read_exact(&mut type_buf)?;
        let picture_type = u32::from_be_bytes(type_buf);

        let type_name = match picture_type {
            0 => "Other",
            1 => "32x32 Icon",
            2 => "Other Icon",
            3 => "Front Cover",
            4 => "Back Cover",
            5 => "Leaflet",
            6 => "Media",
            7 => "Lead Artist",
            8 => "Artist",
            9 => "Conductor",
            10 => "Band",
            11 => "Composer",
            12 => "Lyricist",
            13 => "Recording Location",
            14 => "During Recording",
            15 => "During Performance",
            16 => "Movie Capture",
            17 => "Bright Fish",
            18 => "Illustration",
            19 => "Band Logo",
            20 => "Publisher Logo",
            _ => "Unknown",
        };

        metadata.exif.set("PictureType", AttrValue::Str(type_name.to_string()));

        // MIME type length (4 bytes)
        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf)?;
        let mime_len = u32::from_be_bytes(len_buf) as usize;

        if mime_len > 0 && mime_len < 256 {
            let mut mime = vec![0u8; mime_len];
            reader.read_exact(&mut mime)?;
            if let Ok(mime_str) = String::from_utf8(mime) {
                metadata.exif.set("PictureMime", AttrValue::Str(mime_str));
            }
        } else {
            reader.seek(SeekFrom::Current(mime_len as i64))?;
        }

        // Description length (4 bytes)
        reader.read_exact(&mut len_buf)?;
        let desc_len = u32::from_be_bytes(len_buf) as usize;

        if desc_len > 0 && desc_len < 4096 {
            let mut desc = vec![0u8; desc_len];
            reader.read_exact(&mut desc)?;
            if let Ok(desc_str) = String::from_utf8(desc) {
                if !desc_str.is_empty() {
                    metadata.exif.set("PictureDescription", AttrValue::Str(desc_str));
                }
            }
        } else {
            reader.seek(SeekFrom::Current(desc_len as i64))?;
        }

        // Width (4 bytes)
        reader.read_exact(&mut len_buf)?;
        let width = u32::from_be_bytes(len_buf);

        // Height (4 bytes)
        reader.read_exact(&mut len_buf)?;
        let height = u32::from_be_bytes(len_buf);

        // Color depth (4 bytes)
        reader.read_exact(&mut len_buf)?;
        let color_depth = u32::from_be_bytes(len_buf);

        // Number of colors (4 bytes) - 0 for non-indexed
        reader.read_exact(&mut len_buf)?;
        let _num_colors = u32::from_be_bytes(len_buf);

        // Picture data length (4 bytes)
        reader.read_exact(&mut len_buf)?;
        let data_len = u32::from_be_bytes(len_buf);

        if width > 0 && height > 0 {
            metadata.exif.set("PictureWidth", AttrValue::UInt(width));
            metadata.exif.set("PictureHeight", AttrValue::UInt(height));
        }
        if color_depth > 0 {
            metadata.exif.set("PictureColorDepth", AttrValue::UInt(color_depth));
        }
        metadata.exif.set("PictureSize", AttrValue::UInt(data_len));

        // Seek to end of block (skip picture data)
        reader.seek(SeekFrom::Start(end_pos))?;

        Ok(())
    }
}

/// Format duration in seconds to human-readable string.
fn format_duration(secs: f64) -> String {
    let total_secs = secs as u64;
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs_rem = total_secs % 60;
    let millis = ((secs - total_secs as f64) * 1000.0) as u64;

    if hours > 0 {
        format!("{}:{:02}:{:02}.{:03}", hours, mins, secs_rem, millis)
    } else {
        format!("{}:{:02}.{:03}", mins, secs_rem, millis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_flac_header() -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(b"fLaC");
        data
    }

    fn make_streaminfo_block(is_last: bool) -> Vec<u8> {
        let mut block = Vec::new();

        // Block header: type 0 (STREAMINFO)
        let header_byte = if is_last { 0x80 } else { 0x00 };
        block.push(header_byte);
        // Block size: 34 bytes
        block.push(0);
        block.push(0);
        block.push(34);

        // STREAMINFO data (34 bytes)
        // Min block size (2 bytes): 4096
        block.extend_from_slice(&4096u16.to_be_bytes());
        // Max block size (2 bytes): 4096
        block.extend_from_slice(&4096u16.to_be_bytes());
        // Min frame size (3 bytes): 0
        block.extend_from_slice(&[0, 0, 0]);
        // Max frame size (3 bytes): 0
        block.extend_from_slice(&[0, 0, 0]);

        // Sample rate (20 bits): 44100 = 0xAC44
        // Channels (3 bits): 2-1 = 1 (stereo)
        // Bits per sample (5 bits): 16-1 = 15
        // Total samples (36 bits): 441000
        // Pack these into 8 bytes
        // Sample rate: 44100 in 20 bits
        // 44100 = 0x00AC44
        // Byte 10: (44100 >> 12) = 0x0A
        // Byte 11: (44100 >> 4) & 0xFF = 0xC4
        // Byte 12 upper 4: (44100 & 0x0F) << 4 = 0x40
        // Byte 12 bits 3-1: channels-1 = 1 << 1 = 0x02
        // Byte 12 bit 0: (bits_per_sample-1) >> 4 = 0
        // Byte 13 upper 4: (bits_per_sample-1) & 0x0F = 0xF0
        // Byte 13 lower 4: total_samples >> 32 = 0
        block.push(0x0A); // byte 10
        block.push(0xC4); // byte 11
        block.push(0x42); // byte 12: sample_rate lower | channels | bps upper
        block.push(0xF0); // byte 13: bps lower | samples upper
        // Total samples: 441000 = 0x6BA98
        block.push(0x00); // byte 14
        block.push(0x06); // byte 15
        block.push(0xBA); // byte 16
        block.push(0x98); // byte 17

        // MD5 (16 bytes): zeros
        block.extend_from_slice(&[0u8; 16]);

        block
    }

    #[test]
    fn detect_flac() {
        let parser = FlacParser;
        assert!(parser.can_parse(b"fLaC"));
        assert!(parser.can_parse(&[b'f', b'L', b'a', b'C', 0, 0, 0]));
    }

    #[test]
    fn reject_non_flac() {
        let parser = FlacParser;
        assert!(!parser.can_parse(b"ID3"));
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
    }

    #[test]
    fn parse_streaminfo() {
        let parser = FlacParser;

        let mut data = make_flac_header();
        data.extend_from_slice(&make_streaminfo_block(true));

        let mut cursor = Cursor::new(&data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_u32("SampleRate"), Some(44100));
        assert_eq!(meta.exif.get_u32("AudioChannels"), Some(2));
        assert_eq!(meta.exif.get_u32("BitsPerSample"), Some(16));
    }

    #[test]
    fn parse_vorbis_comment() {
        let parser = FlacParser;

        let mut data = make_flac_header();
        data.extend_from_slice(&make_streaminfo_block(false));

        // VORBIS_COMMENT block
        let vendor = b"Test Encoder";
        let comments = [
            "TITLE=Test Song",
            "ARTIST=Test Artist",
            "ALBUM=Test Album",
        ];

        let mut comment_data = Vec::new();
        // Vendor length (little-endian)
        comment_data.extend_from_slice(&(vendor.len() as u32).to_le_bytes());
        comment_data.extend_from_slice(vendor);
        // Comment count
        comment_data.extend_from_slice(&(comments.len() as u32).to_le_bytes());
        for comment in &comments {
            comment_data.extend_from_slice(&(comment.len() as u32).to_le_bytes());
            comment_data.extend_from_slice(comment.as_bytes());
        }

        // Block header
        data.push(0x84); // last block, type 4 (VORBIS_COMMENT)
        let len = comment_data.len() as u32;
        data.push((len >> 16) as u8);
        data.push((len >> 8) as u8);
        data.push(len as u8);
        data.extend_from_slice(&comment_data);

        let mut cursor = Cursor::new(&data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("Encoder"), Some("Test Encoder"));
        assert_eq!(meta.exif.get_str("Title"), Some("Test Song"));
        assert_eq!(meta.exif.get_str("Artist"), Some("Test Artist"));
        assert_eq!(meta.exif.get_str("Album"), Some("Test Album"));
    }

    #[test]
    fn test_duration_format() {
        assert_eq!(format_duration(65.5), "1:05.500");
        assert_eq!(format_duration(3661.0), "1:01:01.000");
    }
}

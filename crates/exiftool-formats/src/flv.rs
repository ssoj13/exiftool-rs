//! FLV (Flash Video) format parser.
//!
//! FLV is Adobe's Flash Video format.
//!
//! # Structure
//!
//! - Header (9 bytes): "FLV" + version + flags + header size
//! - Tags: audio, video, script data (metadata)

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// FLV format parser.
pub struct FlvParser;

impl FormatParser for FlvParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 4 && &header[0..3] == b"FLV" && header[3] == 0x01
    }

    fn format_name(&self) -> &'static str {
        "FLV"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["flv", "f4v"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("FLV");
        meta.exif.set("File:FileType", AttrValue::Str("FLV".to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("video/x-flv".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // File size
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        reader.seek(SeekFrom::Start(0))?;

        // Read header
        let mut header = [0u8; 9];
        reader.read_exact(&mut header)?;

        // Version
        let version = header[3];
        meta.exif.set("FLV:Version", AttrValue::UInt(version as u32));

        // Flags
        let flags = header[4];
        let has_audio = flags & 0x04 != 0;
        let has_video = flags & 0x01 != 0;
        meta.exif.set("FLV:HasAudio", AttrValue::Str(if has_audio { "Yes" } else { "No" }.to_string()));
        meta.exif.set("FLV:HasVideo", AttrValue::Str(if has_video { "Yes" } else { "No" }.to_string()));

        // Header size (usually 9)
        let header_size = u32::from_be_bytes([header[5], header[6], header[7], header[8]]);
        
        // Skip to first tag (after previous tag size = 0)
        reader.seek(SeekFrom::Start(header_size as u64 + 4))?;

        // Parse tags to find metadata
        let mut found_metadata = false;
        let mut video_codec: Option<&str> = None;
        let mut audio_codec: Option<&str> = None;

        for _ in 0..100 {
            let pos = reader.stream_position()?;
            if pos + 11 >= file_size {
                break;
            }

            let mut tag_header = [0u8; 11];
            if reader.read_exact(&mut tag_header).is_err() {
                break;
            }

            let tag_type = tag_header[0];
            let data_size = ((tag_header[1] as u32) << 16) 
                | ((tag_header[2] as u32) << 8) 
                | (tag_header[3] as u32);

            if data_size == 0 || data_size > 10_000_000 {
                break;
            }

            match tag_type {
                8 => {
                    // Audio tag
                    if audio_codec.is_none() && data_size > 0 {
                        let mut audio_byte = [0u8; 1];
                        reader.read_exact(&mut audio_byte)?;
                        
                        let sound_format = (audio_byte[0] >> 4) & 0x0F;
                        audio_codec = Some(match sound_format {
                            0 => "Linear PCM",
                            1 => "ADPCM",
                            2 => "MP3",
                            3 => "Linear PCM (LE)",
                            4 => "Nellymoser 16kHz",
                            5 => "Nellymoser 8kHz",
                            6 => "Nellymoser",
                            7 => "G.711 A-law",
                            8 => "G.711 mu-law",
                            10 => "AAC",
                            11 => "Speex",
                            14 => "MP3 8kHz",
                            15 => "Device-specific",
                            _ => "Unknown",
                        });
                        
                        let sample_rate = match (audio_byte[0] >> 2) & 0x03 {
                            0 => 5500,
                            1 => 11025,
                            2 => 22050,
                            3 => 44100,
                            _ => 0,
                        };
                        if sample_rate > 0 {
                            meta.exif.set("Audio:SampleRate", AttrValue::UInt(sample_rate));
                        }

                        let bits = if audio_byte[0] & 0x02 != 0 { 16 } else { 8 };
                        meta.exif.set("Audio:BitsPerSample", AttrValue::UInt(bits));

                        let channels = if audio_byte[0] & 0x01 != 0 { 2 } else { 1 };
                        meta.exif.set("Audio:Channels", AttrValue::UInt(channels));

                        // Seek past rest of tag
                        reader.seek(SeekFrom::Current(data_size as i64 - 1))?;
                    } else {
                        reader.seek(SeekFrom::Current(data_size as i64))?;
                    }
                }
                9 => {
                    // Video tag
                    if video_codec.is_none() && data_size > 0 {
                        let mut video_byte = [0u8; 1];
                        reader.read_exact(&mut video_byte)?;
                        
                        let codec_id = video_byte[0] & 0x0F;
                        video_codec = Some(match codec_id {
                            1 => "JPEG",
                            2 => "Sorenson H.263",
                            3 => "Screen Video",
                            4 => "VP6",
                            5 => "VP6 Alpha",
                            6 => "Screen Video V2",
                            7 => "H.264/AVC",
                            _ => "Unknown",
                        });

                        reader.seek(SeekFrom::Current(data_size as i64 - 1))?;
                    } else {
                        reader.seek(SeekFrom::Current(data_size as i64))?;
                    }
                }
                18 => {
                    // Script data (metadata)
                    if !found_metadata && data_size > 0 && data_size < 100000 {
                        let mut script_data = vec![0u8; data_size as usize];
                        reader.read_exact(&mut script_data)?;
                        
                        parse_flv_metadata(&script_data, &mut meta);
                        found_metadata = true;
                    } else {
                        reader.seek(SeekFrom::Current(data_size as i64))?;
                    }
                }
                _ => {
                    reader.seek(SeekFrom::Current(data_size as i64))?;
                }
            }

            // Skip previous tag size (4 bytes)
            reader.seek(SeekFrom::Current(4))?;

            // Stop after finding what we need
            if found_metadata && video_codec.is_some() && audio_codec.is_some() {
                break;
            }
        }

        if let Some(codec) = video_codec {
            meta.exif.set("Video:Codec", AttrValue::Str(codec.to_string()));
        }
        if let Some(codec) = audio_codec {
            meta.exif.set("Audio:Codec", AttrValue::Str(codec.to_string()));
        }

        Ok(meta)
    }
}

/// Parse FLV script data (AMF0 format).
fn parse_flv_metadata(data: &[u8], meta: &mut Metadata) {
    if data.len() < 13 {
        return;
    }

    // First should be AMF string "onMetaData"
    if data[0] != 0x02 {
        return;
    }

    let str_len = u16::from_be_bytes([data[1], data[2]]) as usize;
    if str_len + 3 > data.len() {
        return;
    }

    let name = String::from_utf8_lossy(&data[3..3 + str_len]);
    if name != "onMetaData" {
        return;
    }

    // Next should be ECMA array (type 8)
    let mut pos = 3 + str_len;
    if pos >= data.len() || data[pos] != 0x08 {
        return;
    }
    pos += 1;

    // Array count
    if pos + 4 > data.len() {
        return;
    }
    let _count = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
    pos += 4;

    // Parse properties
    for _ in 0..50 {
        if pos + 2 >= data.len() {
            break;
        }

        // Property name length
        let name_len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2;

        if name_len == 0 || pos + name_len >= data.len() {
            break;
        }

        let prop_name = String::from_utf8_lossy(&data[pos..pos + name_len]).to_string();
        pos += name_len;

        if pos >= data.len() {
            break;
        }

        // Value type
        let value_type = data[pos];
        pos += 1;

        match value_type {
            0 => {
                // Number (double)
                if pos + 8 > data.len() {
                    break;
                }
                let bits = u64::from_be_bytes([
                    data[pos], data[pos + 1], data[pos + 2], data[pos + 3],
                    data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7],
                ]);
                let value = f64::from_bits(bits);
                pos += 8;

                let tag = match prop_name.as_str() {
                    "duration" => Some("Video:Duration"),
                    "width" => Some("Video:Width"),
                    "height" => Some("Video:Height"),
                    "framerate" | "videoframerate" => Some("Video:FrameRate"),
                    "videodatarate" => Some("Video:Bitrate"),
                    "audiodatarate" => Some("Audio:Bitrate"),
                    "audiosamplerate" => Some("Audio:SampleRate"),
                    "audiosamplesize" => Some("Audio:BitsPerSample"),
                    "filesize" => None, // Already have this
                    _ => None,
                };

                if let Some(tag_name) = tag {
                    if tag_name.contains("Width") || tag_name.contains("Height") 
                        || tag_name.contains("Bitrate") || tag_name.contains("Sample") {
                        meta.exif.set(tag_name, AttrValue::UInt(value as u32));
                    } else {
                        meta.exif.set(tag_name, AttrValue::Double(value));
                    }
                }
            }
            1 => {
                // Boolean
                if pos >= data.len() {
                    break;
                }
                let _value = data[pos] != 0;
                pos += 1;
            }
            2 => {
                // String
                if pos + 2 > data.len() {
                    break;
                }
                let str_len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
                pos += 2;
                
                if pos + str_len > data.len() {
                    break;
                }
                let value = String::from_utf8_lossy(&data[pos..pos + str_len]).to_string();
                pos += str_len;

                let tag = match prop_name.as_str() {
                    "encoder" | "metadatacreator" => Some("Video:Encoder"),
                    "videocodecid" => Some("Video:CodecID"),
                    "audiocodecid" => Some("Audio:CodecID"),
                    _ => None,
                };

                if let Some(tag_name) = tag {
                    meta.exif.set(tag_name, AttrValue::Str(value));
                }
            }
            9 => {
                // Object end marker
                break;
            }
            _ => {
                // Unknown type, can't parse further
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_flv_header() -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // FLV signature
        data[0..3].copy_from_slice(b"FLV");
        data[3] = 0x01; // Version 1
        data[4] = 0x05; // Has audio + video
        // Header size = 9
        data[5..9].copy_from_slice(&9u32.to_be_bytes());
        // Previous tag size = 0
        data[9..13].copy_from_slice(&0u32.to_be_bytes());
        data
    }

    #[test]
    fn test_can_parse() {
        let parser = FlvParser;
        let data = make_flv_header();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = FlvParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"RIFF"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = FlvParser;
        let data = make_flv_header();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "FLV");
        assert_eq!(meta.exif.get_u32("FLV:Version"), Some(1));
        assert_eq!(meta.exif.get_str("FLV:HasAudio"), Some("Yes"));
        assert_eq!(meta.exif.get_str("FLV:HasVideo"), Some("Yes"));
    }

    #[test]
    fn test_format_info() {
        let parser = FlvParser;
        assert_eq!(parser.format_name(), "FLV");
        assert!(parser.extensions().contains(&"flv"));
    }
}

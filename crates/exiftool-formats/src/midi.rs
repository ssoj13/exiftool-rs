//! MIDI (Musical Instrument Digital Interface) format parser.
//!
//! Standard MIDI File (SMF) format.
//!
//! # Structure
//!
//! - "MThd" header chunk (14 bytes total)
//! - Track chunks ("MTrk" + length + events)
//!
//! Meta events contain metadata like tempo, time signature, track names.

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// MIDI file parser.
pub struct MidiParser;

impl FormatParser for MidiParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 4 && &header[0..4] == b"MThd"
    }

    fn format_name(&self) -> &'static str {
        "MIDI"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["mid", "midi", "smf", "kar"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("MIDI");
        meta.exif.set("File:FileType", AttrValue::Str("MIDI".to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("audio/midi".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // Read MThd header
        let mut header = [0u8; 14];
        reader.read_exact(&mut header)?;

        // Header length (should be 6)
        let header_len = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);
        meta.exif.set("MIDI:HeaderLength", AttrValue::UInt(header_len));

        // Format type (0, 1, or 2)
        let format = u16::from_be_bytes([header[8], header[9]]);
        let format_str = match format {
            0 => "Single track",
            1 => "Multiple tracks, synchronous",
            2 => "Multiple tracks, asynchronous",
            _ => "Unknown",
        };
        meta.exif.set("MIDI:Format", AttrValue::UInt(format as u32));
        meta.exif.set("MIDI:FormatDescription", AttrValue::Str(format_str.to_string()));

        // Number of tracks
        let num_tracks = u16::from_be_bytes([header[10], header[11]]);
        meta.exif.set("MIDI:NumTracks", AttrValue::UInt(num_tracks as u32));

        // Division (timing)
        let division = u16::from_be_bytes([header[12], header[13]]);
        if division & 0x8000 != 0 {
            // SMPTE timing
            let fps = (!(division >> 8) as i8).unsigned_abs();
            let ticks = division & 0xFF;
            meta.exif.set("MIDI:TimingMode", AttrValue::Str("SMPTE".to_string()));
            meta.exif.set("MIDI:SMPTE_FPS", AttrValue::UInt(fps as u32));
            meta.exif.set("MIDI:TicksPerFrame", AttrValue::UInt(ticks as u32));
        } else {
            // Ticks per quarter note
            meta.exif.set("MIDI:TimingMode", AttrValue::Str("PPQ".to_string()));
            meta.exif.set("MIDI:TicksPerQuarterNote", AttrValue::UInt(division as u32));
        }

        // File size
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        // Parse tracks for metadata
        reader.seek(SeekFrom::Start(8 + header_len as u64))?;
        
        let mut track_idx = 0u32;
        let mut total_duration_ticks = 0u64;
        let mut tempo = 500000u32; // default 120 BPM
        
        while track_idx < num_tracks as u32 {
            let mut chunk_header = [0u8; 8];
            if reader.read_exact(&mut chunk_header).is_err() {
                break;
            }

            if &chunk_header[0..4] != b"MTrk" {
                break;
            }

            let track_len = u32::from_be_bytes([
                chunk_header[4], chunk_header[5], chunk_header[6], chunk_header[7],
            ]);

            let track_start = reader.stream_position()?;
            
            // Parse track events for metadata (first track only for speed)
            if track_idx == 0 || track_idx == 1 {
                parse_track_meta(reader, track_len, &mut meta, &mut tempo, &mut total_duration_ticks)?;
            }

            // Skip to next track
            reader.seek(SeekFrom::Start(track_start + track_len as u64))?;
            track_idx += 1;
        }

        // Calculate duration if we have PPQ timing
        let division_val = division & 0x7FFF;
        if division & 0x8000 == 0 && division_val > 0 && total_duration_ticks > 0 {
            // Duration = ticks * tempo_us / (ppq * 1000000)
            let duration_secs = (total_duration_ticks as f64 * tempo as f64) 
                / (division_val as f64 * 1_000_000.0);
            if duration_secs > 0.0 && duration_secs < 86400.0 {
                meta.exif.set("Audio:Duration", AttrValue::Double(duration_secs));
            }
        }

        // Tempo in BPM
        let bpm = 60_000_000.0 / tempo as f64;
        meta.exif.set("MIDI:Tempo", AttrValue::Double(bpm));

        Ok(meta)
    }
}

/// Parse track for metadata events.
fn parse_track_meta(
    reader: &mut dyn ReadSeek,
    track_len: u32,
    meta: &mut Metadata,
    tempo: &mut u32,
    total_ticks: &mut u64,
) -> Result<()> {
    let start_pos = reader.stream_position()?;
    let end_pos = start_pos + track_len as u64;
    let mut current_ticks = 0u64;

    while reader.stream_position()? < end_pos {
        // Read variable-length delta time
        let delta = read_var_len(reader)?;
        current_ticks += delta as u64;

        // Read event
        let mut status = [0u8; 1];
        if reader.read_exact(&mut status).is_err() {
            break;
        }

        match status[0] {
            0xFF => {
                // Meta event
                let mut meta_type = [0u8; 1];
                reader.read_exact(&mut meta_type)?;
                let len = read_var_len(reader)? as usize;
                
                if len > 0 && len < 10000 {
                    let mut data = vec![0u8; len];
                    reader.read_exact(&mut data)?;
                    
                    match meta_type[0] {
                        0x01 => {
                            // Text event
                            if let Ok(text) = String::from_utf8(data) {
                                if meta.exif.get_str("MIDI:Text").is_none() {
                                    meta.exif.set("MIDI:Text", AttrValue::Str(text.trim().to_string()));
                                }
                            }
                        }
                        0x02 => {
                            // Copyright
                            if let Ok(text) = String::from_utf8(data) {
                                meta.exif.set("MIDI:Copyright", AttrValue::Str(text.trim().to_string()));
                            }
                        }
                        0x03 => {
                            // Track name
                            if let Ok(text) = String::from_utf8(data) {
                                if meta.exif.get_str("MIDI:TrackName").is_none() {
                                    meta.exif.set("MIDI:TrackName", AttrValue::Str(text.trim().to_string()));
                                }
                            }
                        }
                        0x04 => {
                            // Instrument name
                            if let Ok(text) = String::from_utf8(data) {
                                if meta.exif.get_str("MIDI:Instrument").is_none() {
                                    meta.exif.set("MIDI:Instrument", AttrValue::Str(text.trim().to_string()));
                                }
                            }
                        }
                        0x05 => {
                            // Lyric
                            // Skip - too many
                        }
                        0x06 => {
                            // Marker
                            if let Ok(text) = String::from_utf8(data) {
                                if meta.exif.get_str("MIDI:Marker").is_none() {
                                    meta.exif.set("MIDI:Marker", AttrValue::Str(text.trim().to_string()));
                                }
                            }
                        }
                        0x51 => {
                            // Tempo (microseconds per quarter note)
                            if data.len() >= 3 {
                                *tempo = ((data[0] as u32) << 16) 
                                    | ((data[1] as u32) << 8) 
                                    | (data[2] as u32);
                            }
                        }
                        0x58 => {
                            // Time signature
                            if data.len() >= 4 {
                                let num = data[0];
                                let denom = 1u32 << data[1];
                                meta.exif.set("MIDI:TimeSignature", 
                                    AttrValue::Str(format!("{}/{}", num, denom)));
                            }
                        }
                        0x59 => {
                            // Key signature
                            if data.len() >= 2 {
                                let sf = data[0] as i8;
                                let mi = data[1];
                                let key = key_signature_name(sf, mi);
                                meta.exif.set("MIDI:KeySignature", AttrValue::Str(key));
                            }
                        }
                        0x2F => {
                            // End of track
                            *total_ticks = (*total_ticks).max(current_ticks);
                            break;
                        }
                        _ => {}
                    }
                } else if len > 0 {
                    // Skip large data
                    reader.seek(SeekFrom::Current(len as i64))?;
                }
            }
            0xF0 | 0xF7 => {
                // SysEx event
                let len = read_var_len(reader)? as i64;
                reader.seek(SeekFrom::Current(len))?;
            }
            _ => {
                // Channel event
                let msg_type = status[0] & 0xF0;
                let bytes_to_skip = match msg_type {
                    0x80 | 0x90 | 0xA0 | 0xB0 | 0xE0 => 2,
                    0xC0 | 0xD0 => 1,
                    _ => 0,
                };
                if bytes_to_skip > 0 {
                    reader.seek(SeekFrom::Current(bytes_to_skip))?;
                }
            }
        }
    }

    *total_ticks = (*total_ticks).max(current_ticks);
    Ok(())
}

/// Read MIDI variable-length quantity.
fn read_var_len(reader: &mut dyn ReadSeek) -> Result<u32> {
    let mut value = 0u32;
    for _ in 0..4 {
        let mut byte = [0u8; 1];
        reader.read_exact(&mut byte)?;
        value = (value << 7) | (byte[0] & 0x7F) as u32;
        if byte[0] & 0x80 == 0 {
            break;
        }
    }
    Ok(value)
}

/// Get key signature name from MIDI key signature event.
fn key_signature_name(sf: i8, mi: u8) -> String {
    let major_keys = ["C", "G", "D", "A", "E", "B", "F#", "C#"];
    let minor_keys = ["A", "E", "B", "F#", "C#", "G#", "D#", "A#"];
    let flat_major = ["C", "F", "Bb", "Eb", "Ab", "Db", "Gb", "Cb"];
    let flat_minor = ["A", "D", "G", "C", "F", "Bb", "Eb", "Ab"];
    
    let mode = if mi == 0 { "major" } else { "minor" };
    
    let key = if sf >= 0 {
        let idx = sf.min(7) as usize;
        if mi == 0 { major_keys[idx] } else { minor_keys[idx] }
    } else {
        let idx = (-sf).min(7) as usize;
        if mi == 0 { flat_major[idx] } else { flat_minor[idx] }
    };
    
    format!("{} {}", key, mode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_midi_header(format: u16, tracks: u16, division: u16) -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // MThd
        data[0..4].copy_from_slice(b"MThd");
        data[4..8].copy_from_slice(&6u32.to_be_bytes());
        data[8..10].copy_from_slice(&format.to_be_bytes());
        data[10..12].copy_from_slice(&tracks.to_be_bytes());
        data[12..14].copy_from_slice(&division.to_be_bytes());
        
        // Empty track
        data[14..18].copy_from_slice(b"MTrk");
        data[18..22].copy_from_slice(&4u32.to_be_bytes());
        // Delta 0, End of track
        data[22] = 0x00;
        data[23] = 0xFF;
        data[24] = 0x2F;
        data[25] = 0x00;
        
        data
    }

    #[test]
    fn test_can_parse() {
        let parser = MidiParser;
        let data = make_midi_header(1, 1, 480);
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = MidiParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"RIFF"));
    }

    #[test]
    fn test_parse_format0() {
        let parser = MidiParser;
        let data = make_midi_header(0, 1, 480);
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "MIDI");
        assert_eq!(meta.exif.get_u32("MIDI:Format"), Some(0));
        assert_eq!(meta.exif.get_u32("MIDI:NumTracks"), Some(1));
        assert_eq!(meta.exif.get_u32("MIDI:TicksPerQuarterNote"), Some(480));
    }

    #[test]
    fn test_parse_format1() {
        let parser = MidiParser;
        let data = make_midi_header(1, 4, 960);
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_u32("MIDI:Format"), Some(1));
        assert_eq!(meta.exif.get_str("MIDI:FormatDescription"), Some("Multiple tracks, synchronous"));
        assert_eq!(meta.exif.get_u32("MIDI:NumTracks"), Some(4));
    }

    #[test]
    fn test_format_info() {
        let parser = MidiParser;
        assert_eq!(parser.format_name(), "MIDI");
        assert!(parser.extensions().contains(&"mid"));
        assert!(parser.extensions().contains(&"midi"));
    }

    #[test]
    fn test_key_signature() {
        assert_eq!(key_signature_name(0, 0), "C major");
        assert_eq!(key_signature_name(0, 1), "A minor");
        assert_eq!(key_signature_name(2, 0), "D major");
        assert_eq!(key_signature_name(-3, 0), "Eb major");
    }
}

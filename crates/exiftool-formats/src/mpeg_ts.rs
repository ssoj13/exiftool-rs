//! MPEG Transport Stream parser (.ts, .mts, .m2ts).
//!
//! MPEG-TS is used for broadcast video and Blu-ray (M2TS).
//!
//! # Structure
//!
//! - 188-byte packets (or 192 for M2TS with timestamp)
//! - Sync byte 0x47
//! - PAT (Program Association Table) - PID 0
//! - PMT (Program Map Table) - describes streams
//! - PES packets contain audio/video data

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

const TS_PACKET_SIZE: usize = 188;
const M2TS_PACKET_SIZE: usize = 192;
const SYNC_BYTE: u8 = 0x47;

/// MPEG Transport Stream parser.
pub struct MpegTsParser;

impl FormatParser for MpegTsParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 5 {
            return false;
        }
        // Standard TS: sync byte at offset 0
        if header[0] == SYNC_BYTE {
            return true;
        }
        // M2TS: 4-byte timestamp prefix, sync at offset 4
        if header.len() >= 5 && header[4] == SYNC_BYTE {
            return true;
        }
        false
    }

    fn format_name(&self) -> &'static str {
        "MPEG-TS"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["ts", "mts", "m2ts", "mxts"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("MPEG-TS");
        meta.set_file_type("MPEG-TS", "video/mp2t");

        reader.seek(SeekFrom::Start(0))?;

        // File size
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        reader.seek(SeekFrom::Start(0))?;

        // Detect packet size (188 or 192)
        let mut probe = [0u8; 5];
        reader.read_exact(&mut probe)?;

        let (packet_size, is_m2ts) = if probe[0] == SYNC_BYTE {
            (TS_PACKET_SIZE, false)
        } else if probe[4] == SYNC_BYTE {
            meta.format = "M2TS";
            meta.set_file_type("M2TS", "");
            meta.exif.set("MPEG:Container", AttrValue::Str("BDAV (Blu-ray)".to_string()));
            (M2TS_PACKET_SIZE, true)
        } else {
            return Ok(meta);
        };

        meta.exif.set("MPEG:PacketSize", AttrValue::UInt(packet_size as u32));

        // Calculate packet count
        let packet_count = file_size / packet_size as u64;
        meta.exif.set("MPEG:PacketCount", AttrValue::UInt64(packet_count));

        reader.seek(SeekFrom::Start(0))?;

        // Parse packets to find streams
        let mut pmt_pid: Option<u16> = None;
        let mut video_pids: Vec<u16> = Vec::new();
        let mut audio_pids: Vec<u16> = Vec::new();
        let mut video_codec: Option<&str> = None;
        let mut audio_codec: Option<&str> = None;

        // Read first N packets to find PAT/PMT
        let max_packets = 1000.min(packet_count) as usize;
        let mut packet = vec![0u8; packet_size];

        for _ in 0..max_packets {
            if reader.read_exact(&mut packet).is_err() {
                break;
            }

            let offset = if is_m2ts { 4 } else { 0 };
            
            // Verify sync byte
            if packet[offset] != SYNC_BYTE {
                continue;
            }

            // Parse TS header
            let pid = (((packet[offset + 1] & 0x1F) as u16) << 8) | packet[offset + 2] as u16;
            let has_payload = packet[offset + 3] & 0x10 != 0;
            let has_adaptation = packet[offset + 3] & 0x20 != 0;

            if !has_payload {
                continue;
            }

            let payload_start = offset + 4 + if has_adaptation {
                1 + packet[offset + 4] as usize
            } else {
                0
            };

            if payload_start >= packet_size {
                continue;
            }

            let payload = &packet[payload_start..];

            // PAT (PID 0)
            if pid == 0 && payload.len() > 8 {
                // Skip pointer field if present
                let ptr = if packet[offset + 1] & 0x40 != 0 { payload[0] as usize + 1 } else { 0 };
                if ptr < payload.len() - 8 {
                    let pat = &payload[ptr..];
                    if pat[0] == 0x00 { // table_id for PAT
                        // Parse PAT to find PMT PID
                        let section_len = (((pat[1] & 0x0F) as usize) << 8) | pat[2] as usize;
                        if section_len > 5 && pat.len() > 8 {
                            // Skip to program entries (after 8 byte header)
                            let prog_start = 8;
                            if prog_start + 4 <= pat.len().min(section_len + 3) {
                                let prog_pid = (((pat[prog_start + 2] & 0x1F) as u16) << 8) 
                                    | pat[prog_start + 3] as u16;
                                if prog_pid > 0 && prog_pid < 0x1FFF {
                                    pmt_pid = Some(prog_pid);
                                }
                            }
                        }
                    }
                }
            }

            // PMT
            if let Some(pmt) = pmt_pid {
                if pid == pmt && payload.len() > 12 {
                    let ptr = if packet[offset + 1] & 0x40 != 0 { payload[0] as usize + 1 } else { 0 };
                    if ptr < payload.len() - 12 {
                        let pmt_data = &payload[ptr..];
                        if pmt_data[0] == 0x02 { // table_id for PMT
                            let section_len = (((pmt_data[1] & 0x0F) as usize) << 8) | pmt_data[2] as usize;
                            let prog_info_len = (((pmt_data[10] & 0x0F) as usize) << 8) | pmt_data[11] as usize;
                            
                            let mut pos = 12 + prog_info_len;
                            let end = (section_len + 3).min(pmt_data.len()).saturating_sub(4);

                            while pos + 5 <= end {
                                let stream_type = pmt_data[pos];
                                let es_pid = (((pmt_data[pos + 1] & 0x1F) as u16) << 8) 
                                    | pmt_data[pos + 2] as u16;
                                let es_info_len = (((pmt_data[pos + 3] & 0x0F) as usize) << 8) 
                                    | pmt_data[pos + 4] as usize;

                                // Identify stream type
                                match stream_type {
                                    0x01 | 0x02 => {
                                        video_pids.push(es_pid);
                                        video_codec = Some("MPEG-2");
                                    }
                                    0x1B => {
                                        video_pids.push(es_pid);
                                        video_codec = Some("H.264/AVC");
                                    }
                                    0x24 => {
                                        video_pids.push(es_pid);
                                        video_codec = Some("H.265/HEVC");
                                    }
                                    0x03 | 0x04 => {
                                        audio_pids.push(es_pid);
                                        audio_codec = Some("MPEG Audio");
                                    }
                                    0x0F => {
                                        audio_pids.push(es_pid);
                                        audio_codec = Some("AAC");
                                    }
                                    0x81 => {
                                        audio_pids.push(es_pid);
                                        audio_codec = Some("AC-3");
                                    }
                                    0x82 | 0x86 => {
                                        audio_pids.push(es_pid);
                                        audio_codec = Some("DTS");
                                    }
                                    0x83 | 0x84 => {
                                        audio_pids.push(es_pid);
                                        audio_codec = Some("TrueHD/DTS-HD");
                                    }
                                    _ => {}
                                }

                                pos += 5 + es_info_len;
                            }
                            break; // Got PMT, done
                        }
                    }
                }
            }
        }

        // Set metadata
        if !video_pids.is_empty() {
            meta.exif.set("Video:StreamCount", AttrValue::UInt(video_pids.len() as u32));
            if let Some(codec) = video_codec {
                meta.exif.set("Video:Codec", AttrValue::Str(codec.to_string()));
            }
        }

        if !audio_pids.is_empty() {
            meta.exif.set("Audio:StreamCount", AttrValue::UInt(audio_pids.len() as u32));
            if let Some(codec) = audio_codec {
                meta.exif.set("Audio:Codec", AttrValue::Str(codec.to_string()));
            }
        }

        // Estimate duration from bitrate (rough)
        // Average broadcast bitrate ~15-25 Mbps
        if file_size > 0 {
            let est_bitrate = 20_000_000.0; // 20 Mbps estimate
            let est_duration = (file_size as f64 * 8.0) / est_bitrate;
            if est_duration > 0.0 && est_duration < 86400.0 * 7.0 {
                meta.exif.set("Video:EstimatedDuration", AttrValue::Double(est_duration));
            }
        }

        Ok(meta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_ts_packet() -> Vec<u8> {
        let mut data = vec![0u8; 752]; // 4 packets
        // Packet 1: PAT (PID 0)
        data[0] = SYNC_BYTE;
        data[1] = 0x40; // PUSI + PID high
        data[2] = 0x00; // PID low = 0
        data[3] = 0x10; // payload only
        // PAT payload
        data[4] = 0x00; // pointer
        data[5] = 0x00; // table_id
        data[6] = 0xB0; // section syntax + length high
        data[7] = 0x0D; // length low
        // ... simplified
        
        // Packet 2-4: more sync bytes at 188, 376, 564
        data[188] = SYNC_BYTE;
        data[376] = SYNC_BYTE;
        data[564] = SYNC_BYTE;
        
        data
    }

    fn make_m2ts_packet() -> Vec<u8> {
        let mut data = vec![0u8; 768]; // 4 packets
        // M2TS has 4-byte timestamp prefix
        data[4] = SYNC_BYTE;
        data[5] = 0x40;
        data[6] = 0x00;
        data[7] = 0x10;
        
        data[196] = SYNC_BYTE;
        data[388] = SYNC_BYTE;
        data[580] = SYNC_BYTE;
        
        data
    }

    #[test]
    fn test_can_parse_ts() {
        let parser = MpegTsParser;
        let data = make_ts_packet();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_can_parse_m2ts() {
        let parser = MpegTsParser;
        let data = make_m2ts_packet();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = MpegTsParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"RIFF"));
    }

    #[test]
    fn test_parse_ts() {
        let parser = MpegTsParser;
        let data = make_ts_packet();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "MPEG-TS");
        assert_eq!(meta.exif.get_u32("MPEG:PacketSize"), Some(188));
    }

    #[test]
    fn test_parse_m2ts() {
        let parser = MpegTsParser;
        let data = make_m2ts_packet();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "M2TS");
        assert_eq!(meta.exif.get_u32("MPEG:PacketSize"), Some(192));
    }

    #[test]
    fn test_format_info() {
        let parser = MpegTsParser;
        assert_eq!(parser.format_name(), "MPEG-TS");
        assert!(parser.extensions().contains(&"ts"));
        assert!(parser.extensions().contains(&"m2ts"));
    }
}

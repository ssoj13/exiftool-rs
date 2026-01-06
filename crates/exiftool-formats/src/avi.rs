//! AVI format parser.
//!
//! AVI (Audio Video Interleave) uses RIFF container format:
//! - RIFF header: "RIFF" + size + "AVI "
//! - LIST chunks contain sub-chunks (hdrl, movi, INFO)
//! - avih: main AVI header (dimensions, frame rate)
//! - strh/strf: stream headers
//! - INFO: metadata text fields (INAM, IART, etc.)

use crate::tag_lookup::{lookup_exif_subifd, lookup_gps, lookup_ifd0};
use crate::{entry_to_attr, makernotes, Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use exiftool_core::{ByteOrder, IfdReader, RawValue};
use std::io::SeekFrom;

/// AVI parser.
pub struct AviParser;

impl FormatParser for AviParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 12 {
            return false;
        }
        // RIFF magic + AVI form type
        &header[0..4] == b"RIFF" && &header[8..12] == b"AVI "
    }

    fn format_name(&self) -> &'static str {
        "AVI"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["avi", "divx"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut header = [0u8; 12];
        reader.read_exact(&mut header)?;

        if &header[0..4] != b"RIFF" || &header[8..12] != b"AVI " {
            return Err(Error::InvalidStructure("Not a valid AVI file".into()));
        }

        let file_size = u32::from_le_bytes([header[4], header[5], header[6], header[7]]) as u64 + 8;

        let mut metadata = Metadata::new("AVI");
        metadata.exif.set("File:FileType", AttrValue::Str("AVI".to_string()));
        metadata.exif.set("File:MIMEType", AttrValue::Str("video/avi".to_string()));
        metadata.exif.set("File:FileSize", AttrValue::UInt(file_size as u32));

        // Parse RIFF chunks
        self.parse_chunks(reader, file_size, &mut metadata)?;

        Ok(metadata)
    }
}

impl AviParser {
    /// Parse RIFF chunks.
    fn parse_chunks(&self, reader: &mut dyn ReadSeek, end_pos: u64, metadata: &mut Metadata) -> Result<()> {
        while reader.stream_position()? < end_pos {
            let chunk_start = reader.stream_position()?;

            let Some((chunk_id, chunk_size)) = crate::riff::read_chunk_header(reader) else {
                break;
            };

            match &chunk_id {
                b"LIST" => {
                    // LIST chunk has form type
                    let mut form_type = [0u8; 4];
                    if reader.read_exact(&mut form_type).is_err() {
                        break;
                    }

                    let list_end = chunk_start + 8 + chunk_size;

                    match &form_type {
                        b"hdrl" => {
                            // AVI header list
                            self.parse_hdrl(reader, list_end, metadata)?;
                        }
                        b"INFO" => {
                            // Metadata info list
                            crate::riff::parse_info(reader, list_end, metadata)?;
                        }
                        b"strl" => {
                            // Stream header list
                            self.parse_strl(reader, list_end, metadata)?;
                        }
                        _ => {
                            // Skip other LIST types (movi, etc.)
                            reader.seek(SeekFrom::Start(list_end))?;
                        }
                    }
                }
                b"avih" => {
                    self.parse_avih(reader, chunk_size, metadata)?;
                }
                b"JUNK" | b"PAD " => {
                    // Padding chunks
                    reader.seek(SeekFrom::Current(chunk_size as i64))?;
                }
                b"idx1" => {
                    // Index chunk
                    metadata.exif.set("AVI:HasIndex", AttrValue::Bool(true));
                    reader.seek(SeekFrom::Current(chunk_size as i64))?;
                }
                b"EXIF" => {
                    // EXIF data chunk
                    self.parse_exif_chunk(reader, chunk_size, metadata)?;
                }
                b"XMP " | b"_PMX" => {
                    // XMP metadata chunk
                    self.parse_xmp_chunk(reader, chunk_size, metadata)?;
                }
                _ => {
                    // Skip unknown chunks
                    reader.seek(SeekFrom::Current(chunk_size as i64))?;
                }
            }

            // Align to word boundary (RIFF chunks are word-aligned)
            let current = reader.stream_position()?;
            if current % 2 != 0 {
                reader.seek(SeekFrom::Current(1))?;
            }

            // Prevent infinite loop
            if reader.stream_position()? <= chunk_start {
                break;
            }
        }

        Ok(())
    }

    /// Parse AVI header list (hdrl).
    fn parse_hdrl(&self, reader: &mut dyn ReadSeek, end_pos: u64, metadata: &mut Metadata) -> Result<()> {
        while reader.stream_position()? < end_pos {
            let chunk_start = reader.stream_position()?;

            let Some((chunk_id, chunk_size)) = crate::riff::read_chunk_header(reader) else {
                break;
            };

            match &chunk_id {
                b"avih" => {
                    self.parse_avih(reader, chunk_size, metadata)?;
                }
                b"LIST" => {
                    let mut form_type = [0u8; 4];
                    reader.read_exact(&mut form_type)?;

                    if &form_type == b"strl" {
                        let list_end = chunk_start + 8 + chunk_size;
                        self.parse_strl(reader, list_end, metadata)?;
                    } else {
                        reader.seek(SeekFrom::Current(chunk_size as i64 - 4))?;
                    }
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

    /// Parse AVI main header (avih).
    fn parse_avih(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if size < 56 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(());
        }

        let mut avih = [0u8; 56];
        reader.read_exact(&mut avih)?;

        let microsec_per_frame = u32::from_le_bytes([avih[0], avih[1], avih[2], avih[3]]);
        let _max_bytes_per_sec = u32::from_le_bytes([avih[4], avih[5], avih[6], avih[7]]);
        let _padding_granularity = u32::from_le_bytes([avih[8], avih[9], avih[10], avih[11]]);
        let flags = u32::from_le_bytes([avih[12], avih[13], avih[14], avih[15]]);
        let total_frames = u32::from_le_bytes([avih[16], avih[17], avih[18], avih[19]]);
        let _initial_frames = u32::from_le_bytes([avih[20], avih[21], avih[22], avih[23]]);
        let streams = u32::from_le_bytes([avih[24], avih[25], avih[26], avih[27]]);
        let _suggested_buffer = u32::from_le_bytes([avih[28], avih[29], avih[30], avih[31]]);
        let width = u32::from_le_bytes([avih[32], avih[33], avih[34], avih[35]]);
        let height = u32::from_le_bytes([avih[36], avih[37], avih[38], avih[39]]);

        // Frame rate
        if microsec_per_frame > 0 {
            let fps = 1_000_000.0 / microsec_per_frame as f64;
            metadata.exif.set("AVI:FrameRate", AttrValue::Float(fps as f32));
        }

        metadata.exif.set("AVI:TotalFrames", AttrValue::UInt(total_frames));
        metadata.exif.set("AVI:StreamCount", AttrValue::UInt(streams));

        if width > 0 && height > 0 {
            metadata.exif.set("File:ImageWidth", AttrValue::UInt(width));
            metadata.exif.set("File:ImageHeight", AttrValue::UInt(height));
        }

        // Flags
        if flags & 0x10 != 0 {
            metadata.exif.set("AVI:HasIndex", AttrValue::Bool(true));
        }
        if flags & 0x20 != 0 {
            metadata.exif.set("AVI:MustUseIndex", AttrValue::Bool(true));
        }
        if flags & 0x100 != 0 {
            metadata.exif.set("AVI:IsInterleaved", AttrValue::Bool(true));
        }

        // Duration
        if microsec_per_frame > 0 && total_frames > 0 {
            let duration_secs = (total_frames as f64 * microsec_per_frame as f64) / 1_000_000.0;
            metadata.exif.set("AVI:Duration", AttrValue::Float(duration_secs as f32));
        }

        // Skip remaining
        let remaining = size.saturating_sub(56);
        if remaining > 0 {
            reader.seek(SeekFrom::Current(remaining as i64))?;
        }

        Ok(())
    }

    /// Parse stream header list (strl).
    fn parse_strl(&self, reader: &mut dyn ReadSeek, end_pos: u64, metadata: &mut Metadata) -> Result<()> {
        while reader.stream_position()? < end_pos {
            let chunk_start = reader.stream_position()?;

            let Some((chunk_id, chunk_size)) = crate::riff::read_chunk_header(reader) else {
                break;
            };

            match &chunk_id {
                b"strh" => {
                    self.parse_strh(reader, chunk_size, metadata)?;
                }
                b"strf" => {
                    // Stream format - skip for now
                    reader.seek(SeekFrom::Current(chunk_size as i64))?;
                }
                b"strn" => {
                    // Stream name
                    if chunk_size > 0 && chunk_size < 1024 {
                        let mut name = vec![0u8; chunk_size as usize];
                        reader.read_exact(&mut name)?;
                        let name_str = String::from_utf8_lossy(&name)
                            .trim_end_matches('\0')
                            .to_string();
                        if !name_str.is_empty() {
                            metadata.exif.set("AVI:StreamName", AttrValue::Str(name_str));
                        }
                    } else {
                        reader.seek(SeekFrom::Current(chunk_size as i64))?;
                    }
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

    /// Parse stream header (strh).
    fn parse_strh(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if size < 48 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(());
        }

        let mut strh = [0u8; 48];
        reader.read_exact(&mut strh)?;

        let fcc_type = String::from_utf8_lossy(&strh[0..4]).to_string();
        let fcc_handler = String::from_utf8_lossy(&strh[4..8]).to_string();

        match fcc_type.trim() {
            "vids" => {
                let codec = fcc_handler.trim();
                if !codec.is_empty() && codec != "\0\0\0\0" {
                    metadata.exif.set("AVI:VideoCodec", AttrValue::Str(codec.trim_end_matches('\0').to_string()));
                }
            }
            "auds" => {
                let codec = fcc_handler.trim();
                if !codec.is_empty() && codec != "\0\0\0\0" {
                    metadata.exif.set("AVI:AudioCodec", AttrValue::Str(codec.trim_end_matches('\0').to_string()));
                }
            }
            _ => {}
        }

        // Skip remaining
        let remaining = size.saturating_sub(48);
        if remaining > 0 {
            reader.seek(SeekFrom::Current(remaining as i64))?;
        }

        Ok(())
    }


    /// Parse EXIF chunk.
    fn parse_exif_chunk(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
        if !(8..=10 * 1024 * 1024).contains(&size) {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(());
        }

        let mut exif_data = vec![0u8; size as usize];
        reader.read_exact(&mut exif_data)?;

        // Find TIFF header (II or MM)
        let tiff_start = self.find_tiff_header(&exif_data);
        if tiff_start >= exif_data.len() || exif_data.len() - tiff_start < 8 {
            return Ok(());
        }

        let tiff_data = &exif_data[tiff_start..];
        self.parse_tiff_exif(tiff_data, metadata)?;

        Ok(())
    }

    /// Parse XMP chunk.
    fn parse_xmp_chunk(&self, reader: &mut dyn ReadSeek, size: u64, metadata: &mut Metadata) -> Result<()> {
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

    /// Find TIFF header in EXIF data.
    fn find_tiff_header(&self, data: &[u8]) -> usize {
        for i in 0..data.len().saturating_sub(4) {
            let marker = &data[i..i + 2];
            if (marker == b"II" || marker == b"MM") && data.len() > i + 3 {
                let magic = if marker == b"MM" {
                    u16::from_be_bytes([data[i + 2], data[i + 3]])
                } else {
                    u16::from_le_bytes([data[i + 2], data[i + 3]])
                };
                if magic == 42 {
                    return i;
                }
            }
        }
        0
    }

    /// Parse TIFF-format EXIF data.
    fn parse_tiff_exif(&self, tiff_data: &[u8], metadata: &mut Metadata) -> Result<()> {
        let byte_order = match ByteOrder::from_marker([tiff_data[0], tiff_data[1]]) {
            Ok(bo) => bo,
            Err(_) => return Ok(()),
        };

        let reader = IfdReader::new(tiff_data, byte_order, 0);
        let ifd0_offset = match reader.parse_header() {
            Ok(o) => o,
            Err(_) => return Ok(()),
        };

        let (entries, _next_ifd) = match reader.read_ifd(ifd0_offset) {
            Ok(e) => e,
            Err(_) => return Ok(()),
        };

        // Find Make for MakerNotes vendor detection
        let mut vendor = makernotes::Vendor::Unknown;
        for entry in &entries {
            if entry.tag == 0x010F {
                if let RawValue::String(make) = &entry.value {
                    vendor = makernotes::Vendor::from_make(make);
                }
                break;
            }
        }

        // Convert IFD0 entries
        for entry in &entries {
            if let Some(name) = lookup_ifd0(entry.tag) {
                metadata.exif.set(name, entry_to_attr(entry));
            }

            match entry.tag {
                0x8769 => {
                    // ExifIFD pointer
                    if let Some(offset) = entry.value.as_u32() {
                        if let Ok((exif_entries, _)) = reader.read_ifd(offset) {
                            for e in &exif_entries {
                                if e.tag == 0x927C {
                                    // MakerNotes
                                    if let RawValue::Undefined(bytes) = &e.value {
                                        if let Some(mn_data) = makernotes::parse(bytes, vendor, byte_order) {
                                            for (key, val) in mn_data.iter() {
                                                metadata.exif.set(key.clone(), val.clone());
                                            }
                                        }
                                    }
                                } else if let Some(name) = lookup_exif_subifd(e.tag) {
                                    metadata.exif.set(name, entry_to_attr(e));
                                }
                            }
                        }
                    }
                }
                0x8825 => {
                    // GPS IFD pointer
                    if let Some(offset) = entry.value.as_u32() {
                        if let Ok((gps_entries, _)) = reader.read_ifd(offset) {
                            for e in &gps_entries {
                                if let Some(name) = lookup_gps(e.tag) {
                                    metadata.exif.set(name, entry_to_attr(e));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_can_parse() {
        let parser = AviParser;
        let mut header = vec![0u8; 16];
        header[0..4].copy_from_slice(b"RIFF");
        header[4..8].copy_from_slice(&100u32.to_le_bytes());
        header[8..12].copy_from_slice(b"AVI ");

        assert!(parser.can_parse(&header));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = AviParser;
        assert!(!parser.can_parse(b"RIFF\x00\x00\x00\x00WAVE")); // WAV, not AVI
        assert!(!parser.can_parse(b"\x89PNG"));
        assert!(!parser.can_parse(&[]));
    }

    #[test]
    fn test_parse_minimal() {
        let mut data = Vec::new();

        // RIFF header
        data.extend_from_slice(b"RIFF");
        let size_pos = data.len();
        data.extend_from_slice(&0u32.to_le_bytes()); // placeholder
        data.extend_from_slice(b"AVI ");

        // LIST hdrl
        data.extend_from_slice(b"LIST");
        let hdrl_size_pos = data.len();
        data.extend_from_slice(&0u32.to_le_bytes()); // placeholder
        data.extend_from_slice(b"hdrl");

        // avih chunk
        data.extend_from_slice(b"avih");
        data.extend_from_slice(&56u32.to_le_bytes());
        // microsec_per_frame = 33333 (~30fps)
        data.extend_from_slice(&33333u32.to_le_bytes());
        data.extend_from_slice(&0u32.to_le_bytes()); // max_bytes_per_sec
        data.extend_from_slice(&0u32.to_le_bytes()); // padding
        data.extend_from_slice(&0x10u32.to_le_bytes()); // flags (has index)
        data.extend_from_slice(&300u32.to_le_bytes()); // total frames
        data.extend_from_slice(&0u32.to_le_bytes()); // initial frames
        data.extend_from_slice(&2u32.to_le_bytes()); // streams
        data.extend_from_slice(&0u32.to_le_bytes()); // suggested buffer
        data.extend_from_slice(&640u32.to_le_bytes()); // width
        data.extend_from_slice(&480u32.to_le_bytes()); // height
        data.extend_from_slice(&[0u8; 16]); // reserved

        // Update sizes
        let hdrl_size = data.len() - hdrl_size_pos - 4;
        data[hdrl_size_pos..hdrl_size_pos + 4].copy_from_slice(&(hdrl_size as u32).to_le_bytes());

        let riff_size = data.len() - 8;
        data[size_pos..size_pos + 4].copy_from_slice(&(riff_size as u32).to_le_bytes());

        let parser = AviParser;
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("File:FileType"), Some("AVI"));
        assert_eq!(meta.exif.get_u32("File:ImageWidth"), Some(640));
        assert_eq!(meta.exif.get_u32("File:ImageHeight"), Some(480));
        assert_eq!(meta.exif.get_u32("AVI:TotalFrames"), Some(300));
        assert_eq!(meta.exif.get_u32("AVI:StreamCount"), Some(2));
    }

    #[test]
    fn test_parse_with_info() {
        let mut data = Vec::new();

        // RIFF header
        data.extend_from_slice(b"RIFF");
        let size_pos = data.len();
        data.extend_from_slice(&0u32.to_le_bytes());
        data.extend_from_slice(b"AVI ");

        // LIST INFO
        data.extend_from_slice(b"LIST");
        let info_size_pos = data.len();
        data.extend_from_slice(&0u32.to_le_bytes());
        data.extend_from_slice(b"INFO");

        // INAM (title)
        data.extend_from_slice(b"INAM");
        data.extend_from_slice(&10u32.to_le_bytes());
        data.extend_from_slice(b"Test Video");

        // IART (artist)
        data.extend_from_slice(b"IART");
        data.extend_from_slice(&8u32.to_le_bytes());
        data.extend_from_slice(b"John Doe");

        // Update sizes
        let info_size = data.len() - info_size_pos - 4;
        data[info_size_pos..info_size_pos + 4].copy_from_slice(&(info_size as u32).to_le_bytes());

        let riff_size = data.len() - 8;
        data[size_pos..size_pos + 4].copy_from_slice(&(riff_size as u32).to_le_bytes());

        let parser = AviParser;
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("RIFF:Title"), Some("Test Video"));
        assert_eq!(meta.exif.get_str("RIFF:Artist"), Some("John Doe"));
    }
}

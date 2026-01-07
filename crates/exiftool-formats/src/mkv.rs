//! MKV/WebM format parser.
//!
//! Matroska (MKV) and WebM use EBML (Extensible Binary Meta Language).
//! EBML is a binary format similar to XML structure.
//!
//! Key elements:
//! - EBML Header (0x1A45DFA3): document type, version
//! - Segment (0x18538067): contains all data
//! - Info (0x1549A966): duration, title, muxing app
//! - Tracks (0x1654AE6B): video/audio track info
//! - Tags (0x1254C367): metadata tags

use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

// EBML Element IDs (multi-byte)
const EBML_HEADER: u32 = 0x1A45DFA3;
const EBML_DOC_TYPE: u32 = 0x4282;
const EBML_DOC_TYPE_VERSION: u32 = 0x4287;

const SEGMENT: u32 = 0x18538067;
const INFO: u32 = 0x1549A966;
const TRACKS: u32 = 0x1654AE6B;
const TAGS: u32 = 0x1254C367;
const CHAPTERS: u32 = 0x1043A770;
const ATTACHMENTS: u32 = 0x1941A469;

// Chapter element IDs
const EDITION_ENTRY: u32 = 0x45B9;
const CHAPTER_ATOM: u32 = 0xB6;
const CHAPTER_TIME_START: u32 = 0x91;
const CHAPTER_TIME_END: u32 = 0x92;
const CHAPTER_DISPLAY: u32 = 0x80;
const CHAP_STRING: u32 = 0x85;
const CHAP_LANGUAGE: u32 = 0x437C;
const CHAPTER_UID: u32 = 0x73C4;

// Attachment element IDs
const ATTACHED_FILE: u32 = 0x61A7;
const FILE_NAME: u32 = 0x466E;
const FILE_MIME_TYPE: u32 = 0x4660;
const FILE_DATA: u32 = 0x465C;
const FILE_UID: u32 = 0x46AE;
const FILE_DESCRIPTION: u32 = 0x467E;

// Info element IDs
const TIMECODE_SCALE: u32 = 0x2AD7B1;
const DURATION: u32 = 0x4489;
const MUXING_APP: u32 = 0x4D80;
const WRITING_APP: u32 = 0x5741;
const DATE_UTC: u32 = 0x4461;
const TITLE: u32 = 0x7BA9;

// Track element IDs
const TRACK_ENTRY: u32 = 0xAE;
const TRACK_TYPE: u32 = 0x83;
const CODEC_ID: u32 = 0x86;
const CODEC_NAME: u32 = 0x258688;
const VIDEO: u32 = 0xE0;
const AUDIO: u32 = 0xE1;
const PIXEL_WIDTH: u32 = 0xB0;
const PIXEL_HEIGHT: u32 = 0xBA;
const DISPLAY_WIDTH: u32 = 0x54B0;
const DISPLAY_HEIGHT: u32 = 0x54BA;
const SAMPLING_FREQ: u32 = 0xB5;
const CHANNELS: u32 = 0x9F;
const BIT_DEPTH: u32 = 0x6264;

// Tag element IDs
const TAG: u32 = 0x7373;
const SIMPLE_TAG: u32 = 0x67C8;
const TAG_NAME: u32 = 0x45A3;
const TAG_STRING: u32 = 0x4487;

/// MKV/WebM parser.
pub struct MkvParser;

impl FormatParser for MkvParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 4 {
            return false;
        }
        // EBML header element ID: 0x1A45DFA3
        header[0] == 0x1A && header[1] == 0x45 && header[2] == 0xDF && header[3] == 0xA3
    }

    fn format_name(&self) -> &'static str {
        "MKV"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["mkv", "mka", "mks", "mk3d", "webm"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("MKV");

        // Parse EBML header
        let (id, size) = self.read_element(reader)?;
        if id != EBML_HEADER {
            return Err(Error::InvalidStructure("Missing EBML header".into()));
        }

        let header_end = reader.stream_position()? + size;
        self.parse_ebml_header(reader, header_end, &mut metadata)?;

        // Parse Segment
        let (id, size) = self.read_element(reader)?;
        if id != SEGMENT {
            return Err(Error::InvalidStructure("Missing Segment".into()));
        }

        let segment_end = if size == u64::MAX {
            // Unknown size - read to EOF
            reader.seek(SeekFrom::End(0))?
        } else {
            reader.stream_position()? + size
        };

        self.parse_segment(reader, segment_end, &mut metadata)?;

        Ok(metadata)
    }
}

impl MkvParser {
    /// Read EBML variable-length integer (VINT).
    fn read_vint(&self, reader: &mut dyn ReadSeek) -> Result<u64> {
        let mut first = [0u8; 1];
        reader.read_exact(&mut first)?;

        let length = first[0].leading_zeros() + 1;
        if length > 8 {
            return Err(Error::InvalidStructure("Invalid VINT".into()));
        }

        let mask = (1u8 << (8 - length)) - 1;
        let mut value = (first[0] & mask) as u64;

        for _ in 1..length {
            let mut byte = [0u8; 1];
            reader.read_exact(&mut byte)?;
            value = (value << 8) | byte[0] as u64;
        }

        Ok(value)
    }

    /// Read element ID (VINT without masking the length bits for ID).
    fn read_element_id(&self, reader: &mut dyn ReadSeek) -> Result<u32> {
        let mut first = [0u8; 1];
        reader.read_exact(&mut first)?;

        let length = first[0].leading_zeros() + 1;
        if length > 4 {
            return Err(Error::InvalidStructure("Invalid element ID".into()));
        }

        let mut value = first[0] as u32;

        for _ in 1..length {
            let mut byte = [0u8; 1];
            reader.read_exact(&mut byte)?;
            value = (value << 8) | byte[0] as u32;
        }

        Ok(value)
    }

    /// Read element (ID + size).
    fn read_element(&self, reader: &mut dyn ReadSeek) -> Result<(u32, u64)> {
        let id = self.read_element_id(reader)?;
        let size = self.read_vint(reader)?;

        // Check for unknown size (all 1s)
        let unknown_size = match size.leading_zeros() {
            57 => size == 0x7F,                    // 1-byte VINT
            50 => size == 0x3FFF,                  // 2-byte
            43 => size == 0x1FFFFF,                // 3-byte
            36 => size == 0x0FFFFFFF,              // 4-byte
            _ => false,
        };

        if unknown_size {
            Ok((id, u64::MAX))
        } else {
            Ok((id, size))
        }
    }

    /// Parse EBML header.
    fn parse_ebml_header(&self, reader: &mut dyn ReadSeek, end: u64, metadata: &mut Metadata) -> Result<()> {
        while reader.stream_position()? < end {
            let (id, size) = self.read_element(reader)?;

            match id {
                EBML_DOC_TYPE => {
                    let doc_type = self.read_string(reader, size)?;
                    metadata.exif.set("EBML:DocType", AttrValue::Str(doc_type.clone()));

                    // Set format based on doc type
                    match doc_type.as_str() {
                        "webm" => {
                            metadata.format = "WebM";
                            metadata.set_file_type("WebM", "video/webm");
                        }
                        "matroska" => {
                            metadata.set_file_type("MKV", "video/x-matroska");
                        }
                        _ => {
                            metadata.set_file_type("MKV", "");
                        }
                    }
                }
                EBML_DOC_TYPE_VERSION => {
                    let version = self.read_uint(reader, size)?;
                    metadata.exif.set("EBML:DocTypeVersion", AttrValue::UInt(version as u32));
                }
                _ => {
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
            }
        }

        Ok(())
    }

    /// Parse Segment.
    fn parse_segment(&self, reader: &mut dyn ReadSeek, end: u64, metadata: &mut Metadata) -> Result<()> {
        // Track parsing state
        let mut timecode_scale: u64 = 1_000_000; // Default: 1ms
        let mut duration_raw: Option<f64> = None;

        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            let element_end = if size == u64::MAX {
                end
            } else {
                reader.stream_position()? + size
            };

            match id {
                INFO => {
                    let result = self.parse_info(reader, element_end, metadata)?;
                    timecode_scale = result.0;
                    duration_raw = result.1;
                }
                TRACKS => {
                    self.parse_tracks(reader, element_end, metadata)?;
                }
                TAGS => {
                    self.parse_tags(reader, element_end, metadata)?;
                }
                CHAPTERS => {
                    self.parse_chapters(reader, element_end, metadata)?;
                }
                ATTACHMENTS => {
                    self.parse_attachments(reader, element_end, metadata)?;
                }
                _ => {
                    // Skip other elements (Clusters, Chapters, etc.)
                    if size != u64::MAX {
                        reader.seek(SeekFrom::Start(element_end))?;
                    } else {
                        break;
                    }
                }
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        // Calculate duration in seconds
        if let Some(raw) = duration_raw {
            let duration_secs = (raw * timecode_scale as f64) / 1_000_000_000.0;
            metadata.exif.set("MKV:Duration", AttrValue::Float(duration_secs as f32));
        }

        Ok(())
    }

    /// Parse Info element. Returns (timecode_scale, duration).
    fn parse_info(&self, reader: &mut dyn ReadSeek, end: u64, metadata: &mut Metadata) -> Result<(u64, Option<f64>)> {
        let mut timecode_scale: u64 = 1_000_000;
        let mut duration: Option<f64> = None;

        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            match id {
                TIMECODE_SCALE => {
                    timecode_scale = self.read_uint(reader, size)?;
                    metadata.exif.set("MKV:TimecodeScale", AttrValue::UInt(timecode_scale as u32));
                }
                DURATION => {
                    duration = Some(self.read_float(reader, size)?);
                }
                MUXING_APP => {
                    let app = self.read_string(reader, size)?;
                    metadata.exif.set("MKV:MuxingApp", AttrValue::Str(app));
                }
                WRITING_APP => {
                    let app = self.read_string(reader, size)?;
                    metadata.exif.set("MKV:WritingApp", AttrValue::Str(app));
                }
                DATE_UTC => {
                    // Nanoseconds since 2001-01-01 00:00:00 UTC
                    let nanos = self.read_int(reader, size)?;
                    // Convert to Unix timestamp (2001-01-01 = 978307200)
                    let unix_secs = 978307200i64 + (nanos / 1_000_000_000);
                    metadata.exif.set("MKV:DateUTC", AttrValue::Int(unix_secs as i32));
                }
                TITLE => {
                    let title = self.read_string(reader, size)?;
                    metadata.exif.set("MKV:Title", AttrValue::Str(title));
                }
                _ => {
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        Ok((timecode_scale, duration))
    }

    /// Parse Tracks element.
    fn parse_tracks(&self, reader: &mut dyn ReadSeek, end: u64, metadata: &mut Metadata) -> Result<()> {
        let mut video_count = 0u32;
        let mut audio_count = 0u32;

        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            if id == TRACK_ENTRY {
                let track_end = reader.stream_position()? + size;
                let track_type = self.parse_track_entry(reader, track_end, metadata)?;
                match track_type {
                    1 => video_count += 1,
                    2 => audio_count += 1,
                    _ => {}
                }
            } else {
                reader.seek(SeekFrom::Current(size as i64))?;
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        if video_count > 0 {
            metadata.exif.set("MKV:VideoTrackCount", AttrValue::UInt(video_count));
        }
        if audio_count > 0 {
            metadata.exif.set("MKV:AudioTrackCount", AttrValue::UInt(audio_count));
        }

        Ok(())
    }

    /// Parse TrackEntry. Returns track type (1=video, 2=audio, etc.).
    fn parse_track_entry(&self, reader: &mut dyn ReadSeek, end: u64, metadata: &mut Metadata) -> Result<u8> {
        let mut track_type: u8 = 0;

        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            match id {
                TRACK_TYPE => {
                    track_type = self.read_uint(reader, size)? as u8;
                }
                CODEC_ID => {
                    let codec = self.read_string(reader, size)?;
                    match track_type {
                        1 => metadata.exif.set("MKV:VideoCodec", AttrValue::Str(codec)),
                        2 => metadata.exif.set("MKV:AudioCodec", AttrValue::Str(codec)),
                        _ => {}
                    }
                }
                CODEC_NAME => {
                    let name = self.read_string(reader, size)?;
                    metadata.exif.set("MKV:CodecName", AttrValue::Str(name));
                }
                VIDEO => {
                    let video_end = reader.stream_position()? + size;
                    self.parse_video(reader, video_end, metadata)?;
                }
                AUDIO => {
                    let audio_end = reader.stream_position()? + size;
                    self.parse_audio(reader, audio_end, metadata)?;
                }
                _ => {
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        Ok(track_type)
    }

    /// Parse Video element.
    fn parse_video(&self, reader: &mut dyn ReadSeek, end: u64, metadata: &mut Metadata) -> Result<()> {
        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            match id {
                PIXEL_WIDTH => {
                    let w = self.read_uint(reader, size)?;
                    metadata.exif.set("File:ImageWidth", AttrValue::UInt(w as u32));
                }
                PIXEL_HEIGHT => {
                    let h = self.read_uint(reader, size)?;
                    metadata.exif.set("File:ImageHeight", AttrValue::UInt(h as u32));
                }
                DISPLAY_WIDTH => {
                    let w = self.read_uint(reader, size)?;
                    metadata.exif.set("MKV:DisplayWidth", AttrValue::UInt(w as u32));
                }
                DISPLAY_HEIGHT => {
                    let h = self.read_uint(reader, size)?;
                    metadata.exif.set("MKV:DisplayHeight", AttrValue::UInt(h as u32));
                }
                _ => {
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        Ok(())
    }

    /// Parse Audio element.
    fn parse_audio(&self, reader: &mut dyn ReadSeek, end: u64, metadata: &mut Metadata) -> Result<()> {
        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            match id {
                SAMPLING_FREQ => {
                    let freq = self.read_float(reader, size)?;
                    metadata.exif.set("MKV:AudioSampleRate", AttrValue::Float(freq as f32));
                }
                CHANNELS => {
                    let ch = self.read_uint(reader, size)?;
                    metadata.exif.set("MKV:AudioChannels", AttrValue::UInt(ch as u32));
                }
                BIT_DEPTH => {
                    let bits = self.read_uint(reader, size)?;
                    metadata.exif.set("MKV:AudioBitDepth", AttrValue::UInt(bits as u32));
                }
                _ => {
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        Ok(())
    }

    /// Parse Tags element.
    fn parse_tags(&self, reader: &mut dyn ReadSeek, end: u64, metadata: &mut Metadata) -> Result<()> {
        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            if id == TAG {
                let tag_end = reader.stream_position()? + size;
                self.parse_tag(reader, tag_end, metadata)?;
            } else {
                reader.seek(SeekFrom::Current(size as i64))?;
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        Ok(())
    }

    /// Parse Tag element.
    fn parse_tag(&self, reader: &mut dyn ReadSeek, end: u64, metadata: &mut Metadata) -> Result<()> {
        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            if id == SIMPLE_TAG {
                let simple_end = reader.stream_position()? + size;
                self.parse_simple_tag(reader, simple_end, metadata)?;
            } else {
                reader.seek(SeekFrom::Current(size as i64))?;
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        Ok(())
    }

    /// Parse SimpleTag element.
    fn parse_simple_tag(&self, reader: &mut dyn ReadSeek, end: u64, metadata: &mut Metadata) -> Result<()> {
        let mut name: Option<String> = None;
        let mut value: Option<String> = None;

        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            match id {
                TAG_NAME => {
                    name = Some(self.read_string(reader, size)?);
                }
                TAG_STRING => {
                    value = Some(self.read_string(reader, size)?);
                }
                _ => {
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        if let (Some(n), Some(v)) = (name, value) {
            metadata.exif.set(format!("MKV:{}", n), AttrValue::Str(v));
        }

        Ok(())
    }

    /// Read unsigned integer.
    fn read_uint(&self, reader: &mut dyn ReadSeek, size: u64) -> Result<u64> {
        if size > 8 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(0);
        }

        let mut buf = [0u8; 8];
        let start = 8 - size as usize;
        reader.read_exact(&mut buf[start..])?;

        Ok(u64::from_be_bytes(buf))
    }

    /// Read signed integer.
    fn read_int(&self, reader: &mut dyn ReadSeek, size: u64) -> Result<i64> {
        if size > 8 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(0);
        }

        let mut buf = [0u8; 8];
        let start = 8 - size as usize;
        reader.read_exact(&mut buf[start..])?;

        // Sign extend
        let val = i64::from_be_bytes(buf);
        let shift = (8 - size) * 8;
        Ok((val << shift) >> shift)
    }

    /// Read float (4 or 8 bytes).
    fn read_float(&self, reader: &mut dyn ReadSeek, size: u64) -> Result<f64> {
        match size {
            4 => {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                Ok(f32::from_be_bytes(buf) as f64)
            }
            8 => {
                let mut buf = [0u8; 8];
                reader.read_exact(&mut buf)?;
                Ok(f64::from_be_bytes(buf))
            }
            _ => {
                reader.seek(SeekFrom::Current(size as i64))?;
                Ok(0.0)
            }
        }
    }

    /// Parse Chapters element.
    fn parse_chapters(&self, reader: &mut dyn ReadSeek, end: u64, metadata: &mut Metadata) -> Result<()> {
        let mut chapter_count = 0u32;
        let mut chapters: Vec<String> = Vec::new();

        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            if id == EDITION_ENTRY {
                let edition_end = reader.stream_position()? + size;
                let edition_chapters = self.parse_edition(reader, edition_end)?;
                chapter_count += edition_chapters.len() as u32;
                chapters.extend(edition_chapters);
            } else {
                reader.seek(SeekFrom::Current(size as i64))?;
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        if chapter_count > 0 {
            metadata.exif.set("MKV:ChapterCount", AttrValue::UInt(chapter_count));
        }
        if !chapters.is_empty() {
            // Store first few chapter titles
            for (i, title) in chapters.iter().take(10).enumerate() {
                metadata.exif.set(format!("MKV:Chapter{}Title", i + 1), AttrValue::Str(title.clone()));
            }
        }

        Ok(())
    }

    /// Parse EditionEntry. Returns chapter titles.
    fn parse_edition(&self, reader: &mut dyn ReadSeek, end: u64) -> Result<Vec<String>> {
        let mut chapters = Vec::new();

        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            if id == CHAPTER_ATOM {
                let atom_end = reader.stream_position()? + size;
                if let Some(title) = self.parse_chapter_atom(reader, atom_end)? {
                    chapters.push(title);
                }
            } else {
                reader.seek(SeekFrom::Current(size as i64))?;
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        Ok(chapters)
    }

    /// Parse ChapterAtom. Returns chapter title if found.
    fn parse_chapter_atom(&self, reader: &mut dyn ReadSeek, end: u64) -> Result<Option<String>> {
        let mut title: Option<String> = None;

        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            match id {
                CHAPTER_DISPLAY => {
                    let display_end = reader.stream_position()? + size;
                    title = self.parse_chapter_display(reader, display_end)?;
                }
                CHAPTER_TIME_START | CHAPTER_TIME_END | CHAPTER_UID => {
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
                _ => {
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        Ok(title)
    }

    /// Parse ChapterDisplay. Returns chapter string.
    fn parse_chapter_display(&self, reader: &mut dyn ReadSeek, end: u64) -> Result<Option<String>> {
        let mut chap_string: Option<String> = None;

        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            match id {
                CHAP_STRING => {
                    chap_string = Some(self.read_string(reader, size)?);
                }
                CHAP_LANGUAGE => {
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
                _ => {
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        Ok(chap_string)
    }

    /// Parse Attachments element.
    fn parse_attachments(&self, reader: &mut dyn ReadSeek, end: u64, metadata: &mut Metadata) -> Result<()> {
        let mut attachment_count = 0u32;
        let mut attachments: Vec<(String, String, u64)> = Vec::new(); // (name, mime, size)

        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            if id == ATTACHED_FILE {
                let file_end = reader.stream_position()? + size;
                if let Some(attachment) = self.parse_attached_file(reader, file_end)? {
                    attachments.push(attachment);
                    attachment_count += 1;
                }
            } else {
                reader.seek(SeekFrom::Current(size as i64))?;
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        if attachment_count > 0 {
            metadata.exif.set("MKV:AttachmentCount", AttrValue::UInt(attachment_count));
        }
        // Store first few attachment names
        for (i, (name, mime, size)) in attachments.iter().take(10).enumerate() {
            metadata.exif.set(format!("MKV:Attachment{}Name", i + 1), AttrValue::Str(name.clone()));
            metadata.exif.set(format!("MKV:Attachment{}MIMEType", i + 1), AttrValue::Str(mime.clone()));
            metadata.exif.set(format!("MKV:Attachment{}Size", i + 1), AttrValue::UInt(*size as u32));
        }

        Ok(())
    }

    /// Parse AttachedFile. Returns (name, mime_type, size).
    fn parse_attached_file(&self, reader: &mut dyn ReadSeek, end: u64) -> Result<Option<(String, String, u64)>> {
        let mut name: Option<String> = None;
        let mut mime: Option<String> = None;
        let mut data_size: u64 = 0;

        while reader.stream_position()? < end {
            let pos = reader.stream_position()?;
            let (id, size) = match self.read_element(reader) {
                Ok(e) => e,
                Err(_) => break,
            };

            match id {
                FILE_NAME => {
                    name = Some(self.read_string(reader, size)?);
                }
                FILE_MIME_TYPE => {
                    mime = Some(self.read_string(reader, size)?);
                }
                FILE_DATA => {
                    data_size = size;
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
                FILE_DESCRIPTION | FILE_UID => {
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
                _ => {
                    reader.seek(SeekFrom::Current(size as i64))?;
                }
            }

            if reader.stream_position()? <= pos {
                break;
            }
        }

        if let (Some(n), Some(m)) = (name, mime) {
            Ok(Some((n, m, data_size)))
        } else {
            Ok(None)
        }
    }

    /// Read UTF-8 string.
    fn read_string(&self, reader: &mut dyn ReadSeek, size: u64) -> Result<String> {
        if size > 64 * 1024 {
            reader.seek(SeekFrom::Current(size as i64))?;
            return Ok(String::new());
        }

        let mut buf = vec![0u8; size as usize];
        reader.read_exact(&mut buf)?;

        Ok(String::from_utf8_lossy(&buf)
            .trim_end_matches('\0')
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn write_vint(value: u64, length: u8) -> Vec<u8> {
        let mut result = vec![0u8; length as usize];
        let marker = 1u8 << (8 - length);

        for i in 0..length as usize {
            let shift = (length as usize - 1 - i) * 8;
            result[i] = ((value >> shift) & 0xFF) as u8;
        }
        result[0] |= marker;

        result
    }

    fn write_element(id: u32, data: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();

        // Write ID
        if id > 0xFFFFFF {
            result.extend_from_slice(&id.to_be_bytes());
        } else if id > 0xFFFF {
            result.extend_from_slice(&id.to_be_bytes()[1..]);
        } else if id > 0xFF {
            result.extend_from_slice(&id.to_be_bytes()[2..]);
        } else {
            result.push(id as u8);
        }

        // Write size as VINT
        let size = data.len() as u64;
        if size < 0x7F {
            result.push(0x80 | size as u8);
        } else if size < 0x3FFF {
            result.extend_from_slice(&write_vint(size, 2));
        } else {
            result.extend_from_slice(&write_vint(size, 4));
        }

        result.extend_from_slice(data);
        result
    }

    #[test]
    fn test_can_parse() {
        let parser = MkvParser;
        assert!(parser.can_parse(&[0x1A, 0x45, 0xDF, 0xA3, 0x00]));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = MkvParser;
        assert!(!parser.can_parse(b"RIFF"));
        assert!(!parser.can_parse(&[]));
    }

    #[test]
    fn test_parse_minimal() {
        let mut data = Vec::new();

        // EBML header
        let mut header_content = Vec::new();
        header_content.extend(write_element(EBML_DOC_TYPE, b"matroska"));
        header_content.extend(write_element(EBML_DOC_TYPE_VERSION, &[1]));

        data.extend(write_element(EBML_HEADER, &header_content));

        // Segment with Info
        let mut info_content = Vec::new();
        info_content.extend(write_element(TITLE, b"Test Video"));
        info_content.extend(write_element(MUXING_APP, b"TestApp 1.0"));

        let mut segment_content = Vec::new();
        segment_content.extend(write_element(INFO, &info_content));

        data.extend(write_element(SEGMENT, &segment_content));

        let parser = MkvParser;
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("EBML:DocType"), Some("matroska"));
        assert_eq!(meta.exif.get_str("MKV:Title"), Some("Test Video"));
        assert_eq!(meta.exif.get_str("MKV:MuxingApp"), Some("TestApp 1.0"));
    }

    #[test]
    fn test_parse_webm() {
        let mut data = Vec::new();

        // EBML header with webm doc type
        let mut header_content = Vec::new();
        header_content.extend(write_element(EBML_DOC_TYPE, b"webm"));

        data.extend(write_element(EBML_HEADER, &header_content));
        data.extend(write_element(SEGMENT, &[]));

        let parser = MkvParser;
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "WebM");
        assert_eq!(meta.exif.get_str("File:FileType"), Some("WebM"));
    }
}

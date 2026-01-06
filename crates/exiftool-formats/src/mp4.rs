//! MP4/MOV format parser.
//!
//! MP4 and MOV (QuickTime) use ISOBMFF (ISO Base Media File Format).
//! Structure:
//! - ftyp box: file type (mp41, mp42, isom, qt, M4V, M4A, etc.)
//! - moov box: movie container
//!   - mvhd: movie header (duration, timescale, creation date)
//!   - trak: track container (one per audio/video stream)
//!     - tkhd: track header
//!     - mdia: media container
//!       - mdhd: media header
//!       - hdlr: handler type (vide, soun, etc.)
//!       - minf: media info
//!   - udta: user data
//!     - meta: metadata (XMP, etc.)
//! - mdat box: media data (actual audio/video)

use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// MP4/MOV brand identifiers in ftyp box.
const MP4_BRANDS: &[&[u8; 4]] = &[
    b"mp41", // MP4 v1
    b"mp42", // MP4 v2
    b"isom", // ISO Base Media
    b"iso2", // ISO Base Media v2
    b"iso3", // ISO Base Media v3
    b"iso4", // ISO Base Media v4
    b"iso5", // ISO Base Media v5
    b"iso6", // ISO Base Media v6
    b"avc1", // AVC (H.264)
    b"hvc1", // HEVC (H.265)
    b"mp71", // MPEG-7
    b"M4V ", // Apple M4V (video)
    b"M4A ", // Apple M4A (audio)
    b"M4B ", // Apple M4B (audiobook)
    b"M4P ", // Apple M4P (protected)
    b"qt  ", // QuickTime
    b"3gp4", // 3GPP v4
    b"3gp5", // 3GPP v5
    b"3gp6", // 3GPP v6
    b"3g2a", // 3GPP2
    b"mmp4", // Mobile MP4
    b"f4v ", // Flash Video
    b"dash", // DASH
];

/// MP4/MOV format parser.
pub struct Mp4Parser;

impl FormatParser for Mp4Parser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 12 {
            return false;
        }

        // First 4 bytes: box size, next 4: "ftyp"
        if &header[4..8] != b"ftyp" {
            return false;
        }

        // Check major brand (bytes 8-11)
        let brand = &header[8..12];
        MP4_BRANDS.iter().any(|b| brand == *b)
    }

    fn format_name(&self) -> &'static str {
        "MP4"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["mp4", "m4v", "m4a", "m4b", "m4p", "mov", "3gp", "3g2", "f4v"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("MP4");
        let mut state = ParseState::default();

        // Parse ftyp box first
        reader.seek(SeekFrom::Start(0))?;

        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;

        let ftyp_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;

        if &buf[4..8] != b"ftyp" {
            return Err(Error::InvalidStructure("Missing ftyp box".into()));
        }

        // Read major brand
        let mut brand = [0u8; 4];
        reader.read_exact(&mut brand)?;

        let brand_str = String::from_utf8_lossy(&brand).trim().to_string();
        metadata.exif.set("MajorBrand", AttrValue::Str(brand_str.clone()));

        // Determine format variant
        metadata.format = match brand_str.as_str() {
            "qt" | "qt  " => "MOV",
            "M4V" | "M4V " => "M4V",
            "M4A" | "M4A " => "M4A",
            "M4B" | "M4B " => "M4B",
            "3gp4" | "3gp5" | "3gp6" => "3GP",
            "3g2a" => "3G2",
            "f4v" | "f4v " => "F4V",
            _ => "MP4",
        };

        // Read minor version
        let mut version = [0u8; 4];
        reader.read_exact(&mut version)?;
        let minor_version = u32::from_be_bytes(version);
        metadata.exif.set("MinorVersion", AttrValue::UInt(minor_version));

        // Read compatible brands
        let brands_size = ftyp_size.saturating_sub(16);
        if brands_size > 0 && brands_size < 1024 {
            let mut brands_buf = vec![0u8; brands_size as usize];
            reader.read_exact(&mut brands_buf)?;

            let brands: Vec<String> = brands_buf
                .chunks(4)
                .filter(|c| c.len() == 4)
                .map(|c| String::from_utf8_lossy(c).trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            if !brands.is_empty() {
                metadata.exif.set("CompatibleBrands", AttrValue::Str(brands.join(", ")));
            }
        }

        // Parse remaining boxes
        reader.seek(SeekFrom::Start(ftyp_size))?;
        let file_size = crate::utils::get_file_size(reader)?;
        reader.seek(SeekFrom::Start(ftyp_size))?;

        while reader.stream_position()? < file_size {
            let pos = reader.stream_position()?;

            if reader.read_exact(&mut buf).is_err() {
                break;
            }

            let mut box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];

            // Handle extended size (size=1 means 64-bit size follows)
            if box_size == 1 {
                let mut ext_size = [0u8; 8];
                reader.read_exact(&mut ext_size)?;
                box_size = u64::from_be_bytes(ext_size);
            } else if box_size == 0 {
                // Size 0 means box extends to end of file
                box_size = file_size - pos;
            }

            match &box_type {
                b"moov" => {
                    self.parse_moov_box(reader, pos, box_size, &mut metadata, &mut state)?;
                }
                b"mdat" => {
                    state.mdat_offset = Some(pos + 8);
                    let mdat_size = box_size.saturating_sub(8);
                    metadata.exif.set("MediaDataSize", AttrValue::UInt64(mdat_size));
                }
                b"uuid" => {
                    // UUID extension boxes (XMP often stored here)
                    self.parse_uuid_box(reader, pos, box_size, &mut metadata)?;
                }
                _ => {}
            }

            if box_size == 0 || pos + box_size > file_size {
                break;
            }
            reader.seek(SeekFrom::Start(pos + box_size))?;
        }

        // Set track count
        if state.video_tracks > 0 || state.audio_tracks > 0 {
            metadata.exif.set("VideoTrackCount", AttrValue::UInt(state.video_tracks));
            metadata.exif.set("AudioTrackCount", AttrValue::UInt(state.audio_tracks));
        }

        Ok(metadata)
    }
}

/// Parser state for collecting info across boxes.
#[derive(Default)]
struct ParseState {
    #[allow(dead_code)]
    mdat_offset: Option<u64>,
    video_tracks: u32,
    audio_tracks: u32,
    timescale: u32,
}

impl Mp4Parser {
    /// Parse moov (movie) box and its children.
    fn parse_moov_box(
        &self,
        reader: &mut dyn ReadSeek,
        moov_start: u64,
        moov_size: u64,
        metadata: &mut Metadata,
        state: &mut ParseState,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(moov_start + 8))?;

        let moov_end = moov_start + moov_size;
        let mut buf = [0u8; 8];

        while reader.stream_position()? < moov_end {
            let pos = reader.stream_position()?;

            if reader.read_exact(&mut buf).is_err() {
                break;
            }

            let box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];

            if box_size < 8 || pos + box_size > moov_end {
                break;
            }

            match &box_type {
                b"mvhd" => {
                    self.parse_mvhd_box(reader, pos, box_size, metadata, state)?;
                }
                b"trak" => {
                    self.parse_trak_box(reader, pos, box_size, metadata, state)?;
                }
                b"udta" => {
                    self.parse_udta_box(reader, pos, box_size, metadata)?;
                }
                b"meta" => {
                    self.parse_meta_box(reader, pos, box_size, metadata)?;
                }
                _ => {}
            }

            reader.seek(SeekFrom::Start(pos + box_size))?;
        }

        Ok(())
    }

    /// Parse mvhd (movie header) box.
    fn parse_mvhd_box(
        &self,
        reader: &mut dyn ReadSeek,
        _box_start: u64,
        _box_size: u64,
        metadata: &mut Metadata,
        state: &mut ParseState,
    ) -> Result<()> {
        // Version and flags
        let mut vf = [0u8; 4];
        reader.read_exact(&mut vf)?;
        let version = vf[0];

        if version == 0 {
            // 32-bit timestamps
            let mut buf = [0u8; 20];
            reader.read_exact(&mut buf)?;

            let creation_time = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
            let modification_time = u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]);
            let timescale = u32::from_be_bytes([buf[8], buf[9], buf[10], buf[11]]);
            let duration = u32::from_be_bytes([buf[12], buf[13], buf[14], buf[15]]);

            state.timescale = timescale;

            // Convert timestamps (seconds since 1904-01-01)
            if creation_time > 0 {
                metadata.exif.set("CreationTime", AttrValue::Str(mac_time_to_string(creation_time as u64)));
            }
            if modification_time > 0 {
                metadata.exif.set("ModificationTime", AttrValue::Str(mac_time_to_string(modification_time as u64)));
            }

            metadata.exif.set("TimeScale", AttrValue::UInt(timescale));

            // Calculate duration in seconds
            if timescale > 0 {
                let duration_secs = duration as f64 / timescale as f64;
                metadata.exif.set("Duration", AttrValue::Str(format_duration(duration_secs)));
                metadata.exif.set("DurationSeconds", AttrValue::Double(duration_secs));
            }

            // Preferred rate (fixed point 16.16)
            let rate = u32::from_be_bytes([buf[16], buf[17], buf[18], buf[19]]);
            let rate_float = rate as f64 / 65536.0;
            if (rate_float - 1.0).abs() > 0.001 {
                metadata.exif.set("PreferredRate", AttrValue::Double(rate_float));
            }
        } else {
            // Version 1: 64-bit timestamps
            let mut buf = [0u8; 32];
            reader.read_exact(&mut buf)?;

            let creation_time = u64::from_be_bytes([buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]);
            let modification_time = u64::from_be_bytes([buf[8], buf[9], buf[10], buf[11], buf[12], buf[13], buf[14], buf[15]]);
            let timescale = u32::from_be_bytes([buf[16], buf[17], buf[18], buf[19]]);
            let duration = u64::from_be_bytes([buf[20], buf[21], buf[22], buf[23], buf[24], buf[25], buf[26], buf[27]]);

            state.timescale = timescale;

            if creation_time > 0 {
                metadata.exif.set("CreationTime", AttrValue::Str(mac_time_to_string(creation_time)));
            }
            if modification_time > 0 {
                metadata.exif.set("ModificationTime", AttrValue::Str(mac_time_to_string(modification_time)));
            }

            metadata.exif.set("TimeScale", AttrValue::UInt(timescale));

            if timescale > 0 {
                let duration_secs = duration as f64 / timescale as f64;
                metadata.exif.set("Duration", AttrValue::Str(format_duration(duration_secs)));
                metadata.exif.set("DurationSeconds", AttrValue::Double(duration_secs));
            }
        }

        Ok(())
    }

    /// Parse trak (track) box.
    fn parse_trak_box(
        &self,
        reader: &mut dyn ReadSeek,
        trak_start: u64,
        trak_size: u64,
        metadata: &mut Metadata,
        state: &mut ParseState,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(trak_start + 8))?;

        let trak_end = trak_start + trak_size;
        let mut buf = [0u8; 8];
        let mut track_type: Option<[u8; 4]> = None;

        while reader.stream_position()? < trak_end {
            let pos = reader.stream_position()?;

            if reader.read_exact(&mut buf).is_err() {
                break;
            }

            let box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];

            if box_size < 8 || pos + box_size > trak_end {
                break;
            }

            match &box_type {
                b"tkhd" => {
                    self.parse_tkhd_box(reader, metadata, state)?;
                }
                b"mdia" => {
                    track_type = self.parse_mdia_box(reader, pos, box_size, metadata, state)?;
                }
                _ => {}
            }

            reader.seek(SeekFrom::Start(pos + box_size))?;
        }

        // Count track types
        if let Some(handler) = track_type {
            match &handler {
                b"vide" => state.video_tracks += 1,
                b"soun" => state.audio_tracks += 1,
                _ => {}
            }
        }

        Ok(())
    }

    /// Parse tkhd (track header) box.
    fn parse_tkhd_box(
        &self,
        reader: &mut dyn ReadSeek,
        metadata: &mut Metadata,
        _state: &mut ParseState,
    ) -> Result<()> {
        let mut vf = [0u8; 4];
        reader.read_exact(&mut vf)?;
        let version = vf[0];

        if version == 0 {
            // Skip timestamps and other fields to get to dimensions
            // creation (4) + modification (4) + track_id (4) + reserved (4) + duration (4) = 20
            // + reserved (8) + layer (2) + alternate_group (2) + volume (2) + reserved (2) = 16
            let mut skip = [0u8; 68];
            reader.read_exact(&mut skip)?;

            // Matrix (36 bytes) already skipped in skip buffer
            // Width and height at offset 76 from start (fixed point 16.16)
            let width = u32::from_be_bytes([skip[60], skip[61], skip[62], skip[63]]);
            let height = u32::from_be_bytes([skip[64], skip[65], skip[66], skip[67]]);

            let width_px = width >> 16;
            let height_px = height >> 16;

            if width_px > 0 && height_px > 0 {
                metadata.exif.set("TrackWidth", AttrValue::UInt(width_px));
                metadata.exif.set("TrackHeight", AttrValue::UInt(height_px));
            }
        } else {
            // Version 1: 64-bit timestamps
            let mut skip = [0u8; 80];
            reader.read_exact(&mut skip)?;

            let width = u32::from_be_bytes([skip[72], skip[73], skip[74], skip[75]]);
            let height = u32::from_be_bytes([skip[76], skip[77], skip[78], skip[79]]);

            let width_px = width >> 16;
            let height_px = height >> 16;

            if width_px > 0 && height_px > 0 {
                metadata.exif.set("TrackWidth", AttrValue::UInt(width_px));
                metadata.exif.set("TrackHeight", AttrValue::UInt(height_px));
            }
        }

        Ok(())
    }

    /// Parse mdia (media) box. Returns handler type.
    fn parse_mdia_box(
        &self,
        reader: &mut dyn ReadSeek,
        mdia_start: u64,
        mdia_size: u64,
        metadata: &mut Metadata,
        _state: &mut ParseState,
    ) -> Result<Option<[u8; 4]>> {
        reader.seek(SeekFrom::Start(mdia_start + 8))?;

        let mdia_end = mdia_start + mdia_size;
        let mut buf = [0u8; 8];
        let mut handler_type: Option<[u8; 4]> = None;

        while reader.stream_position()? < mdia_end {
            let pos = reader.stream_position()?;

            if reader.read_exact(&mut buf).is_err() {
                break;
            }

            let box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];

            if box_size < 8 || pos + box_size > mdia_end {
                break;
            }

            match &box_type {
                b"hdlr" => {
                    handler_type = self.parse_hdlr_box(reader, metadata)?;
                }
                b"minf" => {
                    self.parse_minf_box(reader, pos, box_size, metadata, handler_type)?;
                }
                _ => {}
            }

            reader.seek(SeekFrom::Start(pos + box_size))?;
        }

        Ok(handler_type)
    }

    /// Parse hdlr (handler) box. Returns handler type (vide, soun, etc.)
    fn parse_hdlr_box(
        &self,
        reader: &mut dyn ReadSeek,
        metadata: &mut Metadata,
    ) -> Result<Option<[u8; 4]>> {
        // Version/flags (4) + pre_defined (4) = 8 bytes
        let mut prefix = [0u8; 8];
        reader.read_exact(&mut prefix)?;

        // Handler type
        let mut handler = [0u8; 4];
        reader.read_exact(&mut handler)?;

        let handler_str = String::from_utf8_lossy(&handler).to_string();
        let handler_name = match &handler {
            b"vide" => "Video",
            b"soun" => "Sound",
            b"hint" => "Hint",
            b"meta" => "Metadata",
            b"text" => "Text",
            b"subt" => "Subtitle",
            _ => &handler_str,
        };

        metadata.exif.set("HandlerType", AttrValue::Str(handler_name.to_string()));

        Ok(Some(handler))
    }

    /// Parse minf (media info) box.
    fn parse_minf_box(
        &self,
        reader: &mut dyn ReadSeek,
        minf_start: u64,
        minf_size: u64,
        metadata: &mut Metadata,
        handler_type: Option<[u8; 4]>,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(minf_start + 8))?;

        let minf_end = minf_start + minf_size;
        let mut buf = [0u8; 8];

        while reader.stream_position()? < minf_end {
            let pos = reader.stream_position()?;

            if reader.read_exact(&mut buf).is_err() {
                break;
            }

            let box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];

            if box_size < 8 || pos + box_size > minf_end {
                break;
            }

            if &box_type == b"stbl" {
                self.parse_stbl_box(reader, pos, box_size, metadata, handler_type)?;
            }

            reader.seek(SeekFrom::Start(pos + box_size))?;
        }

        Ok(())
    }

    /// Parse stbl (sample table) box.
    fn parse_stbl_box(
        &self,
        reader: &mut dyn ReadSeek,
        stbl_start: u64,
        stbl_size: u64,
        metadata: &mut Metadata,
        handler_type: Option<[u8; 4]>,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(stbl_start + 8))?;

        let stbl_end = stbl_start + stbl_size;
        let mut buf = [0u8; 8];

        while reader.stream_position()? < stbl_end {
            let pos = reader.stream_position()?;

            if reader.read_exact(&mut buf).is_err() {
                break;
            }

            let box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];

            if box_size < 8 || pos + box_size > stbl_end {
                break;
            }

            if &box_type == b"stsd" {
                self.parse_stsd_box(reader, pos, box_size, metadata, handler_type)?;
            }

            reader.seek(SeekFrom::Start(pos + box_size))?;
        }

        Ok(())
    }

    /// Parse stsd (sample description) box - contains codec info.
    fn parse_stsd_box(
        &self,
        reader: &mut dyn ReadSeek,
        _stsd_start: u64,
        _stsd_size: u64,
        metadata: &mut Metadata,
        handler_type: Option<[u8; 4]>,
    ) -> Result<()> {
        // Version/flags (4) + entry count (4)
        let mut header = [0u8; 8];
        reader.read_exact(&mut header)?;

        let entry_count = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);
        if entry_count == 0 {
            return Ok(());
        }

        // Read first sample entry
        let mut entry_header = [0u8; 8];
        reader.read_exact(&mut entry_header)?;

        let entry_size = u32::from_be_bytes([entry_header[0], entry_header[1], entry_header[2], entry_header[3]]) as u64;
        let codec_fourcc = [entry_header[4], entry_header[5], entry_header[6], entry_header[7]];
        let codec_str = String::from_utf8_lossy(&codec_fourcc).trim().to_string();

        // Decode codec name
        let codec_name = match &codec_fourcc {
            // Video codecs
            b"avc1" | b"avc2" | b"avc3" | b"avc4" => "H.264/AVC",
            b"hvc1" | b"hev1" => "H.265/HEVC",
            b"vp08" => "VP8",
            b"vp09" => "VP9",
            b"av01" => "AV1",
            b"mp4v" => "MPEG-4 Visual",
            b"mjp2" => "Motion JPEG 2000",
            b"jpeg" => "JPEG",
            b"png " => "PNG",
            b"raw " => "Uncompressed RGB",
            b"2vuy" | b"yuvs" => "Uncompressed YUV",
            b"apch" => "Apple ProRes 422 HQ",
            b"apcn" => "Apple ProRes 422",
            b"apcs" => "Apple ProRes 422 LT",
            b"apco" => "Apple ProRes 422 Proxy",
            b"ap4h" => "Apple ProRes 4444",
            b"ap4x" => "Apple ProRes 4444 XQ",
            // Audio codecs
            b"mp4a" => "AAC",
            b"ac-3" | b"ac3 " => "AC-3",
            b"ec-3" | b"ec3 " => "E-AC-3",
            b"alac" => "Apple Lossless",
            b"fLaC" => "FLAC",
            b"Opus" => "Opus",
            b"dtsc" | b"dtsh" | b"dtsl" | b"dtse" => "DTS",
            b"sowt" | b"twos" => "PCM",
            b"alaw" => "A-Law PCM",
            b"ulaw" => "mu-Law PCM",
            b".mp3" => "MP3",
            _ => &codec_str,
        };

        if handler_type == Some(*b"vide") {
            metadata.exif.set("VideoCodec", AttrValue::Str(codec_name.to_string()));
            metadata.exif.set("VideoCodecFourCC", AttrValue::Str(codec_str.clone()));

            // Parse video sample entry for dimensions
            if entry_size >= 86 {
                // Skip: reserved (6) + data_ref_index (2) + pre_defined (2) + reserved (2) + pre_defined (12)
                let mut skip = [0u8; 24];
                reader.read_exact(&mut skip)?;

                // Width and height (16 bits each)
                let mut dim = [0u8; 4];
                reader.read_exact(&mut dim)?;
                let width = u16::from_be_bytes([dim[0], dim[1]]);
                let height = u16::from_be_bytes([dim[2], dim[3]]);

                if width > 0 && height > 0 {
                    metadata.exif.set("ImageWidth", AttrValue::UInt(width as u32));
                    metadata.exif.set("ImageHeight", AttrValue::UInt(height as u32));
                }

                // Horizontal and vertical resolution (fixed point 16.16)
                let mut res = [0u8; 8];
                reader.read_exact(&mut res)?;
                let hres = u32::from_be_bytes([res[0], res[1], res[2], res[3]]) >> 16;
                let vres = u32::from_be_bytes([res[4], res[5], res[6], res[7]]) >> 16;

                if hres > 0 {
                    metadata.exif.set("VideoResolution", AttrValue::Str(format!("{}x{} dpi", hres, vres)));
                }

                // Skip reserved (4) + frame_count (2) = 6
                let mut skip2 = [0u8; 6];
                reader.read_exact(&mut skip2)?;
                let frame_count = u16::from_be_bytes([skip2[4], skip2[5]]);
                if frame_count > 1 {
                    metadata.exif.set("FrameCount", AttrValue::UInt(frame_count as u32));
                }

                // Compressor name (32 bytes, pascal string)
                let mut compressor = [0u8; 32];
                reader.read_exact(&mut compressor)?;
                let name_len = compressor[0] as usize;
                if name_len > 0 && name_len < 32 {
                    let name = String::from_utf8_lossy(&compressor[1..1 + name_len]).trim().to_string();
                    if !name.is_empty() {
                        metadata.exif.set("CompressorName", AttrValue::Str(name));
                    }
                }

                // Depth (2 bytes)
                let mut depth = [0u8; 2];
                reader.read_exact(&mut depth)?;
                let bit_depth = u16::from_be_bytes(depth);
                if bit_depth > 0 && bit_depth != 0xFFFF {
                    metadata.exif.set("BitDepth", AttrValue::UInt(bit_depth as u32));
                }
            }
        } else if handler_type == Some(*b"soun") {
            metadata.exif.set("AudioCodec", AttrValue::Str(codec_name.to_string()));
            metadata.exif.set("AudioCodecFourCC", AttrValue::Str(codec_str));

            // Parse audio sample entry
            if entry_size >= 36 {
                // Skip: reserved (6) + data_ref_index (2) + version (2) + revision (2) + vendor (4)
                let mut skip = [0u8; 16];
                reader.read_exact(&mut skip)?;

                // Channel count and sample size
                let mut audio = [0u8; 4];
                reader.read_exact(&mut audio)?;
                let channels = u16::from_be_bytes([audio[0], audio[1]]);
                let sample_size = u16::from_be_bytes([audio[2], audio[3]]);

                if channels > 0 {
                    metadata.exif.set("AudioChannels", AttrValue::UInt(channels as u32));
                }
                if sample_size > 0 {
                    metadata.exif.set("AudioSampleSize", AttrValue::UInt(sample_size as u32));
                }

                // Skip compression_id (2) + packet_size (2) = 4
                let mut skip2 = [0u8; 4];
                reader.read_exact(&mut skip2)?;

                // Sample rate (fixed point 16.16)
                let mut rate = [0u8; 4];
                reader.read_exact(&mut rate)?;
                let sample_rate = u32::from_be_bytes(rate) >> 16;

                if sample_rate > 0 {
                    metadata.exif.set("AudioSampleRate", AttrValue::UInt(sample_rate));
                }
            }
        }

        Ok(())
    }

    /// Parse udta (user data) box.
    fn parse_udta_box(
        &self,
        reader: &mut dyn ReadSeek,
        udta_start: u64,
        udta_size: u64,
        metadata: &mut Metadata,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(udta_start + 8))?;

        let udta_end = udta_start + udta_size;
        let mut buf = [0u8; 8];

        while reader.stream_position()? < udta_end {
            let pos = reader.stream_position()?;

            if reader.read_exact(&mut buf).is_err() {
                break;
            }

            let box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];

            if box_size < 8 || pos + box_size > udta_end {
                break;
            }

            match &box_type {
                b"meta" => {
                    self.parse_meta_box(reader, pos, box_size, metadata)?;
                }
                b"\xa9nam" => {
                    // Title
                    if let Some(text) = self.read_text_box(reader, box_size)? {
                        metadata.exif.set("Title", AttrValue::Str(text));
                    }
                }
                b"\xa9ART" => {
                    // Artist
                    if let Some(text) = self.read_text_box(reader, box_size)? {
                        metadata.exif.set("Artist", AttrValue::Str(text));
                    }
                }
                b"\xa9alb" => {
                    // Album
                    if let Some(text) = self.read_text_box(reader, box_size)? {
                        metadata.exif.set("Album", AttrValue::Str(text));
                    }
                }
                b"\xa9day" => {
                    // Year/date
                    if let Some(text) = self.read_text_box(reader, box_size)? {
                        metadata.exif.set("Year", AttrValue::Str(text));
                    }
                }
                b"\xa9cmt" => {
                    // Comment
                    if let Some(text) = self.read_text_box(reader, box_size)? {
                        metadata.exif.set("Comment", AttrValue::Str(text));
                    }
                }
                b"\xa9gen" => {
                    // Genre
                    if let Some(text) = self.read_text_box(reader, box_size)? {
                        metadata.exif.set("Genre", AttrValue::Str(text));
                    }
                }
                b"\xa9wrt" => {
                    // Writer
                    if let Some(text) = self.read_text_box(reader, box_size)? {
                        metadata.exif.set("Writer", AttrValue::Str(text));
                    }
                }
                b"\xa9too" => {
                    // Encoder
                    if let Some(text) = self.read_text_box(reader, box_size)? {
                        metadata.exif.set("Encoder", AttrValue::Str(text));
                    }
                }
                _ => {}
            }

            reader.seek(SeekFrom::Start(pos + box_size))?;
        }

        Ok(())
    }

    /// Parse meta (metadata) box.
    fn parse_meta_box(
        &self,
        reader: &mut dyn ReadSeek,
        meta_start: u64,
        meta_size: u64,
        metadata: &mut Metadata,
    ) -> Result<()> {
        // Skip version/flags (4 bytes)
        reader.seek(SeekFrom::Start(meta_start + 12))?;

        let meta_end = meta_start + meta_size;
        let mut buf = [0u8; 8];

        while reader.stream_position()? < meta_end {
            let pos = reader.stream_position()?;

            if reader.read_exact(&mut buf).is_err() {
                break;
            }

            let box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];

            if box_size < 8 || pos + box_size > meta_end {
                break;
            }

            match &box_type {
                b"ilst" => {
                    self.parse_ilst_box(reader, pos, box_size, metadata)?;
                }
                b"xml " => {
                    // XMP data
                    let xml_size = (box_size - 8) as usize;
                    if xml_size > 0 && xml_size < 10 * 1024 * 1024 {
                        let mut xml_data = vec![0u8; xml_size];
                        reader.read_exact(&mut xml_data)?;
                        if let Ok(xmp) = String::from_utf8(xml_data) {
                            metadata.xmp = Some(xmp);
                        }
                    }
                }
                _ => {}
            }

            reader.seek(SeekFrom::Start(pos + box_size))?;
        }

        Ok(())
    }

    /// Parse ilst (item list) box - iTunes-style metadata.
    fn parse_ilst_box(
        &self,
        reader: &mut dyn ReadSeek,
        ilst_start: u64,
        ilst_size: u64,
        metadata: &mut Metadata,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(ilst_start + 8))?;

        let ilst_end = ilst_start + ilst_size;
        let mut buf = [0u8; 8];

        while reader.stream_position()? < ilst_end {
            let pos = reader.stream_position()?;

            if reader.read_exact(&mut buf).is_err() {
                break;
            }

            let box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];

            if box_size < 8 || pos + box_size > ilst_end {
                break;
            }

            // Parse data box inside
            let tag_name = match &box_type {
                b"\xa9nam" => Some("Title"),
                b"\xa9ART" => Some("Artist"),
                b"\xa9alb" => Some("Album"),
                b"\xa9day" => Some("Year"),
                b"\xa9cmt" => Some("Comment"),
                b"\xa9gen" => Some("Genre"),
                b"\xa9wrt" => Some("Writer"),
                b"\xa9too" => Some("Encoder"),
                b"\xa9lyr" => Some("Lyrics"),
                b"aART" => Some("AlbumArtist"),
                b"cprt" => Some("Copyright"),
                b"desc" => Some("Description"),
                b"gnre" => Some("GenreID"),
                b"trkn" => Some("TrackNumber"),
                b"disk" => Some("DiscNumber"),
                b"cpil" => Some("Compilation"),
                b"tmpo" => Some("Tempo"),
                _ => None,
            };

            if let Some(name) = tag_name {
                if let Some(value) = self.read_ilst_data(reader, pos, box_size)? {
                    metadata.exif.set(name, AttrValue::Str(value));
                }
            }

            reader.seek(SeekFrom::Start(pos + box_size))?;
        }

        Ok(())
    }

    /// Read data from an ilst item.
    fn read_ilst_data(
        &self,
        reader: &mut dyn ReadSeek,
        item_start: u64,
        item_size: u64,
    ) -> Result<Option<String>> {
        reader.seek(SeekFrom::Start(item_start + 8))?;

        let item_end = item_start + item_size;
        let mut buf = [0u8; 8];

        while reader.stream_position()? < item_end {
            let pos = reader.stream_position()?;

            if reader.read_exact(&mut buf).is_err() {
                break;
            }

            let box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];

            if &box_type == b"data" && box_size > 16 {
                // Skip type indicator (4) + locale (4) = 8
                let mut skip = [0u8; 8];
                reader.read_exact(&mut skip)?;

                let data_size = (box_size - 16) as usize;
                if data_size > 0 && data_size < 65536 {
                    let mut data = vec![0u8; data_size];
                    reader.read_exact(&mut data)?;

                    // Try to decode as UTF-8
                    if let Ok(text) = String::from_utf8(data.clone()) {
                        return Ok(Some(text.trim().to_string()));
                    }
                    // Try UTF-16BE
                    if data.len() >= 2 {
                        let utf16: Vec<u16> = data
                            .chunks_exact(2)
                            .map(|c| u16::from_be_bytes([c[0], c[1]]))
                            .collect();
                        if let Ok(text) = String::from_utf16(&utf16) {
                            return Ok(Some(text.trim().to_string()));
                        }
                    }
                }
            }

            if box_size == 0 || pos + box_size > item_end {
                break;
            }
            reader.seek(SeekFrom::Start(pos + box_size))?;
        }

        Ok(None)
    }

    /// Read text from a simple text box.
    fn read_text_box(&self, reader: &mut dyn ReadSeek, box_size: u64) -> Result<Option<String>> {
        let data_size = (box_size - 8) as usize;
        if data_size == 0 || data_size > 65536 {
            return Ok(None);
        }

        let mut data = vec![0u8; data_size];
        reader.read_exact(&mut data)?;

        // Try UTF-8
        if let Ok(text) = String::from_utf8(data.clone()) {
            return Ok(Some(text.trim().to_string()));
        }

        // Try UTF-16BE with BOM
        if data.len() >= 2 && data[0] == 0xFE && data[1] == 0xFF {
            let utf16: Vec<u16> = data[2..]
                .chunks_exact(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            if let Ok(text) = String::from_utf16(&utf16) {
                return Ok(Some(text.trim().to_string()));
            }
        }

        Ok(None)
    }

    /// Parse UUID box (may contain XMP).
    fn parse_uuid_box(
        &self,
        reader: &mut dyn ReadSeek,
        box_start: u64,
        box_size: u64,
        metadata: &mut Metadata,
    ) -> Result<()> {
        // XMP UUID: BE7ACFCB-97A9-42E8-9C71-999491E3AFAC
        const XMP_UUID: [u8; 16] = [
            0xBE, 0x7A, 0xCF, 0xCB, 0x97, 0xA9, 0x42, 0xE8,
            0x9C, 0x71, 0x99, 0x94, 0x91, 0xE3, 0xAF, 0xAC,
        ];

        reader.seek(SeekFrom::Start(box_start + 8))?;

        let mut uuid = [0u8; 16];
        reader.read_exact(&mut uuid)?;

        if uuid == XMP_UUID {
            let xmp_size = (box_size - 24) as usize;
            if xmp_size > 0 && xmp_size < 10 * 1024 * 1024 {
                let mut xmp_data = vec![0u8; xmp_size];
                reader.read_exact(&mut xmp_data)?;
                if let Ok(xmp) = String::from_utf8(xmp_data) {
                    metadata.xmp = Some(xmp);
                }
            }
        }

        Ok(())
    }
}

/// Convert Mac timestamp (seconds since 1904-01-01) to ISO string.
fn mac_time_to_string(mac_time: u64) -> String {
    // Mac epoch: 1904-01-01 00:00:00 UTC
    // Unix epoch: 1970-01-01 00:00:00 UTC
    // Difference: 2082844800 seconds
    const MAC_UNIX_DIFF: u64 = 2082844800;

    if mac_time < MAC_UNIX_DIFF {
        return "Unknown".to_string();
    }

    let unix_time = mac_time - MAC_UNIX_DIFF;

    // Convert to date components (simplified, not accounting for leap seconds)
    let secs = unix_time % 60;
    let mins = (unix_time / 60) % 60;
    let hours = (unix_time / 3600) % 24;
    let days = unix_time / 86400;

    // Calculate year/month/day from days since 1970-01-01
    let (year, month, day) = days_to_ymd(days);

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, mins, secs
    )
}

/// Convert days since Unix epoch to year/month/day.
fn days_to_ymd(days: u64) -> (u32, u32, u32) {
    // Simplified calculation
    let mut remaining = days as i64;
    let mut year = 1970u32;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year += 1;
    }

    let month_days: [i64; 12] = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1u32;
    for &days in &month_days {
        if remaining < days {
            break;
        }
        remaining -= days;
        month += 1;
    }

    (year, month, remaining as u32 + 1)
}

fn is_leap_year(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
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

    fn make_mp4_header(brand: &[u8; 4]) -> Vec<u8> {
        let mut data = Vec::new();
        // ftyp box (20 bytes)
        data.extend_from_slice(&20u32.to_be_bytes()); // size
        data.extend_from_slice(b"ftyp");
        data.extend_from_slice(brand); // major brand
        data.extend_from_slice(&0u32.to_be_bytes()); // minor version
        data.extend_from_slice(b"isom"); // compatible brand
        data
    }

    #[test]
    fn detect_mp4() {
        let parser = Mp4Parser;
        assert!(parser.can_parse(&make_mp4_header(b"isom")));
        assert!(parser.can_parse(&make_mp4_header(b"mp41")));
        assert!(parser.can_parse(&make_mp4_header(b"mp42")));
    }

    #[test]
    fn detect_m4a() {
        let parser = Mp4Parser;
        assert!(parser.can_parse(&make_mp4_header(b"M4A ")));
    }

    #[test]
    fn detect_mov() {
        let parser = Mp4Parser;
        assert!(parser.can_parse(&make_mp4_header(b"qt  ")));
    }

    #[test]
    fn reject_heic() {
        let parser = Mp4Parser;
        assert!(!parser.can_parse(&make_mp4_header(b"heic")));
        assert!(!parser.can_parse(&make_mp4_header(b"avif")));
    }

    #[test]
    fn reject_jpeg() {
        let parser = Mp4Parser;
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
    }

    #[test]
    fn parse_minimal_mp4() {
        let parser = Mp4Parser;
        let data = make_mp4_header(b"mp42");
        let mut cursor = Cursor::new(&data);

        let meta = parser.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "MP4");
        assert_eq!(meta.exif.get_str("MajorBrand"), Some("mp42"));
    }

    #[test]
    fn test_mac_time() {
        // 2024-01-01 00:00:00 UTC = Unix 1704067200 = Mac 3786912000
        let mac = 3786912000u64;
        let result = mac_time_to_string(mac);
        assert!(result.starts_with("2024-01-01"));
    }

    #[test]
    fn test_duration_format() {
        assert_eq!(format_duration(65.5), "1:05.500");
        assert_eq!(format_duration(3661.0), "1:01:01.000");
        assert_eq!(format_duration(30.123), "0:30.123");
    }
}

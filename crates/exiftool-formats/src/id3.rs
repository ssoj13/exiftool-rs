//! ID3 tag parser for MP3 files.
//!
//! ID3 is a metadata container for MP3 files. This parser supports:
//! - ID3v1 (128 bytes at end of file)
//! - ID3v1.1 (adds track number)
//! - ID3v2.2, ID3v2.3, ID3v2.4 (variable-length header at start)
//!
//! ID3v2 structure:
//! - 10-byte header: "ID3", version (2 bytes), flags (1 byte), size (4 bytes syncsafe)
//! - Extended header (optional)
//! - Frames (each with ID, size, flags, data)
//! - Padding (optional zeros)
//!
//! Common frame IDs (v2.3/v2.4):
//! - TIT2: Title
//! - TPE1: Artist
//! - TALB: Album
//! - TYER/TDRC: Year
//! - TRCK: Track number
//! - TCON: Genre
//! - COMM: Comment
//! - APIC: Attached picture (album art)

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// ID3 tag parser for MP3 files.
pub struct Id3Parser;

impl FormatParser for Id3Parser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 3 {
            return false;
        }

        // Check for ID3v2 header ("ID3")
        if &header[0..3] == b"ID3" {
            return true;
        }

        // Check for MP3 frame sync (0xFF 0xFB/FA/F3/F2)
        // This indicates an MP3 file that may have ID3v1 at the end
        if header.len() >= 2 && header[0] == 0xFF && (header[1] & 0xE0) == 0xE0 {
            return true;
        }

        false
    }

    fn format_name(&self) -> &'static str {
        "MP3"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["mp3"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("MP3");

        // Try ID3v2 first (at beginning)
        reader.seek(SeekFrom::Start(0))?;
        let mut header = [0u8; 10];
        reader.read_exact(&mut header)?;

        if &header[0..3] == b"ID3" {
            self.parse_id3v2(reader, &header, &mut metadata)?;
        }

        // Also try ID3v1 (last 128 bytes)
        let file_size = crate::utils::get_file_size(reader)?;
        if file_size >= 128 {
            reader.seek(SeekFrom::End(-128))?;
            let mut id3v1 = [0u8; 128];
            reader.read_exact(&mut id3v1)?;

            if &id3v1[0..3] == b"TAG" {
                self.parse_id3v1(&id3v1, &mut metadata)?;
            }
        }

        // Extract MP3 audio info from first frame (if present)
        reader.seek(SeekFrom::Start(0))?;
        self.extract_audio_info(reader, &mut metadata)?;

        Ok(metadata)
    }
}

impl Id3Parser {
    /// Parse ID3v2 header and frames.
    fn parse_id3v2(
        &self,
        reader: &mut dyn ReadSeek,
        header: &[u8; 10],
        metadata: &mut Metadata,
    ) -> Result<()> {
        let version_major = header[3];
        let version_minor = header[4];
        let flags = header[5];

        // Sync-safe integer: each byte uses only 7 bits
        let size = ((header[6] as u32) << 21)
            | ((header[7] as u32) << 14)
            | ((header[8] as u32) << 7)
            | (header[9] as u32);

        metadata.exif.set("ID3Version", AttrValue::Str(format!("2.{}.{}", version_major, version_minor)));
        metadata.exif.set("ID3Size", AttrValue::UInt(size));

        let unsynchronization = flags & 0x80 != 0;
        let extended_header = flags & 0x40 != 0;
        let _experimental = flags & 0x20 != 0;
        let _footer = flags & 0x10 != 0;

        if unsynchronization {
            metadata.exif.set("ID3Unsynchronization", AttrValue::Bool(true));
        }

        let mut pos = 10u64;
        let end_pos = 10 + size as u64;

        // Skip extended header if present
        if extended_header {
            let mut ext_size = [0u8; 4];
            reader.read_exact(&mut ext_size)?;

            let ext_len = if version_major >= 4 {
                // v2.4: sync-safe
                ((ext_size[0] as u32) << 21)
                    | ((ext_size[1] as u32) << 14)
                    | ((ext_size[2] as u32) << 7)
                    | (ext_size[3] as u32)
            } else {
                // v2.3: normal integer
                u32::from_be_bytes(ext_size)
            };

            pos += 4;
            reader.seek(SeekFrom::Current(ext_len as i64 - 4))?;
            pos += ext_len as u64 - 4;
        }

        // Parse frames
        while pos < end_pos {
            let frame_start = reader.stream_position()?;
            if frame_start >= end_pos {
                break;
            }

            let frame = if version_major >= 3 {
                self.read_frame_v23(reader, version_major)?
            } else {
                self.read_frame_v22(reader)?
            };

            match frame {
                Some((id, data)) => {
                    self.process_frame(&id, &data, version_major, metadata)?;
                    pos = reader.stream_position()?;
                }
                None => break, // Padding or invalid frame
            }
        }

        Ok(())
    }

    /// Read ID3v2.3/v2.4 frame (4-byte ID, 4-byte size, 2-byte flags).
    fn read_frame_v23(&self, reader: &mut dyn ReadSeek, version: u8) -> Result<Option<(String, Vec<u8>)>> {
        let mut frame_header = [0u8; 10];
        if reader.read_exact(&mut frame_header).is_err() {
            return Ok(None);
        }

        // Check for padding (all zeros)
        if frame_header.iter().all(|&b| b == 0) {
            return Ok(None);
        }

        let id = String::from_utf8_lossy(&frame_header[0..4]).to_string();

        // Validate frame ID (should be alphanumeric)
        if !id.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Ok(None);
        }

        let size = if version >= 4 {
            // v2.4: sync-safe integer
            ((frame_header[4] as u32) << 21)
                | ((frame_header[5] as u32) << 14)
                | ((frame_header[6] as u32) << 7)
                | (frame_header[7] as u32)
        } else {
            // v2.3: normal integer
            u32::from_be_bytes([frame_header[4], frame_header[5], frame_header[6], frame_header[7]])
        };

        if size == 0 || size > 10 * 1024 * 1024 {
            return Ok(None);
        }

        let _flags = u16::from_be_bytes([frame_header[8], frame_header[9]]);

        let mut data = vec![0u8; size as usize];
        reader.read_exact(&mut data)?;

        Ok(Some((id, data)))
    }

    /// Read ID3v2.2 frame (3-byte ID, 3-byte size).
    fn read_frame_v22(&self, reader: &mut dyn ReadSeek) -> Result<Option<(String, Vec<u8>)>> {
        let mut frame_header = [0u8; 6];
        if reader.read_exact(&mut frame_header).is_err() {
            return Ok(None);
        }

        // Check for padding
        if frame_header.iter().all(|&b| b == 0) {
            return Ok(None);
        }

        let id = String::from_utf8_lossy(&frame_header[0..3]).to_string();

        // Validate frame ID
        if !id.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Ok(None);
        }

        // 3-byte big-endian size
        let size = ((frame_header[3] as u32) << 16)
            | ((frame_header[4] as u32) << 8)
            | (frame_header[5] as u32);

        if size == 0 || size > 10 * 1024 * 1024 {
            return Ok(None);
        }

        let mut data = vec![0u8; size as usize];
        reader.read_exact(&mut data)?;

        // Convert v2.2 ID to v2.3 equivalent
        let id_v23 = match id.as_str() {
            "TT2" => "TIT2",
            "TP1" => "TPE1",
            "TAL" => "TALB",
            "TYE" => "TYER",
            "TRK" => "TRCK",
            "TCO" => "TCON",
            "COM" => "COMM",
            "PIC" => "APIC",
            _ => &id,
        };

        Ok(Some((id_v23.to_string(), data)))
    }

    /// Process a frame and add to metadata.
    fn process_frame(
        &self,
        id: &str,
        data: &[u8],
        _version: u8,
        metadata: &mut Metadata,
    ) -> Result<()> {
        let tag_name = match id {
            "TIT2" => "Title",
            "TPE1" => "Artist",
            "TPE2" => "AlbumArtist",
            "TALB" => "Album",
            "TYER" | "TDRC" => "Year",
            "TRCK" => "Track",
            "TPOS" => "DiscNumber",
            "TCON" => "Genre",
            "TCOM" => "Composer",
            "TPUB" => "Publisher",
            "TCOP" => "Copyright",
            "TENC" => "EncodedBy",
            "TSSE" => "EncoderSettings",
            "TBPM" => "BPM",
            "TLEN" => "Duration",
            "TKEY" => "InitialKey",
            "TSRC" => "ISRC",
            "COMM" => "Comment",
            "USLT" => "Lyrics",
            "APIC" => "AlbumArt",
            "TXXX" => "UserDefined",
            "WXXX" => "UserURL",
            "PRIV" => "Private",
            _ => return Ok(()), // Skip unknown frames
        };

        // Text frames start with encoding byte
        if id.starts_with('T') && !data.is_empty() {
            let text = self.decode_text(data);
            if !text.is_empty() {
                if id == "TCON" {
                    // Genre may be "(nn)" format
                    let genre = self.decode_genre(&text);
                    metadata.exif.set(tag_name, AttrValue::Str(genre));
                } else {
                    metadata.exif.set(tag_name, AttrValue::Str(text));
                }
            }
        } else if id == "COMM" && data.len() > 4 {
            // Comment: encoding + language (3) + short desc (null-term) + text
            let text = self.decode_comment(data);
            if !text.is_empty() {
                metadata.exif.set(tag_name, AttrValue::Str(text));
            }
        } else if id == "APIC" && !data.is_empty() {
            // Picture frame - just note its presence
            let (mime, pic_type) = self.decode_apic_info(data);
            metadata.exif.set("AlbumArtMime", AttrValue::Str(mime));
            metadata.exif.set("AlbumArtType", AttrValue::Str(pic_type));
            metadata.exif.set("AlbumArtSize", AttrValue::UInt(data.len() as u32));
        }

        Ok(())
    }

    /// Decode text from ID3 frame (handles different encodings).
    fn decode_text(&self, data: &[u8]) -> String {
        if data.is_empty() {
            return String::new();
        }

        let encoding = data[0];
        let text_data = &data[1..];

        match encoding {
            0 => {
                // ISO-8859-1
                text_data
                    .iter()
                    .take_while(|&&b| b != 0)
                    .map(|&b| b as char)
                    .collect()
            }
            1 => {
                // UTF-16 with BOM
                if text_data.len() < 2 {
                    return String::new();
                }
                let bom = u16::from_le_bytes([text_data[0], text_data[1]]);
                let is_le = bom == 0xFEFF;

                let utf16: Vec<u16> = text_data[2..]
                    .chunks_exact(2)
                    .map(|c| {
                        if is_le {
                            u16::from_le_bytes([c[0], c[1]])
                        } else {
                            u16::from_be_bytes([c[0], c[1]])
                        }
                    })
                    .take_while(|&c| c != 0)
                    .collect();

                String::from_utf16_lossy(&utf16)
            }
            2 => {
                // UTF-16BE without BOM
                let utf16: Vec<u16> = text_data
                    .chunks_exact(2)
                    .map(|c| u16::from_be_bytes([c[0], c[1]]))
                    .take_while(|&c| c != 0)
                    .collect();

                String::from_utf16_lossy(&utf16)
            }
            3 => {
                // UTF-8
                let end = text_data.iter().position(|&b| b == 0).unwrap_or(text_data.len());
                String::from_utf8_lossy(&text_data[..end]).to_string()
            }
            _ => String::new(),
        }
    }

    /// Decode comment frame.
    fn decode_comment(&self, data: &[u8]) -> String {
        if data.len() < 5 {
            return String::new();
        }

        let encoding = data[0];
        // Skip language (3 bytes) and short description
        let text_start = if encoding == 1 || encoding == 2 {
            // UTF-16: find double null terminator
            let mut pos = 4;
            while pos + 1 < data.len() {
                if data[pos] == 0 && data[pos + 1] == 0 {
                    pos += 2;
                    break;
                }
                pos += 2;
            }
            pos
        } else {
            // Single byte encoding: find single null
            data[4..].iter().position(|&b| b == 0).map(|p| p + 5).unwrap_or(data.len())
        };

        if text_start >= data.len() {
            return String::new();
        }

        // Create a temporary slice for decoding
        let mut temp = vec![encoding];
        temp.extend_from_slice(&data[text_start..]);
        self.decode_text(&temp)
    }

    /// Decode genre, handling "(nn)" numeric format.
    fn decode_genre(&self, text: &str) -> String {
        // Handle "(nn)" or "nn" format
        let trimmed = text.trim_start_matches('(').trim_end_matches(')');
        if let Ok(num) = trimmed.parse::<u8>() {
            return self.genre_name(num).to_string();
        }

        // Handle "(nn)Genre" format
        if text.starts_with('(') {
            if let Some(close) = text.find(')') {
                if let Ok(num) = text[1..close].parse::<u8>() {
                    let genre_name = self.genre_name(num);
                    let suffix = &text[close + 1..];
                    if suffix.is_empty() || suffix == genre_name {
                        return genre_name.to_string();
                    }
                    return format!("{} ({})", suffix, genre_name);
                }
            }
        }

        text.to_string()
    }

    /// Get genre name from ID3v1 genre number.
    fn genre_name(&self, num: u8) -> &'static str {
        const GENRES: &[&str] = &[
            "Blues", "Classic Rock", "Country", "Dance", "Disco", "Funk", "Grunge",
            "Hip-Hop", "Jazz", "Metal", "New Age", "Oldies", "Other", "Pop", "R&B",
            "Rap", "Reggae", "Rock", "Techno", "Industrial", "Alternative", "Ska",
            "Death Metal", "Pranks", "Soundtrack", "Euro-Techno", "Ambient",
            "Trip-Hop", "Vocal", "Jazz+Funk", "Fusion", "Trance", "Classical",
            "Instrumental", "Acid", "House", "Game", "Sound Clip", "Gospel",
            "Noise", "AlternRock", "Bass", "Soul", "Punk", "Space", "Meditative",
            "Instrumental Pop", "Instrumental Rock", "Ethnic", "Gothic",
            "Darkwave", "Techno-Industrial", "Electronic", "Pop-Folk", "Eurodance",
            "Dream", "Southern Rock", "Comedy", "Cult", "Gangsta", "Top 40",
            "Christian Rap", "Pop/Funk", "Jungle", "Native American", "Cabaret",
            "New Wave", "Psychedelic", "Rave", "Showtunes", "Trailer", "Lo-Fi",
            "Tribal", "Acid Punk", "Acid Jazz", "Polka", "Retro", "Musical",
            "Rock & Roll", "Hard Rock", "Folk", "Folk-Rock", "National Folk",
            "Swing", "Fast Fusion", "Bebop", "Latin", "Revival", "Celtic",
            "Bluegrass", "Avantgarde", "Gothic Rock", "Progressive Rock",
            "Psychedelic Rock", "Symphonic Rock", "Slow Rock", "Big Band",
            "Chorus", "Easy Listening", "Acoustic", "Humour", "Speech", "Chanson",
            "Opera", "Chamber Music", "Sonata", "Symphony", "Booty Bass", "Primus",
            "Porn Groove", "Satire", "Slow Jam", "Club", "Tango", "Samba",
            "Folklore", "Ballad", "Power Ballad", "Rhythmic Soul", "Freestyle",
            "Duet", "Punk Rock", "Drum Solo", "A Cappella", "Euro-House",
            "Dance Hall", "Goa", "Drum & Bass", "Club-House", "Hardcore",
            "Terror", "Indie", "BritPop", "Negerpunk", "Polsk Punk", "Beat",
            "Christian Gangsta Rap", "Heavy Metal", "Black Metal", "Crossover",
            "Contemporary Christian", "Christian Rock", "Merengue", "Salsa",
            "Thrash Metal", "Anime", "Jpop", "Synthpop",
        ];

        GENRES.get(num as usize).copied().unwrap_or("Unknown")
    }

    /// Decode APIC frame info (MIME type and picture type).
    fn decode_apic_info(&self, data: &[u8]) -> (String, String) {
        if data.is_empty() {
            return (String::new(), String::new());
        }

        let encoding = data[0];
        let mut pos = 1;

        // MIME type (null-terminated)
        let mime_end = data[pos..].iter().position(|&b| b == 0).unwrap_or(0) + pos;
        let mime = String::from_utf8_lossy(&data[pos..mime_end]).to_string();
        pos = mime_end + 1;

        if pos >= data.len() {
            return (mime, String::new());
        }

        // Picture type
        let pic_type_num = data[pos];
        let pic_type = match pic_type_num {
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

        // Skip description to get to image data size (for info only)
        let _description_encoding = encoding;
        // We just return the info, not the actual image

        (mime, pic_type.to_string())
    }

    /// Parse ID3v1 tag (128 bytes).
    fn parse_id3v1(&self, data: &[u8; 128], metadata: &mut Metadata) -> Result<()> {
        // Only parse if we don't have ID3v2 data
        if metadata.exif.get("ID3Version").is_some() {
            return Ok(());
        }

        metadata.exif.set("ID3Version", AttrValue::Str("1.x".to_string()));

        // Title: bytes 3-32
        let title = self.trim_null_string(&data[3..33]);
        if !title.is_empty() {
            metadata.exif.set("Title", AttrValue::Str(title));
        }

        // Artist: bytes 33-62
        let artist = self.trim_null_string(&data[33..63]);
        if !artist.is_empty() {
            metadata.exif.set("Artist", AttrValue::Str(artist));
        }

        // Album: bytes 63-92
        let album = self.trim_null_string(&data[63..93]);
        if !album.is_empty() {
            metadata.exif.set("Album", AttrValue::Str(album));
        }

        // Year: bytes 93-96
        let year = self.trim_null_string(&data[93..97]);
        if !year.is_empty() {
            metadata.exif.set("Year", AttrValue::Str(year));
        }

        // Comment: bytes 97-126 (or 97-124 for v1.1)
        // ID3v1.1: if byte 125 is 0 and byte 126 is non-zero, it's track number
        if data[125] == 0 && data[126] != 0 {
            // ID3v1.1
            metadata.exif.set("ID3Version", AttrValue::Str("1.1".to_string()));
            let comment = self.trim_null_string(&data[97..125]);
            if !comment.is_empty() {
                metadata.exif.set("Comment", AttrValue::Str(comment));
            }
            metadata.exif.set("Track", AttrValue::UInt(data[126] as u32));
        } else {
            let comment = self.trim_null_string(&data[97..127]);
            if !comment.is_empty() {
                metadata.exif.set("Comment", AttrValue::Str(comment));
            }
        }

        // Genre: byte 127
        let genre = self.genre_name(data[127]);
        if genre != "Unknown" {
            metadata.exif.set("Genre", AttrValue::Str(genre.to_string()));
        }

        Ok(())
    }

    /// Trim null bytes and whitespace from string.
    fn trim_null_string(&self, data: &[u8]) -> String {
        let s: String = data
            .iter()
            .take_while(|&&b| b != 0)
            .map(|&b| b as char)
            .collect();
        s.trim().to_string()
    }

    /// Extract MP3 audio info from frame header.
    fn extract_audio_info(&self, reader: &mut dyn ReadSeek, metadata: &mut Metadata) -> Result<()> {
        // Skip ID3v2 header if present
        reader.seek(SeekFrom::Start(0))?;
        let mut header = [0u8; 10];
        reader.read_exact(&mut header)?;

        let start_pos = if &header[0..3] == b"ID3" {
            let size = ((header[6] as u64) << 21)
                | ((header[7] as u64) << 14)
                | ((header[8] as u64) << 7)
                | (header[9] as u64);
            10 + size
        } else {
            0
        };

        reader.seek(SeekFrom::Start(start_pos))?;

        // Search for MP3 frame sync
        let mut buf = [0u8; 4];
        let search_limit = 4096;
        let mut searched = 0u64;

        while searched < search_limit {
            if reader.read_exact(&mut buf).is_err() {
                return Ok(());
            }

            // Check frame sync
            if buf[0] == 0xFF && (buf[1] & 0xE0) == 0xE0 {
                // Found frame sync
                let version = (buf[1] >> 3) & 0x03;
                let layer = (buf[1] >> 1) & 0x03;
                let bitrate_idx = (buf[2] >> 4) & 0x0F;
                let sample_rate_idx = (buf[2] >> 2) & 0x03;
                let channel_mode = (buf[3] >> 6) & 0x03;

                // MPEG version
                let mpeg_version = match version {
                    0 => "MPEG 2.5",
                    2 => "MPEG 2",
                    3 => "MPEG 1",
                    _ => "Unknown",
                };

                // Layer
                let layer_name = match layer {
                    1 => "Layer III",
                    2 => "Layer II",
                    3 => "Layer I",
                    _ => "Unknown",
                };

                // Sample rate
                const SAMPLE_RATES: [[u32; 3]; 4] = [
                    [44100, 22050, 11025], // MPEG 1, 2, 2.5
                    [48000, 24000, 12000],
                    [32000, 16000, 8000],
                    [0, 0, 0], // reserved
                ];
                let version_idx = match version {
                    3 => 0,
                    2 => 1,
                    0 => 2,
                    _ => 0,
                };
                let sample_rate = SAMPLE_RATES[sample_rate_idx as usize][version_idx];

                // Bitrate (kbps) for Layer III
                const BITRATES_L3: [[u16; 2]; 16] = [
                    [0, 0], [32, 8], [40, 16], [48, 24],
                    [56, 32], [64, 40], [80, 48], [96, 56],
                    [112, 64], [128, 80], [160, 96], [192, 112],
                    [224, 128], [256, 144], [320, 160], [0, 0],
                ];
                let bitrate = if version == 3 {
                    BITRATES_L3[bitrate_idx as usize][0]
                } else {
                    BITRATES_L3[bitrate_idx as usize][1]
                };

                // Channel mode
                let channels = match channel_mode {
                    0 => "Stereo",
                    1 => "Joint Stereo",
                    2 => "Dual Channel",
                    3 => "Mono",
                    _ => "Unknown",
                };

                metadata.exif.set("AudioFormat", AttrValue::Str(format!("{} {}", mpeg_version, layer_name)));
                if sample_rate > 0 {
                    metadata.exif.set("SampleRate", AttrValue::UInt(sample_rate));
                }
                if bitrate > 0 {
                    metadata.exif.set("AudioBitrate", AttrValue::Str(format!("{} kbps", bitrate)));
                }
                metadata.exif.set("ChannelMode", AttrValue::Str(channels.to_string()));

                return Ok(());
            }

            // Move back 3 bytes to not miss sync
            reader.seek(SeekFrom::Current(-3))?;
            searched += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_id3v2_header(size: u32) -> Vec<u8> {
        let mut data = vec![b'I', b'D', b'3'];
        data.push(4); // version 2.4
        data.push(0); // revision
        data.push(0); // flags
        // Sync-safe size
        data.push(((size >> 21) & 0x7F) as u8);
        data.push(((size >> 14) & 0x7F) as u8);
        data.push(((size >> 7) & 0x7F) as u8);
        data.push((size & 0x7F) as u8);
        data
    }

    fn make_text_frame(id: &str, text: &str) -> Vec<u8> {
        let mut frame = Vec::new();
        frame.extend_from_slice(id.as_bytes());
        let text_bytes = text.as_bytes();
        let size = 1 + text_bytes.len() as u32; // encoding byte + text
        // Sync-safe size for v2.4
        frame.push(((size >> 21) & 0x7F) as u8);
        frame.push(((size >> 14) & 0x7F) as u8);
        frame.push(((size >> 7) & 0x7F) as u8);
        frame.push((size & 0x7F) as u8);
        frame.push(0); // flags
        frame.push(0);
        frame.push(3); // UTF-8 encoding
        frame.extend_from_slice(text_bytes);
        frame
    }

    #[test]
    fn detect_id3v2() {
        let parser = Id3Parser;
        let header = [b'I', b'D', b'3', 4, 0, 0, 0, 0, 0, 0];
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn detect_mp3_frame() {
        let parser = Id3Parser;
        let header = [0xFF, 0xFB, 0x90, 0x00]; // MP3 frame sync
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn reject_non_mp3() {
        let parser = Id3Parser;
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0])); // JPEG
        assert!(!parser.can_parse(&[0x89, b'P', b'N', b'G'])); // PNG
    }

    #[test]
    fn parse_id3v2_title() {
        let parser = Id3Parser;

        let title_frame = make_text_frame("TIT2", "Test Song");
        let mut data = make_id3v2_header(title_frame.len() as u32);
        data.extend_from_slice(&title_frame);

        let mut cursor = Cursor::new(&data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("Title"), Some("Test Song"));
        assert_eq!(meta.exif.get_str("ID3Version"), Some("2.4.0"));
    }

    #[test]
    fn parse_genre_numeric() {
        let parser = Id3Parser;
        assert_eq!(parser.decode_genre("(17)"), "Rock");
        assert_eq!(parser.decode_genre("17"), "Rock");
        assert_eq!(parser.decode_genre("(17)Rock"), "Rock");
        assert_eq!(parser.decode_genre("(17)My Genre"), "My Genre (Rock)");
        assert_eq!(parser.decode_genre("Custom Genre"), "Custom Genre");
    }

    #[test]
    fn test_genre_names() {
        let parser = Id3Parser;
        assert_eq!(parser.genre_name(0), "Blues");
        assert_eq!(parser.genre_name(17), "Rock");
        assert_eq!(parser.genre_name(255), "Unknown");
    }
}

//! ID3 tag writer for MP3 files.
//!
//! Writing strategy:
//! - Build ID3v2.3 tag from metadata
//! - Write ID3v2 tag at beginning
//! - Copy audio data (skip existing ID3v2 if present)
//! - Preserve ID3v1 at end (if present)
//!
//! Supported frames:
//! - TIT2: Title
//! - TPE1: Artist  
//! - TALB: Album
//! - TYER: Year
//! - TRCK: Track number
//! - TCON: Genre
//! - COMM: Comment

use crate::{Error, Metadata, ReadSeek, Result};
use std::io::{SeekFrom, Write};

/// ID3v2.3 writer.
pub struct Id3Writer;

impl Id3Writer {
    /// Write MP3 with updated ID3v2 tags.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        // Read source
        input.seek(SeekFrom::Start(0))?;
        let mut data = Vec::new();
        input.read_to_end(&mut data)?;

        if data.len() < 10 {
            return Err(Error::InvalidStructure("file too small".into()));
        }

        // Find where audio data starts (skip existing ID3v2)
        let audio_start = if &data[0..3] == b"ID3" {
            let size = ((data[6] as usize) << 21)
                | ((data[7] as usize) << 14)
                | ((data[8] as usize) << 7)
                | (data[9] as usize);
            10 + size
        } else {
            0
        };

        // Check for ID3v1 at end
        let has_id3v1 = data.len() >= 128 && &data[data.len() - 128..data.len() - 125] == b"TAG";
        let audio_end = if has_id3v1 {
            data.len() - 128
        } else {
            data.len()
        };

        // Build ID3v2.3 tag
        let id3v2_data = build_id3v2(metadata)?;

        // Write ID3v2 tag
        output.write_all(&id3v2_data)?;

        // Write audio data
        if audio_start < audio_end {
            output.write_all(&data[audio_start..audio_end])?;
        }

        // Preserve ID3v1 if present
        if has_id3v1 {
            output.write_all(&data[data.len() - 128..])?;
        }

        Ok(())
    }
}

/// Build ID3v2.3 tag from metadata.
fn build_id3v2(metadata: &Metadata) -> Result<Vec<u8>> {
    let mut frames = Vec::new();

    // Title
    if let Some(title) = metadata.exif.get_str("Title") {
        frames.extend(build_text_frame(b"TIT2", title));
    }

    // Artist
    if let Some(artist) = metadata.exif.get_str("Artist") {
        frames.extend(build_text_frame(b"TPE1", artist));
    }

    // Album
    if let Some(album) = metadata.exif.get_str("Album") {
        frames.extend(build_text_frame(b"TALB", album));
    }

    // Year
    if let Some(year) = metadata.exif.get_str("Year") {
        frames.extend(build_text_frame(b"TYER", year));
    } else if let Some(year) = metadata.exif.get_u32("Year") {
        frames.extend(build_text_frame(b"TYER", &year.to_string()));
    }

    // Track
    if let Some(track) = metadata.exif.get_str("Track") {
        frames.extend(build_text_frame(b"TRCK", track));
    } else if let Some(track) = metadata.exif.get_u32("Track") {
        frames.extend(build_text_frame(b"TRCK", &track.to_string()));
    }

    // Genre
    if let Some(genre) = metadata.exif.get_str("Genre") {
        frames.extend(build_text_frame(b"TCON", genre));
    }

    // Comment
    if let Some(comment) = metadata.exif.get_str("Comment") {
        frames.extend(build_comment_frame(comment));
    }

    // Album Artist
    if let Some(album_artist) = metadata.exif.get_str("AlbumArtist") {
        frames.extend(build_text_frame(b"TPE2", album_artist));
    }

    // Composer
    if let Some(composer) = metadata.exif.get_str("Composer") {
        frames.extend(build_text_frame(b"TCOM", composer));
    }

    // Build complete ID3v2.3 tag
    let mut tag = Vec::new();

    // Header: "ID3" + version (2.3.0) + flags + size
    tag.extend_from_slice(b"ID3");
    tag.push(3); // version major: 2.3
    tag.push(0); // version minor: 0
    tag.push(0); // flags: none

    // Size as syncsafe integer (4 bytes, 7 bits each)
    let size = frames.len();
    tag.push(((size >> 21) & 0x7F) as u8);
    tag.push(((size >> 14) & 0x7F) as u8);
    tag.push(((size >> 7) & 0x7F) as u8);
    tag.push((size & 0x7F) as u8);

    // Frames
    tag.extend(frames);

    Ok(tag)
}

/// Build ID3v2.3 text frame.
fn build_text_frame(id: &[u8; 4], text: &str) -> Vec<u8> {
    let mut frame = Vec::new();

    // Frame ID (4 bytes)
    frame.extend_from_slice(id);

    // Frame data: encoding (1 byte) + text
    let text_bytes = text.as_bytes();
    let data_size = 1 + text_bytes.len();

    // Size (4 bytes, big-endian, NOT syncsafe in v2.3)
    frame.extend_from_slice(&(data_size as u32).to_be_bytes());

    // Flags (2 bytes)
    frame.push(0);
    frame.push(0);

    // Encoding: 0 = ISO-8859-1 (we use UTF-8 which is compatible for ASCII)
    frame.push(0);

    // Text data
    frame.extend_from_slice(text_bytes);

    frame
}

/// Build ID3v2.3 comment frame (COMM).
fn build_comment_frame(text: &str) -> Vec<u8> {
    let mut frame = Vec::new();

    // Frame ID
    frame.extend_from_slice(b"COMM");

    // Frame data: encoding (1) + language (3) + short desc (null-term) + text
    let text_bytes = text.as_bytes();
    let data_size = 1 + 3 + 1 + text_bytes.len(); // encoding + lang + null + text

    // Size (4 bytes)
    frame.extend_from_slice(&(data_size as u32).to_be_bytes());

    // Flags (2 bytes)
    frame.push(0);
    frame.push(0);

    // Encoding: 0 = ISO-8859-1
    frame.push(0);

    // Language: "eng"
    frame.extend_from_slice(b"eng");

    // Short description (empty, null-terminated)
    frame.push(0);

    // Text
    frame.extend_from_slice(text_bytes);

    frame
}

#[cfg(test)]
mod tests {
    use super::*;
    use exiftool_attrs::AttrValue;
    use std::io::Cursor;

    fn make_minimal_mp3() -> Vec<u8> {
        // Minimal valid MP3: frame sync + padding (>=10 bytes)
        vec![
            0xFF, 0xFB, 0x90, 0x00, // MP3 frame sync + header
            0x00, 0x00, 0x00, 0x00, // padding
            0x00, 0x00, 0x00, 0x00, // more padding
        ]
    }

    fn make_mp3_with_id3v2() -> Vec<u8> {
        let mut data = Vec::new();
        
        // ID3v2.3 header with 10 bytes of frames
        data.extend_from_slice(b"ID3");
        data.push(3); // version 2.3
        data.push(0);
        data.push(0); // flags
        data.push(0); data.push(0); data.push(0); data.push(10); // size = 10 (syncsafe)

        // Dummy frame data (10 bytes)
        data.extend_from_slice(&[0u8; 10]);

        // MP3 audio data
        data.extend_from_slice(&[0xFF, 0xFB, 0x90, 0x00]);

        data
    }

    #[test]
    fn test_write_basic() {
        let input_data = make_minimal_mp3();
        let mut input = Cursor::new(&input_data);
        let mut output = Vec::new();

        let mut meta = Metadata::new("MP3");
        meta.exif.set("Title", AttrValue::Str("Test Song".into()));
        meta.exif.set("Artist", AttrValue::Str("Test Artist".into()));

        Id3Writer::write(&mut input, &mut output, &meta).unwrap();

        // Should have ID3v2 header
        assert_eq!(&output[0..3], b"ID3");
        assert_eq!(output[3], 3); // version 2.3

        // Should contain original audio
        assert!(output.windows(4).any(|w| w == [0xFF, 0xFB, 0x90, 0x00]));
    }

    #[test]
    fn test_write_replaces_id3v2() {
        let input_data = make_mp3_with_id3v2();
        let mut input = Cursor::new(&input_data);
        let mut output = Vec::new();

        let mut meta = Metadata::new("MP3");
        meta.exif.set("Title", AttrValue::Str("New Title".into()));

        Id3Writer::write(&mut input, &mut output, &meta).unwrap();

        // Should have new ID3v2
        assert_eq!(&output[0..3], b"ID3");
        
        // Should contain "New Title" in output
        let output_str = String::from_utf8_lossy(&output);
        assert!(output_str.contains("New Title"));
    }

    #[test]
    fn test_build_text_frame() {
        let frame = build_text_frame(b"TIT2", "Hello");
        
        assert_eq!(&frame[0..4], b"TIT2");
        // Size = 6 (1 encoding + 5 text bytes)
        assert_eq!(&frame[4..8], &[0, 0, 0, 6]);
        // Flags
        assert_eq!(&frame[8..10], &[0, 0]);
        // Encoding
        assert_eq!(frame[10], 0);
        // Text
        assert_eq!(&frame[11..], b"Hello");
    }

    #[test]
    fn test_preserves_id3v1() {
        let mut input_data = make_minimal_mp3();
        
        // Add ID3v1 tag at end
        input_data.extend_from_slice(b"TAG");
        input_data.extend_from_slice(&[0u8; 125]); // rest of 128-byte tag

        let mut input = Cursor::new(&input_data);
        let mut output = Vec::new();

        let meta = Metadata::new("MP3");
        Id3Writer::write(&mut input, &mut output, &meta).unwrap();

        // Should preserve ID3v1 at end
        assert_eq!(&output[output.len() - 128..output.len() - 125], b"TAG");
    }
}

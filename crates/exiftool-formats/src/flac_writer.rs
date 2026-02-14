//! FLAC format writer.
//!
//! Writes/updates VORBIS_COMMENT block from metadata.
//! Preserves all other metadata blocks and audio data.

use crate::{Error, Metadata, ReadSeek, Result};
use std::io::{SeekFrom, Write};

/// FLAC format writer.
pub struct FlacWriter;

impl FlacWriter {
    /// Write FLAC with updated Vorbis comments.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        input.seek(SeekFrom::Start(0))?;
        let data = crate::utils::read_with_limit(input)?;

        if data.len() < 8 {
            return Err(Error::InvalidStructure("FLAC file too small".into()));
        }
        if &data[0..4] != b"fLaC" {
            return Err(Error::InvalidStructure("Invalid FLAC magic".into()));
        }

        output.write_all(b"fLaC")?;

        let vorbis_block = build_vorbis_comment(metadata)?;
        let mut pos = 4;

        while pos < data.len() {
            if pos + 4 > data.len() {
                break;
            }

            let is_last = data[pos] & 0x80 != 0;
            let block_type = data[pos] & 0x7F;
            let block_size = ((data[pos + 1] as u32) << 16)
                | ((data[pos + 2] as u32) << 8)
                | (data[pos + 3] as u32);

            if block_size > 16 * 1024 * 1024 {
                return Err(Error::InvalidStructure("FLAC block too large".into()));
            }

            let block_end = pos + 4 + block_size as usize;
            if block_end > data.len() {
                break;
            }

            if block_type == 4 {
                // VORBIS_COMMENT - replace with our block
                write_block_header(output, is_last, 4, vorbis_block.len() as u32)?;
                output.write_all(&vorbis_block)?;
            } else {
                // Copy block as-is
                output.write_all(&data[pos..block_end])?;
            }

            pos = block_end;
            if is_last {
                break;
            }
        }

        // Copy remaining data (audio frames)
        if pos < data.len() {
            output.write_all(&data[pos..])?;
        }

        Ok(())
    }
}

fn write_block_header<W: Write>(w: &mut W, is_last: bool, block_type: u8, size: u32) -> Result<()> {
    let first = (if is_last { 0x80 } else { 0x00 }) | block_type;
    w.write_all(&[first, (size >> 16) as u8, (size >> 8) as u8, size as u8])?;
    Ok(())
}

fn build_vorbis_comment(metadata: &Metadata) -> Result<Vec<u8>> {
    let mut out = Vec::new();

    let vendor = b"exiftool-rs";
    out.extend_from_slice(&(vendor.len() as u32).to_le_bytes());
    out.extend_from_slice(vendor);

    let mut comments = Vec::new();

    // Map metadata to Vorbis field names
    let field_mapping = [
        ("Title", "TITLE"),
        ("Artist", "ARTIST"),
        ("Album", "ALBUM"),
        ("Year", "DATE"),
        ("Track", "TRACKNUMBER"),
        ("Genre", "GENRE"),
        ("Comment", "COMMENT"),
        ("AlbumArtist", "ALBUMARTIST"),
        ("Composer", "COMPOSER"),
        ("Copyright", "COPYRIGHT"),
        ("Publisher", "ORGANIZATION"),
    ];

    for (key, vorbis_field) in field_mapping {
        if let Some(val) = metadata.exif.get_str(key) {
            if !val.is_empty() {
                comments.push(format!("{}={}", vorbis_field, val));
            }
        }
    }

    // Add any Vorbis: prefixed fields
    for (tag, value) in metadata.exif.iter() {
        if let Some(field) = tag.strip_prefix("Vorbis:") {
            if let Some(s) = value.as_str() {
                comments.push(format!("{}={}", field.to_uppercase(), s));
            }
        }
    }

    out.extend_from_slice(&(comments.len() as u32).to_le_bytes());
    for comment in comments {
        let bytes = comment.as_bytes();
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(bytes);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use exiftool_attrs::AttrValue;
    use std::io::Cursor;

    fn make_minimal_flac() -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(b"fLaC");
        // STREAMINFO block (last)
        data.push(0x80); // last, type 0
        data.push(0); data.push(0); data.push(34);
        data.extend_from_slice(&[0u8; 34]);
        data
    }

    #[test]
    fn write_flac_roundtrip() {
        let input = make_minimal_flac();
        let mut meta = Metadata::new("FLAC");
        meta.exif.set("Title", AttrValue::Str("Test".into()));
        meta.exif.set("Artist", AttrValue::Str("Artist".into()));

        let mut output = Vec::new();
        FlacWriter::write(&mut Cursor::new(&input), &mut output, &meta).unwrap();

        assert_eq!(&output[0..4], b"fLaC");
        assert_eq!(output.len(), 42); // minimal FLAC without VORBIS_COMMENT (we don't add when none exists)
    }

    #[test]
    fn write_flac_replaces_vorbis_comment() {
        // FLAC with STREAMINFO (not last) + VORBIS_COMMENT (TITLE=Old)
        let mut input = make_minimal_flac();
        input[4] = 0x00; // streaminfo not last
        let vendor = b"test";
        let comment = b"TITLE=Old";
        let mut vc = Vec::new();
        vc.extend_from_slice(&(vendor.len() as u32).to_le_bytes());
        vc.extend_from_slice(vendor);
        vc.extend_from_slice(&1u32.to_le_bytes());
        vc.extend_from_slice(&(comment.len() as u32).to_le_bytes());
        vc.extend_from_slice(comment);
        input.push(0x84); // last, type 4
        let len = vc.len() as u32;
        input.push((len >> 16) as u8);
        input.push((len >> 8) as u8);
        input.push(len as u8);
        input.extend_from_slice(&vc);

        let mut meta = Metadata::new("FLAC");
        meta.exif.set("Title", AttrValue::Str("NewTitle".into()));

        let mut output = Vec::new();
        FlacWriter::write(&mut Cursor::new(&input), &mut output, &meta).unwrap();

        let s = String::from_utf8_lossy(&output);
        assert!(s.contains("NewTitle"));
        assert!(!s.contains("Old"));
    }
}

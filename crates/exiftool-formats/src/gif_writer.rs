//! GIF format writer.
//!
//! Injects/replaces Comment Extension (0x21 0xFE) from metadata.
//! Preserves image data and other extensions.

use crate::{Error, Metadata, ReadSeek, Result};
use std::io::{SeekFrom, Write};

/// GIF format writer.
pub struct GifWriter;

impl GifWriter {
    /// Write GIF with updated Comment from metadata.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        input.seek(SeekFrom::Start(0))?;
        let data = crate::utils::read_with_limit(input)?;

        if data.len() < 13 {
            return Err(Error::InvalidStructure("GIF file too small".into()));
        }
        if &data[0..3] != b"GIF" {
            return Err(Error::InvalidStructure("Invalid GIF magic".into()));
        }

        // Comment to write
        let comment = metadata
            .exif
            .get_str("Comment")
            .unwrap_or("")
            .as_bytes()
            .to_vec();
        let comment = if comment.len() > 255 {
            &comment[..255]
        } else {
            &comment[..]
        };

        // Header (6) + LSD (7) = 13
        let has_gct = data.len() > 13 && (data[10] & 0x80) != 0;
        let mut pos = 13;

        if has_gct {
            let gct_size_bits = data[10] & 0x07;
            let gct_entries = 1 << (gct_size_bits + 1);
            pos += gct_entries * 3;
        }

        output.write_all(&data[0..pos])?;

        // Write our Comment Extension
        if !comment.is_empty() {
            output.write_all(&[0x21, 0xFE])?;
            // Sub-blocks: max 255 bytes each
            let mut i = 0;
            while i < comment.len() {
                let chunk_len = (comment.len() - i).min(255) as u8;
                output.write_all(&[chunk_len])?;
                output.write_all(&comment[i..i + chunk_len as usize])?;
                i += chunk_len as usize;
            }
            output.write_all(&[0])?; // terminator
        }

        // Copy rest, but skip existing Comment Extensions (0x21 0xFE)
        while pos < data.len() {
            if data[pos] == 0x21 && pos + 1 < data.len() && data[pos + 1] == 0xFE {
                // Skip existing comment extension
                pos += 2;
                while pos < data.len() && data[pos] != 0 {
                    let block_len = data[pos] as usize;
                    pos += 1 + block_len;
                }
                if pos < data.len() {
                    pos += 1; // skip terminator 0
                }
            } else {
                output.write_all(&[data[pos]])?;
                pos += 1;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use exiftool_attrs::AttrValue;
    use std::io::Cursor;

    fn make_minimal_gif() -> Vec<u8> {
        let mut d = Vec::new();
        d.extend_from_slice(b"GIF89a");
        d.extend_from_slice(&[10u8, 0, 10, 0]); // 10x10
        d.push(0); // packed (no GCT)
        d.push(0); // bg color
        d.push(0); // aspect
        d.extend_from_slice(&[0x2C, 0, 0, 0, 0, 10, 0, 10, 0, 0]); // image descriptor
        d.push(2); // LZW min code size
        d.push(3); // block: 3 bytes
        d.extend_from_slice(&[1, 2, 3]);
        d.push(0); // terminator
        d.push(0x3B); // trailer
        d
    }

    #[test]
    fn write_gif_injects_comment() {
        let input = make_minimal_gif();
        let mut meta = Metadata::new("GIF");
        meta.exif.set("Comment", AttrValue::Str("test".into()));

        let mut output = Vec::new();
        GifWriter::write(&mut Cursor::new(&input), &mut output, &meta).unwrap();

        assert!(output.starts_with(b"GIF89a"));
        // Our comment: 0x21 0xFE 0x04 "test" 0x00
        let idx = output.windows(4).position(|w| w == b"\x21\xFE\x04t");
        assert!(idx.is_some());
    }

    #[test]
    fn write_gif_preserves_image() {
        let input = make_minimal_gif();
        let meta = Metadata::new("GIF");

        let mut output = Vec::new();
        GifWriter::write(&mut Cursor::new(&input), &mut output, &meta).unwrap();

        assert_eq!(output[output.len() - 1], 0x3B);
    }
}

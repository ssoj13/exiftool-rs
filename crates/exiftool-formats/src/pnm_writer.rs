//! PNM (Netpbm) format writer.
//!
//! Injects metadata as # comment lines after the magic number.
//! Preserves image data; replaces existing comments.

use crate::{Error, Metadata, ReadSeek, Result};
use std::io::{SeekFrom, Write};

/// PNM format writer.
pub struct PnmWriter;

impl PnmWriter {
    /// Write PNM with comment lines from metadata.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        input.seek(SeekFrom::Start(0))?;
        let data = crate::utils::read_with_limit(input)?;

        if data.len() < 2 {
            return Err(Error::InvalidStructure("PNM file too small".into()));
        }
        if data[0] != b'P' || !matches!(data[1], b'1'..=b'7') {
            return Err(Error::InvalidStructure("Invalid PNM magic".into()));
        }

        // Find header structure: (payload_start, magic_len)
        // payload_start = first byte of "width height maxval" (skip original comments)
        let (payload_start, magic_len) = parse_header(&data)?;

        // Write magic line
        output.write_all(&data[0..magic_len])?;
        output.write_all(b"\n")?;

        // Write comment lines from metadata
        let comment_lines = metadata_to_comments(metadata);
        for line in &comment_lines {
            output.write_all(b"# ")?;
            output.write_all(line.as_bytes())?;
            output.write_all(b"\n")?;
        }

        // Write dimensions + image data (payload_start points to first digit of width)
        output.write_all(&data[payload_start..])?;

        Ok(())
    }
}

/// Find payload start: position of first digit of width (after magic and any comments).
/// Returns (payload_start, magic_len). Payload = "width height [maxval]\\n" + image.
fn parse_header(data: &[u8]) -> Result<(usize, usize)> {
    if data.len() < 2 {
        return Err(Error::InvalidStructure("PNM header too short".into()));
    }
    let magic_len = 2;
    let mut pos = 2;

    // Skip to end of first line (newline after magic)
    while pos < data.len() && data[pos] != b'\n' && data[pos] != b'\r' {
        pos += 1;
    }
    if pos < data.len() {
        pos += 1;
    }

    if data[1] == b'7' {
        // PAM: different format, read until ENDHDR
        return parse_pam_header(data, pos);
    }

    // Skip comments (# ...) and whitespace to find first digit of width
    let mut in_comment = false;
    while pos < data.len() {
        let ch = data[pos];
        if in_comment {
            if ch == b'\n' || ch == b'\r' {
                in_comment = false;
            }
            pos += 1;
            continue;
        }
        if ch == b'#' {
            in_comment = true;
            pos += 1;
            continue;
        }
        if ch.is_ascii_digit() {
            break; // found first digit of width
        }
        if ch.is_ascii_whitespace() {
            pos += 1;
            continue;
        }
        pos += 1;
    }

    Ok((pos, magic_len))
}

fn parse_pam_header(data: &[u8], start: usize) -> Result<(usize, usize)> {
    let mut pos = start;

    while pos + 7 <= data.len() {
        if &data[pos..pos + 7] == b"ENDHDR\n" || &data[pos..pos + 7] == b"ENDHDR\r" {
            pos += 7;
            return Ok((pos, 2));
        }
        if pos + 6 <= data.len() && data[pos..].starts_with(b"ENDHDR") {
            pos += 6;
            while pos < data.len() && data[pos] != b'\n' && data[pos] != b'\r' {
                pos += 1;
            }
            if pos < data.len() {
                pos += 1;
            }
            return Ok((pos, 2));
        }
        // Skip line
        while pos < data.len() && data[pos] != b'\n' && data[pos] != b'\r' {
            pos += 1;
        }
        if pos < data.len() {
            pos += 1;
        }
    }

    Err(Error::InvalidStructure("PAM ENDHDR not found".into()))
}

fn metadata_to_comments(metadata: &Metadata) -> Vec<String> {
    let mut lines = Vec::new();

    let field_mapping = [
        ("Title", "Title"),
        ("Artist", "Artist"),
        ("ImageDescription", "ImageDescription"),
        ("Comment", "Comment"),
        ("Copyright", "Copyright"),
        ("Software", "Software"),
    ];

    for (key, label) in field_mapping {
        if let Some(val) = metadata.exif.get_str(key) {
            if !val.is_empty() && !val.contains('\n') {
                lines.push(format!("{}={}", label, val));
            }
        }
    }

    // File:Comment from parser (original PNM comments)
    if let Some(comment) = metadata.exif.get_str("File:Comment") {
        for line in comment.split('\n') {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                lines.push(trimmed.to_string());
            }
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use exiftool_attrs::AttrValue;
    use std::io::Cursor;

    #[test]
    fn write_pnm_injects_comment() {
        let input = b"P6\n640 480\n255\n\x00\x01\x02";
        let mut meta = Metadata::new("PPM");
        meta.exif.set("Comment", AttrValue::Str("test comment".into()));

        let mut output = Vec::new();
        PnmWriter::write(&mut Cursor::new(&input), &mut output, &meta).unwrap();

        let s = String::from_utf8_lossy(&output);
        assert!(s.starts_with("P6\n"));
        assert!(s.contains("# Comment=test comment"));
        assert!(s.contains("640 480"));
    }

    #[test]
    fn write_pnm_preserves_image_data() {
        let input = b"P5\n2 2\n255\n\x11\x22\x33\x44";
        let meta = Metadata::new("PGM");

        let mut output = Vec::new();
        PnmWriter::write(&mut Cursor::new(&input), &mut output, &meta).unwrap();

        assert_eq!(&output[output.len() - 4..], b"\x11\x22\x33\x44");
    }
}

//! Fujifilm RAF writer — ExifTool `FujiFilm.pm` `WriteRAF`.
//!
//! Rewrite the embedded JPEG (EXIF lives there), pad to 4 bytes, fix header
//! pointers that sit before the JPEG, copy the RAF directories + CFA from the
//! original `nextPtr` at 0x5C.

use crate::{Error, JpegWriter, Metadata, ReadSeek, Result};
use std::io::{Cursor, Write};

const RAF_MAGIC: &[u8; 16] = b"FUJIFILMCCD-RAW ";
const HEADER_MIN: usize = 0x94;
const PTRS: &[usize] = &[0x5C, 0x64, 0x78, 0x80, 0xCC, 0x114, 0x164];

/// Fujifilm RAF format writer.
pub struct RafWriter;

impl RafWriter {
    /// Write RAF with updated EXIF in the preview JPEG; CFA is copied verbatim.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        let data = crate::utils::read_with_limit(input)?;
        if data.len() < HEADER_MIN || &data[..16] != RAF_MAGIC {
            return Err(Error::InvalidStructure("invalid RAF magic".into()));
        }

        let ver = &data[0x3C..0x40];
        if !ver.iter().all(|b| b.is_ascii_digit()) {
            return Err(Error::InvalidStructure("unsupported RAF version".into()));
        }

        let mpos = be_u32(&data, 0x48);
        let mlen = be_u32(&data, 0x4C);
        let jpos = be_u32(&data, 0x54) as usize;
        let jlen = be_u32(&data, 0x58) as usize;

        // ExifTool: ($mpos > 0x94 or $jpos > 0x94 + $mlen) or $jpos < 0x68 or $jpos & 0x03
        if mpos > 0x94 || (jpos as u32) > 0x94 + mlen || jpos < 0x68 || jpos & 0x03 != 0 {
            return Err(Error::InvalidStructure(
                "unsupported or corrupted RAF image".into(),
            ));
        }
        if jlen == 0 || jpos + jlen > data.len() {
            return Err(Error::InvalidStructure("truncated RAF JPEG".into()));
        }

        let mut hdr = data[..HEADER_MIN].to_vec();
        if mpos != 0 {
            if mlen != 0x11C {
                return Err(Error::InvalidStructure("unsupported M-RAW header".into()));
            }
            let ms = mpos as usize;
            let me = ms + mlen as usize;
            if me > data.len() {
                return Err(Error::InvalidStructure("truncated M-RAW header".into()));
            }
            hdr.extend_from_slice(&data[ms..me]);
            if hdr.len() < 0x118
                || hdr[0xC0..0xC8] != [0u8; 8]
                || hdr[0xC8..0xD0] != hdr[0x110..0x118]
            {
                return Err(Error::InvalidStructure(
                    "unexpected M-RAW header layout".into(),
                ));
            }
        }

        if jpos > hdr.len() {
            return Err(Error::InvalidStructure(
                "RAF JPEG starts past header".into(),
            ));
        }

        let jpeg = &data[jpos..jpos + jlen];
        let mut jpeg_in = Cursor::new(jpeg);
        let mut out_jpeg = Vec::new();
        JpegWriter::write_metadata(&mut jpeg_in, &mut out_jpeg, metadata)?;
        if out_jpeg.is_empty() {
            return Err(Error::InvalidStructure("invalid RAF JPEG rewrite".into()));
        }

        // ExifTool: "\0" x (4 - ($jpegLen % 4)) -- always 1..=4 bytes.
        let pad_len = 4 - (out_jpeg.len() % 4);
        let pad = vec![0u8; pad_len];

        set_be_u32(&mut hdr, 0x58, out_jpeg.len() as u32);
        let next_ptr = be_u32(&hdr, 0x5C) as usize;
        if next_ptr < jpos + jlen || next_ptr > data.len() {
            return Err(Error::InvalidStructure("bad RAF pointer at 0x5c".into()));
        }
        let old_pad = next_ptr - (jpos + jlen);
        if old_pad > 1_000_000 {
            return Err(Error::InvalidStructure("bad RAF pointer at 0x5c".into()));
        }
        let ptr_diff = out_jpeg.len() as i64 + pad_len as i64 - (jlen as i64 + old_pad as i64);

        for &off in PTRS {
            if off >= jpos {
                break;
            }
            if off + 4 > hdr.len() {
                break;
            }
            let old = be_u32(&hdr, off);
            if old == 0 {
                continue;
            }
            apply_raf_ptr(&mut hdr, off, old, ptr_diff)?;
        }

        output.write_all(&hdr[..jpos])?;
        output.write_all(&out_jpeg)?;
        output.write_all(&pad)?;
        output.write_all(&data[next_ptr..])?;
        Ok(())
    }
}

fn be_u32(buf: &[u8], off: usize) -> u32 {
    u32::from_be_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}

fn set_be_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_be_bytes());
}

/// Update a 32-bit RAF header pointer; 0xCC+ may be the low word of an 8-byte int.
fn apply_raf_ptr(hdr: &mut [u8], offset: usize, old: u32, ptr_diff: i64) -> Result<()> {
    let mut new_ptr = i64::from(old) + ptr_diff;
    if (0..=0xFFFF_FFFF).contains(&new_ptr) {
        set_be_u32(hdr, offset, new_ptr as u32);
        return Ok(());
    }
    if offset < 0xCC {
        return Err(Error::InvalidStructure(
            "invalid offset in RAF header".into(),
        ));
    }
    if offset < 4 {
        return Err(Error::InvalidStructure("RAF header offset error".into()));
    }
    let mut high = i64::from(be_u32(hdr, offset - 4));
    if new_ptr < 0 {
        high -= 1;
        new_ptr += 1 << 32;
        if high < 0 {
            return Err(Error::InvalidStructure("RAF header offset error".into()));
        }
    } else {
        high += 1;
        new_ptr -= 1 << 32;
    }
    if !(0..=0xFFFF_FFFF).contains(&new_ptr) || !(0..=0xFFFF_FFFF).contains(&high) {
        return Err(Error::InvalidStructure("RAF header offset error".into()));
    }
    set_be_u32(hdr, offset - 4, high as u32);
    set_be_u32(hdr, offset, new_ptr as u32);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FormatParser, RafParser};
    use exiftool_attrs::AttrValue;
    use std::io::Cursor;
    use std::path::PathBuf;

    #[test]
    fn reject_invalid_magic() {
        let invalid = vec![0u8; 100];
        let mut input = Cursor::new(&invalid);
        let mut output = Vec::new();
        let metadata = Metadata::new("RAF");
        assert!(RafWriter::write(&mut input, &mut output, &metadata).is_err());
    }

    #[test]
    fn write_preview_exif_keeps_raf_directory() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/FujiFilm.raf");
        if !path.exists() {
            return;
        }
        let data = std::fs::read(&path).unwrap();
        let orig_next = u32::from_be_bytes(data[0x5C..0x60].try_into().unwrap()) as usize;
        let tail = data[orig_next..].to_vec();

        let mut metadata = RafParser.parse(&mut Cursor::new(&data)).unwrap();
        metadata
            .exif
            .set("Artist", AttrValue::Str("exiftool-rs".into()));
        metadata.exif.set("Quality", AttrValue::Str("FINE".into()));

        let mut out = Vec::new();
        RafWriter::write(&mut Cursor::new(&data), &mut out, &metadata).unwrap();
        assert_eq!(&out[..16], RAF_MAGIC);

        let new_next = u32::from_be_bytes(out[0x5C..0x60].try_into().unwrap()) as usize;
        assert_eq!(
            &out[new_next..],
            tail.as_slice(),
            "CFA/RAF dir tail must be byte-identical"
        );

        let parsed = RafParser.parse(&mut Cursor::new(&out)).unwrap();
        assert_eq!(parsed.exif.get_str("Artist"), Some("exiftool-rs"));
        assert!(
            out.windows(5).any(|w| w == b"FINE\0"),
            "FINE missing from rewritten RAF bytes"
        );
        assert_eq!(parsed.exif.get_str("Quality"), Some("FINE"));
        assert_eq!(parsed.exif.get_str("RAFVersion"), Some("0106"));
        assert_eq!(parsed.exif.get_str("RawImageFullSize"), Some("4352x1444"));
    }
}

//! Canon CR2 format writer.
//!
//! CR2 is TIFF-based with a 4-byte extension at offset 8: "CR" + version.
//! Delegates to TiffWriter and patches the header to preserve CR2 structure.

use crate::{Error, Metadata, ReadSeek, Result, TiffWriter};
use std::io::Write;

/// Canon CR2 format writer.
pub struct Cr2Writer;

impl Cr2Writer {
    /// Write CR2 with updated metadata.
    ///
    /// Uses TiffWriter internally and injects the CR2 marker ("CR" + version 2.0)
    /// after the standard 8-byte TIFF header.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        let mut buf = Vec::new();
        TiffWriter::write(input, &mut buf, metadata)?;

        if buf.len() < 8 {
            return Err(Error::InvalidStructure("TIFF output too short".into()));
        }

        // Insert CR2 marker (4 bytes) after byte 8
        let mut out: Vec<u8> = Vec::with_capacity(buf.len() + 4);
        out.extend_from_slice(&buf[0..8]);
        out.extend_from_slice(b"CR\x02\x00");
        out.extend_from_slice(&buf[8..]);

        // Update IFD offset (bytes 4-7): add 4 for the inserted bytes
        let byte_order = buf[0] == b'I'; // II = LE, MM = BE
        let old_offset = if byte_order {
            u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]])
        } else {
            u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]])
        };
        let new_offset = old_offset + 4;
        let offset_bytes = if byte_order {
            new_offset.to_le_bytes()
        } else {
            new_offset.to_be_bytes()
        };
        out[4..8].copy_from_slice(&offset_bytes);

        output.write_all(&out)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn write_cr2_injects_marker() {
        // Use a minimal TIFF (no CR2 marker) - Cr2Writer adds it
        let minimal_tiff = crate::tiff_writer::TiffWriter::write_new;
        let mut metadata = crate::Metadata::new("CR2");
        metadata.exif.set("Make", exiftool_attrs::AttrValue::Str("Canon".into()));
        metadata.exif.set("Model", exiftool_attrs::AttrValue::Str("EOS R5".into()));

        let mut tiff_output = Vec::new();
        minimal_tiff(&mut tiff_output, &metadata).unwrap();

        let mut output = Vec::new();
        Cr2Writer::write(
            &mut Cursor::new(&tiff_output),
            &mut output,
            &metadata,
        )
        .unwrap();

        assert_eq!(&output[0..2], b"II");
        assert_eq!(&output[8..12], b"CR\x02\x00");
    }
}

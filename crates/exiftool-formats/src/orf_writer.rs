//! Olympus ORF format writer.
//!
//! ORF uses standard TIFF or Olympus magic (IIRO/IIRS at bytes 2-3).
//! Delegates to TiffWriter and patches the magic for ORF compatibility.

use crate::{Error, Metadata, ReadSeek, Result, TiffWriter};
use std::io::Write;

/// Olympus ORF format writer.
pub struct OrfWriter;

impl OrfWriter {
    /// Write ORF with updated metadata.
    ///
    /// Uses TiffWriter and patches bytes 2-3 to "RO" (0x4F52) for Olympus
    /// compatibility when the source was standard TIFF.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        let mut buf = Vec::new();
        TiffWriter::write(input, &mut buf, metadata)?;

        if buf.len() < 4 {
            return Err(Error::InvalidStructure("TIFF output too short".into()));
        }

        // ORF uses IIRO or IIRS instead of II 0x2A at bytes 2-3
        // Preserve original if already ORF, else use IIRO (0x4F52 LE)
        let is_orf = (buf[2] == b'R' && buf[3] == b'O') || (buf[2] == b'R' && buf[3] == b'S');
        if !is_orf && buf[0] == b'I' && buf[1] == b'I' {
            buf[2] = b'R';
            buf[3] = b'O';
        }

        output.write_all(&buf)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TiffWriter;
    use std::io::Cursor;

    #[test]
    fn write_orf_patches_magic() {
        let mut metadata = crate::Metadata::new("ORF");
        metadata.exif.set("Make", exiftool_attrs::AttrValue::Str("OLYMPUS CORPORATION".into()));

        let mut tiff_out = Vec::new();
        TiffWriter::write_new(&mut tiff_out, &metadata).unwrap();

        let mut output = Vec::new();
        OrfWriter::write(&mut Cursor::new(&tiff_out), &mut output, &metadata).unwrap();

        assert_eq!(&output[0..4], b"IIRO");
    }
}

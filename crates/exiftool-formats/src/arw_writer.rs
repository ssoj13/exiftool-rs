//! Sony ARW format writer.
//!
//! Delegates to `TiffWriter` / `tiff_rewrite`. Original DSLR-A100 files get ExifTool
//! `FinishARW`: 0x14a is CFA offset (not SubIFD); Minolta MRW at 0xc634 is appended after the TIFF.

use crate::{Metadata, ReadSeek, Result, TiffWriter};
use std::io::Write;

/// Sony ARW format writer.
pub struct ArwWriter;

impl ArwWriter {
    /// Write ARW with updated metadata.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        TiffWriter::write(input, output, metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TiffWriter;
    use std::io::Cursor;

    #[test]
    fn write_arw_roundtrip() {
        let mut metadata = crate::Metadata::new("ARW");
        metadata.exif.set("Make", exiftool_attrs::AttrValue::Str("SONY".into()));
        metadata.exif.set("Model", exiftool_attrs::AttrValue::Str("ILCE-7M3".into()));

        let mut tiff_out = Vec::new();
        TiffWriter::write_new(&mut tiff_out, &metadata).unwrap();

        let mut output = Vec::new();
        ArwWriter::write(&mut Cursor::new(&tiff_out), &mut output, &metadata).unwrap();

        assert_eq!(&output[0..2], b"II");
    }
}

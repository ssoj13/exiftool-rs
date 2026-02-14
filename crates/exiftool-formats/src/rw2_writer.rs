//! Panasonic RW2 format writer.
//!
//! RW2 is TIFF-based. Delegates to TiffWriter.

use crate::{Metadata, ReadSeek, Result, TiffWriter};
use std::io::Write;

/// Panasonic RW2 format writer.
pub struct Rw2Writer;

impl Rw2Writer {
    /// Write RW2 with updated metadata.
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
    fn write_rw2_roundtrip() {
        let mut metadata = crate::Metadata::new("RW2");
        metadata.exif.set("Make", exiftool_attrs::AttrValue::Str("Panasonic".into()));
        metadata.exif.set("Model", exiftool_attrs::AttrValue::Str("DC-G9".into()));

        let mut tiff_out = Vec::new();
        TiffWriter::write_new(&mut tiff_out, &metadata).unwrap();

        let mut output = Vec::new();
        Rw2Writer::write(&mut Cursor::new(&tiff_out), &mut output, &metadata).unwrap();

        assert_eq!(&output[0..2], b"II");
    }
}

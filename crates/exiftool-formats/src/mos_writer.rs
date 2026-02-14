//! Leaf MOS format writer.
//!
//! MOS is TIFF-based. Delegates to TiffWriter.

use crate::{Metadata, ReadSeek, Result, TiffWriter};
use std::io::Write;

/// Leaf MOS format writer.
pub struct MosWriter;

impl MosWriter {
    /// Write MOS with updated metadata.
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
    fn write_mos_roundtrip() {
        let mut metadata = crate::Metadata::new("MOS");
        metadata.exif.set("Make", exiftool_attrs::AttrValue::Str("Leaf".into()));
        metadata.exif.set("Model", exiftool_attrs::AttrValue::Str("Credo 60".into()));

        let mut tiff_out = Vec::new();
        TiffWriter::write_new(&mut tiff_out, &metadata).unwrap();

        let mut output = Vec::new();
        MosWriter::write(&mut Cursor::new(&tiff_out), &mut output, &metadata).unwrap();

        assert_eq!(&output[0..2], b"II");
    }
}

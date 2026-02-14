//! Epson ERF format writer.
//!
//! ERF is TIFF-based. Delegates to TiffWriter.

use crate::{Metadata, ReadSeek, Result, TiffWriter};
use std::io::Write;

/// Epson ERF format writer.
pub struct ErfWriter;

impl ErfWriter {
    /// Write ERF with updated metadata.
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
    fn write_erf_roundtrip() {
        let mut metadata = crate::Metadata::new("ERF");
        metadata.exif.set("Make", exiftool_attrs::AttrValue::Str("EPSON".into()));
        metadata.exif.set("Model", exiftool_attrs::AttrValue::Str("R-D1".into()));

        let mut tiff_out = Vec::new();
        TiffWriter::write_new(&mut tiff_out, &metadata).unwrap();

        let mut output = Vec::new();
        ErfWriter::write(&mut Cursor::new(&tiff_out), &mut output, &metadata).unwrap();

        assert_eq!(&output[0..2], b"II");
    }
}

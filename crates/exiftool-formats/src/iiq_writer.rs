//! Phase One IIQ format writer.
//!
//! IIQ is TIFF-based. Delegates to TiffWriter.

use crate::{Metadata, ReadSeek, Result, TiffWriter};
use std::io::Write;

/// Phase One IIQ format writer.
pub struct IiqWriter;

impl IiqWriter {
    /// Write IIQ with updated metadata.
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
    fn write_iiq_roundtrip() {
        let mut metadata = crate::Metadata::new("IIQ");
        metadata.exif.set("Make", exiftool_attrs::AttrValue::Str("Phase One".into()));
        metadata.exif.set("Model", exiftool_attrs::AttrValue::Str("IQ3 100MP".into()));

        let mut tiff_out = Vec::new();
        TiffWriter::write_new(&mut tiff_out, &metadata).unwrap();

        let mut output = Vec::new();
        IiqWriter::write(&mut Cursor::new(&tiff_out), &mut output, &metadata).unwrap();

        assert_eq!(&output[0..2], b"II");
    }
}

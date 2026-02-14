//! Pentax PEF format writer.
//!
//! PEF is TIFF-based. Delegates to TiffWriter.

use crate::{Metadata, ReadSeek, Result, TiffWriter};
use std::io::Write;

/// Pentax PEF format writer.
pub struct PefWriter;

impl PefWriter {
    /// Write PEF with updated metadata.
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
    fn write_pef_roundtrip() {
        let mut metadata = crate::Metadata::new("PEF");
        metadata.exif.set("Make", exiftool_attrs::AttrValue::Str("PENTAX".into()));
        metadata.exif.set("Model", exiftool_attrs::AttrValue::Str("K-1".into()));

        let mut tiff_out = Vec::new();
        TiffWriter::write_new(&mut tiff_out, &metadata).unwrap();

        let mut output = Vec::new();
        PefWriter::write(&mut Cursor::new(&tiff_out), &mut output, &metadata).unwrap();

        assert_eq!(&output[0..2], b"II");
    }
}

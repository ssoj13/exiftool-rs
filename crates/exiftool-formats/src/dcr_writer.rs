//! Kodak DCR/KDC/K25 format writer.
//!
//! All Kodak RAW formats are TIFF-based. Delegates to TiffWriter.

use crate::{Metadata, ReadSeek, Result, TiffWriter};
use std::io::Write;

/// Kodak DCR/KDC/K25 format writer.
pub struct DcrWriter;

impl DcrWriter {
    /// Write DCR/KDC/K25 with updated metadata.
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
    fn write_dcr_roundtrip() {
        let mut metadata = crate::Metadata::new("DCR");
        metadata.exif.set("Make", exiftool_attrs::AttrValue::Str("Kodak".into()));
        metadata.exif.set("Model", exiftool_attrs::AttrValue::Str("DCS Pro 14n".into()));

        let mut tiff_out = Vec::new();
        TiffWriter::write_new(&mut tiff_out, &metadata).unwrap();

        let mut output = Vec::new();
        DcrWriter::write(&mut Cursor::new(&tiff_out), &mut output, &metadata).unwrap();

        assert_eq!(&output[0..2], b"II");
    }
}

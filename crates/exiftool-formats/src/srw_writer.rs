//! Samsung SRW format writer.
//!
//! SRW is TIFF-based. Delegates to TiffWriter.

use crate::{Metadata, ReadSeek, Result, TiffWriter};
use std::io::Write;

/// Samsung SRW format writer.
pub struct SrwWriter;

impl SrwWriter {
    /// Write SRW with updated metadata.
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
    fn write_srw_roundtrip() {
        let mut metadata = crate::Metadata::new("SRW");
        metadata.exif.set("Make", exiftool_attrs::AttrValue::Str("SAMSUNG".into()));
        metadata.exif.set("Model", exiftool_attrs::AttrValue::Str("NX500".into()));

        let mut tiff_out = Vec::new();
        TiffWriter::write_new(&mut tiff_out, &metadata).unwrap();

        let mut output = Vec::new();
        SrwWriter::write(&mut Cursor::new(&tiff_out), &mut output, &metadata).unwrap();

        assert_eq!(&output[0..2], b"II");
    }
}

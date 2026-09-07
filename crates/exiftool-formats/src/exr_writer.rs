//! OpenEXR format writer.
//!
//! Reloads pixels via `exr-core` `Image`, then writes scanline EXR with updated
//! string header attributes (`comments`, `owner`, `software`, …).

use crate::{Metadata, ReadSeek, Result};
use exr_core::attr::{Attribute, ExrString, StringAttribute};
use exr_core::{Image, MemIStream, MemOStream};
use std::io::Write;

/// OpenEXR format writer.
pub struct ExrWriter;

impl ExrWriter {
    /// Write EXR with updated metadata, preserving image data.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        let source_data = crate::utils::read_with_limit(input)?;
        let mut istream = MemIStream::new(source_data);
        let image = Image::read_from(&mut istream).map_err(|e| {
            crate::Error::InvalidStructure(format!("EXR read: {e}"))
        })?;
        let extra = extra_attrs(metadata);
        let mut ostream = MemOStream::new();
        image
            .write_to_with_attrs(&mut ostream, image.compression, extra)
            .map_err(|e| crate::Error::InvalidStructure(format!("EXR write: {e}")))?;
        output.write_all(ostream.data())?;
        Ok(())
    }
}

fn extra_attrs(metadata: &Metadata) -> Vec<(String, Box<dyn Attribute>)> {
    let mut extra = Vec::new();
    let mut push = |name: &str, value: &str| {
        extra.push((
            name.to_string(),
            Box::new(StringAttribute::new(ExrString::from(value))) as Box<dyn Attribute>,
        ));
    };
    if let Some(v) = metadata.exif.get_str("Software") {
        push("software", v);
    }
    if let Some(v) = metadata.exif.get_str("Artist") {
        push("owner", v);
    }
    if let Some(v) = metadata.exif.get_str("ImageDescription") {
        push("comments", v);
    }
    if let Some(v) = metadata.exif.get_str("DateTime") {
        push("capDate", v);
    }
    if let Some(v) = metadata.exif.get_str("Make") {
        push("cameraMake", v);
    }
    if let Some(v) = metadata.exif.get_str("Model") {
        push("cameraModel", v);
    }
    extra
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ExrParser, FormatParser, Metadata};
    use exiftool_attrs::AttrValue;
    use std::io::Cursor;

    #[test]
    fn rewrite_openexr_fixture_comments() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/testdata/OpenEXR.exr");
        if !path.exists() {
            return;
        }
        let data = std::fs::read(&path).unwrap();
        let mut metadata = Metadata::new("EXR");
        metadata
            .exif
            .set("ImageDescription", AttrValue::Str("exiftool-rs".into()));
        let mut out = Vec::new();
        ExrWriter::write(&mut Cursor::new(&data), &mut out, &metadata).unwrap();
        let parsed = ExrParser.parse(&mut Cursor::new(&out)).unwrap();
        assert_eq!(parsed.exif.get_str("ImageDescription"), Some("exiftool-rs"));
    }
}

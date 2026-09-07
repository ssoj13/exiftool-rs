//! Nikon NEF writer.
//!
//! `write` uses `tiff_rewrite` (ExifTool WriteTIFF / ProcessTIFF style):
//! overlay IFD0 / ExifIFD / GPS tags, copy SubIFD trees, strips/tiles, MakerNotes blobs.
//! MakerNotes field tables are not rewritten.

use crate::{Error, Metadata, ReadSeek, Result};
use std::io::Write;

/// Nikon NEF format writer.
pub struct NefWriter;

impl NefWriter {
    /// Rewrite NEF, preserving SubIFD / raw / MakerNotes blobs.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        let data = crate::utils::read_with_limit(input)?;
        let rewritten = crate::tiff_rewrite::rewrite_preserving(&data, metadata)?;
        output.write_all(&rewritten)?;
        Ok(())
    }

    /// Write standalone NEF metadata (no raw data).
    ///
    /// Creates a valid TIFF structure with Nikon-specific metadata.
    /// Useful for sidecar files or testing.
    pub fn write_metadata<W: Write>(output: &mut W, metadata: &Metadata) -> Result<()> {
        use exiftool_attrs::AttrValue;
        use exiftool_core::writer::tags;
        use exiftool_core::{ExifWriter, WriteEntry};

        let mut writer = ExifWriter::new_le();

        // IFD0 - ensure Nikon Make
        let make = metadata.exif.get_str("Make").unwrap_or("NIKON CORPORATION");
        writer.add_ifd0(WriteEntry::from_str(tags::MAKE, make));

        if let Some(v) = metadata.exif.get_str("Model") {
            writer.add_ifd0(WriteEntry::from_str(tags::MODEL, v));
        }
        if let Some(v) = metadata.exif.get_str("Software") {
            writer.add_ifd0(WriteEntry::from_str(tags::SOFTWARE, v));
        }
        if let Some(v) = metadata.exif.get_str("DateTime") {
            writer.add_ifd0(WriteEntry::from_str(tags::DATE_TIME, v));
        }
        if let Some(v) = metadata.exif.get_str("Artist") {
            writer.add_ifd0(WriteEntry::from_str(tags::ARTIST, v));
        }
        if let Some(v) = metadata.exif.get_str("Copyright") {
            writer.add_ifd0(WriteEntry::from_str(tags::COPYRIGHT, v));
        }

        if let Some(AttrValue::UInt(v)) = metadata.exif.get("Orientation") {
            writer.add_ifd0(WriteEntry::from_u16(tags::ORIENTATION, *v as u16));
        }

        // Resolution
        if let Some(AttrValue::URational(n, d)) = metadata.exif.get("XResolution") {
            writer.add_ifd0(WriteEntry::from_urational(tags::X_RESOLUTION, *n, *d));
        } else {
            writer.add_ifd0(WriteEntry::from_urational(tags::X_RESOLUTION, 300, 1));
        }
        if let Some(AttrValue::URational(n, d)) = metadata.exif.get("YResolution") {
            writer.add_ifd0(WriteEntry::from_urational(tags::Y_RESOLUTION, *n, *d));
        } else {
            writer.add_ifd0(WriteEntry::from_urational(tags::Y_RESOLUTION, 300, 1));
        }

        // ExifIFD
        if let Some(v) = metadata.exif.get_str("DateTimeOriginal") {
            writer.add_exif(WriteEntry::from_str(tags::DATE_TIME_ORIGINAL, v));
        }
        if let Some(v) = metadata.exif.get_str("CreateDate") {
            writer.add_exif(WriteEntry::from_str(tags::CREATE_DATE, v));
        }

        if let Some(AttrValue::UInt(v)) = metadata.exif.get("ISO") {
            writer.add_exif(WriteEntry::from_u16(tags::ISO, *v as u16));
        }
        if let Some(AttrValue::URational(n, d)) = metadata.exif.get("ExposureTime") {
            writer.add_exif(WriteEntry::from_urational(tags::EXPOSURE_TIME, *n, *d));
        }
        if let Some(AttrValue::URational(n, d)) = metadata.exif.get("FNumber") {
            writer.add_exif(WriteEntry::from_urational(tags::FNUMBER, *n, *d));
        }
        if let Some(AttrValue::URational(n, d)) = metadata.exif.get("FocalLength") {
            writer.add_exif(WriteEntry::from_urational(tags::FOCAL_LENGTH, *n, *d));
        }

        // GPS
        if let Some(v) = metadata.exif.get_str("GPSLatitudeRef") {
            writer.add_gps(WriteEntry::from_str(tags::GPS_LATITUDE_REF, v));
        }
        if let Some(v) = metadata.exif.get_str("GPSLongitudeRef") {
            writer.add_gps(WriteEntry::from_str(tags::GPS_LONGITUDE_REF, v));
        }

        let bytes = writer
            .serialize()
            .map_err(|e| Error::InvalidStructure(format!("NEF serialize error: {}", e)))?;

        output.write_all(&bytes)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use exiftool_attrs::AttrValue;

    #[test]
    fn write_nef_metadata() {
        let mut metadata = Metadata::new("NEF");
        metadata
            .exif
            .set("Make", AttrValue::Str("NIKON CORPORATION".into()));
        metadata
            .exif
            .set("Model", AttrValue::Str("NIKON Z 8".into()));
        metadata.exif.set("ISO", AttrValue::UInt(800));

        let mut output = Vec::new();
        NefWriter::write_metadata(&mut output, &metadata).unwrap();

        // Check TIFF header
        assert_eq!(&output[0..2], b"II");
        assert_eq!(output[2], 0x2A);
    }

    #[test]
    fn nef_default_make() {
        let metadata = Metadata::new("NEF");

        let mut output = Vec::new();
        NefWriter::write_metadata(&mut output, &metadata).unwrap();

        // Parse back and check Make is set to NIKON
        use crate::{FormatParser, TiffParser};
        use std::io::Cursor;

        let mut cursor = Cursor::new(&output);
        let parsed = TiffParser::default().parse(&mut cursor).unwrap();

        assert_eq!(parsed.exif.get_str("Make"), Some("NIKON CORPORATION"));
    }

    #[test]
    fn write_nef_keeps_subifd_raw() {
        use crate::{FormatParser, NefParser};
        use std::io::Cursor;
        use std::path::PathBuf;

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/Nikon.nef");
        if !path.exists() {
            return;
        }
        let data = std::fs::read(&path).unwrap();
        let mut metadata = NefParser::new().parse(&mut Cursor::new(&data)).unwrap();
        metadata
            .exif
            .set("Artist", AttrValue::Str("exiftool-rs".into()));

        let mut output = Vec::new();
        NefWriter::write(&mut Cursor::new(&data), &mut output, &metadata).unwrap();
        assert!(
            output.len() > 1000,
            "rewritten NEF too small: {}",
            output.len()
        );

        let parsed = NefParser::new().parse(&mut Cursor::new(&output)).unwrap();
        assert_eq!(parsed.exif.get_str("Artist"), Some("exiftool-rs"));
        assert_eq!(
            parsed.exif.get("SubIFD:ImageWidth").map(|v| v.to_string()),
            Some("3040".into())
        );
        assert_eq!(parsed.exif.get_str("LensDataVersion"), Some("0101"));
    }
}

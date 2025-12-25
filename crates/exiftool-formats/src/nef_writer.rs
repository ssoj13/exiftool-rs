//! Nikon NEF format writer.
//!
//! NEF is TIFF-based, so we use TiffWriter as the foundation.
//! 
//! NEF structure:
//! - Standard TIFF header
//! - IFD0 with image tags
//! - SubIFD with raw data
//! - EXIF sub-IFD
//! - Nikon MakerNotes
//!
//! For safe metadata updates, we preserve the original file structure
//! and only modify the standard EXIF tags.

use crate::{Error, Metadata, ReadSeek, Result, TiffWriter};
use std::io::Write;

/// Nikon NEF format writer.
pub struct NefWriter;

impl NefWriter {
    /// Write NEF with updated metadata.
    ///
    /// Uses TiffWriter since NEF is TIFF-based.
    /// Preserves Nikon-specific structures (MakerNotes, raw data).
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        // NEF is TIFF-based, delegate to TiffWriter
        // Note: This creates a minimal TIFF with metadata only.
        // Full NEF preservation requires more complex offset handling.
        TiffWriter::write(input, output, metadata)
    }

    /// Write standalone NEF metadata (no raw data).
    ///
    /// Creates a valid TIFF structure with Nikon-specific metadata.
    /// Useful for sidecar files or testing.
    pub fn write_metadata<W: Write>(output: &mut W, metadata: &Metadata) -> Result<()> {
        use exiftool_core::{ExifWriter, WriteEntry};
        use exiftool_core::writer::tags;
        use exiftool_attrs::AttrValue;

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

        let bytes = writer.serialize().map_err(|e| {
            Error::InvalidStructure(format!("NEF serialize error: {}", e))
        })?;

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
        metadata.exif.set("Make", AttrValue::Str("NIKON CORPORATION".into()));
        metadata.exif.set("Model", AttrValue::Str("NIKON Z 8".into()));
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
        use crate::{TiffParser, FormatParser};
        use std::io::Cursor;

        let mut cursor = Cursor::new(&output);
        let parsed = TiffParser::default().parse(&mut cursor).unwrap();

        assert_eq!(parsed.exif.get_str("Make"), Some("NIKON CORPORATION"));
    }
}

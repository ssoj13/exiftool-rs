//! Fujifilm RAF format writer.
//!
//! RAF writing strategy:
//! - Copy RAF header (first 0x54 bytes)
//! - Modify embedded JPEG preview with new EXIF
//! - Update JPEG offset/length in header if size changed
//! - Copy CFA (raw) data verbatim

use crate::{Error, JpegWriter, Metadata, ReadSeek, Result};
use std::io::{Cursor, Seek, Write};

/// RAF magic signature.
const RAF_MAGIC: &[u8; 16] = b"FUJIFILMCCD-RAW ";

/// Fujifilm RAF format writer.
pub struct RafWriter;

impl RafWriter {
    /// Write RAF with updated EXIF metadata.
    ///
    /// Modifies the embedded JPEG preview while preserving raw data.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write + Seek,
    {
        // Read entire RAF file (with size limit)
        let data = crate::utils::read_with_limit(input)?;

        if data.len() < 0x64 || &data[..16] != RAF_MAGIC {
            return Err(Error::InvalidStructure("invalid RAF magic".into()));
        }

        // Parse header offsets (big-endian)
        let jpeg_offset = u32::from_be_bytes([data[0x54], data[0x55], data[0x56], data[0x57]]) as usize;
        let jpeg_length = u32::from_be_bytes([data[0x58], data[0x59], data[0x5A], data[0x5B]]) as usize;
        let cfa_offset = u32::from_be_bytes([data[0x5C], data[0x5D], data[0x5E], data[0x5F]]) as usize;
        let cfa_length = u32::from_be_bytes([data[0x60], data[0x61], data[0x62], data[0x63]]) as usize;

        if jpeg_offset == 0 || jpeg_length == 0 || jpeg_offset + jpeg_length > data.len() {
            return Err(Error::InvalidStructure("RAF: invalid JPEG offset/length".into()));
        }

        // Extract and modify embedded JPEG
        let jpeg_data = &data[jpeg_offset..jpeg_offset + jpeg_length];
        let mut jpeg_cursor = Cursor::new(jpeg_data);
        let mut new_jpeg = Vec::new();

        // Build new EXIF for JPEG
        let exif_bytes = Self::build_exif(metadata)?;

        JpegWriter::write(&mut jpeg_cursor, &mut new_jpeg, Some(&exif_bytes), None)?;

        // Calculate new offsets
        let new_jpeg_length = new_jpeg.len();
        let size_diff = new_jpeg_length as i64 - jpeg_length as i64;
        let new_cfa_offset = (cfa_offset as i64 + size_diff) as usize;

        // Write RAF header (first 0x54 bytes, unchanged)
        output.write_all(&data[..0x54])?;

        // Write updated offsets
        output.write_all(&(jpeg_offset as u32).to_be_bytes())?;  // JPEG offset unchanged
        output.write_all(&(new_jpeg_length as u32).to_be_bytes())?;  // New JPEG length
        output.write_all(&(new_cfa_offset as u32).to_be_bytes())?;  // Adjusted CFA offset
        output.write_all(&(cfa_length as u32).to_be_bytes())?;  // CFA length unchanged

        // Copy remaining header bytes (0x64 to jpeg_offset)
        if jpeg_offset > 0x64 {
            output.write_all(&data[0x64..jpeg_offset])?;
        }

        // Write modified JPEG
        output.write_all(&new_jpeg)?;

        // Pad if new JPEG is smaller (optional, keeps structure aligned)
        if size_diff < 0 {
            let padding = (-size_diff) as usize;
            output.write_all(&vec![0u8; padding])?;
        }

        // Copy CFA data and anything after
        if cfa_offset > 0 && cfa_offset < data.len() {
            output.write_all(&data[cfa_offset..])?;
        }

        Ok(())
    }

    /// Build EXIF bytes from metadata.
    fn build_exif(metadata: &Metadata) -> Result<Vec<u8>> {
        use exiftool_core::{ExifWriter, WriteEntry};
        use exiftool_core::writer::tags;
        use exiftool_attrs::AttrValue;

        let mut writer = ExifWriter::new_le();

        // IFD0 tags
        if let Some(v) = metadata.exif.get_str("Make") {
            writer.add_ifd0(WriteEntry::from_str(tags::MAKE, v));
        } else {
            writer.add_ifd0(WriteEntry::from_str(tags::MAKE, "FUJIFILM"));
        }

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
        if let Some(AttrValue::UInt(v)) = metadata.exif.get("Orientation") {
            writer.add_ifd0(WriteEntry::from_u16(tags::ORIENTATION, *v as u16));
        }

        // ExifIFD
        if let Some(v) = metadata.exif.get_str("DateTimeOriginal") {
            writer.add_exif(WriteEntry::from_str(tags::DATE_TIME_ORIGINAL, v));
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

        writer.serialize().map_err(|e| {
            Error::InvalidStructure(format!("EXIF serialize error: {}", e))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Full RAF testing requires sample RAF files
    // These tests verify the basic structure

    #[test]
    fn reject_invalid_magic() {
        let invalid = vec![0u8; 100];
        let mut input = Cursor::new(&invalid);
        let mut output = Cursor::new(Vec::new());
        let metadata = Metadata::new("RAF");

        let result = RafWriter::write(&mut input, &mut output, &metadata);
        assert!(result.is_err());
    }

    #[test]
    fn build_exif_works() {
        use exiftool_attrs::AttrValue;

        let mut metadata = Metadata::new("RAF");
        metadata.exif.set("Make", AttrValue::Str("FUJIFILM".into()));
        metadata.exif.set("Model", AttrValue::Str("X-T5".into()));

        let exif = RafWriter::build_exif(&metadata).unwrap();
        
        // Should have TIFF header
        assert_eq!(&exif[0..2], b"II"); // Little-endian
    }
}

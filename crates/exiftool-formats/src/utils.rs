//! Shared utilities for format parsers.

use crate::{Error, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use exiftool_core::writer::tags;
use exiftool_core::{ExifWriter, IfdEntry, RawValue, WriteEntry};
use std::io::SeekFrom;

/// Maximum file size to read into memory (100 MB).
pub const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;

/// Read entire file into memory with size limit check.
///
/// Returns error if file exceeds MAX_FILE_SIZE to prevent OOM attacks.
pub fn read_with_limit<R: ReadSeek + ?Sized>(reader: &mut R) -> Result<Vec<u8>> {
    read_with_limit_custom(reader, MAX_FILE_SIZE)
}

/// Read entire file into memory with custom size limit.
pub fn read_with_limit_custom<R: ReadSeek + ?Sized>(reader: &mut R, max_size: u64) -> Result<Vec<u8>> {
    // Get file size
    let current = reader.stream_position()?;
    let end = reader.seek(SeekFrom::End(0))?;
    let size = end - current;
    reader.seek(SeekFrom::Start(current))?;

    if size > max_size {
        return Err(Error::FileTooLarge(size, max_size));
    }

    let mut data = Vec::with_capacity(size as usize);
    reader.read_to_end(&mut data)?;
    Ok(data)
}

/// Convert IFD entry to AttrValue.
///
/// Single source of truth for IFD → Attr conversion used by all format parsers.
pub fn entry_to_attr(entry: &IfdEntry) -> AttrValue {
    match &entry.value {
        RawValue::String(s) => AttrValue::Str(s.clone()),
        RawValue::UInt8(v) if v.len() == 1 => AttrValue::UInt(v[0] as u32),
        RawValue::UInt16(v) if v.len() == 1 => AttrValue::UInt(v[0] as u32),
        RawValue::UInt32(v) if v.len() == 1 => AttrValue::UInt(v[0]),
        RawValue::Int8(v) if v.len() == 1 => AttrValue::Int(v[0] as i32),
        RawValue::Int16(v) if v.len() == 1 => AttrValue::Int(v[0] as i32),
        RawValue::Int32(v) if v.len() == 1 => AttrValue::Int(v[0]),
        RawValue::URational(v) if v.len() == 1 => AttrValue::URational(v[0].num, v[0].den),
        RawValue::SRational(v) if v.len() == 1 => AttrValue::Rational(v[0].num, v[0].den),
        RawValue::Float(v) if v.len() == 1 => AttrValue::Float(v[0]),
        RawValue::Double(v) if v.len() == 1 => AttrValue::Double(v[0]),
        RawValue::Undefined(v) => AttrValue::Bytes(v.clone()),
        // Arrays and other types - convert to string representation
        _ => AttrValue::Str(entry.value.to_string()),
    }
}

/// Build EXIF TIFF bytes from metadata.
///
/// Single source of truth for EXIF serialization used by all format writers.
/// Returns TIFF-formatted EXIF data ready for embedding in image files.
pub fn build_exif_bytes(metadata: &Metadata) -> Result<Vec<u8>> {
    let mut w = ExifWriter::new_le();

    // IFD0 string tags
    if let Some(v) = metadata.exif.get_str("Make") {
        w.add_ifd0(WriteEntry::from_str(tags::MAKE, v));
    }
    if let Some(v) = metadata.exif.get_str("Model") {
        w.add_ifd0(WriteEntry::from_str(tags::MODEL, v));
    }
    if let Some(v) = metadata.exif.get_str("Software") {
        w.add_ifd0(WriteEntry::from_str(tags::SOFTWARE, v));
    }
    if let Some(v) = metadata.exif.get_str("DateTime") {
        w.add_ifd0(WriteEntry::from_str(tags::DATE_TIME, v));
    }
    if let Some(v) = metadata.exif.get_str("Artist") {
        w.add_ifd0(WriteEntry::from_str(tags::ARTIST, v));
    }
    if let Some(v) = metadata.exif.get_str("Copyright") {
        w.add_ifd0(WriteEntry::from_str(tags::COPYRIGHT, v));
    }
    if let Some(v) = metadata.exif.get_str("ImageDescription") {
        w.add_ifd0(WriteEntry::from_str(tags::IMAGE_DESCRIPTION, v));
    }

    // IFD0 numeric tags
    if let Some(AttrValue::UInt(v)) = metadata.exif.get("Orientation") {
        w.add_ifd0(WriteEntry::from_u16(tags::ORIENTATION, *v as u16));
    }

    // ExifIFD string tags
    if let Some(v) = metadata.exif.get_str("DateTimeOriginal") {
        w.add_exif(WriteEntry::from_str(tags::DATE_TIME_ORIGINAL, v));
    }
    if let Some(v) = metadata.exif.get_str("CreateDate") {
        w.add_exif(WriteEntry::from_str(tags::CREATE_DATE, v));
    }

    // ExifIFD numeric tags
    if let Some(AttrValue::UInt(v)) = metadata.exif.get("ISO") {
        w.add_exif(WriteEntry::from_u16(tags::ISO, *v as u16));
    }
    if let Some(AttrValue::URational(n, d)) = metadata.exif.get("ExposureTime") {
        w.add_exif(WriteEntry::from_urational(tags::EXPOSURE_TIME, *n, *d));
    }
    if let Some(AttrValue::URational(n, d)) = metadata.exif.get("FNumber") {
        w.add_exif(WriteEntry::from_urational(tags::FNUMBER, *n, *d));
    }
    if let Some(AttrValue::URational(n, d)) = metadata.exif.get("FocalLength") {
        w.add_exif(WriteEntry::from_urational(tags::FOCAL_LENGTH, *n, *d));
    }

    w.serialize().map_err(|e| Error::InvalidStructure(format!("EXIF build failed: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read_with_limit_ok() {
        let data = vec![1u8, 2, 3, 4, 5];
        let mut cursor = Cursor::new(data.clone());
        let result = read_with_limit_custom(&mut cursor, 100).unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn read_with_limit_exceeds() {
        let data = vec![0u8; 100];
        let mut cursor = Cursor::new(data);
        let result = read_with_limit_custom(&mut cursor, 50);
        assert!(matches!(result, Err(Error::FileTooLarge(100, 50))));
    }
}

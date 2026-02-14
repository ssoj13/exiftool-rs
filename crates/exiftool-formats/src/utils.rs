//! Shared utilities for format parsers.
//!
//! # Why
//!
//! Many formats embed EXIF as TIFF (JPEG APP1, PNG eXIf, WebP EXIF, HEIC, JXL, AVI).
//! One implementation avoids duplication and bugs (e.g. missing sub-IFDs).
//!
//! # What
//!
//! - [`parse_tiff_exif`] — parse TIFF bytes → Attrs; handles IFD0, ExifIFD, GPS, Interop, MakerNotes
//! - [`entry_to_attr`] — convert IfdEntry → AttrValue (type-aware)
//! - [`build_exif_bytes`] — Metadata → TIFF bytes for writing
//! - [`ifd_tags`] — constants (thumbnail offset, strip offsets, …)
//!
//! # Where used
//!
//! `parse_tiff_exif`: jpeg.rs, png.rs, webp.rs, heic.rs, jxl.rs, avi.rs.
//! `entry_to_attr`: parse_tiff_exif, tiff.rs. `build_exif_bytes`: all writers.

use crate::tag_lookup::{lookup_exif_subifd, lookup_gps, lookup_ifd0};
use crate::{makernotes, Error, Metadata, ReadSeek, Result};
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::writer::tags;
use exiftool_core::{ByteOrder, ExifWriter, IfdEntry, IfdReader, RawValue, WriteEntry};
use std::io::SeekFrom;

/// IFD tag constants shared across parsers (thumbnail, compression, etc.).
pub mod ifd_tags {
    pub const TAG_THUMBNAIL_OFFSET: u16 = 0x0201;     // JPEGInterchangeFormat
    pub const TAG_THUMBNAIL_LENGTH: u16 = 0x0202;     // JPEGInterchangeFormatLength
    pub const TAG_COMPRESSION: u16 = 0x0103;
    pub const TAG_STRIP_OFFSETS: u16 = 0x0111;
    pub const TAG_STRIP_BYTE_COUNTS: u16 = 0x0117;
    pub const TAG_NEW_SUBFILE_TYPE: u16 = 0x00FE;
    pub const TAG_SUBFILE_TYPE: u16 = 0x00FF;
    pub const TAG_IMAGE_WIDTH: u16 = 0x0100;
    pub const TAG_IMAGE_HEIGHT: u16 = 0x0101;
    pub const TAG_BITS_PER_SAMPLE: u16 = 0x0102;
}

/// Options for parse_tiff_exif.
#[derive(Clone, Default)]
pub struct ParseTiffExifOptions {
    /// Extract JPEG thumbnail from IFD1 (JPEG has it, others typically don't).
    pub extract_thumbnail: bool,
}

/// Parse TIFF-format EXIF data into attrs.
///
/// Single source for JPEG, PNG, WebP, HEIC, JXL, AVI EXIF extraction.
/// Handles IFD0, ExifIFD (0x8769), GPS (0x8825), Interop (0xA005), MakerNotes.
pub fn parse_tiff_exif(
    tiff_data: &[u8],
    exif: &mut Attrs,
    thumbnail: Option<&mut Option<Vec<u8>>>,
    options: ParseTiffExifOptions,
) -> Result<()> {
    if tiff_data.len() < 8 {
        return Ok(());
    }

    let byte_order = ByteOrder::from_marker([tiff_data[0], tiff_data[1]]).map_err(Error::Core)?;
    let reader = IfdReader::new(tiff_data, byte_order);
    let ifd0_offset = reader.parse_header().map_err(Error::Core)?;

    let (entries, next_ifd) = reader.read_ifd(ifd0_offset as u64).map_err(Error::Core)?;

    // First pass: extract Make for MakerNotes vendor detection
    let mut vendor = makernotes::Vendor::Unknown;
    for entry in &entries {
        if entry.tag == 0x010F {
            if let RawValue::String(make) = &entry.value {
                vendor = makernotes::Vendor::from_make(make);
            }
            break;
        }
    }

    // Convert IFD0 entries and handle sub-IFDs
    for entry in &entries {
        if let Some(name) = lookup_ifd0(entry.tag) {
            exif.set(name, entry_to_attr(entry));
        }

        match entry.tag {
            0x8769 => {
                // ExifIFD pointer (Ifd or Ifd64 for BigTIFF)
                if let Some(offset) = entry.value.as_u64() {
                    if let Ok((exif_entries, _)) = reader.read_ifd(offset) {
                        for e in &exif_entries {
                            if e.tag == 0x927C {
                                if let RawValue::Undefined(bytes) = &e.value {
                                    if let Some(mn_data) =
                                        makernotes::parse(bytes, vendor, byte_order)
                                    {
                                        for (key, val) in mn_data.iter() {
                                            exif.set(key.clone(), val.clone());
                                        }
                                    }
                                }
                            } else if let Some(name) = lookup_exif_subifd(e.tag) {
                                exif.set(name, entry_to_attr(e));
                            }
                        }
                    }
                }
            }
            0x8825 => {
                // GPS IFD pointer (Ifd or Ifd64 for BigTIFF)
                if let Some(offset) = entry.value.as_u64() {
                    if let Ok((gps_entries, _)) = reader.read_ifd(offset) {
                        for e in &gps_entries {
                            if let Some(name) = lookup_gps(e.tag) {
                                exif.set(name, entry_to_attr(e));
                            }
                        }
                    }
                }
            }
            0xA005 => {
                // Interop IFD pointer (Ifd or Ifd64 for BigTIFF)
                if let Some(offset) = entry.value.as_u64() {
                    if let Ok((interop_entries, _)) = reader.read_ifd(offset) {
                        for e in &interop_entries {
                            if let Some(name) = lookup_exif_subifd(e.tag) {
                                exif.set(name, entry_to_attr(e));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // IFD1 = thumbnail IFD
    if options.extract_thumbnail {
        if let Some(thumb_out) = thumbnail {
            if next_ifd != 0 {
                if let Ok((ifd1_entries, _)) = reader.read_ifd(next_ifd) {
                    if let Some(data) =
                        extract_jpeg_thumbnail_from_ifd(&ifd1_entries, &reader)
                    {
                        *thumb_out = Some(data);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Extract JPEG thumbnail from IFD entries (IFD1 typically).
fn extract_jpeg_thumbnail_from_ifd(
    entries: &[IfdEntry],
    reader: &IfdReader,
) -> Option<Vec<u8>> {
    let mut thumb_offset: Option<u32> = None;
    let mut thumb_length: Option<u32> = None;
    let mut compression: Option<u16> = None;

    for entry in entries {
        match entry.tag {
            ifd_tags::TAG_THUMBNAIL_OFFSET => thumb_offset = entry.value.as_u32(),
            ifd_tags::TAG_THUMBNAIL_LENGTH => thumb_length = entry.value.as_u32(),
            ifd_tags::TAG_COMPRESSION => {
                if let RawValue::UInt16(v) = &entry.value {
                    compression = v.first().copied();
                }
            }
            _ => {}
        }
    }

    let offset = thumb_offset?;
    let length = thumb_length?;
    let is_jpeg = compression.map(|c| c == 6 || c == 7).unwrap_or(true);
    if !is_jpeg || length == 0 || length >= 1_000_000 {
        return None;
    }

    let data = reader.get_bytes(offset as usize, length as usize)?;
    if data.len() >= 2 && data[0] == 0xFF && data[1] == 0xD8 {
        Some(data.to_vec())
    } else {
        None
    }
}

/// Maximum file size to read into memory (100 MB).
pub const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;

/// Get file size without changing stream position.
///
/// Single source of truth for file size retrieval used by all format parsers.
pub fn get_file_size<R: ReadSeek + ?Sized>(reader: &mut R) -> Result<u64> {
    let current = reader.stream_position()?;
    let size = reader.seek(SeekFrom::End(0))?;
    reader.seek(SeekFrom::Start(current))?;
    Ok(size)
}

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

/// Convert RawValue to AttrValue.
///
/// Used by parsers that need to convert raw IFD values directly.
pub fn raw_value_to_attr(value: &RawValue) -> Option<AttrValue> {
    Some(match value {
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
        RawValue::UInt64(v) if v.len() == 1 => AttrValue::UInt(v[0] as u32),
        RawValue::Int64(v) if v.len() == 1 => AttrValue::Int(v[0] as i32),
        // Arrays - convert to string representation
        _ => AttrValue::Str(value.to_string()),
    })
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
        RawValue::UInt64(v) if v.len() == 1 => AttrValue::UInt(v[0] as u32),
        RawValue::Int64(v) if v.len() == 1 => AttrValue::Int(v[0] as i32),
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

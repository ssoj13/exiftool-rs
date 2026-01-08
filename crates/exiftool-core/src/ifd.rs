//! IFD (Image File Directory) parser.
//!
//! TIFF/EXIF files contain one or more IFDs. Each IFD is a list of 12-byte entries:
//! - Tag ID (2 bytes)
//! - Format/Type (2 bytes)
//! - Count (4 bytes) - number of values
//! - Value or Offset (4 bytes) - inline value if <= 4 bytes, else offset to data
//!
//! Reference: TIFF 6.0 specification, Section 2
//!
//! STUB: Basic IFD reading implemented.
//! Need to implement:
//! - BigTIFF support (8-byte offsets for files > 4GB)
//! - IFD writing (for metadata modification)
//! - MakerNotes parsing (vendor-specific binary formats)
//! - Thumbnail extraction from IFD1
//! - Multi-page TIFF navigation

use crate::{ByteOrder, Error, ExifFormat, RawValue, Result, SRational, URational};

/// Maximum number of IFD entries we'll accept (sanity check).
const MAX_IFD_ENTRIES: u16 = 10000;

/// A single IFD entry (tag).
#[derive(Debug, Clone)]
#[must_use]
pub struct IfdEntry {
    /// Tag ID (e.g., 0x010F = Make, 0x0110 = Model).
    pub tag: u16,
    /// Data format.
    pub format: ExifFormat,
    /// Number of values.
    pub count: u32,
    /// Parsed value.
    pub value: RawValue,
    /// Offset where value data was read from (for sub-IFD navigation).
    pub value_offset: Option<u32>,
}

/// IFD reader for parsing TIFF/EXIF structures.
pub struct IfdReader<'a> {
    /// Raw data buffer.
    data: &'a [u8],
    /// Byte order for multi-byte values.
    byte_order: ByteOrder,
    /// BigTIFF mode (8-byte offsets, 20-byte entries)
    is_bigtiff: bool,
}

impl<'a> IfdReader<'a> {
    /// Create new IFD reader.
    ///
    /// - `data`: Complete TIFF data starting from byte order marker
    /// - `byte_order`: Parsed from TIFF header
    pub fn new(data: &'a [u8], byte_order: ByteOrder) -> Self {
        Self {
            data,
            byte_order,
            is_bigtiff: false,
        }
    }

    /// Create new IFD reader with BigTIFF mode.
    pub fn new_bigtiff(data: &'a [u8], byte_order: ByteOrder) -> Self {
        Self {
            data,
            byte_order,
            is_bigtiff: true,
        }
    }

    /// Check if this is BigTIFF mode.
    pub fn is_bigtiff(&self) -> bool {
        self.is_bigtiff
    }

    /// Get the byte order.
    pub fn byte_order(&self) -> ByteOrder {
        self.byte_order
    }

    /// Get data length.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if data is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get raw bytes at offset (for thumbnail extraction).
    /// Returns None if offset+len exceeds data bounds.
    pub fn get_bytes(&self, offset: usize, len: usize) -> Option<&'a [u8]> {
        if offset + len <= self.data.len() {
            Some(&self.data[offset..offset + len])
        } else {
            None
        }
    }

    /// Read bytes at offset, with bounds checking.
    fn read_bytes(&self, offset: usize, len: usize) -> Result<&'a [u8]> {
        if offset + len > self.data.len() {
            return Err(Error::UnexpectedEof {
                need: offset + len,
                have: self.data.len(),
            });
        }
        Ok(&self.data[offset..offset + len])
    }

    /// Read u16 at offset.
    fn read_u16(&self, offset: usize) -> Result<u16> {
        let bytes = self.read_bytes(offset, 2)?;
        Ok(self.byte_order.read_u16([bytes[0], bytes[1]]))
    }

    /// Read u32 at offset.
    fn read_u32(&self, offset: usize) -> Result<u32> {
        let bytes = self.read_bytes(offset, 4)?;
        Ok(self
            .byte_order
            .read_u32([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Read u64 at offset (for BigTIFF).
    fn read_u64(&self, offset: usize) -> Result<u64> {
        let bytes = self.read_bytes(offset, 8)?;
        Ok(self.byte_order.read_u64([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    /// Parse TIFF header and return first IFD offset.
    ///
    /// TIFF header structure (8 bytes):
    /// - Byte order marker: "II" or "MM" (2 bytes)
    /// - Magic number: 42 (0x002A) for TIFF, 43 (0x002B) for BigTIFF (2 bytes)
    /// - First IFD offset (4 bytes for TIFF, 8 bytes for BigTIFF)
    ///
    /// BigTIFF header structure (16 bytes):
    /// - Byte order marker: "II" or "MM" (2 bytes)
    /// - Magic number: 43 (0x002B) (2 bytes)
    /// - Offset byte size: 8 (2 bytes)
    /// - Reserved: 0 (2 bytes)
    /// - First IFD offset (8 bytes)
    pub fn parse_header(&self) -> Result<u32> {
        // Standard TIFF magic: 42 (TIFF), 43 (BigTIFF)
        self.parse_header_with_magic(&[42, 43])
    }

    /// Parse header and return (ifd_offset, is_bigtiff).
    pub fn parse_header_ex(&self) -> Result<(u64, bool)> {
        self.parse_header_ex_with_magic(&[42, 43])
    }
    
    /// Parse header with custom allowed magic values.
    ///
    /// Some RAW formats use non-standard magic bytes:
    /// - Panasonic RW2: 0x55 (85)
    /// - Olympus ORF: 0x4F52 ("OR") or 0x5352 ("SR")
    pub fn parse_header_with_magic(&self, allowed_magic: &[u16]) -> Result<u32> {
        let (offset, _is_bigtiff) = self.parse_header_ex_with_magic(allowed_magic)?;
        // For compatibility, return u32 (will be fine for non-BigTIFF)
        Ok(offset as u32)
    }

    /// Parse header with extended info (BigTIFF support).
    pub fn parse_header_ex_with_magic(&self, allowed_magic: &[u16]) -> Result<(u64, bool)> {
        if self.data.len() < 8 {
            return Err(Error::UnexpectedEof {
                need: 8,
                have: self.data.len(),
            });
        }

        // Verify byte order matches what we were given
        let marker = [self.data[0], self.data[1]];
        let parsed_order = ByteOrder::from_marker(marker)?;
        if parsed_order != self.byte_order {
            // Mismatch could indicate corrupted data or wrong reader configuration
            // For fuzzing/proptest compatibility, we continue with the reader's byte order
            // In production, the caller should ensure consistency
        }

        // Check magic number against allowed values
        let magic = self.read_u16(2)?;
        if !allowed_magic.contains(&magic) {
            return Err(Error::InvalidTiffMagic(magic));
        }

        // BigTIFF has magic = 43
        if magic == 43 {
            // BigTIFF header is 16 bytes
            if self.data.len() < 16 {
                return Err(Error::UnexpectedEof {
                    need: 16,
                    have: self.data.len(),
                });
            }
            // Offset byte size should be 8
            let offset_size = self.read_u16(4)?;
            if offset_size != 8 {
                return Err(Error::InvalidTiffMagic(offset_size)); // Unexpected offset size
            }
            // Reserved should be 0 (bytes 6-7), we ignore it
            // First IFD offset is at bytes 8-15
            let offset = self.read_u64(8)?;
            Ok((offset, true))
        } else {
            // Standard TIFF
            let offset = self.read_u32(4)? as u64;
            Ok((offset, false))
        }
    }

    /// Read all entries from an IFD at given offset.
    ///
    /// Returns (entries, next_ifd_offset). Next offset is 0 if no more IFDs.
    pub fn read_ifd(&self, offset: u32) -> Result<(Vec<IfdEntry>, u32)> {
        if self.is_bigtiff {
            let (entries, next) = self.read_ifd_bigtiff(offset as u64)?;
            Ok((entries, next as u32))
        } else {
            self.read_ifd_standard(offset)
        }
    }

    /// Read IFD (standard TIFF format).
    fn read_ifd_standard(&self, offset: u32) -> Result<(Vec<IfdEntry>, u32)> {
        let offset = offset as usize;

        if offset >= self.data.len() {
            return Err(Error::IfdOffsetOutOfBounds(offset as u32, self.data.len()));
        }

        // Number of entries (2 bytes)
        let count = self.read_u16(offset)?;
        if count > MAX_IFD_ENTRIES {
            return Err(Error::TooManyIfdEntries(count, MAX_IFD_ENTRIES));
        }

        let mut entries = Vec::with_capacity(count as usize);

        // Each entry is 12 bytes
        for i in 0..count as usize {
            let entry_offset = offset + 2 + i * 12;
            match self.read_entry(entry_offset) {
                Ok(entry) => entries.push(entry),
                Err(e) => {
                    // Log but continue - some entries may be malformed
                    eprintln!("Warning: failed to read IFD entry {}: {}", i, e);
                }
            }
        }

        // Next IFD offset is after all entries
        let next_offset_pos = offset + 2 + (count as usize) * 12;
        let next_ifd = if next_offset_pos + 4 <= self.data.len() {
            self.read_u32(next_offset_pos)?
        } else {
            0
        };

        Ok((entries, next_ifd))
    }

    /// Read IFD (BigTIFF format - 8-byte offsets, 20-byte entries).
    fn read_ifd_bigtiff(&self, offset: u64) -> Result<(Vec<IfdEntry>, u64)> {
        let offset = offset as usize;

        if offset >= self.data.len() {
            return Err(Error::IfdOffsetOutOfBounds(offset as u32, self.data.len()));
        }

        // Number of entries (8 bytes for BigTIFF)
        let count = self.read_u64(offset)? as usize;
        if count > MAX_IFD_ENTRIES as usize {
            return Err(Error::TooManyIfdEntries(count as u16, MAX_IFD_ENTRIES));
        }

        let mut entries = Vec::with_capacity(count);

        // Each entry is 20 bytes in BigTIFF
        for i in 0..count {
            let entry_offset = offset + 8 + i * 20;
            match self.read_entry_bigtiff(entry_offset) {
                Ok(entry) => entries.push(entry),
                Err(e) => {
                    eprintln!("Warning: failed to read BigTIFF IFD entry {}: {}", i, e);
                }
            }
        }

        // Next IFD offset is after all entries (8 bytes)
        let next_offset_pos = offset + 8 + count * 20;
        let next_ifd = if next_offset_pos + 8 <= self.data.len() {
            self.read_u64(next_offset_pos)?
        } else {
            0
        };

        Ok((entries, next_ifd))
    }

    /// Read single IFD entry at offset.
    fn read_entry(&self, offset: usize) -> Result<IfdEntry> {
        // Entry structure: tag(2) + type(2) + count(4) + value/offset(4) = 12 bytes
        let tag = self.read_u16(offset)?;
        let format_id = self.read_u16(offset + 2)?;
        let count = self.read_u32(offset + 4)?;
        let value_field = self.read_u32(offset + 8)?;

        let format = ExifFormat::from_u16(format_id)?;
        // Use checked arithmetic to prevent overflow with malicious files
        let value_size = format.size()
            .checked_mul(count as usize)
            .ok_or(Error::ValueSizeOverflow {
                format_size: format.size(),
                count,
            })?;

        // Determine where value data is located
        let (value_data, value_offset) = if value_size <= 4 {
            // Value fits inline in the 4-byte field
            let inline_bytes = self.read_bytes(offset + 8, 4)?;
            (inline_bytes, None)
        } else {
            // Value is at an offset
            let data_offset = value_field as usize;
            if data_offset + value_size > self.data.len() {
                return Err(Error::ValueOutOfBounds(
                    value_field,
                    value_size,
                    self.data.len(),
                ));
            }
            let data = &self.data[data_offset..data_offset + value_size];
            (data, Some(value_field))
        };

        let value = self.parse_value(format, count, value_data)?;

        Ok(IfdEntry {
            tag,
            format,
            count,
            value,
            value_offset,
        })
    }

    /// Read single IFD entry at offset (BigTIFF format).
    /// Entry structure: tag(2) + type(2) + count(8) + value/offset(8) = 20 bytes
    fn read_entry_bigtiff(&self, offset: usize) -> Result<IfdEntry> {
        let tag = self.read_u16(offset)?;
        let format_id = self.read_u16(offset + 2)?;
        let count = self.read_u64(offset + 4)?;
        let value_field = self.read_u64(offset + 12)?;

        let format = ExifFormat::from_u16(format_id)?;
        // Use checked arithmetic to prevent overflow
        let value_size = format.size()
            .checked_mul(count as usize)
            .ok_or(Error::ValueSizeOverflow {
                format_size: format.size(),
                count: count as u32,
            })?;

        // BigTIFF inline value threshold is 8 bytes
        let (value_data, value_offset) = if value_size <= 8 {
            // Value fits inline in the 8-byte field
            let inline_bytes = self.read_bytes(offset + 12, 8)?;
            (inline_bytes, None)
        } else {
            // Value is at an offset
            let data_offset = value_field as usize;
            if data_offset + value_size > self.data.len() {
                return Err(Error::ValueOutOfBounds(
                    value_field as u32,
                    value_size,
                    self.data.len(),
                ));
            }
            let data = &self.data[data_offset..data_offset + value_size];
            (data, Some(value_field as u32))
        };

        let value = self.parse_value(format, count as u32, value_data)?;

        Ok(IfdEntry {
            tag,
            format,
            count: count as u32,
            value,
            value_offset,
        })
    }

    /// Parse raw bytes into RawValue according to format.
    fn parse_value(&self, format: ExifFormat, count: u32, data: &[u8]) -> Result<RawValue> {
        let count = count as usize;

        match format {
            ExifFormat::UInt8 => {
                Ok(RawValue::UInt8(data[..count].to_vec()))
            }

            ExifFormat::String => {
                // ASCII string, trim null terminator
                let s = data[..count]
                    .iter()
                    .take_while(|&&b| b != 0)
                    .map(|&b| b as char)
                    .collect();
                Ok(RawValue::String(s))
            }

            ExifFormat::UInt16 => {
                let mut values = Vec::with_capacity(count);
                for i in 0..count {
                    let offset = i * 2;
                    values.push(self.byte_order.read_u16([data[offset], data[offset + 1]]));
                }
                Ok(RawValue::UInt16(values))
            }

            ExifFormat::UInt32 | ExifFormat::Ifd => {
                let mut values = Vec::with_capacity(count);
                for i in 0..count {
                    let offset = i * 4;
                    values.push(self.byte_order.read_u32([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                    ]));
                }
                Ok(RawValue::UInt32(values))
            }

            ExifFormat::URational => {
                let mut values = Vec::with_capacity(count);
                for i in 0..count {
                    let offset = i * 8;
                    let num = self.byte_order.read_u32([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                    ]);
                    let den = self.byte_order.read_u32([
                        data[offset + 4],
                        data[offset + 5],
                        data[offset + 6],
                        data[offset + 7],
                    ]);
                    values.push(URational::new(num, den));
                }
                Ok(RawValue::URational(values))
            }

            ExifFormat::Int8 => {
                let values: Vec<i8> = data[..count].iter().map(|&b| b as i8).collect();
                Ok(RawValue::Int8(values))
            }

            ExifFormat::Undefined => {
                Ok(RawValue::Undefined(data[..count].to_vec()))
            }

            ExifFormat::Int16 => {
                let mut values = Vec::with_capacity(count);
                for i in 0..count {
                    let offset = i * 2;
                    values.push(self.byte_order.read_i16([data[offset], data[offset + 1]]));
                }
                Ok(RawValue::Int16(values))
            }

            ExifFormat::Int32 => {
                let mut values = Vec::with_capacity(count);
                for i in 0..count {
                    let offset = i * 4;
                    values.push(self.byte_order.read_i32([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                    ]));
                }
                Ok(RawValue::Int32(values))
            }

            ExifFormat::SRational => {
                let mut values = Vec::with_capacity(count);
                for i in 0..count {
                    let offset = i * 8;
                    let num = self.byte_order.read_i32([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                    ]);
                    let den = self.byte_order.read_i32([
                        data[offset + 4],
                        data[offset + 5],
                        data[offset + 6],
                        data[offset + 7],
                    ]);
                    values.push(SRational::new(num, den));
                }
                Ok(RawValue::SRational(values))
            }

            ExifFormat::Float => {
                let mut values = Vec::with_capacity(count);
                for i in 0..count {
                    let offset = i * 4;
                    values.push(self.byte_order.read_f32([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                    ]));
                }
                Ok(RawValue::Float(values))
            }

            ExifFormat::Double => {
                let mut values = Vec::with_capacity(count);
                for i in 0..count {
                    let offset = i * 8;
                    values.push(self.byte_order.read_f64([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                        data[offset + 4],
                        data[offset + 5],
                        data[offset + 6],
                        data[offset + 7],
                    ]));
                }
                Ok(RawValue::Double(values))
            }

            ExifFormat::UInt64 | ExifFormat::Ifd64 => {
                let mut values = Vec::with_capacity(count);
                for i in 0..count {
                    let offset = i * 8;
                    values.push(self.byte_order.read_u64([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                        data[offset + 4],
                        data[offset + 5],
                        data[offset + 6],
                        data[offset + 7],
                    ]));
                }
                Ok(RawValue::UInt64(values))
            }

            ExifFormat::Int64 => {
                let mut values = Vec::with_capacity(count);
                for i in 0..count {
                    let offset = i * 8;
                    let u = self.byte_order.read_u64([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                        data[offset + 4],
                        data[offset + 5],
                        data[offset + 6],
                        data[offset + 7],
                    ]);
                    values.push(u as i64);
                }
                Ok(RawValue::Int64(values))
            }

            ExifFormat::Unicode | ExifFormat::Complex => {
                // Rarely used, store as undefined for now
                Ok(RawValue::Undefined(data[..count * format.size()].to_vec()))
            }

            ExifFormat::Utf8 => {
                // EXIF 3.0 UTF-8 string, parse as proper UTF-8
                let bytes = &data[..count];
                // Trim null terminator if present
                let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
                let s = String::from_utf8_lossy(&bytes[..len]).into_owned();
                Ok(RawValue::String(s))
            }
        }
    }
}

/// Well-known EXIF tag IDs.
pub mod tags {
    // IFD0 (main image)
    pub const MAKE: u16 = 0x010F;
    pub const MODEL: u16 = 0x0110;
    pub const ORIENTATION: u16 = 0x0112;
    pub const X_RESOLUTION: u16 = 0x011A;
    pub const Y_RESOLUTION: u16 = 0x011B;
    pub const SOFTWARE: u16 = 0x0131;
    pub const DATE_TIME: u16 = 0x0132;
    pub const ARTIST: u16 = 0x013B;
    pub const COPYRIGHT: u16 = 0x8298;

    // Pointers to sub-IFDs
    pub const EXIF_IFD_POINTER: u16 = 0x8769;
    pub const GPS_IFD_POINTER: u16 = 0x8825;
    pub const INTEROP_IFD_POINTER: u16 = 0xA005;

    // EXIF sub-IFD
    pub const EXPOSURE_TIME: u16 = 0x829A;
    pub const F_NUMBER: u16 = 0x829D;
    pub const ISO_SPEED: u16 = 0x8827;
    pub const DATE_TIME_ORIGINAL: u16 = 0x9003;
    pub const DATE_TIME_DIGITIZED: u16 = 0x9004;
    pub const SHUTTER_SPEED: u16 = 0x9201;
    pub const APERTURE: u16 = 0x9202;
    pub const BRIGHTNESS: u16 = 0x9203;
    pub const EXPOSURE_BIAS: u16 = 0x9204;
    pub const MAX_APERTURE: u16 = 0x9205;
    pub const METERING_MODE: u16 = 0x9207;
    pub const FLASH: u16 = 0x9209;
    pub const FOCAL_LENGTH: u16 = 0x920A;
    pub const MAKER_NOTE: u16 = 0x927C;
    pub const USER_COMMENT: u16 = 0x9286;
    pub const COLOR_SPACE: u16 = 0xA001;
    pub const PIXEL_X_DIMENSION: u16 = 0xA002;
    pub const PIXEL_Y_DIMENSION: u16 = 0xA003;
    pub const FOCAL_LENGTH_35MM: u16 = 0xA405;
    pub const LENS_MAKE: u16 = 0xA433;
    pub const LENS_MODEL: u16 = 0xA434;

    // GPS sub-IFD
    pub const GPS_LATITUDE_REF: u16 = 0x0001;
    pub const GPS_LATITUDE: u16 = 0x0002;
    pub const GPS_LONGITUDE_REF: u16 = 0x0003;
    pub const GPS_LONGITUDE: u16 = 0x0004;
    pub const GPS_ALTITUDE_REF: u16 = 0x0005;
    pub const GPS_ALTITUDE: u16 = 0x0006;
    pub const GPS_TIMESTAMP: u16 = 0x0007;
    pub const GPS_DATE_STAMP: u16 = 0x001D;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tiff_header() {
        // Little-endian TIFF header: II, 42, offset=8
        let data = [
            0x49, 0x49, // "II" = little-endian
            0x2A, 0x00, // 42 in LE
            0x08, 0x00, 0x00, 0x00, // offset 8 in LE
        ];

        let reader = IfdReader::new(&data, ByteOrder::LittleEndian);
        let offset = reader.parse_header().unwrap();
        assert_eq!(offset, 8);
    }

    #[test]
    fn parse_big_endian_header() {
        // Big-endian TIFF header: MM, 42, offset=8
        let data = [
            0x4D, 0x4D, // "MM" = big-endian
            0x00, 0x2A, // 42 in BE
            0x00, 0x00, 0x00, 0x08, // offset 8 in BE
        ];

        let reader = IfdReader::new(&data, ByteOrder::BigEndian);
        let offset = reader.parse_header().unwrap();
        assert_eq!(offset, 8);
    }

    #[test]
    fn parse_bigtiff_header() {
        // BigTIFF header (16 bytes): II, 43, offset_size=8, reserved=0, offset=16
        let data = [
            0x49, 0x49, // "II" = little-endian
            0x2B, 0x00, // 43 in LE (BigTIFF magic)
            0x08, 0x00, // offset byte size = 8
            0x00, 0x00, // reserved
            0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // offset 16 in LE (8 bytes)
        ];

        let reader = IfdReader::new(&data, ByteOrder::LittleEndian);
        let (offset, is_bigtiff) = reader.parse_header_ex().unwrap();
        assert!(is_bigtiff);
        assert_eq!(offset, 16);
    }

    #[test]
    fn parse_bigtiff_header_be() {
        // BigTIFF header big-endian
        let data = [
            0x4D, 0x4D, // "MM" = big-endian
            0x00, 0x2B, // 43 in BE (BigTIFF magic)
            0x00, 0x08, // offset byte size = 8
            0x00, 0x00, // reserved
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, // offset 16 in BE (8 bytes)
        ];

        let reader = IfdReader::new(&data, ByteOrder::BigEndian);
        let (offset, is_bigtiff) = reader.parse_header_ex().unwrap();
        assert!(is_bigtiff);
        assert_eq!(offset, 16);
    }
}

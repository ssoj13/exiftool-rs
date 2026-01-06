//! EXIF/TIFF writer - serializes metadata to TIFF bytes.
//!
//! TIFF structure:
//! - Header (8 bytes): byte order (II/MM) + magic (0x002A) + IFD0 offset
//! - IFD0: image tags (Make, Model, DateTime, etc.)
//! - ExifIFD: EXIF-specific tags (pointed by 0x8769)
//! - GPSIFD: GPS tags (pointed by 0x8825)
//! - Data area: strings, rationals, arrays that don't fit in 4 bytes

use crate::{ByteOrder, Error, ExifFormat, Result};
use std::io::Write;

/// EXIF tag IDs for IFD0
pub mod tags {
    // IFD0 tags
    pub const IMAGE_DESCRIPTION: u16 = 0x010E;
    pub const MAKE: u16 = 0x010F;
    pub const MODEL: u16 = 0x0110;
    pub const ORIENTATION: u16 = 0x0112;
    pub const X_RESOLUTION: u16 = 0x011A;
    pub const Y_RESOLUTION: u16 = 0x011B;
    pub const RESOLUTION_UNIT: u16 = 0x0128;
    pub const SOFTWARE: u16 = 0x0131;
    pub const DATE_TIME: u16 = 0x0132;
    pub const ARTIST: u16 = 0x013B;
    pub const COPYRIGHT: u16 = 0x8298;
    pub const EXIF_IFD: u16 = 0x8769;
    pub const GPS_IFD: u16 = 0x8825;
    
    // ExifIFD tags
    pub const EXPOSURE_TIME: u16 = 0x829A;
    pub const FNUMBER: u16 = 0x829D;
    pub const EXPOSURE_PROGRAM: u16 = 0x8822;
    pub const ISO: u16 = 0x8827;
    pub const EXIF_VERSION: u16 = 0x9000;
    pub const DATE_TIME_ORIGINAL: u16 = 0x9003;
    pub const CREATE_DATE: u16 = 0x9004;
    pub const SHUTTER_SPEED: u16 = 0x9201;
    pub const APERTURE: u16 = 0x9202;
    pub const EXPOSURE_COMPENSATION: u16 = 0x9204;
    pub const METERING_MODE: u16 = 0x9207;
    pub const FLASH: u16 = 0x9209;
    pub const FOCAL_LENGTH: u16 = 0x920A;
    pub const COLOR_SPACE: u16 = 0xA001;
    pub const EXIF_IMAGE_WIDTH: u16 = 0xA002;
    pub const EXIF_IMAGE_HEIGHT: u16 = 0xA003;
    
    // GPS tags
    pub const GPS_VERSION_ID: u16 = 0x0000;
    pub const GPS_LATITUDE_REF: u16 = 0x0001;
    pub const GPS_LATITUDE: u16 = 0x0002;
    pub const GPS_LONGITUDE_REF: u16 = 0x0003;
    pub const GPS_LONGITUDE: u16 = 0x0004;
    pub const GPS_ALTITUDE_REF: u16 = 0x0005;
    pub const GPS_ALTITUDE: u16 = 0x0006;
    
    // IFD1 (Thumbnail) tags
    pub const COMPRESSION: u16 = 0x0103;
    pub const JPEG_INTERCHANGE_FORMAT: u16 = 0x0201;
    pub const JPEG_INTERCHANGE_FORMAT_LENGTH: u16 = 0x0202;
}

/// IFD entry for writing.
#[derive(Debug, Clone)]
pub struct WriteEntry {
    pub tag: u16,
    pub format: ExifFormat,
    pub count: u32,
    pub data: Vec<u8>,
}

impl WriteEntry {
    /// Create entry from u16 value.
    pub fn from_u16(tag: u16, value: u16) -> Self {
        Self {
            tag,
            format: ExifFormat::UInt16,
            count: 1,
            data: value.to_le_bytes().to_vec(),
        }
    }
    
    /// Create entry from u32 value.
    pub fn from_u32(tag: u16, value: u32) -> Self {
        Self {
            tag,
            format: ExifFormat::UInt32,
            count: 1,
            data: value.to_le_bytes().to_vec(),
        }
    }
    
    /// Create entry from string (null-terminated ASCII).
    pub fn from_str(tag: u16, value: &str) -> Self {
        let mut data = value.as_bytes().to_vec();
        data.push(0); // null terminator
        Self {
            tag,
            format: ExifFormat::String,
            count: data.len() as u32,
            data,
        }
    }
    
    /// Create entry from URational.
    pub fn from_urational(tag: u16, num: u32, den: u32) -> Self {
        let mut data = Vec::with_capacity(8);
        data.extend_from_slice(&num.to_le_bytes());
        data.extend_from_slice(&den.to_le_bytes());
        Self {
            tag,
            format: ExifFormat::URational,
            count: 1,
            data,
        }
    }
    
    /// Create entry from SRational.
    pub fn from_srational(tag: u16, num: i32, den: i32) -> Self {
        let mut data = Vec::with_capacity(8);
        data.extend_from_slice(&num.to_le_bytes());
        data.extend_from_slice(&den.to_le_bytes());
        Self {
            tag,
            format: ExifFormat::SRational,
            count: 1,
            data,
        }
    }
    
    /// Create entry from undefined bytes.
    pub fn from_bytes(tag: u16, bytes: &[u8]) -> Self {
        Self {
            tag,
            format: ExifFormat::Undefined,
            count: bytes.len() as u32,
            data: bytes.to_vec(),
        }
    }
    
    /// Create entry from u32 array (for StripOffsets, etc).
    pub fn from_u32_array(tag: u16, values: &[u32]) -> Self {
        let mut data = Vec::with_capacity(values.len() * 4);
        for v in values {
            data.extend_from_slice(&v.to_le_bytes());
        }
        Self {
            tag,
            format: ExifFormat::UInt32,
            count: values.len() as u32,
            data,
        }
    }
    
    /// Data size in bytes.
    pub fn data_size(&self) -> usize {
        self.data.len()
    }
    
    /// Whether data fits inline (≤ 4 bytes).
    pub fn is_inline(&self) -> bool {
        self.data.len() <= 4
    }
}

/// TIFF/EXIF writer.
#[must_use]
pub struct ExifWriter {
    byte_order: ByteOrder,
    ifd0: Vec<WriteEntry>,
    exif_ifd: Vec<WriteEntry>,
    gps_ifd: Vec<WriteEntry>,
    ifd1: Vec<WriteEntry>,
    thumbnail: Option<Vec<u8>>,
}

impl ExifWriter {
    /// Create new writer with specified byte order.
    pub fn new(byte_order: ByteOrder) -> Self {
        Self {
            byte_order,
            ifd0: Vec::new(),
            exif_ifd: Vec::new(),
            gps_ifd: Vec::new(),
            ifd1: Vec::new(),
            thumbnail: None,
        }
    }
    
    /// Create writer with little-endian byte order (most common).
    pub fn new_le() -> Self {
        Self::new(ByteOrder::LittleEndian)
    }
    
    /// Add entry to IFD0.
    pub fn add_ifd0(&mut self, entry: WriteEntry) {
        self.ifd0.push(entry);
    }
    
    /// Add entry to ExifIFD.
    pub fn add_exif(&mut self, entry: WriteEntry) {
        self.exif_ifd.push(entry);
    }
    
    /// Add entry to GPSIFD.
    pub fn add_gps(&mut self, entry: WriteEntry) {
        self.gps_ifd.push(entry);
    }
    
    /// Add entry to IFD1 (thumbnail IFD).
    pub fn add_ifd1(&mut self, entry: WriteEntry) {
        self.ifd1.push(entry);
    }
    
    /// Set JPEG thumbnail data.
    /// This automatically sets up IFD1 with proper compression and offset tags.
    pub fn set_thumbnail(&mut self, jpeg_data: &[u8]) {
        if !jpeg_data.is_empty() {
            self.thumbnail = Some(jpeg_data.to_vec());
            // Compression = 6 (JPEG)
            self.ifd1.retain(|e| e.tag != tags::COMPRESSION);
            self.ifd1.push(WriteEntry::from_u16(tags::COMPRESSION, 6));
        }
    }
    
    /// Serialize to TIFF bytes.
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(4096);
        
        // Write header
        self.write_header(&mut buf)?;
        
        // Header = 8 bytes, IFD0 starts at offset 8
        let ifd0_offset = 8u32;
        
        // Build IFD0 with pointers to ExifIFD and GPSIFD
        let mut ifd0_entries = self.ifd0.clone();
        
        // Calculate where ExifIFD will be (after IFD0)
        let _ifd0_base_size = self.calc_ifd_size(&ifd0_entries)?;
        
        // Add ExifIFD pointer if needed
        let exif_ifd_offset = if !self.exif_ifd.is_empty() {
            ifd0_entries.retain(|e| e.tag != tags::EXIF_IFD);
            ifd0_entries.push(WriteEntry::from_u32(tags::EXIF_IFD, 0)); // placeholder
            ifd0_offset + self.calc_ifd_size(&ifd0_entries)?
        } else {
            0
        };
        
        // Add GPSIFD pointer if needed
        let mut exif_entries = self.exif_ifd.clone();
        exif_entries.sort_by_key(|e| e.tag);
        let exif_ifd_size = self.calc_ifd_size(&exif_entries)?;
        
        let _gps_ifd_offset = if !self.gps_ifd.is_empty() {
            ifd0_entries.retain(|e| e.tag != tags::GPS_IFD);
            ifd0_entries.push(WriteEntry::from_u32(tags::GPS_IFD, 0)); // placeholder
            if exif_ifd_offset > 0 {
                exif_ifd_offset + exif_ifd_size
            } else {
                ifd0_offset + self.calc_ifd_size(&ifd0_entries)?
            }
        } else {
            0
        };
        
        // Recalculate final IFD0 size with all pointers
        ifd0_entries.sort_by_key(|e| e.tag);
        let ifd0_size = self.calc_ifd_size(&ifd0_entries)?;
        
        // Recalculate ExifIFD offset
        let exif_ifd_offset = if !self.exif_ifd.is_empty() {
            ifd0_offset + ifd0_size
        } else {
            0
        };
        
        // Recalculate GPSIFD offset
        let mut gps_entries = self.gps_ifd.clone();
        gps_entries.sort_by_key(|e| e.tag);
        let gps_ifd_size = self.calc_ifd_size(&gps_entries)?;
        
        let gps_ifd_offset = if !self.gps_ifd.is_empty() {
            if exif_ifd_offset > 0 {
                exif_ifd_offset + exif_ifd_size
            } else {
                ifd0_offset + ifd0_size
            }
        } else {
            0
        };
        
        // Calculate IFD1 offset (for thumbnail)
        let has_ifd1 = self.thumbnail.is_some() || !self.ifd1.is_empty();
        let last_ifd_end = if gps_ifd_offset > 0 {
            gps_ifd_offset + gps_ifd_size
        } else if exif_ifd_offset > 0 {
            exif_ifd_offset + exif_ifd_size
        } else {
            ifd0_offset + ifd0_size
        };
        
        let ifd1_offset = if has_ifd1 { last_ifd_end } else { 0 };
        
        // Build IFD1 entries with thumbnail offset/length
        let mut ifd1_entries = self.ifd1.clone();
        if let Some(ref thumb) = self.thumbnail {
            // JPEGInterchangeFormat (offset) - placeholder, will be calculated
            ifd1_entries.retain(|e| e.tag != tags::JPEG_INTERCHANGE_FORMAT && e.tag != tags::JPEG_INTERCHANGE_FORMAT_LENGTH);
            ifd1_entries.push(WriteEntry::from_u32(tags::JPEG_INTERCHANGE_FORMAT, 0)); // placeholder
            ifd1_entries.push(WriteEntry::from_u32(tags::JPEG_INTERCHANGE_FORMAT_LENGTH, thumb.len() as u32));
        }
        ifd1_entries.sort_by_key(|e| e.tag);
        let ifd1_size = self.calc_ifd_size(&ifd1_entries)?;
        
        // Thumbnail data offset (right after IFD1)
        let thumb_offset = if has_ifd1 { ifd1_offset + ifd1_size } else { 0 };
        
        // Update JPEGInterchangeFormat with actual offset
        if let Some(entry) = ifd1_entries.iter_mut().find(|e| e.tag == tags::JPEG_INTERCHANGE_FORMAT) {
            entry.data = match self.byte_order {
                ByteOrder::LittleEndian => thumb_offset.to_le_bytes().to_vec(),
                ByteOrder::BigEndian => thumb_offset.to_be_bytes().to_vec(),
            };
        }
        
        // Update ExifIFD pointer in IFD0
        if let Some(entry) = ifd0_entries.iter_mut().find(|e| e.tag == tags::EXIF_IFD) {
            entry.data = match self.byte_order {
                ByteOrder::LittleEndian => exif_ifd_offset.to_le_bytes().to_vec(),
                ByteOrder::BigEndian => exif_ifd_offset.to_be_bytes().to_vec(),
            };
        }
        
        // Update GPSIFD pointer in IFD0
        if let Some(entry) = ifd0_entries.iter_mut().find(|e| e.tag == tags::GPS_IFD) {
            entry.data = match self.byte_order {
                ByteOrder::LittleEndian => gps_ifd_offset.to_le_bytes().to_vec(),
                ByteOrder::BigEndian => gps_ifd_offset.to_be_bytes().to_vec(),
            };
        }
        
        // Write IFD0 (with next_ifd pointing to IFD1 if present)
        self.write_ifd_with_next(&mut buf, &ifd0_entries, ifd0_offset, ifd1_offset)?;
        
        // Write ExifIFD
        if !exif_entries.is_empty() {
            self.write_ifd(&mut buf, &exif_entries, exif_ifd_offset)?;
        }
        
        // Write GPSIFD
        if !gps_entries.is_empty() {
            self.write_ifd(&mut buf, &gps_entries, gps_ifd_offset)?;
        }
        
        // Write IFD1
        if has_ifd1 {
            self.write_ifd(&mut buf, &ifd1_entries, ifd1_offset)?;
        }
        
        // Write thumbnail data
        if let Some(ref thumb) = self.thumbnail {
            buf.write_all(thumb)?;
        }
        
        Ok(buf)
    }
    
    /// Write TIFF header.
    fn write_header(&self, buf: &mut Vec<u8>) -> Result<()> {
        // Byte order marker
        match self.byte_order {
            ByteOrder::LittleEndian => buf.write_all(b"II")?,
            ByteOrder::BigEndian => buf.write_all(b"MM")?,
        }
        
        // Magic number (42)
        self.write_u16(buf, 0x002A)?;
        
        // IFD0 offset (always 8 for our format)
        self.write_u32(buf, 8)?;
        
        Ok(())
    }
    
    /// Calculate IFD size including data area.
    fn calc_ifd_size(&self, entries: &[WriteEntry]) -> Result<u32> {
        // 2 bytes count + 12 bytes per entry + 4 bytes next IFD offset
        let ifd_struct_size = 2 + (entries.len() * 12) + 4;
        
        // Data area for values > 4 bytes
        let data_size: usize = entries
            .iter()
            .filter(|e| !e.is_inline())
            .map(|e| e.data_size())
            .sum();
        
        let total = ifd_struct_size + data_size;
        u32::try_from(total).map_err(|_| Error::IfdTooLarge(total))
    }
    
    /// Write IFD to buffer (next IFD = 0).
    fn write_ifd(&self, buf: &mut Vec<u8>, entries: &[WriteEntry], ifd_offset: u32) -> Result<()> {
        self.write_ifd_with_next(buf, entries, ifd_offset, 0)
    }
    
    /// Write IFD to buffer with specified next IFD offset.
    fn write_ifd_with_next(&self, buf: &mut Vec<u8>, entries: &[WriteEntry], ifd_offset: u32, next_ifd: u32) -> Result<()> {
        // Entry count
        self.write_u16(buf, entries.len() as u16)?;
        
        // Calculate data area offset (after all entries + next IFD pointer)
        let data_area_offset = ifd_offset + 2 + (entries.len() as u32 * 12) + 4;
        let mut current_data_offset = data_area_offset;
        
        // Collect data for data area
        let mut data_area = Vec::new();
        
        // Write entries
        for entry in entries {
            self.write_u16(buf, entry.tag)?;
            self.write_u16(buf, entry.format as u16)?;
            self.write_u32(buf, entry.count)?;
            
            if entry.is_inline() {
                // Value fits in 4 bytes - write inline (pad with zeros)
                let mut inline = [0u8; 4];
                inline[..entry.data.len()].copy_from_slice(&entry.data);
                buf.write_all(&inline)?;
            } else {
                // Write offset to data area
                self.write_u32(buf, current_data_offset)?;
                data_area.extend_from_slice(&entry.data);
                current_data_offset += entry.data.len() as u32;
            }
        }
        
        // Next IFD offset
        self.write_u32(buf, next_ifd)?;
        
        // Write data area
        buf.write_all(&data_area)?;
        
        Ok(())
    }
    
    /// Write u16 with correct byte order.
    fn write_u16(&self, buf: &mut Vec<u8>, value: u16) -> Result<()> {
        let bytes = match self.byte_order {
            ByteOrder::LittleEndian => value.to_le_bytes(),
            ByteOrder::BigEndian => value.to_be_bytes(),
        };
        buf.write_all(&bytes)?;
        Ok(())
    }
    
    /// Write u32 with correct byte order.
    fn write_u32(&self, buf: &mut Vec<u8>, value: u32) -> Result<()> {
        let bytes = match self.byte_order {
            ByteOrder::LittleEndian => value.to_le_bytes(),
            ByteOrder::BigEndian => value.to_be_bytes(),
        };
        buf.write_all(&bytes)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IfdReader, RawValue};
    
    #[test]
    fn serialize_simple_exif() {
        let mut writer = ExifWriter::new_le();
        writer.add_ifd0(WriteEntry::from_str(tags::MAKE, "TestCam"));
        writer.add_ifd0(WriteEntry::from_str(tags::MODEL, "Model X"));
        writer.add_ifd0(WriteEntry::from_u16(tags::ORIENTATION, 1));
        
        let bytes = writer.serialize().unwrap();
        
        // Verify header
        assert_eq!(&bytes[0..2], b"II"); // Little-endian
        assert_eq!(bytes[2], 0x2A); // Magic low byte
        assert_eq!(bytes[3], 0x00); // Magic high byte
        
        // Parse back to verify
        let reader = IfdReader::new(&bytes, ByteOrder::LittleEndian, 0);
        let offset = reader.parse_header().unwrap();
        let (entries, _) = reader.read_ifd(offset).unwrap();
        
        assert_eq!(entries.len(), 3);
    }
    
    #[test]
    fn round_trip_exif() {
        let mut writer = ExifWriter::new_le();
        writer.add_ifd0(WriteEntry::from_str(tags::MAKE, "Canon"));
        writer.add_ifd0(WriteEntry::from_str(tags::MODEL, "EOS R5"));
        writer.add_ifd0(WriteEntry::from_u16(tags::ORIENTATION, 6));
        writer.add_ifd0(WriteEntry::from_urational(tags::X_RESOLUTION, 300, 1));
        
        writer.add_exif(WriteEntry::from_u16(tags::ISO, 400));
        writer.add_exif(WriteEntry::from_urational(tags::EXPOSURE_TIME, 1, 250));
        
        let bytes = writer.serialize().unwrap();
        
        // Parse and verify
        let reader = IfdReader::new(&bytes, ByteOrder::LittleEndian, 0);
        let offset = reader.parse_header().unwrap();
        let (entries, _) = reader.read_ifd(offset).unwrap();
        
        // Should have IFD0 entries + ExifIFD pointer
        assert!(entries.len() >= 4);
    }
    
    #[test]
    fn write_thumbnail() {
        let mut writer = ExifWriter::new_le();
        writer.add_ifd0(WriteEntry::from_str(tags::MAKE, "TestCam"));
        
        // Minimal valid JPEG (SOI + EOI)
        let thumb_data = vec![0xFF, 0xD8, 0xFF, 0xD9];
        writer.set_thumbnail(&thumb_data);
        
        let bytes = writer.serialize().unwrap();
        
        // Parse IFD0
        let reader = IfdReader::new(&bytes, ByteOrder::LittleEndian, 0);
        let offset = reader.parse_header().unwrap();
        let (ifd0_entries, next_ifd) = reader.read_ifd(offset).unwrap();
        
        // IFD0 should have Make entry
        assert!(!ifd0_entries.is_empty());
        
        // Should point to IFD1
        assert!(next_ifd > 0, "IFD0 should point to IFD1");
        
        // Parse IFD1
        let (ifd1_entries, _) = reader.read_ifd(next_ifd).unwrap();
        
        // IFD1 should have Compression, JPEGInterchangeFormat, JPEGInterchangeFormatLength
        assert!(ifd1_entries.len() >= 3, "IFD1 should have at least 3 entries");
        
        // Find thumbnail offset and verify data
        let thumb_offset_entry = ifd1_entries.iter()
            .find(|e| e.tag == tags::JPEG_INTERCHANGE_FORMAT)
            .expect("Should have JPEGInterchangeFormat");
        let thumb_offset = match &thumb_offset_entry.value {
            RawValue::UInt32(v) if !v.is_empty() => v[0] as usize,
            _ => panic!("Expected UInt32 for JPEGInterchangeFormat"),
        };
        
        let thumb_len_entry = ifd1_entries.iter()
            .find(|e| e.tag == tags::JPEG_INTERCHANGE_FORMAT_LENGTH)
            .expect("Should have JPEGInterchangeFormatLength");
        let thumb_len = match &thumb_len_entry.value {
            RawValue::UInt32(v) if !v.is_empty() => v[0] as usize,
            _ => panic!("Expected UInt32 for JPEGInterchangeFormatLength"),
        };
        
        // Verify thumbnail data
        assert_eq!(thumb_len, 4);
        assert_eq!(&bytes[thumb_offset..thumb_offset + thumb_len], &thumb_data);
    }
}

//! Leica MakerNotes parser.
//!
//! Leica MakerNotes structure varies by camera:
//! - Type 1: "LEICA\0\0\0" header (8 bytes), Leica D-LUX/V-LUX
//! - Type 2: Standard IFD without header (Leica M, SL, Q, S series)
//! - Type 3: "LEICA CAMERA AG\0" header (Leica M9, M-E)
//! - Type 4: Panasonic format (some Leica cameras based on Panasonic)
//!
//! Known tags:
//! - 0x0001: CameraSerialNumber (S series)
//! - 0x0003: LensType
//! - 0x0004: LensSerialNumber
//! - 0x0005: InternalSerialNumber
//! - 0x0007: Lens
//! - 0x0008: FocusDistance
//! - 0x0009: FocusMode
//! - 0x000A: ApproximateFNumber
//! - 0x000B: ExposureMode
//! - 0x000C: ShotInfo
//! - 0x000D: WhiteBalance
//! - 0x000F: MeteringMode
//! - 0x0010: ISO
//! - 0x0012: ExternalSensorBrightnessValue
//! - 0x0013: MeasuredLV
//! - 0x0014: FilmSpeed
//! - 0x0016: CCD
//! - 0x0018: ModelType (0=Digital)
//! - 0x001D: CameraTemperature
//! - 0x001E: ColorTemperature
//! - 0x0024: WB_RGBLevels
//! - 0x0025: UserProfile
//! - 0x0026: SerialNumber
//! - 0x002E: Brightness
//! - 0x0300: PreviewImage (offset+length)
//! - 0x0301: SerialNumber (M9 etc)
//! - 0x0302: CameraCode
//! - 0x0303: LensCode
//! - 0x0304: ExposureLock
//! - 0x0305: FocusLock
//! - 0x0310: SensorScale (S series)
//! - 0x0311: CameraScale (S series)
//! - 0x0312: ImageResolution (S series)
//! - 0x030F: LensInfo (S series)
//! - 0x0400: ImageNumber
//! - 0x0401: ShutterType
//! - 0x0410: UserData

use super::{parse_ifd_entries, Vendor, VendorParser};
use exiftool_attrs::{Attrs, AttrValue};
use exiftool_core::ByteOrder;

/// Leica MakerNotes parser.
pub struct LeicaParser;

impl LeicaParser {
    /// Tag definitions.
    const TAGS: &'static [(u16, &'static str)] = &[
        (0x0001, "SerialNumber"),
        (0x0003, "LensType"),
        (0x0004, "LensSerialNumber"),
        (0x0005, "InternalSerialNumber"),
        (0x0007, "Lens"),
        (0x0008, "FocusDistance"),
        (0x0009, "FocusMode"),
        (0x000A, "ApproximateFNumber"),
        (0x000B, "ExposureMode"),
        (0x000C, "ShotInfo"),
        (0x000D, "WhiteBalance"),
        (0x000F, "MeteringMode"),
        (0x0010, "ISO"),
        (0x0012, "ExternalSensorBrightnessValue"),
        (0x0013, "MeasuredLV"),
        (0x0014, "FilmSpeed"),
        (0x0016, "CCD"),
        (0x0018, "ModelType"),
        (0x001D, "CameraTemperature"),
        (0x001E, "ColorTemperature"),
        (0x0024, "WB_RGBLevels"),
        (0x0025, "UserProfile"),
        (0x0026, "SerialNumber2"),
        (0x002E, "Brightness"),
        (0x0300, "PreviewImageStart"),
        (0x0301, "SerialNumber3"),
        (0x0302, "CameraCode"),
        (0x0303, "LensCode"),
        (0x0304, "ExposureLock"),
        (0x0305, "FocusLock"),
        (0x030F, "LensInfo"),
        (0x0310, "SensorScale"),
        (0x0311, "CameraScale"),
        (0x0312, "ImageResolution"),
        (0x0400, "ImageNumber"),
        (0x0401, "ShutterType"),
        (0x0410, "UserData"),
    ];
    
    /// Find tag name by ID.
    fn tag_name(tag: u16) -> Option<&'static str> {
        Self::TAGS.iter().find(|(t, _)| *t == tag).map(|(_, n)| *n)
    }
    
    /// Detect header type and return (ifd_offset, byte_order).
    fn detect_format(data: &[u8], parent_order: ByteOrder) -> Option<(u32, ByteOrder)> {
        if data.len() < 8 {
            return None;
        }
        
        // Type 1: "LEICA\0\0\0" header (8 bytes)
        if data.starts_with(b"LEICA\0\0\0") {
            // Check byte order after header
            let order = if data.len() >= 10 {
                match &data[8..10] {
                    b"II" => ByteOrder::LittleEndian,
                    b"MM" => ByteOrder::BigEndian,
                    _ => ByteOrder::LittleEndian,
                }
            } else {
                ByteOrder::LittleEndian
            };
            return Some((8, order));
        }
        
        // Type 3: "LEICA CAMERA AG\0" header (M9, M-E etc)
        if data.starts_with(b"LEICA CAMERA AG") {
            // IFD starts after header
            return Some((16, ByteOrder::LittleEndian));
        }
        
        // Type 4: "LEICA" at offset 0 with different structure
        if data.starts_with(b"LEICA") {
            // Find end of LEICA header
            let header_end = data.iter().position(|&b| b == 0).unwrap_or(5) + 1;
            // Align to even
            let ifd_start = if header_end % 2 == 0 { header_end } else { header_end + 1 };
            return Some((ifd_start as u32, parent_order));
        }
        
        // Type 2: No header, standard IFD (M10, SL, Q series)
        // Check for valid IFD structure
        let count = if parent_order == ByteOrder::LittleEndian {
            u16::from_le_bytes([data[0], data[1]])
        } else {
            u16::from_be_bytes([data[0], data[1]])
        };
        
        // Reasonable entry count (1-100)
        if count > 0 && count < 100 {
            let expected_size = 2 + count as usize * 12 + 4;
            if data.len() >= expected_size {
                return Some((0, parent_order));
            }
        }
        
        None
    }
}

impl VendorParser for LeicaParser {
    fn vendor(&self) -> Vendor {
        Vendor::Leica
    }
    
    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        let (ifd_offset, byte_order) = Self::detect_format(data, parent_byte_order)?;
        
        let entries = parse_ifd_entries(data, byte_order, ifd_offset)?;
        
        let mut attrs = Attrs::new();
        
        for entry in entries {
            let name = Self::tag_name(entry.tag)
                .unwrap_or_else(|| {
                    // Store tag number as string if unknown
                    Box::leak(format!("Tag{:04X}", entry.tag).into_boxed_str())
                });
            
            // Convert entry value to AttrValue
            let value = match &entry.value {
                exiftool_core::RawValue::String(s) => AttrValue::Str(s.clone()),
                exiftool_core::RawValue::UInt8(v) if v.len() == 1 => AttrValue::UInt(v[0] as u32),
                exiftool_core::RawValue::UInt16(v) if v.len() == 1 => AttrValue::UInt(v[0] as u32),
                exiftool_core::RawValue::UInt32(v) if v.len() == 1 => AttrValue::UInt(v[0]),
                exiftool_core::RawValue::Int8(v) if v.len() == 1 => AttrValue::Int(v[0] as i32),
                exiftool_core::RawValue::Int16(v) if v.len() == 1 => AttrValue::Int(v[0] as i32),
                exiftool_core::RawValue::Int32(v) if v.len() == 1 => AttrValue::Int(v[0]),
                exiftool_core::RawValue::URational(v) if v.len() == 1 => {
                    AttrValue::URational(v[0].num, v[0].den)
                }
                exiftool_core::RawValue::SRational(v) if v.len() == 1 => {
                    AttrValue::Rational(v[0].num, v[0].den)
                }
                exiftool_core::RawValue::Float(v) if v.len() == 1 => AttrValue::Float(v[0]),
                exiftool_core::RawValue::Double(v) if v.len() == 1 => AttrValue::Double(v[0]),
                exiftool_core::RawValue::Undefined(v) => AttrValue::Bytes(v.clone()),
                other => AttrValue::Str(other.to_string()),
            };
            
            attrs.set(&format!("Leica:{}", name), value);
        }
        
        Some(attrs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vendor() {
        assert_eq!(LeicaParser.vendor(), Vendor::Leica);
    }
    
    #[test]
    fn test_detect_format_with_header() {
        let data = b"LEICA\0\0\0II\x2a\0\x01\0\0\0\0\0\0\0\0\0\0\0";
        let result = LeicaParser::detect_format(data, ByteOrder::BigEndian);
        assert!(result.is_some());
        let (offset, order) = result.unwrap();
        assert_eq!(offset, 8);
        assert_eq!(order, ByteOrder::LittleEndian);
    }
    
    #[test]
    fn test_detect_format_camera_ag() {
        let data = b"LEICA CAMERA AG\0\x01\0\0\0\0\0\0\0\0\0";
        let result = LeicaParser::detect_format(data, ByteOrder::BigEndian);
        assert!(result.is_some());
        let (offset, _) = result.unwrap();
        assert_eq!(offset, 16);
    }
}

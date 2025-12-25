//! Canon MakerNotes parser.
//!
//! Canon MakerNotes structure:
//! - No header, starts directly with IFD
//! - Uses parent byte order
//! - Contains many sub-IFDs encoded as binary blobs
//!
//! Known sub-IFD tags:
//! - 0x0001: CameraSettings (binary array)
//! - 0x0002: FocalLength
//! - 0x0004: ShotInfo (binary array)
//! - 0x0005: Panorama
//! - 0x0006: ImageType
//! - 0x0007: FirmwareVersion
//! - 0x0009: OwnerName
//! - 0x000C: SerialNumber
//! - 0x000D: CameraInfo
//! - 0x000E: FileLength
//! - 0x000F: CustomFunctions
//! - 0x0010: ModelID
//! - 0x0012: AFInfo
//! - 0x0013: ThumbnailImageValidArea
//! - 0x0015: SerialNumberFormat
//! - 0x001A: SuperMacro
//! - 0x001C: DateStampMode
//! - 0x001D: MyColors
//! - 0x001E: FirmwareRevision
//! - 0x0024: FaceDetect1
//! - 0x0025: FaceDetect2
//! - 0x0026: AFInfo2
//! - 0x0027: ContrastInfo
//! - 0x0028: ImageUniqueID
//! - 0x002F: FaceDetect3
//! - 0x0035: TimeInfo
//! - 0x0093: FileInfo
//! - 0x0095: LensModel
//! - 0x0096: InternalSerialNumber
//! - 0x0097: DustRemovalData
//! - 0x0099: CustomFunctions2
//! - 0x009A: AspectInfo
//! - 0x00A0: ProcessingInfo
//! - 0x00AA: MeasuredColor
//! - 0x00B4: ColorSpace
//! - 0x00D0: VRDOffset
//! - 0x00E0: SensorInfo
//! - 0x4001: ColorData
//! - 0x4002: CRWParam
//! - 0x4003: ColorInfo
//! - 0x4005: Flavor
//! - 0x4008: PictureStyleUserDef
//! - 0x4009: PictureStylePC
//! - 0x4010: CustomPictureStyleFileName
//! - 0x4013: AFMicroAdj
//! - 0x4015: VignettingCorr
//! - 0x4016: VignettingCorr2
//! - 0x4018: LightingOpt
//! - 0x4019: LensInfo
//! - 0x4020: AmbienceInfo
//! - 0x4021: MultiExp
//! - 0x4024: FilterInfo
//! - 0x4025: HDRInfo
//! - 0x4028: AFConfig

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::{ByteOrder, IfdReader};
use exiftool_tags::generated::canon;

/// Canon MakerNotes parser.
pub struct CanonParser;

impl VendorParser for CanonParser {
    fn vendor(&self) -> Vendor {
        Vendor::Canon
    }

    fn parse(&self, data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 2 {
            return None;
        }

        let reader = IfdReader::new(data, byte_order, 0);
        let (entries, _) = reader.read_ifd(0).ok()?;

        let mut attrs = Attrs::new();

        for entry in entries {
            // Check for known sub-IFD tags
            match entry.tag {
                0x0001 => {
                    // CameraSettings - binary array with specific format
                    if let Some(sub_attrs) = parse_camera_settings(&entry.value.as_bytes()?, byte_order) {
                        attrs.set("CameraSettings", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0002 => {
                    // FocalLength
                    if let Some(sub_attrs) = parse_focal_length(&entry.value.as_bytes()?, byte_order) {
                        attrs.set("FocalLength", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0004 => {
                    // ShotInfo - binary array
                    if let Some(sub_attrs) = parse_shot_info(&entry.value.as_bytes()?, byte_order) {
                        attrs.set("ShotInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0012 => {
                    // AFInfo
                    if let Some(sub_attrs) = parse_af_info(&entry.value.as_bytes()?, byte_order) {
                        attrs.set("AFInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0026 => {
                    // AFInfo2
                    if let Some(sub_attrs) = parse_af_info2(&entry.value.as_bytes()?, byte_order) {
                        attrs.set("AFInfo2", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0093 => {
                    // FileInfo
                    if let Some(sub_attrs) = parse_file_info(&entry.value.as_bytes()?, byte_order) {
                        attrs.set("FileInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x00A0 => {
                    // ProcessingInfo
                    if let Some(sub_attrs) = parse_processing_info(&entry.value.as_bytes()?, byte_order) {
                        attrs.set("ProcessingInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                _ => {
                    // Main tag - lookup in CANON_MAIN
                    if let Some(tag_def) = canon::CANON_MAIN.get(&entry.tag) {
                        let value = format_value(&entry, tag_def.values);
                        attrs.set(tag_def.name, value);
                    }
                }
            }
        }

        Some(attrs)
    }
}

/// Parse Canon CameraSettings sub-IFD (tag 0x0001).
/// Binary array of 16-bit signed values.
fn parse_camera_settings(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 2;

    for i in 0..count {
        let offset = i * 2;
        if offset + 2 > data.len() {
            break;
        }

        let value = match byte_order {
            ByteOrder::LittleEndian => i16::from_le_bytes([data[offset], data[offset + 1]]),
            ByteOrder::BigEndian => i16::from_be_bytes([data[offset], data[offset + 1]]),
        };

        // Map index to tag using CANON_CAMERASETTINGS table
        if let Some(tag_def) = canon::CANON_CAMERASETTINGS.get(&(i as u16)) {
            let attr_value = if let Some(values) = tag_def.values {
                // Try to find matching value description
                values
                    .iter()
                    .find(|(k, _)| *k == value as i64)
                    .map(|(_, v)| AttrValue::Str(v.to_string()))
                    .unwrap_or(AttrValue::Int(value as i32))
            } else {
                AttrValue::Int(value as i32)
            };
            attrs.set(tag_def.name, attr_value);
        }
    }

    Some(attrs)
}

/// Parse Canon FocalLength sub-IFD (tag 0x0002).
fn parse_focal_length(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 8 {
        return None;
    }

    let mut attrs = Attrs::new();

    // FocalType at index 0
    let focal_type = read_u16(data, 0, byte_order);
    attrs.set("FocalType", AttrValue::UInt(focal_type as u32));

    // FocalLength at index 1 (in units of focal length / 32)
    let focal_length = read_u16(data, 2, byte_order);
    attrs.set("FocalLength", AttrValue::Str(format!("{} mm", focal_length as f32 / 32.0)));

    // FocalPlaneXSize at index 2
    if data.len() >= 6 {
        let fp_x = read_u16(data, 4, byte_order);
        attrs.set("FocalPlaneXSize", AttrValue::UInt(fp_x as u32));
    }

    // FocalPlaneYSize at index 3
    if data.len() >= 8 {
        let fp_y = read_u16(data, 6, byte_order);
        attrs.set("FocalPlaneYSize", AttrValue::UInt(fp_y as u32));
    }

    Some(attrs)
}

/// Parse Canon ShotInfo sub-IFD (tag 0x0004).
fn parse_shot_info(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 2;

    for i in 0..count {
        let offset = i * 2;
        if offset + 2 > data.len() {
            break;
        }

        let value = match byte_order {
            ByteOrder::LittleEndian => i16::from_le_bytes([data[offset], data[offset + 1]]),
            ByteOrder::BigEndian => i16::from_be_bytes([data[offset], data[offset + 1]]),
        };

        if let Some(tag_def) = canon::CANON_SHOTINFO.get(&(i as u16)) {
            let attr_value = if let Some(values) = tag_def.values {
                values
                    .iter()
                    .find(|(k, _)| *k == value as i64)
                    .map(|(_, v)| AttrValue::Str(v.to_string()))
                    .unwrap_or(AttrValue::Int(value as i32))
            } else {
                AttrValue::Int(value as i32)
            };
            attrs.set(tag_def.name, attr_value);
        }
    }

    Some(attrs)
}

/// Parse Canon AFInfo sub-IFD (tag 0x0012).
fn parse_af_info(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();

    // Number of AF points
    let num_af_points = read_u16(data, 0, byte_order);
    attrs.set("NumAFPoints", AttrValue::UInt(num_af_points as u32));

    // Valid AF points
    if data.len() >= 4 {
        let valid_af_points = read_u16(data, 2, byte_order);
        attrs.set("ValidAFPoints", AttrValue::UInt(valid_af_points as u32));
    }

    // Parse using CANON_AFINFO table for additional fields
    let count = data.len() / 2;
    for i in 0..count.min(20) {
        if let Some(tag_def) = canon::CANON_AFINFO.get(&(i as u16)) {
            let value = read_u16(data, i * 2, byte_order);
            let attr_value = if let Some(values) = tag_def.values {
                values
                    .iter()
                    .find(|(k, _)| *k == value as i64)
                    .map(|(_, v)| AttrValue::Str(v.to_string()))
                    .unwrap_or(AttrValue::UInt(value as u32))
            } else {
                AttrValue::UInt(value as u32)
            };
            attrs.set(tag_def.name, attr_value);
        }
    }

    Some(attrs)
}

/// Parse Canon AFInfo2 sub-IFD (tag 0x0026).
fn parse_af_info2(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();

    // AFInfoSize
    let af_info_size = read_u16(data, 0, byte_order);
    attrs.set("AFInfoSize", AttrValue::UInt(af_info_size as u32));

    // AFAreaMode
    if data.len() >= 4 {
        let af_area_mode = read_u16(data, 2, byte_order);
        let mode_str = match af_area_mode {
            0 => "Off (Manual Focus)",
            1 => "AF Point Expansion (surround)",
            2 => "Single-point AF",
            4 => "Auto",
            5 => "Face Detect AF",
            6 => "Face + Tracking",
            7 => "Zone AF",
            8 => "AF Point Expansion (4 point)",
            9 => "Spot AF",
            10 => "AF Point Expansion (8 point)",
            11 => "Flexizone Multi (49 point)",
            12 => "Flexizone Multi (9 point)",
            13 => "Flexizone Single",
            14 => "Large Zone AF",
            _ => "Unknown",
        };
        attrs.set("AFAreaMode", AttrValue::Str(mode_str.to_string()));
    }

    // NumAFPoints
    if data.len() >= 6 {
        let num_af = read_u16(data, 4, byte_order);
        attrs.set("NumAFPoints", AttrValue::UInt(num_af as u32));
    }

    Some(attrs)
}

/// Parse Canon FileInfo sub-IFD (tag 0x0093).
fn parse_file_info(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 2;

    for i in 0..count.min(30) {
        if let Some(tag_def) = canon::CANON_FILEINFO.get(&(i as u16)) {
            let offset = i * 2;
            if offset + 2 <= data.len() {
                let value = match byte_order {
                    ByteOrder::LittleEndian => i16::from_le_bytes([data[offset], data[offset + 1]]),
                    ByteOrder::BigEndian => i16::from_be_bytes([data[offset], data[offset + 1]]),
                };

                let attr_value = if let Some(values) = tag_def.values {
                    values
                        .iter()
                        .find(|(k, _)| *k == value as i64)
                        .map(|(_, v)| AttrValue::Str(v.to_string()))
                        .unwrap_or(AttrValue::Int(value as i32))
                } else {
                    AttrValue::Int(value as i32)
                };
                attrs.set(tag_def.name, attr_value);
            }
        }
    }

    Some(attrs)
}

/// Parse Canon ProcessingInfo sub-IFD (tag 0x00A0).
fn parse_processing_info(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 2;

    for i in 0..count.min(20) {
        if let Some(tag_def) = canon::CANON_PROCESSING.get(&(i as u16)) {
            let offset = i * 2;
            if offset + 2 <= data.len() {
                let value = match byte_order {
                    ByteOrder::LittleEndian => i16::from_le_bytes([data[offset], data[offset + 1]]),
                    ByteOrder::BigEndian => i16::from_be_bytes([data[offset], data[offset + 1]]),
                };

                let attr_value = if let Some(values) = tag_def.values {
                    values
                        .iter()
                        .find(|(k, _)| *k == value as i64)
                        .map(|(_, v)| AttrValue::Str(v.to_string()))
                        .unwrap_or(AttrValue::Int(value as i32))
                } else {
                    AttrValue::Int(value as i32)
                };
                attrs.set(tag_def.name, attr_value);
            }
        }
    }

    Some(attrs)
}

/// Format IFD entry value with PrintConv lookup.
fn format_value(entry: &exiftool_core::IfdEntry, values_map: Option<&'static [(i64, &'static str)]>) -> AttrValue {
    if let Some(map) = values_map {
        if let Some(int_val) = entry.value.as_u32().map(|v| v as i64) {
            for &(key, label) in map {
                if key == int_val {
                    return AttrValue::Str(label.to_string());
                }
            }
        }
    }
    entry_to_attr(entry)
}

/// Read u16 from byte slice with byte order.
#[inline]
fn read_u16(data: &[u8], offset: usize, byte_order: ByteOrder) -> u16 {
    if offset + 2 > data.len() {
        return 0;
    }
    match byte_order {
        ByteOrder::LittleEndian => u16::from_le_bytes([data[offset], data[offset + 1]]),
        ByteOrder::BigEndian => u16::from_be_bytes([data[offset], data[offset + 1]]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_vendor() {
        assert_eq!(CanonParser.vendor(), Vendor::Canon);
    }
}

//! Sony MakerNotes parser.
//!
//! Sony MakerNotes structure:
//! - Type 1: "SONY DSC \0\0\0" header (12 bytes)
//! - Type 2: "SONY CAM \0\0\0" header (12 bytes)
//! - Type 3: No header, starts with IFD
//!
//! Sony uses multiple sub-IFD structures:
//! - 0x0010: CameraSettings
//! - 0x0020: FocusInfo
//! - 0x0102: Quality
//! - 0x0104: Teleconverter
//! - 0x0105: FlashExposureComp
//! - 0x0114: CameraSettings2
//! - 0x0116: ExposureTime
//! - 0x0118: FNumber
//! - 0x011A: ISO
//! - 0x0156: ExposureMode
//! - 0x0164: AFMode
//! - 0x2001: PreviewImage
//! - 0x2010: Tag2010 (camera-specific sub-IFD)
//! - 0x9050: Tag9050 (encrypted)
//! - 0x9400: Tag9400 (HDRInfo for newer cameras)
//! - 0x9402: Tag9402
//! - 0x9403: Tag9403
//! - 0x9404: Tag9404
//! - 0x9405: Tag9405 (DistortionCorrection)
//! - 0x9406: Tag9406 (PeripheralIllumCorr)

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::{ByteOrder, IfdReader};
use exiftool_tags::generated::sony;

/// Sony MakerNotes parser.
pub struct SonyParser;

/// Header magic for Sony MakerNotes.
const SONY_DSC_HEADER: &[u8] = b"SONY DSC ";
const SONY_CAM_HEADER: &[u8] = b"SONY CAM ";

impl VendorParser for SonyParser {
    fn vendor(&self) -> Vendor {
        Vendor::Sony
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 8 {
            return None;
        }

        // Detect header type and get IFD data
        let (ifd_data, byte_order) = if data.starts_with(SONY_DSC_HEADER) || data.starts_with(SONY_CAM_HEADER) {
            // Skip 12-byte header
            if data.len() < 14 {
                return None;
            }
            (&data[12..], parent_byte_order)
        } else {
            // No header, starts with IFD
            (data, parent_byte_order)
        };

        let reader = IfdReader::new(ifd_data, byte_order, 0);
        let (entries, _) = reader.read_ifd(0).ok()?;

        let mut attrs = Attrs::new();

        for entry in entries {
            match entry.tag {
                0x0010 => {
                    // CameraSettings
                    if let Some(sub_attrs) = parse_camera_settings(&entry.value.as_bytes()?, byte_order) {
                        attrs.set("CameraSettings", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0020 => {
                    // FocusInfo
                    if let Some(sub_attrs) = parse_focus_info(&entry.value.as_bytes()?, byte_order) {
                        attrs.set("FocusInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x2001 => {
                    // PreviewImage - direct preview data or offset
                    // For ARW files, this contains the preview image data directly
                    if let Some(bytes) = entry.value.as_bytes() {
                        // Check if it's a JPEG (starts with FFD8)
                        if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xD8 {
                            // Direct JPEG data - store length for extraction
                            attrs.set("PreviewImageData", AttrValue::UInt(bytes.len() as u32));
                        }
                    }
                    // Also store offset if available (value_offset)
                    if let Some(offset) = entry.value_offset {
                        attrs.set("PreviewImageStart", AttrValue::UInt(offset as u32));
                        attrs.set("PreviewImageLength", AttrValue::UInt(entry.count as u32));
                    }
                }
                0x9405 => {
                    // Tag9405 - DistortionCorrection info
                    if let Some(sub_attrs) = parse_tag9405(&entry.value.as_bytes()?, byte_order) {
                        attrs.set("Tag9405", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                _ => {
                    // Main tag - lookup in SONY_MAIN
                    if let Some(tag_def) = sony::SONY_MAIN.get(&entry.tag) {
                        let value = format_value(&entry, tag_def.values);
                        attrs.set(tag_def.name, value);
                    }
                }
            }
        }

        Some(attrs)
    }
}

/// Parse Sony CameraSettings (tag 0x0010).
fn parse_camera_settings(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 2;

    for i in 0..count.min(50) {
        if let Some(tag_def) = sony::SONY_CAMERASETTINGS.get(&(i as u16)) {
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

/// Parse Sony FocusInfo (tag 0x0020).
fn parse_focus_info(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 2;

    for i in 0..count.min(30) {
        if let Some(tag_def) = sony::SONY_FOCUSINFO.get(&(i as u16)) {
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

/// Parse Sony Tag9405 - distortion/lens correction info.
fn parse_tag9405(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 2;

    for i in 0..count.min(20) {
        if let Some(tag_def) = sony::SONY_TAG9405A.get(&(i as u16)) {
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
        assert_eq!(SonyParser.vendor(), Vendor::Sony);
    }
}

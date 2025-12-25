//! Olympus MakerNotes parser.
//!
//! Olympus MakerNotes structure:
//! - Type 1: "OLYMP\0" header (old cameras)
//! - Type 2: "OLYMPUS\0II" header with embedded TIFF (newer cameras)
//!
//! Olympus uses extensive sub-IFD structure:
//! - 0x2010: Equipment (sub-IFD)
//! - 0x2020: CameraSettings (sub-IFD)
//! - 0x2030: RawDevelopment (sub-IFD)
//! - 0x2031: RawDev2 (sub-IFD)
//! - 0x2040: ImageProcessing (sub-IFD)
//! - 0x2050: FocusInfo (sub-IFD)
//! - 0x2100: Olympus2100 (sub-IFD)
//! - 0x2200: Olympus2200 (sub-IFD)
//! - 0x2300: Olympus2300 (sub-IFD)
//! - 0x2400: Olympus2400 (sub-IFD)
//! - 0x2500: Olympus2500 (sub-IFD)
//! - 0x2600: Olympus2600 (sub-IFD)
//! - 0x2700: Olympus2700 (sub-IFD)
//! - 0x2800: Olympus2800 (sub-IFD)
//! - 0x2900: Olympus2900 (sub-IFD)
//! - 0x3000: RawInfo (sub-IFD)

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::{ByteOrder, IfdReader};
use exiftool_tags::generated::olympus;

/// Olympus MakerNotes parser.
pub struct OlympusParser;

/// Header magic for Olympus MakerNotes.
const OLYMP_HEADER: &[u8] = b"OLYMP\x00";
const OLYMPUS_HEADER: &[u8] = b"OLYMPUS\x00";

impl VendorParser for OlympusParser {
    fn vendor(&self) -> Vendor {
        Vendor::Olympus
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 8 {
            return None;
        }

        // Detect header type
        let (ifd_data, byte_order, ifd_offset) = if data.starts_with(OLYMPUS_HEADER) {
            // Type 2: "OLYMPUS\0II" with embedded TIFF
            if data.len() < 16 {
                return None;
            }
            let byte_order = if &data[8..10] == b"II" {
                ByteOrder::LittleEndian
            } else if &data[8..10] == b"MM" {
                ByteOrder::BigEndian
            } else {
                return None;
            };
            // IFD offset at bytes 12-15
            let offset = match byte_order {
                ByteOrder::LittleEndian => u32::from_le_bytes([data[12], data[13], data[14], data[15]]),
                ByteOrder::BigEndian => u32::from_be_bytes([data[12], data[13], data[14], data[15]]),
            } as usize;
            (data, byte_order, offset)
        } else if data.starts_with(OLYMP_HEADER) {
            // Type 1: "OLYMP\0" - old format
            (&data[8..], parent_byte_order, 0)
        } else {
            // No header, try as direct IFD
            (data, parent_byte_order, 0)
        };

        let reader = IfdReader::new(ifd_data, byte_order, 0);
        let (entries, _) = reader.read_ifd(ifd_offset as u32).ok()?;

        let mut attrs = Attrs::new();

        for entry in entries {
            match entry.tag {
                0x2010 => {
                    // Equipment sub-IFD
                    if let Some(offset) = entry.value.as_u32() {
                        if let Some(sub_attrs) = parse_equipment_ifd(ifd_data, byte_order, offset) {
                            attrs.set("Equipment", AttrValue::Group(Box::new(sub_attrs)));
                        }
                    }
                }
                0x2020 => {
                    // CameraSettings sub-IFD
                    if let Some(offset) = entry.value.as_u32() {
                        if let Some(sub_attrs) = parse_camera_settings_ifd(ifd_data, byte_order, offset) {
                            attrs.set("CameraSettings", AttrValue::Group(Box::new(sub_attrs)));
                        }
                    }
                }
                0x2040 => {
                    // ImageProcessing sub-IFD
                    if let Some(offset) = entry.value.as_u32() {
                        if let Some(sub_attrs) = parse_image_processing_ifd(ifd_data, byte_order, offset) {
                            attrs.set("ImageProcessing", AttrValue::Group(Box::new(sub_attrs)));
                        }
                    }
                }
                0x2050 => {
                    // FocusInfo sub-IFD
                    if let Some(offset) = entry.value.as_u32() {
                        if let Some(sub_attrs) = parse_focus_info_ifd(ifd_data, byte_order, offset) {
                            attrs.set("FocusInfo", AttrValue::Group(Box::new(sub_attrs)));
                        }
                    }
                }
                _ => {
                    // Main tag - lookup in OLYMPUS_MAIN
                    if let Some(tag_def) = olympus::OLYMPUS_MAIN.get(&entry.tag) {
                        let value = format_value(&entry, tag_def.values);
                        attrs.set(tag_def.name, value);
                    }
                }
            }
        }

        Some(attrs)
    }
}

/// Parse Olympus Equipment sub-IFD (tag 0x2010).
fn parse_equipment_ifd(data: &[u8], byte_order: ByteOrder, offset: u32) -> Option<Attrs> {
    if (offset as usize) >= data.len() {
        return None;
    }

    let reader = IfdReader::new(data, byte_order, 0);
    let (entries, _) = reader.read_ifd(offset).ok()?;

    let mut attrs = Attrs::new();

    for entry in entries {
        if let Some(tag_def) = olympus::OLYMPUS_EQUIPMENT.get(&entry.tag) {
            let value = format_value(&entry, tag_def.values);
            attrs.set(tag_def.name, value);
        }
    }

    Some(attrs)
}

/// Parse Olympus CameraSettings sub-IFD (tag 0x2020).
fn parse_camera_settings_ifd(data: &[u8], byte_order: ByteOrder, offset: u32) -> Option<Attrs> {
    if (offset as usize) >= data.len() {
        return None;
    }

    let reader = IfdReader::new(data, byte_order, 0);
    let (entries, _) = reader.read_ifd(offset).ok()?;

    let mut attrs = Attrs::new();

    for entry in entries {
        if let Some(tag_def) = olympus::OLYMPUS_CAMERASETTINGS.get(&entry.tag) {
            let value = format_value(&entry, tag_def.values);
            attrs.set(tag_def.name, value);
        }
    }

    Some(attrs)
}

/// Parse Olympus ImageProcessing sub-IFD (tag 0x2040).
fn parse_image_processing_ifd(data: &[u8], byte_order: ByteOrder, offset: u32) -> Option<Attrs> {
    if (offset as usize) >= data.len() {
        return None;
    }

    let reader = IfdReader::new(data, byte_order, 0);
    let (entries, _) = reader.read_ifd(offset).ok()?;

    let mut attrs = Attrs::new();

    for entry in entries {
        if let Some(tag_def) = olympus::OLYMPUS_IMAGEPROCESSING.get(&entry.tag) {
            let value = format_value(&entry, tag_def.values);
            attrs.set(tag_def.name, value);
        }
    }

    Some(attrs)
}

/// Parse Olympus FocusInfo sub-IFD (tag 0x2050).
fn parse_focus_info_ifd(data: &[u8], byte_order: ByteOrder, offset: u32) -> Option<Attrs> {
    if (offset as usize) >= data.len() {
        return None;
    }

    let reader = IfdReader::new(data, byte_order, 0);
    let (entries, _) = reader.read_ifd(offset).ok()?;

    let mut attrs = Attrs::new();

    for entry in entries {
        if let Some(tag_def) = olympus::OLYMPUS_FOCUSINFO.get(&entry.tag) {
            let value = format_value(&entry, tag_def.values);
            attrs.set(tag_def.name, value);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_vendor() {
        assert_eq!(OlympusParser.vendor(), Vendor::Olympus);
    }
}

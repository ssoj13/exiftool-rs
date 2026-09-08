//! Kodak MakerNotes parser.
//!
//! Kodak MakerNotes have various formats depending on model:
//! - Type1: Standard IFD (older cameras / headerless)
//! - Kodak1a: Header "KDK INFO" + Kodak::Main ProcessBinaryData (BE, skip 8)
//! - Kodak1b: Header "KDK" + Kodak::Main ProcessBinaryData (LE, skip 8)
//! - Type2–11: other ExifTool Kodak tables (not all implemented)
//!
//! Known tags (Type1/common):
//! - 0x0001: KodakModel
//! - 0x0003: YearCreated
//! - 0x0005: BurstMode
//! - 0x000e: ImageWidth
//! - 0x000f: ImageHeight
//! - 0x0010: Year/MonthDayCreated
//! - 0x0011: TimeCreated
//! - 0x0012: BurstMode2
//! - 0x001c: SerialNumber
//! - 0x001d: WhiteBalance
//! - 0x0024: FlashMode
//! - 0x0025: FlashFired
//! - 0x0026: ISOSetting
//! - 0x0027: ISO
//! - 0x0028: TotalZoom
//! - 0x0029: DateTimeStamp
//! - 0x0102: FocusMode
//! - 0x0104: Quality
//! - 0x0108: Flash
//! - 0x0109: RedEyeReduction
//! - 0x010a: DigitalZoom
//! - 0x010f: Sharpness

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::ByteOrder;

/// Kodak MakerNotes parser.
pub struct KodakParser;

/// Known Kodak MakerNote tags.
pub(crate) static KODAK_TAGS: &[(u16, &str)] = &[
    (0x0001, "KodakModel"),
    (0x0003, "YearCreated"),
    (0x0005, "BurstMode"),
    (0x000e, "ImageWidth"),
    (0x000f, "ImageHeight"),
    (0x0010, "YearMonthDayCreated"),
    (0x0011, "TimeCreated"),
    (0x0012, "BurstMode2"),
    (0x001c, "SerialNumber"),
    (0x001d, "WhiteBalance"),
    (0x0024, "FlashMode"),
    (0x0025, "FlashFired"),
    (0x0026, "ISOSetting"),
    (0x0027, "ISO"),
    (0x0028, "TotalZoom"),
    (0x0029, "DateTimeStamp"),
    (0x0037, "Sharpness"),
    (0x0038, "ExposureTime"),
    (0x0039, "FNumber"),
    (0x003b, "VariousModes"),
    (0x003c, "VariousModes2"),
    (0x0102, "FocusMode"),
    (0x0104, "Quality"),
    (0x0108, "Flash"),
    (0x0109, "RedEyeReduction"),
    (0x010a, "DigitalZoom"),
    (0x010f, "Sharpness2"),
    (0x0ffc, "FirmwareVersion"),
];

impl VendorParser for KodakParser {
    fn vendor(&self) -> Vendor {
        Vendor::Kodak
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 6 {
            return None;
        }

        // ExifTool MakerNoteKodak1a/1b: KDK* is ProcessBinaryData, not IFD.
        if data.starts_with(b"KDK INFO") && data.len() > 8 {
            return parse_kdk_main(&data[8..], ByteOrder::BigEndian);
        }
        if data.starts_with(b"KDK") && data.len() > 8 {
            return parse_kdk_main(&data[8..], ByteOrder::LittleEndian);
        }

        let entries = super::parse_ifd_entries(data, parent_byte_order, 0)?;

        let mut attrs = Attrs::new();

        for entry in entries {
            let tag_name = KODAK_TAGS
                .iter()
                .find(|(t, _)| *t == entry.tag)
                .map(|(_, n)| *n);

            if let Some(name) = tag_name {
                attrs.set(name, entry_to_attr(&entry));
            } else {
                let name = format!("Unknown_0x{:04X}", entry.tag);
                attrs.set(&name, entry_to_attr(&entry));
            }
        }

        Some(attrs)
    }
}

/// ExifTool Kodak::Main ProcessBinaryData after the 8-byte KDK header.
/// Scalar tags only (no ValueConv / multi-byte PrintConv strings).
pub(crate) struct KdkMainField {
    pub offset: usize,
    pub name: &'static str,
    pub width: u8,
    pub values: Option<&'static [(i64, &'static str)]>,
}

pub(crate) static KDK_MAIN_FIELDS: &[KdkMainField] = &[
    KdkMainField {
        offset: 9,
        name: "Quality",
        width: 1,
        values: Some(&[(1, "Fine"), (2, "Normal")]),
    },
    KdkMainField {
        offset: 10,
        name: "BurstMode",
        width: 1,
        values: Some(&[(0, "Off"), (1, "On")]),
    },
    KdkMainField {
        offset: 12,
        name: "KodakImageWidth",
        width: 2,
        values: None,
    },
    KdkMainField {
        offset: 14,
        name: "KodakImageHeight",
        width: 2,
        values: None,
    },
    KdkMainField {
        offset: 16,
        name: "YearCreated",
        width: 2,
        values: None,
    },
    KdkMainField {
        offset: 24,
        name: "BurstMode2",
        width: 2,
        values: None,
    },
    KdkMainField {
        offset: 27,
        name: "ShutterMode",
        width: 1,
        values: Some(&[(0, "Auto"), (8, "Aperture Priority"), (32, "Manual?")]),
    },
    KdkMainField {
        offset: 28,
        name: "MeteringMode",
        width: 1,
        values: Some(&[
            (0, "Multi-segment"),
            (1, "Center-weighted average"),
            (2, "Spot"),
        ]),
    },
    KdkMainField {
        offset: 29,
        name: "SequenceNumber",
        width: 1,
        values: None,
    },
];

fn kdk_print(value: u32, map: Option<&'static [(i64, &'static str)]>) -> AttrValue {
    if let Some(values) = map {
        values
            .iter()
            .find(|(k, _)| *k == i64::from(value))
            .map(|(_, v)| AttrValue::Str((*v).to_string()))
            .unwrap_or(AttrValue::UInt(value))
    } else {
        AttrValue::UInt(value)
    }
}

pub(crate) fn parse_kdk_main(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 10 {
        return None;
    }
    let mut attrs = Attrs::new();
    if data.len() >= 8 {
        let model = String::from_utf8_lossy(&data[..8]);
        let model = model.trim_end_matches('\0').trim();
        if !model.is_empty() {
            attrs.set("KodakModel", AttrValue::Str(model.to_string()));
        }
    }
    for field in KDK_MAIN_FIELDS {
        let end = field.offset + field.width as usize;
        if data.len() < end {
            continue;
        }
        let value = match field.width {
            1 => u32::from(data[field.offset]),
            2 => {
                let b = [data[field.offset], data[field.offset + 1]];
                u32::from(match byte_order {
                    ByteOrder::LittleEndian => u16::from_le_bytes(b),
                    ByteOrder::BigEndian => u16::from_be_bytes(b),
                })
            }
            _ => continue,
        };
        attrs.set(field.name, kdk_print(value, field.values));
    }
    Some(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_vendor() {
        assert_eq!(KodakParser.vendor(), Vendor::Kodak);
    }
}

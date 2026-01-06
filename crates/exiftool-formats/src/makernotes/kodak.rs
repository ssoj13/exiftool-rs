//! Kodak MakerNotes parser.
//!
//! Kodak MakerNotes have various formats depending on model:
//! - Type1: Standard IFD (older cameras)
//! - Type2: Header "KDK INFO" + IFD
//! - Type3: Header "KDK" + version + IFD
//! - Type4-11: Various other formats
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
use exiftool_attrs::Attrs;
use exiftool_core::ByteOrder;

/// Kodak MakerNotes parser.
pub struct KodakParser;

/// Known Kodak MakerNote tags.
static KODAK_TAGS: &[(u16, &str)] = &[
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

        // Check for Kodak headers
        let (ifd_data, byte_order) = if data.starts_with(b"KDK INFO") {
            // Type2: Skip 8-byte header
            (&data[8..], ByteOrder::BigEndian)
        } else if data.starts_with(b"KDK") && data.len() > 10 {
            // Type3: Skip header (varies by model)
            let skip = if data[3] == 0 { 4 } else { 8 };
            (&data[skip..], ByteOrder::BigEndian)
        } else {
            // Type1 or unknown: Direct IFD
            (data, parent_byte_order)
        };

        let entries = super::parse_ifd_entries(ifd_data, byte_order, 0)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_vendor() {
        assert_eq!(KodakParser.vendor(), Vendor::Kodak);
    }
}

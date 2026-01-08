//! Realme MakerNotes parser.
//!
//! Realme phones (Oppo sub-brand, later spun off but still uses similar format).
//!
//! Known tags:
//! - 0x0001: MakerNoteVersion
//! - 0x0002: DeviceModel
//! - 0x0003: FirmwareVersion
//! - 0x0100: SerialNumber
//! - 0x0200: SceneMode
//! - 0x0201: FocusMode
//! - 0x0202: ExposureMode
//! - 0x0203: WhiteBalance
//! - 0x0204: FlashMode
//! - 0x0210: AIScene
//! - 0x0211: AISceneType
//! - 0x0212: AISceneConfidence
//! - 0x0300: LensType
//! - 0x0301: ZoomLevel
//! - 0x0400: NightMode
//! - 0x0401: HDRMode
//! - 0x0402: ProMode
//! - 0x0403: StreetPhotography
//! - 0x0500: BeautyMode
//! - 0x0501: PortraitMode
//! - 0x0600: VideoMode
//! - 0x0601: SuperSteadyVideo

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::Attrs;
use exiftool_core::ByteOrder;

/// Realme MakerNotes parser.
pub struct RealmeParser;

/// Known Realme MakerNote tags.
static REALME_TAGS: &[(u16, &str)] = &[
    (0x0001, "MakerNoteVersion"),
    (0x0002, "DeviceModel"),
    (0x0003, "FirmwareVersion"),
    (0x0100, "SerialNumber"),
    (0x0200, "SceneMode"),
    (0x0201, "FocusMode"),
    (0x0202, "ExposureMode"),
    (0x0203, "WhiteBalance"),
    (0x0204, "FlashMode"),
    (0x0210, "AIScene"),
    (0x0211, "AISceneType"),
    (0x0212, "AISceneConfidence"),
    (0x0300, "LensType"),
    (0x0301, "ZoomLevel"),
    (0x0400, "NightMode"),
    (0x0401, "HDRMode"),
    (0x0402, "ProMode"),
    (0x0403, "StreetPhotography"),
    (0x0500, "BeautyMode"),
    (0x0501, "PortraitMode"),
    (0x0600, "VideoMode"),
    (0x0601, "SuperSteadyVideo"),
];

impl VendorParser for RealmeParser {
    fn vendor(&self) -> Vendor {
        Vendor::Realme
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        let entries = super::parse_ifd_entries(data, parent_byte_order, 0)?;

        let mut attrs = Attrs::new();

        for entry in entries {
            let tag_name = REALME_TAGS
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
        assert_eq!(RealmeParser.vendor(), Vendor::Realme);
    }
}

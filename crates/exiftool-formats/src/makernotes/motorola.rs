//! Motorola MakerNotes parser.
//!
//! Motorola phone cameras use standard TIFF IFD format.
//!
//! Known tags:
//! - 0x0001: MakerNoteVersion
//! - 0x0100: SerialNumber
//! - 0x0200: SceneMode
//! - 0x0201: FocusMode
//! - 0x0202: ExposureMode
//! - 0x0203: WhiteBalance
//! - 0x0204: FlashMode
//! - 0x0205: ISO
//! - 0x0300: AIScene
//! - 0x0301: AISceneConfidence
//! - 0x0400: LensType (wide/ultra-wide/telephoto/macro)
//! - 0x0401: ZoomRatio
//! - 0x0500: BeautyMode
//! - 0x0501: BeautyLevel
//! - 0x0600: HDRMode
//! - 0x0601: NightMode

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::Attrs;
use exiftool_core::ByteOrder;

/// Motorola MakerNotes parser.
pub struct MotorolaParser;

/// Known Motorola MakerNote tags.
static MOTOROLA_TAGS: &[(u16, &str)] = &[
    (0x0001, "MakerNoteVersion"),
    (0x0100, "SerialNumber"),
    (0x0200, "SceneMode"),
    (0x0201, "FocusMode"),
    (0x0202, "ExposureMode"),
    (0x0203, "WhiteBalance"),
    (0x0204, "FlashMode"),
    (0x0205, "ISO"),
    (0x0300, "AIScene"),
    (0x0301, "AISceneConfidence"),
    (0x0400, "LensType"),
    (0x0401, "ZoomRatio"),
    (0x0500, "BeautyMode"),
    (0x0501, "BeautyLevel"),
    (0x0600, "HDRMode"),
    (0x0601, "NightMode"),
];

impl VendorParser for MotorolaParser {
    fn vendor(&self) -> Vendor {
        Vendor::Motorola
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        // Motorola uses standard TIFF IFD format
        let entries = super::parse_ifd_entries(data, parent_byte_order, 0)?;

        let mut attrs = Attrs::new();

        for entry in entries {
            let tag_name = MOTOROLA_TAGS
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
        assert_eq!(MotorolaParser.vendor(), Vendor::Motorola);
    }
}

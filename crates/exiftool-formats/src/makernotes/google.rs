//! Google Pixel MakerNotes parser.
//!
//! Google Pixel cameras use standard TIFF IFD format.
//!
//! Known tags:
//! - 0x0001: MakerNoteVersion
//! - 0x0002: HDRPlusUsed
//! - 0x0003: NightModeUsed
//! - 0x0004: MotionPhoto
//! - 0x0005: MicroVideoVersion
//! - 0x0006: MicroVideoOffset
//! - 0x0007: MicroVideoPresentationTimestampUs
//! - 0x0008: PortraitModeUsed
//! - 0x0009: PortraitVersion
//! - 0x000a: DepthMap
//! - 0x000b: SpecialTypeID
//! - 0x000c: BurstId
//! - 0x000d: BurstPrimary
//! - 0x0010: CameraMode
//! - 0x0011: PhotoSphereInfo
//! - 0x0012: AstroCaptureMode
//! - 0x0013: LongExposureUsed
//! - 0x0014: MacroModeUsed

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::Attrs;
use exiftool_core::ByteOrder;

/// Google Pixel MakerNotes parser.
pub struct GoogleParser;

/// Known Google Pixel MakerNote tags.
static GOOGLE_TAGS: &[(u16, &str)] = &[
    (0x0001, "MakerNoteVersion"),
    (0x0002, "HDRPlusUsed"),
    (0x0003, "NightModeUsed"),
    (0x0004, "MotionPhoto"),
    (0x0005, "MicroVideoVersion"),
    (0x0006, "MicroVideoOffset"),
    (0x0007, "MicroVideoPresentationTimestampUs"),
    (0x0008, "PortraitModeUsed"),
    (0x0009, "PortraitVersion"),
    (0x000a, "DepthMap"),
    (0x000b, "SpecialTypeID"),
    (0x000c, "BurstId"),
    (0x000d, "BurstPrimary"),
    (0x0010, "CameraMode"),
    (0x0011, "PhotoSphereInfo"),
    (0x0012, "AstroCaptureMode"),
    (0x0013, "LongExposureUsed"),
    (0x0014, "MacroModeUsed"),
];

impl VendorParser for GoogleParser {
    fn vendor(&self) -> Vendor {
        Vendor::Google
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        // Google uses standard TIFF IFD format
        let entries = super::parse_ifd_entries(data, parent_byte_order, 0)?;

        let mut attrs = Attrs::new();

        for entry in entries {
            let tag_name = GOOGLE_TAGS
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
        assert_eq!(GoogleParser.vendor(), Vendor::Google);
    }
}

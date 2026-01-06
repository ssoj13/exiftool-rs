//! Huawei/Honor MakerNotes parser.
//!
//! Huawei phone cameras use standard TIFF IFD format.
//!
//! Known tags:
//! - 0x0001: MakerNoteVersion
//! - 0x0100: CaptureMode
//! - 0x0101: BurstNumber
//! - 0x0102: FocusMode
//! - 0x0103: AEMode
//! - 0x0104: RollAngle
//! - 0x0105: PitchAngle
//! - 0x0106: PhysicalAperture
//! - 0x0200: FrontCamera
//! - 0x0201: Orientation
//! - 0x0202: LightSource
//! - 0x0203: Brightness
//! - 0x0204: SceneMode
//! - 0x0205: FocusDistance
//! - 0x0206: AELock
//! - 0x0207: AWBLock
//! - 0x0210: FaceDetection
//! - 0x0211: FaceInfo
//! - 0x0212: FaceConfidence

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::Attrs;
use exiftool_core::ByteOrder;

/// Huawei MakerNotes parser.
pub struct HuaweiParser;

/// Known Huawei MakerNote tags.
static HUAWEI_TAGS: &[(u16, &str)] = &[
    (0x0001, "MakerNoteVersion"),
    (0x0100, "CaptureMode"),
    (0x0101, "BurstNumber"),
    (0x0102, "FocusMode"),
    (0x0103, "AEMode"),
    (0x0104, "RollAngle"),
    (0x0105, "PitchAngle"),
    (0x0106, "PhysicalAperture"),
    (0x0200, "FrontCamera"),
    (0x0201, "Orientation"),
    (0x0202, "LightSource"),
    (0x0203, "Brightness"),
    (0x0204, "SceneMode"),
    (0x0205, "FocusDistance"),
    (0x0206, "AELock"),
    (0x0207, "AWBLock"),
    (0x0210, "FaceDetection"),
    (0x0211, "FaceInfo"),
    (0x0212, "FaceConfidence"),
];

impl VendorParser for HuaweiParser {
    fn vendor(&self) -> Vendor {
        Vendor::Huawei
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        // Huawei uses standard TIFF IFD format
        let entries = super::parse_ifd_entries(data, parent_byte_order, 0)?;

        let mut attrs = Attrs::new();

        for entry in entries {
            let tag_name = HUAWEI_TAGS
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
        assert_eq!(HuaweiParser.vendor(), Vendor::Huawei);
    }
}

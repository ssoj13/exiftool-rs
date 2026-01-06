//! Xiaomi MakerNotes parser.
//!
//! Xiaomi phone cameras use standard TIFF IFD format.
//!
//! Known tags:
//! - 0x0001: MakerNoteVersion
//! - 0x0100: SerialNumber
//! - 0x0200: SceneMode
//! - 0x0201: AEMode
//! - 0x0202: FocusMode
//! - 0x0203: AWBMode
//! - 0x0204: FocusDistance
//! - 0x0205: FNumber
//! - 0x0206: ExposureProgram
//! - 0x0207: FlashMode
//! - 0x0208: FlashStatus
//! - 0x0210: AISceneDetection
//! - 0x0211: AISceneType
//! - 0x0212: BeautifyLevel
//! - 0x0213: NightMode
//! - 0x0214: HDRMode
//! - 0x0215: PortraitMode
//! - 0x0216: UltraWideAngle
//! - 0x0217: MacroMode
//! - 0x0218: ZoomLevel

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::Attrs;
use exiftool_core::{ByteOrder, IfdReader};

/// Xiaomi MakerNotes parser.
pub struct XiaomiParser;

/// Known Xiaomi MakerNote tags.
static XIAOMI_TAGS: &[(u16, &str)] = &[
    (0x0001, "MakerNoteVersion"),
    (0x0100, "SerialNumber"),
    (0x0200, "SceneMode"),
    (0x0201, "AEMode"),
    (0x0202, "FocusMode"),
    (0x0203, "AWBMode"),
    (0x0204, "FocusDistance"),
    (0x0205, "FNumber"),
    (0x0206, "ExposureProgram"),
    (0x0207, "FlashMode"),
    (0x0208, "FlashStatus"),
    (0x0210, "AISceneDetection"),
    (0x0211, "AISceneType"),
    (0x0212, "BeautifyLevel"),
    (0x0213, "NightMode"),
    (0x0214, "HDRMode"),
    (0x0215, "PortraitMode"),
    (0x0216, "UltraWideAngle"),
    (0x0217, "MacroMode"),
    (0x0218, "ZoomLevel"),
];

impl VendorParser for XiaomiParser {
    fn vendor(&self) -> Vendor {
        Vendor::Xiaomi
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 6 {
            return None;
        }

        // Xiaomi uses standard TIFF IFD format  
        let reader = IfdReader::new(data, parent_byte_order, 0);
        let (entries, _) = reader.read_ifd(0).ok()?;

        let mut attrs = Attrs::new();

        for entry in entries {
            let tag_name = XIAOMI_TAGS
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
        assert_eq!(XiaomiParser.vendor(), Vendor::Xiaomi);
    }
}

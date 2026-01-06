//! Hasselblad MakerNotes parser.
//!
//! Hasselblad medium format cameras use standard TIFF IFD format.
//!
//! Known tags:
//! - 0x0002: SerialNumber
//! - 0x0003: Model
//! - 0x0004: RawMode
//! - 0x0005: WhiteBalance
//! - 0x0006: SharpnessMode
//! - 0x0008: FlashMode
//! - 0x0009: FlashInfo
//! - 0x000b: AE_Lock
//! - 0x0010: ExposureMode
//! - 0x0011: ExposureCompensation
//! - 0x0012: MeteringMode
//! - 0x0015: DriveMode
//! - 0x001a: FocusMode
//! - 0x001b: ColorMode
//! - 0x001c: ColorProfile
//! - 0x0020: WhiteBalancePreset
//! - 0x0021: Sharpness
//! - 0x0022: Contrast
//! - 0x0023: Saturation
//! - 0x0028: ISO

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::Attrs;
use exiftool_core::{ByteOrder, IfdReader};

/// Hasselblad MakerNotes parser.
pub struct HasselbladParser;

/// Known Hasselblad MakerNote tags.
static HASSELBLAD_TAGS: &[(u16, &str)] = &[
    (0x0002, "SerialNumber"),
    (0x0003, "Model"),
    (0x0004, "RawMode"),
    (0x0005, "WhiteBalance"),
    (0x0006, "SharpnessMode"),
    (0x0008, "FlashMode"),
    (0x0009, "FlashInfo"),
    (0x000b, "AE_Lock"),
    (0x0010, "ExposureMode"),
    (0x0011, "ExposureCompensation"),
    (0x0012, "MeteringMode"),
    (0x0015, "DriveMode"),
    (0x001a, "FocusMode"),
    (0x001b, "ColorMode"),
    (0x001c, "ColorProfile"),
    (0x0020, "WhiteBalancePreset"),
    (0x0021, "Sharpness"),
    (0x0022, "Contrast"),
    (0x0023, "Saturation"),
    (0x0028, "ISO"),
];

impl VendorParser for HasselbladParser {
    fn vendor(&self) -> Vendor {
        Vendor::Hasselblad
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 6 {
            return None;
        }

        // Hasselblad uses standard TIFF IFD format
        let reader = IfdReader::new(data, parent_byte_order, 0);
        let (entries, _) = reader.read_ifd(0).ok()?;

        let mut attrs = Attrs::new();

        for entry in entries {
            let tag_name = HASSELBLAD_TAGS
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
        assert_eq!(HasselbladParser.vendor(), Vendor::Hasselblad);
    }
}

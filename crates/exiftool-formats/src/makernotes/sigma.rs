//! Sigma/Foveon MakerNotes parser.
//!
//! Sigma MakerNotes structure:
//! - Header: "SIGMA\0\0\0" or "FOVEON\0\0" (8 bytes)
//! - Standard TIFF IFD after header
//!
//! Known tags:
//! - 0x0002: SerialNumber
//! - 0x0003: DriveMode
//! - 0x0004: ResolutionMode
//! - 0x0005: AFMode
//! - 0x0006: FocusSetting
//! - 0x0007: WhiteBalance
//! - 0x0008: ExposureMode
//! - 0x0009: MeteringMode
//! - 0x000a: LensFocalRange
//! - 0x000b: ColorSpace
//! - 0x000c: ExposureCompensation
//! - 0x000d: Contrast
//! - 0x000e: Shadow
//! - 0x000f: Highlight
//! - 0x0010: Saturation
//! - 0x0011: Sharpness
//! - 0x0012: X3FillLight
//! - 0x0014: ColorAdjustment
//! - 0x0015: AdjustmentMode
//! - 0x0016: Quality
//! - 0x0017: Firmware
//! - 0x0018: Software
//! - 0x0019: AutoBracket

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::Attrs;
use exiftool_core::ByteOrder;

/// Sigma MakerNotes parser.
pub struct SigmaParser;

/// Known Sigma MakerNote tags.
static SIGMA_TAGS: &[(u16, &str)] = &[
    (0x0002, "SerialNumber"),
    (0x0003, "DriveMode"),
    (0x0004, "ResolutionMode"),
    (0x0005, "AFMode"),
    (0x0006, "FocusSetting"),
    (0x0007, "WhiteBalance"),
    (0x0008, "ExposureMode"),
    (0x0009, "MeteringMode"),
    (0x000a, "LensFocalRange"),
    (0x000b, "ColorSpace"),
    (0x000c, "ExposureCompensation"),
    (0x000d, "Contrast"),
    (0x000e, "Shadow"),
    (0x000f, "Highlight"),
    (0x0010, "Saturation"),
    (0x0011, "Sharpness"),
    (0x0012, "X3FillLight"),
    (0x0014, "ColorAdjustment"),
    (0x0015, "AdjustmentMode"),
    (0x0016, "Quality"),
    (0x0017, "Firmware"),
    (0x0018, "Software"),
    (0x0019, "AutoBracket"),
];

impl VendorParser for SigmaParser {
    fn vendor(&self) -> Vendor {
        Vendor::Sigma
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 10 {
            return None;
        }

        // Check for Sigma/Foveon header
        let (ifd_data, byte_order) = if data.starts_with(b"SIGMA\0\0\0") || data.starts_with(b"FOVEON\0\0") {
            // Skip 8-byte header, use LE for Sigma
            (&data[8..], ByteOrder::LittleEndian)
        } else {
            // No header, try as direct IFD
            (data, parent_byte_order)
        };

        let entries = super::parse_ifd_entries(ifd_data, byte_order, 0)?;

        let mut attrs = Attrs::new();

        for entry in entries {
            let tag_name = SIGMA_TAGS
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
        assert_eq!(SigmaParser.vendor(), Vendor::Sigma);
    }
}

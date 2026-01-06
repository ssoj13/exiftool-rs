//! Minolta/Konica Minolta MakerNotes parser.
//!
//! Minolta MakerNotes structure:
//! - Standard TIFF IFD format (no header)
//! - Uses parent byte order
//!
//! Known tags:
//! - 0x0000: MakerNoteVersion
//! - 0x0001: MinoltaCameraSettingsOld (binary)
//! - 0x0003: MinoltaCameraSettings (binary)
//! - 0x0004: MinoltaCameraSettings7D (binary)
//! - 0x0010: CameraInfoA100 (binary)
//! - 0x0018: ImageStabilization
//! - 0x0020: CameraInfo (binary)
//! - 0x0040: CompressedImageSize
//! - 0x0081: JPEGQuality (0=Normal, 1=Fine)
//! - 0x0088: ExposureMode
//! - 0x0089: FlashMode
//! - 0x0100: ZoneMatching
//! - 0x0101: ColorTemperature
//! - 0x0102: LensID
//! - 0x0103: Quality (0=RAW, 16=Normal, 21=Fine, etc.)
//! - 0x0104: ImageSize (0=Full, 1=1600x1200, 2=1280x960, 3=640x480)
//! - 0x0e00: PrintIM
//! - 0x0f00: MinoltaCameraSettings2 (binary)

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::ByteOrder;

/// Minolta MakerNotes parser.
pub struct MinoltaParser;

/// Known Minolta MakerNote tags.
static MINOLTA_TAGS: &[(u16, &str)] = &[
    (0x0000, "MakerNoteVersion"),
    (0x0001, "MinoltaCameraSettingsOld"),
    (0x0003, "MinoltaCameraSettings"),
    (0x0004, "MinoltaCameraSettings7D"),
    (0x0010, "CameraInfoA100"),
    (0x0018, "ImageStabilization"),
    (0x0020, "CameraInfo"),
    (0x0040, "CompressedImageSize"),
    (0x0081, "JPEGQuality"),
    (0x0088, "ExposureMode"),
    (0x0089, "FlashMode"),
    (0x008b, "WhiteBalance"),
    (0x008c, "ImageSize"),
    (0x008d, "Quality"),
    (0x0100, "ZoneMatching"),
    (0x0101, "ColorTemperature"),
    (0x0102, "LensID"),
    (0x0103, "Quality2"),
    (0x0104, "ImageSize2"),
    (0x0200, "FreeMemoryCardImages"),
    (0x0201, "ColorMode"),
    (0x0202, "MinoltaImageSize"),
    (0x0203, "MinoltaQuality"),
    (0x0e00, "PrintIM"),
    (0x0f00, "MinoltaCameraSettings2"),
];

/// JPEG Quality values.
static JPEG_QUALITY: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Fine"),
];

/// Exposure Mode values.
static EXPOSURE_MODE: &[(i64, &str)] = &[
    (0, "Program"),
    (1, "Aperture Priority"),
    (2, "Shutter Priority"),
    (3, "Manual"),
];

/// Flash Mode values.
static FLASH_MODE: &[(i64, &str)] = &[
    (0, "Fill Flash"),
    (1, "Red-eye reduction"),
    (2, "Rear flash sync"),
    (3, "Wireless"),
    (4, "Off"),
];

/// White Balance values.
static WHITE_BALANCE: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Daylight"),
    (2, "Cloudy"),
    (3, "Tungsten"),
    (5, "Custom"),
    (7, "Fluorescent"),
    (8, "Fluorescent 2"),
    (11, "Custom 2"),
    (12, "Custom 3"),
];

impl VendorParser for MinoltaParser {
    fn vendor(&self) -> Vendor {
        Vendor::Minolta
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        // Minolta uses standard TIFF IFD format starting at offset 0
        let entries = super::parse_ifd_entries(data, parent_byte_order, 0)?;

        let mut attrs = Attrs::new();

        for entry in entries {
            // Lookup tag name
            let tag_name = MINOLTA_TAGS
                .iter()
                .find(|(t, _)| *t == entry.tag)
                .map(|(_, n)| *n)
                .unwrap_or_else(|| {
                    // Unknown tag - use hex ID
                    ""
                });

            if tag_name.is_empty() {
                // Skip unknown tags or add with hex ID
                let name = format!("Unknown_0x{:04X}", entry.tag);
                attrs.set(&name, entry_to_attr(&entry));
                continue;
            }

            // Apply PrintConv for known values
            let value = match entry.tag {
                0x0081 => format_with_map(&entry, JPEG_QUALITY),
                0x0088 => format_with_map(&entry, EXPOSURE_MODE),
                0x0089 => format_with_map(&entry, FLASH_MODE),
                0x008b => format_with_map(&entry, WHITE_BALANCE),
                _ => entry_to_attr(&entry),
            };

            attrs.set(tag_name, value);
        }

        Some(attrs)
    }
}

/// Format IFD entry value with lookup map.
fn format_with_map(entry: &exiftool_core::IfdEntry, map: &[(i64, &str)]) -> AttrValue {
    if let Some(int_val) = entry.value.as_u32().map(|v| v as i64) {
        for &(key, label) in map {
            if key == int_val {
                return AttrValue::Str(label.to_string());
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
        assert_eq!(MinoltaParser.vendor(), Vendor::Minolta);
    }
}

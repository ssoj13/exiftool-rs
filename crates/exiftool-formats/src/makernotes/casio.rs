//! Casio MakerNotes parser.
//!
//! Casio has two MakerNotes formats:
//! - Type1: Standard TIFF IFD (older cameras)
//! - Type2: Header "QVC\0\0\0" (6 bytes) + TIFF IFD (newer cameras)
//!
//! Known tags (Type1):
//! - 0x0001: RecordingMode
//! - 0x0002: Quality
//! - 0x0003: FocusingMode
//! - 0x0004: FlashMode
//! - 0x0005: FlashIntensity
//! - 0x0006: ObjectDistance
//! - 0x0007: WhiteBalance
//! - 0x000a: DigitalZoom
//! - 0x000b: Sharpness
//! - 0x000c: Contrast
//! - 0x000d: Saturation
//! - 0x0014: CCDISOSensitivity
//!
//! Known tags (Type2):
//! - 0x0002: PreviewImageSize
//! - 0x0003: PreviewImageLength
//! - 0x0004: PreviewImageStart
//! - 0x0008: QualityMode
//! - 0x0009: ImageSize
//! - 0x000d: FocusMode
//! - 0x0014: ISO
//! - 0x0019: WhiteBalance
//! - 0x001d: FocalLength
//! - 0x001f: Saturation
//! - 0x0020: Contrast
//! - 0x0021: Sharpness
//! - 0x2000: PreviewImage
//! - 0x2011: WhiteBalanceBias
//! - 0x2012: WhiteBalance2
//! - 0x2022: ObjectDistance
//! - 0x2034: FlashDistance
//! - 0x3000: RecordMode
//! - 0x3001: ReleaseMode
//! - 0x3002: Quality2
//! - 0x3003: FocusMode2
//! - 0x3006: HometownCity
//! - 0x3007: BestShotMode
//! - 0x3014: CCDISOSensitivity
//! - 0x3015: ColourMode
//! - 0x3016: Enhancement
//! - 0x3017: Filter

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::Attrs;
use exiftool_core::{ByteOrder, IfdReader};

/// Casio MakerNotes parser.
pub struct CasioParser;

/// Known Casio Type1 tags.
static CASIO_TYPE1_TAGS: &[(u16, &str)] = &[
    (0x0001, "RecordingMode"),
    (0x0002, "Quality"),
    (0x0003, "FocusingMode"),
    (0x0004, "FlashMode"),
    (0x0005, "FlashIntensity"),
    (0x0006, "ObjectDistance"),
    (0x0007, "WhiteBalance"),
    (0x000a, "DigitalZoom"),
    (0x000b, "Sharpness"),
    (0x000c, "Contrast"),
    (0x000d, "Saturation"),
    (0x0014, "CCDISOSensitivity"),
];

/// Known Casio Type2 tags.
static CASIO_TYPE2_TAGS: &[(u16, &str)] = &[
    (0x0002, "PreviewImageSize"),
    (0x0003, "PreviewImageLength"),
    (0x0004, "PreviewImageStart"),
    (0x0008, "QualityMode"),
    (0x0009, "CasioImageSize"),
    (0x000d, "FocusMode"),
    (0x0014, "ISO"),
    (0x0019, "WhiteBalance"),
    (0x001d, "FocalLength"),
    (0x001f, "Saturation"),
    (0x0020, "Contrast"),
    (0x0021, "Sharpness"),
    (0x2000, "PreviewImage"),
    (0x2011, "WhiteBalanceBias"),
    (0x2012, "WhiteBalance2"),
    (0x2022, "ObjectDistance"),
    (0x2034, "FlashDistance"),
    (0x3000, "RecordMode"),
    (0x3001, "ReleaseMode"),
    (0x3002, "Quality2"),
    (0x3003, "FocusMode2"),
    (0x3006, "HometownCity"),
    (0x3007, "BestShotMode"),
    (0x3014, "CCDISOSensitivity"),
    (0x3015, "ColourMode"),
    (0x3016, "Enhancement"),
    (0x3017, "Filter"),
];

impl VendorParser for CasioParser {
    fn vendor(&self) -> Vendor {
        Vendor::Casio
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 6 {
            return None;
        }

        // Check for Type2 header "QVC\0\0\0"
        let (ifd_data, tags, byte_order) = if data.starts_with(b"QVC\0\0\0") {
            // Type2: Skip 6-byte header
            (&data[6..], &CASIO_TYPE2_TAGS[..], ByteOrder::LittleEndian)
        } else {
            // Type1: Direct IFD
            (data, &CASIO_TYPE1_TAGS[..], parent_byte_order)
        };

        let reader = IfdReader::new(ifd_data, byte_order, 0);
        let (entries, _) = reader.read_ifd(0).ok()?;

        let mut attrs = Attrs::new();

        for entry in entries {
            let tag_name = tags
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
        assert_eq!(CasioParser.vendor(), Vendor::Casio);
    }
}

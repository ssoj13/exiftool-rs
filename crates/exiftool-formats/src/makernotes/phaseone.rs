//! Phase One MakerNotes parser.
//!
//! Phase One medium format cameras use standard TIFF IFD format.
//! Also covers Leaf and Mamiya digital backs.
//!
//! Known tags:
//! - 0x0100: CameraOrientation
//! - 0x0102: Software
//! - 0x0105: SerialNumber
//! - 0x0106: ISO
//! - 0x0107: ImageFormat
//! - 0x0108: RawFormat
//! - 0x0109: SensorWidth
//! - 0x010a: SensorHeight
//! - 0x010b: SensorLeftMargin
//! - 0x010c: SensorTopMargin
//! - 0x010d: ImageWidth
//! - 0x010e: ImageHeight
//! - 0x0110: DateTimeOriginal
//! - 0x0112: SensorTemperature
//! - 0x0203: SensorTemperature2
//! - 0x0210: StripOffsets
//! - 0x0211: StripByteCounts
//! - 0x021c: WhiteBalance
//! - 0x0220: UserCrop
//! - 0x0301: ShutterSpeedValue
//! - 0x0303: ApertureValue
//! - 0x0304: Brightness
//! - 0x0305: ExposureCompensation
//! - 0x0306: FocusingDistance
//! - 0x0308: FocalLength
//! - 0x0400: System  
//! - 0x0401: SensorCalibration
//! - 0x0402: SoftwareRelease

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::Attrs;
use exiftool_core::{ByteOrder, IfdReader};

/// Phase One MakerNotes parser.
pub struct PhaseOneParser;

/// Known Phase One MakerNote tags.
static PHASEONE_TAGS: &[(u16, &str)] = &[
    (0x0100, "CameraOrientation"),
    (0x0102, "Software"),
    (0x0105, "SerialNumber"),
    (0x0106, "ISO"),
    (0x0107, "ImageFormat"),
    (0x0108, "RawFormat"),
    (0x0109, "SensorWidth"),
    (0x010a, "SensorHeight"),
    (0x010b, "SensorLeftMargin"),
    (0x010c, "SensorTopMargin"),
    (0x010d, "ImageWidth"),
    (0x010e, "ImageHeight"),
    (0x0110, "DateTimeOriginal"),
    (0x0112, "SensorTemperature"),
    (0x0203, "SensorTemperature2"),
    (0x0210, "StripOffsets"),
    (0x0211, "StripByteCounts"),
    (0x021c, "WhiteBalance"),
    (0x0220, "UserCrop"),
    (0x0301, "ShutterSpeedValue"),
    (0x0303, "ApertureValue"),
    (0x0304, "Brightness"),
    (0x0305, "ExposureCompensation"),
    (0x0306, "FocusingDistance"),
    (0x0308, "FocalLength"),
    (0x0400, "System"),
    (0x0401, "SensorCalibration"),
    (0x0402, "SoftwareRelease"),
    (0x0403, "LensFocalLength"),
    (0x0404, "LensMaxAperture"),
    (0x0405, "LensMinAperture"),
    (0x0406, "LensSerialNumber"),
];

impl VendorParser for PhaseOneParser {
    fn vendor(&self) -> Vendor {
        Vendor::PhaseOne
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 6 {
            return None;
        }

        // Phase One uses standard TIFF IFD format
        let reader = IfdReader::new(data, parent_byte_order, 0);
        let (entries, _) = reader.read_ifd(0).ok()?;

        let mut attrs = Attrs::new();

        for entry in entries {
            let tag_name = PHASEONE_TAGS
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
        assert_eq!(PhaseOneParser.vendor(), Vendor::PhaseOne);
    }
}

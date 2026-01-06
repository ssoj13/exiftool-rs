//! Samsung MakerNotes parser.
//!
//! Samsung MakerNotes structure:
//! - No header, starts with IFD
//! - Uses parent byte order
//!
//! Known tags:
//! - 0x0001: MakerNoteVersion
//! - 0x0002: DeviceType
//! - 0x0003: SamsungModelID
//! - 0x0021: PictureWizard
//! - 0x0030: LocalLocationName
//! - 0x0031: LocationName
//! - 0x0035: Preview
//! - 0x0043: CameraTemperature
//! - 0x0050: RawDataByteOrder
//! - 0x0060: RawDataCFAPattern
//! - 0x0100: FaceDetect
//! - 0x0120: FaceRecognition
//! - 0x0123: FaceName
//! - 0x0140: SmartRange
//! - 0x0a01: FirmwareName
//! - 0xa001: ColorSpace2
//! - 0xa003: ExposureCompensation
//! - 0xa004: Contrast
//! - 0xa010: ColorMode
//! - 0xa011: Sharpness
//! - 0xa012: Saturation
//! - 0xa013: WB_RGGBLevels
//! - 0xa018: ExposureBracketValue
//! - 0xa019: ISO
//! - 0xa020: DigitalZoom
//! - 0xa021: HDR
//! - 0xa022: SmartFilter1
//! - 0xa023: SmartFilter2
//! - 0xa024: SmartFilter3
//! - 0xa025: SmartFilter4
//! - 0xa028: PanoramaMode
//! - 0xa030: Highlight
//! - 0xa031: Shadow
//! - 0xa033: Hue
//! - 0xa034: SmartRange
//! - 0xa035: ExposureProgram
//! - 0xa036: SmartAlbumColor
//! - 0xa040: SensorAreas
//! - 0xa041: SamsungExposureComp
//! - 0xa043: RawData
//! - 0xa048: OrientationInfo
//! - 0xa050: SmartLensInfo
//! - 0xa060: Panorama
//! - 0xa101: PhotoStyleSelect
//! - 0xa102: MovieColorControl
//! - 0xa103: MovieColor
//! - 0xa104: SmartMovie
//! - 0xa200: SensorInfo
//! - 0xa201: SensorCalibration
//! - 0xa202: EncryptionKey

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::ByteOrder;
use exiftool_tags::generated::samsung;

/// Samsung MakerNotes parser.
pub struct SamsungParser;

impl VendorParser for SamsungParser {
    fn vendor(&self) -> Vendor {
        Vendor::Samsung
    }

    fn parse(&self, data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
        let entries = super::parse_ifd_entries(data, byte_order, 0)?;

        let mut attrs = Attrs::new();

        for entry in entries {
            // Main tag - lookup in SAMSUNG_MAIN
            if let Some(tag_def) = samsung::SAMSUNG_MAIN.get(&entry.tag) {
                let value = format_value(&entry, tag_def.values);
                attrs.set(tag_def.name, value);
            }
        }

        Some(attrs)
    }
}

/// Format IFD entry value with PrintConv lookup.
fn format_value(entry: &exiftool_core::IfdEntry, values_map: Option<&'static [(i64, &'static str)]>) -> AttrValue {
    if let Some(map) = values_map {
        if let Some(int_val) = entry.value.as_u32().map(|v| v as i64) {
            for &(key, label) in map {
                if key == int_val {
                    return AttrValue::Str(label.to_string());
                }
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
        assert_eq!(SamsungParser.vendor(), Vendor::Samsung);
    }
}

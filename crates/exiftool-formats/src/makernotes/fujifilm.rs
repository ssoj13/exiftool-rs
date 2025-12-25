//! Fujifilm MakerNotes parser.
//!
//! Fujifilm MakerNotes structure:
//! - Header: "FUJIFILM" (8 bytes) + version (4 bytes)
//! - Offset to IFD in header (relative to start of MakerNotes)
//! - Always little-endian regardless of parent byte order
//!
//! Known tags:
//! - 0x0000: Version
//! - 0x0010: InternalSerialNumber
//! - 0x1000: Quality
//! - 0x1001: Sharpness
//! - 0x1002: WhiteBalance
//! - 0x1003: Saturation
//! - 0x1004: Contrast
//! - 0x1005: ColorTemperature
//! - 0x1006: Contrast (shadow/highlight)
//! - 0x100A: WhiteBalanceFineTune
//! - 0x100B: NoiseReduction
//! - 0x100E: HighISONoiseReduction
//! - 0x1010: FujiFlashMode
//! - 0x1011: FlashExposureComp
//! - 0x1020: Macro
//! - 0x1021: FocusMode
//! - 0x1022: AFMode
//! - 0x1023: FocusPixel
//! - 0x102B: PrioritySettings
//! - 0x102D: FocusSettings
//! - 0x102E: AFCSettings
//! - 0x1030: SlowSync
//! - 0x1031: PictureMode
//! - 0x1032: ExposureCount
//! - 0x1033: EXRAuto
//! - 0x1034: EXRMode
//! - 0x1040: ShadowTone
//! - 0x1041: HighlightTone
//! - 0x1044: DigitalZoom
//! - 0x1045: LensModulationOptimizer
//! - 0x1047: GrainEffect
//! - 0x1048: ColorChromeEffect
//! - 0x1049: BWAdjustment
//! - 0x104D: ColorChromeFXBlue
//! - 0x1050: ShutterType
//! - 0x1100: AutoBracketing
//! - 0x1101: SequenceNumber
//! - 0x1103: DriveSettings
//! - 0x1210: Panorama
//! - 0x1400: DynamicRange
//! - 0x1401: FilmMode
//! - 0x1402: DynamicRangeSetting
//! - 0x1403: DevelopmentDynamicRange
//! - 0x1404: MinFocalLength
//! - 0x1405: MaxFocalLength
//! - 0x1406: MaxApertureAtMinFocal
//! - 0x1407: MaxApertureAtMaxFocal
//! - 0x140B: AutoDynamicRange
//! - 0x1422: ImageStabilization
//! - 0x1425: SceneRecognition
//! - 0x1431: Rating
//! - 0x1436: ImageGeneration
//! - 0x1438: ImageCount
//! - 0x1443: DRangePriority
//! - 0x1444: DRangePriorityAuto
//! - 0x1445: DRangePriorityFixed
//! - 0x1446: FlickerReduction
//! - 0x4100: FacesDetected
//! - 0x4200: NumFaceElements
//! - 0x8000: FileSource
//! - 0x8002: OrderNumber
//! - 0x8003: FrameNumber

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::{ByteOrder, IfdReader};
use exiftool_tags::generated::fujifilm;

/// Fujifilm MakerNotes parser.
pub struct FujifilmParser;

/// Header magic for Fujifilm MakerNotes.
const FUJIFILM_HEADER: &[u8] = b"FUJIFILM";

impl VendorParser for FujifilmParser {
    fn vendor(&self) -> Vendor {
        Vendor::Fujifilm
    }

    fn parse(&self, data: &[u8], _parent_byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 12 {
            return None;
        }

        // Check header
        if !data.starts_with(FUJIFILM_HEADER) {
            return None;
        }

        // Fujifilm MakerNotes are always little-endian
        let byte_order = ByteOrder::LittleEndian;

        // IFD offset at bytes 8-11
        let ifd_offset = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;
        
        if ifd_offset >= data.len() {
            return None;
        }

        let reader = IfdReader::new(data, byte_order, 0);
        let (entries, _) = reader.read_ifd(ifd_offset as u32).ok()?;

        let mut attrs = Attrs::new();

        for entry in entries {
            match entry.tag {
                0x102E => {
                    // AFCSettings
                    if let Some(sub_attrs) = parse_afc_settings(&entry.value.as_bytes()?, byte_order) {
                        attrs.set("AFCSettings", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                _ => {
                    // Main tag - lookup in FUJIFILM_MAIN
                    if let Some(tag_def) = fujifilm::FUJIFILM_MAIN.get(&entry.tag) {
                        let value = format_value(&entry, tag_def.values);
                        attrs.set(tag_def.name, value);
                    }
                }
            }
        }

        Some(attrs)
    }
}

/// Parse Fujifilm AFCSettings (tag 0x102E).
fn parse_afc_settings(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    
    // AFCSettings is a structured binary blob
    // Specific parsing depends on firmware version
    if data.len() >= 2 {
        let value = read_u16(data, 0, byte_order);
        attrs.set("AFCSetting", AttrValue::UInt(value as u32));
    }

    Some(attrs)
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

/// Read u16 from byte slice with byte order.
#[inline]
fn read_u16(data: &[u8], offset: usize, byte_order: ByteOrder) -> u16 {
    if offset + 2 > data.len() {
        return 0;
    }
    match byte_order {
        ByteOrder::LittleEndian => u16::from_le_bytes([data[offset], data[offset + 1]]),
        ByteOrder::BigEndian => u16::from_be_bytes([data[offset], data[offset + 1]]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_vendor() {
        assert_eq!(FujifilmParser.vendor(), Vendor::Fujifilm);
    }
}

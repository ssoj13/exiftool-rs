//! Pentax MakerNotes parser.
//!
//! Pentax MakerNotes structure:
//! - Type 1: "AOC\0" header (4 bytes) - older cameras
//! - Type 2: "PENTAX \0" header (8 bytes) - newer cameras
//! - Byte order from header or parent
//!
//! Known tags:
//! - 0x0000: PentaxVersion
//! - 0x0001: PentaxMode
//! - 0x0002: PreviewImageSize
//! - 0x0003: PreviewImageLength
//! - 0x0004: PreviewImageStart
//! - 0x0005: PentaxModelID
//! - 0x0006: Date
//! - 0x0007: Time
//! - 0x0008: Quality
//! - 0x0009: PentaxImageSize
//! - 0x000B: PictureMode
//! - 0x000C: FlashMode
//! - 0x000D: FocusMode
//! - 0x000E: AFPointSelected
//! - 0x000F: AFPointsInFocus
//! - 0x0010: FocusPosition
//! - 0x0012: ExposureTime
//! - 0x0013: FNumber
//! - 0x0014: ISO
//! - 0x0015: LightReading (older cameras)
//! - 0x0016: ExposureCompensation
//! - 0x0017: MeteringMode
//! - 0x0018: AutoBracketing
//! - 0x0019: WhiteBalance
//! - 0x001A: WhiteBalanceMode
//! - 0x001B: BlueBalance
//! - 0x001C: RedBalance
//! - 0x001D: FocalLength
//! - 0x001E: DigitalZoom
//! - 0x001F: Saturation
//! - 0x0020: Contrast
//! - 0x0021: Sharpness
//! - 0x0022: WorldTimeLocation
//! - 0x0023: HometownCity
//! - 0x0024: DestinationCity
//! - 0x0025: HometownDST
//! - 0x0026: DestinationDST
//! - 0x0027: DSPFirmwareVersion
//! - 0x0028: CPUFirmwareVersion
//! - 0x0029: FrameNumber
//! - 0x002D: EffectiveLV
//! - 0x0032: ImageProcessing
//! - 0x0033: PictureMode2
//! - 0x0034: DriveMode
//! - 0x0037: ColorSpace
//! - 0x0038: ImageAreaOffset
//! - 0x0039: RawImageSize
//! - 0x003C: AFPointsInFocus2
//! - 0x003E: PreviewImageBorders
//! - 0x003F: LensRec
//! - 0x0040: SensitivityAdjust
//! - 0x0041: ImageEditCount
//! - 0x0047: CameraTemperature
//! - 0x0048: AELock
//! - 0x0049: NoiseReduction
//! - 0x004D: FlashExposureComp
//! - 0x004F: ImageTone
//! - 0x0050: ColorTemperature
//! - 0x005C: ShakeReduction
//! - 0x005D: ShutterCount
//! - 0x0060: FaceInfo (sub-IFD)
//! - 0x0067: Hue
//! - 0x0068: AWBInfo
//! - 0x0069: DynamicRangeExpansion
//! - 0x006B: TimeInfo
//! - 0x006C: HighLowKeyAdj
//! - 0x006D: ContrastHighlight
//! - 0x006E: ContrastShadow
//! - 0x006F: ContrastHighlightShadowAdj
//! - 0x0070: FineSharpness
//! - 0x0071: HighISONoiseReduction
//! - 0x0072: AFAdjustment
//! - 0x0073: MonochromeFilterEffect
//! - 0x0074: MonochromeToning
//! - 0x0076: FaceDetect
//! - 0x0077: FaceDetectFrameSize
//! - 0x0079: ShadowCorrection
//! - 0x007A: ISOAutoParameters
//! - 0x007B: CrossProcess
//! - 0x007D: LensCorr
//! - 0x007E: WhiteLevel
//! - 0x007F: BleachBypassToning
//! - 0x0082: AspectRatio
//! - 0x0085: HDR
//! - 0x0087: ShutterType
//! - 0x0088: NeutralDensityFilter
//! - 0x008B: ISO2
//! - 0x0200: BlackPoint
//! - 0x0201: WhitePoint
//! - 0x0205: ShotInfo (sub-IFD)
//! - 0x0206: AEInfo (sub-IFD)
//! - 0x0207: LensInfo (sub-IFD)
//! - 0x0208: FlashInfo (sub-IFD)
//! - 0x0209: AEMeteringSegments
//! - 0x020A: FlashMeteringSegments
//! - 0x020B: SlaveFlashMeteringSegments
//! - 0x020D: WBShiftAB
//! - 0x020E: WBShiftMG
//! - 0x020F: CameraInfo (sub-IFD)
//! - 0x0210: BatteryInfo (sub-IFD)
//! - 0x0211: SaturationInfo
//! - 0x0215: AFInfo (sub-IFD)
//! - 0x0216: HuffmanTable
//! - 0x0220: ColorInfo (sub-IFD)
//! - 0x0222: EVStepInfo (sub-IFD)

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::ByteOrder;
use exiftool_tags::generated::pentax;

/// Pentax MakerNotes parser.
pub struct PentaxParser;

/// Header magic for Pentax MakerNotes.
const AOC_HEADER: &[u8] = b"AOC\x00";
const PENTAX_HEADER: &[u8] = b"PENTAX ";

impl VendorParser for PentaxParser {
    fn vendor(&self) -> Vendor {
        Vendor::Pentax
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 8 {
            return None;
        }

        // Detect header type
        let (ifd_data, byte_order) = if data.starts_with(AOC_HEADER) {
            // Type 1: "AOC\0" + byte order marker
            let byte_order = if data.len() > 4 {
                if &data[4..6] == b"II" {
                    ByteOrder::LittleEndian
                } else if &data[4..6] == b"MM" {
                    ByteOrder::BigEndian
                } else {
                    parent_byte_order
                }
            } else {
                parent_byte_order
            };
            (&data[6..], byte_order)
        } else if data.starts_with(PENTAX_HEADER) {
            // Type 2: "PENTAX \0" + byte order marker
            let byte_order = if data.len() > 8 {
                if &data[8..10] == b"II" {
                    ByteOrder::LittleEndian
                } else if &data[8..10] == b"MM" {
                    ByteOrder::BigEndian
                } else {
                    parent_byte_order
                }
            } else {
                parent_byte_order
            };
            (&data[10..], byte_order)
        } else {
            // No header
            (data, parent_byte_order)
        };

        let entries = super::parse_ifd_entries(ifd_data, byte_order, 0)?;

        let mut attrs = Attrs::new();

        // First pass: collect preview offset/length
        let mut preview_length: Option<u32> = None;
        let mut preview_start: Option<u32> = None;
        
        for entry in &entries {
            match entry.tag {
                0x0003 => preview_length = entry.value.as_u32(),
                0x0004 => preview_start = entry.value.as_u32(),
                _ => {}
            }
        }
        
        // Store preview info if found
        if let (Some(off), Some(len)) = (preview_start, preview_length) {
            attrs.set("PreviewImageStart", AttrValue::UInt(off));
            attrs.set("PreviewImageLength", AttrValue::UInt(len));
        }

        for entry in entries {
            match entry.tag {
                0x0003 | 0x0004 => {
                    // Already handled above
                }
                0x0207 => {
                    // LensInfo sub-IFD
                    if let Some(offset) = entry.value.as_u32() {
                        if let Some(sub_attrs) = parse_lens_info(ifd_data, byte_order, offset) {
                            attrs.set("LensInfo", AttrValue::Group(Box::new(sub_attrs)));
                        }
                    }
                }
                0x0215 => {
                    // AFInfo sub-IFD  
                    if let Some(offset) = entry.value.as_u32() {
                        if let Some(sub_attrs) = parse_af_info(ifd_data, byte_order, offset) {
                            attrs.set("AFInfo", AttrValue::Group(Box::new(sub_attrs)));
                        }
                    }
                }
                _ => {
                    // Main tag - lookup in PENTAX_MAIN
                    if let Some(tag_def) = pentax::PENTAX_MAIN.get(&entry.tag) {
                        let value = format_value(&entry, tag_def.values);
                        attrs.set(tag_def.name, value);
                    }
                }
            }
        }

        Some(attrs)
    }
}

/// Parse Pentax LensInfo sub-IFD (tag 0x0207).
fn parse_lens_info(data: &[u8], byte_order: ByteOrder, offset: u32) -> Option<Attrs> {
    let entries = super::parse_ifd_entries(data, byte_order, offset)?;

    let mut attrs = Attrs::new();

    for entry in entries {
        if let Some(tag_def) = pentax::PENTAX_LENSINFO.get(&entry.tag) {
            let value = format_value(&entry, tag_def.values);
            attrs.set(tag_def.name, value);
        }
    }

    Some(attrs)
}

/// Parse Pentax AFInfo sub-IFD (tag 0x0215).
fn parse_af_info(data: &[u8], byte_order: ByteOrder, offset: u32) -> Option<Attrs> {
    let entries = super::parse_ifd_entries(data, byte_order, offset)?;

    let mut attrs = Attrs::new();

    for entry in entries {
        if let Some(tag_def) = pentax::PENTAX_AFINFO.get(&entry.tag) {
            let value = format_value(&entry, tag_def.values);
            attrs.set(tag_def.name, value);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_vendor() {
        assert_eq!(PentaxParser.vendor(), Vendor::Pentax);
    }
}

//! Panasonic MakerNotes parser.
//!
//! Panasonic MakerNotes structure:
//! - Header: "Panasonic\0\0\0" (12 bytes)
//! - IFD starts immediately after header
//! - Little-endian byte order
//!
//! Known tags:
//! - 0x0001: ImageQuality
//! - 0x0002: FirmwareVersion
//! - 0x0003: WhiteBalance
//! - 0x0007: FocusMode
//! - 0x000F: AFAreaMode
//! - 0x001A: ImageStabilization
//! - 0x001C: MacroMode
//! - 0x001F: ShootingMode
//! - 0x0020: Audio
//! - 0x0023: WhiteBalanceBias
//! - 0x0024: FlashBias
//! - 0x0025: InternalSerialNumber
//! - 0x0026: PanasonicExifVersion
//! - 0x0028: ColorEffect
//! - 0x0029: TimeSincePowerOn
//! - 0x002A: BurstMode
//! - 0x002B: SequenceNumber
//! - 0x002C: ContrastMode
//! - 0x002D: NoiseReduction
//! - 0x002E: SelfTimer
//! - 0x0030: Rotation
//! - 0x0031: AFAssistLamp
//! - 0x0032: ColorMode
//! - 0x0033: BabyAge
//! - 0x0034: OpticalZoomMode
//! - 0x0035: ConversionLens
//! - 0x0036: TravelDay
//! - 0x0039: Contrast
//! - 0x003A: WorldTimeLocation
//! - 0x003B: TextStamp
//! - 0x003C: ProgramISO
//! - 0x003D: AdvancedSceneType
//! - 0x003E: TextStamp2
//! - 0x003F: FacesDetected
//! - 0x0040: Saturation
//! - 0x0041: Sharpness
//! - 0x0042: FilmMode
//! - 0x0044: ColorTempKelvin
//! - 0x0045: BracketSettings
//! - 0x0046: WBShiftAB
//! - 0x0047: WBShiftGM
//! - 0x0048: FlashCurtain
//! - 0x0049: LongExposureNoiseReduction
//! - 0x004B: PanasonicImageWidth
//! - 0x004C: PanasonicImageHeight
//! - 0x004D: AFPointPosition
//! - 0x004E: FaceDetInfo (sub-IFD)
//! - 0x0051: LensType
//! - 0x0052: LensSerialNumber
//! - 0x0053: AccessoryType
//! - 0x0054: AccessorySerialNumber
//! - 0x0059: Transform
//! - 0x005D: IntelligentExposure
//! - 0x0061: FaceRecInfo
//! - 0x0062: FlashWarning
//! - 0x0063: RecognizedFaceFlags
//! - 0x0065: Title
//! - 0x0066: BabyName
//! - 0x0067: Location
//! - 0x0069: Country
//! - 0x006B: State
//! - 0x006D: City
//! - 0x006F: Landmark
//! - 0x0070: IntelligentResolution
//! - 0x0077: BurstSpeed
//! - 0x0079: IntelligentD-Range
//! - 0x007C: ClearRetouch
//! - 0x0080: City2
//! - 0x0086: PhotoStyle
//! - 0x0089: ShadingCompensation
//! - 0x008A: AccelerometerZ
//! - 0x008B: AccelerometerX
//! - 0x008C: AccelerometerY
//! - 0x008D: CameraOrientation
//! - 0x008E: RollAngle
//! - 0x008F: PitchAngle
//! - 0x0090: SweepPanoramaDirection
//! - 0x0091: PanoramaSequenceNumber
//! - 0x0093: ClearRetouchValue
//! - 0x0096: HDRShot
//! - 0x009C: TouchAE
//! - 0x009D: HighlightShadow
//! - 0x00A3: RedEyeRemoval
//! - 0x00AB: VideoBurstMode
//! - 0x00AF: DiffCorrection
//! - 0x00B3: FocusBracket
//! - 0x00B4: LongExposureNRUsed
//! - 0x00B7: PostFocusMerge
//! - 0x00BD: FilterEffect
//! - 0x8000: MakerNoteVersion
//! - 0x8001: SceneMode
//! - 0x8004: WBRedLevel
//! - 0x8005: WBGreenLevel
//! - 0x8006: WBBlueLevel
//! - 0x8007: FlashFired
//! - 0x8008: TextStamp3
//! - 0x8009: TextStamp4
//! - 0x8010: BabyAge2

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::{ByteOrder, IfdReader};
use exiftool_tags::generated::panasonic;

/// Panasonic MakerNotes parser.
pub struct PanasonicParser;

/// Header magic for Panasonic MakerNotes.
const PANASONIC_HEADER: &[u8] = b"Panasonic";

impl VendorParser for PanasonicParser {
    fn vendor(&self) -> Vendor {
        Vendor::Panasonic
    }

    fn parse(&self, data: &[u8], _parent_byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 12 {
            return None;
        }

        // Check header
        let (ifd_data, header_len) = if data.starts_with(PANASONIC_HEADER) {
            (&data[12..], 12)
        } else {
            // Try without header
            (data, 0)
        };

        let _ = header_len;

        // Panasonic MakerNotes are always little-endian
        let byte_order = ByteOrder::LittleEndian;

        let reader = IfdReader::new(ifd_data, byte_order, 0);
        let (entries, _) = reader.read_ifd(0).ok()?;

        let mut attrs = Attrs::new();

        for entry in entries {
            match entry.tag {
                0x004E => {
                    // FaceDetInfo sub-IFD
                    if let Some(sub_attrs) = parse_face_detect(&entry.value.as_bytes()?, byte_order) {
                        attrs.set("FaceDetect", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                _ => {
                    // Main tag - lookup in PANASONIC_MAIN
                    if let Some(tag_def) = panasonic::PANASONIC_MAIN.get(&entry.tag) {
                        let value = format_value(&entry, tag_def.values);
                        attrs.set(tag_def.name, value);
                    }
                }
            }
        }

        Some(attrs)
    }
}

/// Parse Panasonic FaceDetect info (tag 0x004E).
fn parse_face_detect(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();

    // Face count at start
    if data.len() >= 2 {
        let face_count = read_u16(data, 0, byte_order);
        attrs.set("FaceCount", AttrValue::UInt(face_count as u32));
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
        assert_eq!(PanasonicParser.vendor(), Vendor::Panasonic);
    }
}

//! Apple MakerNotes parser.
//!
//! Apple MakerNotes structure:
//! - Header: "Apple iOS\0" (10 bytes) followed by version
//! - IFD starts after header
//! - Uses parent byte order (typically big-endian for iOS)
//!
//! Known tags:
//! - 0x0001: MakerNoteVersion
//! - 0x0002: AEStable
//! - 0x0003: AETarget
//! - 0x0004: AEAverage
//! - 0x0005: AFStable
//! - 0x0006: AccelerationVector
//! - 0x0007: HDRImageType
//! - 0x0008: BurstUUID
//! - 0x000A: TargetExposureDuration
//! - 0x000B: FocusDistanceRange
//! - 0x000C: FocusRange
//! - 0x000D: AFPerformance
//! - 0x000E: HDRGain
//! - 0x000F: SISMethod
//! - 0x0010: SignalToNoiseRatioType
//! - 0x0011: SignalToNoiseRatio
//! - 0x0013: PhotoIdentifier
//! - 0x0014: ImageCaptureRequestID
//! - 0x0015: FocusPosition
//! - 0x0016: HDRHeadroom
//! - 0x0017: SemanticRendering
//! - 0x0019: GainControl
//! - 0x001A: SemanticStyle
//! - 0x001B: SemanticVideoFlags
//! - 0x001D: VirtualLen
//! - 0x001F: SceneFlags
//! - 0x0020: SignalToNoiseRatioMeasured
//! - 0x0021: PhotoZoomFactor
//! - 0x0023: ContentIdentifier
//! - 0x0025: ImageCaptureType
//! - 0x0026: ImageUniquID
//! - 0x0027: LivePhotoVideoIndex
//! - 0x002A: ImageProcessingFlags
//! - 0x002B: QualityHint
//! - 0x002C: LuminanceNoiseAmplitude
//! - 0x002E: SpatialOverCaptureGroupUUID
//! - 0x002F: ImageGroupUUID
//! - 0x0030: ObjectAreas
//! - 0x0031: MediaGroupUUID
//! - 0x0033: CaptureMode
//! - 0x0036: PhotoOffsetTime
//! - 0x0038: FrontFacingCamera

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::{ByteOrder, IfdReader};
use exiftool_tags::generated::apple;

/// Apple MakerNotes parser.
pub struct AppleParser;

/// Header magic for Apple MakerNotes.
const APPLE_HEADER: &[u8] = b"Apple iOS";

impl VendorParser for AppleParser {
    fn vendor(&self) -> Vendor {
        Vendor::Apple
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 14 {
            return None;
        }

        // Check header
        let (ifd_data, byte_order) = if data.starts_with(APPLE_HEADER) {
            // Skip "Apple iOS\0" (10 bytes) + version (4 bytes)
            (&data[14..], parent_byte_order)
        } else {
            // No header, try as direct IFD
            (data, parent_byte_order)
        };

        let reader = IfdReader::new(ifd_data, byte_order, 0);
        let (entries, _) = reader.read_ifd(0).ok()?;

        let mut attrs = Attrs::new();

        for entry in entries {
            // Main tag - lookup in APPLE_MAIN
            if let Some(tag_def) = apple::APPLE_MAIN.get(&entry.tag) {
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
        assert_eq!(AppleParser.vendor(), Vendor::Apple);
    }
}

//! Nikon MakerNotes parser.
//!
//! Nikon MakerNotes structure:
//! - Type 1: Old format without header (Coolpix 880, etc.)
//! - Type 2: "Nikon\0\x01\0" header (DSLR, mirrorless)
//! - Type 3: "Nikon\0\x02\x10" header with embedded TIFF
//!
//! Known sub-IFD tags:
//! - 0x0001: MakerNoteVersion
//! - 0x0002: ISO
//! - 0x0004: ColorMode
//! - 0x0005: ImageQuality
//! - 0x0006: WhiteBalance
//! - 0x0007: Focus
//! - 0x0008: FlashSetting
//! - 0x0009: FlashType
//! - 0x000B: WhiteBalanceFine
//! - 0x000C: WBRBLevels
//! - 0x000D: ProgramShift
//! - 0x000E: ExposureDiff
//! - 0x000F: ISOSelection
//! - 0x0011: PreviewIFD (sub-IFD)
//! - 0x0012: FlashExposureComp
//! - 0x0013: ISOSetting
//! - 0x0016: ImageBoundary
//! - 0x0017: ExternalFlashExposureComp
//! - 0x0018: FlashExposureBracketValue
//! - 0x0019: ExposureBracketValue
//! - 0x001A: ImageProcessing
//! - 0x001B: CropHiSpeed
//! - 0x001C: ExposureTuning
//! - 0x001D: SerialNumber
//! - 0x001E: ColorSpace
//! - 0x001F: VRInfo
//! - 0x0020: ImageAuthentication
//! - 0x0021: FaceDetect
//! - 0x0022: ActiveD-Lighting
//! - 0x0023: PictureControl (sub-IFD)
//! - 0x0024: WorldTime
//! - 0x0025: ISOInfo
//! - 0x002A: VignetteControl
//! - 0x002B: DistortInfo
//! - 0x002C: HDRInfo
//! - 0x0035: LocationInfo
//! - 0x0037: BarometerInfo
//! - 0x0039: AFInfo2 (sub-IFD)
//! - 0x003D: FileInfo
//! - 0x0083: LensType
//! - 0x0084: Lens
//! - 0x0085: ManualFocusDistance
//! - 0x0086: DigitalZoom
//! - 0x0087: FlashMode
//! - 0x0088: AFInfo
//! - 0x0089: ShootingMode
//! - 0x008B: LensFStops
//! - 0x008C: ContrastCurve
//! - 0x008D: ColorHue
//! - 0x008F: SceneMode
//! - 0x0090: LightSource
//! - 0x0091: ShotInfo (sub-IFD)
//! - 0x0092: HueAdjustment
//! - 0x0093: NEFCompression
//! - 0x0094: Saturation
//! - 0x0095: NoiseReduction
//! - 0x0096: LinearizationTable
//! - 0x0097: ColorBalance
//! - 0x0098: LensData (sub-IFD)
//! - 0x0099: RawImageCenter
//! - 0x009A: SensorPixelSize
//! - 0x00A0: SerialNumber2
//! - 0x00A2: ImageDataSize
//! - 0x00A5: ImageCount
//! - 0x00A6: DeletedImageCount
//! - 0x00A7: ShutterCount
//! - 0x00A8: FlashInfo (sub-IFD)
//! - 0x00A9: ImageOptimization
//! - 0x00AB: VariProgram
//! - 0x00AC: ImageStabilization
//! - 0x00AD: AFResponse
//! - 0x00B0: MultiExposure
//! - 0x00B1: HighISONoiseReduction
//! - 0x00B6: PowerUpTime
//! - 0x00B7: AFInfo2
//! - 0x00B8: FileInfo
//! - 0x00B9: AFTune
//! - 0x00BB: RetouchInfo
//! - 0x00BD: PictureControlData
//! - 0x0E00: PrintIM

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::{ByteOrder, IfdReader};
use exiftool_tags::generated::nikon;

/// Nikon MakerNotes parser.
pub struct NikonParser;

/// Header magic for Nikon Type 3 MakerNotes.
const NIKON_HEADER: &[u8] = b"Nikon\x00";

impl VendorParser for NikonParser {
    fn vendor(&self) -> Vendor {
        Vendor::Nikon
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 8 {
            return None;
        }

        // Detect header type
        let (ifd_data, byte_order, base_offset) = if data.starts_with(NIKON_HEADER) {
            // Type 2/3 with header
            let header_type = data.get(6)?;
            
            match *header_type {
                0x01 => {
                    // Type 2: "Nikon\0\x01\0" - simple header, use parent byte order
                    (&data[10..], parent_byte_order, 10)
                }
                0x02 => {
                    // Type 3: "Nikon\0\x02\x10" - embedded TIFF with its own byte order
                    if data.len() < 18 {
                        return None;
                    }
                    let tiff_header = &data[10..];
                    let byte_order = if tiff_header.starts_with(b"II") {
                        ByteOrder::LittleEndian
                    } else if tiff_header.starts_with(b"MM") {
                        ByteOrder::BigEndian
                    } else {
                        return None;
                    };
                    // IFD offset from TIFF header
                    let ifd_offset = match byte_order {
                        ByteOrder::LittleEndian => u32::from_le_bytes([tiff_header[4], tiff_header[5], tiff_header[6], tiff_header[7]]),
                        ByteOrder::BigEndian => u32::from_be_bytes([tiff_header[4], tiff_header[5], tiff_header[6], tiff_header[7]]),
                    } as usize;
                    (tiff_header, byte_order, 10 + ifd_offset)
                }
                _ => {
                    // Unknown type, try as Type 2
                    (&data[10..], parent_byte_order, 10)
                }
            }
        } else {
            // Type 1: No header, starts directly with IFD
            (data, parent_byte_order, 0)
        };

        let _ = base_offset; // Used for offset calculations if needed
        
        let reader = IfdReader::new(ifd_data, byte_order, 0);
        let (entries, _) = reader.read_ifd(0).ok()?;

        let mut attrs = Attrs::new();

        for entry in entries {
            match entry.tag {
                0x0011 => {
                    // PreviewIFD - sub-IFD containing preview image offsets
                    // Note: offsets in PreviewIFD are relative to TIFF base (file start)
                    if let Some(offset) = entry.value.as_u32() {
                        if let Ok((preview_entries, _)) = reader.read_ifd(offset) {
                            // Extract preview offset/length from sub-IFD
                            let mut preview_offset: Option<u32> = None;
                            let mut preview_length: Option<u32> = None;
                            
                            for pe in &preview_entries {
                                match pe.tag {
                                    0x0201 => preview_offset = pe.value.as_u32(), // PreviewImageStart
                                    0x0202 => preview_length = pe.value.as_u32(), // PreviewImageLength  
                                    _ => {}
                                }
                            }
                            
                            // Store as MakerNotes attrs for extraction by TiffParser
                            if let (Some(off), Some(len)) = (preview_offset, preview_length) {
                                attrs.set("PreviewImageStart", AttrValue::UInt(off));
                                attrs.set("PreviewImageLength", AttrValue::UInt(len));
                            }
                        }
                    }
                }
                0x0025 => {
                    // ISOInfo
                    if let Some(sub_attrs) = parse_iso_info(entry.value.as_bytes()?, byte_order) {
                        attrs.set("ISOInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x002B => {
                    // DistortInfo
                    if let Some(sub_attrs) = parse_distort_info(entry.value.as_bytes()?, byte_order) {
                        attrs.set("DistortInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x002C => {
                    // HDRInfo
                    if let Some(sub_attrs) = parse_hdr_info(entry.value.as_bytes()?, byte_order) {
                        attrs.set("HDRInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0035 => {
                    // LocationInfo
                    if let Some(sub_attrs) = parse_location_info(entry.value.as_bytes()?, byte_order) {
                        attrs.set("LocationInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0037 => {
                    // BarometerInfo  
                    if let Some(sub_attrs) = parse_barometer_info(entry.value.as_bytes()?, byte_order) {
                        attrs.set("BarometerInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0039 | 0x00B7 => {
                    // AFInfo2
                    if let Some(sub_attrs) = parse_af_info2(entry.value.as_bytes()?, byte_order) {
                        attrs.set("AFInfo2", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0088 => {
                    // AFInfo (old format)
                    if let Some(sub_attrs) = parse_af_info(entry.value.as_bytes()?, byte_order) {
                        attrs.set("AFInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x00A8 => {
                    // FlashInfo
                    if let Some(sub_attrs) = parse_flash_info(entry.value.as_bytes()?, byte_order) {
                        attrs.set("FlashInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x00B9 => {
                    // AFTune
                    if let Some(sub_attrs) = parse_af_tune(entry.value.as_bytes()?, byte_order) {
                        attrs.set("AFTune", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                _ => {
                    // Main tag - lookup in NIKON_MAIN
                    if let Some(tag_def) = nikon::NIKON_MAIN.get(&entry.tag) {
                        let value = format_value(&entry, tag_def.values);
                        attrs.set(tag_def.name, value);
                    }
                }
            }
        }

        Some(attrs)
    }
}

/// Parse Nikon ISOInfo (tag 0x0025).
fn parse_iso_info(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 2;

    for i in 0..count.min(10) {
        if let Some(tag_def) = nikon::NIKON_ISOINFO.get(&(i as u16)) {
            let value = read_u16(data, i * 2, byte_order);
            let attr_value = if let Some(values) = tag_def.values {
                values
                    .iter()
                    .find(|(k, _)| *k == value as i64)
                    .map(|(_, v)| AttrValue::Str(v.to_string()))
                    .unwrap_or(AttrValue::UInt(value as u32))
            } else {
                AttrValue::UInt(value as u32)
            };
            attrs.set(tag_def.name, attr_value);
        }
    }

    Some(attrs)
}

/// Parse Nikon DistortInfo (tag 0x002B).
fn parse_distort_info(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 2;

    for i in 0..count.min(10) {
        if let Some(tag_def) = nikon::NIKON_DISTORTINFO.get(&(i as u16)) {
            let value = read_u16(data, i * 2, byte_order);
            let attr_value = if let Some(values) = tag_def.values {
                values
                    .iter()
                    .find(|(k, _)| *k == value as i64)
                    .map(|(_, v)| AttrValue::Str(v.to_string()))
                    .unwrap_or(AttrValue::UInt(value as u32))
            } else {
                AttrValue::UInt(value as u32)
            };
            attrs.set(tag_def.name, attr_value);
        }
    }

    Some(attrs)
}

/// Parse Nikon HDRInfo (tag 0x002C).
fn parse_hdr_info(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 2;

    for i in 0..count.min(10) {
        if let Some(tag_def) = nikon::NIKON_HDRINFO.get(&(i as u16)) {
            let value = read_u16(data, i * 2, byte_order);
            let attr_value = if let Some(values) = tag_def.values {
                values
                    .iter()
                    .find(|(k, _)| *k == value as i64)
                    .map(|(_, v)| AttrValue::Str(v.to_string()))
                    .unwrap_or(AttrValue::UInt(value as u32))
            } else {
                AttrValue::UInt(value as u32)
            };
            attrs.set(tag_def.name, attr_value);
        }
    }

    Some(attrs)
}

/// Parse Nikon LocationInfo (tag 0x0035).
fn parse_location_info(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 2;

    for i in 0..count.min(10) {
        if let Some(tag_def) = nikon::NIKON_LOCATIONINFO.get(&(i as u16)) {
            let value = read_u16(data, i * 2, byte_order);
            let attr_value = if let Some(values) = tag_def.values {
                values
                    .iter()
                    .find(|(k, _)| *k == value as i64)
                    .map(|(_, v)| AttrValue::Str(v.to_string()))
                    .unwrap_or(AttrValue::UInt(value as u32))
            } else {
                AttrValue::UInt(value as u32)
            };
            attrs.set(tag_def.name, attr_value);
        }
    }

    Some(attrs)
}

/// Parse Nikon BarometerInfo (tag 0x0037).
fn parse_barometer_info(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 4;

    for i in 0..count.min(5) {
        if let Some(tag_def) = nikon::NIKON_BAROMETERINFO.get(&(i as u16)) {
            let value = read_u32(data, i * 4, byte_order);
            attrs.set(tag_def.name, AttrValue::UInt(value));
        }
    }

    Some(attrs)
}

/// Parse Nikon AFInfo (old format, tag 0x0088).
fn parse_af_info(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 2;

    for i in 0..count.min(10) {
        if let Some(tag_def) = nikon::NIKON_AFINFO.get(&(i as u16)) {
            let value = read_u16(data, i * 2, byte_order);
            let attr_value = if let Some(values) = tag_def.values {
                values
                    .iter()
                    .find(|(k, _)| *k == value as i64)
                    .map(|(_, v)| AttrValue::Str(v.to_string()))
                    .unwrap_or(AttrValue::UInt(value as u32))
            } else {
                AttrValue::UInt(value as u32)
            };
            attrs.set(tag_def.name, attr_value);
        }
    }

    Some(attrs)
}

/// Parse Nikon AFInfo2 (tag 0x0039 or 0x00B7).
fn parse_af_info2(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();

    // Version check at start
    let version = read_u16(data, 0, byte_order);
    
    // Use appropriate table based on version
    let count = data.len() / 2;
    for i in 0..count.min(20) {
        if let Some(tag_def) = nikon::NIKON_AFINFO2V0100.get(&(i as u16)) {
            let value = read_u16(data, i * 2, byte_order);
            let attr_value = if let Some(values) = tag_def.values {
                values
                    .iter()
                    .find(|(k, _)| *k == value as i64)
                    .map(|(_, v)| AttrValue::Str(v.to_string()))
                    .unwrap_or(AttrValue::UInt(value as u32))
            } else {
                AttrValue::UInt(value as u32)
            };
            attrs.set(tag_def.name, attr_value);
        }
    }

    attrs.set("AFInfo2Version", AttrValue::UInt(version as u32));
    Some(attrs)
}

/// Parse Nikon FlashInfo (tag 0x00A8).
fn parse_flash_info(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 2;

    for i in 0..count.min(30) {
        if let Some(tag_def) = nikon::NIKON_FLASHINFO0100.get(&(i as u16)) {
            let value = read_u16(data, i * 2, byte_order);
            let attr_value = if let Some(values) = tag_def.values {
                values
                    .iter()
                    .find(|(k, _)| *k == value as i64)
                    .map(|(_, v)| AttrValue::Str(v.to_string()))
                    .unwrap_or(AttrValue::UInt(value as u32))
            } else {
                AttrValue::UInt(value as u32)
            };
            attrs.set(tag_def.name, attr_value);
        }
    }

    Some(attrs)
}

/// Parse Nikon AFTune (tag 0x00B9).
fn parse_af_tune(data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
    if data.len() < 4 {
        return None;
    }

    let mut attrs = Attrs::new();
    let count = data.len() / 2;

    for i in 0..count.min(10) {
        if let Some(tag_def) = nikon::NIKON_AFTUNE.get(&(i as u16)) {
            let value = read_u16(data, i * 2, byte_order);
            let attr_value = if let Some(values) = tag_def.values {
                values
                    .iter()
                    .find(|(k, _)| *k == value as i64)
                    .map(|(_, v)| AttrValue::Str(v.to_string()))
                    .unwrap_or(AttrValue::UInt(value as u32))
            } else {
                AttrValue::UInt(value as u32)
            };
            attrs.set(tag_def.name, attr_value);
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

/// Read u32 from byte slice with byte order.
#[inline]
fn read_u32(data: &[u8], offset: usize, byte_order: ByteOrder) -> u32 {
    if offset + 4 > data.len() {
        return 0;
    }
    match byte_order {
        ByteOrder::LittleEndian => u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]),
        ByteOrder::BigEndian => u32::from_be_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_vendor() {
        assert_eq!(NikonParser.vendor(), Vendor::Nikon);
    }
}

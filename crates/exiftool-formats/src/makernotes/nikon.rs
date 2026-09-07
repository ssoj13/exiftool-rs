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
use exiftool_core::{ByteOrder, IfdReader, RawValue};
use exiftool_tags::generated::nikon;

#[path = "nikon_decrypt.rs"]
mod nikon_decrypt;

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
        let (ifd_data, byte_order, ifd_pos) = if data.starts_with(NIKON_HEADER) {
            let header_type = data.get(6)?;
            match *header_type {
                0x01 => {
                    // Type 2: "Nikon\0\x01\0" — IFD starts after the 10-byte header.
                    (&data[10..], parent_byte_order, 0u64)
                }
                0x02 => {
                    // Type 3: embedded TIFF at offset 10; IFD0 from that TIFF header.
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
                    let ifd_offset = match byte_order {
                        ByteOrder::LittleEndian => u32::from_le_bytes([
                            tiff_header[4],
                            tiff_header[5],
                            tiff_header[6],
                            tiff_header[7],
                        ]),
                        ByteOrder::BigEndian => u32::from_be_bytes([
                            tiff_header[4],
                            tiff_header[5],
                            tiff_header[6],
                            tiff_header[7],
                        ]),
                    };
                    (tiff_header, byte_order, u64::from(ifd_offset))
                }
                _ => (&data[10..], parent_byte_order, 0u64),
            }
        } else {
            (data, parent_byte_order, 0u64)
        };

        let reader = IfdReader::new(ifd_data, byte_order);
        let mut errors = Vec::new();
        let (entries, _) = reader.read_ifd_lenient(ifd_pos, &mut errors).ok()?;
        if entries.is_empty() {
            return None;
        }

        let mut attrs = Attrs::new();

        let mut serial_str = String::new();
        let mut shutter_count = 0u32;
        for entry in &entries {
            match entry.tag {
                0x00A0 => {
                    if let RawValue::String(s) = &entry.value {
                        serial_str = s.clone();
                    }
                }
                0x00A7 => {
                    if let Some(n) = entry.value.as_u32() {
                        shutter_count = n;
                    }
                }
                _ => {}
            }
        }
        let serial_key = nikon_decrypt::serial_key(&serial_str, None);

        for entry in entries {
            match entry.tag {
                0x0011 => {
                    // PreviewIFD - sub-IFD containing preview image offsets
                    // Note: offsets in PreviewIFD are relative to TIFF base (file start)
                    if let Some(offset) = entry.value.as_u32() {
                        if let Ok((preview_entries, _)) = reader.read_ifd(offset as u64) {
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
                    if let Some(sub_attrs) =
                        parse_iso_info(entry.value.as_bytes().unwrap_or(&[]), byte_order)
                    {
                        attrs.set("ISOInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x002B => {
                    // DistortInfo
                    if let Some(sub_attrs) =
                        parse_distort_info(entry.value.as_bytes().unwrap_or(&[]), byte_order)
                    {
                        attrs.set("DistortInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x002C => {
                    // HDRInfo
                    if let Some(sub_attrs) =
                        parse_hdr_info(entry.value.as_bytes().unwrap_or(&[]), byte_order)
                    {
                        attrs.set("HDRInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0035 => {
                    // LocationInfo
                    if let Some(sub_attrs) =
                        parse_location_info(entry.value.as_bytes().unwrap_or(&[]), byte_order)
                    {
                        attrs.set("LocationInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0037 => {
                    // BarometerInfo
                    if let Some(sub_attrs) =
                        parse_barometer_info(entry.value.as_bytes().unwrap_or(&[]), byte_order)
                    {
                        attrs.set("BarometerInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0039 | 0x00B7 => {
                    // AFInfo2
                    if let Some(sub_attrs) =
                        parse_af_info2(entry.value.as_bytes().unwrap_or(&[]), byte_order)
                    {
                        attrs.set("AFInfo2", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0088 => {
                    // AFInfo (old format)
                    if let Some(sub_attrs) =
                        parse_af_info(entry.value.as_bytes().unwrap_or(&[]), byte_order)
                    {
                        attrs.set("AFInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x00A8 => {
                    // FlashInfo
                    if let Some(sub_attrs) =
                        parse_flash_info(entry.value.as_bytes().unwrap_or(&[]), byte_order)
                    {
                        attrs.set("FlashInfo", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x00B9 => {
                    // AFTune
                    if let Some(sub_attrs) =
                        parse_af_tune(entry.value.as_bytes().unwrap_or(&[]), byte_order)
                    {
                        attrs.set("AFTune", AttrValue::Group(Box::new(sub_attrs)));
                    }
                }
                0x0091 => {
                    parse_shot_info(
                        entry.value.as_bytes().unwrap_or(&[]),
                        serial_key,
                        shutter_count,
                        &mut attrs,
                    );
                }
                0x0097 => {
                    parse_color_balance(
                        entry.value.as_bytes().unwrap_or(&[]),
                        byte_order,
                        serial_key,
                        shutter_count,
                        &mut attrs,
                    );
                }
                0x0098 => {
                    parse_lens_data(
                        entry.value.as_bytes().unwrap_or(&[]),
                        serial_key,
                        shutter_count,
                        &mut attrs,
                    );
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

/// LensData: 0100/0101 plaintext; 02xx+ decrypted from offset 4 (`ProcessNikonEncrypted`).
fn parse_lens_data(data: &[u8], serial: u32, shutter: u32, attrs: &mut Attrs) {
    if data.len() < 4 {
        return;
    }
    let ver = String::from_utf8_lossy(&data[..4]).to_string();
    attrs.set("LensDataVersion", AttrValue::Str(ver.clone()));
    let decrypted;
    let payload: &[u8] = if ver.starts_with("02") || ver.starts_with("04") || ver.starts_with("08")
    {
        decrypted = nikon_decrypt::decrypt(data, 4, serial, shutter);
        &decrypted
    } else {
        data
    };
    let table_01 = ver.starts_with("0101") || ver.starts_with("02") || ver.starts_with("040");
    if table_01 {
        apply_lens_data_01(payload, attrs);
    } else if ver.starts_with("0100") {
        apply_lens_data_00(payload, attrs);
    }
}

fn apply_lens_data_01(data: &[u8], attrs: &mut Attrs) {
    if data.len() > 0x0b {
        attrs.set("LensIDNumber", AttrValue::UInt(u32::from(data[0x0b])));
    }
    if data.len() > 0x0c {
        attrs.set(
            "LensFStops",
            AttrValue::Str(format!("{:.2}", f64::from(data[0x0c]) / 12.0)),
        );
    }
    if data.len() > 0x0d {
        attrs.set("MinFocalLength", AttrValue::Str(nikon_focal_mm(data[0x0d])));
    }
    if data.len() > 0x0e {
        attrs.set("MaxFocalLength", AttrValue::Str(nikon_focal_mm(data[0x0e])));
    }
    if data.len() > 0x0f {
        attrs.set(
            "MaxApertureAtMinFocal",
            AttrValue::Str(nikon_aperture(data[0x0f])),
        );
    }
    if data.len() > 0x10 {
        attrs.set(
            "MaxApertureAtMaxFocal",
            AttrValue::Str(nikon_aperture(data[0x10])),
        );
    }
}

fn apply_lens_data_00(data: &[u8], attrs: &mut Attrs) {
    if data.len() > 0x06 {
        attrs.set("LensIDNumber", AttrValue::UInt(u32::from(data[0x06])));
    }
    if data.len() > 0x08 {
        attrs.set("MinFocalLength", AttrValue::Str(nikon_focal_mm(data[0x08])));
    }
    if data.len() > 0x09 {
        attrs.set("MaxFocalLength", AttrValue::Str(nikon_focal_mm(data[0x09])));
    }
}

fn parse_color_balance(
    data: &[u8],
    byte_order: ByteOrder,
    serial: u32,
    shutter: u32,
    attrs: &mut Attrs,
) {
    if data.len() < 4 {
        return;
    }
    let ver = String::from_utf8_lossy(&data[..4]).to_string();
    attrs.set("ColorBalanceVersion", AttrValue::Str(ver.clone()));
    let (decrypt_start, dir_off, levels_name) = color_balance_layout(&ver);
    let decrypted;
    let payload: &[u8] = if let Some(start) = decrypt_start {
        decrypted = nikon_decrypt::decrypt(data, start, serial, shutter);
        &decrypted
    } else {
        data
    };
    if let Some(levels) = read_u16x4(payload, dir_off, byte_order) {
        attrs.set(
            levels_name,
            AttrValue::Str(format!(
                "{} {} {} {}",
                levels[0], levels[1], levels[2], levels[3]
            )),
        );
    }
}

fn color_balance_layout(ver: &str) -> (Option<usize>, usize, &'static str) {
    if ver.starts_with("0100") {
        return (None, 72, "WB_RBGGLevels");
    }
    if ver.starts_with("0102") {
        return (None, 10, "WB_RGGBLevels");
    }
    if ver.starts_with("0103") {
        return (None, 20, "WB_RGBGLevels");
    }
    if ver.starts_with("0205") {
        return (Some(4), 4 + 14, "WB_RGGBLevels");
    }
    if ver.starts_with("0209") || ver.starts_with("0212") || ver.starts_with("0214") {
        return (Some(284), 284 + 10, "WB_GRBGLevels");
    }
    if ver.starts_with("0211") {
        return (Some(284), 284 + 16, "WB_GRBGLevels");
    }
    if ver.starts_with("0213") {
        return (Some(284), 284 + 10, "WB_RGGBLevels");
    }
    if ver.starts_with("0215") || ver.starts_with("0216") || ver.starts_with("0217") {
        return (Some(284), 284 + 4, "WB_GRBGLevels");
    }
    if ver.starts_with("02") {
        return (Some(284), 284 + 6, "WB_RGGBLevels");
    }
    (None, 0, "WB_RGGBLevels")
}

/// ShotInfo (`Nikon.pm` 0x0091): version dispatch, `DecryptStart => 4`,
/// per-model offsets, and `NIKON_OFFSETS` piecewise decrypt.
fn parse_shot_info(data: &[u8], serial: u32, shutter: u32, attrs: &mut Attrs) {
    if data.len() < 4 {
        return;
    }
    let ver = String::from_utf8_lossy(&data[..4]).to_string();
    attrs.set("ShotInfoVersion", AttrValue::Str(ver.clone()));
    let count = data.len();
    let crypt = shot_crypt(&ver, count);
    let decrypted;
    let payload: &[u8] = match crypt {
        ShotCrypt::None => data,
        ShotCrypt::Full { .. } => {
            decrypted = nikon_decrypt::decrypt(data, 4, serial, shutter);
            &decrypted
        }
        ShotCrypt::Offsets { table, big_endian } => {
            let mut buf = data.to_vec();
            if let Some(n) = nikon_decrypt::prepare_nikon_offsets(
                &mut buf, 4, table, big_endian, serial, shutter,
            ) {
                attrs.set("NumberOffsets", AttrValue::UInt(n));
            }
            decrypted = buf;
            &decrypted
        }
    };
    let fw_len = if ver.starts_with("08") { 8 } else { 5 };
    if payload.len() >= 4 + fw_len {
        let fw = String::from_utf8_lossy(&payload[4..4 + fw_len])
            .trim_end_matches('\0')
            .trim()
            .to_string();
        if fw.len() >= 3 && fw.as_bytes()[0].is_ascii_digit() && fw.as_bytes().get(1) == Some(&b'.')
        {
            attrs.set("FirmwareVersion", AttrValue::Str(fw));
        }
    }
    let be = match crypt {
        ShotCrypt::Full { big_endian } | ShotCrypt::Offsets { big_endian, .. } => big_endian,
        ShotCrypt::None => true,
    };
    let order = if be {
        ByteOrder::BigEndian
    } else {
        ByteOrder::LittleEndian
    };
    if let Some(off) = shutter_count_offset(&ver, count) {
        if payload.len() >= off + 4 && attrs.get("ShutterCount").is_none() {
            attrs.set(
                "ShutterCount",
                AttrValue::UInt(read_u32(payload, off, order)),
            );
        }
    }
    if ver.starts_with("0209") && payload.len() > 586 {
        let on = payload[586] & 0x08 != 0;
        attrs.set(
            "VibrationReduction",
            AttrValue::Str(if on { "On".into() } else { "Off".into() }),
        );
    }
    if ver.starts_with("0204") {
        if payload.len() > 0x82 {
            let vr = payload[0x82];
            attrs.set(
                "VibrationReduction",
                AttrValue::Str(if vr == 0 { "Off".into() } else { "On".into() }),
            );
        }
        if payload.len() >= 0x6e && attrs.get("ShutterCount").is_none() {
            attrs.set(
                "ShutterCount",
                AttrValue::UInt(read_u32(payload, 0x6a, ByteOrder::BigEndian)),
            );
        }
    } else if ver.starts_with("0207") && payload.len() > 0x75 {
        let vr = payload[0x75];
        let label = match vr {
            0 => Some("Off"),
            1 => Some("On (1)"),
            2 => Some("On (2)"),
            3 => Some("On (3)"),
            _ => None,
        };
        if let Some(label) = label {
            attrs.set("VibrationReduction", AttrValue::Str(label.into()));
        }
    }
}

enum ShotCrypt {
    None,
    Full { big_endian: bool },
    Offsets { table: usize, big_endian: bool },
}

/// `Nikon.pm` 0x0091 Condition order (version + size).
fn shot_crypt(ver: &str, count: usize) -> ShotCrypt {
    if ver.starts_with("0209") {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0208") {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0213") {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0210") && count == 5399 {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0210") && (count == 5408 || count == 5412) {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0214") && count == 5409 {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0218") && (count == 5356 || count == 5388) {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0210") && count == 5291 {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0210") && count == 5303 {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0216") && count == 5311 {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0212") && count == 5312 {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0245") {
        return ShotCrypt::Offsets {
            table: 0x24,
            big_endian: false,
        };
    }
    if ver.starts_with("0242") {
        return ShotCrypt::Offsets {
            table: 0x0c,
            big_endian: false,
        };
    }
    if ver.starts_with("0222") {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0233") {
        return ShotCrypt::Offsets {
            table: 0x0c,
            big_endian: false,
        };
    }
    if ver.starts_with("0243") {
        return ShotCrypt::Offsets {
            table: 0x0c,
            big_endian: false,
        };
    }
    if ver.starts_with("0215") && count == 6745 {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0221") && count == 8902 {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0226") && count == 11587 {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0220") {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0223") {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0231") {
        return ShotCrypt::Full { big_endian: false };
    }
    if ver.starts_with("0238") || ver.starts_with("0239") {
        return ShotCrypt::Offsets {
            table: 0x0c,
            big_endian: false,
        };
    }
    if ver.starts_with("0246") {
        return ShotCrypt::Offsets {
            table: 0x24,
            big_endian: false,
        };
    }
    if ver.starts_with("0232") {
        return ShotCrypt::Full { big_endian: true };
    }
    if ver.starts_with("0809") || ver.starts_with("0810") || ver.starts_with("0811") {
        return ShotCrypt::Offsets {
            table: 0x24,
            big_endian: false,
        };
    }
    if ver.starts_with("0806") {
        return ShotCrypt::Offsets {
            table: 0x24,
            big_endian: false,
        };
    }
    if ver.starts_with("0805") {
        return ShotCrypt::Offsets {
            table: 0x24,
            big_endian: false,
        };
    }
    if matches!(
        &ver[..4.min(ver.len())],
        "0800" | "0801" | "0802" | "0803" | "0804" | "0807" | "0808"
    ) {
        return ShotCrypt::Offsets {
            table: 0x24,
            big_endian: false,
        };
    }
    if ver.starts_with("02") || ver.starts_with("08") {
        return ShotCrypt::Full { big_endian: true };
    }
    ShotCrypt::None
}

fn shutter_count_offset(ver: &str, count: usize) -> Option<usize> {
    if ver.starts_with("0209") {
        return Some(582);
    }
    if ver.starts_with("0208") {
        return Some(586);
    }
    if ver.starts_with("0213") {
        return Some(0x2d5);
    }
    if ver.starts_with("0210") && count == 5399 {
        return Some(0x276);
    }
    if ver.starts_with("0210") && (count == 5408 || count == 5412) {
        return Some(0x27d);
    }
    if ver.starts_with("0214") && count == 5409 {
        return Some(0x280);
    }
    if ver.starts_with("0218") && (count == 5356 || count == 5388) {
        return Some(0x242);
    }
    if ver.starts_with("0210") && count == 5291 {
        return Some(633);
    }
    if ver.starts_with("0210") && count == 5303 {
        return Some(644);
    }
    if ver.starts_with("0216") && count == 5311 {
        return Some(646);
    }
    if ver.starts_with("0212") && count == 5312 {
        return Some(0x287);
    }
    if ver.starts_with("0222") {
        return Some(0x5fb);
    }
    if ver.starts_with("0215") && count == 6745 {
        return Some(0x2d6);
    }
    if ver.starts_with("0221") && count == 8902 {
        return Some(0x321);
    }
    if ver.starts_with("0226") && count == 11587 {
        return Some(0xbd8);
    }
    if ver.starts_with("0220") {
        return Some(0x320);
    }
    None
}

fn read_u16x4(data: &[u8], off: usize, byte_order: ByteOrder) -> Option<[u16; 4]> {
    if data.len() < off + 8 {
        return None;
    }
    Some([
        read_u16(data, off, byte_order),
        read_u16(data, off + 2, byte_order),
        read_u16(data, off + 4, byte_order),
        read_u16(data, off + 6, byte_order),
    ])
}

fn nikon_focal_mm(val: u8) -> String {
    format!("{:.1} mm", 5.0 * 2f64.powf(f64::from(val) / 24.0))
}

fn nikon_aperture(val: u8) -> String {
    format!("{:.1}", 2f64.powf(f64::from(val) / 24.0))
}

/// Format IFD entry value with PrintConv lookup.
fn format_value(
    entry: &exiftool_core::IfdEntry,
    values_map: Option<&'static [(i64, &'static str)]>,
) -> AttrValue {
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
        ByteOrder::LittleEndian => u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]),
        ByteOrder::BigEndian => u32::from_be_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_vendor() {
        assert_eq!(NikonParser.vendor(), Vendor::Nikon);
    }

    #[test]
    fn parse_d70_makernotes_blob() {
        let path =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/Nikon.nef");
        if !path.exists() {
            return;
        }
        let data = std::fs::read(&path).unwrap();
        let mn = &data[1744..1744 + 4374];
        let attrs = NikonParser
            .parse(mn, ByteOrder::LittleEndian)
            .expect("nikon mn");
        assert_eq!(attrs.get_str("LensDataVersion"), Some("0101"));
        assert!(attrs.get_str("WB_RGBGLevels").is_some());
    }
}

//! MakerNotes field rewrite (ExifTool: existing tags only, no create/delete).
//!
//! FujiFilm rebuilds the IFD (offsets from MakerNotes start). Other IFD vendors
//! patch existing directory values in place so sub-IFD / preview offsets stay valid.
//! Nikon ShotInfo (`0x0091`) Full-crypt and `NIKON_OFFSETS` blobs are decrypted, patched, and re-encrypted.
//! ColorBalance (`0x0097`) levels and LensData (`0x0098`) `LensIDNumber` use the same keys.
//! DJI overlay includes FLOAT tags. GoPro GPMF patches same-size KLV leaves in place.
//! IFD overlay tag names follow each vendor parser table (Hasselblad is not Sony).
//! Olympus Equipment/CameraSettings/ImageProcessing/FocusInfo sub-IFDs overlay in place.
//! Canon CameraSettings and ShotInfo overlay existing int16 slots.

use crate::Metadata;
use exiftool_attrs::AttrValue;
use exiftool_core::{ByteOrder, ExifFormat, IfdEntry, RawValue, SRational, URational};
use exiftool_tags::generated::{
    apple, canon, dji, fujifilm, gopro, nikon, olympus, panasonic, pentax, samsung, sony,
};

const FUJI_MAGIC: &[u8] = b"FUJIFILM";
const NIKON_HEADER: &[u8] = b"Nikon\x00";

type TagLookup = fn(u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)>;

/// Rewrite a MakerNotes blob. Unknown layouts are returned unchanged.
pub fn rewrite_blob(data: &[u8], metadata: &Metadata) -> Vec<u8> {
    if data.starts_with(FUJI_MAGIC) {
        if let Some(out) = rewrite_fujifilm(data, metadata) {
            return out;
        }
        return data.to_vec();
    }
    if let Some(out) = rewrite_known(data, metadata) {
        return out;
    }
    data.to_vec()
}

fn lookup_panasonic(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    panasonic::PANASONIC_MAIN
        .get(&tag)
        .map(|d| (d.name, d.values))
}
fn lookup_sony(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    sony::SONY_MAIN.get(&tag).map(|d| (d.name, d.values))
}
fn lookup_olympus(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    olympus::OLYMPUS_MAIN.get(&tag).map(|d| (d.name, d.values))
}
fn lookup_nikon(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    nikon::NIKON_MAIN.get(&tag).map(|d| (d.name, d.values))
}
fn lookup_canon(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    canon::CANON_MAIN.get(&tag).map(|d| (d.name, d.values))
}
fn lookup_canon_camerasettings(
    tag: u16,
) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    canon::CANON_CAMERASETTINGS
        .get(&tag)
        .map(|d| (d.name, d.values))
}
fn lookup_canon_shotinfo(
    tag: u16,
) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    canon::CANON_SHOTINFO.get(&tag).map(|d| (d.name, d.values))
}
fn lookup_pentax(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    pentax::PENTAX_MAIN.get(&tag).map(|d| (d.name, d.values))
}
fn lookup_apple(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    apple::APPLE_MAIN.get(&tag).map(|d| (d.name, d.values))
}
fn lookup_samsung(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    samsung::SAMSUNG_MAIN.get(&tag).map(|d| (d.name, d.values))
}
fn lookup_minolta(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    sony::MINOLTA_MAIN.get(&tag).map(|d| (d.name, d.values))
}
fn lookup_casio_type2(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::casio::CASIO_TYPE2_TAGS, tag)
}
fn lookup_casio_type1(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::casio::CASIO_TYPE1_TAGS, tag)
}
fn lookup_sigma(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::sigma::SIGMA_TAGS, tag)
}
fn lookup_ricoh(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::ricoh::RICOH_TAGS, tag)
}
fn lookup_kodak(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::kodak::KODAK_TAGS, tag)
}
fn lookup_motorola(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::motorola::MOTOROLA_TAGS, tag)
}
fn lookup_xiaomi(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::xiaomi::XIAOMI_TAGS, tag)
}
fn lookup_oppo(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::oppo::OPPO_TAGS, tag)
}
fn lookup_oneplus(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::oneplus::ONEPLUS_TAGS, tag)
}
fn lookup_vivo(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::vivo::VIVO_TAGS, tag)
}
fn lookup_realme(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::realme::REALME_TAGS, tag)
}
fn lookup_huawei(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::huawei::HUAWEI_TAGS, tag)
}
fn lookup_google(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::google::GOOGLE_TAGS, tag)
}
fn lookup_phaseone(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::phaseone::PHASEONE_TAGS, tag)
}
fn lookup_leica(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    super::leica::LeicaParser::lookup_write(tag)
}
fn lookup_hasselblad(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    lookup_pairs(super::hasselblad::HASSELBLAD_TAGS, tag)
}
fn lookup_pairs(
    table: &'static [(u16, &'static str)],
    tag: u16,
) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    table
        .iter()
        .find(|(t, _)| *t == tag)
        .map(|(_, n)| (*n, None))
}
fn lookup_dji(tag: u16) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    dji::DJI_MAIN.get(&tag).map(|d| (d.name, d.values))
}

fn rewrite_known(data: &[u8], metadata: &Metadata) -> Option<Vec<u8>> {
    if data.starts_with(b"Panasonic") && data.len() >= 14 {
        return patch_ifd(
            data,
            12,
            ByteOrder::LittleEndian,
            lookup_panasonic,
            metadata,
            true,
        );
    }
    if data.starts_with(b"LEICA\0\0\0") && data.len() >= 10 {
        let order = detect_order(data, 8)?;
        return patch_ifd(data, 8, order, lookup_panasonic, metadata, true);
    }
    if data.starts_with(b"LEICA CAMERA AG") && data.len() >= 18 {
        let order = detect_order(data, 16)?;
        return patch_ifd(data, 16, order, lookup_leica, metadata, true);
    }
    if (data.starts_with(b"SONY DSC ") || data.starts_with(b"SONY CAM ")) && data.len() >= 14 {
        let order = detect_order(data, 12)?;
        return patch_ifd(data, 12, order, lookup_sony, metadata, true);
    }
    if data.starts_with(b"OLYMPUS\0") && data.len() >= 14 {
        let order = order_from_marker(&data[8..10])?;
        return patch_olympus(data, 12, order, metadata, false);
    }
    if data.starts_with(b"OM SYSTEM\0") && data.len() >= 18 {
        let order = order_from_marker(&data[10..12])?;
        return patch_olympus(data, 16, order, metadata, false);
    }
    if (data.starts_with(b"OLYMP\0") || data.starts_with(b"EPSON\0")) && data.len() >= 10 {
        let order = detect_order(data, 8)?;
        return patch_olympus(data, 8, order, metadata, true);
    }
    if data.starts_with(b"PENTAX \0") && data.len() >= 12 {
        let order = order_from_marker(&data[8..10])?;
        return patch_ifd(data, 10, order, lookup_pentax, metadata, false);
    }
    if data.starts_with(b"AOC\0") && data.len() >= 8 {
        let order = order_from_marker(&data[4..6]).or_else(|| detect_order(data, 6))?;
        return patch_ifd(data, 6, order, lookup_pentax, metadata, true);
    }
    if data.starts_with(b"Apple iOS\0") && data.len() >= 16 {
        let order = detect_order(data, 14)?;
        return patch_ifd(data, 14, order, lookup_apple, metadata, false);
    }
    if data.starts_with(b"QVC\0") || data.starts_with(b"DCI\0") {
        let order = detect_order(data, 6)?;
        return patch_ifd(data, 6, order, lookup_casio_type2, metadata, true);
    }
    if data.len() >= 10 && (data.starts_with(b"RICOH\0II") || data.starts_with(b"RICOH\0MM")) {
        let order = order_from_marker(&data[6..8])?;
        return patch_ifd(data, 8, order, lookup_pentax, metadata, false);
    }
    if data.starts_with(b"RICOH\0") && data.len() >= 10 {
        let order = detect_order(data, 8).or_else(|| detect_order(data, 6))?;
        let off = if ifd_plausible(data, 8, order) { 8 } else { 6 };
        return patch_ifd(data, off, order, lookup_ricoh, metadata, true);
    }
    if (data.starts_with(b"SIGMA\0\0\0") || data.starts_with(b"FOVEON\0\0")) && data.len() >= 12 {
        return patch_ifd(
            data,
            10,
            ByteOrder::LittleEndian,
            lookup_sigma,
            metadata,
            true,
        );
    }
    if (data.starts_with(b"MINOL\0") || data.starts_with(b"CAMER\0")) && data.len() >= 10 {
        let order = detect_order(data, 8)?;
        return patch_olympus(data, 8, order, metadata, true);
    }
    if data.starts_with(b"SONY PI\0") && data.len() >= 14 {
        let order = detect_order(data, 12)?;
        return patch_olympus(data, 12, order, metadata, true);
    }
    if data.starts_with(b"PREMI\0") && data.len() >= 10 {
        let order = detect_order(data, 8)?;
        return patch_olympus(data, 8, order, metadata, true);
    }
    if data.starts_with(b"GoPro\0") {
        return patch_gpmf(data, metadata);
    }
    if data.starts_with(NIKON_HEADER) && data.len() >= 18 {
        match data[6] {
            0x01 => {
                return patch_nikon(data, 8, ByteOrder::LittleEndian, metadata, true);
            }
            0x02 => {
                let tiff = &data[10..];
                let order = order_from_marker(&tiff[0..2])?;
                let ifd_rel = match order {
                    ByteOrder::LittleEndian => u32::from_le_bytes(tiff[4..8].try_into().ok()?),
                    ByteOrder::BigEndian => u32::from_be_bytes(tiff[4..8].try_into().ok()?),
                };
                let patched = patch_nikon(tiff, ifd_rel, order, metadata, false)?;
                let mut out = Vec::with_capacity(10 + patched.len());
                out.extend_from_slice(&data[..10]);
                out.extend_from_slice(&patched);
                return Some(out);
            }
            _ => {}
        }
    }
    let make = metadata
        .exif
        .get_str("Make")
        .unwrap_or("")
        .to_ascii_lowercase();
    if make.contains("canon") {
        let order = detect_order(data, 0)?;
        return patch_canon(data, 0, order, metadata, true);
    }
    if make.contains("nikon") && !data.starts_with(NIKON_HEADER) {
        let order = detect_order(data, 0)?;
        return patch_nikon(data, 0, order, metadata, true);
    }
    if make.contains("kodak") && !data.starts_with(b"KDK") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_kodak, metadata, true);
    }
    if make.contains("sony") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_sony, metadata, true);
    }
    if make.contains("hasselblad") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_hasselblad, metadata, true);
    }
    if make.contains("casio") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_casio_type1, metadata, true);
    }
    if make.contains("samsung") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_samsung, metadata, true);
    }
    if make.contains("minolta") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_minolta, metadata, true);
    }
    if make.contains("motorola") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_motorola, metadata, true);
    }
    if make.contains("xiaomi") || make.contains("redmi") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_xiaomi, metadata, true);
    }
    if make.contains("oneplus") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_oneplus, metadata, true);
    }
    if make.contains("oppo") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_oppo, metadata, true);
    }
    if make.contains("vivo") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_vivo, metadata, true);
    }
    if make.contains("realme") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_realme, metadata, true);
    }
    if make.contains("huawei") || make.contains("honor") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_huawei, metadata, true);
    }
    if make.contains("google") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_google, metadata, true);
    }
    if make.contains("phase one")
        || make.contains("phaseone")
        || make.contains("leaf")
        || make.contains("mamiya")
    {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_phaseone, metadata, true);
    }
    if make.contains("leica") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_leica, metadata, true);
    }
    if make.contains("dji") {
        let order = detect_order(data, 0)?;
        return patch_ifd(data, 0, order, lookup_dji, metadata, true);
    }
    if make.contains("gopro") && looks_like_gpmf(data) {
        return patch_gpmf(data, metadata);
    }
    None
}

fn looks_like_gpmf(data: &[u8]) -> bool {
    if data.len() < 8 {
        return false;
    }
    if !data[..4].iter().all(|b| b.is_ascii_graphic()) {
        return false;
    }
    matches!(
        data[4] as char,
        '\0' | '?' | 'c' | 'U' | 'F' | 'b' | 'B' | 's' | 'S' | 'l' | 'L' | 'f' | 'd' | 'J' | 'G'
    )
}

fn patch_gpmf(data: &[u8], metadata: &Metadata) -> Option<Vec<u8>> {
    let mut out = data.to_vec();
    let start = if data.len() >= 6 && data.starts_with(b"GoPro\0") {
        6
    } else {
        0
    };
    let end = out.len();
    overlay_gpmf(&mut out, start, end, metadata, "")?;
    Some(out)
}

fn overlay_gpmf(
    buf: &mut [u8],
    start: usize,
    end: usize,
    metadata: &Metadata,
    prefix: &str,
) -> Option<()> {
    let mut offset = start;
    while offset + 8 <= end {
        let fourcc_str = String::from_utf8_lossy(&buf[offset..offset + 4]).into_owned();
        let type_char = buf[offset + 4] as char;
        let struct_size = buf[offset + 5] as usize;
        let repeat = u16::from_be_bytes([buf[offset + 6], buf[offset + 7]]) as usize;
        let data_size = struct_size.checked_mul(repeat)?;
        let padded_size = (data_size + 3) & !3;
        offset += 8;
        if offset + padded_size > end {
            break;
        }
        let value_start = offset;
        let value_end = offset + data_size;
        if type_char == '\0' || type_char == '?' {
            let new_prefix = if prefix.is_empty() {
                fourcc_str.clone()
            } else {
                format!("{}:{}", prefix, fourcc_str)
            };
            overlay_gpmf(buf, value_start, value_end, metadata, &new_prefix)?;
        } else {
            let tag_name = gopro::lookup(&fourcc_str)
                .map(|d| d.name.to_string())
                .unwrap_or_else(|| fourcc_str.clone());
            let full_name = if prefix.is_empty() {
                tag_name
            } else {
                format!("{}:{}", prefix, tag_name)
            };
            if let Some(val) = metadata.exif.get(&full_name) {
                if let Some(bytes) =
                    encode_gpmf_leaf(type_char, struct_size, repeat, data_size, val)
                {
                    buf[value_start..value_end].copy_from_slice(&bytes);
                }
            }
        }
        offset += padded_size;
    }
    Some(())
}

fn encode_gpmf_leaf(
    type_char: char,
    struct_size: usize,
    repeat: usize,
    data_size: usize,
    val: &AttrValue,
) -> Option<Vec<u8>> {
    if data_size == 0 {
        return None;
    }
    let mut out = match type_char {
        'c' | 'U' => {
            let s = match val {
                AttrValue::Str(s) => s.as_bytes(),
                _ => return None,
            };
            let mut b = s.to_vec();
            if b.len() > data_size {
                b.truncate(data_size);
            }
            b.resize(data_size, 0);
            b
        }
        'F' => {
            let s = match val {
                AttrValue::Str(s) => s.as_bytes(),
                _ => return None,
            };
            let mut b = vec![0u8; data_size];
            let n = s.len().min(4).min(data_size);
            b[..n].copy_from_slice(&s[..n]);
            b
        }
        'b' | 'B' => encode_gpmf_ints(val, repeat, 1, type_char == 'b')?,
        's' | 'S' if struct_size == 2 => encode_gpmf_ints(val, repeat, 2, type_char == 's')?,
        'l' | 'L' if struct_size == 4 => encode_gpmf_ints(val, repeat, 4, type_char == 'l')?,
        'f' if struct_size == 4 => encode_gpmf_floats(val, repeat)?,
        'd' if struct_size == 8 => encode_gpmf_doubles(val, repeat)?,
        'J' if struct_size == 8 && data_size >= 8 => {
            let n = match val {
                AttrValue::UInt64(v) => *v,
                AttrValue::Int64(v) => *v as u64,
                AttrValue::UInt(v) => u64::from(*v),
                AttrValue::Int(v) => *v as u64,
                AttrValue::Str(s) => s.parse().ok()?,
                _ => return None,
            };
            n.to_be_bytes().to_vec()
        }
        _ => return None,
    };
    if out.len() != data_size {
        if out.len() > data_size {
            return None;
        }
        out.resize(data_size, 0);
    }
    Some(out)
}

fn gpmf_i64_list(val: &AttrValue, repeat: usize) -> Option<Vec<i64>> {
    match val {
        AttrValue::Int(v) if repeat == 1 => Some(vec![i64::from(*v)]),
        AttrValue::UInt(v) if repeat == 1 => Some(vec![i64::from(*v)]),
        AttrValue::Int64(v) if repeat == 1 => Some(vec![*v]),
        AttrValue::UInt64(v) if repeat == 1 => Some(vec![*v as i64]),
        AttrValue::Str(s) => {
            let parts: Vec<i64> = s
                .split_whitespace()
                .map(|p| p.parse::<i64>())
                .collect::<Result<Vec<_>, _>>()
                .ok()?;
            if parts.len() == repeat {
                Some(parts)
            } else if parts.len() == 1 && repeat == 1 {
                Some(parts)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn encode_gpmf_ints(val: &AttrValue, repeat: usize, width: usize, signed: bool) -> Option<Vec<u8>> {
    let nums = gpmf_i64_list(val, repeat)?;
    let mut out = Vec::with_capacity(repeat * width);
    for n in nums {
        match (width, signed) {
            (1, true) => out.push(n as i8 as u8),
            (1, false) => out.push(n as u8),
            (2, true) => out.extend_from_slice(&(n as i16).to_be_bytes()),
            (2, false) => out.extend_from_slice(&(n as u16).to_be_bytes()),
            (4, true) => out.extend_from_slice(&(n as i32).to_be_bytes()),
            (4, false) => out.extend_from_slice(&(n as u32).to_be_bytes()),
            _ => return None,
        }
    }
    Some(out)
}

fn encode_gpmf_floats(val: &AttrValue, repeat: usize) -> Option<Vec<u8>> {
    let nums = match val {
        AttrValue::Float(v) if repeat == 1 => vec![*v],
        AttrValue::Double(v) if repeat == 1 => vec![*v as f32],
        AttrValue::Int(v) if repeat == 1 => vec![*v as f32],
        AttrValue::UInt(v) if repeat == 1 => vec![*v as f32],
        AttrValue::Str(s) => {
            let parts: Vec<f32> = s
                .split_whitespace()
                .map(|p| p.parse::<f32>())
                .collect::<Result<Vec<_>, _>>()
                .ok()?;
            if parts.len() != repeat {
                return None;
            }
            parts
        }
        _ => return None,
    };
    let mut out = Vec::with_capacity(repeat * 4);
    for n in nums {
        out.extend_from_slice(&n.to_be_bytes());
    }
    Some(out)
}

fn encode_gpmf_doubles(val: &AttrValue, repeat: usize) -> Option<Vec<u8>> {
    let nums = match val {
        AttrValue::Double(v) if repeat == 1 => vec![*v],
        AttrValue::Float(v) if repeat == 1 => vec![f64::from(*v)],
        AttrValue::Str(s) => {
            let parts: Vec<f64> = s
                .split_whitespace()
                .map(|p| p.parse::<f64>())
                .collect::<Result<Vec<_>, _>>()
                .ok()?;
            if parts.len() != repeat {
                return None;
            }
            parts
        }
        _ => return None,
    };
    let mut out = Vec::with_capacity(repeat * 8);
    for n in nums {
        out.extend_from_slice(&n.to_be_bytes());
    }
    Some(out)
}

fn rewrite_fujifilm(data: &[u8], metadata: &Metadata) -> Option<Vec<u8>> {
    if data.len() < 12 {
        return None;
    }
    let order = ByteOrder::LittleEndian;
    let ifd_off = u32::from_le_bytes(data[8..12].try_into().ok()?);
    let mut entries = super::parse_ifd_entries(data, order, ifd_off)?;
    if entries.is_empty() {
        return None;
    }
    for e in &mut entries {
        overlay_fuji_entry(e, metadata);
    }
    let ifd = emit_ifd(&entries, order, 12)?;
    let mut out = Vec::with_capacity(12 + ifd.len());
    out.extend_from_slice(FUJI_MAGIC);
    out.extend_from_slice(&12u32.to_le_bytes());
    out.extend_from_slice(&ifd);
    Some(out)
}

fn overlay_fuji_entry(entry: &mut IfdEntry, metadata: &Metadata) {
    let Some(def) = fujifilm::FUJIFILM_MAIN.get(&entry.tag) else {
        return;
    };
    let Some(val) = metadata.exif.get(def.name) else {
        return;
    };
    if matches!(val, AttrValue::Group(_)) {
        return;
    }
    if let Some(patched) = attr_to_entry(entry, val, def.values) {
        *entry = patched;
    }
}

fn attr_to_entry(
    orig: &IfdEntry,
    val: &AttrValue,
    print_map: Option<&'static [(i64, &'static str)]>,
) -> Option<IfdEntry> {
    if let (Some(map), AttrValue::Str(s)) = (print_map, val) {
        for &(key, label) in map {
            if label == s {
                return int_entry(orig, key);
            }
        }
    }
    match (orig.format, val) {
        (ExifFormat::String | ExifFormat::Utf8, AttrValue::Str(s)) => Some(IfdEntry {
            tag: orig.tag,
            format: ExifFormat::String,
            count: (s.len() + 1) as u32,
            value: RawValue::String(s.clone()),
            value_offset: None,
        }),
        (ExifFormat::UInt16, AttrValue::UInt(v)) => int_entry(orig, *v as i64),
        (ExifFormat::UInt32, AttrValue::UInt(v)) => int_entry(orig, *v as i64),
        (ExifFormat::UInt8, AttrValue::UInt(v)) => int_entry(orig, *v as i64),
        (ExifFormat::Int16, AttrValue::Int(v)) => int_entry(orig, i64::from(*v)),
        (ExifFormat::Int32, AttrValue::Int(v)) => int_entry(orig, i64::from(*v)),
        (ExifFormat::URational, AttrValue::URational(n, d)) => Some(IfdEntry {
            tag: orig.tag,
            format: ExifFormat::URational,
            count: 1,
            value: RawValue::URational(vec![URational::new(*n, *d)]),
            value_offset: None,
        }),
        (ExifFormat::SRational, AttrValue::Rational(n, d)) => Some(IfdEntry {
            tag: orig.tag,
            format: ExifFormat::SRational,
            count: 1,
            value: RawValue::SRational(vec![SRational::new(*n, *d)]),
            value_offset: None,
        }),
        (ExifFormat::UInt16, AttrValue::Str(s)) => s
            .parse::<u16>()
            .ok()
            .and_then(|n| int_entry(orig, n as i64)),
        (ExifFormat::Float, AttrValue::Float(v)) => Some(IfdEntry {
            tag: orig.tag,
            format: ExifFormat::Float,
            count: 1,
            value: RawValue::Float(vec![*v]),
            value_offset: None,
        }),
        (ExifFormat::Float, AttrValue::Double(v)) => Some(IfdEntry {
            tag: orig.tag,
            format: ExifFormat::Float,
            count: 1,
            value: RawValue::Float(vec![*v as f32]),
            value_offset: None,
        }),
        (ExifFormat::Float, AttrValue::Str(s)) => s.parse::<f32>().ok().map(|v| IfdEntry {
            tag: orig.tag,
            format: ExifFormat::Float,
            count: 1,
            value: RawValue::Float(vec![v]),
            value_offset: None,
        }),
        (ExifFormat::Double, AttrValue::Double(v)) => Some(IfdEntry {
            tag: orig.tag,
            format: ExifFormat::Double,
            count: 1,
            value: RawValue::Double(vec![*v]),
            value_offset: None,
        }),
        _ => None,
    }
}

fn int_entry(orig: &IfdEntry, key: i64) -> Option<IfdEntry> {
    let (format, value) = match orig.format {
        ExifFormat::UInt8 => (ExifFormat::UInt8, RawValue::UInt8(vec![key as u8])),
        ExifFormat::UInt16 => (ExifFormat::UInt16, RawValue::UInt16(vec![key as u16])),
        ExifFormat::UInt32 => (ExifFormat::UInt32, RawValue::UInt32(vec![key as u32])),
        ExifFormat::Int8 => (ExifFormat::Int8, RawValue::Int8(vec![key as i8])),
        ExifFormat::Int16 => (ExifFormat::Int16, RawValue::Int16(vec![key as i16])),
        ExifFormat::Int32 => (ExifFormat::Int32, RawValue::Int32(vec![key as i32])),
        _ => return None,
    };
    Some(IfdEntry {
        tag: orig.tag,
        format,
        count: 1,
        value,
        value_offset: None,
    })
}

fn order_from_marker(m: &[u8]) -> Option<ByteOrder> {
    match m {
        b"II" => Some(ByteOrder::LittleEndian),
        b"MM" => Some(ByteOrder::BigEndian),
        _ => None,
    }
}

fn detect_order(data: &[u8], ifd_off: u32) -> Option<ByteOrder> {
    let le_ok = ifd_plausible(data, ifd_off, ByteOrder::LittleEndian);
    let be_ok = ifd_plausible(data, ifd_off, ByteOrder::BigEndian);
    match (le_ok, be_ok) {
        (true, false) => Some(ByteOrder::LittleEndian),
        (false, true) => Some(ByteOrder::BigEndian),
        (true, true) => Some(ByteOrder::LittleEndian),
        (false, false) => None,
    }
}

fn ifd_plausible(data: &[u8], ifd_off: u32, order: ByteOrder) -> bool {
    let o = ifd_off as usize;
    if o + 2 > data.len() {
        return false;
    }
    let n = read_u16_at(data, o, order) as usize;
    n >= 1 && n <= 512 && o + 2 + n * 12 + 4 <= data.len()
}

fn read_u16_at(data: &[u8], at: usize, order: ByteOrder) -> u16 {
    match order {
        ByteOrder::LittleEndian => u16::from_le_bytes([data[at], data[at + 1]]),
        ByteOrder::BigEndian => u16::from_be_bytes([data[at], data[at + 1]]),
    }
}

fn patch_ifd(
    data: &[u8],
    ifd_off: u32,
    order: ByteOrder,
    lookup: TagLookup,
    metadata: &Metadata,
    offsets_from_ifd: bool,
) -> Option<Vec<u8>> {
    let start = ifd_off as usize;
    if start > data.len() {
        return None;
    }
    let (parse_data, parse_off, extra_add) = if offsets_from_ifd {
        (&data[start..], 0u32, start)
    } else {
        (data, ifd_off, 0usize)
    };
    let entries = super::parse_ifd_entries(parse_data, order, parse_off)?;
    if entries.is_empty() {
        return None;
    }
    let mut out = data.to_vec();
    for (i, e) in entries.iter().enumerate() {
        let Some((name, print_map)) = lookup(e.tag) else {
            continue;
        };
        let keyed = format!("Leica:{name}");
        let Some(val) = metadata
            .exif
            .get(name)
            .or_else(|| metadata.exif.get(&keyed))
        else {
            continue;
        };
        if matches!(val, AttrValue::Group(_)) {
            continue;
        }
        let Some(patched) = attr_to_entry_fixed(e, val, print_map) else {
            continue;
        };
        let Some(encoded) = encode_value(&patched.value, order) else {
            continue;
        };
        let orig = encode_value(&e.value, order)?;
        if encoded.len() != orig.len() {
            continue;
        }
        let entry_pos = start + 2 + i * 12;
        if encoded.len() <= 4 {
            let mut field = [0u8; 4];
            field[..encoded.len()].copy_from_slice(&encoded);
            out[entry_pos + 8..entry_pos + 12].copy_from_slice(&field);
        } else if let Some(off) = e.value_offset {
            let off = extra_add + off as usize;
            if off + encoded.len() <= out.len() {
                out[off..off + encoded.len()].copy_from_slice(&encoded);
            }
        }
    }
    Some(out)
}

fn patch_olympus(
    data: &[u8],
    ifd_off: u32,
    order: ByteOrder,
    metadata: &Metadata,
    offsets_from_ifd: bool,
) -> Option<Vec<u8>> {
    let mut out = patch_ifd(
        data,
        ifd_off,
        order,
        lookup_olympus,
        metadata,
        offsets_from_ifd,
    )?;
    overlay_olympus_subifds(&mut out, ifd_off, order, offsets_from_ifd, metadata);
    Some(out)
}

fn lookup_olympus_equipment(
    tag: u16,
) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    olympus::OLYMPUS_EQUIPMENT
        .get(&tag)
        .map(|d| (d.name, d.values))
}
fn lookup_olympus_camerasettings(
    tag: u16,
) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    olympus::OLYMPUS_CAMERASETTINGS
        .get(&tag)
        .map(|d| (d.name, d.values))
}
fn lookup_olympus_imageprocessing(
    tag: u16,
) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    olympus::OLYMPUS_IMAGEPROCESSING
        .get(&tag)
        .map(|d| (d.name, d.values))
}
fn lookup_olympus_focusinfo(
    tag: u16,
) -> Option<(&'static str, Option<&'static [(i64, &'static str)]>)> {
    olympus::OLYMPUS_FOCUSINFO
        .get(&tag)
        .map(|d| (d.name, d.values))
}

fn overlay_olympus_subifds(
    out: &mut Vec<u8>,
    ifd_off: u32,
    order: ByteOrder,
    offsets_from_ifd: bool,
    metadata: &Metadata,
) {
    let start = ifd_off as usize;
    if start > out.len() {
        return;
    }
    let (parse_data, parse_off, extra_add) = if offsets_from_ifd {
        (&out[start..], 0u32, start)
    } else {
        (out.as_slice(), ifd_off, 0usize)
    };
    let Some(entries) = super::parse_ifd_entries(parse_data, order, parse_off) else {
        return;
    };
    for e in entries {
        let (group, lookup): (&str, TagLookup) = match e.tag {
            0x2010 => ("Equipment", lookup_olympus_equipment),
            0x2020 => ("CameraSettings", lookup_olympus_camerasettings),
            0x2040 => ("ImageProcessing", lookup_olympus_imageprocessing),
            0x2050 => ("FocusInfo", lookup_olympus_focusinfo),
            _ => continue,
        };
        let Some(rel) = e.value.as_u32() else {
            continue;
        };
        let Some(gmeta) = metadata_from_group(metadata, group) else {
            continue;
        };
        let abs = if offsets_from_ifd {
            extra_add as u32 + rel
        } else {
            rel
        };
        if let Some(patched) = patch_ifd(out, abs, order, lookup, &gmeta, offsets_from_ifd) {
            *out = patched;
        }
    }
}

fn patch_canon(
    data: &[u8],
    ifd_off: u32,
    order: ByteOrder,
    metadata: &Metadata,
    offsets_from_ifd: bool,
) -> Option<Vec<u8>> {
    let mut out = patch_ifd(
        data,
        ifd_off,
        order,
        lookup_canon,
        metadata,
        offsets_from_ifd,
    )?;
    overlay_canon_i16_groups(&mut out, ifd_off, order, offsets_from_ifd, metadata);
    Some(out)
}

fn overlay_canon_i16_groups(
    out: &mut [u8],
    ifd_off: u32,
    order: ByteOrder,
    offsets_from_ifd: bool,
    metadata: &Metadata,
) {
    let start = ifd_off as usize;
    if start > out.len() {
        return;
    }
    let parse_buf = out.to_vec();
    let (parse_data, parse_off, extra_add) = if offsets_from_ifd {
        (&parse_buf[start..], 0u32, start)
    } else {
        (parse_buf.as_slice(), ifd_off, 0usize)
    };
    let Some(entries) = super::parse_ifd_entries(parse_data, order, parse_off) else {
        return;
    };
    for (i, e) in entries.iter().enumerate() {
        let (group, lookup): (&str, TagLookup) = match e.tag {
            0x0001 => ("CameraSettings", lookup_canon_camerasettings),
            0x0004 => ("ShotInfo", lookup_canon_shotinfo),
            _ => continue,
        };
        let Some(gmeta) = metadata.exif.get(group) else {
            continue;
        };
        let AttrValue::Group(g) = gmeta else {
            continue;
        };
        let Some(orig) = encode_value(&e.value, order) else {
            continue;
        };
        if orig.len() < 2 {
            continue;
        }
        let entry_pos = start + 2 + i * 12;
        let dest = if orig.len() <= 4 {
            let end = entry_pos + 8 + orig.len();
            if end > out.len() {
                continue;
            }
            &mut out[entry_pos + 8..end]
        } else if let Some(off) = e.value_offset {
            let off = extra_add + off as usize;
            let end = off + orig.len();
            if end > out.len() {
                continue;
            }
            &mut out[off..end]
        } else {
            continue;
        };
        let count = dest.len() / 2;
        for idx in 0..count {
            let Some((name, print_map)) = lookup(idx as u16) else {
                continue;
            };
            let Some(val) = g.get(name) else {
                continue;
            };
            let Some(n) = i16_from_attr(val, print_map) else {
                continue;
            };
            let b = i16_bytes(n, order);
            dest[idx * 2..idx * 2 + 2].copy_from_slice(&b);
        }
    }
}

fn metadata_from_group(metadata: &Metadata, name: &str) -> Option<Metadata> {
    let AttrValue::Group(g) = metadata.exif.get(name)? else {
        return None;
    };
    let mut m = Metadata::new(metadata.format);
    for (k, v) in g.iter() {
        m.exif.set(k.clone(), v.clone());
    }
    Some(m)
}

fn i16_from_attr(
    val: &AttrValue,
    print_map: Option<&'static [(i64, &'static str)]>,
) -> Option<i16> {
    if let (Some(map), AttrValue::Str(s)) = (print_map, val) {
        for &(key, label) in map {
            if label == s {
                return Some(key as i16);
            }
        }
    }
    match val {
        AttrValue::Int(v) => Some(*v as i16),
        AttrValue::UInt(v) => Some(*v as i16),
        AttrValue::Int8(v) => Some(i16::from(*v)),
        AttrValue::Str(s) => s.parse().ok(),
        _ => None,
    }
}

fn patch_nikon(
    data: &[u8],
    ifd_off: u32,
    order: ByteOrder,
    metadata: &Metadata,
    offsets_from_ifd: bool,
) -> Option<Vec<u8>> {
    let mut out = patch_ifd(
        data,
        ifd_off,
        order,
        lookup_nikon,
        metadata,
        offsets_from_ifd,
    )?;
    overlay_shot_info(&mut out, data, ifd_off, order, offsets_from_ifd, metadata);
    Some(out)
}

fn overlay_shot_info(
    out: &mut [u8],
    keys_src: &[u8],
    ifd_off: u32,
    order: ByteOrder,
    offsets_from_ifd: bool,
    metadata: &Metadata,
) {
    let write = super::nikon::ShotInfoWrite {
        firmware_version: metadata.exif.get_str("FirmwareVersion").map(str::to_string),
        vibration_reduction: metadata
            .exif
            .get_str("VibrationReduction")
            .map(str::to_string),
        shutter_count: metadata.exif.get_u32("ShutterCount"),
    };
    let lens_id = metadata.exif.get_u32("LensIDNumber").map(|n| n as u8);
    let start = ifd_off as usize;
    let (keys_parse, parse_off, extra_add) = if offsets_from_ifd {
        if start >= keys_src.len() {
            return;
        }
        (&keys_src[start..], 0u32, start)
    } else {
        (keys_src, ifd_off, 0usize)
    };
    let Some(key_entries) = super::parse_ifd_entries(keys_parse, order, parse_off) else {
        return;
    };
    let (serial_key, shutter_key) =
        super::nikon::decrypt_keys_from_ifd(&key_entries, metadata.exif.get_str("Model"));
    let out_parse: &[u8] = if offsets_from_ifd {
        if start >= out.len() {
            return;
        }
        &out[start..]
    } else {
        out
    };
    let Some(entries) = super::parse_ifd_entries(out_parse, order, parse_off) else {
        return;
    };
    for e in entries {
        let Some(off) = e.value_offset else {
            continue;
        };
        let abs = extra_add + off as usize;
        let len = match &e.value {
            RawValue::Undefined(v) => v.len(),
            RawValue::UInt8(v) => v.len(),
            _ => continue,
        };
        if abs + len > out.len() {
            continue;
        }
        let extra = out[abs..abs + len].to_vec();
        let new = match e.tag {
            0x0091 => {
                if write.is_empty() {
                    None
                } else {
                    super::nikon::rewrite_shot_info_full(&extra, serial_key, shutter_key, &write)
                }
            }
            0x0097 => (|| {
                let name = super::nikon::color_balance_tag_name(&extra)?;
                let s = metadata.exif.get_str(name)?;
                let levels = parse_u16x4(s)?;
                super::nikon::rewrite_color_balance(&extra, serial_key, shutter_key, order, &levels)
            })(),
            0x0098 => lens_id.and_then(|id| {
                super::nikon::rewrite_lens_data(&extra, serial_key, shutter_key, id)
            }),
            _ => None,
        };
        if let Some(new) = new {
            if new.len() == len {
                out[abs..abs + len].copy_from_slice(&new);
            }
        }
    }
}

fn parse_u16x4(s: &str) -> Option<[u16; 4]> {
    let mut out = [0u16; 4];
    let mut n = 0;
    for p in s.split_whitespace() {
        if n >= 4 {
            return None;
        }
        out[n] = p.parse().ok()?;
        n += 1;
    }
    if n == 4 {
        Some(out)
    } else {
        None
    }
}

fn attr_to_entry_fixed(
    orig: &IfdEntry,
    val: &AttrValue,
    print_map: Option<&'static [(i64, &'static str)]>,
) -> Option<IfdEntry> {
    if let (Some(map), AttrValue::Str(s)) = (print_map, val) {
        for &(key, label) in map {
            if label == s {
                return int_entry(orig, key);
            }
        }
    }
    match (orig.format, val) {
        (ExifFormat::String | ExifFormat::Utf8, AttrValue::Str(s)) => {
            let n = orig.count as usize;
            if n == 0 {
                return None;
            }
            let mut body = s.as_bytes().to_vec();
            if body.len() + 1 > n {
                body.truncate(n.saturating_sub(1));
            }
            while body.len() + 1 < n {
                body.push(0);
            }
            Some(IfdEntry {
                tag: orig.tag,
                format: ExifFormat::String,
                count: orig.count,
                value: RawValue::String(String::from_utf8_lossy(&body).into_owned()),
                value_offset: orig.value_offset,
            })
        }
        _ => attr_to_entry(orig, val, None).map(|mut e| {
            e.count = orig.count;
            e.value_offset = orig.value_offset;
            e
        }),
    }
}

fn emit_ifd(entries: &[IfdEntry], order: ByteOrder, value_base: u32) -> Option<Vec<u8>> {
    if entries.len() > u16::MAX as usize {
        return None;
    }
    let n = entries.len() as u16;
    let dir_len = 2 + entries.len() * 12 + 4;
    let mut extras: Vec<Vec<u8>> = Vec::with_capacity(entries.len());
    let mut extra_off = value_base + dir_len as u32;
    let mut dir = vec![0u8; dir_len];
    put_u16(&mut dir, 0, n, order);
    put_u32(&mut dir, dir_len - 4, 0, order);
    for (i, e) in entries.iter().enumerate() {
        let encoded = encode_value(&e.value, order)?;
        let pos = 2 + i * 12;
        put_u16(&mut dir, pos, e.tag, order);
        put_u16(&mut dir, pos + 2, e.format as u16, order);
        put_u32(&mut dir, pos + 4, e.count, order);
        if encoded.len() <= 4 {
            let mut field = [0u8; 4];
            field[..encoded.len()].copy_from_slice(&encoded);
            dir[pos + 8..pos + 12].copy_from_slice(&field);
            extras.push(Vec::new());
        } else {
            put_u32(&mut dir, pos + 8, extra_off, order);
            extra_off += encoded.len() as u32;
            extras.push(encoded);
        }
    }
    let mut out = dir;
    for x in extras {
        out.extend_from_slice(&x);
    }
    Some(out)
}

fn encode_value(value: &RawValue, order: ByteOrder) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    match value {
        RawValue::UInt8(v) => buf.extend_from_slice(v),
        RawValue::String(s) => {
            buf.extend_from_slice(s.as_bytes());
            buf.push(0);
        }
        RawValue::UInt16(v) => {
            for n in v {
                buf.extend_from_slice(&u16_bytes(*n, order));
            }
        }
        RawValue::UInt32(v) => {
            for n in v {
                buf.extend_from_slice(&u32_bytes(*n, order));
            }
        }
        RawValue::URational(v) => {
            for r in v {
                buf.extend_from_slice(&u32_bytes(r.num, order));
                buf.extend_from_slice(&u32_bytes(r.den, order));
            }
        }
        RawValue::Int8(v) => {
            for n in v {
                buf.push(*n as u8);
            }
        }
        RawValue::Undefined(v) => buf.extend_from_slice(v),
        RawValue::Int16(v) => {
            for n in v {
                buf.extend_from_slice(&i16_bytes(*n, order));
            }
        }
        RawValue::Int32(v) => {
            for n in v {
                buf.extend_from_slice(&i32_bytes(*n, order));
            }
        }
        RawValue::SRational(v) => {
            for r in v {
                buf.extend_from_slice(&i32_bytes(r.num, order));
                buf.extend_from_slice(&i32_bytes(r.den, order));
            }
        }
        RawValue::Float(v) => {
            for n in v {
                let b = match order {
                    ByteOrder::LittleEndian => n.to_le_bytes(),
                    ByteOrder::BigEndian => n.to_be_bytes(),
                };
                buf.extend_from_slice(&b);
            }
        }
        RawValue::Double(v) => {
            for n in v {
                let b = match order {
                    ByteOrder::LittleEndian => n.to_le_bytes(),
                    ByteOrder::BigEndian => n.to_be_bytes(),
                };
                buf.extend_from_slice(&b);
            }
        }
        RawValue::UInt64(v) => {
            for n in v {
                let b = match order {
                    ByteOrder::LittleEndian => n.to_le_bytes(),
                    ByteOrder::BigEndian => n.to_be_bytes(),
                };
                buf.extend_from_slice(&b);
            }
        }
        RawValue::Int64(v) => {
            for n in v {
                let b = match order {
                    ByteOrder::LittleEndian => n.to_le_bytes(),
                    ByteOrder::BigEndian => n.to_be_bytes(),
                };
                buf.extend_from_slice(&b);
            }
        }
    }
    Some(buf)
}

fn put_u16(buf: &mut [u8], at: usize, v: u16, order: ByteOrder) {
    buf[at..at + 2].copy_from_slice(&u16_bytes(v, order));
}

fn put_u32(buf: &mut [u8], at: usize, v: u32, order: ByteOrder) {
    buf[at..at + 4].copy_from_slice(&u32_bytes(v, order));
}

fn u16_bytes(v: u16, order: ByteOrder) -> [u8; 2] {
    match order {
        ByteOrder::LittleEndian => v.to_le_bytes(),
        ByteOrder::BigEndian => v.to_be_bytes(),
    }
}

fn u32_bytes(v: u32, order: ByteOrder) -> [u8; 4] {
    match order {
        ByteOrder::LittleEndian => v.to_le_bytes(),
        ByteOrder::BigEndian => v.to_be_bytes(),
    }
}

fn i16_bytes(v: i16, order: ByteOrder) -> [u8; 2] {
    match order {
        ByteOrder::LittleEndian => v.to_le_bytes(),
        ByteOrder::BigEndian => v.to_be_bytes(),
    }
}

fn i32_bytes(v: i32, order: ByteOrder) -> [u8; 4] {
    match order {
        ByteOrder::LittleEndian => v.to_le_bytes(),
        ByteOrder::BigEndian => v.to_be_bytes(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::makernotes::{FujifilmParser, VendorParser};
    use exiftool_attrs::Attrs;

    fn fuji_quality(q: &str) -> Vec<u8> {
        let entry = IfdEntry {
            tag: 0x1000,
            format: ExifFormat::String,
            count: (q.len() + 1) as u32,
            value: RawValue::String(q.to_string()),
            value_offset: None,
        };
        let ifd = emit_ifd(&[entry], ByteOrder::LittleEndian, 12).unwrap();
        let mut b = Vec::from(FUJI_MAGIC);
        b.extend_from_slice(&12u32.to_le_bytes());
        b.extend_from_slice(&ifd);
        b
    }

    #[test]
    fn fujifilm_quality_overlay_keeps_other_bytes_layout() {
        let src = fuji_quality("NORMAL ");
        let mut meta = Metadata::new("RAF");
        meta.exif.set("Quality", AttrValue::Str("FINE".into()));
        let out = rewrite_blob(&src, &meta);
        assert!(out.starts_with(FUJI_MAGIC));
        let parsed = FujifilmParser.parse(&out, ByteOrder::LittleEndian).unwrap();
        assert_eq!(parsed.get_str("Quality"), Some("FINE"));
    }

    #[test]
    fn fujifilm_raf_mn_blob_quality() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/testdata/FujiFilm.raf");
        if !path.exists() {
            return;
        }
        let data = std::fs::read(&path).unwrap();
        let jpos = u32::from_be_bytes(data[0x54..0x58].try_into().unwrap()) as usize;
        let jlen = u32::from_be_bytes(data[0x58..0x5C].try_into().unwrap()) as usize;
        let jpeg = &data[jpos..jpos + jlen];
        assert_eq!(&jpeg[0..4], &[0xFF, 0xD8, 0xFF, 0xE1]);
        let seglen = u16::from_be_bytes(jpeg[4..6].try_into().unwrap()) as usize;
        let tiff = &jpeg[12..2 + seglen];
        let mn = &tiff[1654..1654 + 618];
        assert!(mn.starts_with(FUJI_MAGIC));
        let mut meta = Metadata::new("RAF");
        meta.exif.set("Quality", AttrValue::Str("FINE".into()));
        let out = rewrite_blob(mn, &meta);
        let parsed = FujifilmParser.parse(&out, ByteOrder::LittleEndian).unwrap();
        assert_eq!(
            parsed.get_str("Quality"),
            Some("FINE"),
            "rewrite_blob on RAF MN"
        );
    }

    #[test]
    fn unknown_blob_copied() {
        let data = b"Nikon\0\0\x02\x00\x00\x00";
        let meta = Metadata::new("NEF");
        assert_eq!(rewrite_blob(data, &meta), data);
    }

    fn prefix_ifd(prefix: &[u8], tag: u16, format: ExifFormat, value: RawValue) -> Vec<u8> {
        let count = match &value {
            RawValue::String(s) => (s.len() + 1) as u32,
            RawValue::UInt16(v) => v.len() as u32,
            RawValue::Int16(v) => v.len() as u32,
            RawValue::UInt32(v) => v.len() as u32,
            RawValue::Int32(v) => v.len() as u32,
            RawValue::Undefined(v) => v.len() as u32,
            _ => 1,
        };
        let entry = IfdEntry {
            tag,
            format,
            count,
            value,
            value_offset: None,
        };
        let ifd = emit_ifd(&[entry], ByteOrder::LittleEndian, 0).unwrap();
        let mut b = prefix.to_vec();
        b.extend_from_slice(&ifd);
        b
    }

    #[test]
    fn panasonic_imagequality_inplace() {
        let mut prefix = b"Panasonic".to_vec();
        prefix.extend_from_slice(&[0, 0, 0]);
        let src = prefix_ifd(&prefix, 1, ExifFormat::UInt16, RawValue::UInt16(vec![3]));
        let mut meta = Metadata::new("JPG");
        meta.exif.set("ImageQuality", AttrValue::Str("High".into()));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::PanasonicParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_str("ImageQuality"), Some("High"));
        assert_eq!(out.len(), src.len());
    }

    #[test]
    fn sony_quality_inplace() {
        let mut prefix = b"SONY DSC ".to_vec();
        prefix.extend_from_slice(&[0, 0, 0]);
        let src = prefix_ifd(
            &prefix,
            0x0102,
            ExifFormat::UInt16,
            RawValue::UInt16(vec![0]),
        );
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Quality", AttrValue::Str("Fine".into()));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::SonyParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_str("Quality"), Some("Fine"));
    }

    #[test]
    fn canon_ownername_inplace() {
        let src = prefix_ifd(
            b"",
            9,
            ExifFormat::String,
            RawValue::String("OrigName".into()),
        );
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Make", AttrValue::Str("Canon".into()));
        meta.exif.set("OwnerName", AttrValue::Str("NewName".into()));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::CanonParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_str("OwnerName"), Some("NewName"));
        assert_eq!(out.len(), src.len());
    }

    #[test]
    fn olympus_type1_serial_inplace() {
        let mut prefix = b"OLYMP\x00".to_vec();
        prefix.extend_from_slice(&[0, 0]);
        let src = prefix_ifd(
            &prefix,
            0x0404,
            ExifFormat::String,
            RawValue::String("OLD SERIAL".into()),
        );
        let mut meta = Metadata::new("JPG");
        meta.exif
            .set("SerialNumber", AttrValue::Str("NEW SERIAL".into()));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::OlympusParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_str("SerialNumber"), Some("NEW SERIAL"));
    }

    #[test]
    fn pentax5_quality_inplace() {
        let prefix = b"PENTAX \0II".to_vec();
        let src = prefix_ifd(&prefix, 8, ExifFormat::UInt16, RawValue::UInt16(vec![3]));
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Quality", AttrValue::Str("Best".into()));
        let out = rewrite_blob(&src, &meta);
        assert_eq!(out.len(), src.len());
        let entries =
            crate::makernotes::parse_ifd_entries(&out, ByteOrder::LittleEndian, 10).unwrap();
        match &entries[0].value {
            RawValue::UInt16(v) => assert_eq!(v[0], 2),
            _ => panic!("expected uint16 Quality"),
        }
    }

    #[test]
    fn nikon_type3_quality_inplace() {
        let entry = IfdEntry {
            tag: 4,
            format: ExifFormat::String,
            count: 9,
            value: RawValue::String("NORMAL  ".into()),
            value_offset: None,
        };
        let ifd = emit_ifd(&[entry], ByteOrder::LittleEndian, 8).unwrap();
        let mut tiff = Vec::from(b"II\x2a\x00\x08\x00\x00\x00".as_slice());
        tiff.extend_from_slice(&ifd);
        let mut src = Vec::from(b"Nikon\x00\x02\x10\x00\x00".as_slice());
        src.extend_from_slice(&tiff);
        let mut meta = Metadata::new("NEF");
        meta.exif.set("Quality", AttrValue::Str("FINE    ".into()));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::NikonParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_str("Quality").map(str::trim), Some("FINE"));
    }

    #[test]
    fn apple_hdr_inplace() {
        let mut prefix = b"Apple iOS\0".to_vec();
        prefix.extend_from_slice(&[0, 0, 0, 0]);
        let src = prefix_ifd(&prefix, 10, ExifFormat::UInt32, RawValue::UInt32(vec![3]));
        let mut meta = Metadata::new("JPG");
        meta.exif
            .set("HDRImageType", AttrValue::Str("Original Image".into()));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::AppleParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_str("HDRImageType"), Some("Original Image"));
    }

    #[test]
    fn casio_qvc_qualitymode_inplace() {
        let mut prefix = b"QVC\0".to_vec();
        prefix.extend_from_slice(&[0, 0]);
        let src = prefix_ifd(&prefix, 8, ExifFormat::UInt16, RawValue::UInt16(vec![1]));
        let mut meta = Metadata::new("JPG");
        meta.exif.set("QualityMode", AttrValue::UInt(2));
        let out = rewrite_blob(&src, &meta);
        let entries =
            crate::makernotes::parse_ifd_entries(&out[6..], ByteOrder::LittleEndian, 0).unwrap();
        match &entries[0].value {
            RawValue::UInt16(v) => assert_eq!(v[0], 2),
            _ => panic!("expected uint16"),
        }
    }

    #[test]
    fn sigma_serial_inplace() {
        let mut prefix = b"SIGMA\0\0\0".to_vec();
        prefix.extend_from_slice(&[0, 0]);
        let src = prefix_ifd(
            &prefix,
            2,
            ExifFormat::String,
            RawValue::String("OLD SERIAL".into()),
        );
        let mut meta = Metadata::new("JPG");
        meta.exif
            .set("SerialNumber", AttrValue::Str("NEW SERIAL".into()));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::SigmaParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_str("SerialNumber"), Some("NEW SERIAL"));
    }

    #[test]
    fn minolta_scenemode_inplace() {
        let src = prefix_ifd(b"", 256, ExifFormat::UInt16, RawValue::UInt16(vec![0]));
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Make", AttrValue::Str("Minolta".into()));
        meta.exif
            .set("SceneMode", AttrValue::Str("Portrait".into()));
        let out = rewrite_blob(&src, &meta);
        let entries =
            crate::makernotes::parse_ifd_entries(&out, ByteOrder::LittleEndian, 0).unwrap();
        match &entries[0].value {
            RawValue::UInt16(v) => assert_eq!(v[0], 1),
            _ => panic!("expected uint16 SceneMode"),
        }
    }

    #[test]
    fn samsung_preview_length_inplace() {
        let src = prefix_ifd(b"", 3, ExifFormat::UInt32, RawValue::UInt32(vec![100]));
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Make", AttrValue::Str("Samsung".into()));
        meta.exif.set("PreviewImageLength", AttrValue::UInt(200));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::SamsungParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_u32("PreviewImageLength"), Some(200));
    }

    #[test]
    fn nikon_type2_shotinfo_full_overlay() {
        let serial = 7u32;
        let shutter = 99u32;
        let mut plain = vec![0u8; 140];
        plain[0..4].copy_from_slice(b"0204");
        plain[4..9].copy_from_slice(b"1.00\0");
        plain[0x6a..0x6e].copy_from_slice(&100u32.to_be_bytes());
        plain[0x82] = 1;
        let extra = super::super::nikon::crypt_shot_info(&plain, serial, shutter);
        let entries = [
            IfdEntry {
                tag: 0x00A0,
                format: ExifFormat::String,
                count: 2,
                value: RawValue::String("7".into()),
                value_offset: None,
            },
            IfdEntry {
                tag: 0x00A7,
                format: ExifFormat::UInt32,
                count: 1,
                value: RawValue::UInt32(vec![shutter]),
                value_offset: None,
            },
            IfdEntry {
                tag: 0x0091,
                format: ExifFormat::Undefined,
                count: extra.len() as u32,
                value: RawValue::Undefined(extra),
                value_offset: None,
            },
        ];
        let ifd = emit_ifd(&entries, ByteOrder::LittleEndian, 0).unwrap();
        let mut src = Vec::from(b"Nikon\x00\x01\x00".as_slice());
        src.extend_from_slice(&ifd);
        let mut meta = Metadata::new("NEF");
        meta.exif
            .set("FirmwareVersion", AttrValue::Str("2.10".into()));
        meta.exif
            .set("VibrationReduction", AttrValue::Str("Off".into()));
        let out = rewrite_blob(&src, &meta);
        assert_eq!(out.len(), src.len());
        let parsed = crate::makernotes::NikonParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_str("FirmwareVersion"), Some("2.10"));
        assert_eq!(parsed.get_str("VibrationReduction"), Some("Off"));
    }

    #[test]
    fn kodak_type1_serial_inplace() {
        let src = prefix_ifd(
            b"",
            0x001c,
            ExifFormat::String,
            RawValue::String("OLDKODAK".into()),
        );
        let mut meta = Metadata::new("JPG");
        meta.exif
            .set("Make", AttrValue::Str("EASTMAN KODAK COMPANY".into()));
        meta.exif
            .set("SerialNumber", AttrValue::Str("NEWKODAK".into()));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::KodakParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_str("SerialNumber"), Some("NEWKODAK"));
    }

    #[test]
    fn kodak_kdk_info_not_overlaid_as_ifd() {
        let mut src = Vec::from(b"KDK INFO".as_slice());
        src.extend_from_slice(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Make", AttrValue::Str("Kodak".into()));
        meta.exif.set("SerialNumber", AttrValue::Str("NOPE".into()));
        assert_eq!(rewrite_blob(&src, &meta), src);
    }

    #[test]
    fn motorola_iso_inplace() {
        let src = prefix_ifd(b"", 0x0205, ExifFormat::UInt16, RawValue::UInt16(vec![100]));
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Make", AttrValue::Str("Motorola".into()));
        meta.exif.set("ISO", AttrValue::UInt(200));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::MotorolaParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_u32("ISO"), Some(200));
    }

    #[test]
    fn xiaomi_serial_inplace() {
        let src = prefix_ifd(
            b"",
            0x0100,
            ExifFormat::String,
            RawValue::String("OLDXIAOMI".into()),
        );
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Make", AttrValue::Str("Xiaomi".into()));
        meta.exif
            .set("SerialNumber", AttrValue::Str("NEWXIAOMI".into()));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::XiaomiParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_str("SerialNumber"), Some("NEWXIAOMI"));
    }

    #[test]
    fn google_hdrplus_inplace() {
        let src = prefix_ifd(b"", 0x0002, ExifFormat::UInt16, RawValue::UInt16(vec![0]));
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Make", AttrValue::Str("Google".into()));
        meta.exif.set("HDRPlusUsed", AttrValue::UInt(1));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::GoogleParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_u32("HDRPlusUsed"), Some(1));
    }

    #[test]
    fn phaseone_iso_inplace() {
        let src = prefix_ifd(b"", 0x0106, ExifFormat::UInt16, RawValue::UInt16(vec![100]));
        let mut meta = Metadata::new("IIQ");
        meta.exif.set("Make", AttrValue::Str("Phase One".into()));
        meta.exif.set("ISO", AttrValue::UInt(200));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::PhaseOneParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_u32("ISO"), Some(200));
    }

    #[test]
    fn leica_headerless_iso_inplace() {
        let src = prefix_ifd(b"", 0x0010, ExifFormat::UInt16, RawValue::UInt16(vec![100]));
        let mut meta = Metadata::new("JPG");
        meta.exif
            .set("Make", AttrValue::Str("Leica Camera AG".into()));
        meta.exif.set("ISO", AttrValue::UInt(400));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::LeicaParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_u32("Leica:ISO"), Some(400));
    }

    #[test]
    fn huawei_capturemode_inplace() {
        let src = prefix_ifd(b"", 0x0100, ExifFormat::UInt16, RawValue::UInt16(vec![1]));
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Make", AttrValue::Str("HUAWEI".into()));
        meta.exif.set("CaptureMode", AttrValue::UInt(2));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::HuaweiParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_u32("CaptureMode"), Some(2));
    }

    #[test]
    fn dji_speedx_float_inplace() {
        let src = prefix_ifd(b"", 0x0003, ExifFormat::Float, RawValue::Float(vec![1.5]));
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Make", AttrValue::Str("DJI".into()));
        meta.exif.set("SpeedX", AttrValue::Float(2.25));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::DjiParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        let v = parsed.get_f32("SpeedX").unwrap();
        assert!((v - 2.25).abs() < 1e-6);
        assert_eq!(out.len(), src.len());
    }

    #[test]
    fn gopro_devicename_gpmf_inplace() {
        let mut src = b"GoPro\0".to_vec();
        src.extend_from_slice(b"DVNM");
        src.push(b'c');
        src.push(1);
        src.extend_from_slice(&8u16.to_be_bytes());
        src.extend_from_slice(b"Hero 12\0");
        let mut meta = Metadata::new("JPG");
        meta.exif
            .set("DeviceName", AttrValue::Str("Hero 13".into()));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::GoProParser
            .parse(&out, ByteOrder::BigEndian)
            .unwrap();
        assert_eq!(parsed.get_str("DeviceName"), Some("Hero 13"));
        assert_eq!(out.len(), src.len());
    }

    #[test]
    fn gopro_temperature_gpmf_inplace() {
        let mut src = Vec::new();
        src.extend_from_slice(b"TMPC");
        src.push(b'f');
        src.push(4);
        src.extend_from_slice(&1u16.to_be_bytes());
        src.extend_from_slice(&25.5f32.to_be_bytes());
        let mut meta = Metadata::new("MP4");
        meta.exif.set("Make", AttrValue::Str("GoPro".into()));
        meta.exif.set("Temperature", AttrValue::Float(18.0));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::GoProParser
            .parse(&out, ByteOrder::BigEndian)
            .unwrap();
        let v = parsed.get_f32("Temperature").unwrap();
        assert!((v - 18.0).abs() < 0.01);
        assert_eq!(out.len(), src.len());
    }

    #[test]
    fn hasselblad_iso_inplace() {
        let src = prefix_ifd(b"", 0x0028, ExifFormat::UInt16, RawValue::UInt16(vec![100]));
        let mut meta = Metadata::new("3FR");
        meta.exif.set("Make", AttrValue::Str("Hasselblad".into()));
        meta.exif.set("ISO", AttrValue::UInt(200));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::HasselbladParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_u32("ISO"), Some(200));
    }

    #[test]
    fn kodak_iso_inplace() {
        let src = prefix_ifd(b"", 0x0027, ExifFormat::UInt16, RawValue::UInt16(vec![80]));
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Make", AttrValue::Str("Kodak".into()));
        meta.exif.set("ISO", AttrValue::UInt(200));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::KodakParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_u32("ISO"), Some(200));
    }

    #[test]
    fn casio_type1_whitebalance_inplace() {
        let src = prefix_ifd(b"", 0x0007, ExifFormat::UInt16, RawValue::UInt16(vec![1]));
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Make", AttrValue::Str("CASIO".into()));
        meta.exif.set("WhiteBalance", AttrValue::UInt(2));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::CasioParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_u32("WhiteBalance"), Some(2));
    }

    #[test]
    fn xiaomi_nightmode_inplace() {
        let src = prefix_ifd(b"", 0x0213, ExifFormat::UInt16, RawValue::UInt16(vec![0]));
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Make", AttrValue::Str("Xiaomi".into()));
        meta.exif.set("NightMode", AttrValue::UInt(1));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::XiaomiParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_u32("NightMode"), Some(1));
    }

    #[test]
    fn leica_prefixed_iso_inplace() {
        let src = prefix_ifd(b"", 0x0010, ExifFormat::UInt16, RawValue::UInt16(vec![100]));
        let mut meta = Metadata::new("JPG");
        meta.exif
            .set("Make", AttrValue::Str("Leica Camera AG".into()));
        meta.exif.set("Leica:ISO", AttrValue::UInt(800));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::LeicaParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        assert_eq!(parsed.get_u32("Leica:ISO"), Some(800));
    }

    #[test]
    fn canon_camerasettings_macromode_inplace() {
        let src = prefix_ifd(
            b"",
            1,
            ExifFormat::Undefined,
            RawValue::Undefined(vec![0, 0, 1, 0, 0, 0, 0, 0]),
        );
        let mut group = Attrs::new();
        group.set("MacroMode", AttrValue::Str("Normal".into()));
        let mut meta = Metadata::new("JPG");
        meta.exif.set("Make", AttrValue::Str("Canon".into()));
        meta.exif
            .set("CameraSettings", AttrValue::Group(Box::new(group)));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::CanonParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        let AttrValue::Group(g) = parsed.get("CameraSettings").unwrap() else {
            panic!("expected CameraSettings group");
        };
        assert_eq!(g.get_str("MacroMode"), Some("Normal"));
        assert_eq!(out.len(), src.len());
    }

    #[test]
    fn olympus_equipment_serial_inplace() {
        let mut hdr = b"OLYMPUS\0II".to_vec();
        hdr.extend_from_slice(&[0x2a, 0]);
        let sub_off = 12u32 + 18;
        let main = IfdEntry {
            tag: 0x2010,
            format: ExifFormat::UInt32,
            count: 1,
            value: RawValue::UInt32(vec![sub_off]),
            value_offset: None,
        };
        let main_ifd = emit_ifd(&[main], ByteOrder::LittleEndian, 12).unwrap();
        let eq = IfdEntry {
            tag: 0x0101,
            format: ExifFormat::String,
            count: 8,
            value: RawValue::String("OLDNAME".into()),
            value_offset: None,
        };
        let eq_ifd = emit_ifd(&[eq], ByteOrder::LittleEndian, sub_off).unwrap();
        let mut src = hdr;
        src.extend_from_slice(&main_ifd);
        src.extend_from_slice(&eq_ifd);
        let mut group = Attrs::new();
        group.set("SerialNumber", AttrValue::Str("NEWNAME".into()));
        let mut meta = Metadata::new("ORF");
        meta.exif
            .set("Equipment", AttrValue::Group(Box::new(group)));
        let out = rewrite_blob(&src, &meta);
        let parsed = crate::makernotes::OlympusParser
            .parse(&out, ByteOrder::LittleEndian)
            .unwrap();
        let AttrValue::Group(g) = parsed.get("Equipment").unwrap() else {
            panic!("expected Equipment group");
        };
        assert_eq!(g.get_str("SerialNumber"), Some("NEWNAME"));
        assert_eq!(out.len(), src.len());
    }
}

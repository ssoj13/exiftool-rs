//! TIFF rewrite that preserves image IFDs, strips/tiles, and MakerNotes blobs.
//!
//! Used for TIFF-based RAW (NEF): update IFD0 / ExifIFD / GPS metadata while
//! copying SubIFD trees and pixel payloads. Unlike [`crate::TiffWriter`], this
//! does not rebuild IFD0 from a short tag list.

use crate::{Error, Metadata, Result};
use exiftool_attrs::AttrValue;
use exiftool_core::writer::tags;
use exiftool_core::{ByteOrder, ExifFormat, IfdEntry, IfdReader, RawValue, URational};
use std::collections::HashSet;

const TAG_SUB_IFD: u16 = 0x014A;
const TAG_STRIP_OFFSETS: u16 = 0x0111;
const TAG_STRIP_COUNTS: u16 = 0x0117;
const TAG_TILE_OFFSETS: u16 = 0x0144;
const TAG_TILE_COUNTS: u16 = 0x0145;
const TAG_JPEG_OFF: u16 = 0x0201;
const TAG_JPEG_LEN: u16 = 0x0202;
const TAG_INTEROP: u16 = 0xA005;
const TAG_SR2_PRIVATE: u16 = 0xC634;
const MAX_IFDS: usize = 64;

/// Rewrite `original` TIFF, overlaying writable IFD0/Exif/GPS tags from `metadata`.
///
/// Accepts standard TIFF magic plus ORF (`0x4F52`/`0x5352`) and RW2 (`0x55`).
/// BigTIFF (magic 43) is rewritten with 16-byte header, 20-byte IFD entries, and 8-byte offsets.
/// CR2 16-byte header (`CR\x02`) is kept; IFD0 is rewritten at offset 16 (ExifTool `WriteCR2`).
pub fn rewrite_preserving(original: &[u8], metadata: &Metadata) -> Result<Vec<u8>> {
    rewrite_preserving_kind(original, metadata, IfdKind::Ifd0)
}

/// Overlay `root_kind` on the TIFF root (CR3 CMT1=IFD0, CMT2=Exif, CMT4=GPS).
pub(crate) fn rewrite_preserving_kind(
    original: &[u8],
    metadata: &Metadata,
    root_kind: IfdKind,
) -> Result<Vec<u8>> {
    if original.len() < 8 {
        return Err(Error::InvalidStructure("TIFF too small".into()));
    }
    let order = ByteOrder::from_marker([original[0], original[1]]).map_err(Error::Core)?;
    let magic = [original[2], original[3]];
    let ident = order.read_u16(magic);
    let bigtiff = ident == 43;
    if bigtiff && original.len() < 16 {
        return Err(Error::InvalidStructure("BigTIFF too small".into()));
    }
    let reader = if bigtiff {
        IfdReader::new_bigtiff(original, order)
    } else {
        IfdReader::new(original, order)
    };
    let allowed = [ident, 42, 43, 0x4F52, 0x5352, 0x55];
    let (ifd0_u64, is_bt) = reader
        .parse_header_ex_with_magic(&allowed)
        .map_err(Error::Core)?;
    if is_bt != bigtiff {
        return Err(Error::InvalidStructure(
            "TIFF/BigTIFF header mismatch".into(),
        ));
    }
    if ifd0_u64 > u32::MAX as u64 {
        return Err(Error::InvalidStructure(
            "BigTIFF IFD0 offset exceeds 4GB (file load cap)".into(),
        ));
    }
    let ifd0_off = ifd0_u64 as u32;
    let mut seen = HashSet::new();
    let a100 = detect_a100_mrw(original, &reader, ifd0_u64, order);
    let mut root = parse_ifd_tree(&reader, ifd0_u64, &mut seen, 0, a100.is_some())?;
    overlay_ifd(&mut root, metadata, root_kind);
    apply_child_overlays(&mut root, metadata);
    let mut out = emit_tiff(original, order, magic, root, ifd0_off, bigtiff)?;
    if let Some(layout) = a100 {
        finish_a100_arw(&mut out, original, layout, order)?;
    }
    Ok(out)
}

#[derive(Clone, Copy)]
pub(crate) enum IfdKind {
    Ifd0,
    Exif,
    Gps,
    Other,
}

struct IfdNode {
    entries: Vec<IfdEntry>,
    next: Option<Box<IfdNode>>,
    children: Vec<Option<Vec<IfdNode>>>,
}

fn parse_ifd_tree(
    reader: &IfdReader<'_>,
    offset: u64,
    seen: &mut HashSet<u64>,
    depth: usize,
    skip_a100_subifd: bool,
) -> Result<IfdNode> {
    if depth > 12 || seen.len() >= MAX_IFDS {
        return Err(Error::InvalidStructure("TIFF IFD tree too deep".into()));
    }
    if offset == 0 || !seen.insert(offset) {
        return Err(Error::InvalidStructure("invalid TIFF IFD offset".into()));
    }
    let (entries, next_off) = reader.read_ifd(offset).map_err(Error::Core)?;
    let mut children = Vec::with_capacity(entries.len());
    for e in &entries {
        if is_ifd_pointer(e.tag) && !(skip_a100_subifd && e.tag == TAG_SUB_IFD) {
            let mut kids = Vec::new();
            for off in u32s(&e.value) {
                if off == 0 {
                    continue;
                }
                kids.push(parse_ifd_tree(
                    reader,
                    u64::from(off),
                    seen,
                    depth + 1,
                    skip_a100_subifd,
                )?);
            }
            children.push(Some(kids));
        } else {
            children.push(None);
        }
    }
    let next = if next_off != 0 {
        Some(Box::new(parse_ifd_tree(
            reader,
            next_off,
            seen,
            depth + 1,
            skip_a100_subifd,
        )?))
    } else {
        None
    };
    Ok(IfdNode {
        entries,
        next,
        children,
    })
}

fn apply_child_overlays(node: &mut IfdNode, metadata: &Metadata) {
    for (entry, kids) in node.entries.iter().zip(node.children.iter_mut()) {
        let Some(kids) = kids else {
            continue;
        };
        let kind = match entry.tag {
            tags::EXIF_IFD => IfdKind::Exif,
            tags::GPS_IFD => IfdKind::Gps,
            _ => IfdKind::Other,
        };
        for kid in kids {
            overlay_ifd(kid, metadata, kind);
            apply_child_overlays(kid, metadata);
        }
    }
    if let Some(next) = node.next.as_mut() {
        overlay_ifd(next, metadata, IfdKind::Other);
        apply_child_overlays(next, metadata);
    }
}

fn overlay_ifd(node: &mut IfdNode, metadata: &Metadata, kind: IfdKind) {
    match kind {
        IfdKind::Ifd0 => {
            patch_str(node, tags::MAKE, metadata.exif.get_str("Make"));
            patch_str(node, tags::MODEL, metadata.exif.get_str("Model"));
            patch_str(node, tags::SOFTWARE, metadata.exif.get_str("Software"));
            patch_str(node, tags::DATE_TIME, metadata.exif.get_str("DateTime"));
            patch_str(node, tags::ARTIST, metadata.exif.get_str("Artist"));
            patch_str(node, tags::COPYRIGHT, metadata.exif.get_str("Copyright"));
            patch_str(
                node,
                tags::IMAGE_DESCRIPTION,
                metadata.exif.get_str("ImageDescription"),
            );
            if let Some(AttrValue::UInt(v)) = metadata.exif.get("Orientation") {
                patch_u16(node, tags::ORIENTATION, *v as u16);
            }
            patch_str(
                node,
                tags::DATE_TIME_ORIGINAL,
                metadata.exif.get_str("DateTimeOriginal"),
            );
        }
        IfdKind::Exif => {
            patch_str(
                node,
                tags::DATE_TIME_ORIGINAL,
                metadata.exif.get_str("DateTimeOriginal"),
            );
            patch_str(node, tags::CREATE_DATE, metadata.exif.get_str("CreateDate"));
            if let Some(AttrValue::UInt(v)) = metadata.exif.get("ISO") {
                patch_u16(node, tags::ISO, *v as u16);
            }
            if let Some(AttrValue::URational(n, d)) = metadata.exif.get("ExposureTime") {
                patch_urational(node, tags::EXPOSURE_TIME, *n, *d);
            }
            if let Some(AttrValue::URational(n, d)) = metadata.exif.get("FNumber") {
                patch_urational(node, tags::FNUMBER, *n, *d);
            }
            if let Some(AttrValue::URational(n, d)) = metadata.exif.get("FocalLength") {
                patch_urational(node, tags::FOCAL_LENGTH, *n, *d);
            }
        }
        IfdKind::Gps => {
            patch_str(
                node,
                tags::GPS_LATITUDE_REF,
                metadata.exif.get_str("GPSLatitudeRef"),
            );
            patch_str(
                node,
                tags::GPS_LONGITUDE_REF,
                metadata.exif.get_str("GPSLongitudeRef"),
            );
        }
        IfdKind::Other => {}
    }
}

fn patch_str(node: &mut IfdNode, tag: u16, value: Option<&str>) {
    let Some(s) = value else {
        return;
    };
    let entry = IfdEntry {
        tag,
        format: ExifFormat::String,
        count: (s.len() + 1) as u32,
        value: RawValue::String(s.to_string()),
        value_offset: None,
    };
    if let Some(e) = node.entries.iter_mut().find(|e| e.tag == tag) {
        *e = entry;
        return;
    }
    node.entries.push(entry);
    node.children.push(None);
}

fn patch_u16(node: &mut IfdNode, tag: u16, value: u16) {
    let entry = IfdEntry {
        tag,
        format: ExifFormat::UInt16,
        count: 1,
        value: RawValue::UInt16(vec![value]),
        value_offset: None,
    };
    if let Some(e) = node.entries.iter_mut().find(|e| e.tag == tag) {
        *e = entry;
        return;
    }
    node.entries.push(entry);
    node.children.push(None);
}

fn patch_urational(node: &mut IfdNode, tag: u16, num: u32, den: u32) {
    let entry = IfdEntry {
        tag,
        format: ExifFormat::URational,
        count: 1,
        value: RawValue::URational(vec![URational { num, den }]),
        value_offset: None,
    };
    if let Some(e) = node.entries.iter_mut().find(|e| e.tag == tag) {
        *e = entry;
        return;
    }
    node.entries.push(entry);
    node.children.push(None);
}

fn is_ifd_pointer(tag: u16) -> bool {
    matches!(
        tag,
        TAG_SUB_IFD | tags::EXIF_IFD | tags::GPS_IFD | TAG_INTEROP
    )
}

struct A100Layout {
    mrw_start: u32,
    raw_start: u32,
}

fn entry_as_file_offset(entry: &IfdEntry, order: ByteOrder) -> Option<u32> {
    match &entry.value {
        RawValue::UInt32(v) if v.len() == 1 => Some(v[0]),
        RawValue::UInt16(v) if v.len() == 1 => Some(u32::from(v[0])),
        RawValue::UInt64(v) if v.len() == 1 => u32::try_from(v[0]).ok(),
        RawValue::UInt8(v) if v.len() == 4 => {
            let b: [u8; 4] = v.as_slice().try_into().ok()?;
            Some(match order {
                ByteOrder::LittleEndian => u32::from_le_bytes(b),
                ByteOrder::BigEndian => u32::from_be_bytes(b),
            })
        }
        _ => entry.value.as_u32(),
    }
}

/// Original A100 ARW: IFD0 0x14a is CFA offset (not SubIFD); 0xc634 points at Minolta `\0MR[IM]`.
fn detect_a100_mrw(
    original: &[u8],
    reader: &IfdReader<'_>,
    ifd0: u64,
    order: ByteOrder,
) -> Option<A100Layout> {
    let (entries, _) = reader.read_ifd(ifd0).ok()?;
    let mut raw_start = None;
    let mut mrw_start = None;
    for e in &entries {
        match e.tag {
            TAG_SUB_IFD => raw_start = entry_as_file_offset(e, order),
            TAG_SR2_PRIVATE => mrw_start = entry_as_file_offset(e, order),
            _ => {}
        }
    }
    let mrw_start = mrw_start?;
    let raw_start = raw_start?;
    let mrw = mrw_start as usize;
    if raw_start <= mrw_start || mrw + 4 > original.len() {
        return None;
    }
    let mag = &original[mrw..mrw + 4];
    if mag != b"\0MRI" && mag != b"\0MRM" {
        return None;
    }
    if raw_start as usize > original.len() {
        return None;
    }
    Some(A100Layout {
        mrw_start,
        raw_start,
    })
}

/// ExifTool `Sony::FinishARW`: 4-byte-align, append MRW directory, then CFA trailer; patch 0xc634 / 0x14a.
fn finish_a100_arw(
    out: &mut Vec<u8>,
    original: &[u8],
    layout: A100Layout,
    order: ByteOrder,
) -> Result<()> {
    let mrw0 = layout.mrw_start as usize;
    let raw0 = layout.raw_start as usize;
    if raw0 > original.len() || mrw0 >= raw0 {
        return Err(Error::InvalidStructure("A100 MRW range invalid".into()));
    }
    let mut mrw = original[mrw0..raw0].to_vec();
    if mrw.len() % 4 != 0 {
        mrw.resize(mrw.len() + (4 - mrw.len() % 4), 0);
    }
    let trailer = &original[raw0..];
    let remain = out.len() % 4;
    if remain != 0 {
        out.resize(out.len() + (4 - remain), 0);
    }
    let mrw_at = u32::try_from(out.len())
        .map_err(|_| Error::InvalidStructure("A100 MRW offset exceeds 4GB".into()))?;
    out.extend_from_slice(&mrw);
    let raw_at = u32::try_from(out.len())
        .map_err(|_| Error::InvalidStructure("A100 CFA offset exceeds 4GB".into()))?;
    out.extend_from_slice(trailer);
    patch_ifd0_u32(out, order, TAG_SR2_PRIVATE, mrw_at)?;
    patch_ifd0_u32(out, order, TAG_SUB_IFD, raw_at)?;
    Ok(())
}

fn patch_ifd0_u32(out: &mut [u8], order: ByteOrder, tag: u16, value: u32) -> Result<()> {
    if out.len() < 8 {
        return Err(Error::InvalidStructure("TIFF too small to patch".into()));
    }
    let ifd = match order {
        ByteOrder::LittleEndian => u32::from_le_bytes(out[4..8].try_into().unwrap()),
        ByteOrder::BigEndian => u32::from_be_bytes(out[4..8].try_into().unwrap()),
    } as usize;
    if ifd + 2 > out.len() {
        return Err(Error::InvalidStructure(
            "IFD0 past EOF on A100 patch".into(),
        ));
    }
    let n = match order {
        ByteOrder::LittleEndian => u16::from_le_bytes(out[ifd..ifd + 2].try_into().unwrap()),
        ByteOrder::BigEndian => u16::from_be_bytes(out[ifd..ifd + 2].try_into().unwrap()),
    } as usize;
    for i in 0..n {
        let pos = ifd + 2 + i * 12;
        if pos + 12 > out.len() {
            break;
        }
        let t = match order {
            ByteOrder::LittleEndian => u16::from_le_bytes(out[pos..pos + 2].try_into().unwrap()),
            ByteOrder::BigEndian => u16::from_be_bytes(out[pos..pos + 2].try_into().unwrap()),
        };
        if t == tag {
            put_u32(out, pos + 8, value, order);
            return Ok(());
        }
    }
    Err(Error::InvalidStructure(format!(
        "A100 IFD0 missing tag {tag:#06x} to patch"
    )))
}

fn u32s(v: &RawValue) -> Vec<u32> {
    match v {
        RawValue::UInt16(x) => x.iter().map(|&n| u32::from(n)).collect(),
        RawValue::UInt32(x) => x.clone(),
        RawValue::UInt64(x) => x.iter().filter_map(|&n| u32::try_from(n).ok()).collect(),
        _ => v.as_u32_vec().unwrap_or_default(),
    }
}

fn copy_blob(original: &[u8], offset: u32, len: u32) -> Result<Vec<u8>> {
    let start = offset as usize;
    let end = start.saturating_add(len as usize);
    if end > original.len() {
        return Err(Error::InvalidStructure(format!(
            "TIFF blob {offset}+{len} past EOF {}",
            original.len()
        )));
    }
    Ok(original[start..end].to_vec())
}

enum Body {
    Bytes(Vec<u8>),
    Image(Vec<Vec<u8>>),
    Ifds(Vec<PrepIfd>),
}

struct Slot {
    tag: u16,
    format: ExifFormat,
    count: u32,
    body: Body,
    data_at: u32,
    /// Per-strip offsets for `Image` parts.
    part_at: Vec<u32>,
    /// Padding after each strip/tile (ExifTool PreserveImagePadding).
    part_pad: Vec<Vec<u8>>,
}

struct PrepIfd {
    slots: Vec<Slot>,
    next: Option<Box<PrepIfd>>,
    dir_at: u32,
}

fn prepare(node: IfdNode, original: &[u8], order: ByteOrder) -> Result<PrepIfd> {
    let strip_off = find_u32s(&node.entries, TAG_STRIP_OFFSETS);
    let strip_len = find_u32s(&node.entries, TAG_STRIP_COUNTS);
    let tile_off = find_u32s(&node.entries, TAG_TILE_OFFSETS);
    let tile_len = find_u32s(&node.entries, TAG_TILE_COUNTS);
    let jpeg_off = find_u32s(&node.entries, TAG_JPEG_OFF);
    let jpeg_len = find_u32s(&node.entries, TAG_JPEG_LEN);

    let mut pairs: Vec<(IfdEntry, Option<Vec<IfdNode>>)> =
        node.entries.into_iter().zip(node.children).collect();
    pairs.sort_by_key(|(e, _)| e.tag);

    let mut slots = Vec::with_capacity(pairs.len());
    for (entry, kids) in pairs {
        if let Some(kids) = kids {
            let mut prepared = Vec::with_capacity(kids.len());
            for k in kids {
                prepared.push(prepare(k, original, order)?);
            }
            slots.push(Slot {
                tag: entry.tag,
                format: ExifFormat::UInt32,
                count: prepared.len() as u32,
                body: Body::Ifds(prepared),
                data_at: 0,
                part_at: Vec::new(),
                part_pad: Vec::new(),
            });
            continue;
        }
        let (body, part_pad) = if entry.tag == TAG_STRIP_OFFSETS
            && !strip_off.is_empty()
            && strip_off.len() == strip_len.len()
        {
            (
                Body::Image(copy_parts(original, &strip_off, &strip_len)?),
                pads_after(original, &strip_off, &strip_len),
            )
        } else if entry.tag == TAG_TILE_OFFSETS
            && !tile_off.is_empty()
            && tile_off.len() == tile_len.len()
        {
            (
                Body::Image(copy_parts(original, &tile_off, &tile_len)?),
                pads_after(original, &tile_off, &tile_len),
            )
        } else if entry.tag == TAG_JPEG_OFF && jpeg_off.len() == 1 && jpeg_len.len() == 1 {
            (
                Body::Image(vec![copy_blob(original, jpeg_off[0], jpeg_len[0])?]),
                Vec::new(),
            )
        } else {
            (Body::Bytes(encode_value(&entry.value, order)?), Vec::new())
        };
        slots.push(Slot {
            tag: entry.tag,
            format: entry.format,
            count: entry.count,
            body,
            data_at: 0,
            part_at: Vec::new(),
            part_pad,
        });
    }
    let next = match node.next {
        Some(n) => Some(Box::new(prepare(*n, original, order)?)),
        None => None,
    };
    Ok(PrepIfd {
        slots,
        next,
        dir_at: 0,
    })
}

fn find_u32s(entries: &[IfdEntry], tag: u16) -> Vec<u32> {
    entries
        .iter()
        .find(|e| e.tag == tag)
        .map(|e| u32s(&e.value))
        .unwrap_or_default()
}

fn copy_parts(original: &[u8], offs: &[u32], lens: &[u32]) -> Result<Vec<Vec<u8>>> {
    offs.iter()
        .zip(lens.iter())
        .map(|(&o, &l)| copy_blob(original, o, l))
        .collect()
}

fn pads_after(original: &[u8], offs: &[u32], lens: &[u32]) -> Vec<Vec<u8>> {
    let n = offs.len();
    let mut pads = vec![Vec::new(); n];
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&i| offs[i]);
    for w in order.windows(2) {
        let i = w[0];
        let j = w[1];
        let end = offs[i] as usize + lens[i] as usize;
        let next = offs[j] as usize;
        if next > end && next <= original.len() {
            pads[i] = original[end..next].to_vec();
        }
    }
    pads
}

fn encode_value(value: &RawValue, order: ByteOrder) -> Result<Vec<u8>> {
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
                buf.extend_from_slice(&u32_bytes(n.to_bits(), order));
            }
        }
        RawValue::Double(v) => {
            for n in v {
                buf.extend_from_slice(&u64_bytes(n.to_bits(), order));
            }
        }
        RawValue::UInt64(v) => {
            for n in v {
                buf.extend_from_slice(&u64_bytes(*n, order));
            }
        }
        RawValue::Int64(v) => {
            for n in v {
                buf.extend_from_slice(&i64_bytes(*n, order));
            }
        }
    }
    Ok(buf)
}

fn u16_bytes(n: u16, order: ByteOrder) -> [u8; 2] {
    match order {
        ByteOrder::LittleEndian => n.to_le_bytes(),
        ByteOrder::BigEndian => n.to_be_bytes(),
    }
}
fn u32_bytes(n: u32, order: ByteOrder) -> [u8; 4] {
    match order {
        ByteOrder::LittleEndian => n.to_le_bytes(),
        ByteOrder::BigEndian => n.to_be_bytes(),
    }
}
fn i16_bytes(n: i16, order: ByteOrder) -> [u8; 2] {
    match order {
        ByteOrder::LittleEndian => n.to_le_bytes(),
        ByteOrder::BigEndian => n.to_be_bytes(),
    }
}
fn i32_bytes(n: i32, order: ByteOrder) -> [u8; 4] {
    match order {
        ByteOrder::LittleEndian => n.to_le_bytes(),
        ByteOrder::BigEndian => n.to_be_bytes(),
    }
}
fn u64_bytes(n: u64, order: ByteOrder) -> [u8; 8] {
    match order {
        ByteOrder::LittleEndian => n.to_le_bytes(),
        ByteOrder::BigEndian => n.to_be_bytes(),
    }
}
fn i64_bytes(n: i64, order: ByteOrder) -> [u8; 8] {
    match order {
        ByteOrder::LittleEndian => n.to_le_bytes(),
        ByteOrder::BigEndian => n.to_be_bytes(),
    }
}

fn align2(cur: &mut u32) {
    if *cur % 2 == 1 {
        *cur += 1;
    }
}

fn ifd_dir_bytes(n: usize, bigtiff: bool) -> u32 {
    if bigtiff {
        8 + 20 * n as u32 + 8
    } else {
        2 + 12 * n as u32 + 4
    }
}

fn inline_max(bigtiff: bool) -> usize {
    if bigtiff {
        8
    } else {
        4
    }
}

fn ptr_width(bigtiff: bool) -> u32 {
    if bigtiff {
        8
    } else {
        4
    }
}

fn layout_dirs(ifd: &mut PrepIfd, cur: &mut u32, bigtiff: bool) {
    ifd.dir_at = *cur;
    *cur += ifd_dir_bytes(ifd.slots.len(), bigtiff);
    let inline = inline_max(bigtiff);
    let ptr = ptr_width(bigtiff);
    for slot in &mut ifd.slots {
        match &mut slot.body {
            Body::Bytes(b) if b.len() > inline => {
                align2(cur);
                slot.data_at = *cur;
                *cur += b.len() as u32;
            }
            Body::Ifds(kids) => {
                if kids.len() > 1 {
                    align2(cur);
                    slot.data_at = *cur;
                    *cur += kids.len() as u32 * ptr;
                }
                for k in kids {
                    layout_dirs(k, cur, bigtiff);
                }
            }
            _ => {}
        }
    }
    if let Some(next) = ifd.next.as_mut() {
        layout_dirs(next, cur, bigtiff);
    }
}

fn layout_images(ifd: &mut PrepIfd, cur: &mut u32, bigtiff: bool) {
    let ptr = ptr_width(bigtiff);
    for slot in &mut ifd.slots {
        match &mut slot.body {
            Body::Image(parts) => {
                slot.part_at.clear();
                for (i, p) in parts.iter().enumerate() {
                    slot.part_at.push(*cur);
                    *cur += p.len() as u32;
                    if let Some(pad) = slot.part_pad.get(i) {
                        *cur += pad.len() as u32;
                    }
                }
                if slot.part_at.len() == 1 {
                    slot.data_at = slot.part_at[0];
                } else if slot.part_at.len() > 1 {
                    align2(cur);
                    slot.data_at = *cur;
                    *cur += slot.part_at.len() as u32 * ptr;
                }
            }
            Body::Ifds(kids) => {
                for k in kids {
                    layout_images(k, cur, bigtiff);
                }
            }
            _ => {}
        }
    }
    if let Some(next) = ifd.next.as_mut() {
        layout_images(next, cur, bigtiff);
    }
}

fn emit_tiff(
    original: &[u8],
    order: ByteOrder,
    magic: [u8; 2],
    root: IfdNode,
    ifd0_off: u32,
    bigtiff: bool,
) -> Result<Vec<u8>> {
    let cr2 = original.len() >= 16 && original[8] == b'C' && original[9] == b'R';
    let layout_start = if cr2 {
        16u32
    } else if bigtiff {
        ifd0_off.max(16)
    } else {
        ifd0_off.max(8)
    };
    let mut prep = prepare(root, original, order)?;
    let mut cur = layout_start;
    layout_dirs(&mut prep, &mut cur, bigtiff);
    layout_images(&mut prep, &mut cur, bigtiff);
    let mut out = vec![0u8; cur as usize];
    let prefix = layout_start as usize;
    if prefix > original.len() {
        return Err(Error::InvalidStructure("TIFF IFD0 past EOF".into()));
    }
    out[..prefix].copy_from_slice(&original[..prefix]);
    out[0..2].copy_from_slice(match order {
        ByteOrder::LittleEndian => b"II",
        ByteOrder::BigEndian => b"MM",
    });
    out[2..4].copy_from_slice(&magic);
    if bigtiff {
        put_u16(&mut out, 4, 8, order);
        put_u16(&mut out, 6, 0, order);
        put_u64(&mut out, 8, u64::from(prep.dir_at), order);
    } else {
        put_u32(&mut out, 4, prep.dir_at, order);
        if cr2 {
            put_u32(&mut out, 12, last_ifd_at(&prep), order);
        }
    }
    write_ifd(&mut out, &prep, order, bigtiff)?;
    Ok(out)
}

fn last_ifd_at(ifd: &PrepIfd) -> u32 {
    let mut p = ifd;
    while let Some(n) = p.next.as_ref() {
        p = n;
    }
    p.dir_at
}

fn write_ifd(out: &mut [u8], ifd: &PrepIfd, order: ByteOrder, bigtiff: bool) -> Result<()> {
    let n = ifd.slots.len();
    if bigtiff {
        put_u64(out, ifd.dir_at as usize, n as u64, order);
    } else {
        put_u16(out, ifd.dir_at as usize, n as u16, order);
    }
    let entry_size = if bigtiff { 20 } else { 12 };
    let dir_hdr = if bigtiff { 8 } else { 2 };
    for (i, slot) in ifd.slots.iter().enumerate() {
        let pos = ifd.dir_at as usize + dir_hdr + i * entry_size;
        put_u16(out, pos, slot.tag, order);
        let (format, count, field, extra) = slot_field(slot, order, bigtiff)?;
        put_u16(out, pos + 2, format as u16, order);
        if bigtiff {
            put_u64(out, pos + 4, u64::from(count), order);
            out[pos + 12..pos + 20].copy_from_slice(&field);
        } else {
            put_u32(out, pos + 4, count, order);
            out[pos + 8..pos + 12].copy_from_slice(&field);
        }
        if let Some((at, bytes)) = extra {
            let a = at as usize;
            out[a..a + bytes.len()].copy_from_slice(&bytes);
        }
        match &slot.body {
            Body::Ifds(kids) => {
                for k in kids {
                    write_ifd(out, k, order, bigtiff)?;
                }
            }
            Body::Image(parts) => {
                for (i, (p, &at)) in parts.iter().zip(slot.part_at.iter()).enumerate() {
                    let a = at as usize;
                    out[a..a + p.len()].copy_from_slice(p);
                    if let Some(pad) = slot.part_pad.get(i) {
                        let s = a + p.len();
                        out[s..s + pad.len()].copy_from_slice(pad);
                    }
                }
            }
            Body::Bytes(_) => {}
        }
    }
    let next_at = ifd.dir_at as usize + dir_hdr + n * entry_size;
    let next_off = ifd.next.as_ref().map(|n| n.dir_at).unwrap_or(0);
    if bigtiff {
        put_u64(out, next_at, u64::from(next_off), order);
    } else {
        put_u32(out, next_at, next_off, order);
    }
    if let Some(n) = ifd.next.as_ref() {
        write_ifd(out, n, order, bigtiff)?;
    }
    Ok(())
}

fn slot_field(
    slot: &Slot,
    order: ByteOrder,
    bigtiff: bool,
) -> Result<(ExifFormat, u32, Vec<u8>, Option<(u32, Vec<u8>)>)> {
    match &slot.body {
        Body::Ifds(kids) => {
            if bigtiff {
                let offs: Vec<u64> = kids.iter().map(|k| u64::from(k.dir_at)).collect();
                let bytes = encode_u64_slice(&offs, order);
                pack_field(
                    ExifFormat::UInt64,
                    offs.len() as u32,
                    &bytes,
                    slot.data_at,
                    order,
                    true,
                )
            } else {
                let offs: Vec<u32> = kids.iter().map(|k| k.dir_at).collect();
                let bytes = encode_u32_slice(&offs, order);
                pack_field(
                    ExifFormat::UInt32,
                    offs.len() as u32,
                    &bytes,
                    slot.data_at,
                    order,
                    false,
                )
            }
        }
        Body::Image(_) => {
            if bigtiff {
                let offs: Vec<u64> = slot.part_at.iter().copied().map(u64::from).collect();
                let bytes = encode_u64_slice(&offs, order);
                pack_field(
                    ExifFormat::UInt64,
                    offs.len() as u32,
                    &bytes,
                    slot.data_at,
                    order,
                    true,
                )
            } else {
                let bytes = encode_u32_slice(&slot.part_at, order);
                pack_field(
                    ExifFormat::UInt32,
                    slot.part_at.len() as u32,
                    &bytes,
                    slot.data_at,
                    order,
                    false,
                )
            }
        }
        Body::Bytes(b) => pack_field(slot.format, slot.count, b, slot.data_at, order, bigtiff),
    }
}

fn encode_u32_slice(v: &[u32], order: ByteOrder) -> Vec<u8> {
    let mut b = Vec::with_capacity(v.len() * 4);
    for n in v {
        b.extend_from_slice(&u32_bytes(*n, order));
    }
    b
}

fn encode_u64_slice(v: &[u64], order: ByteOrder) -> Vec<u8> {
    let mut b = Vec::with_capacity(v.len() * 8);
    for n in v {
        b.extend_from_slice(&u64_bytes(*n, order));
    }
    b
}

fn pack_field(
    format: ExifFormat,
    count: u32,
    bytes: &[u8],
    data_at: u32,
    order: ByteOrder,
    bigtiff: bool,
) -> Result<(ExifFormat, u32, Vec<u8>, Option<(u32, Vec<u8>)>)> {
    let inline = inline_max(bigtiff);
    let mut field = vec![0u8; inline];
    if bytes.len() <= inline {
        field[..bytes.len()].copy_from_slice(bytes);
        Ok((format, count, field, None))
    } else if bigtiff {
        field.copy_from_slice(&u64_bytes(u64::from(data_at), order));
        Ok((format, count, field, Some((data_at, bytes.to_vec()))))
    } else {
        field.copy_from_slice(&u32_bytes(data_at, order));
        Ok((format, count, field, Some((data_at, bytes.to_vec()))))
    }
}

fn put_u16(out: &mut [u8], at: usize, v: u16, order: ByteOrder) {
    out[at..at + 2].copy_from_slice(&u16_bytes(v, order));
}

fn put_u32(out: &mut [u8], at: usize, v: u32, order: ByteOrder) {
    out[at..at + 4].copy_from_slice(&u32_bytes(v, order));
}

fn put_u64(out: &mut [u8], at: usize, v: u64, order: ByteOrder) {
    out[at..at + 8].copy_from_slice(&u64_bytes(v, order));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FormatParser, NefParser};
    use std::io::Cursor;
    use std::path::PathBuf;

    fn fixture() -> Option<Vec<u8>> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/Nikon.nef");
        path.exists().then(|| std::fs::read(path).unwrap())
    }

    #[test]
    fn nef_rewrite_keeps_subifd_and_sets_artist() {
        let Some(data) = fixture() else {
            return;
        };
        let mut meta = NefParser::new().parse(&mut Cursor::new(&data)).unwrap();
        meta.exif
            .set("Artist", AttrValue::Str("exiftool-rs".into()));
        let out = rewrite_preserving(&data, &meta).unwrap();
        assert!(out.len() > 1000, "rewritten NEF too small: {}", out.len());
        let parsed = NefParser::new().parse(&mut Cursor::new(&out)).unwrap();
        assert_eq!(parsed.exif.get_str("Artist"), Some("exiftool-rs"));
        assert_eq!(parsed.exif.get_str("Make"), Some("NIKON CORPORATION"));
        assert_eq!(
            parsed.exif.get("SubIFD:ImageWidth").map(|v| v.to_string()),
            Some("3040".into())
        );
        assert_eq!(parsed.exif.get_str("LensDataVersion"), Some("0101"));
    }

    #[test]
    fn orf_magic_survives_rewrite() {
        let Some(data) = fixture() else {
            return;
        };
        let mut orf = data.clone();
        orf[2] = b'R';
        orf[3] = b'O';
        let ident = u16::from_le_bytes([orf[2], orf[3]]);
        let reader = exiftool_core::IfdReader::new(&orf, exiftool_core::ByteOrder::LittleEndian);
        reader
            .parse_header_with_magic(&[ident])
            .unwrap_or_else(|e| panic!("header ident={ident:#x} err={e:?}"));
        let meta = NefParser::new().parse(&mut Cursor::new(&data)).unwrap();
        let out = rewrite_preserving(&orf, &meta).unwrap();
        assert_eq!(&out[0..4], b"IIRO");
        assert!(crate::OrfParser::new().can_parse(&out));
        let _ = crate::OrfParser::new()
            .parse(&mut Cursor::new(&out))
            .unwrap();
    }

    fn minimal_bigtiff() -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&[0x49, 0x49, 0x2B, 0x00, 0x08, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&[0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&[0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&[
            0x00, 0x01, 0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        data.extend_from_slice(&[0x80, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&[
            0x01, 0x01, 0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        data.extend_from_slice(&[0x38, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        data
    }

    #[test]
    fn bigtiff_rewrite_sets_artist_and_keeps_size() {
        let data = minimal_bigtiff();
        let mut meta = crate::TiffParser::default()
            .parse(&mut Cursor::new(&data))
            .unwrap();
        assert_eq!(meta.format, "BigTIFF");
        meta.exif
            .set("Artist", AttrValue::Str("exiftool-rs".into()));
        let out = rewrite_preserving(&data, &meta).unwrap();
        assert_eq!(&out[0..4], &[0x49, 0x49, 0x2B, 0x00]);
        assert_eq!(&out[4..8], &[0x08, 0x00, 0x00, 0x00]);
        let parsed = crate::TiffParser::default()
            .parse(&mut Cursor::new(&out))
            .unwrap();
        assert_eq!(parsed.format, "BigTIFF");
        assert_eq!(parsed.exif.get_str("Artist"), Some("exiftool-rs"));
        assert_eq!(parsed.exif.get_u32("ImageWidth"), Some(1920));
        assert_eq!(parsed.exif.get_u32("ImageHeight"), Some(1080));
    }

    fn a100_fixture() -> Option<Vec<u8>> {
        let path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/RAW_SONY_A100.ARW");
        path.exists().then(|| std::fs::read(path).unwrap())
    }

    #[test]
    fn a100_finish_arw_keeps_mrw_and_cfa() {
        let Some(data) = a100_fixture() else {
            panic!("missing testdata/RAW_SONY_A100.ARW (rawsamples.ch Sony A100)");
        };
        let mut meta = crate::TiffParser::default()
            .parse(&mut Cursor::new(&data))
            .unwrap();
        assert_eq!(meta.exif.get_str("Model"), Some("DSLR-A100"));
        meta.exif
            .set("Artist", AttrValue::Str("exiftool-rs".into()));
        let out = rewrite_preserving(&data, &meta).unwrap();
        assert!(out.len() > data.len() - 1024, "rewritten A100 lost trailer");
        let parsed = crate::TiffParser::default()
            .parse(&mut Cursor::new(&out))
            .unwrap();
        assert_eq!(parsed.exif.get_str("Artist"), Some("exiftool-rs"));
        assert_eq!(parsed.exif.get_str("Model"), Some("DSLR-A100"));
        let order = exiftool_core::ByteOrder::LittleEndian;
        let orig_reader = exiftool_core::IfdReader::new(&data, order);
        let orig_ifd0 = orig_reader.parse_header().unwrap() as u64;
        let orig_layout = detect_a100_mrw(&data, &orig_reader, orig_ifd0, order).unwrap();
        let reader = exiftool_core::IfdReader::new(&out, order);
        let ifd0 = reader.parse_header().unwrap() as u64;
        let layout = detect_a100_mrw(&out, &reader, ifd0, order)
            .expect("rewritten file must still be original-style A100 MRW");
        let mrw = layout.mrw_start as usize;
        assert_eq!(&out[mrw..mrw + 4], b"\0MRI");
        let raw = layout.raw_start as usize;
        assert_eq!(&out[raw..], &data[orig_layout.raw_start as usize..]);
    }
}

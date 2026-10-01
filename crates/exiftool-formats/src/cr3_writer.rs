//! Canon CR3 writer (ExifTool WriteQuickTime CR3 map).
//!
//! Rewrites CMT1/CMT2/CMT4 TIFF boxes inside the Canon UUID, optional XMP UUID,
//! then patches CTBO and stco/co64 after `mdat` moves.

use crate::tiff_rewrite::{rewrite_preserving_kind, IfdKind};
use crate::{Error, Metadata, ReadSeek, Result};
use std::io::Write;

const XMP_UUID: [u8; 16] = [
    0xBE, 0x7A, 0xCF, 0xCB, 0x97, 0xA9, 0x42, 0xE8, 0x9C, 0x71, 0x99, 0x94, 0x91, 0xE3, 0xAF, 0xAC,
];
const PREVIEW_UUID: [u8; 16] = [
    0xEA, 0xF4, 0x2B, 0x5E, 0x1C, 0x98, 0x4B, 0x88, 0xB9, 0xFB, 0xB7, 0xDC, 0x40, 0x6E, 0x4D, 0x16,
];

/// Canon CR3 writer.
pub struct Cr3Writer;

impl Cr3Writer {
    /// Write CR3 with updated IFD0/Exif/GPS (CMT1/2/4) and optional XMP UUID.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        let data = crate::utils::read_with_limit(input)?;
        if data.len() < 12 || &data[4..8] != b"ftyp" {
            return Err(Error::InvalidStructure("Invalid CR3 file".into()));
        }
        let old_mdat = find_mdat_start(&data)?;
        let nodes = parse_top(&data)?;
        let rebuilt = rebuild(&nodes, metadata)?;
        let mut out = serialize_all(&rebuilt)?;
        let new_mdat = find_mdat_start(&out)?;
        let delta = new_mdat as i64 - old_mdat as i64;
        patch_chunk_offsets(&mut out, old_mdat, delta)?;
        patch_ctbo(&mut out)?;
        output.write_all(&out)?;
        Ok(())
    }
}

#[derive(Clone)]
enum Node {
    Leaf {
        typ: [u8; 4],
        payload: Vec<u8>,
        /// ISOBMFF `size==1` 16-byte header. `mdat` often uses this; shrinking to 8 bytes moves media vs co64.
        largesize: bool,
    },
    Container {
        typ: [u8; 4],
        children: Vec<Node>,
    },
    Uuid {
        uuid: [u8; 16],
        children: Option<Vec<Node>>,
        payload: Vec<u8>,
    },
}

fn parse_top(data: &[u8]) -> Result<Vec<Node>> {
    parse_seq(data, 0, data.len())
}

fn parse_seq(data: &[u8], mut pos: usize, end: usize) -> Result<Vec<Node>> {
    let mut out = Vec::new();
    while pos + 8 <= end {
        let (size, hdr) = box_header(data, pos, end)?;
        if size < hdr || pos + size > end {
            break;
        }
        let typ = [data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]];
        let body_at = pos + hdr;
        let body_end = pos + size;
        out.push(parse_one(data, typ, hdr, body_at, body_end)?);
        pos += size;
    }
    Ok(out)
}

fn parse_one(
    data: &[u8],
    typ: [u8; 4],
    hdr: usize,
    body_at: usize,
    body_end: usize,
) -> Result<Node> {
    if typ == *b"uuid" {
        if body_end < body_at + 16 {
            return Err(Error::InvalidStructure("Truncated CR3 uuid".into()));
        }
        let mut uuid = [0u8; 16];
        uuid.copy_from_slice(&data[body_at..body_at + 16]);
        let rest_at = body_at + 16;
        if nested_boxes(data, rest_at, body_end) {
            let children = parse_seq(data, rest_at, body_end)?;
            return Ok(Node::Uuid {
                uuid,
                children: Some(children),
                payload: Vec::new(),
            });
        }
        return Ok(Node::Uuid {
            uuid,
            children: None,
            payload: data[rest_at..body_end].to_vec(),
        });
    }
    if is_container(&typ) {
        let children = parse_seq(data, body_at, body_end)?;
        return Ok(Node::Container { typ, children });
    }
    Ok(Node::Leaf {
        typ,
        payload: data[body_at..body_end].to_vec(),
        largesize: hdr == 16,
    })
}

fn is_container(typ: &[u8; 4]) -> bool {
    matches!(typ, b"moov" | b"trak" | b"mdia" | b"minf" | b"stbl")
}

fn nested_boxes(data: &[u8], start: usize, end: usize) -> bool {
    if start + 8 > end {
        return false;
    }
    let Ok((size, hdr)) = box_header(data, start, end) else {
        return false;
    };
    if size < hdr || start + size > end {
        return false;
    }
    // Canon UUID children only. Do not treat XMP/preview payloads as ISO boxes.
    matches!(
        &data[start + 4..start + 8],
        b"CNCV" | b"CCTP" | b"CTBO" | b"CMT1" | b"CMT2" | b"CMT3" | b"CMT4" | b"THMB" | b"free"
    )
}

fn box_header(data: &[u8], pos: usize, end: usize) -> Result<(usize, usize)> {
    if pos + 8 > end || pos + 8 > data.len() {
        return Err(Error::InvalidStructure("Truncated CR3 box".into()));
    }
    let size32 = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
    if size32 == 1 {
        if pos + 16 > end || pos + 16 > data.len() {
            return Err(Error::InvalidStructure("Truncated CR3 largesize".into()));
        }
        let size = u64::from_be_bytes([
            data[pos + 8],
            data[pos + 9],
            data[pos + 10],
            data[pos + 11],
            data[pos + 12],
            data[pos + 13],
            data[pos + 14],
            data[pos + 15],
        ]) as usize;
        Ok((size, 16))
    } else if size32 == 0 {
        Ok((end - pos, 8))
    } else {
        Ok((size32 as usize, 8))
    }
}

fn rebuild(nodes: &[Node], metadata: &Metadata) -> Result<Vec<Node>> {
    nodes.iter().map(|n| rebuild_node(n, metadata)).collect()
}

fn rebuild_node(node: &Node, metadata: &Metadata) -> Result<Node> {
    match node {
        Node::Leaf {
            typ,
            payload,
            largesize,
        } => {
            let kind = match typ {
                b"CMT1" => Some(IfdKind::Ifd0),
                b"CMT2" => Some(IfdKind::Exif),
                b"CMT4" => Some(IfdKind::Gps),
                _ => None,
            };
            if let Some(kind) = kind {
                if payload.len() >= 8 {
                    if let Ok(new_tiff) = rewrite_preserving_kind(payload, metadata, kind) {
                        return Ok(Node::Leaf {
                            typ: *typ,
                            payload: new_tiff,
                            largesize: *largesize,
                        });
                    }
                }
            }
            Ok(Node::Leaf {
                typ: *typ,
                payload: payload.clone(),
                largesize: *largesize,
            })
        }
        Node::Container { typ, children } => Ok(Node::Container {
            typ: *typ,
            children: rebuild(children, metadata)?,
        }),
        Node::Uuid {
            uuid,
            children,
            payload,
        } => {
            if uuid == &XMP_UUID {
                if let Some(xmp) = metadata.xmp.as_deref().filter(|s| !s.trim().is_empty()) {
                    return Ok(Node::Uuid {
                        uuid: *uuid,
                        children: None,
                        payload: xmp.as_bytes().to_vec(),
                    });
                }
            }
            Ok(Node::Uuid {
                uuid: *uuid,
                children: match children {
                    Some(c) => Some(rebuild(c, metadata)?),
                    None => None,
                },
                payload: payload.clone(),
            })
        }
    }
}

fn serialize_all(nodes: &[Node]) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    for n in nodes {
        serialize_node(n, &mut out)?;
    }
    Ok(out)
}

fn serialize_node(node: &Node, out: &mut Vec<u8>) -> Result<()> {
    match node {
        Node::Leaf {
            typ,
            payload,
            largesize,
        } => write_box(out, typ, payload, *largesize),
        Node::Container { typ, children } => {
            let mut body = Vec::new();
            for c in children {
                serialize_node(c, &mut body)?;
            }
            write_box(out, typ, &body, false)
        }
        Node::Uuid {
            uuid,
            children,
            payload,
        } => {
            let mut body = Vec::from(*uuid);
            if let Some(ch) = children {
                for c in ch {
                    serialize_node(c, &mut body)?;
                }
            } else {
                body.extend_from_slice(payload);
            }
            write_box(out, b"uuid", &body, false)
        }
    }
}

fn write_box(out: &mut Vec<u8>, typ: &[u8; 4], payload: &[u8], largesize: bool) -> Result<()> {
    let hdr = if largesize { 16usize } else { 8usize };
    let total = hdr.saturating_add(payload.len());
    if largesize || u32::try_from(total).is_err() {
        let total64 = u64::try_from(total)
            .map_err(|_| Error::InvalidStructure("CR3 box exceeds u64".into()))?;
        out.extend_from_slice(&1u32.to_be_bytes());
        out.extend_from_slice(typ);
        out.extend_from_slice(&total64.to_be_bytes());
        out.extend_from_slice(payload);
        return Ok(());
    }
    let size =
        u32::try_from(total).map_err(|_| Error::InvalidStructure("CR3 box exceeds 4GB".into()))?;
    out.extend_from_slice(&size.to_be_bytes());
    out.extend_from_slice(typ);
    out.extend_from_slice(payload);
    Ok(())
}

fn find_mdat_start(data: &[u8]) -> Result<u64> {
    let mut pos = 0usize;
    while pos + 8 <= data.len() {
        let (size, hdr) = box_header(data, pos, data.len())?;
        if size < hdr || pos + size > data.len() {
            break;
        }
        if &data[pos + 4..pos + 8] == b"mdat" {
            return Ok(pos as u64);
        }
        pos += size;
    }
    Err(Error::InvalidStructure("CR3 missing mdat".into()))
}

fn patch_chunk_offsets(data: &mut [u8], old_mdat: u64, delta: i64) -> Result<()> {
    if delta == 0 {
        return Ok(());
    }
    walk_mut(data, 0, data.len(), &mut |typ, payload| {
        if typ != *b"stco" && typ != *b"co64" {
            return Ok(());
        }
        if payload.len() < 8 {
            return Ok(());
        }
        let n = u32::from_be_bytes([payload[4], payload[5], payload[6], payload[7]]) as usize;
        if typ == *b"stco" {
            if payload.len() < 8 + n * 4 {
                return Err(Error::InvalidStructure("Truncated stco".into()));
            }
            for i in 0..n {
                let at = 8 + i * 4;
                let val = u32::from_be_bytes([
                    payload[at],
                    payload[at + 1],
                    payload[at + 2],
                    payload[at + 3],
                ]);
                if u64::from(val) >= old_mdat {
                    let nv = (i64::from(val) + delta) as u32;
                    payload[at..at + 4].copy_from_slice(&nv.to_be_bytes());
                }
            }
        } else {
            if payload.len() < 8 + n * 8 {
                return Err(Error::InvalidStructure("Truncated co64".into()));
            }
            for i in 0..n {
                let at = 8 + i * 8;
                let val = u64::from_be_bytes([
                    payload[at],
                    payload[at + 1],
                    payload[at + 2],
                    payload[at + 3],
                    payload[at + 4],
                    payload[at + 5],
                    payload[at + 6],
                    payload[at + 7],
                ]);
                if val >= old_mdat {
                    let nv = val.wrapping_add(delta as u64);
                    payload[at..at + 8].copy_from_slice(&nv.to_be_bytes());
                }
            }
        }
        Ok(())
    })
}

fn patch_ctbo(data: &mut [u8]) -> Result<()> {
    let mut xmp = None;
    let mut preview = None;
    let mut mdat = None;
    let mut pos = 0usize;
    while pos + 8 <= data.len() {
        let (size, hdr) = box_header(data, pos, data.len())?;
        if size < hdr || pos + size > data.len() {
            break;
        }
        let typ = &data[pos + 4..pos + 8];
        if typ == b"uuid" && pos + hdr + 16 <= data.len() {
            let uuid = &data[pos + hdr..pos + hdr + 16];
            if uuid == XMP_UUID {
                xmp = Some((pos as u64, size as u64));
            } else if uuid == PREVIEW_UUID {
                preview = Some((pos as u64, size as u64));
            }
        } else if typ == b"mdat" {
            mdat = Some((pos as u64, size as u64));
        }
        pos += size;
    }
    let mdat =
        mdat.ok_or_else(|| Error::InvalidStructure("CR3 missing mdat after write".into()))?;
    walk_mut(data, 0, data.len(), &mut |typ, payload| {
        if typ != *b"CTBO" || payload.len() < 4 {
            return Ok(());
        }
        let n = u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]) as usize;
        if payload.len() < 4 + n * 20 {
            return Err(Error::InvalidStructure("Truncated CTBO".into()));
        }
        for i in 0..n {
            let at = 4 + i * 20;
            let id = u32::from_be_bytes([
                payload[at],
                payload[at + 1],
                payload[at + 2],
                payload[at + 3],
            ]);
            let loc = match id {
                1 => xmp,
                2 => preview,
                3 => Some(mdat),
                _ => None,
            };
            if let Some((off, sz)) = loc {
                payload[at + 4..at + 12].copy_from_slice(&off.to_be_bytes());
                payload[at + 12..at + 20].copy_from_slice(&sz.to_be_bytes());
            } else if id == 1 || id == 2 {
                payload[at + 4..at + 20].fill(0);
            }
        }
        Ok(())
    })
}

fn walk_mut(
    data: &mut [u8],
    start: usize,
    end: usize,
    f: &mut dyn FnMut([u8; 4], &mut [u8]) -> Result<()>,
) -> Result<()> {
    let mut pos = start;
    while pos + 8 <= end {
        let size32 = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        let (size, hdr) = if size32 == 1 {
            if pos + 16 > end {
                break;
            }
            let size = u64::from_be_bytes([
                data[pos + 8],
                data[pos + 9],
                data[pos + 10],
                data[pos + 11],
                data[pos + 12],
                data[pos + 13],
                data[pos + 14],
                data[pos + 15],
            ]) as usize;
            (size, 16usize)
        } else if size32 == 0 {
            (end - pos, 8usize)
        } else {
            (size32 as usize, 8usize)
        };
        if size < hdr || pos + size > end {
            break;
        }
        let typ = [data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]];
        let body_at = pos + hdr;
        let body_end = pos + size;
        if is_container(&typ) {
            walk_mut(data, body_at, body_end, f)?;
        } else if typ == *b"uuid" && body_at + 16 <= body_end {
            let rest = body_at + 16;
            if nested_boxes(data, rest, body_end) {
                walk_mut(data, rest, body_end, f)?;
            }
        } else {
            f(typ, &mut data[body_at..body_end])?;
        }
        pos += size;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cr3Parser, FormatParser};
    use exiftool_attrs::AttrValue;
    use std::io::Cursor;
    use std::path::PathBuf;

    fn fixture() -> Option<Vec<u8>> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/CanonRaw.cr3");
        path.exists().then(|| std::fs::read(path).unwrap())
    }

    #[test]
    fn cr3_write_sets_artist_and_keeps_mdat() {
        let Some(data) = fixture() else {
            panic!("missing testdata/CanonRaw.cr3");
        };
        let mut meta = Cr3Parser.parse(&mut Cursor::new(&data)).unwrap();
        assert!(
            meta.exif.get_str("Make").is_some(),
            "parser must read CMT1 Make"
        );
        meta.exif
            .set("Artist", AttrValue::Str("exiftool-rs".into()));
        let mut out = Vec::new();
        Cr3Writer::write(&mut Cursor::new(&data), &mut out, &meta).unwrap();
        assert_eq!(&out[4..12], b"ftypcrx ");
        assert!(
            out.windows(12).any(|w| w == b"exiftool-rs\0"),
            "serialized CR3 missing Artist in CMT TIFF"
        );
        let parsed = Cr3Parser.parse(&mut Cursor::new(&out)).unwrap();
        assert_eq!(parsed.exif.get_str("Artist"), Some("exiftool-rs"));
        assert!(parsed.exif.get_str("Make").is_some());
        let old_m = find_mdat_start(&data).unwrap() as usize;
        let new_m = find_mdat_start(&out).unwrap() as usize;
        let (old_sz, old_hdr) = box_header(&data, old_m, data.len()).unwrap();
        let (new_sz, new_hdr) = box_header(&out, new_m, out.len()).unwrap();
        assert_eq!(
            &out[new_m + new_hdr..new_m + new_sz],
            &data[old_m + old_hdr..old_m + old_sz]
        );
        assert!(Cr3Parser.can_parse(&out));
    }
}

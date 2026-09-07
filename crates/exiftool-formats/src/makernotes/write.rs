//! MakerNotes field rewrite (ExifTool: existing tags only, no create/delete).
//!
//! First vendor: FujiFilm IFD after `FUJIFILM` + IFD offset. Other blobs are copied.

use crate::Metadata;
use exiftool_attrs::AttrValue;
use exiftool_core::{ByteOrder, ExifFormat, IfdEntry, RawValue, SRational, URational};
use exiftool_tags::generated::fujifilm;

const FUJI_MAGIC: &[u8] = b"FUJIFILM";

/// Rewrite a MakerNotes blob. Unknown layouts are returned unchanged.
pub fn rewrite_blob(data: &[u8], metadata: &Metadata) -> Vec<u8> {
    if data.starts_with(FUJI_MAGIC) {
        if let Some(out) = rewrite_fujifilm(data, metadata) {
            return out;
        }
    }
    data.to_vec()
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
}

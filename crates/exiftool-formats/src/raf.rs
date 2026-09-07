//! Fujifilm RAF (ExifTool `FujiFilm.pm` `ProcessRAF` / `ProcessFujiDir`).
//!
//! Layout (big-endian header):
//! - Magic `FUJIFILMCCD-RAW `
//! - `RAFVersion` / `FirmwareVersion` at 0x3C (4 bytes)
//! - M-RAW start/len at 0x48 / 0x4C
//! - JPEG preview start/len at 0x54 / 0x58
//! - RAF directory at 0x5C; FujiIFD TIFF at 0x64; RAF2 at 0x78; FujiIFD1 at 0x80
//!
//! Embedded JPEG supplies EXIF + FujiFilm MakerNotes. RAF directories supply CFA / WB tags.

use crate::{Error, FormatParser, JpegParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use exiftool_core::{ByteOrder, IfdReader};
use std::io::{Cursor, SeekFrom};

const RAF_MAGIC: &[u8; 16] = b"FUJIFILMCCD-RAW ";
const HEADER_LEN: usize = 0x88;
const MAX_JPEG: u32 = 50_000_000;
const MAX_TAG: usize = 1_048_576;
const MAX_MRAW: u32 = 16 * 1024 * 1024;

/// Fujifilm RAF format parser.
pub struct RafParser;

impl FormatParser for RafParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 16 && header[..16] == *RAF_MAGIC
    }

    fn format_name(&self) -> &'static str {
        "RAF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["raf"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        parse_raf(reader)
    }
}

fn parse_raf(reader: &mut dyn ReadSeek) -> Result<Metadata> {
    reader.seek(SeekFrom::Start(0))?;
    let mut header = vec![0u8; HEADER_LEN];
    let n = read_some(reader, &mut header)?;
    if n < 16 || &header[..16] != RAF_MAGIC {
        return Err(Error::InvalidStructure("invalid RAF magic".into()));
    }
    header.truncate(n);

    let version = if header.len() >= 0x40 {
        String::from_utf8_lossy(&header[0x3C..0x40])
            .trim_end_matches('\0')
            .trim()
            .to_string()
    } else {
        String::new()
    };

    let (jpos, jlen) = pair_at(&header, 0x54).unwrap_or((0, 0));
    let (_mpos, mlen) = pair_at(&header, 0x48).unwrap_or((0, 0));
    if jpos & 0x8000 != 0 {
        return Err(Error::InvalidStructure("invalid RAF JPEG offset".into()));
    }

    let mut metadata = if jpos != 0 && jlen != 0 && jlen <= MAX_JPEG {
        reader.seek(SeekFrom::Start(u64::from(jpos)))?;
        let mut jpeg_data = vec![0u8; jlen as usize];
        reader.read_exact(&mut jpeg_data)?;
        let mut jpeg_reader = Cursor::new(&jpeg_data);
        let mut metadata = JpegParser.parse(&mut jpeg_reader).unwrap_or_else(|_| Metadata::new("RAF"));
        metadata.preview = Some(jpeg_data);
        metadata
    } else {
        Metadata::new("RAF")
    };

    metadata.format = "RAF";
    metadata.set_file_type("RAF", "image/x-fujifilm-raf");
    if !version.is_empty() {
        metadata.exif.set("RAFVersion", AttrValue::Str(version.clone()));
        metadata.exif.set("FirmwareVersion", AttrValue::Str(version));
    }
    if header.len() >= 0x70 && header[0x6c..0x6f] == [0, 0, 0] {
        let c = be_u32(&header, 0x6c);
        let s = match c {
            0 => "Uncompressed",
            2 => "Lossless",
            3 => "Lossy",
            _ => "",
        };
        if !s.is_empty() {
            metadata.exif.set("RAFCompression", AttrValue::Str(s.into()));
        }
    }

    let mut raf_num: u32 = 0;
    let mut ifd_num: u32 = 0;
    let mut fuji_layout = false;
    for offset in [0x48u32, 0x5c, 0x64, 0x78, 0x80] {
        if jpos != 0 && offset >= jpos {
            break;
        }
        let Some((start, len)) = pair_at(&header, offset as usize) else {
            continue;
        };
        if start == 0 {
            continue;
        }
        if offset == 0x64 || offset == 0x80 {
            let _ = parse_fuji_ifd(reader, start, len, ifd_num, &mut metadata);
            ifd_num = if ifd_num == 0 { 2 } else { ifd_num + 1 };
        } else if offset == 0x48 {
            let ft = metadata
                .exif
                .get_str("File:FileType")
                .unwrap_or("RAF")
                .to_string();
            if !ft.contains("M-RAW") {
                metadata.set_file_type(&format!("{ft} (M-RAW)"), "image/x-fujifilm-raf");
            }
            if mlen == 0 || mlen > MAX_MRAW {
                continue;
            }
            reader.seek(SeekFrom::Start(u64::from(start)))?;
            let mut buf = vec![0u8; mlen as usize];
            if reader.read_exact(&mut buf).is_ok() {
                parse_mraw(&buf, &mut metadata);
            }
        } else if parse_fuji_dir(reader, start, raf_num, &mut fuji_layout, &mut metadata).is_ok() {
            raf_num = if raf_num == 0 { 2 } else { raf_num + 1 };
        }
    }

    Ok(metadata)
}

fn parse_fuji_dir(
    reader: &mut dyn ReadSeek,
    start: u32,
    raf_num: u32,
    fuji_layout: &mut bool,
    metadata: &mut Metadata,
) -> Result<()> {
    reader.seek(SeekFrom::Start(u64::from(start)))?;
    let mut nbuf = [0u8; 4];
    reader.read_exact(&mut nbuf)?;
    let entries = u32::from_be_bytes(nbuf);
    if entries >= 256 {
        return Err(Error::InvalidStructure("RAF directory too large".into()));
    }
    for _ in 0..entries {
        let mut hdr = [0u8; 4];
        reader.read_exact(&mut hdr)?;
        let tag = u16::from_be_bytes([hdr[0], hdr[1]]);
        let len = u16::from_be_bytes([hdr[2], hdr[3]]) as usize;
        if len > MAX_TAG {
            return Err(Error::InvalidStructure("RAF tag too large".into()));
        }
        let mut payload = vec![0u8; len];
        reader.read_exact(&mut payload)?;
        apply_raf_tag(tag, &payload, raf_num, fuji_layout, metadata);
    }
    Ok(())
}

fn apply_raf_tag(
    tag: u16,
    payload: &[u8],
    raf_num: u32,
    fuji_layout: &mut bool,
    metadata: &mut Metadata,
) {
    let Some((name, kind)) = raf_tag(tag) else {
        return;
    };
    if raf_num != 0 && metadata.exif.contains(name) {
        return;
    }
    match kind {
        RafKind::Skip => {}
        RafKind::U16x2ReverseXh => {
            if let Some(v) = u16x2(payload) {
                set_first(metadata, name, AttrValue::Str(format!("{}x{}", v[1], v[0])));
            }
        }
        RafKind::U16x2ReverseColon => {
            if let Some(v) = u16x2(payload) {
                set_first(metadata, name, AttrValue::Str(format!("{}:{}", v[1], v[0])));
            }
        }
        RafKind::U16x2Space => {
            if let Some(v) = u16x2(payload) {
                set_first(metadata, name, AttrValue::Str(format!("{} {}", v[0], v[1])));
            }
        }
        RafKind::RawImageSize => {
            if let Some(v) = u16x2(payload) {
                let mut w = f64::from(v[1]);
                let mut h = f64::from(v[0]);
                if *fuji_layout {
                    w /= 2.0;
                    h *= 2.0;
                }
                set_first(metadata, name, AttrValue::Str(format!("{w}x{h}")));
            }
        }
        RafKind::FujiLayout => {
            if let Some(b) = payload.first() {
                *fuji_layout = b & 0x80 != 0;
                set_first(metadata, name, AttrValue::UInt(u32::from(*b)));
            }
        }
        RafKind::XTrans => {
            if payload.len() >= 36 {
                let rgb: String = payload[..36]
                    .iter()
                    .map(|b| match b {
                        0 => 'R',
                        1 => 'G',
                        2 => 'B',
                        _ => ' ',
                    })
                    .collect();
                let chunks: Vec<&str> = rgb.as_bytes().chunks(6).filter_map(|c| std::str::from_utf8(c).ok()).collect();
                set_first(metadata, name, AttrValue::Str(chunks.join(" ")));
            }
        }
        RafKind::U16x4 => {
            if let Some(v) = u16xn(payload, 4) {
                set_first(
                    metadata,
                    name,
                    AttrValue::Str(format!("{} {} {} {}", v[0], v[1], v[2], v[3])),
                );
            }
        }
        RafKind::YesNo => {
            if payload.len() >= 4 {
                let v = u32::from_be_bytes(payload[..4].try_into().unwrap());
                set_first(
                    metadata,
                    name,
                    AttrValue::Str(if v == 1 { "Yes".into() } else { "No".into() }),
                );
            }
        }
        RafKind::RationalLog2 => {
            if let Some((n, d)) = i32_pair(payload) {
                if d != 0 {
                    let val = n as f64 / d as f64;
                    if val > 0.0 {
                        let ev = val.log2();
                        set_first(metadata, name, AttrValue::Str(format!("{ev:+.1}")));
                    } else {
                        set_first(metadata, name, AttrValue::Str("0".into()));
                    }
                }
            }
        }
        RafKind::RationalEv => {
            if let Some((n, d)) = i32_pair(payload) {
                if d != 0 {
                    let val = n as f64 / d as f64;
                    let s = if val == 0.0 {
                        "0".into()
                    } else {
                        format!("{val:+.1}")
                    };
                    set_first(metadata, name, AttrValue::Str(s));
                }
            }
        }
    }
}

fn parse_fuji_ifd(
    reader: &mut dyn ReadSeek,
    start: u32,
    len: u32,
    ifd_num: u32,
    metadata: &mut Metadata,
) -> Result<()> {
    if len < 8 || len > MAX_TAG as u32 {
        return Ok(());
    }
    reader.seek(SeekFrom::Start(u64::from(start)))?;
    let mut data = vec![0u8; len as usize];
    reader.read_exact(&mut data)?;
    if data.len() < 8 {
        return Ok(());
    }
    let is_tiff = (data[0] == b'I' && data[1] == b'I') || (data[0] == b'M' && data[1] == b'M');
    if !is_tiff {
        return Ok(());
    }
    let Ok(byte_order) = ByteOrder::from_marker([data[0], data[1]]) else {
        return Ok(());
    };
    let ifd = IfdReader::new(&data, byte_order);
    let Ok((ifd0, _)) = ifd.parse_header_ex_with_magic(&[42, 43]) else {
        return Ok(());
    };
    let Ok((entries, _)) = ifd.read_ifd(ifd0) else {
        return Ok(());
    };
    let prefix = if ifd_num == 0 { "FujiIFD" } else { "FujiIFD2" };
    for e in entries {
        let name = match e.tag {
            0xf001 => "RawImageFullWidth",
            0xf002 => "RawImageFullHeight",
            0xf003 => "BitsPerSample",
            0xf00a => "BlackLevel",
            0xf00d => "WB_GRBLevelsAuto",
            0xf00e => "WB_GRBLevels",
            _ => continue,
        };
        let key = format!("{prefix}:{name}");
        if metadata.exif.contains(&key) {
            continue;
        }
        if let Some(u) = e.value.as_u32() {
            metadata.exif.set(key, AttrValue::UInt(u));
        }
    }
    Ok(())
}

fn parse_mraw(buf: &[u8], metadata: &mut Metadata) {
    if buf.len() < 20 || !buf.starts_with(b"FUJIFILMM-RAW  ") {
        return;
    }
    let ver = String::from_utf8_lossy(&buf[16..20]).trim().to_string();
    if !ver.is_empty() {
        set_first(metadata, "MRAWVersion", AttrValue::Str(ver));
    }
}

#[derive(Clone, Copy)]
enum RafKind {
    Skip,
    U16x2ReverseXh,
    U16x2ReverseColon,
    U16x2Space,
    RawImageSize,
    FujiLayout,
    XTrans,
    U16x4,
    YesNo,
    RationalLog2,
    RationalEv,
}

fn raf_tag(tag: u16) -> Option<(&'static str, RafKind)> {
    Some(match tag {
        0x0100 => ("RawImageFullSize", RafKind::U16x2ReverseXh),
        0x0110 => ("RawImageCropTopLeft", RafKind::U16x2Space),
        0x0111 => ("RawImageCroppedSize", RafKind::U16x2ReverseXh),
        0x0115 => ("RawImageAspectRatio", RafKind::U16x2ReverseColon),
        0x0117 => ("RawZoomActive", RafKind::YesNo),
        0x0118 => ("RawZoomTopLeft", RafKind::U16x2ReverseXh),
        0x0119 => ("RawZoomSize", RafKind::U16x2ReverseXh),
        0x0121 => ("RawImageSize", RafKind::RawImageSize),
        0x0130 => ("FujiLayout", RafKind::FujiLayout),
        0x0131 => ("XTransLayout", RafKind::XTrans),
        0x2000 => ("WB_GRGBLevelsAuto", RafKind::U16x4),
        0x2100 => ("WB_GRGBLevelsDaylight", RafKind::U16x4),
        0x2200 => ("WB_GRGBLevelsCloudy", RafKind::U16x4),
        0x2300 => ("WB_GRGBLevelsDaylightFluor", RafKind::U16x4),
        0x2301 => ("WB_GRGBLevelsDayWhiteFluor", RafKind::U16x4),
        0x2302 => ("WB_GRGBLevelsWhiteFluorescent", RafKind::U16x4),
        0x2310 => ("WB_GRGBLevelsWarmWhiteFluor", RafKind::U16x4),
        0x2311 => ("WB_GRGBLevelsLivingRoomWarmWhiteFluor", RafKind::U16x4),
        0x2400 => ("WB_GRGBLevelsTungsten", RafKind::U16x4),
        0x2410 => ("WB_GRGBLevelsFlash", RafKind::U16x4),
        0x2ff0 => ("WB_GRGBLevels", RafKind::U16x4),
        0x9200 => ("RelativeExposure", RafKind::RationalLog2),
        0x9650 => ("RawExposureBias", RafKind::RationalEv),
        0xc000 => ("RAFData", RafKind::Skip),
        _ => return None,
    })
}

fn set_first(metadata: &mut Metadata, name: &str, value: AttrValue) {
    if !metadata.exif.contains(name) {
        metadata.exif.set(name, value);
    }
}

fn pair_at(header: &[u8], off: usize) -> Option<(u32, u32)> {
    if header.len() < off + 8 {
        return None;
    }
    Some((be_u32(header, off), be_u32(header, off + 4)))
}

fn be_u32(buf: &[u8], off: usize) -> u32 {
    u32::from_be_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}

fn u16x2(p: &[u8]) -> Option<[u16; 2]> {
    if p.len() < 4 {
        return None;
    }
    Some([
        u16::from_be_bytes([p[0], p[1]]),
        u16::from_be_bytes([p[2], p[3]]),
    ])
}

fn u16xn(p: &[u8], n: usize) -> Option<Vec<u16>> {
    if p.len() < n * 2 {
        return None;
    }
    Some(
        (0..n)
            .map(|i| u16::from_be_bytes([p[i * 2], p[i * 2 + 1]]))
            .collect(),
    )
}

fn i32_pair(p: &[u8]) -> Option<(i32, i32)> {
    if p.len() < 8 {
        return None;
    }
    Some((
        i32::from_be_bytes(p[0..4].try_into().ok()?),
        i32::from_be_bytes(p[4..8].try_into().ok()?),
    ))
}

fn read_some(reader: &mut dyn ReadSeek, buf: &mut [u8]) -> Result<usize> {
    let mut n = 0;
    while n < buf.len() {
        match std::io::Read::read(reader, &mut buf[n..]) {
            Ok(0) => break,
            Ok(k) => n += k,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e.into()),
        }
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn detect_raf() {
        let parser = RafParser;
        let mut header = RAF_MAGIC.to_vec();
        header.extend_from_slice(&[0u8; 16]);
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn reject_jpeg() {
        let parser = RafParser;
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
    }

    #[test]
    fn reject_tiff() {
        let parser = RafParser;
        assert!(!parser.can_parse(&[b'I', b'I', 0x2A, 0x00]));
    }

    #[test]
    fn parse_fujifilm_raf_directory() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/FujiFilm.raf");
        if !path.exists() {
            return;
        }
        let data = std::fs::read(&path).unwrap();
        let mut cur = Cursor::new(data);
        let meta = RafParser.parse(&mut cur).unwrap();
        assert_eq!(meta.format, "RAF");
        assert_eq!(meta.exif.get_str("RAFVersion"), Some("0106"));
        assert_eq!(meta.exif.get_str("RawImageFullSize"), Some("4352x1444"));
        assert_eq!(meta.exif.get_str("WB_GRGBLevels"), Some("384 601 384 515"));
        assert!(meta.preview.is_some());
    }
}

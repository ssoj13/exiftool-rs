//! DICOM / ACR-NEMA medical image metadata (ExifTool `DICOM.pm`).
//!
//! Detect: 128-byte preamble + `DICM`, or ACR-NEMA group/element at offset 0.
//! Walk data elements; names from generated [`dicom_tags`] (ExifTool Main table).
//! Pixel Data is not loaded (seek past it). Implicit VR values are strings, as in ExifTool.

use crate::dicom_tags;
use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use flate2::read::DeflateDecoder;
use std::io::{Read, SeekFrom};

/// ExifTool magic needs 128 + `DICM`.
pub const DICOM_PREAMBLE: usize = 128;

const VR32: &[&[u8]] = &[b"OB", b"OW", b"OF", b"SQ", b"UT", b"UN"];

const TS_IMPLICIT_LE: &str = "1.2.840.10008.1.2";
const TS_EXPLICIT_BE: &str = "1.2.840.10008.1.2.2";
const TS_DEFLATED: &str = "1.2.840.10008.1.2.1.99";

const MAX_META_VALUE: u32 = 64 * 1024;
const MAX_ELEMENTS: usize = 100_000;

/// DICOM / ACR-NEMA parser.
pub struct DicomParser;

impl FormatParser for DicomParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        looks_like_dicom(header)
    }

    fn format_name(&self) -> &'static str {
        "DICOM"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["dcm", "dicom", "dic", "dicm", "dc3"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        parse_dicom(reader)
    }
}

/// ExifTool.pm `%magicNumber` DICOM: `.{128}DICM` or ACR group in first bytes.
#[must_use]
pub fn looks_like_dicom(header: &[u8]) -> bool {
    if header.len() >= DICOM_PREAMBLE + 4 && &header[DICOM_PREAMBLE..DICOM_PREAMBLE + 4] == b"DICM"
    {
        return true;
    }
    acr_header(header).is_some()
}

fn acr_header(hdr: &[u8]) -> Option<bool> {
    if hdr.len() < 8 {
        return None;
    }
    for le in [true, false] {
        let g = u16_at(hdr, 0, le);
        if !(2..=8).contains(&g) || g & 1 != 0 {
            continue;
        }
        let e = u16_at(hdr, 2, le);
        if e > 0x20 {
            continue;
        }
        return Some(le);
    }
    None
}

fn u16_at(b: &[u8], off: usize, le: bool) -> u16 {
    let x = [b[off], b[off + 1]];
    if le {
        u16::from_le_bytes(x)
    } else {
        u16::from_be_bytes(x)
    }
}

fn u32_at(b: &[u8], off: usize, le: bool) -> u32 {
    let x = [b[off], b[off + 1], b[off + 2], b[off + 3]];
    if le {
        u32::from_le_bytes(x)
    } else {
        u32::from_be_bytes(x)
    }
}

fn is_vr32(vr: &[u8]) -> bool {
    VR32.iter().any(|v| *v == vr)
}

fn implicit_vr_tag(group: u16, element: u16) -> bool {
    matches!((group, element), (0xFFFE, 0xE000 | 0xE00D | 0xE0DD))
}

fn parse_dicom(reader: &mut dyn ReadSeek) -> Result<Metadata> {
    let mut hdr12 = [0u8; 12];
    reader.read_exact(&mut hdr12)?;
    reader.seek(SeekFrom::Start(DICOM_PREAMBLE as u64))?;
    let mut sig = [0u8; 4];
    reader.read_exact(&mut sig)?;

    let mut little = true;
    let implicit = false;
    let format = if &sig == b"DICM" {
        "DICOM"
    } else {
        let Some(le) = acr_header(&hdr12) else {
            return Err(Error::InvalidStructure("not DICOM/ACR".into()));
        };
        little = le;
        reader.seek(SeekFrom::Start(0))?;
        "ACR"
    };

    let mut metadata = Metadata::new(format);
    walk_elements(reader, &mut metadata, little, implicit)?;
    Ok(metadata)
}

fn walk_elements(
    reader: &mut dyn ReadSeek,
    metadata: &mut Metadata,
    mut little: bool,
    mut implicit: bool,
) -> Result<()> {
    let mut transfer_syntax: Option<String> = None;
    let mut group2_end: Option<u64> = None;
    let mut applied_ts = false;

    for _ in 0..MAX_ELEMENTS {
        let pos = reader.stream_position()?;
        let mut hdr = [0u8; 8];
        if reader.read_exact(&mut hdr).is_err() {
            return Ok(());
        }

        if !applied_ts {
            if let Some(ts) = transfer_syntax.as_deref() {
                let group_peek = u16_at(&hdr, 0, little);
                let past_g2 =
                    group_peek != 0x0002 || group2_end.map(|end| pos + 8 > end).unwrap_or(false);
                if past_g2 {
                    let ts = ts.trim_end_matches('\0').trim();
                    if ts == TS_DEFLATED {
                        reader.seek(SeekFrom::Start(pos))?;
                        let mut rest = Vec::new();
                        reader.read_to_end(&mut rest)?;
                        let mut dec = DeflateDecoder::new(rest.as_slice());
                        let mut inflated = Vec::new();
                        if dec.read_to_end(&mut inflated).is_err() {
                            return Ok(());
                        }
                        let mut cur = std::io::Cursor::new(inflated);
                        return walk_elements(&mut cur, metadata, true, false);
                    }
                    apply_transfer_syntax(ts, &mut little, &mut implicit, reader, &mut hdr, pos)?;
                    applied_ts = true;
                }
            }
        }

        let group = u16_at(&hdr, 0, little);
        let element = u16_at(&hdr, 2, little);

        let (vr, len) = if implicit || implicit_vr_tag(group, element) {
            (None, u32_at(&hdr, 4, little))
        } else {
            let vr = [hdr[4], hdr[5]];
            if !(vr[0].is_ascii_uppercase() && vr[1].is_ascii_uppercase()) {
                return Ok(());
            }
            if is_vr32(&vr) {
                let mut extra = [0u8; 4];
                reader.read_exact(&mut extra)?;
                let mut n = u32_at(&extra, 0, little);
                if vr == *b"SQ" {
                    n = 0;
                }
                (Some(vr), n)
            } else {
                (Some(vr), u32::from(u16_at(&hdr, 6, little)))
            }
        };

        if group == 0x7FE0 && element == 0x0010 {
            let name = dicom_tags::lookup(group, element).unwrap_or("PixelData");
            metadata
                .exif
                .set(name, AttrValue::Str(format!("Binary data {len} bytes")));
            if len != 0xFFFF_FFFF {
                reader.seek(SeekFrom::Current(i64::from(len)))?;
            }
            return Ok(());
        }

        if len == 0xFFFF_FFFF {
            continue;
        }

        let mut value = Vec::new();
        if len > 0 {
            if len > MAX_META_VALUE {
                reader.seek(SeekFrom::Current(i64::from(len)))?;
                if let Some(name) = dicom_tags::lookup(group, element) {
                    metadata
                        .exif
                        .set(name, AttrValue::Str(format!("Binary data {len} bytes")));
                }
                continue;
            }
            value.resize(len as usize, 0);
            if reader.read_exact(&mut value).is_err() {
                return Ok(());
            }
        }

        if element == 0 && value.len() == 4 {
            let gl = u32_at(&value, 0, little);
            if group == 2 {
                group2_end = Some(reader.stream_position()? + u64::from(gl));
            }
        }

        let tag_name = if element == 0 {
            Some(format!("Group{group:04X}_Length"))
        } else {
            dicom_tags::lookup(group, element).map(str::to_string)
        };

        if let Some(name) = tag_name {
            let s = decode_value(&value, vr.as_ref().map(|v| v.as_slice()), little);
            if group == 2 && element == 0x0010 {
                transfer_syntax = Some(s.trim_end_matches('\0').trim().to_string());
            }
            if !s.is_empty() {
                metadata.exif.set(name, AttrValue::Str(s));
            }
        }
    }
    Ok(())
}

fn apply_transfer_syntax(
    ts: &str,
    little: &mut bool,
    implicit: &mut bool,
    reader: &mut dyn ReadSeek,
    hdr: &mut [u8; 8],
    pos: u64,
) -> Result<()> {
    let ts = ts.trim_end_matches('\0').trim();
    if ts == TS_IMPLICIT_LE {
        *implicit = true;
        *little = true;
    } else if ts == TS_EXPLICIT_BE {
        *implicit = false;
        *little = false;
        *hdr = reread_hdr(reader, pos)?;
    } else if ts.starts_with("1.2.840.10008.1.2") {
        *implicit = false;
        *little = true;
    }
    let _ = reader;
    Ok(())
}

fn reread_hdr(reader: &mut dyn ReadSeek, pos: u64) -> Result<[u8; 8]> {
    reader.seek(SeekFrom::Start(pos))?;
    let mut hdr = [0u8; 8];
    reader.read_exact(&mut hdr)?;
    Ok(hdr)
}

fn decode_value(buf: &[u8], vr: Option<&[u8]>, little: bool) -> String {
    if buf.is_empty() {
        return String::new();
    }
    match vr {
        Some(b"US") if buf.len() >= 2 => u16_at(buf, 0, little).to_string(),
        Some(b"UL" | b"SL") if buf.len() >= 4 => {
            if vr == Some(b"SL") {
                let v = if little {
                    i32::from_le_bytes(buf[..4].try_into().unwrap())
                } else {
                    i32::from_be_bytes(buf[..4].try_into().unwrap())
                };
                v.to_string()
            } else {
                u32_at(buf, 0, little).to_string()
            }
        }
        Some(b"SS") if buf.len() >= 2 => {
            let v = if little {
                i16::from_le_bytes([buf[0], buf[1]])
            } else {
                i16::from_be_bytes([buf[0], buf[1]])
            };
            v.to_string()
        }
        Some(b"FL") if buf.len() >= 4 => {
            let bits = u32_at(buf, 0, little);
            f32::from_bits(bits).to_string()
        }
        Some(b"FD") if buf.len() >= 8 => {
            let b: [u8; 8] = buf[..8].try_into().unwrap();
            let bits = if little {
                u64::from_le_bytes(b)
            } else {
                u64::from_be_bytes(b)
            };
            f64::from_bits(bits).to_string()
        }
        Some(b"AT") if buf.len() >= 4 => {
            format!(
                "{:04X},{:04X}",
                u16_at(buf, 0, little),
                u16_at(buf, 2, little)
            )
        }
        Some(b"DA") => format_da(buf),
        Some(b"TM") => format_tm(buf),
        Some(b"DT") => format_dt(buf),
        _ => {
            let mut s = String::from_utf8_lossy(buf).into_owned();
            s = s.trim_end_matches('\0').to_string();
            s.trim().to_string()
        }
    }
}

fn format_da(buf: &[u8]) -> String {
    let s = String::from_utf8_lossy(buf);
    let t = s.trim();
    if t.len() >= 8 && t.as_bytes().iter().take(8).all(|c| c.is_ascii_digit()) {
        format!("{}:{}:{}", &t[0..4], &t[4..6], &t[6..8])
    } else {
        t.to_string()
    }
}

fn format_tm(buf: &[u8]) -> String {
    let s = String::from_utf8_lossy(buf);
    let t = s.trim();
    if t.len() >= 6 && t.as_bytes().iter().take(6).all(|c| c.is_ascii_digit()) {
        format!("{}:{}:{}", &t[0..2], &t[2..4], &t[4..])
    } else {
        t.to_string()
    }
}

fn format_dt(buf: &[u8]) -> String {
    let s = String::from_utf8_lossy(buf);
    let t = s.trim();
    if t.len() >= 14 && t.as_bytes().iter().take(14).all(|c| c.is_ascii_digit()) {
        format!(
            "{}:{}:{} {}:{}:{}",
            &t[0..4],
            &t[4..6],
            &t[6..8],
            &t[8..10],
            &t[10..12],
            &t[12..]
        )
    } else {
        t.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn minimal_dicom_patient(name: &str) -> Vec<u8> {
        let mut d = vec![0u8; DICOM_PREAMBLE];
        d.extend_from_slice(b"DICM");
        // (0002,0010) UI TransferSyntaxUID explicit LE: 1.2.840.10008.1.2.1\0 padded even
        let uid = b"1.2.840.10008.1.2.1\0";
        d.extend_from_slice(&0x0002u16.to_le_bytes());
        d.extend_from_slice(&0x0010u16.to_le_bytes());
        d.extend_from_slice(b"UI");
        d.extend_from_slice(&(uid.len() as u16).to_le_bytes());
        d.extend_from_slice(uid);
        // (0010,0010) PN PatientName
        let pn = name.as_bytes();
        let mut pn_buf = pn.to_vec();
        if pn_buf.len() % 2 == 1 {
            pn_buf.push(b' ');
        }
        d.extend_from_slice(&0x0010u16.to_le_bytes());
        d.extend_from_slice(&0x0010u16.to_le_bytes());
        d.extend_from_slice(b"PN");
        d.extend_from_slice(&(pn_buf.len() as u16).to_le_bytes());
        d.extend_from_slice(&pn_buf);
        d
    }

    #[test]
    fn detect_dicm() {
        let data = minimal_dicom_patient("DOE^JOHN");
        assert!(DicomParser.can_parse(&data));
        assert!(!DicomParser.can_parse(b"\xFF\xD8\xFF"));
    }

    #[test]
    fn parse_patient_name() {
        let data = minimal_dicom_patient("DOE^JOHN");
        let mut c = Cursor::new(data);
        let meta = DicomParser.parse(&mut c).unwrap();
        assert_eq!(meta.format, "DICOM");
        assert_eq!(meta.exif.get_str("PatientName"), Some("DOE^JOHN"));
        assert_eq!(
            meta.exif.get_str("TransferSyntaxUID"),
            Some("1.2.840.10008.1.2.1")
        );
    }
}

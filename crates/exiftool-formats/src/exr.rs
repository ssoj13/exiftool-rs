//! OpenEXR format parser.
//!
//! Header attributes only (no pixel decode). Uses `exr-core` from
//! `ssh://git@github.com/ssoj13/exr-rs.git` (`Header::read_from`).

use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use exr_core::attr::ExrString;
use exr_core::header::Header;
use exr_core::{Chromaticities, Compression, LineOrder, MemIStream, TimeCode};

/// EXR magic signature (4 bytes): 0x76, 0x2F, 0x31, 0x01
const EXR_MAGIC: [u8; 4] = [0x76, 0x2F, 0x31, 0x01];

const SKIP_STRING_ATTRS: &[&str] = &[
    "channels",
    "compression",
    "displayWindow",
    "dataWindow",
    "lineOrder",
    "pixelAspectRatio",
    "screenWindowCenter",
    "screenWindowWidth",
    "type",
];

/// OpenEXR format parser.
pub struct ExrParser;

impl FormatParser for ExrParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 4 && header[..4] == EXR_MAGIC
    }

    fn format_name(&self) -> &'static str {
        "EXR"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["exr"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("EXR");
        let data = crate::utils::read_with_limit(reader)?;
        let mut stream = MemIStream::new(data);
        let (header, _version) = Header::read_from(&mut stream)
            .map_err(|e| Error::InvalidStructure(format!("EXR parse error: {e}")))?;
        apply_exr_header(&header, "", &mut metadata);
        Ok(metadata)
    }
}

fn apply_exr_header(header: &Header, prefix: &str, metadata: &mut Metadata) {
    if let Ok(dw) = header.display_window() {
        let width = dw.max.x as i64 - dw.min.x as i64 + 1;
        let height = dw.max.y as i64 - dw.min.y as i64 + 1;
        if width > 0 {
            metadata
                .exif
                .set(format!("{prefix}ImageWidth"), AttrValue::UInt(width as u32));
        }
        if height > 0 {
            metadata.exif.set(
                format!("{prefix}ImageHeight"),
                AttrValue::UInt(height as u32),
            );
        }
    }
    if let Ok(par) = header.pixel_aspect_ratio() {
        metadata
            .exif
            .set(format!("{prefix}PixelAspectRatio"), AttrValue::Float(par));
    }
    if let Ok(c) = header.compression() {
        metadata.exif.set(
            format!("{prefix}Compression"),
            AttrValue::Str(compression_name(c).into()),
        );
    }
    if let Ok(lo) = header.line_order() {
        metadata.exif.set(
            format!("{prefix}LineOrder"),
            AttrValue::Str(line_order_name(lo).into()),
        );
    }
    if let Ok(ch) = header.channels() {
        let names: Vec<String> = ch.iter().map(|(n, _)| n.clone()).collect();
        metadata.exif.set(
            format!("{prefix}Channels"),
            AttrValue::Str(names.join(", ")),
        );
        metadata.exif.set(
            format!("{prefix}ChannelCount"),
            AttrValue::UInt(ch.len() as u32),
        );
    }
    if let Some(attr) = header.find_typed_attribute::<Chromaticities>("chromaticities") {
        let c = &attr.value;
        metadata.exif.set(
            format!("{prefix}Chromaticities"),
            AttrValue::Str(format!(
                "R({:.3},{:.3}) G({:.3},{:.3}) B({:.3},{:.3}) W({:.3},{:.3})",
                c.red[0],
                c.red[1],
                c.green[0],
                c.green[1],
                c.blue[0],
                c.blue[1],
                c.white[0],
                c.white[1]
            )),
        );
    }
    if let Some(attr) = header.find_typed_attribute::<TimeCode>("timeCode") {
        let tc = &attr.value;
        metadata.exif.set(
            format!("{prefix}TimeCode"),
            AttrValue::Str(format!(
                "{:02}:{:02}:{:02}:{:02}",
                tc.hours(),
                tc.minutes(),
                tc.seconds(),
                tc.frame()
            )),
        );
    }
    for (name, _) in header.iter() {
        if SKIP_STRING_ATTRS.contains(&name.as_str()) {
            continue;
        }
        let Some(attr) = header.find_typed_attribute::<ExrString>(name) else {
            continue;
        };
        let text = attr.value.to_string_lossy().into_owned();
        if text.is_empty() {
            continue;
        }
        match name.as_str() {
            "comments" => {
                metadata.exif.set(
                    format!("{prefix}ImageDescription"),
                    AttrValue::Str(text.clone()),
                );
            }
            "owner" => {
                metadata
                    .exif
                    .set(format!("{prefix}Artist"), AttrValue::Str(text.clone()));
            }
            "software" => {
                metadata
                    .exif
                    .set(format!("{prefix}Software"), AttrValue::Str(text.clone()));
            }
            "capDate" => {
                metadata
                    .exif
                    .set(format!("{prefix}DateTime"), AttrValue::Str(text.clone()));
            }
            "cameraMake" => {
                metadata
                    .exif
                    .set(format!("{prefix}Make"), AttrValue::Str(text.clone()));
            }
            "cameraModel" => {
                metadata
                    .exif
                    .set(format!("{prefix}Model"), AttrValue::Str(text.clone()));
            }
            _ => {}
        }
        metadata
            .exif
            .set(format!("{prefix}EXR:{name}"), AttrValue::Str(text));
    }
}

fn compression_name(c: Compression) -> &'static str {
    match c {
        Compression::NoCompression => "None",
        Compression::Rle => "RLE",
        Compression::Zips => "ZIPS",
        Compression::Zip => "ZIP",
        Compression::Piz => "PIZ",
        Compression::Pxr24 => "PXR24",
        Compression::B44 => "B44",
        Compression::B44a => "B44A",
        Compression::Dwaa => "DWAA",
        Compression::Dwab => "DWAB",
        _ => "Unknown",
    }
}

fn line_order_name(lo: LineOrder) -> &'static str {
    match lo {
        LineOrder::IncreasingY => "Increasing",
        LineOrder::DecreasingY => "Decreasing",
        LineOrder::RandomY => "RandomY",
        LineOrder::Invalid => "Invalid",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn detect_exr() {
        let parser = ExrParser;
        assert!(parser.can_parse(&EXR_MAGIC));
        assert!(parser.can_parse(&[0x76, 0x2F, 0x31, 0x01, 0x02, 0x00]));
    }

    #[test]
    fn reject_jpeg() {
        let parser = ExrParser;
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
    }

    #[test]
    fn parse_openexr_fixture() {
        let path =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/OpenEXR.exr");
        if !path.exists() {
            return;
        }
        let data = std::fs::read(&path).unwrap();
        let mut cur = Cursor::new(data);
        let meta = ExrParser.parse(&mut cur).unwrap();
        assert_eq!(meta.format, "EXR");
        assert_eq!(
            meta.exif.get("ImageWidth").map(|v| v.to_string()),
            Some("3".into())
        );
        assert_eq!(meta.exif.get_str("Compression"), Some("PIZ"));
    }
}

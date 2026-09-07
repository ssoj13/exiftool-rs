//! ZIP archives and ZIP-based packages (ExifTool `ZIP.pm`).
//!
//! Local-file-header walk (same as ExifTool without Archive::Zip). Stored and
//! deflate members can be inflated with `flate2` to classify OOXML / OpenDocument
//! and pull `docProps/core.xml` or `meta.xml`.

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use flate2::read::DeflateDecoder;
use std::io::{Read, SeekFrom};

const LOCAL_SIG: &[u8] = b"PK\x03\x04";
const EOCD_SIG: &[u8] = b"PK\x05\x06";

/// ZIP / OOXML / ODF parser.
pub struct ZipParser;

impl FormatParser for ZipParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 4 && header.starts_with(LOCAL_SIG)
    }

    fn format_name(&self) -> &'static str {
        "ZIP"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &[
            "zip", "docx", "docm", "xlsx", "xlsm", "pptx", "pptm", "odt", "ods", "odp", "odg",
            "epub", "idml", "pages", "numbers", "key",
        ]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        parse_zip(reader)
    }
}

struct Member {
    name: String,
    method: u16,
    flags: u16,
    dos_time: u32,
    crc: u32,
    comp_size: u32,
    uncomp_size: u32,
    data_off: u64,
}

fn parse_zip(reader: &mut dyn ReadSeek) -> Result<Metadata> {
    let mut metadata = Metadata::new("ZIP");
    reader.seek(SeekFrom::Start(0))?;
    let mut members = Vec::new();
    let mut hdr = [0u8; 30];

    loop {
        if reader.read_exact(&mut hdr).is_err() {
            break;
        }
        if &hdr[0..4] != LOCAL_SIG {
            break;
        }
        let ver = u16::from_le_bytes([hdr[4], hdr[5]]);
        let flags = u16::from_le_bytes([hdr[6], hdr[7]]);
        let method = u16::from_le_bytes([hdr[8], hdr[9]]);
        let dos_time = u32::from_le_bytes([hdr[10], hdr[11], hdr[12], hdr[13]]);
        let crc = u32::from_le_bytes([hdr[14], hdr[15], hdr[16], hdr[17]]);
        let comp_size = u32::from_le_bytes([hdr[18], hdr[19], hdr[20], hdr[21]]);
        let uncomp_size = u32::from_le_bytes([hdr[22], hdr[23], hdr[24], hdr[25]]);
        let name_len = u16::from_le_bytes([hdr[26], hdr[27]]) as usize;
        let extra_len = u16::from_le_bytes([hdr[28], hdr[29]]) as usize;
        let mut name_buf = vec![0u8; name_len];
        reader.read_exact(&mut name_buf)?;
        reader.seek(SeekFrom::Current(extra_len as i64))?;
        let data_off = reader.stream_position()?;
        let name = String::from_utf8_lossy(&name_buf).into_owned();

        if flags & 0x08 != 0 {
            metadata.exif.set(
                "Zip:Warning",
                AttrValue::Str("Stream mode data encountered, file list may be incomplete".into()),
            );
            members.push(Member {
                name,
                method,
                flags,
                dos_time,
                crc,
                comp_size,
                uncomp_size,
                data_off,
            });
            break;
        }

        members.push(Member {
            name,
            method,
            flags,
            dos_time,
            crc,
            comp_size,
            uncomp_size,
            data_off,
        });
        let _ = ver;
        reader.seek(SeekFrom::Current(i64::from(comp_size)))?;
    }

    if let Some(comment) = read_eocd_comment(reader) {
        metadata.exif.set("Comment", AttrValue::Str(comment));
    }

    metadata
        .exif
        .set("ZipMemberCount", AttrValue::UInt(members.len() as u32));
    let names: Vec<&str> = members.iter().map(|m| m.name.as_str()).collect();
    if !names.is_empty() {
        metadata
            .exif
            .set("ZipFiles", AttrValue::Str(names.join(";")));
    }

    if let Some(first) = members.first() {
        set_member_tags(&mut metadata, first, 0);
    }

    classify_package(reader, &members, &mut metadata)?;
    Ok(metadata)
}

fn set_member_tags(metadata: &mut Metadata, m: &Member, index: usize) {
    let p = if index == 0 {
        String::new()
    } else {
        format!("{index}:")
    };
    metadata
        .exif
        .set(format!("Zip:{p}FileName"), AttrValue::Str(m.name.clone()));
    metadata.exif.set(
        format!("Zip:{p}Compression"),
        AttrValue::Str(compression_name(m.method).into()),
    );
    metadata.exif.set(
        format!("Zip:{p}CompressedSize"),
        AttrValue::UInt(m.comp_size),
    );
    metadata.exif.set(
        format!("Zip:{p}UncompressedSize"),
        AttrValue::UInt(m.uncomp_size),
    );
    metadata.exif.set(
        format!("Zip:{p}CRC"),
        AttrValue::Str(format!("0x{:08x}", m.crc)),
    );
    metadata.exif.set(
        format!("Zip:{p}ModifyDate"),
        AttrValue::Str(dos_datetime(m.dos_time)),
    );
    metadata.exif.set(
        format!("Zip:{p}BitFlag"),
        AttrValue::UInt(u32::from(m.flags)),
    );
}

fn compression_name(m: u16) -> &'static str {
    match m {
        0 => "None",
        8 => "Deflated",
        9 => "Enhanced Deflate using Deflate64(tm)",
        12 => "BZIP2",
        14 => "LZMA (EFS)",
        _ => "Unknown",
    }
}

fn dos_datetime(val: u32) -> String {
    format!(
        "{:04}:{:02}:{:02} {:02}:{:02}:{:02}",
        (val >> 25) + 1980,
        (val >> 21) & 0x0f,
        (val >> 16) & 0x1f,
        (val >> 11) & 0x1f,
        (val >> 5) & 0x3f,
        (val & 0x1f) * 2
    )
}

fn read_eocd_comment(reader: &mut dyn ReadSeek) -> Option<String> {
    let len = reader.seek(SeekFrom::End(0)).ok()?;
    if len < 22 {
        return None;
    }
    let scan = len.min(22 + 65535);
    reader.seek(SeekFrom::Start(len - scan)).ok()?;
    let mut buf = vec![0u8; scan as usize];
    reader.read_exact(&mut buf).ok()?;
    let pos = buf.windows(4).rposition(|w| w == EOCD_SIG)?;
    if pos + 22 > buf.len() {
        return None;
    }
    let clen = u16::from_le_bytes([buf[pos + 20], buf[pos + 21]]) as usize;
    if clen == 0 || pos + 22 + clen > buf.len() {
        return None;
    }
    Some(String::from_utf8_lossy(&buf[pos + 22..pos + 22 + clen]).into_owned())
}

fn inflate_member(reader: &mut dyn ReadSeek, m: &Member) -> Option<Vec<u8>> {
    if m.uncomp_size > 8 * 1024 * 1024 {
        return None;
    }
    reader.seek(SeekFrom::Start(m.data_off)).ok()?;
    let mut comp = vec![0u8; m.comp_size as usize];
    reader.read_exact(&mut comp).ok()?;
    match m.method {
        0 => Some(comp),
        8 => {
            let mut dec = DeflateDecoder::new(comp.as_slice());
            let mut out = Vec::new();
            dec.read_to_end(&mut out).ok()?;
            Some(out)
        }
        _ => None,
    }
}

fn member_named<'a>(members: &'a [Member], name: &str) -> Option<&'a Member> {
    members.iter().find(|m| m.name.eq_ignore_ascii_case(name))
}

fn classify_package(
    reader: &mut dyn ReadSeek,
    members: &[Member],
    metadata: &mut Metadata,
) -> Result<()> {
    if let Some(m) = member_named(members, "[Content_Types].xml") {
        if let Some(xml) = inflate_member(reader, m) {
            let s = String::from_utf8_lossy(&xml);
            if let Some(fmt) = ooxml_type(&s) {
                metadata.format = fmt;
            }
        }
    }
    if let Some(m) = member_named(members, "mimetype") {
        if let Some(raw) = inflate_member(reader, m) {
            if let Ok(mime) = std::str::from_utf8(&raw) {
                let mime = mime.trim();
                metadata
                    .exif
                    .set("MIMEType", AttrValue::Str(mime.to_string()));
                if let Some(fmt) = open_doc_type(mime) {
                    metadata.format = fmt;
                }
            }
        }
    }
    for (path, prefix) in [
        ("docProps/core.xml", "OOXML"),
        ("meta.xml", "ODF"),
        ("META-INF/metadata.xml", "ODF"),
    ] {
        if let Some(m) = member_named(members, path) {
            if let Some(raw) = inflate_member(reader, m) {
                if let Ok(xml) = String::from_utf8(raw) {
                    extract_simple_xml(&xml, prefix, metadata);
                }
            }
        }
    }
    if members
        .iter()
        .any(|m| m.name.starts_with("CaptureOne/") && m.name.to_ascii_lowercase().ends_with(".cos"))
    {
        metadata.format = "EIP";
    }
    if members
        .iter()
        .any(|m| m.name == "Index/Document.iwa" || m.name == "Index/Slide.iwa")
    {
        if metadata.format == "ZIP" {
            metadata.format = "PAGES";
        }
    }
    Ok(())
}

fn ooxml_type(content_types: &str) -> Option<&'static str> {
    let s = content_types.to_ascii_lowercase();
    if s.contains("wordprocessingml.document") {
        Some("DOCX")
    } else if s.contains("spreadsheetml.sheet") {
        Some("XLSX")
    } else if s.contains("presentationml.presentation") {
        Some("PPTX")
    } else {
        None
    }
}

fn open_doc_type(mime: &str) -> Option<&'static str> {
    let m = mime.to_ascii_lowercase();
    Some(match m.as_str() {
        "application/vnd.oasis.opendocument.text" => "ODT",
        "application/vnd.oasis.opendocument.spreadsheet" => "ODS",
        "application/vnd.oasis.opendocument.presentation" => "ODP",
        "application/vnd.oasis.opendocument.graphics" => "ODG",
        "application/vnd.oasis.opendocument.graphics-template" => "ODG",
        "application/vnd.oasis.opendocument.image" => "ODI",
        "application/vnd.oasis.opendocument.chart" => "ODC",
        "application/vnd.oasis.opendocument.formula" => "ODF",
        "application/vnd.oasis.opendocument.database" => "ODB",
        "application/vnd.adobe.indesign-idml-package" => "IDML",
        "application/epub+zip" => "EPUB",
        _ => return None,
    })
}

fn extract_simple_xml(xml: &str, prefix: &str, metadata: &mut Metadata) {
    for (tag, name) in [
        ("dc:title", "Title"),
        ("dc:creator", "Creator"),
        ("dc:description", "Description"),
        ("dc:subject", "Subject"),
        ("dcterms:created", "CreateDate"),
        ("dcterms:modified", "ModifyDate"),
        ("cp:lastModifiedBy", "LastModifiedBy"),
        ("cp:revision", "Revision"),
        ("meta:generator", "Generator"),
    ] {
        if let Some(v) = xml_text(xml, tag) {
            metadata
                .exif
                .set(format!("{prefix}:{name}"), AttrValue::Str(v));
        }
    }
}

fn xml_text(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let start = xml.find(&open)?;
    let after = xml.get(start..)?;
    let gt = after.find('>')?;
    let inner = after.get(gt + 1..)?;
    let end = inner.find(&close)?;
    Some(inner[..end].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn stored_zip(name: &str, payload: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(LOCAL_SIG);
        out.extend_from_slice(&20u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        let crc = 0u32;
        out.extend_from_slice(&crc.to_le_bytes());
        out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(payload);
        // EOCD
        let cdh_off = out.len() as u32;
        out.extend_from_slice(b"PK\x01\x02");
        out.extend_from_slice(&[0u8; 42]);
        // minimal EOCD so comment scan works
        let eocd_start = out.len();
        let _ = eocd_start;
        let _ = cdh_off;
        out.extend_from_slice(EOCD_SIG);
        out.extend_from_slice(&[0u8; 16]);
        out.extend_from_slice(&0u16.to_le_bytes());
        out
    }

    #[test]
    fn detect_pk() {
        assert!(ZipParser.can_parse(b"PK\x03\x04xxxx"));
        assert!(!ZipParser.can_parse(b"Rar!"));
    }

    #[test]
    fn parse_stored_member() {
        let data = stored_zip("hello.txt", b"hi");
        let mut c = Cursor::new(data);
        let meta = ZipParser.parse(&mut c).unwrap();
        assert_eq!(meta.format, "ZIP");
        assert_eq!(meta.exif.get_str("Zip:FileName"), Some("hello.txt"));
        assert_eq!(meta.exif.get_u32("ZipMemberCount"), Some(1));
    }

    #[test]
    fn ooxml_from_content_types() {
        let xml = br#"<?xml version="1.0"?><Types>
            <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
            </Types>"#;
        let data = stored_zip("[Content_Types].xml", xml);
        let mut c = Cursor::new(data);
        let meta = ZipParser.parse(&mut c).unwrap();
        assert_eq!(meta.format, "DOCX");
    }
}

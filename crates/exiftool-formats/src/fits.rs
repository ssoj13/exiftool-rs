//! Flexible Image Transport System header cards (ExifTool `FITS.pm`).
//!
//! Detect: first 80-byte card `SIMPLE  =` + 20 spaces + `T`.
//! Tags start at card 2 (`ProcessFITS` consumes card 1 for identification only).
//! Garmin `.fit` is a different format and does not match this magic.

use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;

const CARD: usize = 80;
const MAX_CARDS: usize = 100_000;

const SPECIAL_KEYS: &[&str] = &[
    "TABLE_NAME",
    "SHORT_NAME",
    "PROCESS_PROC",
    "WRITE_PROC",
    "CHECK_PROC",
    "GROUPS",
    "FORMAT",
    "FIRST_ENTRY",
    "TAG_PREFIX",
    "PRINT_CONV",
    "WRITABLE",
    "TABLE_DESC",
    "NOTES",
    "IS_OFFSET",
    "IS_SUBDIR",
    "EXTRACT_UNKNOWN",
    "NAMESPACE",
    "PREFERRED",
    "SRC_TABLE",
    "PRIORITY",
    "AVOID",
    "WRITE_GROUP",
    "LANG_INFO",
    "VARS",
    "DATAMEMBER",
    "SET_GROUP1",
    "PERMANENT",
    "INIT_TABLE",
];

/// FITS astronomy image parser.
pub struct FitsParser;

impl FormatParser for FitsParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        looks_like_fits(header)
    }

    fn format_name(&self) -> &'static str {
        "FITS"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["fits", "fts"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        parse_fits(reader)
    }
}

/// ExifTool `FITS.pm`: `/^SIMPLE  = {20}T/`.
#[must_use]
pub fn looks_like_fits(header: &[u8]) -> bool {
    header.len() >= 30
        && header.starts_with(b"SIMPLE  =")
        && header[9..29].iter().all(|&c| c == b' ')
        && header[29] == b'T'
}

fn parse_fits(reader: &mut dyn ReadSeek) -> Result<Metadata> {
    let mut first = [0u8; CARD];
    reader.read_exact(&mut first)?;
    if !looks_like_fits(&first) {
        return Err(Error::InvalidStructure("not FITS".into()));
    }

    let mut metadata = Metadata::new("FITS");
    metadata.set_file_type("FITS", "image/fits");

    let mut continue_val: Option<String> = None;
    let mut tag_key = String::new();
    let mut cards = 0usize;

    loop {
        if cards >= MAX_CARDS {
            metadata.exif.set(
                "FITS:Warning",
                AttrValue::Str("FITS header card limit".into()),
            );
            break;
        }
        cards += 1;
        let mut buff = [0u8; CARD];
        if reader.read_exact(&mut buff).is_err() {
            metadata.exif.set(
                "FITS:Warning",
                AttrValue::Str("Truncated FITS header".into()),
            );
            break;
        }
        let card = ascii_card(&buff);
        let mut key = card[..8.min(card.len())].to_string();
        key = key.trim_end_matches(' ').to_string();

        if key == "CONTINUE" {
            if continue_val.is_none() {
                metadata.exif.set(
                    "FITS:Warning",
                    AttrValue::Str("Unexpected FITS CONTINUE keyword".into()),
                );
                continue;
            }
        } else {
            if let Some(prev) = continue_val.take() {
                store_tag(&mut metadata, &tag_key, format!("{prev}&"));
            }
            if key == "END" {
                break;
            }
            if !valid_fits_key(&key) {
                metadata.exif.set(
                    "FITS:Warning",
                    AttrValue::Str("Format error in FITS header".into()),
                );
                break;
            }
            if key == "COMMENT" || key == "HISTORY" {
                let mut val = if card.len() > 8 {
                    card[8..].to_string()
                } else {
                    String::new()
                };
                val = val.trim_end_matches(' ').to_string();
                val = strip_leading_spaces(&val);
                store_list(&mut metadata, &fits_tag_name(&key), val);
                continue;
            }
            if card.len() < 10 || &card[8..10] != "= " {
                continue;
            }
            tag_key = if SPECIAL_KEYS.iter().any(|s| *s == key.as_str()) {
                format!("_{key}")
            } else {
                key
            };
        }

        let val_field = if card.len() > 10 { &card[10..] } else { "" };
        if let Some((mut val, _rest)) = parse_quoted(val_field) {
            val = val.trim_end_matches(' ').to_string();
            if let Some(prev) = continue_val.take() {
                val = format!("{prev}{val}");
            }
            if let Some(stripped) = val.strip_suffix('&') {
                continue_val = Some(stripped.to_string());
                continue;
            }
            store_tag(&mut metadata, &tag_key, val);
        } else if continue_val.is_some() {
            metadata.exif.set(
                "FITS:Warning",
                AttrValue::Str("Invalid FITS CONTINUE value".into()),
            );
            continue_val = None;
        } else {
            let mut val = strip_comment_and_trail(val_field);
            if val.is_empty() {
                continue;
            }
            val = strip_leading_spaces(&val);
            if is_fits_float(&val) {
                val = val.replace(['D', 'E'], "e");
            }
            store_tag(&mut metadata, &tag_key, val);
        }
    }

    Ok(metadata)
}

fn ascii_card(buff: &[u8; CARD]) -> String {
    buff.iter()
        .map(|&b| if b.is_ascii() { b as char } else { '?' })
        .collect()
}

fn valid_fits_key(key: &str) -> bool {
    key.bytes()
        .all(|b| matches!(b, b'-' | b'_' | b'A'..=b'Z' | b'0'..=b'9'))
}

fn strip_leading_spaces(s: &str) -> String {
    s.trim_start_matches(' ').to_string()
}

fn strip_comment_and_trail(s: &str) -> String {
    let s = if let Some(i) = s.find('/') {
        &s[..i]
    } else {
        s
    };
    s.trim_end_matches(' ').to_string()
}

/// Perl `/^'(.*?)'(.*)/` then escaped `''` pairs.
fn parse_quoted(s: &str) -> Option<(String, String)> {
    let s = s.trim_end_matches('\0');
    let rest = s.strip_prefix('\'')?;
    let end = rest.find('\'')?;
    let mut val = rest[..end].to_string();
    let mut buff = &rest[end + 1..];
    while let Some(after) = buff.strip_prefix('\'') {
        let Some(p) = after.find('\'') else {
            break;
        };
        val.push('\'');
        val.push_str(&after[..p]);
        buff = &after[p + 1..];
    }
    Some((val, buff.to_string()))
}

/// Perl `/^[+-]?(?=\d|\.\d)\d*(\.\d*)?([ED]([+-]?\d+))?$/`.
fn is_fits_float(s: &str) -> bool {
    let b = s.as_bytes();
    if b.is_empty() {
        return false;
    }
    let mut i = 0;
    if b[0] == b'+' || b[0] == b'-' {
        i = 1;
    }
    if i >= b.len() {
        return false;
    }
    let ok_start =
        b[i].is_ascii_digit() || (b[i] == b'.' && i + 1 < b.len() && b[i + 1].is_ascii_digit());
    if !ok_start {
        return false;
    }
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i < b.len() && b[i] == b'.' {
        i += 1;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
    }
    if i < b.len() && (b[i] == b'E' || b[i] == b'D') {
        i += 1;
        if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
            i += 1;
        }
        let exp0 = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == exp0 {
            return false;
        }
    }
    i == b.len()
}

fn fits_tag_name(key: &str) -> String {
    match key {
        "TELESCOP" => "Telescope".into(),
        "BACKGRND" => "Background".into(),
        "INSTRUME" => "Instrument".into(),
        "OBJECT" => "Object".into(),
        "OBSERVER" => "Observer".into(),
        "DATE" => "CreateDate".into(),
        "AUTHOR" => "Author".into(),
        "REFERENC" => "Reference".into(),
        "DATE-OBS" => "ObservationDate".into(),
        "TIME-OBS" => "ObservationTime".into(),
        "DATE-END" => "ObservationDateEnd".into(),
        "TIME-END" => "ObservationTimeEnd".into(),
        "COMMENT" => "Comment".into(),
        "HISTORY" => "History".into(),
        _ => unknown_tag_name(key),
    }
}

/// ExifTool: `ucfirst lc $tag` then `s/_(.)/\U$1/g`.
fn unknown_tag_name(tag: &str) -> String {
    let lower = tag.to_ascii_lowercase();
    let mut chars = lower.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    let mut name: String = first.to_uppercase().collect();
    name.extend(chars);
    let mut out = String::with_capacity(name.len());
    let bytes: Vec<char> = name.chars().collect();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == '_' && i + 1 < bytes.len() {
            out.extend(bytes[i + 1].to_uppercase());
            i += 2;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    out
}

fn store_tag(meta: &mut Metadata, key: &str, val: String) {
    meta.exif.set(fits_tag_name(key), AttrValue::Str(val));
}

fn store_list(meta: &mut Metadata, name: &str, val: String) {
    match meta.exif.get_mut(name) {
        Some(AttrValue::List(v)) => v.push(AttrValue::Str(val)),
        Some(AttrValue::Str(existing)) => {
            let first = existing.clone();
            meta.exif.set(
                name,
                AttrValue::List(vec![AttrValue::Str(first), AttrValue::Str(val)]),
            );
        }
        _ => meta.exif.set(name, AttrValue::Str(val)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::path::PathBuf;

    fn pad_card(text: &str) -> [u8; CARD] {
        let mut c = [b' '; CARD];
        let b = text.as_bytes();
        let n = b.len().min(CARD);
        c[..n].copy_from_slice(&b[..n]);
        c
    }

    fn simple_true() -> [u8; CARD] {
        pad_card("SIMPLE  =                    T")
    }

    fn fits_bytes(cards: &[[u8; CARD]]) -> Vec<u8> {
        cards.iter().flatten().copied().collect()
    }

    #[test]
    fn magic_matches_exiftool_simple_card() {
        let c = simple_true();
        assert!(looks_like_fits(&c));
        assert!(looks_like_fits(&c[..30]));
        assert!(!looks_like_fits(&c[..29]));
        assert!(!looks_like_fits(b"SIMPLE  =                   F"));
        // Garmin FIT: header size + `.FIT` at offset 8, not FITS SIMPLE.
        let mut fit = vec![14u8, 0x10, 0x02, 0x00];
        fit.extend_from_slice(&[0; 4]);
        fit.extend_from_slice(b".FIT");
        assert!(!looks_like_fits(&fit));
    }

    #[test]
    fn unknown_names_match_exiftool() {
        assert_eq!(unknown_tag_name("BITPIX"), "Bitpix");
        assert_eq!(unknown_tag_name("RA_OBJ"), "RaObj");
        assert_eq!(unknown_tag_name("OBS_ID"), "ObsId");
        assert_eq!(unknown_tag_name("MJDREFI"), "Mjdrefi");
    }

    #[test]
    fn quoted_escapes_and_float_reformat() {
        let (v, _) = parse_quoted("'O''Brien' / comment").unwrap();
        assert_eq!(v, "O'Brien");
        assert!(is_fits_float("3.37843167E+00"));
        assert_eq!("3.37843167E+00".replace(['D', 'E'], "e"), "3.37843167e+00");
        assert!(!is_fits_float("T"));
    }

    #[test]
    fn parse_minimal_header() {
        let data = fits_bytes(&[
            simple_true(),
            pad_card("BITPIX  =                    8 / bits"),
            pad_card("NAXIS   =                    0"),
            pad_card("OBJECT  = '47_TUCANAE'"),
            pad_card("DATE-OBS= '09/12/96'"),
            pad_card("COMMENT   hello"),
            pad_card("COMMENT   world"),
            pad_card("END"),
        ]);
        let mut cur = Cursor::new(data);
        let meta = FitsParser.parse(&mut cur).unwrap();
        assert_eq!(meta.format, "FITS");
        assert_eq!(meta.exif.get_str("Bitpix"), Some("8"));
        assert_eq!(meta.exif.get_str("Naxis"), Some("0"));
        assert_eq!(meta.exif.get_str("Object"), Some("47_TUCANAE"));
        assert_eq!(meta.exif.get_str("ObservationDate"), Some("09/12/96"));
        assert!(!meta.is_writable());
        match meta.exif.get("Comment") {
            Some(AttrValue::List(v)) => {
                assert_eq!(v.len(), 2);
                assert_eq!(v[0], AttrValue::Str("hello".into()));
                assert_eq!(v[1], AttrValue::Str("world".into()));
            }
            other => panic!("expected Comment list, got {other:?}"),
        }
    }

    #[test]
    fn continue_concatenates_quoted_string() {
        let data = fits_bytes(&[
            simple_true(),
            pad_card("LONGVAL = 'hello&'"),
            pad_card("CONTINUE  ' world'"),
            pad_card("END"),
        ]);
        let mut cur = Cursor::new(data);
        let meta = FitsParser.parse(&mut cur).unwrap();
        assert_eq!(meta.exif.get_str("Longval"), Some("hello world"));
    }

    #[test]
    fn sample_fits_from_exiftool_t() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../_ref/exiftool/t/images/FITS.fits");
        if !path.exists() {
            return;
        }
        let data = std::fs::read(&path).unwrap();
        assert!(looks_like_fits(&data));
        let mut cur = Cursor::new(data);
        let meta = FitsParser.parse(&mut cur).unwrap();
        assert_eq!(meta.exif.get_str("Bitpix"), Some("8"));
        assert_eq!(meta.exif.get_str("Naxis"), Some("0"));
        assert_eq!(meta.exif.get_str("Object"), Some("47_TUCANAE_Slew"));
        assert_eq!(meta.exif.get_str("ObservationDate"), Some("09/12/96"));
        assert_eq!(meta.exif.get_str("CreateDate"), Some("28/01/97"));
        assert_eq!(meta.exif.get_str("RaObj"), Some("6.02170000e+00"));
        assert_eq!(meta.exif.get_str("ObsId"), Some("10080-01-02-00A"));
        match meta.exif.get("Comment") {
            Some(AttrValue::List(v)) => assert!(v.len() >= 5),
            Some(AttrValue::Str(_)) => {}
            other => panic!("missing Comment, got {other:?}"),
        }
    }
}

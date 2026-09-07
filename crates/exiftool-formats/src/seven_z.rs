//! 7z archives (ExifTool `7Z.pm`, tags from `ZIP.pm` RAR5 table).
//!
//! Magic `7z\xbc\xaf\x27\x1c`. Unencoded header (id 1) lists files. Encoded header
//! (id 23) is LZMA-compressed StreamsInfo + packed payload (ExifTool `Compress::Raw::Lzma`).

use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::{self, Cursor, Read, SeekFrom};

const MAGIC: &[u8; 6] = b"7z\xbc\xaf\x27\x1c";
const MAX_FILES: u64 = 50_000;
const MAX_HEADER: u64 = 16 * 1024 * 1024;

/// 7z archive parser.
pub struct SevenZParser;

impl FormatParser for SevenZParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 6 && header.starts_with(MAGIC)
    }

    fn format_name(&self) -> &'static str {
        "7Z"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["7z"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        parse_7z(reader)
    }
}

struct SevenFile {
    filename: String,
    lastwritetime: Option<f64>,
}

struct Coder {
    method: Vec<u8>,
    properties: Vec<u8>,
    num_out: u64,
}

struct Folder {
    coders: Vec<Coder>,
    unpacksizes: Vec<u64>,
    digestdefined: bool,
}

struct PackInfo {
    packpos: u64,
    packsizes: Vec<u64>,
}

struct StreamsInfo {
    pack: PackInfo,
    folders: Vec<Folder>,
}

fn parse_7z(reader: &mut dyn ReadSeek) -> Result<Metadata> {
    reader.seek(SeekFrom::Start(0))?;
    let mut mag = [0u8; 6];
    reader.read_exact(&mut mag)?;
    if &mag != MAGIC {
        return Err(Error::InvalidStructure("not 7z".into()));
    }
    let mut ver = [0u8; 2];
    reader.read_exact(&mut ver)?;
    let mut metadata = Metadata::new("7Z");
    metadata.set_file_type("7Z", "application/x-7z-compressed");
    metadata.exif.set(
        "Zip:FileVersion",
        AttrValue::Str(format!("7z v{}.{:02}", ver[0], ver[1])),
    );
    reader.seek(SeekFrom::Current(4))?;
    let mut nh = [0u8; 20];
    reader.read_exact(&mut nh)?;
    let next_off = u64::from_le_bytes(nh[0..8].try_into().unwrap());
    let next_size = u64::from_le_bytes(nh[8..16].try_into().unwrap());
    if next_size == 0 || next_size > MAX_HEADER {
        metadata
            .exif
            .set("Zip:Warning", AttrValue::Str("7z header too large".into()));
        return Ok(metadata);
    }
    reader.seek(SeekFrom::Start(32 + next_off))?;
    let mut pidb = [0u8; 1];
    reader.read_exact(&mut pidb)?;
    let pid = pidb[0];
    let mut rest = vec![0u8; next_size.saturating_sub(1) as usize];
    if !rest.is_empty() {
        reader.read_exact(&mut rest)?;
    }
    let mut cur = Cursor::new(rest);
    if pid == 1 {
        match extract_header_info(&mut cur) {
            Ok(files) => apply_files(&mut metadata, files),
            Err(e) => {
                metadata.exif.set(
                    "Zip:Warning",
                    AttrValue::Str(format!("Invalid or corrupted file: {e}")),
                );
            }
        }
    } else if pid == 23 {
        match decode_encoded_header(reader, &mut cur) {
            Ok(files) => apply_files(&mut metadata, files),
            Err(e) => {
                metadata.exif.set(
                    "Zip:Warning",
                    AttrValue::Str(format!("Encoded 7z header: {e}")),
                );
            }
        }
    } else {
        return Err(Error::InvalidStructure(format!(
            "unknown 7z header id {pid}"
        )));
    }
    let _ = next_size;
    Ok(metadata)
}

fn apply_files(metadata: &mut Metadata, files: Vec<SevenFile>) {
    let names: Vec<AttrValue> = files
        .iter()
        .map(|f| AttrValue::Str(f.filename.clone()))
        .collect();
    if names.len() == 1 {
        metadata
            .exif
            .set("Zip:ArchivedFileName", names.into_iter().next().unwrap());
    } else if !names.is_empty() {
        metadata
            .exif
            .set("Zip:ArchivedFileName", AttrValue::List(names));
    }
    if let Some(ts) = files.iter().find_map(|f| f.lastwritetime) {
        metadata
            .exif
            .set("Zip:ModifyDate", AttrValue::Str(unix_exif(ts)));
    }
}

fn unix_exif(ts: f64) -> String {
    let sec = ts as i64;
    let days = sec.div_euclid(86400);
    let tod = sec.rem_euclid(86400) as u32;
    let (y, m, d) = civil_from_days(days);
    let h = tod / 3600;
    let mi = (tod % 3600) / 60;
    let s = tod % 60;
    format!("{y:04}:{m:02}:{d:02} {h:02}:{mi:02}:{s:02}")
}

/// Howard Hinnant `civil_from_days` (days since 1970-01-01).
fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m, d)
}

fn decode_encoded_header(
    reader: &mut dyn ReadSeek,
    streams_src: &mut Cursor<Vec<u8>>,
) -> io::Result<Vec<SevenFile>> {
    let streams = read_streams_info(streams_src)?;
    let folder = streams
        .folders
        .first()
        .ok_or_else(|| ioerr("encoded header has no folder"))?;
    let coder = folder
        .coders
        .first()
        .ok_or_else(|| ioerr("encoded header has no coder"))?;
    match native_coder(coder) {
        NativeCoder::Aes => return Err(ioerr("File is encrypted.")),
        NativeCoder::Lzma => {}
        NativeCoder::Other => {
            return Err(ioerr("encoded 7z header is not LZMA"));
        }
    }
    let pack_size = *streams.pack.packsizes.first().unwrap_or(&0);
    if pack_size == 0 || pack_size > MAX_HEADER {
        return Err(ioerr("encoded header pack size"));
    }
    let unpacked = *folder.unpacksizes.last().unwrap_or(&0);
    if unpacked == 0 || unpacked > MAX_HEADER {
        return Err(ioerr("encoded header unpack size"));
    }
    let src = 32u64.saturating_add(streams.pack.packpos);
    reader.seek(SeekFrom::Start(src))?;
    let mut packed = vec![0u8; pack_size as usize];
    reader.read_exact(&mut packed)?;
    let decoded = lzma1_decode(&coder.properties, &packed, unpacked)?;
    let mut header = Cursor::new(decoded);
    let hid = read_u8(&mut header)?;
    if hid != 1 {
        return Err(ioerr("header field expected after LZMA"));
    }
    extract_header_info(&mut header)
}

enum NativeCoder {
    Lzma,
    Aes,
    Other,
}

fn native_coder(coder: &Coder) -> NativeCoder {
    let m = &coder.method;
    if m.len() >= 3 && m[0] == 3 && m[1] == 1 && m[2] == 1 {
        return NativeCoder::Lzma;
    }
    if m.len() >= 4 && m[0] == 6 && m[1] == 0xf1 && m[2] == 7 && m[3] == 1 {
        return NativeCoder::Aes;
    }
    NativeCoder::Other
}

/// 7z stores LZMA properties on the coder; packed payload has no `.lzma` header.
/// Rebuild the 13-byte `.lzma` prefix (5-byte props + 8-byte unpack size) for `lzma-rs`.
fn lzma1_decode(props: &[u8], packed: &[u8], unpacked: u64) -> io::Result<Vec<u8>> {
    if props.len() != 5 {
        return Err(ioerr("LZMA properties must be 5 bytes"));
    }
    let mut input = Vec::with_capacity(13 + packed.len());
    input.extend_from_slice(props);
    input.extend_from_slice(&unpacked.to_le_bytes());
    input.extend_from_slice(packed);
    let mut out = Vec::new();
    let mut input_cur = Cursor::new(input);
    lzma_rs::lzma_decompress(&mut input_cur, &mut out).map_err(|e| ioerr(&e.to_string()))?;
    Ok(out)
}

fn extract_header_info<R: Read>(r: &mut R) -> io::Result<Vec<SevenFile>> {
    let pid = read_u8(r)?;
    if pid == 0x04 {
        read_streams_info(r)?;
        let pid = read_u8(r)?;
        if pid == 0x05 {
            return read_files_info(r);
        }
        if pid != 0 {
            return Err(ioerr("ExtractHeaderInfo pid after streams"));
        }
        return Ok(Vec::new());
    }
    if pid == 0x05 {
        return read_files_info(r);
    }
    if pid == 0 {
        return Ok(Vec::new());
    }
    Err(ioerr("ExtractHeaderInfo"))
}

fn read_streams_info<R: Read>(r: &mut R) -> io::Result<StreamsInfo> {
    let mut pid = read_u8(r)?;
    let mut pack = PackInfo {
        packpos: 0,
        packsizes: Vec::new(),
    };
    if pid == 6 {
        pack = read_pack_info(r)?;
        pid = read_u8(r)?;
    }
    let mut folders = Vec::new();
    if pid == 7 {
        folders = read_unpack_info(r)?;
        pid = read_u8(r)?;
    }
    if pid == 8 {
        read_substreams_info(r, &folders)?;
        pid = read_u8(r)?;
    }
    if pid != 0 {
        return Err(ioerr("ReadStreamsInfo end"));
    }
    Ok(StreamsInfo { pack, folders })
}

fn read_pack_info<R: Read>(r: &mut R) -> io::Result<PackInfo> {
    let packpos = read_packed_u64(r)?;
    let numstreams = read_packed_u64(r)?;
    if numstreams > MAX_FILES {
        return Err(ioerr("too many pack streams"));
    }
    let mut packsizes = Vec::new();
    let mut pid = read_u8(r)?;
    if pid == 9 {
        for _ in 0..numstreams {
            packsizes.push(read_packed_u64(r)?);
        }
        pid = read_u8(r)?;
        if pid == 10 {
            let defined = read_boolean(r, numstreams as usize, true)?;
            for d in defined {
                if d {
                    let _ = read_u32_le(r)?;
                }
            }
            pid = read_u8(r)?;
        }
    }
    if pid != 0 {
        return Err(ioerr("ReadPackInfo end"));
    }
    Ok(PackInfo { packpos, packsizes })
}

fn read_unpack_info<R: Read>(r: &mut R) -> io::Result<Vec<Folder>> {
    let pid = read_u8(r)?;
    if pid != 0x0b {
        return Err(ioerr("folder id"));
    }
    let numfolders = read_packed_u64(r)?;
    if numfolders > MAX_FILES {
        return Err(ioerr("too many folders"));
    }
    let external = read_u8(r)?;
    if external != 0 {
        return Err(ioerr("external folders"));
    }
    let mut folders = Vec::new();
    for _ in 0..numfolders {
        folders.push(read_folder(r)?);
    }
    retrieve_coders_info(r, &mut folders)?;
    Ok(folders)
}

fn read_folder<R: Read>(r: &mut R) -> io::Result<Folder> {
    let num_coders = read_packed_u64(r)?;
    if num_coders > 256 {
        return Err(ioerr("too many coders"));
    }
    let mut totalin = 0u64;
    let mut totalout = 0u64;
    let mut coders = Vec::new();
    for _ in 0..num_coders {
        let b = read_u8(r)?;
        let methodsize = (b & 0x0f) as usize;
        let iscomplex = b & 0x10 != 0;
        let hasattr = b & 0x20 != 0;
        let mut method = vec![0u8; methodsize];
        if methodsize > 0 {
            r.read_exact(&mut method)?;
        }
        let (nin, nout) = if iscomplex {
            (read_packed_u64(r)?, read_packed_u64(r)?)
        } else {
            (1, 1)
        };
        totalin += nin;
        totalout += nout;
        let mut properties = Vec::new();
        if hasattr {
            let proplen = read_packed_u64(r)?;
            properties.resize(proplen as usize, 0);
            r.read_exact(&mut properties)?;
        }
        coders.push(Coder {
            method,
            properties,
            num_out: nout,
        });
    }
    let num_bindpairs = totalout.saturating_sub(1);
    for _ in 0..num_bindpairs {
        let _ = read_packed_u64(r)?;
        let _ = read_packed_u64(r)?;
    }
    let num_packed = totalin.saturating_sub(num_bindpairs);
    if num_packed != 1 {
        for _ in 0..num_packed {
            let _ = read_packed_u64(r)?;
        }
    }
    Ok(Folder {
        coders,
        unpacksizes: Vec::new(),
        digestdefined: false,
    })
}

fn retrieve_coders_info<R: Read>(r: &mut R, folders: &mut [Folder]) -> io::Result<()> {
    let pid = read_u8(r)?;
    if pid != 0x0c {
        return Err(ioerr("coders unpack size"));
    }
    for folder in folders.iter_mut() {
        folder.unpacksizes.clear();
        for c in &folder.coders {
            for _ in 0..c.num_out {
                folder.unpacksizes.push(read_packed_u64(r)?);
            }
        }
    }
    let mut pid = read_u8(r)?;
    if pid == 0x0a {
        let defined = read_boolean(r, folders.len(), true)?;
        for (i, d) in defined.into_iter().enumerate() {
            folders[i].digestdefined = d;
            if d {
                let _ = read_u32_le(r)?;
            }
        }
        pid = read_u8(r)?;
    }
    if pid != 0 {
        return Err(ioerr("RetrieveCodersInfo end"));
    }
    Ok(())
}

fn read_substreams_info<R: Read>(r: &mut R, folders: &[Folder]) -> io::Result<()> {
    let numfolders = folders.len();
    let mut pid = read_u8(r)?;
    let mut num_unpack = vec![1u64; numfolders];
    if pid == 13 {
        for slot in &mut num_unpack {
            *slot = read_packed_u64(r)?;
        }
        pid = read_u8(r)?;
    }
    if pid == 9 {
        for &n in &num_unpack {
            for _ in 1..n {
                let _ = read_packed_u64(r)?;
            }
        }
        pid = read_u8(r)?;
    }
    let mut num_digests = 0usize;
    for (i, folder) in folders.iter().enumerate() {
        let n = num_unpack[i] as usize;
        if n != 1 || !folder.digestdefined {
            num_digests += n;
        }
    }
    if pid == 10 {
        let _defined = read_boolean(r, num_digests, true)?;
        // ExifTool 7Z.pm reads a CRC for every digest slot (not only defined=true).
        for _ in 0..num_digests {
            let _ = read_u32_le(r)?;
        }
        pid = read_u8(r)?;
    }
    if pid != 0 {
        return Err(ioerr("ReadSubstreamsInfo end"));
    }
    Ok(())
}

fn read_files_info<R: Read>(r: &mut R) -> io::Result<Vec<SevenFile>> {
    let numfiles = read_packed_u64(r)?;
    if numfiles > MAX_FILES {
        return Err(ioerr("too many files"));
    }
    let n = numfiles as usize;
    let mut files: Vec<SevenFile> = (0..n)
        .map(|_| SevenFile {
            filename: String::new(),
            lastwritetime: None,
        })
        .collect();
    loop {
        let prop = read_u8(r)?;
        if prop == 0 {
            return Ok(files);
        }
        let size = read_packed_u64(r)?;
        if size > MAX_HEADER {
            return Err(ioerr("files prop too large"));
        }
        if prop == 25 {
            skip_n(r, size as usize)?;
            continue;
        }
        let mut buf = vec![0u8; size as usize];
        r.read_exact(&mut buf)?;
        let mut cur = Cursor::new(buf);
        match prop {
            14 => {
                let _ = read_boolean(&mut cur, n, false)?;
            }
            17 => {
                let ext = read_u8(&mut cur)?;
                if ext == 0 {
                    for f in &mut files {
                        f.filename = read_utf16(&mut cur)?;
                    }
                }
            }
            20 => {
                let defined = read_boolean(&mut cur, n, true)?;
                let ext = read_u8(&mut cur)?;
                if ext != 0 {
                    return Err(ioerr("ReadTimes external"));
                }
                for (i, d) in defined.into_iter().enumerate() {
                    if d {
                        let value = read_u64_le(&mut cur)?;
                        files[i].lastwritetime =
                            Some(value as f64 / 10_000_000.0 - 11_644_473_600.0);
                    }
                }
            }
            21 => {
                let defined = read_boolean(&mut cur, n, true)?;
                let ext = read_u8(&mut cur)?;
                if ext == 0 {
                    for d in defined {
                        if d {
                            let _ = read_u32_le(&mut cur)?;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn read_utf16<R: Read>(r: &mut R) -> io::Result<String> {
    let mut units = Vec::new();
    for _ in 0..65536 {
        let mut p = [0u8; 2];
        r.read_exact(&mut p)?;
        if p == [0, 0] {
            break;
        }
        units.push(u16::from_le_bytes(p));
    }
    Ok(String::from_utf16_lossy(&units))
}

fn read_boolean<R: Read>(r: &mut R, count: usize, checkall: bool) -> io::Result<Vec<bool>> {
    if checkall {
        let all = read_u8(r)?;
        if all != 0 {
            return Ok(vec![true; count]);
        }
    }
    let mut result = Vec::with_capacity(count);
    let mut b = 0u8;
    let mut mask = 0u8;
    for _ in 0..count {
        if mask == 0 {
            b = read_u8(r)?;
            mask = 0x80;
        }
        result.push(b & mask != 0);
        mask >>= 1;
    }
    Ok(result)
}

fn read_packed_u64<R: Read>(r: &mut R) -> io::Result<u64> {
    let b = read_u8(r)?;
    if b == 255 {
        return read_u64_le(r);
    }
    let blen = [0x7Fu8, 0xBF, 0xDF, 0xEF, 0xF7, 0xFB, 0xFD, 0xFE];
    let mut mask = 0x80u8;
    let mut vlen = 8usize;
    for (l, v) in blen.iter().enumerate() {
        if b <= *v {
            vlen = l;
            break;
        }
        mask >>= 1;
    }
    if vlen == 0 {
        return Ok(u64::from(b & mask.wrapping_sub(1)));
    }
    let mut rest = vec![0u8; vlen];
    r.read_exact(&mut rest)?;
    rest.resize(8, 0);
    let mut arr = [0u8; 8];
    arr.copy_from_slice(&rest[..8]);
    let value = u64::from_le_bytes(arr);
    let high = u64::from(b & mask.wrapping_sub(1));
    Ok(value + (high << (vlen * 8)))
}

fn read_u8<R: Read>(r: &mut R) -> io::Result<u8> {
    let mut b = [0u8; 1];
    r.read_exact(&mut b)?;
    Ok(b[0])
}

fn read_u32_le<R: Read>(r: &mut R) -> io::Result<u32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}

fn read_u64_le<R: Read>(r: &mut R) -> io::Result<u64> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b)?;
    Ok(u64::from_le_bytes(b))
}

fn skip_n<R: Read>(r: &mut R, n: usize) -> io::Result<()> {
    let mut left = n;
    let mut tmp = [0u8; 256];
    while left > 0 {
        let k = left.min(tmp.len());
        r.read_exact(&mut tmp[..k])?;
        left -= k;
    }
    Ok(())
}

fn ioerr(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn magic_7z() {
        assert!(SevenZParser.can_parse(MAGIC));
        assert!(!SevenZParser.can_parse(b"PK\x03\x04"));
    }

    #[test]
    fn parse_fixture_seven_7z() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/seven.7z");
        if !path.exists() {
            return;
        }
        let data = std::fs::read(&path).unwrap();
        let mut cur = Cursor::new(data);
        let meta = SevenZParser.parse(&mut cur).unwrap();
        assert_eq!(meta.format, "7Z");
        assert_eq!(meta.exif.get_str("Zip:FileVersion"), Some("7z v0.04"));
        assert_eq!(meta.exif.get_str("Zip:ArchivedFileName"), Some("hello.txt"));
    }

    #[test]
    fn parse_fixture_encoded_7z() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/encoded.7z");
        if !path.exists() {
            return;
        }
        let data = std::fs::read(&path).unwrap();
        let mut cur = Cursor::new(data);
        let meta = SevenZParser.parse(&mut cur).unwrap();
        assert_eq!(meta.format, "7Z");
        assert!(
            meta.exif.get_str("Zip:Warning").is_none(),
            "{:?}",
            meta.exif.get_str("Zip:Warning")
        );
        match meta.exif.get("Zip:ArchivedFileName") {
            Some(AttrValue::List(names)) => {
                assert_eq!(names.len(), 40);
                assert_eq!(
                    names[0],
                    AttrValue::Str("file_00_with_a_reasonably_long_name.txt".into())
                );
            }
            other => panic!("expected 40 archived names, got {other:?}"),
        }
    }
}

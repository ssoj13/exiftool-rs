//! JPEG format parser.
//!
//! JPEG files consist of segments, each starting with 0xFF marker:
//! - SOI (0xFFD8) - Start of Image
//! - APP0 (0xFFE0) - JFIF
//! - APP1 (0xFFE1) - EXIF or XMP
//! - APP2 (0xFFE2) - ICC Profile
//! - DQT, DHT, SOF, SOS... - image data
//! - EOI (0xFFD9) - End of Image

use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use crate::utils::{parse_tiff_exif, ParseTiffExifOptions};
use exiftool_attrs::AttrValue;
use exiftool_xmp::XmpParser;
use crate::iptc::IptcParser;
use exiftool_icc::IccParser;
use exiftool_core::{ByteOrder, IfdReader};
use std::collections::HashMap;
use std::io::SeekFrom;

/// JPEG format parser.
pub struct JpegParser;

impl FormatParser for JpegParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 2 && header[0] == 0xFF && header[1] == 0xD8
    }

    fn format_name(&self) -> &'static str {
        "JPEG"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["jpg", "jpeg", "jpe"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("JPEG");

        // Verify SOI marker
        let mut soi = [0u8; 2];
        reader.read_exact(&mut soi)?;
        if soi != [0xFF, 0xD8] {
            return Err(Error::InvalidStructure("missing JPEG SOI marker".into()));
        }

        // Track ICC profile chunks for multi-segment profiles
        let mut icc_chunks: Vec<(u8, u8, Vec<u8>)> = Vec::new();
        // Adobe Extended XMP: GUID -> (full_length, offset+payload pieces)
        let mut ext_xmp: HashMap<String, (u32, Vec<(u32, Vec<u8>)>)> = HashMap::new();

        // Read segments until SOS or EOI
        loop {
            let mut marker = [0u8; 2];
            if reader.read_exact(&mut marker).is_err() {
                break;
            }

            if marker[0] != 0xFF {
                return Err(Error::InvalidStructure("invalid JPEG marker".into()));
            }

            // Skip padding 0xFF bytes
            let mut marker_id = marker[1];
            while marker_id == 0xFF {
                let mut b = [0u8; 1];
                reader.read_exact(&mut b)?;
                marker_id = b[0];
            }

            // EOI — any bytes after this are a JPEG trailer (AFCP, CanonVRD, …).
            if marker_id == 0xD9 {
                apply_jpeg_trailer(reader, &mut metadata);
                break;
            }
            // SOS — skip scan, then EOI, then trailer.
            if marker_id == 0xDA {
                skip_sos_and_scan(reader)?;
                apply_jpeg_trailer(reader, &mut metadata);
                break;
            }

            // Standalone markers (RST, TEM) - no length
            if (0xD0..=0xD7).contains(&marker_id) || marker_id == 0x01 {
                continue;
            }

            // Read segment length (includes length bytes)
            let mut len_bytes = [0u8; 2];
            reader.read_exact(&mut len_bytes)?;
            let seg_len = u16::from_be_bytes(len_bytes) as usize;

            if seg_len < 2 {
                return Err(Error::InvalidStructure("invalid segment length".into()));
            }

            let data_len = seg_len - 2;
            let seg_start = reader.stream_position()? as usize;

            match marker_id {
                0xE0 => {
                    // APP0 - JFIF/JFXX
                    let mut data = vec![0u8; data_len];
                    reader.read_exact(&mut data)?;
                    parse_jfif(&data, &mut metadata);
                }
                0xE1 => {
                    // APP1 - EXIF or XMP
                    let mut data = vec![0u8; data_len];
                    reader.read_exact(&mut data)?;

                    if data.starts_with(b"Exif\x00\x00") {
                        let tiff_data = &data[6..];
                        metadata.exif_offset = Some(seg_start + 6);
                        parse_tiff_exif(
                            tiff_data,
                            &mut metadata.exif,
                            Some(&mut metadata.thumbnail),
                            ParseTiffExifOptions {
                                extract_thumbnail: true,
                            },
                        )?;
                    } else if data.starts_with(b"http://ns.adobe.com/xap/1.0/\x00") {
                        let xmp_start = b"http://ns.adobe.com/xap/1.0/\x00".len();
                        let xmp_data = &data[xmp_start..];
                        
                        let xmp = if let Ok(s) = String::from_utf8(xmp_data.to_vec()) {
                            Some(s)
                        } else {
                            decode_utf16(xmp_data)
                        };
                        
                        if let Some(xmp) = xmp {
                            if let Ok(xmp_attrs) = XmpParser::parse(&xmp) {
                                for (key, value) in xmp_attrs.iter() {
                                    metadata.exif.set(format!("XMP:{}", key), value.clone());
                                }
                            }
                            metadata.xmp = Some(xmp);
                        }
                    } else if data.starts_with(b"http://ns.adobe.com/xmp/extension/\0") {
                        collect_extended_xmp(&data, &mut ext_xmp);
                    }
                }
                0xE2 => {
                    // APP2 - ICC Profile
                    let mut data = vec![0u8; data_len];
                    reader.read_exact(&mut data)?;
                    
                    if data.starts_with(b"ICC_PROFILE\x00") && data.len() > 14 {
                        let chunk_num = data[12];
                        let total_chunks = data[13];
                        let chunk_data = data[14..].to_vec();
                        icc_chunks.push((chunk_num, total_chunks, chunk_data));
                    } else if data.starts_with(b"MPF\x00") && data.len() > 8 {
                        parse_mpf(&data[4..], &mut metadata);
                    }
                }
                0xC0 | 0xC1 | 0xC2 | 0xC3 | 0xC5 | 0xC6 | 0xC7 |
                0xC9 | 0xCA | 0xCB | 0xCD | 0xCE | 0xCF => {
                    // SOF - Start of Frame (image dimensions)
                    let mut data = vec![0u8; data_len];
                    reader.read_exact(&mut data)?;
                    parse_sof(marker_id, &data, &mut metadata);
                }
                0xEC => {
                    // APP12 - Ducky (Photoshop Save for Web quality)
                    let mut data = vec![0u8; data_len];
                    reader.read_exact(&mut data)?;
                    if data.starts_with(b"Ducky") {
                        parse_ducky(&data, &mut metadata);
                    }
                }
                0xED => {
                    // APP13 - IPTC/Photoshop IRB
                    let mut data = vec![0u8; data_len];
                    reader.read_exact(&mut data)?;
                    if data.starts_with(b"Photoshop 3.0\x00") {
                        parse_photoshop_irb(&data, &mut metadata);
                    }
                }
                0xEE => {
                    // APP14 - Adobe color transform
                    let mut data = vec![0u8; data_len];
                    reader.read_exact(&mut data)?;
                    if data.starts_with(b"Adobe") && data.len() >= 12 {
                        let transform = data[11];
                        let transform_name = match transform {
                            0 => "Unknown (RGB or CMYK)",
                            1 => "YCbCr",
                            2 => "YCCK",
                            _ => "Unknown",
                        };
                        metadata.exif.set("AdobeColorTransform", AttrValue::Str(transform_name.into()));
                    }
                }
                0xFE => {
                    // COM - Comment
                    let mut data = vec![0u8; data_len];
                    reader.read_exact(&mut data)?;
                    if let Ok(comment) = String::from_utf8(data.clone()) {
                        let comment = comment.trim_end_matches('\0').trim();
                        if !comment.is_empty() {
                            metadata.exif.set("Comment", AttrValue::Str(comment.to_string()));
                        }
                    }
                }
                _ => {
                    // Skip other segments
                    reader.seek(SeekFrom::Current(data_len as i64))?;
                }
            }
        }

        // Process ICC profile if present
        if !icc_chunks.is_empty() {
            parse_icc_profile(&mut icc_chunks, &mut metadata);
        }

        if let Some(xml) = assemble_extended_xmp(&ext_xmp) {
            if let Ok(xmp_attrs) = XmpParser::parse(&xml) {
                for (key, value) in xmp_attrs.iter() {
                    metadata.exif.set(format!("XMP:{}", key), value.clone());
                }
            }
            metadata.xmp = Some(xml);
        }

        Ok(metadata)
    }
}

/// Parse JFIF APP0 segment.
fn parse_jfif(data: &[u8], metadata: &mut Metadata) {
    if data.starts_with(b"JFIF\x00") && data.len() >= 14 {
        let version_major = data[5];
        let version_minor = data[6];
        metadata.exif.set("JFIFVersion", AttrValue::Str(format!("{}.{:02}", version_major, version_minor)));
        
        let units = data[7];
        let x_density = u16::from_be_bytes([data[8], data[9]]);
        let y_density = u16::from_be_bytes([data[10], data[11]]);
        
        let unit_str = match units {
            0 => "aspect ratio",
            1 => "dpi",
            2 => "dpcm",
            _ => "unknown",
        };
        
        if x_density > 0 && y_density > 0 {
            metadata.exif.set("XResolution", AttrValue::UInt(x_density as u32));
            metadata.exif.set("YResolution", AttrValue::UInt(y_density as u32));
            metadata.exif.set("ResolutionUnit", AttrValue::Str(unit_str.to_string()));
        }
        
        // Thumbnail dimensions (if present)
        let thumb_w = data[12];
        let thumb_h = data[13];
        if thumb_w > 0 && thumb_h > 0 {
            metadata.exif.set("ThumbnailWidth", AttrValue::UInt(thumb_w as u32));
            metadata.exif.set("ThumbnailHeight", AttrValue::UInt(thumb_h as u32));
        }
    } else if data.starts_with(b"JFXX\x00") && data.len() >= 6 {
        // JFXX extension
        let ext_code = data[5];
        let ext_type = match ext_code {
            0x10 => "JPEG thumbnail",
            0x11 => "1 byte/pixel thumbnail",
            0x13 => "3 byte/pixel thumbnail",
            _ => "unknown",
        };
        metadata.exif.set("JFXXExtension", AttrValue::Str(ext_type.to_string()));
    }
}

/// Parse SOF (Start of Frame) for image dimensions.
fn parse_sof(marker: u8, data: &[u8], metadata: &mut Metadata) {
    if data.len() < 6 {
        return;
    }
    
    let precision = data[0];
    let height = u16::from_be_bytes([data[1], data[2]]);
    let width = u16::from_be_bytes([data[3], data[4]]);
    let components = data[5];
    
    metadata.exif.set("ImageWidth", AttrValue::UInt(width as u32));
    metadata.exif.set("ImageHeight", AttrValue::UInt(height as u32));
    metadata.exif.set("BitsPerSample", AttrValue::UInt(precision as u32));
    metadata.exif.set("ColorComponents", AttrValue::UInt(components as u32));
    
    // Compression type based on SOF marker
    let compression = match marker {
        0xC0 => "Baseline DCT",
        0xC1 => "Extended Sequential DCT",
        0xC2 => "Progressive DCT",
        0xC3 => "Lossless",
        0xC5 => "Differential Sequential DCT",
        0xC6 => "Differential Progressive DCT",
        0xC7 => "Differential Lossless",
        0xC9 => "Extended Sequential DCT (Arithmetic)",
        0xCA => "Progressive DCT (Arithmetic)",
        0xCB => "Lossless (Arithmetic)",
        0xCD => "Differential Sequential (Arithmetic)",
        0xCE => "Differential Progressive (Arithmetic)",
        0xCF => "Differential Lossless (Arithmetic)",
        _ => "Unknown",
    };
    metadata.exif.set("Compression", AttrValue::Str(compression.to_string()));
}

/// Parse ICC profile chunks using exiftool-icc crate.
fn parse_icc_profile(chunks: &mut [(u8, u8, Vec<u8>)], metadata: &mut Metadata) {
    // Sort by chunk number
    chunks.sort_by_key(|(num, _, _)| *num);
    
    // Concatenate chunks
    let mut profile_data = Vec::new();
    for (_, _, data) in chunks {
        profile_data.extend_from_slice(data);
    }
    
    // Parse with IccParser
    match IccParser::parse(&profile_data) {
        Ok(icc_attrs) => {
            // Copy all ICC attributes to metadata
            for (key, value) in icc_attrs.iter() {
                metadata.exif.set(key.clone(), value.clone());
            }
        }
        Err(_) => {
            // Fallback: just store size
            metadata.exif.set("ICC:ProfileSize", AttrValue::UInt(profile_data.len() as u32));
        }
    }
}

/// Parse Ducky APP12 segment (Photoshop Save for Web).
fn parse_ducky(data: &[u8], metadata: &mut Metadata) {
    if !data.starts_with(b"Ducky") || data.len() < 8 {
        return;
    }
    
    let mut pos = 5; // Skip "Ducky"
    
    while pos + 4 <= data.len() {
        let tag = u16::from_be_bytes([data[pos], data[pos + 1]]);
        let len = u16::from_be_bytes([data[pos + 2], data[pos + 3]]) as usize;
        pos += 4;
        
        if pos + len > data.len() {
            break;
        }
        
        match tag {
            1 => {
                // Quality
                if len >= 4 {
                    let quality = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
                    metadata.exif.set("DuckyQuality", AttrValue::UInt(quality));
                }
            }
            2 => {
                // Comment
                if let Ok(comment) = String::from_utf8(data[pos..pos + len].to_vec()) {
                    let comment = comment.trim_end_matches('\0').trim();
                    if !comment.is_empty() {
                        metadata.exif.set("DuckyComment", AttrValue::Str(comment.to_string()));
                    }
                }
            }
            3 => {
                // Copyright
                if let Ok(copyright) = String::from_utf8(data[pos..pos + len].to_vec()) {
                    let copyright = copyright.trim_end_matches('\0').trim();
                    if !copyright.is_empty() {
                        metadata.exif.set("DuckyCopyright", AttrValue::Str(copyright.to_string()));
                    }
                }
            }
            _ => {}
        }
        
        pos += len;
    }
}

/// Parse Photoshop IRB APP13 segment (contains IPTC).
fn parse_photoshop_irb(data: &[u8], metadata: &mut Metadata) {
    const HEADER: &[u8] = b"Photoshop 3.0\x00";
    if !data.starts_with(HEADER) {
        return;
    }
    
    let mut pos = HEADER.len();
    
    // Parse 8BIM resources
    while pos + 12 <= data.len() {
        // 8BIM signature
        if &data[pos..pos + 4] != b"8BIM" {
            break;
        }
        pos += 4;
        
        // Resource ID
        let resource_id = u16::from_be_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        
        // Pascal string (name) - first byte is length
        let name_len = data[pos] as usize;
        pos += 1 + name_len;
        // Pad to even offset
        if !(1 + name_len).is_multiple_of(2) {
            pos += 1;
        }
        
        if pos + 4 > data.len() {
            break;
        }
        
        // Resource size
        let size = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        pos += 4;
        
        if pos + size > data.len() {
            break;
        }
        
        // IPTC-NAA record is resource ID 0x0404
        if resource_id == 0x0404 {
            parse_iptc(&data[pos..pos + size], metadata);
        }
        
        pos += size;
        // Pad to even
        if !size.is_multiple_of(2) {
            pos += 1;
        }
    }
}

/// Parse IPTC-NAA record using exiftool-iptc crate.
fn parse_iptc(data: &[u8], metadata: &mut Metadata) {
    if let Some(iptc_attrs) = IptcParser::parse(data) {
        // Merge IPTC attrs into metadata
        for (key, value) in iptc_attrs.iter() {
            metadata.exif.set(key, value.clone());
        }
    }
}

/// Adobe Extended XMP (XMP spec): header + 32-byte GUID + u32be length + u32be offset + payload.
const EXTENDED_XMP_HEADER: &[u8] = b"http://ns.adobe.com/xmp/extension/\0";

fn collect_extended_xmp(
    data: &[u8],
    dest: &mut HashMap<String, (u32, Vec<(u32, Vec<u8>)>)>,
) {
    let min = EXTENDED_XMP_HEADER.len() + 32 + 8;
    if data.len() < min {
        return;
    }
    let guid = String::from_utf8_lossy(&data[EXTENDED_XMP_HEADER.len()..EXTENDED_XMP_HEADER.len() + 32])
        .into_owned();
    let rest = &data[EXTENDED_XMP_HEADER.len() + 32..];
    let full_len = u32::from_be_bytes([rest[0], rest[1], rest[2], rest[3]]);
    let offset = u32::from_be_bytes([rest[4], rest[5], rest[6], rest[7]]);
    let payload = rest[8..].to_vec();
    dest.entry(guid).or_insert_with(|| (full_len, Vec::new())).1.push((offset, payload));
}

fn assemble_extended_xmp(chunks: &HashMap<String, (u32, Vec<(u32, Vec<u8>)>)>) -> Option<String> {
    let (_, (full_len, pieces)) = chunks.iter().next()?;
    if *full_len == 0 || *full_len > 50_000_000 {
        return None;
    }
    let mut buf = vec![0u8; *full_len as usize];
    for (offset, payload) in pieces {
        let start = *offset as usize;
        let end = start.saturating_add(payload.len());
        if end > buf.len() {
            continue;
        }
        buf[start..end].copy_from_slice(payload);
    }
    String::from_utf8(buf.clone()).ok().or_else(|| {
        let end = buf.iter().rposition(|&b| b != 0).map(|i| i + 1).unwrap_or(0);
        String::from_utf8(buf[..end].to_vec()).ok()
    })
}

/// Multi-Picture Format (CIPA DC-007): APP2 `MPF\0` + TIFF IFD.
fn parse_mpf(tiff_data: &[u8], metadata: &mut Metadata) {
    let byte_order = match ByteOrder::from_marker([tiff_data[0], tiff_data[1]]) {
        Ok(bo) => bo,
        Err(_) => return,
    };
    let reader = IfdReader::new(tiff_data, byte_order);
    let Ok(ifd0) = reader.parse_header() else {
        return;
    };
    let Ok((entries, _)) = reader.read_ifd(ifd0 as u64) else {
        return;
    };
    for entry in entries {
        match entry.tag {
            0xB000 => {
                if let Some(n) = entry.value.as_u32() {
                    metadata.exif.set("MPF:NumberOfImages", AttrValue::UInt(n));
                }
            }
            0xB001 => {
                metadata.exif.set("MPF:MPEntry", entry_to_mpf_attr(&entry.value));
            }
            0xB002 => {
                if let Some(n) = entry.value.as_u32() {
                    metadata.exif.set("MPF:ImageUIDList", AttrValue::UInt(n));
                }
            }
            0xB003 => {
                if let Some(n) = entry.value.as_u32() {
                    metadata.exif.set("MPF:TotalFrames", AttrValue::UInt(n));
                }
            }
            _ => {}
        }
    }
}

fn entry_to_mpf_attr(value: &exiftool_core::RawValue) -> AttrValue {
    match value {
        exiftool_core::RawValue::UInt32(v) => {
            AttrValue::Str(v.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(","))
        }
        exiftool_core::RawValue::Undefined(b) => AttrValue::UInt(b.len() as u32),
        _ => AttrValue::Str(format!("{:?}", value)),
    }
}

/// Decode UTF-16 XMP data (BE or LE based on BOM or heuristics).
fn decode_utf16(data: &[u8]) -> Option<String> {
    if data.len() < 2 {
        return None;
    }
    
    let (is_be, start) = if data.starts_with(&[0xFE, 0xFF]) {
        (true, 2)
    } else if data.starts_with(&[0xFF, 0xFE]) {
        (false, 2)
    } else {
        let is_le = data.len() >= 2 && data[1] == 0x00 && data[0] != 0x00;
        (!is_le, 0)
    };
    
    let bytes = &data[start..];
    if !bytes.len().is_multiple_of(2) {
        return None;
    }
    
    let u16_iter = bytes.chunks_exact(2).map(|chunk| {
        if is_be {
            u16::from_be_bytes([chunk[0], chunk[1]])
        } else {
            u16::from_le_bytes([chunk[0], chunk[1]])
        }
    });
    
    String::from_utf16(&u16_iter.collect::<Vec<_>>()).ok()
}

fn skip_sos_and_scan(reader: &mut dyn ReadSeek) -> Result<()> {
    let mut len_bytes = [0u8; 2];
    reader.read_exact(&mut len_bytes)?;
    let seg_len = u16::from_be_bytes(len_bytes) as usize;
    if seg_len < 2 {
        return Err(Error::InvalidStructure("invalid SOS length".into()));
    }
    let mut skip = vec![0u8; seg_len - 2];
    reader.read_exact(&mut skip)?;
    loop {
        let mut b = [0u8; 1];
        if reader.read_exact(&mut b).is_err() {
            return Ok(());
        }
        if b[0] != 0xFF {
            continue;
        }
        loop {
            if reader.read_exact(&mut b).is_err() {
                return Ok(());
            }
            if b[0] != 0xFF {
                break;
            }
        }
        match b[0] {
            0x00 | 0xD0..=0xD7 => {}
            0xD9 => return Ok(()),
            _ => {}
        }
    }
}

fn apply_jpeg_trailer(reader: &mut dyn ReadSeek, metadata: &mut Metadata) {
    let mut rest = Vec::new();
    if reader.read_to_end(&mut rest).is_err() || rest.is_empty() {
        return;
    }
    metadata
        .exif
        .set("TrailerLength", AttrValue::UInt(rest.len() as u32));
    if let Some(kind) = classify_jpeg_trailer(&rest) {
        metadata.exif.set("TrailerType", AttrValue::Str(kind.into()));
    }
}

fn classify_jpeg_trailer(data: &[u8]) -> Option<&'static str> {
    if data.len() >= 12 {
        let sig = &data[data.len() - 12..data.len() - 8];
        if sig == b"AXS!" || sig == b"AXS*" {
            return Some("AFCP");
        }
    }
    if data.ends_with(&[0xa1, 0xb2, 0xc3, 0xd4]) {
        return Some("FotoStation");
    }
    if data.ends_with(b"cbipcbbl") {
        return Some("PhotoMechanic");
    }
    if data.ends_with(b"QDIOBS") {
        return Some("Samsung");
    }
    if data.windows(20).any(|w| w == b"CANON OPTIONAL DATA\0") {
        return Some("CanonVRD");
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_jpeg() {
        let parser = JpegParser;
        assert!(parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
        assert!(!parser.can_parse(&[0x89, 0x50, 0x4E, 0x47])); // PNG
    }

    #[test]
    fn classify_afcp_trailer() {
        let mut t = vec![0u8; 4];
        t.extend_from_slice(b"AXS!");
        t.extend_from_slice(&[0u8; 8]);
        assert_eq!(classify_jpeg_trailer(&t), Some("AFCP"));
    }

    #[test]
    fn parse_jpeg_with_afcp_trailer() {
        let mut jpeg = vec![0xFF, 0xD8, 0xFF, 0xDA, 0x00, 0x08, 0x01, 0x00, 0x00, 0x3F, 0x00, 0x00];
        jpeg.extend_from_slice(&[0xFF, 0xD9]);
        jpeg.extend_from_slice(b"AXS!");
        jpeg.extend_from_slice(&[0u8; 8]);
        let mut cur = std::io::Cursor::new(jpeg);
        let meta = JpegParser.parse(&mut cur).unwrap();
        assert_eq!(meta.exif.get_str("TrailerType"), Some("AFCP"));
        assert_eq!(meta.exif.get("TrailerLength").map(|v| v.to_string()), Some("12".into()));
    }
}

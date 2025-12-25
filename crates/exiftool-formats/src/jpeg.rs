//! JPEG format parser.
//!
//! JPEG files consist of segments, each starting with 0xFF marker:
//! - SOI (0xFFD8) - Start of Image
//! - APP0 (0xFFE0) - JFIF
//! - APP1 (0xFFE1) - EXIF or XMP
//! - APP2 (0xFFE2) - ICC Profile
//! - DQT, DHT, SOF, SOS... - image data
//! - EOI (0xFFD9) - End of Image

use crate::tag_lookup::{lookup_exif_subifd, lookup_gps, lookup_ifd0};
use crate::{makernotes, Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use exiftool_core::{ByteOrder, IfdReader, RawValue};
use exiftool_xmp::XmpParser;
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

            // EOI or SOS - stop parsing
            if marker_id == 0xD9 || marker_id == 0xDA {
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
                        parse_exif(tiff_data, &mut metadata)?;
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

/// Parse ICC profile chunks.
fn parse_icc_profile(chunks: &mut [(u8, u8, Vec<u8>)], metadata: &mut Metadata) {
    // Sort by chunk number
    chunks.sort_by_key(|(num, _, _)| *num);
    
    // Concatenate chunks
    let mut profile_data = Vec::new();
    for (_, _, data) in chunks {
        profile_data.extend_from_slice(data);
    }
    
    if profile_data.len() < 128 {
        metadata.exif.set("ICCProfile", AttrValue::Str(format!("{} bytes", profile_data.len())));
        return;
    }
    
    // Parse ICC header (first 128 bytes)
    // Bytes 4-7: preferred CMM type
    // Bytes 8-11: profile version
    // Bytes 12-15: profile/device class
    // Bytes 16-19: color space
    // Bytes 20-23: PCS (Profile Connection Space)
    // Bytes 48-51: primary platform
    // Bytes 80-83: profile creator
    
    let profile_size = u32::from_be_bytes([profile_data[0], profile_data[1], profile_data[2], profile_data[3]]);
    metadata.exif.set("ICCProfileSize", AttrValue::UInt(profile_size));
    
    // Profile class
    if let Ok(class) = std::str::from_utf8(&profile_data[12..16]) {
        let class_name = match class.trim() {
            "scnr" => "Input Device",
            "mntr" => "Display Device",
            "prtr" => "Output Device",
            "link" => "DeviceLink",
            "spac" => "ColorSpace Conversion",
            "abst" => "Abstract",
            "nmcl" => "Named Color",
            _ => class,
        };
        metadata.exif.set("ICCDeviceClass", AttrValue::Str(class_name.to_string()));
    }
    
    // Color space
    if let Ok(space) = std::str::from_utf8(&profile_data[16..20]) {
        let space_name = match space.trim() {
            "RGB" => "RGB",
            "GRAY" => "Grayscale",
            "CMYK" => "CMYK",
            "Lab" => "Lab",
            "XYZ" => "XYZ",
            _ => space,
        };
        metadata.exif.set("ICCColorSpace", AttrValue::Str(space_name.to_string()));
    }
    
    // Profile description (tag 'desc' at offset in tag table)
    // Tag table starts at offset 128, each entry is 12 bytes
    let tag_count = u32::from_be_bytes([profile_data[128], profile_data[129], profile_data[130], profile_data[131]]) as usize;
    
    for i in 0..tag_count.min(50) {
        let tag_offset = 132 + i * 12;
        if tag_offset + 12 > profile_data.len() {
            break;
        }
        
        let sig = &profile_data[tag_offset..tag_offset + 4];
        if sig == b"desc" {
            let data_offset = u32::from_be_bytes([
                profile_data[tag_offset + 4],
                profile_data[tag_offset + 5],
                profile_data[tag_offset + 6],
                profile_data[tag_offset + 7],
            ]) as usize;
            let data_size = u32::from_be_bytes([
                profile_data[tag_offset + 8],
                profile_data[tag_offset + 9],
                profile_data[tag_offset + 10],
                profile_data[tag_offset + 11],
            ]) as usize;
            
            if data_offset + data_size <= profile_data.len() && data_size > 12 {
                // 'desc' tag: 4 bytes type + 4 bytes reserved + 4 bytes length + ASCII string
                let desc_data = &profile_data[data_offset..data_offset + data_size];
                if desc_data.starts_with(b"desc") && desc_data.len() > 12 {
                    let str_len = u32::from_be_bytes([desc_data[8], desc_data[9], desc_data[10], desc_data[11]]) as usize;
                    if str_len > 0 && 12 + str_len <= desc_data.len() {
                        if let Ok(desc) = std::str::from_utf8(&desc_data[12..12 + str_len - 1]) {
                            metadata.exif.set("ICCProfileName", AttrValue::Str(desc.to_string()));
                        }
                    }
                }
            }
            break;
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

/// Parse IPTC-NAA record.
fn parse_iptc(data: &[u8], metadata: &mut Metadata) {
    let mut pos = 0;
    
    while pos + 5 <= data.len() {
        // IPTC tag marker
        if data[pos] != 0x1C {
            pos += 1;
            continue;
        }
        
        let record = data[pos + 1];
        let dataset = data[pos + 2];
        let size = u16::from_be_bytes([data[pos + 3], data[pos + 4]]) as usize;
        pos += 5;
        
        if pos + size > data.len() {
            break;
        }
        
        let value_data = &data[pos..pos + size];
        
        // Only process application record (2:xx)
        if record == 2 {
            if let Ok(value) = String::from_utf8(value_data.to_vec()) {
                let value = value.trim();
                if !value.is_empty() {
                    let tag_name = iptc_tag_name(dataset);
                    metadata.exif.set(format!("IPTC:{}", tag_name), AttrValue::Str(value.to_string()));
                }
            }
        }
        
        pos += size;
    }
}

/// Get IPTC tag name.
fn iptc_tag_name(dataset: u8) -> &'static str {
    match dataset {
        0 => "RecordVersion",
        3 => "ObjectType",
        4 => "ObjectAttribute",
        5 => "ObjectName",
        7 => "EditStatus",
        10 => "Urgency",
        12 => "SubjectReference",
        15 => "Category",
        20 => "SupplementalCategories",
        22 => "FixtureIdentifier",
        25 => "Keywords",
        26 => "ContentLocationCode",
        27 => "ContentLocationName",
        30 => "ReleaseDate",
        35 => "ReleaseTime",
        37 => "ExpirationDate",
        38 => "ExpirationTime",
        40 => "SpecialInstructions",
        45 => "ReferenceService",
        47 => "ReferenceDate",
        50 => "ReferenceNumber",
        55 => "DateCreated",
        60 => "TimeCreated",
        62 => "DigitalCreationDate",
        63 => "DigitalCreationTime",
        65 => "OriginatingProgram",
        70 => "ProgramVersion",
        75 => "ObjectCycle",
        80 => "Byline",
        85 => "BylineTitle",
        90 => "City",
        92 => "Sublocation",
        95 => "Province-State",
        100 => "Country-PrimaryLocationCode",
        101 => "Country-PrimaryLocationName",
        103 => "OriginalTransmissionReference",
        105 => "Headline",
        110 => "Credit",
        115 => "Source",
        116 => "CopyrightNotice",
        118 => "Contact",
        120 => "Caption-Abstract",
        121 => "LocalCaption",
        122 => "Writer-Editor",
        130 => "ImageType",
        131 => "ImageOrientation",
        135 => "LanguageIdentifier",
        _ => "Unknown",
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

/// Parse EXIF TIFF data into metadata.
fn parse_exif(tiff_data: &[u8], metadata: &mut Metadata) -> Result<()> {
    if tiff_data.len() < 8 {
        return Ok(());
    }

    let byte_order =
        ByteOrder::from_marker([tiff_data[0], tiff_data[1]]).map_err(Error::Core)?;

    let reader = IfdReader::new(tiff_data, byte_order, 0);
    let ifd0_offset = reader.parse_header().map_err(Error::Core)?;

    let (entries, _next_ifd) = reader.read_ifd(ifd0_offset).map_err(Error::Core)?;

    // First pass: extract Make to detect vendor
    let mut vendor = makernotes::Vendor::Unknown;
    for entry in &entries {
        if entry.tag == 0x010F {
            if let RawValue::String(make) = &entry.value {
                vendor = makernotes::Vendor::from_make(make);
            }
            break;
        }
    }

    // Convert IFD entries to attributes
    for entry in &entries {
        if let Some(name) = lookup_ifd0(entry.tag) {
            let value = entry_to_attr(entry);
            metadata.exif.set(name, value);
        }

        // Handle sub-IFDs
        match entry.tag {
            0x8769 => {
                // ExifIFD pointer
                if let Some(offset) = entry.value.as_u32() {
                    if let Ok((exif_entries, _)) = reader.read_ifd(offset) {
                        for e in &exif_entries {
                            // Parse MakerNotes with vendor-specific decoder
                            if e.tag == 0x927C {
                                if let RawValue::Undefined(bytes) = &e.value {
                                    if let Some(mn_data) = makernotes::parse(bytes, vendor, byte_order) {
                                        for (key, val) in mn_data.iter() {
                                            metadata.exif.set(key.clone(), val.clone());
                                        }
                                    }
                                }
                            } else if let Some(name) = lookup_exif_subifd(e.tag) {
                                metadata.exif.set(name, entry_to_attr(e));
                            }
                        }
                    }
                }
            }
            0x8825 => {
                // GPS IFD pointer
                if let Some(offset) = entry.value.as_u32() {
                    if let Ok((gps_entries, _)) = reader.read_ifd(offset) {
                        for e in &gps_entries {
                            if let Some(name) = lookup_gps(e.tag) {
                                metadata.exif.set(name, entry_to_attr(e));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

// Use shared entry_to_attr from crate::utils
use crate::utils::entry_to_attr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_jpeg() {
        let parser = JpegParser;
        assert!(parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
        assert!(!parser.can_parse(&[0x89, 0x50, 0x4E, 0x47])); // PNG
    }
}

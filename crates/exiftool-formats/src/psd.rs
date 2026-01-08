//! Adobe Photoshop PSD/PSB format parser.
//!
//! PSD (Photoshop Document) and PSB (Photoshop Big) are Adobe's native formats.
//! Metadata is stored in:
//! - File header (dimensions, color mode, depth)
//! - Image Resources section (8BIM resources) containing IPTC, EXIF, XMP, ICC
//!
//! File structure:
//! 1. Header (26 bytes for PSD, 30 for PSB)
//! 2. Color Mode Data
//! 3. Image Resources (contains metadata)
//! 4. Layer and Mask Information
//! 5. Image Data
//!
//! Reference: Adobe Photoshop File Formats Specification

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::ByteOrder;
use std::io::{Read, Seek, SeekFrom};

/// PSD/PSB format parser.
pub struct PsdParser;

impl PsdParser {
    /// PSD magic: "8BPS"
    const MAGIC: &'static [u8] = b"8BPS";
    
    /// Image resource signature: "8BIM"
    const RESOURCE_SIG: &'static [u8] = b"8BIM";
    
    /// Known resource IDs
    const RES_IPTC: u16 = 0x0404;        // IPTC-NAA record
    const RES_EXIF1: u16 = 0x0422;       // EXIF data 1
    const RES_EXIF3: u16 = 0x0423;       // EXIF data 3 (without TIFF header)
    const RES_XMP: u16 = 0x0424;         // XMP metadata
    const RES_ICC: u16 = 0x040F;         // ICC profile
    const RES_THUMBNAIL: u16 = 0x0409;   // Thumbnail (Photoshop 4+)
    const RES_THUMBNAIL_OLD: u16 = 0x0408; // Thumbnail (Photoshop 2.5)
    const RES_RESOLUTION: u16 = 0x03ED;  // Resolution info
    const RES_COPYRIGHT: u16 = 0x040A;   // Copyright flag
    const RES_URL: u16 = 0x040B;         // URL
    
    /// Parse PSD header.
    fn parse_header<R: Read + Seek + ?Sized>(reader: &mut R) -> Result<(bool, u16, u32, u32, u16, u16)> {
        let mut header = [0u8; 30];
        reader.read_exact(&mut header[..26])?;
        
        // Check magic
        if &header[0..4] != Self::MAGIC {
            return Err(crate::Error::InvalidStructure("Not a PSD file".into()));
        }
        
        // Version: 1 = PSD, 2 = PSB
        let version = u16::from_be_bytes([header[4], header[5]]);
        let is_psb = version == 2;
        
        if version != 1 && version != 2 {
            return Err(crate::Error::InvalidStructure(format!("Unknown PSD version: {}", version)));
        }
        
        // Reserved (6 bytes, should be zero)
        // Channels
        let channels = u16::from_be_bytes([header[12], header[13]]);
        
        // Height and width (4 bytes each)
        let height = u32::from_be_bytes([header[14], header[15], header[16], header[17]]);
        let width = u32::from_be_bytes([header[18], header[19], header[20], header[21]]);
        
        // Depth (bits per channel)
        let depth = u16::from_be_bytes([header[22], header[23]]);
        
        // Color mode
        let color_mode = u16::from_be_bytes([header[24], header[25]]);
        
        Ok((is_psb, channels, height, width, depth, color_mode))
    }
    
    /// Get color mode name.
    fn color_mode_name(mode: u16) -> &'static str {
        match mode {
            0 => "Bitmap",
            1 => "Grayscale",
            2 => "Indexed",
            3 => "RGB",
            4 => "CMYK",
            7 => "Multichannel",
            8 => "Duotone",
            9 => "Lab",
            _ => "Unknown",
        }
    }
    
    /// Skip color mode data section.
    fn skip_color_mode<R: Read + Seek + ?Sized>(reader: &mut R) -> Result<()> {
        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf)?;
        let len = u32::from_be_bytes(len_buf);
        if len > 0 {
            reader.seek(SeekFrom::Current(len as i64))?;
        }
        Ok(())
    }
    
    /// Parse image resources section.
    fn parse_resources<R: Read + Seek + ?Sized>(
        reader: &mut R,
        attrs: &mut Attrs,
    ) -> Result<(Option<String>, Option<Vec<u8>>, Option<Vec<u8>>)> {
        let mut xmp = None;
        let mut icc = None;
        let mut thumbnail = None;
        
        // Read section length
        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf)?;
        let section_len = u32::from_be_bytes(len_buf) as u64;
        
        if section_len == 0 {
            return Ok((xmp, icc, thumbnail));
        }
        
        let section_end = reader.stream_position()? + section_len;
        
        // Parse individual resources
        while reader.stream_position()? < section_end {
            // Read resource signature
            let mut sig = [0u8; 4];
            if reader.read_exact(&mut sig).is_err() {
                break;
            }
            
            if &sig != Self::RESOURCE_SIG {
                // Try to recover by seeking back
                reader.seek(SeekFrom::Current(-3))?;
                continue;
            }
            
            // Resource ID (2 bytes)
            let mut id_buf = [0u8; 2];
            reader.read_exact(&mut id_buf)?;
            let resource_id = u16::from_be_bytes(id_buf);
            
            // Pascal string (name) - padded to even length
            let mut name_len = [0u8; 1];
            reader.read_exact(&mut name_len)?;
            let name_len = name_len[0] as u64;
            let padded_len = if (name_len + 1) % 2 == 0 { name_len } else { name_len + 1 };
            reader.seek(SeekFrom::Current(padded_len as i64))?;
            
            // Resource data length (4 bytes)
            let mut data_len_buf = [0u8; 4];
            reader.read_exact(&mut data_len_buf)?;
            let data_len = u32::from_be_bytes(data_len_buf);
            
            // Padded to even length
            let padded_data_len = if data_len % 2 == 0 { data_len } else { data_len + 1 };
            
            match resource_id {
                Self::RES_XMP => {
                    let mut data = vec![0u8; data_len as usize];
                    reader.read_exact(&mut data)?;
                    if let Ok(s) = String::from_utf8(data) {
                        xmp = Some(s);
                    }
                    if data_len % 2 != 0 {
                        reader.seek(SeekFrom::Current(1))?;
                    }
                }
                Self::RES_ICC => {
                    let mut data = vec![0u8; data_len as usize];
                    reader.read_exact(&mut data)?;
                    icc = Some(data);
                    if data_len % 2 != 0 {
                        reader.seek(SeekFrom::Current(1))?;
                    }
                }
                Self::RES_EXIF1 | Self::RES_EXIF3 => {
                    let mut data = vec![0u8; data_len as usize];
                    reader.read_exact(&mut data)?;
                    // Parse EXIF data
                    Self::parse_exif_resource(&data, attrs, resource_id == Self::RES_EXIF3);
                    if data_len % 2 != 0 {
                        reader.seek(SeekFrom::Current(1))?;
                    }
                }
                Self::RES_IPTC => {
                    let mut data = vec![0u8; data_len as usize];
                    reader.read_exact(&mut data)?;
                    Self::parse_iptc_resource(&data, attrs);
                    if data_len % 2 != 0 {
                        reader.seek(SeekFrom::Current(1))?;
                    }
                }
                Self::RES_THUMBNAIL | Self::RES_THUMBNAIL_OLD => {
                    let mut data = vec![0u8; data_len as usize];
                    reader.read_exact(&mut data)?;
                    // Thumbnail format: 4 bytes format, 4 bytes width, 4 bytes height, etc.
                    // JPEG data starts at offset 28
                    if data.len() > 28 {
                        thumbnail = Some(data[28..].to_vec());
                    }
                    if data_len % 2 != 0 {
                        reader.seek(SeekFrom::Current(1))?;
                    }
                }
                Self::RES_RESOLUTION => {
                    if data_len >= 16 {
                        let mut data = [0u8; 16];
                        reader.read_exact(&mut data)?;
                        // Resolution is stored as fixed-point 16.16
                        let h_res = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                        let v_res = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);
                        attrs.set("XResolution", AttrValue::Double((h_res >> 16) as f64));
                        attrs.set("YResolution", AttrValue::Double((v_res >> 16) as f64));
                        if data_len > 16 {
                            reader.seek(SeekFrom::Current((padded_data_len - 16) as i64))?;
                        }
                    } else {
                        reader.seek(SeekFrom::Current(padded_data_len as i64))?;
                    }
                }
                Self::RES_COPYRIGHT => {
                    let mut data = vec![0u8; data_len as usize];
                    reader.read_exact(&mut data)?;
                    if !data.is_empty() && data[0] != 0 {
                        attrs.set("CopyrightFlag", AttrValue::Bool(true));
                    }
                    if data_len % 2 != 0 {
                        reader.seek(SeekFrom::Current(1))?;
                    }
                }
                Self::RES_URL => {
                    let mut data = vec![0u8; data_len as usize];
                    reader.read_exact(&mut data)?;
                    if let Ok(s) = String::from_utf8(data) {
                        attrs.set("URL", AttrValue::Str(s));
                    }
                    if data_len % 2 != 0 {
                        reader.seek(SeekFrom::Current(1))?;
                    }
                }
                _ => {
                    // Skip unknown resources
                    reader.seek(SeekFrom::Current(padded_data_len as i64))?;
                }
            }
        }
        
        Ok((xmp, icc, thumbnail))
    }
    
    /// Parse EXIF resource data.
    fn parse_exif_resource(data: &[u8], attrs: &mut Attrs, _without_header: bool) {
        if data.len() < 8 {
            return;
        }
        
        // Try to detect byte order from TIFF header
        let byte_order = if data.len() >= 2 {
            match &data[0..2] {
                b"II" => ByteOrder::LittleEndian,
                b"MM" => ByteOrder::BigEndian,
                _ => ByteOrder::LittleEndian,
            }
        } else {
            ByteOrder::LittleEndian
        };
        
        // Parse using IFD reader
        let ifd_reader = exiftool_core::IfdReader::new(data, byte_order);
        if let Ok((entries, _)) = ifd_reader.read_ifd(8) {
            for entry in entries {
                // Lookup tag name in IFD0 and EXIF tables
                let name = exiftool_tags::IFD0_TAGS.get(&entry.tag)
                    .or_else(|| exiftool_tags::EXIF_TAGS.get(&entry.tag))
                    .or_else(|| exiftool_tags::GPS_TAGS.get(&entry.tag))
                    .map(|def| def.name);
                
                if let Some(tag_name) = name {
                    if let Some(value) = crate::utils::raw_value_to_attr(&entry.value) {
                        attrs.set(tag_name, value);
                    }
                }
            }
        }
    }
    
    /// Parse IPTC resource data.
    fn parse_iptc_resource(data: &[u8], attrs: &mut Attrs) {
        // IPTC-NAA format: series of records
        // Each record: 0x1C, record number, dataset number, size (2 bytes), data
        let mut pos = 0;
        while pos + 5 <= data.len() {
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
            pos += size;
            
            // Record 2 is application record (most common)
            if record == 2 {
                let tag_name = match dataset {
                    5 => "ObjectName",
                    25 => "Keywords",
                    55 => "DateCreated",
                    60 => "TimeCreated",
                    80 => "ByLine",
                    85 => "ByLineTitle",
                    90 => "City",
                    95 => "Province-State",
                    100 => "Country-PrimaryLocationCode",
                    101 => "Country-PrimaryLocationName",
                    105 => "Headline",
                    110 => "Credit",
                    115 => "Source",
                    116 => "CopyrightNotice",
                    120 => "Caption-Abstract",
                    122 => "Writer-Editor",
                    _ => continue,
                };
                
                if let Ok(s) = String::from_utf8(value_data.to_vec()) {
                    // Keywords can be repeated
                    if dataset == 25 {
                        if let Some(existing) = attrs.get(tag_name) {
                            if let Some(existing_str) = existing.as_str() {
                                attrs.set(tag_name, AttrValue::Str(format!("{}, {}", existing_str, s)));
                                continue;
                            }
                        }
                    }
                    attrs.set(tag_name, AttrValue::Str(s));
                }
            }
        }
    }
}

impl FormatParser for PsdParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 4 && &header[0..4] == Self::MAGIC
    }
    
    fn format_name(&self) -> &'static str {
        "PSD"
    }
    
    fn extensions(&self) -> &'static [&'static str] {
        &["psd", "psb"]
    }
    
    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut attrs = Attrs::new();
        
        // Parse header
        let (is_psb, channels, height, width, depth, color_mode) = Self::parse_header(reader)?;
        
        let format = if is_psb { "PSB" } else { "PSD" };
        
        attrs.set("ImageWidth", AttrValue::UInt(width));
        attrs.set("ImageHeight", AttrValue::UInt(height));
        attrs.set("BitDepth", AttrValue::UInt(depth as u32));
        attrs.set("ColorMode", AttrValue::Str(Self::color_mode_name(color_mode).to_string()));
        attrs.set("NumChannels", AttrValue::UInt(channels as u32));
        
        // Skip color mode data
        Self::skip_color_mode(reader)?;
        
        // Parse image resources (contains metadata)
        let (xmp, icc, thumbnail) = Self::parse_resources(reader, &mut attrs)?;
        
        Ok(Metadata {
            format,
            exif: attrs,
            exif_offset: None,
            xmp,
            thumbnail,
            preview: None,
            icc,
            pages: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_can_parse() {
        let parser = PsdParser;
        assert!(parser.can_parse(b"8BPS\x00\x01"));
        assert!(parser.can_parse(b"8BPS\x00\x02")); // PSB
        assert!(!parser.can_parse(b"PNG\r\n"));
        assert!(!parser.can_parse(b"\xFF\xD8\xFF"));
    }
    
    #[test]
    fn test_color_modes() {
        assert_eq!(PsdParser::color_mode_name(0), "Bitmap");
        assert_eq!(PsdParser::color_mode_name(3), "RGB");
        assert_eq!(PsdParser::color_mode_name(4), "CMYK");
        assert_eq!(PsdParser::color_mode_name(9), "Lab");
    }
}

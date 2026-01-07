//! BRAW (Blackmagic RAW) format parser.
//!
//! BRAW is Blackmagic Design's RAW codec used in their cameras.
//! Based on MP4/MOV container with custom codec.
//!
//! # Structure
//!
//! - ISO Base Media File Format container
//! - 'braw' or 'bmd ' brand in ftyp
//! - Custom metadata in moov/udta

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// BRAW format parser.
pub struct BrawParser;

impl FormatParser for BrawParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 12 {
            return false;
        }
        // Check for ftyp box with BRAW brands
        if &header[4..8] == b"ftyp" {
            let brand = &header[8..12];
            return brand == b"braw" || brand == b"bmd ";
        }
        false
    }

    fn format_name(&self) -> &'static str {
        "BRAW"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["braw"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("BRAW");
        meta.set_file_type("BRAW", "video/x-blackmagic-raw");
        meta.exif.set("Video:Codec", AttrValue::Str("Blackmagic RAW".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // File size
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        reader.seek(SeekFrom::Start(0))?;

        // Parse atoms (MP4/MOV style)
        parse_braw_atoms(reader, file_size, &mut meta, 0)?;

        Ok(meta)
    }
}

/// Parse BRAW/MP4 atoms.
fn parse_braw_atoms(reader: &mut dyn ReadSeek, end_pos: u64, meta: &mut Metadata, depth: u32) -> Result<()> {
    if depth > 10 {
        return Ok(());
    }

    while reader.stream_position()? + 8 <= end_pos {
        let atom_start = reader.stream_position()?;
        
        let mut header = [0u8; 8];
        if reader.read_exact(&mut header).is_err() {
            break;
        }

        let mut size = u32::from_be_bytes([header[0], header[1], header[2], header[3]]) as u64;
        let atom_type = [header[4], header[5], header[6], header[7]];

        // Handle extended size
        if size == 1 {
            let mut ext_size = [0u8; 8];
            reader.read_exact(&mut ext_size)?;
            size = u64::from_be_bytes(ext_size);
        } else if size == 0 {
            size = end_pos - atom_start;
        }

        if size < 8 || atom_start + size > end_pos {
            break;
        }

        let data_start = reader.stream_position()?;
        let data_size = size - (data_start - atom_start);

        match &atom_type {
            b"ftyp" => {
                if data_size >= 4 {
                    let mut brand = [0u8; 4];
                    reader.read_exact(&mut brand)?;
                    meta.exif.set("BRAW:Brand", AttrValue::Str(
                        String::from_utf8_lossy(&brand).to_string()
                    ));
                }
            }
            b"moov" | b"trak" | b"mdia" | b"minf" | b"stbl" | b"udta" => {
                // Container atoms - recurse
                parse_braw_atoms(reader, atom_start + size, meta, depth + 1)?;
            }
            b"mvhd" => {
                // Movie header
                if data_size >= 100 {
                    let mut data = [0u8; 100];
                    reader.read_exact(&mut data)?;
                    
                    let version = data[0];
                    let (timescale, duration) = if version == 1 {
                        let ts = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
                        let dur = u64::from_be_bytes([
                            data[24], data[25], data[26], data[27],
                            data[28], data[29], data[30], data[31],
                        ]);
                        (ts, dur)
                    } else {
                        let ts = u32::from_be_bytes([data[12], data[13], data[14], data[15]]);
                        let dur = u32::from_be_bytes([data[16], data[17], data[18], data[19]]) as u64;
                        (ts, dur)
                    };
                    
                    if timescale > 0 && duration > 0 {
                        let dur_secs = duration as f64 / timescale as f64;
                        meta.exif.set("Video:Duration", AttrValue::Double(dur_secs));
                    }
                }
            }
            b"tkhd" => {
                // Track header
                if data_size >= 84 {
                    let mut data = [0u8; 84];
                    reader.read_exact(&mut data)?;
                    
                    let (width_off, height_off) = (76, 80);
                    
                    // Width/height are 16.16 fixed point
                    let width = u32::from_be_bytes([
                        data[width_off], data[width_off + 1], 
                        data[width_off + 2], data[width_off + 3]
                    ]) >> 16;
                    let height = u32::from_be_bytes([
                        data[height_off], data[height_off + 1],
                        data[height_off + 2], data[height_off + 3]
                    ]) >> 16;
                    
                    if width > 0 && width < 20000 && meta.exif.get_u32("Video:Width").is_none() {
                        meta.exif.set("Video:Width", AttrValue::UInt(width));
                    }
                    if height > 0 && height < 20000 && meta.exif.get_u32("Video:Height").is_none() {
                        meta.exif.set("Video:Height", AttrValue::UInt(height));
                    }
                }
            }
            b"stsd" => {
                // Sample description - find video codec
                if data_size > 16 {
                    let mut data = vec![0u8; data_size.min(256) as usize];
                    reader.read_exact(&mut data)?;
                    
                    // Skip version (1) + flags (3) + entry count (4)
                    if data.len() > 16 {
                        let codec = &data[12..16];
                        let codec_str = String::from_utf8_lossy(codec).trim().to_string();
                        if !codec_str.is_empty() && codec_str.chars().all(|c| c.is_ascii_alphanumeric()) {
                            meta.exif.set("BRAW:CodecID", AttrValue::Str(codec_str));
                        }
                    }
                }
            }
            b"BMDM" | b"bmdm" => {
                // Blackmagic metadata
                if data_size > 0 && data_size < 10000 {
                    let mut data = vec![0u8; data_size as usize];
                    reader.read_exact(&mut data)?;
                    parse_braw_metadata(&data, meta);
                }
            }
            b"mdat" => {
                // Media data - just note its size
                meta.exif.set("BRAW:MediaDataSize", AttrValue::UInt64(data_size));
            }
            _ => {}
        }

        // Move to next atom
        reader.seek(SeekFrom::Start(atom_start + size))?;
    }

    Ok(())
}

/// Parse Blackmagic-specific metadata.
fn parse_braw_metadata(data: &[u8], meta: &mut Metadata) {
    // BMDM is typically key-value pairs
    // Format varies, try to extract known fields
    
    let mut pos = 0;
    while pos + 8 < data.len() {
        // Try to find recognizable strings
        if let Some(end) = data[pos..].iter().position(|&b| b == 0) {
            if end > 2 && end < 64 {
                let key = String::from_utf8_lossy(&data[pos..pos + end]).to_string();
                pos += end + 1;
                
                // Look for value
                if pos < data.len() {
                    if let Some(val_end) = data[pos..].iter().position(|&b| b == 0) {
                        if val_end > 0 && val_end < 256 {
                            let value = String::from_utf8_lossy(&data[pos..pos + val_end]).to_string();
                            
                            // Map known keys
                            let tag = match key.to_lowercase().as_str() {
                                "camera" | "camera model" => Some("BRAW:CameraModel"),
                                "camera serial" => Some("BRAW:CameraSerial"),
                                "lens" | "lens model" => Some("BRAW:Lens"),
                                "iso" => Some("BRAW:ISO"),
                                "white balance" | "wb" => Some("BRAW:WhiteBalance"),
                                "shutter" => Some("BRAW:Shutter"),
                                "firmware" => Some("BRAW:Firmware"),
                                "project" => Some("BRAW:Project"),
                                "scene" => Some("BRAW:Scene"),
                                "take" => Some("BRAW:Take"),
                                _ => None,
                            };
                            
                            if let Some(tag_name) = tag {
                                meta.exif.set(tag_name, AttrValue::Str(value));
                            }
                            
                            pos += val_end + 1;
                            continue;
                        }
                    }
                }
            }
        }
        pos += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_braw_header() -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // ftyp atom
        data[0..4].copy_from_slice(&20u32.to_be_bytes());
        data[4..8].copy_from_slice(b"ftyp");
        data[8..12].copy_from_slice(b"braw");
        data[12..16].copy_from_slice(&0u32.to_be_bytes());
        data[16..20].copy_from_slice(b"braw");
        
        // moov atom (minimal)
        data[20..24].copy_from_slice(&100u32.to_be_bytes());
        data[24..28].copy_from_slice(b"moov");
        
        data
    }

    #[test]
    fn test_can_parse_braw() {
        let parser = BrawParser;
        let data = make_braw_header();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_can_parse_bmd() {
        let parser = BrawParser;
        let mut data = vec![0u8; 20];
        data[0..4].copy_from_slice(&20u32.to_be_bytes());
        data[4..8].copy_from_slice(b"ftyp");
        data[8..12].copy_from_slice(b"bmd ");
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = BrawParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"RIFF"));
        // Regular MP4
        let mut mp4 = vec![0u8; 20];
        mp4[4..8].copy_from_slice(b"ftyp");
        mp4[8..12].copy_from_slice(b"mp42");
        assert!(!parser.can_parse(&mp4));
    }

    #[test]
    fn test_parse_basic() {
        let parser = BrawParser;
        let data = make_braw_header();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "BRAW");
        assert_eq!(meta.exif.get_str("BRAW:Brand"), Some("braw"));
    }

    #[test]
    fn test_format_info() {
        let parser = BrawParser;
        assert_eq!(parser.format_name(), "BRAW");
        assert!(parser.extensions().contains(&"braw"));
    }
}

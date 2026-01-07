//! R3D (RED Digital Cinema) RAW format parser.
//!
//! R3D is RED's proprietary RAW video format.
//!
//! # Structure
//!
//! - REDCODE RAW container
//! - Atom-based structure similar to QuickTime
//! - RED1/RED2 magic

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// R3D format parser.
pub struct R3dParser;

impl FormatParser for R3dParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 8 {
            return false;
        }
        // R3D files start with size + 'RED1' or 'RED2'
        &header[4..8] == b"RED1" || &header[4..8] == b"RED2"
    }

    fn format_name(&self) -> &'static str {
        "R3D"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["r3d"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("R3D");
        meta.set_file_type("R3D", "video/x-red-r3d");
        meta.exif.set("Video:Codec", AttrValue::Str("REDCODE RAW".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // File size
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        reader.seek(SeekFrom::Start(0))?;

        // Parse atoms (similar to QuickTime)
        parse_r3d_atoms(reader, file_size, &mut meta)?;

        Ok(meta)
    }
}

/// Parse R3D atoms recursively.
fn parse_r3d_atoms(reader: &mut dyn ReadSeek, end_pos: u64, meta: &mut Metadata) -> Result<()> {
    while reader.stream_position()? + 8 <= end_pos {
        let mut header = [0u8; 8];
        if reader.read_exact(&mut header).is_err() {
            break;
        }

        let size = u32::from_be_bytes([header[0], header[1], header[2], header[3]]) as u64;
        let atom_type = &header[4..8];

        if size < 8 || size > end_pos {
            break;
        }

        let atom_start = reader.stream_position()? - 8;
        let data_size = size - 8;

        match atom_type {
            b"RED1" | b"RED2" => {
                let version = if atom_type == b"RED1" { 1 } else { 2 };
                meta.exif.set("R3D:Version", AttrValue::UInt(version));
                // Container atom, recurse
                parse_r3d_atoms(reader, atom_start + size, meta)?;
            }
            b"REOB" => {
                // RED Object - container for metadata
                parse_r3d_atoms(reader, atom_start + size, meta)?;
            }
            b"REOS" => {
                // RED Object Specific
                parse_r3d_atoms(reader, atom_start + size, meta)?;
            }
            b"RDVO" => {
                // Video object info
                if data_size >= 16 {
                    let mut data = [0u8; 16];
                    reader.read_exact(&mut data)?;
                    
                    let width = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                    let height = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
                    
                    if width > 0 && width < 20000 {
                        meta.exif.set("Video:Width", AttrValue::UInt(width));
                    }
                    if height > 0 && height < 20000 {
                        meta.exif.set("Video:Height", AttrValue::UInt(height));
                    }
                }
            }
            b"RDVS" => {
                // Video settings
                if data_size >= 24 {
                    let mut data = [0u8; 24];
                    reader.read_exact(&mut data)?;
                    
                    // Frame rate as rational
                    let fps_num = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                    let fps_den = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
                    
                    if fps_den > 0 && fps_num > 0 {
                        let fps = fps_num as f64 / fps_den as f64;
                        if fps > 0.0 && fps < 1000.0 {
                            meta.exif.set("Video:FrameRate", AttrValue::Double(fps));
                        }
                    }
                }
            }
            b"RDAO" => {
                // Audio object
                if data_size >= 8 {
                    let mut data = [0u8; 8];
                    reader.read_exact(&mut data)?;
                    
                    let sample_rate = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                    let channels = u16::from_be_bytes([data[4], data[5]]);
                    
                    if sample_rate > 0 {
                        meta.exif.set("Audio:SampleRate", AttrValue::UInt(sample_rate));
                    }
                    if channels > 0 {
                        meta.exif.set("Audio:Channels", AttrValue::UInt(channels as u32));
                    }
                }
            }
            b"RDI " => {
                // Recording date/time info
                if data_size >= 20 {
                    let mut data = [0u8; 20];
                    reader.read_exact(&mut data)?;
                    
                    let year = u16::from_be_bytes([data[0], data[1]]);
                    let month = data[2];
                    let day = data[3];
                    let hour = data[4];
                    let min = data[5];
                    let sec = data[6];
                    
                    if year > 2000 && year < 2100 {
                        meta.exif.set("R3D:DateRecorded", AttrValue::Str(
                            format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", 
                                year, month, day, hour, min, sec)
                        ));
                    }
                }
            }
            b"RMDC" | b"RMD1" | b"RMD2" => {
                // Metadata container
                parse_r3d_atoms(reader, atom_start + size, meta)?;
            }
            b"LENS" => {
                // Lens info (string)
                if data_size > 0 && data_size < 256 {
                    let mut data = vec![0u8; data_size as usize];
                    reader.read_exact(&mut data)?;
                    let lens = read_cstring(&data);
                    if !lens.is_empty() {
                        meta.exif.set("R3D:Lens", AttrValue::Str(lens));
                    }
                }
            }
            b"CAMR" | b"CAME" => {
                // Camera model
                if data_size > 0 && data_size < 256 {
                    let mut data = vec![0u8; data_size as usize];
                    reader.read_exact(&mut data)?;
                    let camera = read_cstring(&data);
                    if !camera.is_empty() {
                        meta.exif.set("R3D:CameraModel", AttrValue::Str(camera));
                    }
                }
            }
            b"CAMS" => {
                // Camera serial
                if data_size > 0 && data_size < 64 {
                    let mut data = vec![0u8; data_size as usize];
                    reader.read_exact(&mut data)?;
                    let serial = read_cstring(&data);
                    if !serial.is_empty() {
                        meta.exif.set("R3D:CameraSerial", AttrValue::Str(serial));
                    }
                }
            }
            b"EXPO" => {
                // Exposure info
                if data_size >= 8 {
                    let mut data = [0u8; 8];
                    reader.read_exact(&mut data)?;
                    
                    // ISO
                    let iso = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                    if iso > 0 && iso < 1000000 {
                        meta.exif.set("R3D:ISO", AttrValue::UInt(iso));
                    }
                    
                    // Shutter angle or exposure time
                    let shutter = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
                    if shutter > 0 {
                        meta.exif.set("R3D:ShutterAngle", AttrValue::Double(shutter as f64 / 1000.0));
                    }
                }
            }
            b"WBAL" => {
                // White balance
                if data_size >= 4 {
                    let mut data = [0u8; 4];
                    reader.read_exact(&mut data)?;
                    let kelvin = u32::from_be_bytes(data);
                    if kelvin > 1000 && kelvin < 20000 {
                        meta.exif.set("R3D:WhiteBalance", AttrValue::UInt(kelvin));
                    }
                }
            }
            b"TIMO" => {
                // Timecode origin
                if data_size >= 4 {
                    let mut data = [0u8; 4];
                    reader.read_exact(&mut data)?;
                    let frames = u32::from_be_bytes(data);
                    // Convert to timecode string (assuming 24fps)
                    let fps = 24;
                    let total_sec = frames / fps;
                    let f = frames % fps;
                    let s = total_sec % 60;
                    let m = (total_sec / 60) % 60;
                    let h = total_sec / 3600;
                    meta.exif.set("R3D:Timecode", AttrValue::Str(
                        format!("{:02}:{:02}:{:02}:{:02}", h, m, s, f)
                    ));
                }
            }
            b"CLIP" | b"REEN" | b"TAKE" => {
                // Clip/Reel/Take name
                if data_size > 0 && data_size < 256 {
                    let mut data = vec![0u8; data_size as usize];
                    reader.read_exact(&mut data)?;
                    let name = read_cstring(&data);
                    if !name.is_empty() {
                        let tag = match atom_type {
                            b"CLIP" => "R3D:ClipName",
                            b"REEN" => "R3D:ReelName",
                            b"TAKE" => "R3D:TakeName",
                            _ => "R3D:Name",
                        };
                        meta.exif.set(tag, AttrValue::Str(name));
                    }
                }
            }
            _ => {}
        }

        // Move to next atom
        reader.seek(SeekFrom::Start(atom_start + size))?;
    }

    Ok(())
}

/// Read null-terminated C string.
fn read_cstring(data: &[u8]) -> String {
    let end = data.iter().position(|&b| b == 0).unwrap_or(data.len());
    String::from_utf8_lossy(&data[..end]).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_r3d_header() -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // RED2 atom (size=512)
        data[0..4].copy_from_slice(&512u32.to_be_bytes());
        data[4..8].copy_from_slice(b"RED2");
        
        // RDVO atom at offset 8 (size=24)
        data[8..12].copy_from_slice(&24u32.to_be_bytes());
        data[12..16].copy_from_slice(b"RDVO");
        // Width = 4096
        data[16..20].copy_from_slice(&4096u32.to_be_bytes());
        // Height = 2160
        data[20..24].copy_from_slice(&2160u32.to_be_bytes());
        
        data
    }

    #[test]
    fn test_can_parse_red1() {
        let parser = R3dParser;
        let mut data = vec![0u8; 20];
        data[4..8].copy_from_slice(b"RED1");
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_can_parse_red2() {
        let parser = R3dParser;
        let data = make_r3d_header();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = R3dParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"RIFF"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = R3dParser;
        let data = make_r3d_header();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "R3D");
        assert_eq!(meta.exif.get_u32("R3D:Version"), Some(2));
        assert_eq!(meta.exif.get_u32("Video:Width"), Some(4096));
        assert_eq!(meta.exif.get_u32("Video:Height"), Some(2160));
    }

    #[test]
    fn test_format_info() {
        let parser = R3dParser;
        assert_eq!(parser.format_name(), "R3D");
        assert!(parser.extensions().contains(&"r3d"));
    }
}

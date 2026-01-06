//! DPX (Digital Picture Exchange) format parser.
//!
//! DPX is used for film scanning and digital cinema.
//! Based on SMPTE 268M standard.
//!
//! # Structure
//!
//! - File header (768 bytes)
//! - Image header (640 bytes)
//! - Orientation header (256 bytes)
//! - Film header (256 bytes)
//! - TV header (128 bytes)
//! - User data (optional)
//! - Image data

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::{Read, Seek, SeekFrom};

const DPX_MAGIC_BE: u32 = 0x53445058; // "SDPX"
const DPX_MAGIC_LE: u32 = 0x58504453; // "XPDS"

/// DPX format parser.
pub struct DpxParser;

impl FormatParser for DpxParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 4 {
            return false;
        }
        let magic = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
        magic == DPX_MAGIC_BE || magic == DPX_MAGIC_LE
    }

    fn format_name(&self) -> &'static str {
        "DPX"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["dpx"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("DPX");
        meta.exif.set("File:FileType", AttrValue::Str("DPX".to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("image/x-dpx".to_string()));

        reader.seek(SeekFrom::Start(0))?;

        // Read file header
        let mut header = [0u8; 768];
        reader.read_exact(&mut header)?;

        // Determine byte order
        let magic = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
        let be = magic == DPX_MAGIC_BE;

        let read_u32 = |data: &[u8]| -> u32 {
            if be {
                u32::from_be_bytes([data[0], data[1], data[2], data[3]])
            } else {
                u32::from_le_bytes([data[0], data[1], data[2], data[3]])
            }
        };

        let read_u16 = |data: &[u8]| -> u16 {
            if be {
                u16::from_be_bytes([data[0], data[1]])
            } else {
                u16::from_le_bytes([data[0], data[1]])
            }
        };

        meta.exif.set("DPX:ByteOrder", AttrValue::Str(
            if be { "Big-endian" } else { "Little-endian" }.to_string()
        ));

        // Image offset (4 bytes at offset 4)
        let image_offset = read_u32(&header[4..8]);
        meta.exif.set("DPX:ImageOffset", AttrValue::UInt(image_offset));

        // Version (8 bytes at offset 8)
        let version = read_string(&header[8..16]);
        if !version.is_empty() {
            meta.exif.set("DPX:Version", AttrValue::Str(version));
        }

        // File size (4 bytes at offset 16)
        let file_size = read_u32(&header[16..20]);
        meta.exif.set("File:FileSize", AttrValue::UInt(file_size));

        // Ditto key (4 bytes at offset 20)
        // Generic header size (4 bytes at offset 24)
        // Industry header size (4 bytes at offset 28)
        // User data size (4 bytes at offset 32)

        // File name (100 bytes at offset 36)
        let filename = read_string(&header[36..136]);
        if !filename.is_empty() {
            meta.exif.set("DPX:FileName", AttrValue::Str(filename));
        }

        // Creation time (24 bytes at offset 136)
        let create_time = read_string(&header[136..160]);
        if !create_time.is_empty() {
            meta.exif.set("DPX:CreateTime", AttrValue::Str(create_time));
        }

        // Creator (100 bytes at offset 160)
        let creator = read_string(&header[160..260]);
        if !creator.is_empty() {
            meta.exif.set("DPX:Creator", AttrValue::Str(creator));
        }

        // Project (200 bytes at offset 260)
        let project = read_string(&header[260..460]);
        if !project.is_empty() {
            meta.exif.set("DPX:Project", AttrValue::Str(project));
        }

        // Copyright (200 bytes at offset 460)
        let copyright = read_string(&header[460..660]);
        if !copyright.is_empty() {
            meta.exif.set("DPX:Copyright", AttrValue::Str(copyright));
        }

        // Encryption key (4 bytes at offset 660)
        let encrypt_key = read_u32(&header[660..664]);
        if encrypt_key != 0xFFFFFFFF {
            meta.exif.set("DPX:EncryptionKey", AttrValue::UInt(encrypt_key));
        }

        // Read image header (at offset 768)
        let mut img_header = [0u8; 640];
        reader.read_exact(&mut img_header)?;

        // Orientation (2 bytes at offset 0)
        let orientation = read_u16(&img_header[0..2]);
        let orient_str = match orientation {
            0 => "Left to right, top to bottom",
            1 => "Right to left, top to bottom",
            2 => "Left to right, bottom to top",
            3 => "Right to left, bottom to top",
            4 => "Top to bottom, left to right",
            5 => "Top to bottom, right to left",
            6 => "Bottom to top, left to right",
            7 => "Bottom to top, right to left",
            _ => "Unknown",
        };
        meta.exif.set("DPX:Orientation", AttrValue::Str(orient_str.to_string()));

        // Number of image elements (2 bytes at offset 2)
        let num_elements = read_u16(&img_header[2..4]);
        meta.exif.set("DPX:ImageElements", AttrValue::UInt(num_elements as u32));

        // Pixels per line (4 bytes at offset 4)
        let width = read_u32(&img_header[4..8]);
        meta.exif.set("Image:Width", AttrValue::UInt(width));

        // Lines per image (4 bytes at offset 8)
        let height = read_u32(&img_header[8..12]);
        meta.exif.set("Image:Height", AttrValue::UInt(height));

        // Parse first image element (72 bytes each, starting at offset 12)
        if num_elements > 0 {
            let elem = &img_header[12..84];

            // Data sign (4 bytes)
            let data_sign = read_u32(&elem[0..4]);
            meta.exif.set("DPX:DataSign", AttrValue::Str(
                if data_sign == 0 { "Unsigned" } else { "Signed" }.to_string()
            ));

            // Descriptor (1 byte at offset 20)
            let descriptor = elem[20];
            let desc_str = match descriptor {
                0 => "User defined",
                1 => "Red",
                2 => "Green", 
                3 => "Blue",
                4 => "Alpha",
                6 => "Luma (Y)",
                7 => "Color difference (Cb/Cr)",
                8 => "Depth (Z)",
                9 => "Composite video",
                50 => "RGB",
                51 => "RGBA",
                52 => "ABGR",
                100 => "CbYCrY (4:2:2)",
                101 => "CbYaCrYa (4:2:2:4)",
                102 => "CbYCr (4:4:4)",
                103 => "CbYCrA (4:4:4:4)",
                150 => "User 2-component",
                151 => "User 3-component",
                152 => "User 4-component",
                153 => "User 5-component",
                154 => "User 6-component",
                155 => "User 7-component",
                156 => "User 8-component",
                _ => "Unknown",
            };
            meta.exif.set("DPX:Descriptor", AttrValue::Str(desc_str.to_string()));

            // Transfer characteristic (1 byte at offset 21)
            let transfer = elem[21];
            let transfer_str = match transfer {
                0 => "User defined",
                1 => "Printing density",
                2 => "Linear",
                3 => "Logarithmic",
                4 => "Unspecified video",
                5 => "SMPTE 274M",
                6 => "ITU-R 709-4",
                7 => "ITU-R 601-5 (625)",
                8 => "ITU-R 601-5 (525)",
                9 => "Composite NTSC",
                10 => "Composite PAL",
                11 => "Z (linear depth)",
                12 => "Z (homogeneous)",
                _ => "Unknown",
            };
            meta.exif.set("DPX:TransferCharacteristic", AttrValue::Str(transfer_str.to_string()));

            // Colorimetric (1 byte at offset 22)
            let colorimetric = elem[22];
            let color_str = match colorimetric {
                0 => "User defined",
                1 => "Printing density",
                4 => "Unspecified video",
                5 => "SMPTE 274M",
                6 => "ITU-R 709-4",
                7 => "ITU-R 601-5 (625)",
                8 => "ITU-R 601-5 (525)",
                9 => "Composite NTSC",
                10 => "Composite PAL",
                _ => "Unknown",
            };
            meta.exif.set("DPX:Colorimetric", AttrValue::Str(color_str.to_string()));

            // Bit depth (1 byte at offset 23)
            let bit_depth = elem[23];
            meta.exif.set("Image:BitDepth", AttrValue::UInt(bit_depth as u32));

            // Packing (2 bytes at offset 24)
            let packing = read_u16(&elem[24..26]);
            let pack_str = match packing {
                0 => "Packed",
                1 => "Filled (32-bit word, type A)",
                2 => "Filled (32-bit word, type B)",
                _ => "Unknown",
            };
            meta.exif.set("DPX:Packing", AttrValue::Str(pack_str.to_string()));
        }

        // Read orientation header (at offset 768+640 = 1408)
        let mut orient_header = [0u8; 256];
        reader.read_exact(&mut orient_header)?;

        // X/Y offset (4+4 bytes at offset 0)
        // X/Y center (4+4 bytes at offset 8)
        // X/Y original size (4+4 bytes at offset 16)

        // Source file name (100 bytes at offset 24)
        let source_file = read_string(&orient_header[24..124]);
        if !source_file.is_empty() {
            meta.exif.set("DPX:SourceFileName", AttrValue::Str(source_file));
        }

        // Source time/date (24 bytes at offset 124)
        let source_time = read_string(&orient_header[124..148]);
        if !source_time.is_empty() {
            meta.exif.set("DPX:SourceTime", AttrValue::Str(source_time));
        }

        // Input device (32 bytes at offset 148)
        let input_device = read_string(&orient_header[148..180]);
        if !input_device.is_empty() {
            meta.exif.set("DPX:InputDevice", AttrValue::Str(input_device));
        }

        // Input serial (32 bytes at offset 180)
        let input_serial = read_string(&orient_header[180..212]);
        if !input_serial.is_empty() {
            meta.exif.set("DPX:InputDeviceSerial", AttrValue::Str(input_serial));
        }

        // Read film header (at offset 1664)
        let mut film_header = [0u8; 256];
        reader.read_exact(&mut film_header)?;

        // Film manufacturer ID (2 bytes)
        let film_mfg = read_string(&film_header[0..2]);
        
        // Film type (2 bytes)
        let film_type = read_string(&film_header[2..4]);
        
        // Offset in perfs (2 bytes)
        // Prefix (6 bytes)
        // Count (4 bytes)
        
        // Format (32 bytes at offset 16)
        let format = read_string(&film_header[16..48]);
        if !format.is_empty() {
            meta.exif.set("DPX:FilmFormat", AttrValue::Str(format));
        }

        // Frame position (4 bytes at offset 48)
        let frame_pos = read_u32(&film_header[48..52]);
        if frame_pos != 0xFFFFFFFF {
            meta.exif.set("DPX:FramePosition", AttrValue::UInt(frame_pos));
        }

        // Sequence length (4 bytes at offset 52)
        let seq_len = read_u32(&film_header[52..56]);
        if seq_len != 0xFFFFFFFF {
            meta.exif.set("DPX:SequenceLength", AttrValue::UInt(seq_len));
        }

        // Frame rate (4 bytes float at offset 60)
        let frame_rate_bits = read_u32(&film_header[60..64]);
        let frame_rate = f32::from_bits(frame_rate_bits);
        if frame_rate > 0.0 && frame_rate < 1000.0 {
            meta.exif.set("DPX:FrameRate", AttrValue::Double(frame_rate as f64));
        }

        // Combine film info
        if !film_mfg.is_empty() || !film_type.is_empty() {
            meta.exif.set("DPX:FilmStock", AttrValue::Str(format!("{} {}", film_mfg, film_type).trim().to_string()));
        }

        Ok(meta)
    }
}

/// Read null-terminated string from buffer.
fn read_string(data: &[u8]) -> String {
    let end = data.iter().position(|&b| b == 0).unwrap_or(data.len());
    String::from_utf8_lossy(&data[..end]).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_dpx_header() -> Vec<u8> {
        let mut data = vec![0u8; 2048];
        // Magic "SDPX" (big-endian)
        data[0..4].copy_from_slice(b"SDPX");
        // Image offset
        data[4..8].copy_from_slice(&2048u32.to_be_bytes());
        // Version
        data[8..16].copy_from_slice(b"V2.0\0\0\0\0");
        // File size
        data[16..20].copy_from_slice(&4096u32.to_be_bytes());
        
        // Image header at 768
        // Orientation
        data[768..770].copy_from_slice(&0u16.to_be_bytes());
        // Num elements
        data[770..772].copy_from_slice(&1u16.to_be_bytes());
        // Width
        data[772..776].copy_from_slice(&1920u32.to_be_bytes());
        // Height
        data[776..780].copy_from_slice(&1080u32.to_be_bytes());
        // First element (starts at 768+12=780)
        // Descriptor at offset 20 within element = 780+20=800
        data[800] = 50; // RGB
        // Bit depth at offset 23 within element = 780+23=803
        data[803] = 10;
        
        data
    }

    #[test]
    fn test_can_parse_be() {
        let parser = DpxParser;
        let data = make_dpx_header();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_can_parse_le() {
        let parser = DpxParser;
        let mut data = vec![0u8; 20];
        data[0..4].copy_from_slice(b"XPDS");
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = DpxParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"RIFF"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = DpxParser;
        let data = make_dpx_header();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "DPX");
        assert_eq!(meta.exif.get_u32("Image:Width"), Some(1920));
        assert_eq!(meta.exif.get_u32("Image:Height"), Some(1080));
        assert_eq!(meta.exif.get_u32("Image:BitDepth"), Some(10));
    }

    #[test]
    fn test_format_info() {
        let parser = DpxParser;
        assert_eq!(parser.format_name(), "DPX");
        assert!(parser.extensions().contains(&"dpx"));
    }
}

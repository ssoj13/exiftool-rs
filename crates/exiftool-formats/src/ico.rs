//! ICO/CUR format parser.
//!
//! ICO (Windows Icon) / CUR (Cursor) structure:
//! - Header (6 bytes): reserved, type (1=icon, 2=cursor), image count
//! - Directory entries (16 bytes each): width, height, colors, reserved, planes/hotspot, bpp/hotspot, size, offset
//! - Image data (BMP DIB or PNG)
//!
//! Metadata extracted:
//! - ImageCount, FileType (Icon/Cursor)
//! - Per-image: Width, Height, BitsPerPixel, ColorCount

use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;

/// ICO/CUR format parser.
pub struct IcoParser;

impl FormatParser for IcoParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 4 {
            return false;
        }
        // ICO/CUR magic: 00 00 01 00 (icon) or 00 00 02 00 (cursor)
        header[0] == 0 && header[1] == 0 && (header[2] == 1 || header[2] == 2) && header[3] == 0
    }

    fn format_name(&self) -> &'static str {
        "ICO"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["ico", "cur"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        // Read header (6 bytes)
        let mut header = [0u8; 6];
        reader.read_exact(&mut header)?;

        if header[0] != 0 || header[1] != 0 {
            return Err(Error::InvalidStructure("Invalid ICO header".into()));
        }

        let file_type = u16::from_le_bytes([header[2], header[3]]);
        let image_count = u16::from_le_bytes([header[4], header[5]]);

        let (format_name, type_name) = match file_type {
            1 => ("ICO", "Icon"),
            2 => ("CUR", "Cursor"),
            _ => return Err(Error::InvalidStructure(format!("Unknown ICO type: {}", file_type))),
        };

        let mut metadata = Metadata::new(format_name);
        metadata.exif.set("FileType", AttrValue::Str(type_name.to_string()));
        metadata.exif.set("ImageCount", AttrValue::UInt(image_count as u32));

        // Parse directory entries
        let mut max_width = 0u32;
        let mut max_height = 0u32;
        let mut max_bpp = 0u32;

        for i in 0..image_count {
            let mut entry = [0u8; 16];
            reader.read_exact(&mut entry)?;

            // Width/height: 0 means 256
            let width = if entry[0] == 0 { 256u32 } else { entry[0] as u32 };
            let height = if entry[1] == 0 { 256u32 } else { entry[1] as u32 };
            let color_count = if entry[2] == 0 { 256u32 } else { entry[2] as u32 };
            let _reserved = entry[3];

            // For icons: planes (2 bytes) and bits per pixel (2 bytes)
            // For cursors: hotspot X and Y
            let planes_or_hotspot_x = u16::from_le_bytes([entry[4], entry[5]]);
            let bpp_or_hotspot_y = u16::from_le_bytes([entry[6], entry[7]]);

            let _data_size = u32::from_le_bytes([entry[8], entry[9], entry[10], entry[11]]);
            let _data_offset = u32::from_le_bytes([entry[12], entry[13], entry[14], entry[15]]);

            // Track maximum dimensions
            if width > max_width {
                max_width = width;
            }
            if height > max_height {
                max_height = height;
            }

            // Store per-image info for first few images
            if i < 10 {
                let prefix = format!("Image{}", i);

                metadata.exif.set(format!("{}Width", prefix), AttrValue::UInt(width));
                metadata.exif.set(format!("{}Height", prefix), AttrValue::UInt(height));

                if file_type == 1 {
                    // Icon
                    let bpp = bpp_or_hotspot_y as u32;
                    if bpp > 0 {
                        metadata.exif.set(format!("{}BitsPerPixel", prefix), AttrValue::UInt(bpp));
                        if bpp > max_bpp {
                            max_bpp = bpp;
                        }
                    } else if color_count > 0 {
                        // Estimate BPP from color count
                        let estimated_bpp = (color_count as f64).log2().ceil() as u32;
                        metadata.exif.set(format!("{}BitsPerPixel", prefix), AttrValue::UInt(estimated_bpp));
                    }
                    if color_count > 0 && color_count < 256 {
                        metadata.exif.set(format!("{}ColorCount", prefix), AttrValue::UInt(color_count));
                    }
                } else {
                    // Cursor
                    metadata.exif.set(format!("{}HotspotX", prefix), AttrValue::UInt(planes_or_hotspot_x as u32));
                    metadata.exif.set(format!("{}HotspotY", prefix), AttrValue::UInt(bpp_or_hotspot_y as u32));
                }
            }
        }

        // Set overall dimensions (largest image)
        metadata.exif.set("ImageWidth", AttrValue::UInt(max_width));
        metadata.exif.set("ImageHeight", AttrValue::UInt(max_height));
        if max_bpp > 0 {
            metadata.exif.set("BitsPerPixel", AttrValue::UInt(max_bpp));
        }

        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_ico(icon_type: u16, count: u16, entries: &[(u8, u8, u8, u16)]) -> Vec<u8> {
        let mut data = Vec::new();
        // Header
        data.extend_from_slice(&[0, 0]); // Reserved
        data.extend_from_slice(&icon_type.to_le_bytes()); // Type
        data.extend_from_slice(&count.to_le_bytes()); // Count

        // Directory entries
        for (width, height, colors, bpp) in entries {
            data.push(*width);
            data.push(*height);
            data.push(*colors);
            data.push(0); // Reserved
            data.extend_from_slice(&1u16.to_le_bytes()); // Planes
            data.extend_from_slice(&bpp.to_le_bytes()); // BPP
            data.extend_from_slice(&100u32.to_le_bytes()); // Size (dummy)
            data.extend_from_slice(&(6 + count as u32 * 16).to_le_bytes()); // Offset
        }

        data
    }

    #[test]
    fn detect_ico() {
        let parser = IcoParser;
        assert!(parser.can_parse(&[0, 0, 1, 0])); // Icon
        assert!(parser.can_parse(&[0, 0, 2, 0])); // Cursor
    }

    #[test]
    fn reject_non_ico() {
        let parser = IcoParser;
        assert!(!parser.can_parse(b"BM")); // BMP
        assert!(!parser.can_parse(b"\xFF\xD8")); // JPEG
        assert!(!parser.can_parse(&[0, 0, 3, 0])); // Invalid type
    }

    #[test]
    fn parse_single_icon() {
        let parser = IcoParser;
        let data = make_ico(1, 1, &[(32, 32, 0, 32)]);
        let mut cursor = Cursor::new(data);

        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "ICO");
        assert_eq!(meta.exif.get_str("FileType"), Some("Icon"));
        assert_eq!(meta.exif.get_u32("ImageCount"), Some(1));
        assert_eq!(meta.exif.get_u32("ImageWidth"), Some(32));
        assert_eq!(meta.exif.get_u32("ImageHeight"), Some(32));
        assert_eq!(meta.exif.get_u32("BitsPerPixel"), Some(32));
    }

    #[test]
    fn parse_multi_icon() {
        let parser = IcoParser;
        let data = make_ico(1, 3, &[
            (16, 16, 0, 32),
            (32, 32, 0, 32),
            (0, 0, 0, 32), // 256x256
        ]);
        let mut cursor = Cursor::new(data);

        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_u32("ImageCount"), Some(3));
        assert_eq!(meta.exif.get_u32("ImageWidth"), Some(256)); // Largest
        assert_eq!(meta.exif.get_u32("ImageHeight"), Some(256));
    }

    #[test]
    fn parse_cursor() {
        let parser = IcoParser;
        // Cursor with hotspot at (5, 10)
        let mut data = Vec::new();
        data.extend_from_slice(&[0, 0, 2, 0, 1, 0]); // Header: cursor, 1 image
        data.push(32); // Width
        data.push(32); // Height
        data.push(0); // Colors
        data.push(0); // Reserved
        data.extend_from_slice(&5u16.to_le_bytes()); // Hotspot X
        data.extend_from_slice(&10u16.to_le_bytes()); // Hotspot Y
        data.extend_from_slice(&100u32.to_le_bytes()); // Size
        data.extend_from_slice(&22u32.to_le_bytes()); // Offset

        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "CUR");
        assert_eq!(meta.exif.get_str("FileType"), Some("Cursor"));
        assert_eq!(meta.exif.get_u32("Image0HotspotX"), Some(5));
        assert_eq!(meta.exif.get_u32("Image0HotspotY"), Some(10));
    }
}

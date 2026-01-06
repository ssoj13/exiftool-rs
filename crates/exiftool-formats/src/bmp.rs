//! BMP format parser.
//!
//! BMP (Windows Bitmap) structure:
//! - File Header (14 bytes): "BM" magic, file size, reserved, pixel data offset
//! - DIB Header (variable): BITMAPINFOHEADER (40 bytes) or newer variants
//!
//! Metadata extracted:
//! - ImageWidth, ImageHeight
//! - BitsPerPixel, Compression
//! - XResolution, YResolution (pixels per meter)
//! - ColorTableSize

use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// BMP format parser.
pub struct BmpParser;

impl FormatParser for BmpParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 2 {
            return false;
        }
        // BMP magic: "BM"
        header[0] == b'B' && header[1] == b'M'
    }

    fn format_name(&self) -> &'static str {
        "BMP"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["bmp", "dib"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("BMP");

        // Read file header (14 bytes)
        let mut file_header = [0u8; 14];
        reader.read_exact(&mut file_header)?;

        if file_header[0] != b'B' || file_header[1] != b'M' {
            return Err(Error::InvalidStructure("Not a valid BMP file".into()));
        }

        let file_size = u32::from_le_bytes([file_header[2], file_header[3], file_header[4], file_header[5]]);
        let _pixel_offset = u32::from_le_bytes([file_header[10], file_header[11], file_header[12], file_header[13]]);

        metadata.exif.set("FileSize", AttrValue::UInt(file_size));

        // Read DIB header size (first 4 bytes of DIB header)
        let mut dib_size_buf = [0u8; 4];
        reader.read_exact(&mut dib_size_buf)?;
        let dib_header_size = u32::from_le_bytes(dib_size_buf);

        // Determine header type and parse accordingly
        let header_type = match dib_header_size {
            12 => "BITMAPCOREHEADER",
            40 => "BITMAPINFOHEADER",
            52 => "BITMAPV2INFOHEADER",
            56 => "BITMAPV3INFOHEADER",
            108 => "BITMAPV4HEADER",
            124 => "BITMAPV5HEADER",
            _ => "Unknown",
        };
        metadata.exif.set("DIBHeaderType", AttrValue::Str(header_type.to_string()));

        if dib_header_size == 12 {
            // BITMAPCOREHEADER (OS/2 1.x format)
            self.parse_core_header(reader, &mut metadata)?;
        } else if dib_header_size >= 40 {
            // BITMAPINFOHEADER or newer
            self.parse_info_header(reader, &mut metadata, dib_header_size)?;
        } else {
            return Err(Error::InvalidStructure(format!("Unknown DIB header size: {}", dib_header_size)));
        }

        Ok(metadata)
    }
}

impl BmpParser {
    /// Parse BITMAPCOREHEADER (12 bytes, OS/2 1.x format).
    fn parse_core_header(&self, reader: &mut dyn ReadSeek, metadata: &mut Metadata) -> Result<()> {
        // Already read 4 bytes (header size), need 8 more
        let mut data = [0u8; 8];
        reader.read_exact(&mut data)?;

        let width = u16::from_le_bytes([data[0], data[1]]);
        let height = u16::from_le_bytes([data[2], data[3]]);
        let _planes = u16::from_le_bytes([data[4], data[5]]);
        let bpp = u16::from_le_bytes([data[6], data[7]]);

        metadata.exif.set("ImageWidth", AttrValue::UInt(width as u32));
        metadata.exif.set("ImageHeight", AttrValue::UInt(height as u32));
        metadata.exif.set("BitsPerPixel", AttrValue::UInt(bpp as u32));

        Ok(())
    }

    /// Parse BITMAPINFOHEADER or newer (40+ bytes).
    fn parse_info_header(&self, reader: &mut dyn ReadSeek, metadata: &mut Metadata, header_size: u32) -> Result<()> {
        // Already read 4 bytes (header size), need at least 36 more for BITMAPINFOHEADER
        let mut data = [0u8; 36];
        reader.read_exact(&mut data)?;

        let width = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let height = i32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let _planes = u16::from_le_bytes([data[8], data[9]]);
        let bpp = u16::from_le_bytes([data[10], data[11]]);
        let compression = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
        let _image_size = u32::from_le_bytes([data[16], data[17], data[18], data[19]]);
        let x_ppm = i32::from_le_bytes([data[20], data[21], data[22], data[23]]); // X pixels per meter
        let y_ppm = i32::from_le_bytes([data[24], data[25], data[26], data[27]]); // Y pixels per meter
        let colors_used = u32::from_le_bytes([data[28], data[29], data[30], data[31]]);
        let _colors_important = u32::from_le_bytes([data[32], data[33], data[34], data[35]]);

        // Height can be negative (top-down DIB)
        let (actual_height, top_down) = if height < 0 {
            (-height, true)
        } else {
            (height, false)
        };

        metadata.exif.set("ImageWidth", AttrValue::UInt(width.unsigned_abs()));
        metadata.exif.set("ImageHeight", AttrValue::UInt(actual_height as u32));
        metadata.exif.set("BitsPerPixel", AttrValue::UInt(bpp as u32));

        // Compression type
        let compression_name = match compression {
            0 => "None (BI_RGB)",
            1 => "RLE8 (BI_RLE8)",
            2 => "RLE4 (BI_RLE4)",
            3 => "Bitfields (BI_BITFIELDS)",
            4 => "JPEG (BI_JPEG)",
            5 => "PNG (BI_PNG)",
            6 => "Bitfields + Alpha (BI_ALPHABITFIELDS)",
            _ => "Unknown",
        };
        metadata.exif.set("Compression", AttrValue::Str(compression_name.to_string()));

        if top_down {
            metadata.exif.set("Orientation", AttrValue::Str("Top-Down".to_string()));
        }

        // Resolution in pixels per meter -> DPI
        if x_ppm > 0 {
            let x_dpi = (x_ppm as f64 * 0.0254).round() as u32;
            metadata.exif.set("XResolution", AttrValue::UInt(x_dpi));
        }
        if y_ppm > 0 {
            let y_dpi = (y_ppm as f64 * 0.0254).round() as u32;
            metadata.exif.set("YResolution", AttrValue::UInt(y_dpi));
        }

        if colors_used > 0 {
            metadata.exif.set("ColorTableSize", AttrValue::UInt(colors_used));
        } else if bpp <= 8 {
            // Default color table size for indexed images
            metadata.exif.set("ColorTableSize", AttrValue::UInt(1 << bpp));
        }

        // Skip remaining header bytes if V4 or V5
        if header_size > 40 {
            let remaining = header_size - 40;
            reader.seek(SeekFrom::Current(remaining as i64))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_bmp(width: u32, height: i32, bpp: u16) -> Vec<u8> {
        let mut data = Vec::new();
        // File header (14 bytes)
        data.extend_from_slice(b"BM");
        data.extend_from_slice(&100u32.to_le_bytes()); // File size (dummy)
        data.extend_from_slice(&[0, 0, 0, 0]); // Reserved
        data.extend_from_slice(&54u32.to_le_bytes()); // Pixel data offset
        // DIB header (BITMAPINFOHEADER - 40 bytes)
        data.extend_from_slice(&40u32.to_le_bytes()); // Header size
        data.extend_from_slice(&width.to_le_bytes());
        data.extend_from_slice(&height.to_le_bytes());
        data.extend_from_slice(&1u16.to_le_bytes()); // Planes
        data.extend_from_slice(&bpp.to_le_bytes()); // Bits per pixel
        data.extend_from_slice(&0u32.to_le_bytes()); // Compression (none)
        data.extend_from_slice(&0u32.to_le_bytes()); // Image size
        data.extend_from_slice(&2835i32.to_le_bytes()); // X pixels/meter (72 DPI)
        data.extend_from_slice(&2835i32.to_le_bytes()); // Y pixels/meter (72 DPI)
        data.extend_from_slice(&0u32.to_le_bytes()); // Colors used
        data.extend_from_slice(&0u32.to_le_bytes()); // Important colors
        data
    }

    #[test]
    fn detect_bmp() {
        let parser = BmpParser;
        assert!(parser.can_parse(b"BM"));
        assert!(parser.can_parse(b"BM\x00\x00"));
    }

    #[test]
    fn reject_non_bmp() {
        let parser = BmpParser;
        assert!(!parser.can_parse(b"\xFF\xD8\xFF")); // JPEG
        assert!(!parser.can_parse(b"\x89PNG")); // PNG
        assert!(!parser.can_parse(b"GIF89a")); // GIF
    }

    #[test]
    fn parse_simple_bmp() {
        let parser = BmpParser;
        let data = make_bmp(640, 480, 24);
        let mut cursor = Cursor::new(data);

        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "BMP");
        assert_eq!(meta.exif.get_u32("ImageWidth"), Some(640));
        assert_eq!(meta.exif.get_u32("ImageHeight"), Some(480));
        assert_eq!(meta.exif.get_u32("BitsPerPixel"), Some(24));
        assert_eq!(meta.exif.get_str("Compression"), Some("None (BI_RGB)"));
    }

    #[test]
    fn parse_top_down_bmp() {
        let parser = BmpParser;
        let data = make_bmp(100, -50, 32); // Negative height = top-down
        let mut cursor = Cursor::new(data);

        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_u32("ImageHeight"), Some(50));
        assert_eq!(meta.exif.get_str("Orientation"), Some("Top-Down"));
    }

    #[test]
    fn parse_indexed_bmp() {
        let parser = BmpParser;
        let data = make_bmp(256, 256, 8);
        let mut cursor = Cursor::new(data);

        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_u32("BitsPerPixel"), Some(8));
        assert_eq!(meta.exif.get_u32("ColorTableSize"), Some(256));
    }
}

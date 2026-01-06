//! SGI (Silicon Graphics Image) format parser.
//!
//! Also known as IRIS or RGB format.
//!
//! # Header Structure (512 bytes)
//!
//! - 2 bytes: Magic (0x01DA)
//! - 1 byte: Storage type (0=verbatim, 1=RLE)
//! - 1 byte: Bytes per pixel channel (1 or 2)
//! - 2 bytes: Dimension (1, 2, or 3)
//! - 2 bytes: Width
//! - 2 bytes: Height
//! - 2 bytes: Channels
//! - 4 bytes: Min pixel value
//! - 4 bytes: Max pixel value
//! - 4 bytes: Reserved
//! - 80 bytes: Image name
//! - 4 bytes: Colormap ID
//! - 404 bytes: Padding

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// SGI format parser.
pub struct SgiParser;

impl FormatParser for SgiParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 12 {
            return false;
        }
        // Magic: 0x01DA (big-endian)
        if header[0] != 0x01 || header[1] != 0xDA {
            return false;
        }
        // Storage: 0 (verbatim) or 1 (RLE)
        if header[2] > 1 {
            return false;
        }
        // BPC: 1 or 2 bytes per channel
        let bpc = header[3];
        if bpc != 1 && bpc != 2 {
            return false;
        }
        // Dimension: 1, 2, or 3
        let dim = u16::from_be_bytes([header[4], header[5]]);
        if dim == 0 || dim > 3 {
            return false;
        }
        // Width sanity check
        let width = u16::from_be_bytes([header[6], header[7]]);
        if width == 0 {
            return false;
        }
        true
    }

    fn format_name(&self) -> &'static str {
        "SGI"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["sgi", "rgb", "rgba", "bw", "iris", "int"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("SGI");

        // Read 512-byte header
        let mut header = [0u8; 512];
        reader.seek(SeekFrom::Start(0))?;
        reader.read_exact(&mut header)?;

        // Validate magic
        if header[0] != 0x01 || header[1] != 0xDA {
            return Err(crate::Error::InvalidStructure("Not a valid SGI file".to_string()));
        }

        meta.exif.set("File:FileType", AttrValue::Str("SGI".to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("image/x-sgi".to_string()));

        // Storage type
        let storage = header[2];
        meta.exif.set(
            "SGI:Compression",
            AttrValue::Str(if storage == 0 { "None" } else { "RLE" }.to_string()),
        );

        // Bytes per channel
        let bpc = header[3] as u32;
        meta.exif.set("SGI:BytesPerChannel", AttrValue::UInt(bpc));

        // Dimension
        let dimension = u16::from_be_bytes([header[4], header[5]]);
        meta.exif.set("SGI:Dimension", AttrValue::UInt(dimension as u32));

        // Dimensions
        let width = u16::from_be_bytes([header[6], header[7]]) as u32;
        let height = u16::from_be_bytes([header[8], header[9]]) as u32;
        let channels = u16::from_be_bytes([header[10], header[11]]) as u32;

        meta.exif.set("File:ImageWidth", AttrValue::UInt(width));
        meta.exif.set("File:ImageHeight", AttrValue::UInt(height));
        meta.exif.set("SGI:NumChannels", AttrValue::UInt(channels));

        // Bits per sample
        meta.exif.set("File:BitsPerSample", AttrValue::UInt(bpc * 8));

        // Color mode
        let color_mode = match channels {
            1 => "Grayscale",
            2 => "Grayscale+Alpha",
            3 => "RGB",
            4 => "RGBA",
            _ => "Unknown",
        };
        meta.exif.set("SGI:ColorMode", AttrValue::Str(color_mode.to_string()));

        // Pixel range
        let min_val = u32::from_be_bytes([header[12], header[13], header[14], header[15]]);
        let max_val = u32::from_be_bytes([header[16], header[17], header[18], header[19]]);
        meta.exif.set("SGI:MinPixelValue", AttrValue::UInt(min_val));
        meta.exif.set("SGI:MaxPixelValue", AttrValue::UInt(max_val));

        // Image name (80 bytes at offset 24)
        let name_bytes = &header[24..104];
        let name = extract_string(name_bytes);
        if !name.is_empty() {
            meta.exif.set("SGI:ImageName", AttrValue::Str(name));
        }

        // Colormap ID (offset 104)
        let colormap = u32::from_be_bytes([header[104], header[105], header[106], header[107]]);
        let colormap_name = match colormap {
            0 => "Normal",
            1 => "Dithered",
            2 => "Screen",
            3 => "Colormap",
            _ => "Unknown",
        };
        meta.exif.set("SGI:ColormapType", AttrValue::Str(colormap_name.to_string()));

        // File size
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        Ok(meta)
    }
}

/// Extract null-terminated string from bytes.
fn extract_string(data: &[u8]) -> String {
    let end = data.iter().position(|&b| b == 0).unwrap_or(data.len());
    String::from_utf8_lossy(&data[..end]).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_sgi_header(width: u16, height: u16, channels: u16, name: &str) -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // Magic
        data[0] = 0x01;
        data[1] = 0xDA;
        // Storage: verbatim
        data[2] = 0;
        // BPC: 1
        data[3] = 1;
        // Dimension: 3 (RGB)
        data[4..6].copy_from_slice(&3u16.to_be_bytes());
        // Width
        data[6..8].copy_from_slice(&width.to_be_bytes());
        // Height
        data[8..10].copy_from_slice(&height.to_be_bytes());
        // Channels
        data[10..12].copy_from_slice(&channels.to_be_bytes());
        // Min/max pixel
        data[12..16].copy_from_slice(&0u32.to_be_bytes());
        data[16..20].copy_from_slice(&255u32.to_be_bytes());
        // Name
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(79);
        data[24..24 + len].copy_from_slice(&name_bytes[..len]);
        data
    }

    #[test]
    fn test_can_parse() {
        let parser = SgiParser;
        let data = make_sgi_header(640, 480, 3, "");
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = SgiParser;
        // Wrong magic
        assert!(!parser.can_parse(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]));
        // Not enough data
        assert!(!parser.can_parse(&[0x01, 0xDA]));
    }

    #[test]
    fn test_parse_basic() {
        let parser = SgiParser;
        let data = make_sgi_header(800, 600, 3, "");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "SGI");
        assert_eq!(meta.exif.get_u32("File:ImageWidth"), Some(800));
        assert_eq!(meta.exif.get_u32("File:ImageHeight"), Some(600));
        assert_eq!(meta.exif.get_u32("SGI:NumChannels"), Some(3));
        assert_eq!(meta.exif.get_str("SGI:ColorMode"), Some("RGB"));
        assert_eq!(meta.exif.get_str("SGI:Compression"), Some("None"));
    }

    #[test]
    fn test_parse_with_name() {
        let parser = SgiParser;
        let data = make_sgi_header(320, 240, 4, "Test Image");
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("SGI:ImageName"), Some("Test Image"));
        assert_eq!(meta.exif.get_str("SGI:ColorMode"), Some("RGBA"));
    }

    #[test]
    fn test_parse_grayscale() {
        let parser = SgiParser;
        let mut data = make_sgi_header(256, 256, 1, "");
        // Set dimension to 2 for grayscale
        data[4..6].copy_from_slice(&2u16.to_be_bytes());
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("SGI:ColorMode"), Some("Grayscale"));
    }

    #[test]
    fn test_parse_rle() {
        let parser = SgiParser;
        let mut data = make_sgi_header(512, 512, 3, "");
        data[2] = 1; // RLE compression
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("SGI:Compression"), Some("RLE"));
    }
}

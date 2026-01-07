//! PCX (ZSoft Paintbrush) format parser.
//!
//! PCX is a legacy raster image format from DOS era.
//!
//! Structure:
//! - 128-byte header
//! - RLE-compressed image data
//! - Optional 256-color palette (769 bytes from EOF)

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// PCX magic byte.
const PCX_MAGIC: u8 = 0x0A;

/// PCX parser.
pub struct PcxParser;

impl FormatParser for PcxParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 4 {
            return false;
        }

        // Magic byte
        if header[0] != PCX_MAGIC {
            return false;
        }

        // Version: 0, 2, 3, 4, 5
        let version = header[1];
        if !matches!(version, 0 | 2 | 3 | 4 | 5) {
            return false;
        }

        // Encoding: 0 (none) or 1 (RLE)
        let encoding = header[2];
        if encoding > 1 {
            return false;
        }

        // Bits per plane: 1, 2, 4, 8
        let bpp = header[3];
        if !matches!(bpp, 1 | 2 | 4 | 8) {
            return false;
        }

        true
    }

    fn format_name(&self) -> &'static str {
        "PCX"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["pcx", "pcc", "dcx"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut header = [0u8; 128];
        reader.read_exact(&mut header)?;

        let mut metadata = Metadata::new("PCX");
        metadata.set_file_type("PCX", "image/x-pcx");

        // Parse header
        let version = header[1];
        let encoding = header[2];
        let bits_per_plane = header[3];

        // Image bounds
        let x_min = u16::from_le_bytes([header[4], header[5]]);
        let y_min = u16::from_le_bytes([header[6], header[7]]);
        let x_max = u16::from_le_bytes([header[8], header[9]]);
        let y_max = u16::from_le_bytes([header[10], header[11]]);

        let width = x_max - x_min + 1;
        let height = y_max - y_min + 1;

        metadata.exif.set("File:ImageWidth", AttrValue::UInt(width as u32));
        metadata.exif.set("File:ImageHeight", AttrValue::UInt(height as u32));

        // DPI
        let h_dpi = u16::from_le_bytes([header[12], header[13]]);
        let v_dpi = u16::from_le_bytes([header[14], header[15]]);
        if h_dpi > 0 && h_dpi < 10000 {
            metadata.exif.set("PCX:XResolution", AttrValue::UInt(h_dpi as u32));
        }
        if v_dpi > 0 && v_dpi < 10000 {
            metadata.exif.set("PCX:YResolution", AttrValue::UInt(v_dpi as u32));
        }

        // Version
        let version_str = match version {
            0 => "2.5",
            2 => "2.8 with palette",
            3 => "2.8 without palette",
            4 => "Paintbrush for Windows",
            5 => "3.0+",
            _ => "Unknown",
        };
        metadata.exif.set("PCX:Version", AttrValue::Str(version_str.to_string()));

        // Encoding
        let encoding_str = if encoding == 1 { "RLE" } else { "None" };
        metadata.exif.set("PCX:Compression", AttrValue::Str(encoding_str.to_string()));

        // Bits per plane
        metadata.exif.set("PCX:BitsPerPlane", AttrValue::UInt(bits_per_plane as u32));

        // Number of planes (byte 65)
        let num_planes = header[65];
        metadata.exif.set("PCX:NumPlanes", AttrValue::UInt(num_planes as u32));

        // Calculate total bits per pixel
        let total_bpp = bits_per_plane as u32 * num_planes as u32;
        metadata.exif.set("PCX:BitsPerPixel", AttrValue::UInt(total_bpp));

        // Color mode
        let color_mode = match (bits_per_plane, num_planes) {
            (1, 1) => "Monochrome",
            (1, 4) => "16-color",
            (4, 1) => "16-color",
            (8, 1) => "256-color (indexed)",
            (8, 3) => "24-bit RGB",
            (8, 4) => "32-bit RGBA",
            _ => "Custom",
        };
        metadata.exif.set("PCX:ColorMode", AttrValue::Str(color_mode.to_string()));

        // Bytes per line (bytes 66-67)
        let bytes_per_line = u16::from_le_bytes([header[66], header[67]]);
        metadata.exif.set("PCX:BytesPerLine", AttrValue::UInt(bytes_per_line as u32));

        // Palette type (bytes 68-69)
        let palette_type = u16::from_le_bytes([header[68], header[69]]);
        let palette_str = match palette_type {
            1 => "Color/BW",
            2 => "Grayscale",
            _ => "Unknown",
        };
        metadata.exif.set("PCX:PaletteType", AttrValue::Str(palette_str.to_string()));

        // Screen size (bytes 70-73) - PCX 3.0+
        let screen_width = u16::from_le_bytes([header[70], header[71]]);
        let screen_height = u16::from_le_bytes([header[72], header[73]]);
        if screen_width > 0 && screen_height > 0 {
            metadata.exif.set("PCX:ScreenWidth", AttrValue::UInt(screen_width as u32));
            metadata.exif.set("PCX:ScreenHeight", AttrValue::UInt(screen_height as u32));
        }

        // Check for 256-color palette at EOF (version 5, 8-bit)
        if version == 5 && bits_per_plane == 8 && num_planes == 1 {
            self.check_vga_palette(reader, &mut metadata)?;
        }

        Ok(metadata)
    }
}

impl PcxParser {
    /// Check for 256-color VGA palette at end of file.
    fn check_vga_palette(&self, reader: &mut dyn ReadSeek, metadata: &mut Metadata) -> Result<()> {
        let file_size = crate::utils::get_file_size(reader)?;
        if file_size < 769 {
            return Ok(());
        }

        reader.seek(SeekFrom::End(-769))?;
        let mut marker = [0u8; 1];
        reader.read_exact(&mut marker)?;

        if marker[0] == 0x0C {
            metadata.exif.set("PCX:HasVGAPalette", AttrValue::Bool(true));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_pcx_header(width: u16, height: u16, bpp: u8, planes: u8) -> Vec<u8> {
        let mut header = vec![0u8; 128];
        header[0] = PCX_MAGIC;
        header[1] = 5; // Version 3.0+
        header[2] = 1; // RLE encoding
        header[3] = bpp;
        // x_min, y_min = 0
        header[8..10].copy_from_slice(&(width - 1).to_le_bytes()); // x_max
        header[10..12].copy_from_slice(&(height - 1).to_le_bytes()); // y_max
        header[12..14].copy_from_slice(&72u16.to_le_bytes()); // h_dpi
        header[14..16].copy_from_slice(&72u16.to_le_bytes()); // v_dpi
        header[65] = planes;
        header[66..68].copy_from_slice(&((width as u16 + 1) / 2 * 2).to_le_bytes()); // bytes per line
        header[68..70].copy_from_slice(&1u16.to_le_bytes()); // palette type
        header
    }

    #[test]
    fn test_can_parse() {
        let parser = PcxParser;
        let header = make_pcx_header(640, 480, 8, 1);
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn test_can_parse_24bit() {
        let parser = PcxParser;
        let header = make_pcx_header(800, 600, 8, 3);
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = PcxParser;
        // Wrong magic
        let mut header = make_pcx_header(640, 480, 8, 1);
        header[0] = 0xFF;
        assert!(!parser.can_parse(&header));

        // Invalid version
        let mut header = make_pcx_header(640, 480, 8, 1);
        header[1] = 99;
        assert!(!parser.can_parse(&header));
    }

    #[test]
    fn test_parse_basic() {
        let parser = PcxParser;
        let header = make_pcx_header(640, 480, 8, 3);
        let mut cursor = Cursor::new(header);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("File:FileType"), Some("PCX"));
        assert_eq!(meta.exif.get_u32("File:ImageWidth"), Some(640));
        assert_eq!(meta.exif.get_u32("File:ImageHeight"), Some(480));
        assert_eq!(meta.exif.get_u32("PCX:BitsPerPixel"), Some(24));
        assert_eq!(meta.exif.get_str("PCX:ColorMode"), Some("24-bit RGB"));
    }

    #[test]
    fn test_parse_indexed() {
        let parser = PcxParser;
        let header = make_pcx_header(320, 200, 8, 1);
        let mut cursor = Cursor::new(header);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("PCX:ColorMode"), Some("256-color (indexed)"));
        assert_eq!(meta.exif.get_u32("PCX:BitsPerPixel"), Some(8));
    }
}

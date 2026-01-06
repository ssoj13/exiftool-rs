//! TGA (Truevision TGA/TARGA) format parser.
//!
//! TGA is a raster graphics format commonly used in game development.
//!
//! Structure:
//! - 18-byte header
//! - Optional image ID (0-255 bytes)
//! - Optional color map
//! - Image data
//! - Optional TGA 2.0 footer (26 bytes from EOF)

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// TGA 2.0 footer signature.
const TGA_FOOTER_SIG: &[u8] = b"TRUEVISION-XFILE.";

/// TGA parser.
pub struct TgaParser;

impl FormatParser for TgaParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 18 {
            return false;
        }

        // TGA has no magic number, so we validate header fields
        let color_map_type = header[1];
        let image_type = header[2];

        // Color map type: 0 or 1
        if color_map_type > 1 {
            return false;
        }

        // Valid image types: 0-3, 9-11, 32-33
        let valid_types = [0, 1, 2, 3, 9, 10, 11, 32, 33];
        if !valid_types.contains(&image_type) {
            return false;
        }

        // If no color map, color map fields should be zero
        if color_map_type == 0 {
            let cm_first = u16::from_le_bytes([header[3], header[4]]);
            let cm_length = u16::from_le_bytes([header[5], header[6]]);
            let cm_depth = header[7];
            if cm_first != 0 || cm_length != 0 || cm_depth != 0 {
                return false;
            }
        }

        // Pixel depth: 8, 15, 16, 24, 32
        let pixel_depth = header[16];
        let valid_depths = [8, 15, 16, 24, 32];
        if image_type != 0 && !valid_depths.contains(&pixel_depth) {
            return false;
        }

        true
    }

    fn format_name(&self) -> &'static str {
        "TGA"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["tga", "tpic", "vda", "icb", "vst"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut header = [0u8; 18];
        reader.read_exact(&mut header)?;

        let mut metadata = Metadata::new("TGA");
        metadata.exif.set("File:FileType", AttrValue::Str("TGA".to_string()));
        metadata.exif.set("File:MIMEType", AttrValue::Str("image/x-tga".to_string()));

        // Parse header
        let id_length = header[0] as usize;
        let color_map_type = header[1];
        let image_type = header[2];

        // Color map spec
        let _cm_first = u16::from_le_bytes([header[3], header[4]]);
        let cm_length = u16::from_le_bytes([header[5], header[6]]);
        let cm_depth = header[7];

        // Image spec
        let x_origin = u16::from_le_bytes([header[8], header[9]]);
        let y_origin = u16::from_le_bytes([header[10], header[11]]);
        let width = u16::from_le_bytes([header[12], header[13]]);
        let height = u16::from_le_bytes([header[14], header[15]]);
        let pixel_depth = header[16];
        let descriptor = header[17];

        // Image dimensions
        metadata.exif.set("File:ImageWidth", AttrValue::UInt(width as u32));
        metadata.exif.set("File:ImageHeight", AttrValue::UInt(height as u32));
        metadata.exif.set("TGA:BitsPerPixel", AttrValue::UInt(pixel_depth as u32));

        // Image type
        let (type_name, compression) = match image_type {
            0 => ("No Image", "None"),
            1 => ("Color-Mapped", "None"),
            2 => ("True-Color", "None"),
            3 => ("Grayscale", "None"),
            9 => ("Color-Mapped", "RLE"),
            10 => ("True-Color", "RLE"),
            11 => ("Grayscale", "RLE"),
            32 => ("Color-Mapped", "Huffman/Delta/RLE"),
            33 => ("Color-Mapped", "Huffman/Delta/RLE (4-pass)"),
            _ => ("Unknown", "Unknown"),
        };
        metadata.exif.set("TGA:ImageType", AttrValue::Str(type_name.to_string()));
        metadata.exif.set("TGA:Compression", AttrValue::Str(compression.to_string()));

        // Color map info
        if color_map_type == 1 {
            metadata.exif.set("TGA:ColorMapEntries", AttrValue::UInt(cm_length as u32));
            metadata.exif.set("TGA:ColorMapDepth", AttrValue::UInt(cm_depth as u32));
        }

        // Origin
        if x_origin != 0 || y_origin != 0 {
            metadata.exif.set("TGA:XOrigin", AttrValue::UInt(x_origin as u32));
            metadata.exif.set("TGA:YOrigin", AttrValue::UInt(y_origin as u32));
        }

        // Alpha bits
        let alpha_bits = descriptor & 0x0F;
        if alpha_bits > 0 {
            metadata.exif.set("TGA:AlphaBits", AttrValue::UInt(alpha_bits as u32));
        }

        // Image origin (top-left vs bottom-left)
        let origin_top = (descriptor & 0x20) != 0;
        let origin_right = (descriptor & 0x10) != 0;
        let origin_str = match (origin_top, origin_right) {
            (false, false) => "Bottom-Left",
            (false, true) => "Bottom-Right",
            (true, false) => "Top-Left",
            (true, true) => "Top-Right",
        };
        metadata.exif.set("TGA:ImageOrigin", AttrValue::Str(origin_str.to_string()));

        // Read image ID if present
        if id_length > 0 {
            let mut id_buf = vec![0u8; id_length];
            reader.read_exact(&mut id_buf)?;
            let id_str = String::from_utf8_lossy(&id_buf)
                .trim_end_matches('\0')
                .to_string();
            if !id_str.is_empty() {
                metadata.exif.set("TGA:ImageID", AttrValue::Str(id_str));
            }
        }

        // Check for TGA 2.0 footer
        self.parse_footer(reader, &mut metadata)?;

        Ok(metadata)
    }
}

impl TgaParser {
    /// Parse TGA 2.0 footer (26 bytes from EOF).
    fn parse_footer(&self, reader: &mut dyn ReadSeek, metadata: &mut Metadata) -> Result<()> {
        let file_size = reader.seek(SeekFrom::End(0))?;
        if file_size < 26 {
            return Ok(());
        }

        reader.seek(SeekFrom::End(-26))?;
        let mut footer = [0u8; 26];
        reader.read_exact(&mut footer)?;

        // Check signature (bytes 8-24)
        if &footer[8..25] != TGA_FOOTER_SIG {
            return Ok(()); // Not TGA 2.0
        }

        metadata.exif.set("TGA:Version", AttrValue::Str("2.0".to_string()));

        let ext_offset = u32::from_le_bytes([footer[0], footer[1], footer[2], footer[3]]);
        let dev_offset = u32::from_le_bytes([footer[4], footer[5], footer[6], footer[7]]);

        if ext_offset != 0 {
            metadata.exif.set("TGA:HasExtension", AttrValue::Bool(true));
            self.parse_extension(reader, ext_offset as u64, metadata)?;
        }

        if dev_offset != 0 {
            metadata.exif.set("TGA:HasDeveloperArea", AttrValue::Bool(true));
        }

        Ok(())
    }

    /// Parse TGA 2.0 extension area.
    fn parse_extension(&self, reader: &mut dyn ReadSeek, offset: u64, metadata: &mut Metadata) -> Result<()> {
        reader.seek(SeekFrom::Start(offset))?;

        let mut ext = [0u8; 495];
        if reader.read_exact(&mut ext).is_err() {
            return Ok(());
        }

        let ext_size = u16::from_le_bytes([ext[0], ext[1]]);
        if ext_size < 495 {
            return Ok(()); // Invalid extension
        }

        // Author name (bytes 2-42)
        let author = String::from_utf8_lossy(&ext[2..43])
            .trim_end_matches('\0')
            .to_string();
        if !author.is_empty() {
            metadata.exif.set("TGA:Author", AttrValue::Str(author));
        }

        // Author comments (bytes 43-366, 4 lines of 81 chars)
        let mut comments = String::new();
        for i in 0..4 {
            let start = 43 + i * 81;
            let line = String::from_utf8_lossy(&ext[start..start + 80])
                .trim_end_matches('\0')
                .to_string();
            if !line.is_empty() {
                if !comments.is_empty() {
                    comments.push('\n');
                }
                comments.push_str(&line);
            }
        }
        if !comments.is_empty() {
            metadata.exif.set("TGA:Comments", AttrValue::Str(comments));
        }

        // Date/time (bytes 367-378)
        let month = u16::from_le_bytes([ext[367], ext[368]]);
        let day = u16::from_le_bytes([ext[369], ext[370]]);
        let year = u16::from_le_bytes([ext[371], ext[372]]);
        let hour = u16::from_le_bytes([ext[373], ext[374]]);
        let minute = u16::from_le_bytes([ext[375], ext[376]]);
        let second = u16::from_le_bytes([ext[377], ext[378]]);

        if year != 0 && month != 0 && day != 0 {
            let datetime = format!(
                "{:04}:{:02}:{:02} {:02}:{:02}:{:02}",
                year, month, day, hour, minute, second
            );
            metadata.exif.set("TGA:DateTimeCreated", AttrValue::Str(datetime));
        }

        // Job name (bytes 379-419)
        let job = String::from_utf8_lossy(&ext[379..420])
            .trim_end_matches('\0')
            .to_string();
        if !job.is_empty() {
            metadata.exif.set("TGA:JobName", AttrValue::Str(job));
        }

        // Software ID (bytes 426-466)
        let software = String::from_utf8_lossy(&ext[426..467])
            .trim_end_matches('\0')
            .to_string();
        if !software.is_empty() {
            metadata.exif.set("TGA:Software", AttrValue::Str(software));
        }

        // Software version (bytes 467-469)
        let sw_ver_num = u16::from_le_bytes([ext[467], ext[468]]);
        let sw_ver_letter = ext[469];
        if sw_ver_num != 0 {
            let version = if sw_ver_letter != 0 && sw_ver_letter != b' ' {
                format!("{}.{}{}", sw_ver_num / 100, sw_ver_num % 100, sw_ver_letter as char)
            } else {
                format!("{}.{}", sw_ver_num / 100, sw_ver_num % 100)
            };
            metadata.exif.set("TGA:SoftwareVersion", AttrValue::Str(version));
        }

        // Key color (bytes 470-473) - ARGB
        let key_a = ext[473];
        let key_r = ext[472];
        let key_g = ext[471];
        let key_b = ext[470];
        if key_a != 0 || key_r != 0 || key_g != 0 || key_b != 0 {
            let key_color = format!("#{:02X}{:02X}{:02X}{:02X}", key_a, key_r, key_g, key_b);
            metadata.exif.set("TGA:KeyColor", AttrValue::Str(key_color));
        }

        // Pixel aspect ratio (bytes 474-477)
        let aspect_num = u16::from_le_bytes([ext[474], ext[475]]);
        let aspect_den = u16::from_le_bytes([ext[476], ext[477]]);
        if aspect_den != 0 && aspect_num != 0 {
            let ratio = aspect_num as f32 / aspect_den as f32;
            metadata.exif.set("TGA:PixelAspectRatio", AttrValue::Float(ratio));
        }

        // Gamma (bytes 478-481)
        let gamma_num = u16::from_le_bytes([ext[478], ext[479]]);
        let gamma_den = u16::from_le_bytes([ext[480], ext[481]]);
        if gamma_den != 0 && gamma_num != 0 {
            let gamma = gamma_num as f32 / gamma_den as f32;
            metadata.exif.set("TGA:Gamma", AttrValue::Float(gamma));
        }

        // Alpha type (byte 494)
        let alpha_type = ext[494];
        let alpha_str = match alpha_type {
            0 => "No Alpha",
            1 => "Undefined (ignore)",
            2 => "Undefined (retain)",
            3 => "Useful Alpha",
            4 => "Pre-multiplied Alpha",
            _ => "Unknown",
        };
        metadata.exif.set("TGA:AlphaType", AttrValue::Str(alpha_str.to_string()));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_tga_header(width: u16, height: u16, depth: u8, image_type: u8) -> Vec<u8> {
        let mut header = vec![0u8; 18];
        header[2] = image_type;
        header[12..14].copy_from_slice(&width.to_le_bytes());
        header[14..16].copy_from_slice(&height.to_le_bytes());
        header[16] = depth;
        header[17] = 0x20; // Top-left origin
        header
    }

    #[test]
    fn test_can_parse_true_color() {
        let parser = TgaParser;
        let header = make_tga_header(640, 480, 24, 2);
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn test_can_parse_rle() {
        let parser = TgaParser;
        let header = make_tga_header(640, 480, 32, 10);
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = TgaParser;
        // Invalid image type
        let mut header = make_tga_header(640, 480, 24, 2);
        header[2] = 99;
        assert!(!parser.can_parse(&header));

        // Invalid color map type
        let mut header = make_tga_header(640, 480, 24, 2);
        header[1] = 5;
        assert!(!parser.can_parse(&header));
    }

    #[test]
    fn test_parse_basic() {
        let parser = TgaParser;
        let header = make_tga_header(800, 600, 32, 2);
        let mut cursor = Cursor::new(header);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("File:FileType"), Some("TGA"));
        assert_eq!(meta.exif.get_u32("File:ImageWidth"), Some(800));
        assert_eq!(meta.exif.get_u32("File:ImageHeight"), Some(600));
        assert_eq!(meta.exif.get_u32("TGA:BitsPerPixel"), Some(32));
        assert_eq!(meta.exif.get_str("TGA:ImageType"), Some("True-Color"));
        assert_eq!(meta.exif.get_str("TGA:Compression"), Some("None"));
    }

    #[test]
    fn test_parse_with_image_id() {
        let parser = TgaParser;
        let mut data = make_tga_header(100, 100, 24, 2);
        data[0] = 10; // ID length
        data.extend_from_slice(b"Test ID\0\0\0"); // 10 bytes

        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("TGA:ImageID"), Some("Test ID"));
    }

    #[test]
    fn test_parse_with_footer() {
        let parser = TgaParser;
        let mut data = make_tga_header(64, 64, 24, 2);
        
        // Add minimal image data
        data.extend(vec![0u8; 64 * 64 * 3]);
        
        // TGA 2.0 footer
        data.extend_from_slice(&0u32.to_le_bytes()); // Extension offset (none)
        data.extend_from_slice(&0u32.to_le_bytes()); // Developer offset (none)
        data.extend_from_slice(TGA_FOOTER_SIG);
        data.push(0); // Terminating null

        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("TGA:Version"), Some("2.0"));
    }
}

//! GIF format parser.
//!
//! GIF (Graphics Interchange Format) structure:
//! - Header: "GIF87a" or "GIF89a"
//! - Logical Screen Descriptor (7 bytes)
//! - Global Color Table (optional)
//! - Extensions (comment, application, graphics control)
//! - Image Data blocks
//!
//! Metadata extracted:
//! - ImageWidth, ImageHeight
//! - ColorTableSize, BackgroundColor
//! - GIFVersion (87a or 89a)
//! - AnimationFrameCount
//! - XMP (from Application Extension)
//! - Comment (from Comment Extension)

use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// GIF format parser.
pub struct GifParser;

impl FormatParser for GifParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 6 {
            return false;
        }
        // GIF87a or GIF89a
        &header[0..3] == b"GIF" && (&header[3..6] == b"87a" || &header[3..6] == b"89a")
    }

    fn format_name(&self) -> &'static str {
        "GIF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["gif"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("GIF");

        // Read header (6 bytes)
        let mut header = [0u8; 6];
        reader.read_exact(&mut header)?;

        if !self.can_parse(&header) {
            return Err(Error::InvalidStructure("Not a valid GIF file".into()));
        }

        // GIF version
        let version = std::str::from_utf8(&header[3..6]).unwrap_or("???");
        metadata.exif.set("GIFVersion", AttrValue::Str(version.to_string()));

        // Logical Screen Descriptor (7 bytes)
        let mut lsd = [0u8; 7];
        reader.read_exact(&mut lsd)?;

        let width = u16::from_le_bytes([lsd[0], lsd[1]]);
        let height = u16::from_le_bytes([lsd[2], lsd[3]]);
        let packed = lsd[4];
        let bg_color = lsd[5];
        let _aspect_ratio = lsd[6];

        metadata.exif.set("ImageWidth", AttrValue::UInt(width as u32));
        metadata.exif.set("ImageHeight", AttrValue::UInt(height as u32));

        // Packed field:
        // bit 7: Global Color Table Flag
        // bits 4-6: Color Resolution (bits per primary color - 1)
        // bit 3: Sort Flag
        // bits 0-2: Size of Global Color Table (2^(N+1) colors)
        let has_gct = (packed & 0x80) != 0;
        let color_resolution = ((packed >> 4) & 0x07) + 1;
        let gct_size_bits = packed & 0x07;

        metadata.exif.set("ColorResolution", AttrValue::UInt(color_resolution as u32));

        if has_gct {
            let gct_entries = 1 << (gct_size_bits + 1);
            metadata.exif.set("ColorTableSize", AttrValue::UInt(gct_entries));
            metadata.exif.set("BackgroundColorIndex", AttrValue::UInt(bg_color as u32));

            // Skip Global Color Table (3 bytes per entry)
            let gct_bytes = gct_entries * 3;
            reader.seek(SeekFrom::Current(gct_bytes as i64))?;
        }

        // Parse extensions and image descriptors
        let mut frame_count = 0u32;
        let mut comment = String::new();
        let mut xmp_data: Option<String> = None;

        loop {
            let mut marker = [0u8; 1];
            if reader.read_exact(&mut marker).is_err() {
                break;
            }

            match marker[0] {
                0x21 => {
                    // Extension block
                    let mut ext_type = [0u8; 1];
                    reader.read_exact(&mut ext_type)?;

                    match ext_type[0] {
                        0xF9 => {
                            // Graphics Control Extension (animation frame marker)
                            self.skip_sub_blocks(reader)?;
                        }
                        0xFE => {
                            // Comment Extension
                            let c = self.read_sub_blocks(reader)?;
                            if comment.is_empty() {
                                comment = String::from_utf8_lossy(&c).to_string();
                            }
                        }
                        0xFF => {
                            // Application Extension
                            let app_data = self.read_sub_blocks(reader)?;
                            if app_data.len() >= 11 {
                                let app_id = &app_data[0..11];
                                // XMP extension: "XMP DataXMP"
                                if app_id == b"XMP DataXMP" && app_data.len() > 11 {
                                    // XMP data follows the app ID
                                    // Find the start of actual XMP (after the magic packet trailer)
                                    if let Some(xmp_start) = find_xmp_start(&app_data[11..]) {
                                        let xmp_bytes = &app_data[11 + xmp_start..];
                                        if let Ok(xmp_str) = std::str::from_utf8(xmp_bytes) {
                                            xmp_data = Some(xmp_str.trim_end_matches('\0').to_string());
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            // Unknown extension, skip
                            self.skip_sub_blocks(reader)?;
                        }
                    }
                }
                0x2C => {
                    // Image Descriptor - this is a frame
                    frame_count += 1;

                    // Image Descriptor is 9 bytes (after the 0x2C marker)
                    let mut img_desc = [0u8; 9];
                    reader.read_exact(&mut img_desc)?;

                    let _left = u16::from_le_bytes([img_desc[0], img_desc[1]]);
                    let _top = u16::from_le_bytes([img_desc[2], img_desc[3]]);
                    let _img_width = u16::from_le_bytes([img_desc[4], img_desc[5]]);
                    let _img_height = u16::from_le_bytes([img_desc[6], img_desc[7]]);
                    let img_packed = img_desc[8];

                    // Local Color Table?
                    let has_lct = (img_packed & 0x80) != 0;
                    if has_lct {
                        let lct_size_bits = img_packed & 0x07;
                        let lct_entries = 1 << (lct_size_bits + 1);
                        reader.seek(SeekFrom::Current((lct_entries * 3) as i64))?;
                    }

                    // Skip LZW minimum code size
                    reader.seek(SeekFrom::Current(1))?;

                    // Skip image data sub-blocks
                    self.skip_sub_blocks(reader)?;
                }
                0x3B => {
                    // Trailer - end of GIF
                    break;
                }
                _ => {
                    // Unknown block, try to continue
                    break;
                }
            }
        }

        // Set metadata
        if frame_count > 1 {
            metadata.exif.set("FrameCount", AttrValue::UInt(frame_count));
            metadata.exif.set("Animation", AttrValue::Str("Yes".to_string()));
        } else {
            metadata.exif.set("Animation", AttrValue::Str("No".to_string()));
        }

        if !comment.is_empty() {
            metadata.exif.set("Comment", AttrValue::Str(comment));
        }

        if let Some(xmp) = xmp_data {
            metadata.xmp = Some(xmp);
        }

        Ok(metadata)
    }
}

impl GifParser {
    /// Skip sub-blocks (each starts with length byte, 0 terminates).
    fn skip_sub_blocks(&self, reader: &mut dyn ReadSeek) -> Result<()> {
        loop {
            let mut len = [0u8; 1];
            reader.read_exact(&mut len)?;
            if len[0] == 0 {
                break;
            }
            reader.seek(SeekFrom::Current(len[0] as i64))?;
        }
        Ok(())
    }

    /// Read sub-blocks into a buffer.
    fn read_sub_blocks(&self, reader: &mut dyn ReadSeek) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        loop {
            let mut len = [0u8; 1];
            reader.read_exact(&mut len)?;
            if len[0] == 0 {
                break;
            }
            let mut block = vec![0u8; len[0] as usize];
            reader.read_exact(&mut block)?;
            data.extend_from_slice(&block);
        }
        Ok(data)
    }
}

/// Find start of XMP data (after magic packet trailer).
fn find_xmp_start(data: &[u8]) -> Option<usize> {
    // XMP in GIF has a "magic trailer" of 258 bytes (0x01..0xFF, 0x00)
    // followed by the actual XMP. We look for "<?xpacket" or "<x:xmpmeta"
    if let Some(pos) = data.windows(9).position(|w| w == b"<?xpacket") {
        return Some(pos);
    }
    if let Some(pos) = data.windows(10).position(|w| w == b"<x:xmpmeta") {
        return Some(pos);
    }
    // If no XMP markers found, data might start at 0
    if data.starts_with(b"<?") || data.starts_with(b"<x") {
        return Some(0);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_gif_header(version: &[u8; 3], width: u16, height: u16, gct_bits: u8) -> Vec<u8> {
        let mut data = Vec::new();
        // Header
        data.extend_from_slice(b"GIF");
        data.extend_from_slice(version);
        // Logical Screen Descriptor
        data.extend_from_slice(&width.to_le_bytes());
        data.extend_from_slice(&height.to_le_bytes());
        // Packed: GCT flag (if gct_bits > 0), 8-bit color res, gct size
        let packed = if gct_bits > 0 {
            0x80 | (7 << 4) | (gct_bits - 1)
        } else {
            7 << 4
        };
        data.push(packed);
        data.push(0); // Background color
        data.push(0); // Aspect ratio
        // GCT (if present)
        if gct_bits > 0 {
            let gct_entries = 1 << gct_bits;
            for _ in 0..(gct_entries * 3) {
                data.push(0);
            }
        }
        // Trailer
        data.push(0x3B);
        data
    }

    #[test]
    fn detect_gif87a() {
        let parser = GifParser;
        assert!(parser.can_parse(b"GIF87a"));
    }

    #[test]
    fn detect_gif89a() {
        let parser = GifParser;
        assert!(parser.can_parse(b"GIF89a"));
    }

    #[test]
    fn reject_non_gif() {
        let parser = GifParser;
        assert!(!parser.can_parse(b"\xFF\xD8\xFF")); // JPEG
        assert!(!parser.can_parse(b"\x89PNG")); // PNG
        assert!(!parser.can_parse(b"GIF90a")); // Invalid version
    }

    #[test]
    fn parse_simple_gif() {
        let parser = GifParser;
        let data = make_gif_header(b"89a", 320, 240, 8);
        let mut cursor = Cursor::new(data);

        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "GIF");
        assert_eq!(meta.exif.get_str("GIFVersion"), Some("89a"));
        assert_eq!(meta.exif.get_u32("ImageWidth"), Some(320));
        assert_eq!(meta.exif.get_u32("ImageHeight"), Some(240));
        assert_eq!(meta.exif.get_u32("ColorTableSize"), Some(256));
    }

    #[test]
    fn parse_gif_no_gct() {
        let parser = GifParser;
        let data = make_gif_header(b"87a", 100, 50, 0);
        let mut cursor = Cursor::new(data);

        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("GIFVersion"), Some("87a"));
        assert_eq!(meta.exif.get_u32("ImageWidth"), Some(100));
        assert_eq!(meta.exif.get_u32("ImageHeight"), Some(50));
        assert_eq!(meta.exif.get_u32("ColorTableSize"), None);
    }
}

//! PNG format parser.
//!
//! PNG structure:
//! - 8-byte signature: 0x89 P N G \r \n 0x1A \n
//! - Chunks: length (4) + type (4) + data (length) + CRC (4)
//!
//! Metadata chunks:
//! - eXIf: Raw EXIF data (since PNG 1.5, 2017)
//! - tEXt: Uncompressed text (keyword\0value)
//! - zTXt: Compressed text (keyword\0compression\0data)
//! - iTXt: International text with encoding info
//!
//! XMP is stored in iTXt with keyword "XML:com.adobe.xmp"

use crate::tag_lookup::{lookup_exif_subifd, lookup_gps, lookup_ifd0};
use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use exiftool_core::{ByteOrder, IfdReader};
use exiftool_xmp::XmpParser;
use flate2::read::ZlibDecoder;
use std::io::Read;

/// PNG magic signature.
const PNG_SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

/// PNG format parser.
pub struct PngParser;

impl FormatParser for PngParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 8 && header[..8] == PNG_SIGNATURE
    }

    fn format_name(&self) -> &'static str {
        "PNG"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["png"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("PNG");

        // Read entire file with size limit
        let data = crate::utils::read_with_limit(reader)?;

        if data.len() < 8 || data[..8] != PNG_SIGNATURE {
            return Err(Error::InvalidStructure("invalid PNG signature".into()));
        }

        // Parse chunks starting after signature
        let mut pos = 8;

        while pos + 12 <= data.len() {
            // Chunk: length (4) + type (4) + data + CRC (4)
            let length = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
            let chunk_type = &data[pos + 4..pos + 8];

            // Check we have enough data
            if pos + 12 + length > data.len() {
                break;
            }

            let chunk_data = &data[pos + 8..pos + 8 + length];

            match chunk_type {
                b"IHDR" => {
                    // Image header - dimensions, bit depth, color type
                    if chunk_data.len() >= 13 {
                        let width = u32::from_be_bytes([chunk_data[0], chunk_data[1], chunk_data[2], chunk_data[3]]);
                        let height = u32::from_be_bytes([chunk_data[4], chunk_data[5], chunk_data[6], chunk_data[7]]);
                        let bit_depth = chunk_data[8];
                        let color_type = chunk_data[9];
                        let compression = chunk_data[10];
                        let _filter = chunk_data[11];
                        let interlace = chunk_data[12];
                        
                        metadata.exif.set("ImageWidth", AttrValue::UInt(width));
                        metadata.exif.set("ImageHeight", AttrValue::UInt(height));
                        metadata.exif.set("BitDepth", AttrValue::UInt(bit_depth as u32));
                        
                        let color_type_str = match color_type {
                            0 => "Grayscale",
                            2 => "RGB",
                            3 => "Palette",
                            4 => "Grayscale+Alpha",
                            6 => "RGBA",
                            _ => "Unknown",
                        };
                        metadata.exif.set("ColorType", AttrValue::Str(color_type_str.into()));
                        
                        if compression == 0 {
                            metadata.exif.set("Compression", AttrValue::Str("Deflate".into()));
                        }
                        if interlace == 1 {
                            metadata.exif.set("Interlace", AttrValue::Str("Adam7".into()));
                        }
                    }
                }
                b"pHYs" => {
                    // Physical dimensions
                    if chunk_data.len() >= 9 {
                        let x_pixels = u32::from_be_bytes([chunk_data[0], chunk_data[1], chunk_data[2], chunk_data[3]]);
                        let y_pixels = u32::from_be_bytes([chunk_data[4], chunk_data[5], chunk_data[6], chunk_data[7]]);
                        let unit = chunk_data[8];
                        
                        if unit == 1 {
                            // Pixels per meter -> DPI
                            let x_dpi = (x_pixels as f64 * 0.0254).round() as u32;
                            let y_dpi = (y_pixels as f64 * 0.0254).round() as u32;
                            metadata.exif.set("XResolution", AttrValue::UInt(x_dpi));
                            metadata.exif.set("YResolution", AttrValue::UInt(y_dpi));
                            metadata.exif.set("ResolutionUnit", AttrValue::Str("dpi".into()));
                        } else {
                            metadata.exif.set("PixelAspectRatio", AttrValue::Str(format!("{}:{}", x_pixels, y_pixels)));
                        }
                    }
                }
                b"tIME" => {
                    // Last modification time
                    if chunk_data.len() >= 7 {
                        let year = u16::from_be_bytes([chunk_data[0], chunk_data[1]]);
                        let month = chunk_data[2];
                        let day = chunk_data[3];
                        let hour = chunk_data[4];
                        let minute = chunk_data[5];
                        let second = chunk_data[6];
                        metadata.exif.set("ModifyDate", AttrValue::Str(
                            format!("{:04}:{:02}:{:02} {:02}:{:02}:{:02}", year, month, day, hour, minute, second)
                        ));
                    }
                }
                b"gAMA" => {
                    // Gamma
                    if chunk_data.len() >= 4 {
                        let gamma = u32::from_be_bytes([chunk_data[0], chunk_data[1], chunk_data[2], chunk_data[3]]);
                        metadata.exif.set("Gamma", AttrValue::Float(gamma as f32 / 100000.0));
                    }
                }
                b"cHRM" => {
                    // Chromaticity
                    if chunk_data.len() >= 32 {
                        metadata.exif.set("Chromaticity", AttrValue::Str("Present".into()));
                    }
                }
                b"sRGB" => {
                    // sRGB rendering intent
                    if !chunk_data.is_empty() {
                        let intent = match chunk_data[0] {
                            0 => "Perceptual",
                            1 => "Relative Colorimetric",
                            2 => "Saturation",
                            3 => "Absolute Colorimetric",
                            _ => "Unknown",
                        };
                        metadata.exif.set("sRGBRendering", AttrValue::Str(intent.into()));
                    }
                }
                b"iCCP" => {
                    // ICC Profile
                    if let Some(null_pos) = chunk_data.iter().position(|&b| b == 0) {
                        if let Ok(name) = std::str::from_utf8(&chunk_data[..null_pos]) {
                            metadata.exif.set("ICCProfileName", AttrValue::Str(name.into()));
                        }
                        // Compression method at null_pos+1, then zlib data
                        if null_pos + 2 < chunk_data.len() {
                            let compressed = &chunk_data[null_pos + 2..];
                            let mut decoder = ZlibDecoder::new(compressed);
                            let mut profile = Vec::new();
                            if decoder.read_to_end(&mut profile).is_ok() && profile.len() >= 20 {
                                metadata.exif.set("ICCProfileSize", AttrValue::UInt(profile.len() as u32));
                                if let Ok(space) = std::str::from_utf8(&profile[16..20]) {
                                    metadata.exif.set("ICCColorSpace", AttrValue::Str(space.trim().into()));
                                }
                            }
                        }
                    }
                }
                b"eXIf" => {
                    // Raw EXIF data (PNG 1.5+)
                    self.parse_exif(chunk_data, &mut metadata)?;
                }
                b"tEXt" => {
                    // Uncompressed text: keyword\0value
                    self.parse_text(chunk_data, &mut metadata);
                }
                b"zTXt" => {
                    // Compressed text: keyword\0compression_method\0compressed_data
                    self.parse_ztxt(chunk_data, &mut metadata);
                }
                b"iTXt" => {
                    // International text with possible XMP
                    self.parse_itxt(chunk_data, &mut metadata);
                }
                b"IEND" => {
                    // End of image
                    break;
                }
                _ => {}
            }

            // Move to next chunk
            pos += 12 + length;
        }

        Ok(metadata)
    }
}

impl PngParser {
    /// Parse eXIf chunk (raw EXIF data).
    fn parse_exif(&self, data: &[u8], metadata: &mut Metadata) -> Result<()> {
        if data.len() < 8 {
            return Ok(());
        }

        // eXIf contains raw TIFF data (byte order + IFDs)
        let byte_order = ByteOrder::from_marker([data[0], data[1]]).map_err(Error::Core)?;
        let reader = IfdReader::new(data, byte_order);
        let ifd0_offset = reader.parse_header().map_err(Error::Core)?;

        // Parse IFD0
        if let Ok((entries, _)) = reader.read_ifd(ifd0_offset) {
            for entry in &entries {
                if let Some(name) = lookup_ifd0(entry.tag) {
                    metadata.exif.set(name, entry_to_attr(entry));
                }

                // Handle EXIF sub-IFD
                if entry.tag == 0x8769 {
                    if let Some(offset) = entry.value.as_u32() {
                        if let Ok((exif_entries, _)) = reader.read_ifd(offset) {
                            for e in &exif_entries {
                                if let Some(name) = lookup_exif_subifd(e.tag) {
                                    metadata.exif.set(name, entry_to_attr(e));
                                }
                            }
                        }
                    }
                }

                // Handle GPS sub-IFD
                if entry.tag == 0x8825 {
                    if let Some(offset) = entry.value.as_u32() {
                        if let Ok((gps_entries, _)) = reader.read_ifd(offset) {
                            for e in &gps_entries {
                                if let Some(name) = lookup_gps(e.tag) {
                                    metadata.exif.set(name, entry_to_attr(e));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Parse tEXt chunk (uncompressed).
    fn parse_text(&self, data: &[u8], metadata: &mut Metadata) {
        // Format: keyword\0value
        if let Some(null_pos) = data.iter().position(|&b| b == 0) {
            let keyword = String::from_utf8_lossy(&data[..null_pos]).to_string();
            let value = String::from_utf8_lossy(&data[null_pos + 1..]).to_string();

            if !keyword.is_empty() && !value.is_empty() {
                let key = format!("PNG:{}", keyword);
                metadata.exif.set(key, AttrValue::Str(value));
            }
        }
    }

    /// Parse zTXt chunk (compressed).
    fn parse_ztxt(&self, data: &[u8], metadata: &mut Metadata) {
        // Format: keyword\0compression_method\0compressed_data
        if let Some(null_pos) = data.iter().position(|&b| b == 0) {
            let keyword = String::from_utf8_lossy(&data[..null_pos]).to_string();

            if null_pos + 2 <= data.len() {
                let compression_method = data[null_pos + 1];
                let compressed_data = &data[null_pos + 2..];

                // Only zlib (method 0) is defined
                if compression_method == 0 && !compressed_data.is_empty() {
                    // Decompress using flate2
                    if let Some(text) = decompress_zlib(compressed_data) {
                        let key = format!("PNG:{}", keyword);
                        metadata.exif.set(key, AttrValue::Str(text));
                    }
                }
            }
        }
    }

    /// Parse iTXt chunk (international text).
    fn parse_itxt(&self, data: &[u8], metadata: &mut Metadata) {
        // Format: keyword\0compression_flag\0compression_method\0language\0translated_keyword\0text
        if let Some(null_pos) = data.iter().position(|&b| b == 0) {
            let keyword = String::from_utf8_lossy(&data[..null_pos]).to_string();

            // Check for XMP
            if keyword == "XML:com.adobe.xmp" {
                // Skip header fields to get to actual text
                let mut pos = null_pos + 1;

                if pos + 2 <= data.len() {
                    let compression_flag = data[pos];
                    pos += 2; // Skip compression flag and method

                    // Skip language tag (null-terminated)
                    if let Some(lang_end) = data[pos..].iter().position(|&b| b == 0) {
                        pos += lang_end + 1;

                        // Skip translated keyword (null-terminated)
                        if let Some(tk_end) = data[pos..].iter().position(|&b| b == 0) {
                            pos += tk_end + 1;

                            // Rest is text/XMP
                            let text_data = &data[pos..];

                            if compression_flag == 0 {
                                // Uncompressed
                                if let Ok(xmp) = String::from_utf8(text_data.to_vec()) {
                                    // Parse XMP and add tags to metadata
                                    if let Ok(xmp_attrs) = XmpParser::parse(&xmp) {
                                        for (key, value) in xmp_attrs.iter() {
                                            metadata.exif.set(format!("XMP:{}", key), value.clone());
                                        }
                                    }
                                    metadata.xmp = Some(xmp);
                                }
                            } else {
                                // Compressed XMP - decompress with zlib
                                if let Some(xmp) = decompress_zlib(text_data) {
                                    if let Ok(xmp_attrs) = XmpParser::parse(&xmp) {
                                        for (key, value) in xmp_attrs.iter() {
                                            metadata.exif.set(format!("XMP:{}", key), value.clone());
                                        }
                                    }
                                    metadata.xmp = Some(xmp);
                                }
                            }
                        }
                    }
                }
            } else {
                // Regular iTXt - extract as text
                // Simplified: just get keyword for now
                let key = format!("PNG:{}", keyword);
                metadata.exif.set(key, AttrValue::Str("<iTXt data>".into()));
            }
        }
    }
}

/// Decompress zlib data to string.
fn decompress_zlib(data: &[u8]) -> Option<String> {
    let decoder = ZlibDecoder::new(data);
    let mut decompressed = Vec::new();
    
    // Limit decompression to 10MB to prevent zip bombs
    let mut limited = decoder.take(10 * 1024 * 1024);
    
    if limited.read_to_end(&mut decompressed).is_err() {
        return None;
    }
    
    String::from_utf8(decompressed).ok()
}

// Use shared entry_to_attr from crate::utils
use crate::utils::entry_to_attr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_png() {
        let parser = PngParser;
        assert!(parser.can_parse(&PNG_SIGNATURE));
        assert!(parser.can_parse(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00]));
    }

    #[test]
    fn reject_jpeg() {
        let parser = PngParser;
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
    }

    #[test]
    fn reject_tiff() {
        let parser = PngParser;
        assert!(!parser.can_parse(&[b'I', b'I', 0x2A, 0x00]));
    }
}

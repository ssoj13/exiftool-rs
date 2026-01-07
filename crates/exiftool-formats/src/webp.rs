//! WebP format parser.
//!
//! WebP is Google's modern image format supporting lossy, lossless, and animation.
//! Structure: RIFF container with VP8/VP8L/VP8X chunks.
//!
//! Metadata locations:
//! - EXIF chunk: Contains standard EXIF data (TIFF structure)
//! - XMP chunk: Contains XMP metadata
//! - ICCP chunk: ICC color profile
//!
//! File structure:
//! ```text
//! RIFF <size> WEBP
//!   VP8X <size> <flags> <canvas>     ; Extended header (optional)
//!   EXIF <size> <exif-data>          ; EXIF metadata (optional)
//!   XMP  <size> <xmp-data>           ; XMP metadata (optional)
//!   VP8  <size> <lossy-data>         ; Lossy image data
//!   VP8L <size> <lossless-data>      ; Lossless image data
//! ```

use crate::{Error, FormatParser, Metadata, ReadSeek, Result, utils::entry_to_attr, tag_lookup};
use exiftool_attrs::{Attrs, AttrValue};
use exiftool_core::{ByteOrder, IfdReader};
use std::io::SeekFrom;

/// WebP format parser.
pub struct WebpParser;

impl WebpParser {
    pub fn new() -> Self {
        Self
    }

    /// Read a 4-byte chunk ID.
    fn read_fourcc(reader: &mut dyn ReadSeek) -> Result<[u8; 4]> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Read a 32-bit little-endian size.
    fn read_u32_le(reader: &mut dyn ReadSeek) -> Result<u32> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    /// Parse EXIF chunk data (TIFF structure).
    fn parse_exif_chunk(&self, data: &[u8]) -> Result<Attrs> {
        if data.len() < 8 {
            return Ok(Attrs::new());
        }

        // EXIF data starts with TIFF header
        let byte_order = ByteOrder::from_marker([data[0], data[1]])
            .map_err(|_| Error::InvalidStructure("Invalid EXIF byte order".into()))?;

        let reader = IfdReader::new(data, byte_order);
        let ifd_offset = reader.parse_header()
            .map_err(|e| Error::InvalidStructure(format!("EXIF header: {}", e)))?;

        let (entries, _) = reader.read_ifd(ifd_offset)
            .map_err(|e| Error::InvalidStructure(format!("EXIF IFD: {}", e)))?;

        let mut attrs = Attrs::new();
        for entry in entries {
            let name = tag_lookup::lookup_exif(entry.tag)
                .unwrap_or("Unknown")
                .to_string();
            attrs.set(&name, entry_to_attr(&entry));
        }

        Ok(attrs)
    }
}

impl Default for WebpParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for WebpParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // RIFF....WEBP signature
        header.len() >= 12
            && &header[0..4] == b"RIFF"
            && &header[8..12] == b"WEBP"
    }

    fn format_name(&self) -> &'static str {
        "WebP"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["webp"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        // Read RIFF header
        let riff = Self::read_fourcc(reader)?;
        if &riff != b"RIFF" {
            return Err(Error::InvalidStructure("Not a RIFF file".into()));
        }

        let file_size = Self::read_u32_le(reader)?;
        let _ = file_size;  // Total file size minus 8

        let webp = Self::read_fourcc(reader)?;
        if &webp != b"WEBP" {
            return Err(Error::InvalidStructure("Not a WebP file".into()));
        }

        let mut attrs = Attrs::new();
        let mut xmp_data: Option<String> = None;
        let mut _has_exif = false;
        let mut _has_xmp = false;
        let mut _has_icc = false;
        let mut width: u32 = 0;
        #[allow(unused_assignments)]
        let mut height: u32 = 0;

        // Parse chunks
        loop {
            let chunk_id = match Self::read_fourcc(reader) {
                Ok(id) => id,
                Err(_) => break,  // EOF
            };

            let chunk_size = match Self::read_u32_le(reader) {
                Ok(s) => s,
                Err(_) => break,
            };

            // Chunk sizes are padded to even bytes
            let padded_size = (chunk_size + 1) & !1;

            match &chunk_id {
                b"VP8X" => {
                    // Extended WebP header
                    if chunk_size >= 10 {
                        let mut data = vec![0u8; chunk_size as usize];
                        reader.read_exact(&mut data)?;

                        let flags = data[0];
                        _has_icc = (flags & 0x20) != 0;
                        _has_exif = (flags & 0x08) != 0;
                        _has_xmp = (flags & 0x04) != 0;

                        // Canvas size (24-bit values)
                        width = u32::from_le_bytes([data[4], data[5], data[6], 0]) + 1;
                        height = u32::from_le_bytes([data[7], data[8], data[9], 0]) + 1;

                        attrs.set("ImageWidth", AttrValue::UInt(width));
                        attrs.set("ImageHeight", AttrValue::UInt(height));

                        // Skip padding if any
                        if padded_size > chunk_size {
                            reader.seek(SeekFrom::Current((padded_size - chunk_size) as i64))?;
                        }
                    } else {
                        reader.seek(SeekFrom::Current(padded_size as i64))?;
                    }
                }

                b"VP8 " => {
                    // Lossy WebP - extract dimensions from VP8 header
                    if chunk_size >= 10 && width == 0 {
                        let mut data = vec![0u8; 10.min(chunk_size as usize)];
                        reader.read_exact(&mut data)?;

                        // VP8 frame header: 3 bytes frame tag + 3 bytes start code + dimensions
                        if data.len() >= 10 && data[3] == 0x9D && data[4] == 0x01 && data[5] == 0x2A {
                            width = u16::from_le_bytes([data[6], data[7]]) as u32 & 0x3FFF;
                            height = u16::from_le_bytes([data[8], data[9]]) as u32 & 0x3FFF;
                            attrs.set("ImageWidth", AttrValue::UInt(width));
                            attrs.set("ImageHeight", AttrValue::UInt(height));
                        }

                        // Skip rest of chunk
                        let remaining = padded_size as usize - data.len();
                        if remaining > 0 {
                            reader.seek(SeekFrom::Current(remaining as i64))?;
                        }
                    } else {
                        reader.seek(SeekFrom::Current(padded_size as i64))?;
                    }
                }

                b"VP8L" => {
                    // Lossless WebP - extract dimensions
                    if chunk_size >= 5 && width == 0 {
                        let mut data = vec![0u8; 5];
                        reader.read_exact(&mut data)?;

                        // VP8L signature + packed dimensions
                        if data[0] == 0x2F {
                            let bits = u32::from_le_bytes([data[1], data[2], data[3], data[4]]);
                            width = (bits & 0x3FFF) + 1;
                            height = ((bits >> 14) & 0x3FFF) + 1;
                            attrs.set("ImageWidth", AttrValue::UInt(width));
                            attrs.set("ImageHeight", AttrValue::UInt(height));
                        }

                        let remaining = padded_size as usize - 5;
                        if remaining > 0 {
                            reader.seek(SeekFrom::Current(remaining as i64))?;
                        }
                    } else {
                        reader.seek(SeekFrom::Current(padded_size as i64))?;
                    }
                }

                b"EXIF" => {
                    // EXIF metadata chunk
                    let mut data = vec![0u8; chunk_size as usize];
                    reader.read_exact(&mut data)?;

                    // Skip "Exif\0\0" prefix if present
                    let exif_data = if data.starts_with(b"Exif\x00\x00") {
                        &data[6..]
                    } else {
                        &data
                    };

                    if let Ok(exif_attrs) = self.parse_exif_chunk(exif_data) {
                        for (key, value) in exif_attrs.iter() {
                            attrs.set(key.clone(), value.clone());
                        }
                    }

                    if padded_size > chunk_size {
                        reader.seek(SeekFrom::Current((padded_size - chunk_size) as i64))?;
                    }
                }

                b"XMP " => {
                    // XMP metadata chunk
                    let mut data = vec![0u8; chunk_size as usize];
                    reader.read_exact(&mut data)?;

                    xmp_data = String::from_utf8(data).ok();

                    if padded_size > chunk_size {
                        reader.seek(SeekFrom::Current((padded_size - chunk_size) as i64))?;
                    }
                }

                b"ICCP" => {
                    // ICC color profile - skip for now
                    reader.seek(SeekFrom::Current(padded_size as i64))?;
                }

                _ => {
                    // Unknown chunk - skip
                    reader.seek(SeekFrom::Current(padded_size as i64))?;
                }
            }
        }

        Ok(Metadata {
            format: "WebP",
            exif: attrs,
            xmp: xmp_data,
            thumbnail: None,
            preview: None,
            exif_offset: None,
            pages: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_info() {
        let parser = WebpParser::new();
        assert_eq!(parser.format_name(), "WebP");
        assert!(parser.extensions().contains(&"webp"));
    }

    #[test]
    fn detect_webp() {
        let parser = WebpParser::new();
        // Valid WebP header
        assert!(parser.can_parse(b"RIFF\x00\x00\x00\x00WEBP"));
        // Invalid
        assert!(!parser.can_parse(b"RIFF\x00\x00\x00\x00WAVE"));
        assert!(!parser.can_parse(b"PNG\r\n\x1a\n"));
    }
}

//! OpenEXR format parser.
//!
//! EXR (Extended Dynamic Range) is ILM's HDR image format.
//! Metadata is stored in header attributes:
//! - Standard: channels, compression, displayWindow, pixelAspectRatio
//! - Optional: chromaticities, owner, comments, capDate, etc.
//!
//! Uses the `exr` crate for parsing.

use crate::{Error, FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;

/// EXR magic signature (4 bytes): 0x76, 0x2F, 0x31, 0x01
const EXR_MAGIC: [u8; 4] = [0x76, 0x2F, 0x31, 0x01];

/// OpenEXR format parser.
pub struct ExrParser;

impl FormatParser for ExrParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 4 && header[..4] == EXR_MAGIC
    }

    fn format_name(&self) -> &'static str {
        "EXR"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["exr"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("EXR");

        // Read entire file for exr (with size limit)
        let data = crate::utils::read_with_limit(reader)?;

        // Parse with exr crate - read only meta (no pixel data)
        let meta = exr::meta::MetaData::read_from_buffered(
            std::io::Cursor::new(&data),
            false, // not pedantic
        )
        .map_err(|e| Error::InvalidStructure(format!("EXR parse error: {}", e)))?;

        // Extract header attributes from each layer
        for (layer_idx, header) in meta.headers.iter().enumerate() {
            let prefix = if meta.headers.len() > 1 {
                format!("Layer{}:", layer_idx)
            } else {
                String::new()
            };

            // Image dimensions from display window
            let display_window = &header.shared_attributes.display_window;
            metadata.exif.set(
                format!("{}ImageWidth", prefix),
                AttrValue::UInt(display_window.size.width() as u32),
            );
            metadata.exif.set(
                format!("{}ImageHeight", prefix),
                AttrValue::UInt(display_window.size.height() as u32),
            );

            // Pixel aspect ratio
            metadata.exif.set(
                format!("{}PixelAspectRatio", prefix),
                AttrValue::Float(header.shared_attributes.pixel_aspect),
            );

            // Compression type
            let compression = format!("{:?}", header.compression);
            metadata.exif.set(
                format!("{}Compression", prefix),
                AttrValue::Str(compression),
            );

            // Channel info
            let channels: Vec<String> = header
                .channels
                .list
                .iter()
                .map(|ch| ch.name.to_string())
                .collect();
            metadata.exif.set(
                format!("{}Channels", prefix),
                AttrValue::Str(channels.join(", ")),
            );
            
            // Channel count
            metadata.exif.set(
                format!("{}ChannelCount", prefix),
                AttrValue::UInt(header.channels.list.len() as u32),
            );

            // Line order (from header, not shared_attributes)
            let line_order = format!("{:?}", header.line_order);
            metadata.exif.set(
                format!("{}LineOrder", prefix),
                AttrValue::Str(line_order),
            );

            // Chromaticities if present
            if let Some(chroma) = &header.shared_attributes.chromaticities {
                metadata.exif.set(
                    format!("{}Chromaticities", prefix),
                    AttrValue::Str(format!(
                        "R({:.3},{:.3}) G({:.3},{:.3}) B({:.3},{:.3}) W({:.3},{:.3})",
                        chroma.red.0, chroma.red.1,
                        chroma.green.0, chroma.green.1,
                        chroma.blue.0, chroma.blue.1,
                        chroma.white.0, chroma.white.1
                    )),
                );
            }

            // Time code if present
            if let Some(tc) = &header.shared_attributes.time_code {
                metadata.exif.set(
                    format!("{}TimeCode", prefix),
                    AttrValue::Str(format!(
                        "{:02}:{:02}:{:02}:{:02}",
                        tc.hours, tc.minutes, tc.seconds, tc.frame
                    )),
                );
            }

            // Other custom attributes
            for (name, value) in &header.shared_attributes.other {
                metadata.exif.set(
                    format!("{}EXR:{}", prefix, name),
                    AttrValue::Str(format!("{:?}", value)),
                );
            }

            // Layer-specific attributes
            if let Some(layer_name) = &header.own_attributes.layer_name {
                metadata.exif.set(
                    format!("{}LayerName", prefix),
                    AttrValue::Str(layer_name.to_string()),
                );
            }

            // Other layer attributes
            for (name, value) in &header.own_attributes.other {
                metadata.exif.set(
                    format!("{}Layer:{}", prefix, name),
                    AttrValue::Str(format!("{:?}", value)),
                );
            }
        }

        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_exr() {
        let parser = ExrParser;
        assert!(parser.can_parse(&EXR_MAGIC));
        assert!(parser.can_parse(&[0x76, 0x2F, 0x31, 0x01, 0x02, 0x00]));
    }

    #[test]
    fn reject_jpeg() {
        let parser = ExrParser;
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
    }

    #[test]
    fn reject_png() {
        let parser = ExrParser;
        assert!(!parser.can_parse(&[0x89, b'P', b'N', b'G']));
    }
}

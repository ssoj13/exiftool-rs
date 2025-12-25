//! HDR (Radiance RGBE) format writer.
//!
//! HDR writing strategy:
//! - Parse header from source
//! - Update/add metadata key=value lines
//! - Copy resolution line and pixel data verbatim

use crate::{Error, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::Write;

/// HDR format writer.
pub struct HdrWriter;

impl HdrWriter {
    /// Write HDR with updated metadata.
    ///
    /// Preserves pixel data, updates header attributes.
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        // Read source (with size limit)
        let data = crate::utils::read_with_limit(input)?;

        // Find header end (empty line before resolution)
        let mut resolution_start = 0;
        let mut i = 0;
        
        while i < data.len() {
            // Find line end
            let line_start = i;
            while i < data.len() && data[i] != b'\n' {
                i += 1;
            }
            let line = &data[line_start..i];
            i += 1; // skip \n

            // Empty line marks header end
            if line.is_empty() || (line.len() == 1 && line[0] == b'\r') {
                resolution_start = i;
                break;
            }

            // Resolution line (starts with +/-)
            if !line.is_empty() && (line[0] == b'+' || line[0] == b'-') {
                resolution_start = line_start;
                break;
            }
        }

        if resolution_start == 0 {
            return Err(Error::InvalidStructure("HDR: can't find header end".into()));
        }

        // Write magic
        let format_id = metadata.exif.get_str("FormatIdentifier").unwrap_or("RADIANCE");
        writeln!(output, "#?{}", format_id)?;

        // Write metadata as key=value pairs
        Self::write_header_field(output, "FORMAT", 
            metadata.exif.get_str("Format").unwrap_or("32-bit_rle_rgbe"))?;

        if let Some(v) = metadata.exif.get_str("Software") {
            Self::write_header_field(output, "SOFTWARE", v)?;
        }
        if let Some(AttrValue::Float(exp)) = metadata.exif.get("Exposure") {
            Self::write_header_field(output, "EXPOSURE", &format!("{}", exp))?;
        }
        if let Some(AttrValue::Float(gamma)) = metadata.exif.get("Gamma") {
            Self::write_header_field(output, "GAMMA", &format!("{}", gamma))?;
        }
        if let Some(AttrValue::Float(aspect)) = metadata.exif.get("PixelAspectRatio") {
            Self::write_header_field(output, "PIXASPECT", &format!("{}", aspect))?;
        }
        if let Some(v) = metadata.exif.get_str("Primaries") {
            Self::write_header_field(output, "PRIMARIES", v)?;
        }
        if let Some(v) = metadata.exif.get_str("View") {
            Self::write_header_field(output, "VIEW", v)?;
        }

        // Write any HDR: prefixed custom attrs
        for (key, value) in metadata.exif.iter() {
            if let Some(hdr_key) = key.strip_prefix("HDR:") {
                // strip "HDR:" prefix
                if let AttrValue::Str(v) = value {
                    Self::write_header_field(output, hdr_key, v)?;
                }
            }
        }

        // Empty line separator
        writeln!(output)?;

        // Copy resolution line and pixel data from source
        output.write_all(&data[resolution_start..])?;

        Ok(())
    }

    /// Write a header field.
    fn write_header_field<W: Write>(output: &mut W, key: &str, value: &str) -> Result<()> {
        writeln!(output, "{}={}", key, value)?;
        Ok(())
    }

    /// Write new HDR file with metadata (no pixel data).
    pub fn write_header_only<W: Write>(
        output: &mut W,
        metadata: &Metadata,
        width: u32,
        height: u32,
    ) -> Result<()> {
        let format_id = metadata.exif.get_str("FormatIdentifier").unwrap_or("RADIANCE");
        writeln!(output, "#?{}", format_id)?;

        Self::write_header_field(output, "FORMAT", "32-bit_rle_rgbe")?;

        if let Some(v) = metadata.exif.get_str("Software") {
            Self::write_header_field(output, "SOFTWARE", v)?;
        }
        if let Some(AttrValue::Float(exp)) = metadata.exif.get("Exposure") {
            Self::write_header_field(output, "EXPOSURE", &format!("{}", exp))?;
        }

        // Empty line
        writeln!(output)?;

        // Resolution line
        writeln!(output, "-Y {} +X {}", height, width)?;

        // Note: No pixel data written - this is header-only

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_minimal_hdr() -> Vec<u8> {
        let mut hdr = Vec::new();
        hdr.extend_from_slice(b"#?RADIANCE\n");
        hdr.extend_from_slice(b"FORMAT=32-bit_rle_rgbe\n");
        hdr.extend_from_slice(b"SOFTWARE=test\n");
        hdr.extend_from_slice(b"\n");
        hdr.extend_from_slice(b"-Y 2 +X 2\n");
        // Minimal pixel data (2x2, each pixel = 4 bytes RGBE)
        hdr.extend_from_slice(&[0u8; 16]); // 4 black pixels (4 bytes each RGBE)
        hdr
    }

    #[test]
    fn write_hdr_preserves_pixels() {
        let hdr = make_minimal_hdr();
        
        let mut metadata = Metadata::new("HDR");
        metadata.exif.set("Software", AttrValue::Str("exiftool-rs".into()));
        metadata.exif.set("Exposure", AttrValue::Float(1.5));

        let mut input = Cursor::new(&hdr);
        let mut output = Vec::new();

        HdrWriter::write(&mut input, &mut output, &metadata).unwrap();

        let output_str = String::from_utf8_lossy(&output);
        
        // Check header
        assert!(output_str.starts_with("#?RADIANCE\n"));
        assert!(output_str.contains("SOFTWARE=exiftool-rs"));
        assert!(output_str.contains("EXPOSURE=1.5"));
        assert!(output_str.contains("-Y 2 +X 2"));
    }

    #[test]
    fn write_header_only() {
        let mut metadata = Metadata::new("HDR");
        metadata.exif.set("Software", AttrValue::Str("test-app".into()));

        let mut output = Vec::new();
        HdrWriter::write_header_only(&mut output, &metadata, 100, 50).unwrap();

        let output_str = String::from_utf8_lossy(&output);
        assert!(output_str.contains("#?RADIANCE"));
        assert!(output_str.contains("SOFTWARE=test-app"));
        assert!(output_str.contains("-Y 50 +X 100"));
    }
}

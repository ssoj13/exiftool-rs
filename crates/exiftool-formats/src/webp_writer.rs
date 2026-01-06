//! WebP format writer.
//!
//! WebP writing strategy (based on ExifTool WriteRIFF.pl):
//! - Two-pass algorithm: first calculate sizes, then write
//! - Parse RIFF chunks from source
//! - Update VP8X flags when adding/removing metadata
//! - Replace/add EXIF, XMP, ICCP chunks
//! - Recalculate RIFF file size
//! - Preserve image data chunks (VP8/VP8L/ANIM/ANMF/ALPH)
//!
//! Chunk order (per WebP spec):
//! VP8X (if extended) -> ICCP -> ANIM -> ANMF* -> ALPH -> VP8/VP8L -> EXIF -> XMP
//!
//! Reference: https://developers.google.com/speed/webp/docs/riff_container

use crate::{Error, Metadata, ReadSeek, Result};
use std::io::Write;

/// VP8X flags
const VP8X_FLAG_ANIM: u8 = 0x02;
const VP8X_FLAG_XMP: u8 = 0x04;
const VP8X_FLAG_EXIF: u8 = 0x08;
const VP8X_FLAG_ALPH: u8 = 0x10;
const VP8X_FLAG_ICCP: u8 = 0x20;

/// Parsed chunk info
#[derive(Clone)]
struct Chunk {
    id: [u8; 4],
    data: Vec<u8>,
}

/// WebP format writer.
pub struct WebpWriter;

impl WebpWriter {
    /// Write WebP with updated metadata.
    ///
    /// Supports writing:
    /// - EXIF metadata (from metadata.exif)
    /// - XMP metadata (from metadata.xmp)
    /// - ICC profile preservation
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        let data = crate::utils::read_with_limit(input)?;

        if data.len() < 12 || &data[0..4] != b"RIFF" || &data[8..12] != b"WEBP" {
            return Err(Error::InvalidStructure("Invalid WebP file".into()));
        }

        // Build new EXIF bytes
        let exif_bytes = crate::utils::build_exif_bytes(metadata)?;
        let has_new_exif = !exif_bytes.is_empty();

        // Build XMP bytes
        let xmp_bytes = metadata.xmp.as_ref().map(|s| s.as_bytes().to_vec());
        let has_new_xmp = xmp_bytes.as_ref().map(|v| !v.is_empty()).unwrap_or(false);

        // Parse existing chunks
        let mut chunks: Vec<Chunk> = Vec::new();
        let mut pos = 12;
        let mut has_vp8x = false;
        let mut _vp8x_flags: u8 = 0;
        let mut canvas_width: u32 = 0;
        let mut canvas_height: u32 = 0;
        let mut has_alpha_in_vp8l = false;

        while pos + 8 <= data.len() {
            let chunk_id: [u8; 4] = [data[pos], data[pos + 1], data[pos + 2], data[pos + 3]];
            let chunk_size =
                u32::from_le_bytes([data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]])
                    as usize;
            let padded_size = (chunk_size + 1) & !1;

            if pos + 8 + padded_size > data.len() {
                break;
            }

            let chunk_data = data[pos + 8..pos + 8 + chunk_size].to_vec();

            match &chunk_id {
                b"VP8X" => {
                    has_vp8x = true;
                    if chunk_data.len() >= 10 {
                        _vp8x_flags = chunk_data[0];
                        canvas_width =
                            u32::from_le_bytes([chunk_data[4], chunk_data[5], chunk_data[6], 0])
                                + 1;
                        canvas_height =
                            u32::from_le_bytes([chunk_data[7], chunk_data[8], chunk_data[9], 0])
                                + 1;
                    }
                    // Don't add VP8X to chunks - we'll recreate it if needed
                }
                b"EXIF" => {
                    // Skip old EXIF - we'll add new one
                }
                b"XMP " | b"XMP\0" => {
                    // Skip old XMP (including incorrect "XMP\0" from some software)
                }
                b"VP8 " => {
                    // Lossy VP8 - extract dimensions if no VP8X
                    if canvas_width == 0 && chunk_data.len() >= 10
                        && chunk_data[3] == 0x9D && chunk_data[4] == 0x01 && chunk_data[5] == 0x2A {
                            canvas_width =
                                (u16::from_le_bytes([chunk_data[6], chunk_data[7]]) & 0x3FFF) as u32;
                            canvas_height =
                                (u16::from_le_bytes([chunk_data[8], chunk_data[9]]) & 0x3FFF) as u32;
                        }
                    chunks.push(Chunk {
                        id: chunk_id,
                        data: chunk_data,
                    });
                }
                b"VP8L" => {
                    // Lossless VP8L - extract dimensions if no VP8X
                    if canvas_width == 0 && chunk_data.len() >= 5
                        && chunk_data[0] == 0x2F {
                            let bits = u32::from_le_bytes([
                                chunk_data[1],
                                chunk_data[2],
                                chunk_data[3],
                                chunk_data[4],
                            ]);
                            canvas_width = (bits & 0x3FFF) + 1;
                            canvas_height = ((bits >> 14) & 0x3FFF) + 1;
                            // Check alpha flag in VP8L
                            has_alpha_in_vp8l = (bits & 0x100000) != 0;
                        }
                    chunks.push(Chunk {
                        id: chunk_id,
                        data: chunk_data,
                    });
                }
                _ => {
                    chunks.push(Chunk {
                        id: chunk_id,
                        data: chunk_data,
                    });
                }
            }

            pos += 8 + padded_size;
        }

        // Determine which chunks exist
        let has_iccp = chunks.iter().any(|c| &c.id == b"ICCP");
        let has_anim = chunks.iter().any(|c| &c.id == b"ANIM");
        let has_alph = chunks.iter().any(|c| &c.id == b"ALPH") || has_alpha_in_vp8l;

        // Determine if we need VP8X (extended format)
        let needs_vp8x = has_iccp || has_new_xmp || has_anim || has_alph || has_new_exif;

        // If we had VP8X but don't need it anymore, we can make a simple WebP
        let delete_vp8x = has_vp8x && !needs_vp8x;

        // Build new VP8X flags
        let new_vp8x_flags = if needs_vp8x {
            let mut flags: u8 = 0;
            if has_anim {
                flags |= VP8X_FLAG_ANIM;
            }
            if has_new_xmp {
                flags |= VP8X_FLAG_XMP;
            }
            if has_new_exif {
                flags |= VP8X_FLAG_EXIF;
            }
            if has_alph {
                flags |= VP8X_FLAG_ALPH;
            }
            if has_iccp {
                flags |= VP8X_FLAG_ICCP;
            }
            flags
        } else {
            0
        };

        // Calculate total file size (RIFF size = everything after "RIFF" + size field)
        let mut file_size: u32 = 4; // "WEBP"

        if needs_vp8x {
            file_size += 8 + 10; // VP8X chunk (header + 10 bytes data)
        }

        // ICCP comes right after VP8X
        for chunk in &chunks {
            if &chunk.id == b"ICCP" {
                file_size += 8 + ((chunk.data.len() as u32 + 1) & !1);
            }
        }

        // ANIM/ANMF chunks
        for chunk in &chunks {
            if &chunk.id == b"ANIM" || &chunk.id == b"ANMF" {
                file_size += 8 + ((chunk.data.len() as u32 + 1) & !1);
            }
        }

        // ALPH chunk
        for chunk in &chunks {
            if &chunk.id == b"ALPH" {
                file_size += 8 + ((chunk.data.len() as u32 + 1) & !1);
            }
        }

        // VP8/VP8L image data
        for chunk in &chunks {
            if &chunk.id == b"VP8 " || &chunk.id == b"VP8L" {
                file_size += 8 + ((chunk.data.len() as u32 + 1) & !1);
            }
        }

        // EXIF chunk
        if has_new_exif {
            file_size += 8 + ((exif_bytes.len() as u32 + 1) & !1);
        }

        // XMP chunk
        if let Some(ref xmp) = xmp_bytes {
            if !xmp.is_empty() {
                file_size += 8 + ((xmp.len() as u32 + 1) & !1);
            }
        }

        // Other chunks (excluding already counted ones)
        for chunk in &chunks {
            if &chunk.id != b"ICCP"
                && &chunk.id != b"ANIM"
                && &chunk.id != b"ANMF"
                && &chunk.id != b"ALPH"
                && &chunk.id != b"VP8 "
                && &chunk.id != b"VP8L"
            {
                file_size += 8 + ((chunk.data.len() as u32 + 1) & !1);
            }
        }

        // Write RIFF header
        output.write_all(b"RIFF")?;
        output.write_all(&file_size.to_le_bytes())?;
        output.write_all(b"WEBP")?;

        // Write VP8X if needed
        if needs_vp8x {
            let mut vp8x_data = vec![0u8; 10];
            vp8x_data[0] = new_vp8x_flags;
            // Canvas width - 1 (24-bit LE)
            let w = canvas_width.saturating_sub(1);
            vp8x_data[4] = (w & 0xFF) as u8;
            vp8x_data[5] = ((w >> 8) & 0xFF) as u8;
            vp8x_data[6] = ((w >> 16) & 0xFF) as u8;
            // Canvas height - 1 (24-bit LE)
            let h = canvas_height.saturating_sub(1);
            vp8x_data[7] = (h & 0xFF) as u8;
            vp8x_data[8] = ((h >> 8) & 0xFF) as u8;
            vp8x_data[9] = ((h >> 16) & 0xFF) as u8;

            Self::write_chunk(output, b"VP8X", &vp8x_data)?;
        }

        // Write ICCP right after VP8X
        for chunk in &chunks {
            if &chunk.id == b"ICCP" {
                Self::write_chunk(output, &chunk.id, &chunk.data)?;
            }
        }

        // Write ANIM/ANMF
        for chunk in &chunks {
            if &chunk.id == b"ANIM" || &chunk.id == b"ANMF" {
                Self::write_chunk(output, &chunk.id, &chunk.data)?;
            }
        }

        // Write ALPH
        for chunk in &chunks {
            if &chunk.id == b"ALPH" {
                Self::write_chunk(output, &chunk.id, &chunk.data)?;
            }
        }

        // Write VP8/VP8L image data
        for chunk in &chunks {
            if &chunk.id == b"VP8 " || &chunk.id == b"VP8L" {
                Self::write_chunk(output, &chunk.id, &chunk.data)?;
            }
        }

        // Write EXIF
        if has_new_exif {
            Self::write_chunk(output, b"EXIF", &exif_bytes)?;
        }

        // Write XMP (use correct "XMP " tag, not "XMP\0")
        if let Some(ref xmp) = xmp_bytes {
            if !xmp.is_empty() {
                Self::write_chunk(output, b"XMP ", xmp)?;
            }
        }

        // Write other chunks (excluding metadata and image data)
        for chunk in &chunks {
            if &chunk.id != b"ICCP"
                && &chunk.id != b"ANIM"
                && &chunk.id != b"ANMF"
                && &chunk.id != b"ALPH"
                && &chunk.id != b"VP8 "
                && &chunk.id != b"VP8L"
            {
                Self::write_chunk(output, &chunk.id, &chunk.data)?;
            }
        }

        let _ = delete_vp8x; // suppress warning

        Ok(())
    }

    /// Write a RIFF chunk (little-endian size, padded to even).
    fn write_chunk<W: Write>(output: &mut W, chunk_id: &[u8; 4], data: &[u8]) -> Result<()> {
        output.write_all(chunk_id)?;
        output.write_all(&(data.len() as u32).to_le_bytes())?;
        output.write_all(data)?;

        // Pad to even byte boundary
        if !data.len().is_multiple_of(2) {
            output.write_all(&[0])?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use exiftool_attrs::AttrValue;
    use std::io::Cursor;

    fn make_minimal_webp() -> Vec<u8> {
        let mut webp = Vec::new();

        // RIFF header (size will be updated)
        webp.extend_from_slice(b"RIFF");
        webp.extend_from_slice(&0u32.to_le_bytes()); // placeholder
        webp.extend_from_slice(b"WEBP");

        // Minimal VP8L chunk (1x1 lossless)
        let vp8l_data: Vec<u8> = vec![
            0x2F, // signature
            0x00, 0x00, 0x00, 0x00, // 1x1 dimensions packed
            0x00, // additional data
        ];

        webp.extend_from_slice(b"VP8L");
        webp.extend_from_slice(&(vp8l_data.len() as u32).to_le_bytes());
        webp.extend_from_slice(&vp8l_data);
        if vp8l_data.len() % 2 != 0 {
            webp.push(0);
        }

        // Update RIFF size
        let riff_size = (webp.len() - 8) as u32;
        webp[4..8].copy_from_slice(&riff_size.to_le_bytes());

        webp
    }

    fn make_extended_webp() -> Vec<u8> {
        let mut webp = Vec::new();

        webp.extend_from_slice(b"RIFF");
        webp.extend_from_slice(&0u32.to_le_bytes());
        webp.extend_from_slice(b"WEBP");

        // VP8X chunk with ICCP flag
        let vp8x = vec![
            VP8X_FLAG_ICCP, 0, 0, 0, // flags + reserved
            0, 0, 0, // width-1 (24-bit) = 0 -> 1px
            0, 0, 0, // height-1 (24-bit) = 0 -> 1px
        ];
        webp.extend_from_slice(b"VP8X");
        webp.extend_from_slice(&(vp8x.len() as u32).to_le_bytes());
        webp.extend_from_slice(&vp8x);

        // ICCP chunk (minimal profile)
        let iccp = vec![0u8; 16];
        webp.extend_from_slice(b"ICCP");
        webp.extend_from_slice(&(iccp.len() as u32).to_le_bytes());
        webp.extend_from_slice(&iccp);

        // VP8L
        let vp8l = vec![0x2F, 0, 0, 0, 0, 0];
        webp.extend_from_slice(b"VP8L");
        webp.extend_from_slice(&(vp8l.len() as u32).to_le_bytes());
        webp.extend_from_slice(&vp8l);

        // Update RIFF size
        let riff_size = (webp.len() - 8) as u32;
        webp[4..8].copy_from_slice(&riff_size.to_le_bytes());

        webp
    }

    #[test]
    fn write_exif_to_simple_webp() {
        let webp = make_minimal_webp();

        let mut metadata = Metadata::new("WebP");
        metadata
            .exif
            .set("Make", AttrValue::Str("TestCamera".into()));

        let mut input = Cursor::new(&webp);
        let mut output = Vec::new();

        WebpWriter::write(&mut input, &mut output, &metadata).unwrap();

        // Verify RIFF header
        assert_eq!(&output[0..4], b"RIFF");
        assert_eq!(&output[8..12], b"WEBP");

        // Should have VP8X now (because we added EXIF)
        assert_eq!(&output[12..16], b"VP8X");

        // Check VP8X EXIF flag
        let vp8x_flags = output[20];
        assert!(vp8x_flags & VP8X_FLAG_EXIF != 0, "EXIF flag not set");
    }

    #[test]
    fn write_xmp_to_webp() {
        let webp = make_minimal_webp();

        let mut metadata = Metadata::new("WebP");
        metadata.xmp = Some("<x:xmpmeta>test</x:xmpmeta>".to_string());

        let mut input = Cursor::new(&webp);
        let mut output = Vec::new();

        WebpWriter::write(&mut input, &mut output, &metadata).unwrap();

        // Find XMP chunk
        let mut pos = 12;
        let mut found_xmp = false;

        while pos + 8 <= output.len() {
            let chunk_id = &output[pos..pos + 4];
            let chunk_size = u32::from_le_bytes([
                output[pos + 4],
                output[pos + 5],
                output[pos + 6],
                output[pos + 7],
            ]) as usize;

            if chunk_id == b"XMP " {
                found_xmp = true;
                let xmp_data = &output[pos + 8..pos + 8 + chunk_size];
                assert!(String::from_utf8_lossy(xmp_data).contains("xmpmeta"));
            }

            pos += 8 + ((chunk_size + 1) & !1);
        }

        assert!(found_xmp, "XMP chunk not found");

        // Check VP8X XMP flag
        let vp8x_flags = output[20];
        assert!(vp8x_flags & VP8X_FLAG_XMP != 0, "XMP flag not set");
    }

    #[test]
    fn preserve_iccp() {
        let webp = make_extended_webp();

        let mut metadata = Metadata::new("WebP");
        metadata.exif.set("Make", AttrValue::Str("Test".into()));

        let mut input = Cursor::new(&webp);
        let mut output = Vec::new();

        WebpWriter::write(&mut input, &mut output, &metadata).unwrap();

        // Verify ICCP is preserved
        let mut pos = 12;
        let mut found_iccp = false;

        while pos + 8 <= output.len() {
            let chunk_id = &output[pos..pos + 4];
            let chunk_size = u32::from_le_bytes([
                output[pos + 4],
                output[pos + 5],
                output[pos + 6],
                output[pos + 7],
            ]) as usize;

            if chunk_id == b"ICCP" {
                found_iccp = true;
            }

            pos += 8 + ((chunk_size + 1) & !1);
        }

        assert!(found_iccp, "ICCP chunk was not preserved");

        // Check VP8X ICCP flag
        let vp8x_flags = output[20];
        assert!(vp8x_flags & VP8X_FLAG_ICCP != 0, "ICCP flag not set");
    }

    #[test]
    fn chunk_order_correct() {
        let webp = make_extended_webp();

        let mut metadata = Metadata::new("WebP");
        metadata.exif.set("Make", AttrValue::Str("Test".into()));
        metadata.xmp = Some("<xmp>test</xmp>".to_string());

        let mut input = Cursor::new(&webp);
        let mut output = Vec::new();

        WebpWriter::write(&mut input, &mut output, &metadata).unwrap();

        // Collect chunk order
        let mut chunk_order = Vec::new();
        let mut pos = 12;

        while pos + 8 <= output.len() {
            let chunk_id = String::from_utf8_lossy(&output[pos..pos + 4]).to_string();
            let chunk_size = u32::from_le_bytes([
                output[pos + 4],
                output[pos + 5],
                output[pos + 6],
                output[pos + 7],
            ]) as usize;

            chunk_order.push(chunk_id);
            pos += 8 + ((chunk_size + 1) & !1);
        }

        // Expected order: VP8X -> ICCP -> VP8L -> EXIF -> XMP
        let vp8x_pos = chunk_order.iter().position(|s| s == "VP8X");
        let iccp_pos = chunk_order.iter().position(|s| s == "ICCP");
        let vp8l_pos = chunk_order.iter().position(|s| s == "VP8L");
        let exif_pos = chunk_order.iter().position(|s| s == "EXIF");
        let xmp_pos = chunk_order.iter().position(|s| s == "XMP ");

        assert!(vp8x_pos < iccp_pos, "VP8X should come before ICCP");
        assert!(iccp_pos < vp8l_pos, "ICCP should come before VP8L");
        assert!(vp8l_pos < exif_pos, "VP8L should come before EXIF");
        assert!(exif_pos < xmp_pos, "EXIF should come before XMP");
    }
}

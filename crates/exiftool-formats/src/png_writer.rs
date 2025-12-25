//! PNG format writer.
//!
//! PNG writing strategy:
//! - Parse chunks from source
//! - Replace/add eXIf chunk with new EXIF data
//! - Recalculate CRCs for modified chunks
//! - Preserve all other chunks (including image data)

use crate::{Error, Metadata, ReadSeek, Result};
use std::io::Write;

/// PNG magic signature.
const PNG_SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

/// PNG format writer.
pub struct PngWriter;

impl PngWriter {
    /// Write PNG with updated EXIF data.
    ///
    /// - Replaces existing eXIf chunk or inserts new one after IHDR
    /// - Preserves all other chunks including image data
    pub fn write<R, W>(
        input: &mut R,
        output: &mut W,
        metadata: &Metadata,
    ) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        // Read source file (with size limit)
        let data = crate::utils::read_with_limit(input)?;

        if data.len() < 8 || data[..8] != PNG_SIGNATURE {
            return Err(Error::InvalidStructure("invalid PNG signature".into()));
        }

        // Build EXIF bytes
        let exif_bytes = crate::utils::build_exif_bytes(metadata)?;

        // Write PNG signature
        output.write_all(&PNG_SIGNATURE)?;

        // Parse and rewrite chunks
        let mut pos = 8;
        let mut wrote_exif = false;
        let mut after_ihdr = false;

        while pos + 12 <= data.len() {
            let length = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
            let chunk_type = &data[pos + 4..pos + 8];

            if pos + 12 + length > data.len() {
                break;
            }

            let chunk_data = &data[pos + 8..pos + 8 + length];

            match chunk_type {
                b"IHDR" => {
                    // Copy IHDR as-is
                    Self::write_chunk(output, b"IHDR", chunk_data)?;
                    after_ihdr = true;

                    // Insert eXIf right after IHDR (before other chunks)
                    if !exif_bytes.is_empty() {
                        Self::write_chunk(output, b"eXIf", &exif_bytes)?;
                        wrote_exif = true;
                    }
                }
                b"eXIf" => {
                    // Skip old eXIf - we already wrote new one after IHDR
                    // If we haven't written yet (shouldn't happen), write now
                    if !wrote_exif && !exif_bytes.is_empty() {
                        Self::write_chunk(output, b"eXIf", &exif_bytes)?;
                        wrote_exif = true;
                    }
                }
                b"IEND" => {
                    // Write IEND last
                    Self::write_chunk(output, b"IEND", chunk_data)?;
                    break;
                }
                _ => {
                    // Copy other chunks as-is
                    // If we're after IHDR and haven't written EXIF yet, do it now
                    if after_ihdr && !wrote_exif && !exif_bytes.is_empty() {
                        Self::write_chunk(output, b"eXIf", &exif_bytes)?;
                        wrote_exif = true;
                    }
                    Self::write_chunk(output, chunk_type, chunk_data)?;
                }
            }

            pos += 12 + length;
        }

        Ok(())
    }

    /// Write a PNG chunk with CRC.
    fn write_chunk<W: Write>(output: &mut W, chunk_type: &[u8], data: &[u8]) -> Result<()> {
        // Length (4 bytes, big-endian)
        output.write_all(&(data.len() as u32).to_be_bytes())?;

        // Type (4 bytes)
        output.write_all(chunk_type)?;

        // Data
        output.write_all(data)?;

        // CRC32 (type + data)
        let crc = Self::calc_crc(chunk_type, data);
        output.write_all(&crc.to_be_bytes())?;

        Ok(())
    }

    /// Calculate PNG CRC32.
    fn calc_crc(chunk_type: &[u8], data: &[u8]) -> u32 {
        // PNG uses CRC32 with specific polynomial
        let mut crc = 0xFFFF_FFFFu32;

        for &byte in chunk_type.iter().chain(data.iter()) {
            crc = CRC_TABLE[((crc ^ byte as u32) & 0xFF) as usize] ^ (crc >> 8);
        }

        crc ^ 0xFFFF_FFFF
    }

}

/// Pre-computed CRC32 table for PNG.
static CRC_TABLE: [u32; 256] = {
    let mut table = [0u32; 256];
    let mut n = 0;
    while n < 256 {
        let mut c = n as u32;
        let mut k = 0;
        while k < 8 {
            if c & 1 != 0 {
                c = 0xEDB88320 ^ (c >> 1);
            } else {
                c >>= 1;
            }
            k += 1;
        }
        table[n] = c;
        n += 1;
    }
    table
};

#[cfg(test)]
mod tests {
    use super::*;
    use exiftool_attrs::AttrValue;
    use std::io::Cursor;

    fn make_minimal_png() -> Vec<u8> {
        let mut png = Vec::new();
        
        // PNG signature
        png.extend_from_slice(&PNG_SIGNATURE);
        
        // IHDR chunk (13 bytes: width, height, bit depth, color type, etc.)
        let ihdr_data = [
            0, 0, 0, 1,  // width = 1
            0, 0, 0, 1,  // height = 1
            8,           // bit depth = 8
            2,           // color type = 2 (RGB)
            0,           // compression = 0
            0,           // filter = 0
            0,           // interlace = 0
        ];
        png.extend_from_slice(&(ihdr_data.len() as u32).to_be_bytes());
        png.extend_from_slice(b"IHDR");
        png.extend_from_slice(&ihdr_data);
        let crc = PngWriter::calc_crc(b"IHDR", &ihdr_data);
        png.extend_from_slice(&crc.to_be_bytes());
        
        // IDAT chunk (minimal compressed data)
        let idat_data = [0x08, 0xD7, 0x63, 0xF8, 0x0F, 0x00, 0x00, 0x01, 0x01, 0x00];
        png.extend_from_slice(&(idat_data.len() as u32).to_be_bytes());
        png.extend_from_slice(b"IDAT");
        png.extend_from_slice(&idat_data);
        let crc = PngWriter::calc_crc(b"IDAT", &idat_data);
        png.extend_from_slice(&crc.to_be_bytes());
        
        // IEND chunk
        png.extend_from_slice(&0u32.to_be_bytes());
        png.extend_from_slice(b"IEND");
        let crc = PngWriter::calc_crc(b"IEND", &[]);
        png.extend_from_slice(&crc.to_be_bytes());
        
        png
    }

    #[test]
    fn write_exif_to_png() {
        let png = make_minimal_png();
        
        let mut metadata = Metadata::new("PNG");
        metadata.exif.set("Make", AttrValue::Str("TestCam".into()));
        metadata.exif.set("Software", AttrValue::Str("exiftool-rs".into()));

        let mut input = Cursor::new(&png);
        let mut output = Vec::new();

        PngWriter::write(&mut input, &mut output, &metadata).unwrap();

        // Check PNG signature
        assert_eq!(&output[0..8], &PNG_SIGNATURE);

        // Find eXIf chunk
        let mut pos = 8;
        let mut found_exif = false;
        while pos + 12 <= output.len() {
            let length = u32::from_be_bytes([output[pos], output[pos+1], output[pos+2], output[pos+3]]) as usize;
            let chunk_type = &output[pos + 4..pos + 8];
            
            if chunk_type == b"eXIf" {
                found_exif = true;
                // eXIf data should start with TIFF header (II or MM)
                assert!(output[pos + 8] == b'I' || output[pos + 8] == b'M');
                break;
            }
            
            pos += 12 + length;
        }
        
        assert!(found_exif, "eXIf chunk not found in output");
    }

    #[test]
    fn crc_calculation() {
        // Known PNG CRC for IEND chunk (empty data)
        let crc = PngWriter::calc_crc(b"IEND", &[]);
        assert_eq!(crc, 0xAE426082);
    }
}

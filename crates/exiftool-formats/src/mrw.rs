//! MRW (Minolta RAW) format parser.
//!
//! MRW is Minolta/Konica Minolta's RAW format.
//!
//! # Structure
//!
//! - 4 bytes: Magic (0x00 "MRM")
//! - 4 bytes: Offset to PRD block
//! - Blocks: PRD (dimensions), TTW (TIFF-like), WBG, RIF, PAD

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// MRW format parser.
pub struct MrwParser;

impl FormatParser for MrwParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // Magic: 0x00 'M' 'R' 'M' (or just check MRM at offset 1)
        header.len() >= 4 && header[0] == 0x00 && &header[1..4] == b"MRM"
    }

    fn format_name(&self) -> &'static str {
        "MRW"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["mrw"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("MRW");
        meta.exif.set("File:FileType", AttrValue::Str("MRW".to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("image/x-minolta-mrw".to_string()));
        meta.exif.set("Make", AttrValue::Str("Minolta".to_string()));

        // Read header
        let mut header = [0u8; 8];
        reader.seek(SeekFrom::Start(0))?;
        reader.read_exact(&mut header)?;

        // Validate magic
        if header[0] != 0x00 || &header[1..4] != b"MRM" {
            return Err(crate::Error::InvalidStructure("Not a valid MRW file".to_string()));
        }

        // Offset to PRD block (big-endian)
        let prd_offset = u32::from_be_bytes([header[4], header[5], header[6], header[7]]) as u64;
        meta.exif.set("MRW:DataOffset", AttrValue::UInt64(prd_offset));

        // File size
        let file_size = crate::utils::get_file_size(reader)?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        // Parse blocks starting at offset 8
        self.parse_blocks(reader, &mut meta, 8, prd_offset)?;

        Ok(meta)
    }
}

impl MrwParser {
    /// Parse MRW blocks.
    fn parse_blocks(
        &self,
        reader: &mut dyn ReadSeek,
        meta: &mut Metadata,
        start: u64,
        end: u64,
    ) -> Result<()> {
        let mut pos = start;

        while pos + 8 <= end {
            reader.seek(SeekFrom::Start(pos))?;

            let mut block_header = [0u8; 8];
            if reader.read_exact(&mut block_header).is_err() {
                break;
            }

            let block_id = &block_header[0..4];
            let block_size = u32::from_be_bytes([
                block_header[4],
                block_header[5],
                block_header[6],
                block_header[7],
            ]) as u64;

            let data_start = pos + 8;

            match block_id {
                b"\x00PRD" => self.parse_prd(reader, meta, data_start, block_size)?,
                b"\x00TTW" => self.parse_ttw(reader, meta, data_start, block_size)?,
                b"\x00WBG" => self.parse_wbg(reader, meta, data_start, block_size)?,
                b"\x00RIF" => self.parse_rif(reader, meta, data_start, block_size)?,
                _ => {}
            }

            pos = data_start + block_size;
        }

        Ok(())
    }

    /// Parse PRD (Picture Raw Dimensions) block.
    fn parse_prd(
        &self,
        reader: &mut dyn ReadSeek,
        meta: &mut Metadata,
        offset: u64,
        size: u64,
    ) -> Result<()> {
        if size < 8 {
            return Ok(());
        }

        reader.seek(SeekFrom::Start(offset))?;
        let mut data = [0u8; 16];
        let read_size = size.min(16) as usize;
        reader.read_exact(&mut data[..read_size])?;

        // Version (bytes 0-7 as string)
        let version = String::from_utf8_lossy(&data[0..8])
            .trim_end_matches('\0')
            .to_string();
        if !version.is_empty() {
            meta.exif.set("MRW:Version", AttrValue::Str(version));
        }

        if size >= 16 {
            // Sensor dimensions at offset 8
            reader.seek(SeekFrom::Start(offset + 8))?;
            let mut dim = [0u8; 8];
            reader.read_exact(&mut dim)?;

            let sensor_height = u16::from_be_bytes([dim[0], dim[1]]) as u32;
            let sensor_width = u16::from_be_bytes([dim[2], dim[3]]) as u32;
            let image_height = u16::from_be_bytes([dim[4], dim[5]]) as u32;
            let image_width = u16::from_be_bytes([dim[6], dim[7]]) as u32;

            meta.exif.set("MRW:SensorWidth", AttrValue::UInt(sensor_width));
            meta.exif.set("MRW:SensorHeight", AttrValue::UInt(sensor_height));
            meta.exif.set("File:ImageWidth", AttrValue::UInt(image_width));
            meta.exif.set("File:ImageHeight", AttrValue::UInt(image_height));
        }

        if size >= 22 {
            reader.seek(SeekFrom::Start(offset + 16))?;
            let mut extra = [0u8; 6];
            reader.read_exact(&mut extra)?;

            // Data size
            let data_size = u8::from_be_bytes([extra[0]]);
            meta.exif.set("MRW:BitsPerSample", AttrValue::UInt(data_size as u32));

            // Pixel size
            let pixel_size = u8::from_be_bytes([extra[1]]);
            meta.exif.set("MRW:PixelSize", AttrValue::UInt(pixel_size as u32));

            // Storage method
            let storage = u8::from_be_bytes([extra[2]]);
            let storage_name = match storage {
                0x52 => "Packed",
                0x59 => "Linear",
                _ => "Unknown",
            };
            meta.exif.set("MRW:StorageMethod", AttrValue::Str(storage_name.to_string()));

            // Bayer pattern
            let bayer = u8::from_be_bytes([extra[5]]);
            let bayer_name = match bayer {
                0x01 => "RGGB",
                0x04 => "GBRG",
                _ => "Unknown",
            };
            meta.exif.set("MRW:BayerPattern", AttrValue::Str(bayer_name.to_string()));
        }

        Ok(())
    }

    /// Parse TTW (TIFF-like) block - contains EXIF.
    fn parse_ttw(
        &self,
        reader: &mut dyn ReadSeek,
        meta: &mut Metadata,
        offset: u64,
        _size: u64,
    ) -> Result<()> {
        // TTW block contains TIFF IFD structure
        // For now, just mark the offset
        meta.exif.set("MRW:TiffOffset", AttrValue::UInt64(offset));
        meta.exif_offset = Some(offset as usize);

        // Read TIFF header to verify
        reader.seek(SeekFrom::Start(offset))?;
        let mut tiff_header = [0u8; 8];
        if reader.read_exact(&mut tiff_header).is_ok() {
            let is_le = &tiff_header[0..2] == b"II";
            let is_be = &tiff_header[0..2] == b"MM";

            if is_le || is_be {
                meta.exif.set("MRW:TiffByteOrder", AttrValue::Str(
                    if is_le { "Little-endian" } else { "Big-endian" }.to_string()
                ));
            }
        }

        Ok(())
    }

    /// Parse WBG (White Balance Gains) block.
    fn parse_wbg(
        &self,
        reader: &mut dyn ReadSeek,
        meta: &mut Metadata,
        offset: u64,
        size: u64,
    ) -> Result<()> {
        if size < 12 {
            return Ok(());
        }

        reader.seek(SeekFrom::Start(offset))?;
        let mut data = [0u8; 12];
        reader.read_exact(&mut data)?;

        // WB denominators and RGB gains
        let wb_denom0 = u8::from_be_bytes([data[0]]) as f32;
        let wb_denom1 = u8::from_be_bytes([data[1]]) as f32;

        if wb_denom0 > 0.0 && wb_denom1 > 0.0 {
            let r_gain = u16::from_be_bytes([data[4], data[5]]) as f32 / wb_denom0;
            let g_gain = u16::from_be_bytes([data[6], data[7]]) as f32 / wb_denom1;
            let b_gain = u16::from_be_bytes([data[8], data[9]]) as f32 / wb_denom0;

            meta.exif.set(
                "MRW:WBGains",
                AttrValue::Str(format!("{:.4} {:.4} {:.4}", r_gain, g_gain, b_gain)),
            );
        }

        Ok(())
    }

    /// Parse RIF (Raw Image Info) block.
    fn parse_rif(
        &self,
        reader: &mut dyn ReadSeek,
        meta: &mut Metadata,
        offset: u64,
        size: u64,
    ) -> Result<()> {
        if size < 4 {
            return Ok(());
        }

        reader.seek(SeekFrom::Start(offset))?;
        let mut data = [0u8; 4];
        reader.read_exact(&mut data)?;

        // Saturation
        let saturation = data[1];
        meta.exif.set("MRW:Saturation", AttrValue::Int(saturation as i32 - 3));

        // Contrast
        let contrast = data[2];
        meta.exif.set("MRW:Contrast", AttrValue::Int(contrast as i32 - 3));

        // Sharpness
        let sharpness = data[3];
        meta.exif.set("MRW:Sharpness", AttrValue::Int(sharpness as i32 - 3));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_mrw_file(width: u16, height: u16) -> Vec<u8> {
        let mut data = vec![0u8; 512];

        // Header
        data[0] = 0x00;
        data[1..4].copy_from_slice(b"MRM");
        // PRD offset (data starts after all blocks)
        data[4..8].copy_from_slice(&256u32.to_be_bytes());

        // PRD block at offset 8
        data[8..12].copy_from_slice(b"\x00PRD");
        data[12..16].copy_from_slice(&22u32.to_be_bytes()); // size

        // PRD data at offset 16
        data[16..24].copy_from_slice(b"27730001"); // version
        // Sensor dimensions
        data[24..26].copy_from_slice(&height.to_be_bytes());
        data[26..28].copy_from_slice(&width.to_be_bytes());
        // Image dimensions
        data[28..30].copy_from_slice(&height.to_be_bytes());
        data[30..32].copy_from_slice(&width.to_be_bytes());
        // Bits, pixel, storage, unknown, unknown, bayer
        data[32] = 12; // bits
        data[33] = 12; // pixel size
        data[34] = 0x52; // packed
        data[37] = 0x01; // RGGB

        data
    }

    #[test]
    fn test_can_parse() {
        let parser = MrwParser;
        let data = make_mrw_file(3008, 2000);
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = MrwParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"JUNK"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = MrwParser;
        let data = make_mrw_file(3008, 2000);
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "MRW");
        assert_eq!(meta.exif.get_str("Make"), Some("Minolta"));
        assert_eq!(meta.exif.get_u32("File:ImageWidth"), Some(3008));
        assert_eq!(meta.exif.get_u32("File:ImageHeight"), Some(2000));
        assert_eq!(meta.exif.get_str("MRW:BayerPattern"), Some("RGGB"));
    }

    #[test]
    fn test_parse_version() {
        let parser = MrwParser;
        let data = make_mrw_file(3008, 2000);
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("MRW:Version"), Some("27730001"));
    }
}

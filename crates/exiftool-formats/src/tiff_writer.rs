//! TIFF format writer.
//!
//! TIFF writing strategy:
//! - Read entire file, parse IFD structure
//! - Rebuild TIFF with updated metadata
//! - Preserve image data (strip/tile offsets adjusted)

use crate::{Error, Metadata, ReadSeek, Result};
use exiftool_core::{ByteOrder, ExifWriter, IfdReader, WriteEntry};
use exiftool_core::writer::tags;
use std::io::{SeekFrom, Write};

/// TIFF tag IDs for image data.
const TAG_STRIP_OFFSETS: u16 = 0x0111;
const TAG_STRIP_BYTE_COUNTS: u16 = 0x0117;
const TAG_TILE_OFFSETS: u16 = 0x0144;
const TAG_TILE_BYTE_COUNTS: u16 = 0x0145;

/// Image data chunk (strip or tile).
#[derive(Debug, Clone)]
struct ImageChunk {
    offset: u32,
    length: u32,
}

/// Parsed TIFF structure for preservation.
struct TiffStructure {
    byte_order: ByteOrder,
    chunks: Vec<ImageChunk>,
    is_tiled: bool,
}

/// TIFF format writer.
pub struct TiffWriter;

impl TiffWriter {
    /// Write TIFF with updated metadata, preserving image data.
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
        input.seek(SeekFrom::Start(0))?;
        let data = crate::utils::read_with_limit(input)?;

        if data.len() < 8 {
            return Err(Error::InvalidStructure("TIFF file too small".into()));
        }

        // Parse original structure
        let structure = Self::parse_structure(&data)?;
        
        // Build new EXIF/IFD from metadata
        let mut writer = ExifWriter::new(structure.byte_order);
        Self::populate_from_metadata(&mut writer, metadata);
        
        // Add image-related tags that we need to preserve
        // These will be updated with correct offsets after we know the header size
        
        // Serialize metadata part first to know its size
        let meta_bytes = writer.serialize().map_err(|e| {
            Error::InvalidStructure(format!("EXIF serialize error: {}", e))
        })?;
        
        if structure.chunks.is_empty() {
            // No image data - just output metadata
            output.write_all(&meta_bytes)?;
            return Ok(());
        }
        
        // Calculate where image data will start
        // We need to rebuild with correct offsets
        let image_data_start = meta_bytes.len() as u32;
        
        // Build final TIFF with correct strip/tile offsets
        let final_bytes = Self::build_with_image_data(
            &data,
            &structure,
            metadata,
            image_data_start,
        )?;
        
        output.write_all(&final_bytes)?;
        
        Ok(())
    }

    /// Parse TIFF structure to find image chunks.
    fn parse_structure(data: &[u8]) -> Result<TiffStructure> {
        let byte_order = if data[0] == b'I' && data[1] == b'I' {
            ByteOrder::LittleEndian
        } else if data[0] == b'M' && data[1] == b'M' {
            ByteOrder::BigEndian
        } else {
            return Err(Error::InvalidStructure("invalid TIFF byte order".into()));
        };
        
        let reader = IfdReader::new(data, byte_order, 0);
        let ifd0_offset = reader.parse_header().map_err(Error::Core)?;
        
        let mut chunks = Vec::new();
        let mut is_tiled = false;
        
        if let Ok((entries, _)) = reader.read_ifd(ifd0_offset) {
            let mut strip_offsets: Vec<u32> = Vec::new();
            let mut strip_counts: Vec<u32> = Vec::new();
            let mut tile_offsets: Vec<u32> = Vec::new();
            let mut tile_counts: Vec<u32> = Vec::new();
            
            for entry in &entries {
                match entry.tag {
                    TAG_STRIP_OFFSETS => {
                        strip_offsets = Self::extract_u32_array(&entry.value);
                    }
                    TAG_STRIP_BYTE_COUNTS => {
                        strip_counts = Self::extract_u32_array(&entry.value);
                    }
                    TAG_TILE_OFFSETS => {
                        tile_offsets = Self::extract_u32_array(&entry.value);
                        is_tiled = true;
                    }
                    TAG_TILE_BYTE_COUNTS => {
                        tile_counts = Self::extract_u32_array(&entry.value);
                    }
                    _ => {}
                }
            }
            
            // Build chunks from strips or tiles
            if is_tiled && !tile_offsets.is_empty() {
                for (offset, length) in tile_offsets.iter().zip(tile_counts.iter()) {
                    chunks.push(ImageChunk {
                        offset: *offset,
                        length: *length,
                    });
                }
            } else if !strip_offsets.is_empty() {
                for (offset, length) in strip_offsets.iter().zip(strip_counts.iter()) {
                    chunks.push(ImageChunk {
                        offset: *offset,
                        length: *length,
                    });
                }
            }
        }
        
        Ok(TiffStructure {
            byte_order,
            chunks,
            is_tiled,
        })
    }
    
    /// Extract u32 array from RawValue.
    fn extract_u32_array(value: &exiftool_core::RawValue) -> Vec<u32> {
        use exiftool_core::RawValue;
        match value {
            RawValue::UInt16(v) => v.iter().map(|&x| x as u32).collect(),
            RawValue::UInt32(v) => v.clone(),
            _ => Vec::new(),
        }
    }
    
    /// Build complete TIFF with metadata and image data.
    fn build_with_image_data(
        original: &[u8],
        structure: &TiffStructure,
        metadata: &Metadata,
        _image_start: u32,
    ) -> Result<Vec<u8>> {
        // Strategy: Build metadata, append image data, fix offsets
        
        let mut writer = ExifWriter::new(structure.byte_order);
        Self::populate_from_metadata(&mut writer, metadata);
        
        // Get preliminary size to calculate image data position
        let prelim = writer.serialize().map_err(|e| {
            Error::InvalidStructure(format!("serialize error: {}", e))
        })?;
        
        // Image data starts right after IFD structure
        // We need some padding to align properly
        let header_size = prelim.len();
        let image_data_offset = header_size.div_ceil(4) * 4; // Align to 4 bytes
        
        // Calculate new offsets for each chunk
        let mut new_offsets: Vec<u32> = Vec::new();
        let mut current_offset = image_data_offset as u32;
        
        for chunk in &structure.chunks {
            new_offsets.push(current_offset);
            current_offset += chunk.length;
        }
        
        // Rebuild writer with correct strip/tile offsets
        let mut writer = ExifWriter::new(structure.byte_order);
        Self::populate_from_metadata(&mut writer, metadata);
        
        // Add strip/tile offset and count tags
        if !new_offsets.is_empty() {
            let counts: Vec<u32> = structure.chunks.iter().map(|c| c.length).collect();
            
            if structure.is_tiled {
                writer.add_ifd0(WriteEntry::from_u32_array(TAG_TILE_OFFSETS, &new_offsets));
                writer.add_ifd0(WriteEntry::from_u32_array(TAG_TILE_BYTE_COUNTS, &counts));
            } else {
                writer.add_ifd0(WriteEntry::from_u32_array(TAG_STRIP_OFFSETS, &new_offsets));
                writer.add_ifd0(WriteEntry::from_u32_array(TAG_STRIP_BYTE_COUNTS, &counts));
            }
        }
        
        // Serialize final header
        let header = writer.serialize().map_err(|e| {
            Error::InvalidStructure(format!("serialize error: {}", e))
        })?;
        
        // Build final output
        let total_size = image_data_offset + structure.chunks.iter().map(|c| c.length as usize).sum::<usize>();
        let mut output = vec![0u8; total_size];
        
        // Copy header
        output[..header.len()].copy_from_slice(&header);
        
        // Copy image data from original file
        let mut write_pos = image_data_offset;
        for chunk in &structure.chunks {
            let src_start = chunk.offset as usize;
            let src_end = src_start + chunk.length as usize;
            
            if src_end <= original.len() {
                output[write_pos..write_pos + chunk.length as usize]
                    .copy_from_slice(&original[src_start..src_end]);
            }
            write_pos += chunk.length as usize;
        }
        
        Ok(output)
    }

    /// Write standalone TIFF/EXIF bytes (no source file needed).
    pub fn write_new<W: Write>(output: &mut W, metadata: &Metadata) -> Result<()> {
        let mut writer = ExifWriter::new_le();
        Self::populate_from_metadata(&mut writer, metadata);

        let bytes = writer.serialize().map_err(|e| {
            Error::InvalidStructure(format!("EXIF serialize error: {}", e))
        })?;

        output.write_all(&bytes)?;
        Ok(())
    }

    /// Populate ExifWriter from Metadata attrs.
    fn populate_from_metadata(writer: &mut ExifWriter, metadata: &Metadata) {
        use exiftool_attrs::AttrValue;

        // IFD0 tags
        if let Some(v) = metadata.exif.get_str("Make") {
            writer.add_ifd0(WriteEntry::from_str(tags::MAKE, v));
        }
        if let Some(v) = metadata.exif.get_str("Model") {
            writer.add_ifd0(WriteEntry::from_str(tags::MODEL, v));
        }
        if let Some(v) = metadata.exif.get_str("Software") {
            writer.add_ifd0(WriteEntry::from_str(tags::SOFTWARE, v));
        }
        if let Some(v) = metadata.exif.get_str("DateTime") {
            writer.add_ifd0(WriteEntry::from_str(tags::DATE_TIME, v));
        }
        if let Some(v) = metadata.exif.get_str("Artist") {
            writer.add_ifd0(WriteEntry::from_str(tags::ARTIST, v));
        }
        if let Some(v) = metadata.exif.get_str("Copyright") {
            writer.add_ifd0(WriteEntry::from_str(tags::COPYRIGHT, v));
        }
        if let Some(v) = metadata.exif.get_str("ImageDescription") {
            writer.add_ifd0(WriteEntry::from_str(tags::IMAGE_DESCRIPTION, v));
        }

        // Orientation
        if let Some(AttrValue::UInt(v)) = metadata.exif.get("Orientation") {
            writer.add_ifd0(WriteEntry::from_u16(tags::ORIENTATION, *v as u16));
        }

        // Resolution
        if let Some(AttrValue::URational(n, d)) = metadata.exif.get("XResolution") {
            writer.add_ifd0(WriteEntry::from_urational(tags::X_RESOLUTION, *n, *d));
        }
        if let Some(AttrValue::URational(n, d)) = metadata.exif.get("YResolution") {
            writer.add_ifd0(WriteEntry::from_urational(tags::Y_RESOLUTION, *n, *d));
        }
        if let Some(AttrValue::UInt(v)) = metadata.exif.get("ResolutionUnit") {
            writer.add_ifd0(WriteEntry::from_u16(tags::RESOLUTION_UNIT, *v as u16));
        }

        // ExifIFD tags
        if let Some(v) = metadata.exif.get_str("DateTimeOriginal") {
            writer.add_exif(WriteEntry::from_str(tags::DATE_TIME_ORIGINAL, v));
        }
        if let Some(v) = metadata.exif.get_str("CreateDate") {
            writer.add_exif(WriteEntry::from_str(tags::CREATE_DATE, v));
        }
        if let Some(AttrValue::UInt(v)) = metadata.exif.get("ISO") {
            writer.add_exif(WriteEntry::from_u16(tags::ISO, *v as u16));
        }

        // Exposure time
        if let Some(AttrValue::URational(n, d)) = metadata.exif.get("ExposureTime") {
            writer.add_exif(WriteEntry::from_urational(tags::EXPOSURE_TIME, *n, *d));
        }

        // F-number
        if let Some(AttrValue::URational(n, d)) = metadata.exif.get("FNumber") {
            writer.add_exif(WriteEntry::from_urational(tags::FNUMBER, *n, *d));
        }

        // Focal length
        if let Some(AttrValue::URational(n, d)) = metadata.exif.get("FocalLength") {
            writer.add_exif(WriteEntry::from_urational(tags::FOCAL_LENGTH, *n, *d));
        }

        // GPS tags
        if let Some(v) = metadata.exif.get_str("GPSLatitudeRef") {
            writer.add_gps(WriteEntry::from_str(tags::GPS_LATITUDE_REF, v));
        }
        if let Some(v) = metadata.exif.get_str("GPSLongitudeRef") {
            writer.add_gps(WriteEntry::from_str(tags::GPS_LONGITUDE_REF, v));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use exiftool_attrs::AttrValue;
    use std::io::Cursor;

    #[test]
    fn write_new_tiff() {
        let mut metadata = Metadata::new("TIFF");
        metadata.exif.set("Make", AttrValue::Str("TestCam".into()));
        metadata.exif.set("Model", AttrValue::Str("Model 1".into()));

        let mut output = Vec::new();
        TiffWriter::write_new(&mut output, &metadata).unwrap();

        // Check TIFF header
        assert_eq!(&output[0..2], b"II"); // Little-endian
        assert_eq!(output[2], 0x2A); // Magic
    }

    #[test]
    fn round_trip_metadata() {
        let mut metadata = Metadata::new("TIFF");
        metadata.exif.set("Make", AttrValue::Str("RustCam".into()));
        metadata.exif.set("Software", AttrValue::Str("exiftool-rs".into()));

        let mut output = Vec::new();
        TiffWriter::write_new(&mut output, &metadata).unwrap();

        // Parse back
        use crate::TiffParser;
        use crate::FormatParser;

        let mut cursor = Cursor::new(&output);
        let parsed = TiffParser::default().parse(&mut cursor).unwrap();

        assert_eq!(parsed.exif.get_str("Make"), Some("RustCam"));
        assert_eq!(parsed.exif.get_str("Software"), Some("exiftool-rs"));
    }
    
    #[test]
    fn preserve_strip_structure() {
        // Create a minimal TIFF with strip data
        let mut tiff = Vec::new();
        
        // Little-endian header
        tiff.extend_from_slice(b"II");
        tiff.extend_from_slice(&42u16.to_le_bytes()); // Magic
        tiff.extend_from_slice(&8u32.to_le_bytes());  // IFD offset
        
        // IFD with 2 entries (StripOffsets, StripByteCounts)
        tiff.extend_from_slice(&2u16.to_le_bytes()); // Entry count
        
        // StripOffsets entry (tag 0x0111)
        tiff.extend_from_slice(&0x0111u16.to_le_bytes());
        tiff.extend_from_slice(&4u16.to_le_bytes()); // LONG
        tiff.extend_from_slice(&1u32.to_le_bytes()); // count
        tiff.extend_from_slice(&50u32.to_le_bytes()); // offset value (where strip data starts)
        
        // StripByteCounts entry (tag 0x0117)
        tiff.extend_from_slice(&0x0117u16.to_le_bytes());
        tiff.extend_from_slice(&4u16.to_le_bytes()); // LONG
        tiff.extend_from_slice(&1u32.to_le_bytes()); // count
        tiff.extend_from_slice(&10u32.to_le_bytes()); // 10 bytes of data
        
        // Next IFD offset (0 = none)
        tiff.extend_from_slice(&0u32.to_le_bytes());
        
        // Pad to offset 50
        while tiff.len() < 50 {
            tiff.push(0);
        }
        
        // Strip data (10 bytes)
        tiff.extend_from_slice(b"PIXELDATA!");
        
        // Parse and verify structure
        let structure = TiffWriter::parse_structure(&tiff).unwrap();
        assert_eq!(structure.chunks.len(), 1);
        assert_eq!(structure.chunks[0].offset, 50);
        assert_eq!(structure.chunks[0].length, 10);
        assert!(!structure.is_tiled);
    }
}

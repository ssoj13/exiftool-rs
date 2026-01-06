//! HEIC/HEIF format parser.
//!
//! HEIC (High Efficiency Image Container) uses ISOBMFF (ISO Base Media File Format).
//! Structure:
//! - ftyp box: file type identifier (heic, mif1, msf1, hevc, heix, etc.)
//! - meta box: contains metadata including:
//!   - hdlr: handler type
//!   - pitm: primary item
//!   - iloc: item locations (offset/length for each item)
//!   - iinf: item info (item types - identifies which item is Exif)
//!   - iprp: item properties
//!   - idat: inline item data
//! - mdat box: media data (actual image data, often contains EXIF)
//!
//! EXIF is stored as an item, referenced via iloc.
//! The EXIF item data has a 4-byte prefix (offset to TIFF header), then TIFF structure.

use crate::tag_lookup::{lookup_exif_subifd, lookup_gps, lookup_ifd0};
use crate::{Error, FormatParser, Metadata, ReadSeek, Result, makernotes};
use exiftool_attrs::AttrValue;
use exiftool_core::{ByteOrder, IfdReader, RawValue};
use std::io::SeekFrom;

/// HEIC/HEIF brand identifiers in ftyp box.
const HEIC_BRANDS: &[&[u8; 4]] = &[
    b"heic", // HEIC (HEVC)
    b"heix", // HEIC (HEVC) extended
    b"hevc", // HEVC sequence
    b"hevx", // HEVC extended
    b"mif1", // HEIF image
    b"msf1", // HEIF sequence
    b"avif", // AVIF (AV1)
    b"avis", // AVIF sequence
];

/// Item location info from iloc box.
#[derive(Debug, Default, Clone)]
struct ItemLocation {
    construction_method: u8,
    data_ref_index: u16,
    base_offset: u64,
    extents: Vec<(u64, u64)>, // (offset, length)
}

/// HEIC/HEIF format parser.
pub struct HeicParser;

impl FormatParser for HeicParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 12 {
            return false;
        }
        
        // First 4 bytes: box size, next 4: "ftyp"
        if &header[4..8] != b"ftyp" {
            return false;
        }
        
        // Check major brand (bytes 8-11)
        let brand = &header[8..12];
        HEIC_BRANDS.iter().any(|b| brand == *b)
    }

    fn format_name(&self) -> &'static str {
        "HEIC"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["heic", "heif", "avif"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("HEIC");
        let mut state = ParseState::default();

        // Parse ftyp box first
        reader.seek(SeekFrom::Start(0))?;
        
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        
        let ftyp_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
        
        if &buf[4..8] != b"ftyp" {
            return Err(Error::InvalidStructure("Missing ftyp box".into()));
        }
        
        // Read major brand
        let mut brand = [0u8; 4];
        reader.read_exact(&mut brand)?;
        
        let brand_str = String::from_utf8_lossy(&brand).to_string();
        metadata.exif.set("MajorBrand", AttrValue::Str(brand_str.clone()));
        
        // Determine format variant
        metadata.format = match brand_str.as_str() {
            "avif" | "avis" => "AVIF",
            "mif1" | "msf1" => "HEIF",
            _ => "HEIC",
        };
        
        // Read minor version
        let mut version = [0u8; 4];
        reader.read_exact(&mut version)?;
        let minor_version = u32::from_be_bytes(version);
        metadata.exif.set("MinorVersion", AttrValue::UInt(minor_version));
        
        // Read compatible brands
        let brands_size = ftyp_size.saturating_sub(16);
        if brands_size > 0 && brands_size < 1024 {
            let mut brands_buf = vec![0u8; brands_size as usize];
            reader.read_exact(&mut brands_buf)?;
            
            let brands: Vec<String> = brands_buf
                .chunks(4)
                .filter(|c| c.len() == 4)
                .map(|c| String::from_utf8_lossy(c).trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            if !brands.is_empty() {
                metadata.exif.set("CompatibleBrands", AttrValue::Str(brands.join(", ")));
            }
        }
        
        // Parse remaining boxes
        reader.seek(SeekFrom::Start(ftyp_size))?;
        let file_size = crate::utils::get_file_size(reader)?;
        reader.seek(SeekFrom::Start(ftyp_size))?;
        
        while reader.stream_position()? < file_size {
            let pos = reader.stream_position()?;
            
            if reader.read_exact(&mut buf).is_err() {
                break;
            }
            
            let mut box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];
            
            // Handle extended size
            if box_size == 1 {
                let mut ext_size = [0u8; 8];
                reader.read_exact(&mut ext_size)?;
                box_size = u64::from_be_bytes(ext_size);
            } else if box_size == 0 {
                box_size = file_size - pos;
            }
            
            match &box_type {
                b"meta" => {
                    self.parse_meta_box(reader, pos, box_size, &mut metadata, &mut state)?;
                }
                b"mdat" => {
                    state.mdat_offset = Some(pos + 8);
                    metadata.exif.set("MediaDataSize", AttrValue::UInt((box_size - 8) as u32));
                }
                _ => {}
            }
            
            if box_size == 0 || pos + box_size > file_size {
                break;
            }
            reader.seek(SeekFrom::Start(pos + box_size))?;
        }
        
        // Now extract EXIF if we found its location
        if let Some(exif_item_id) = state.exif_item_id {
            if let Some(loc) = state.item_locations.get(&exif_item_id) {
                self.extract_exif(reader, loc, &state, &mut metadata)?;
            }
        }
        
        Ok(metadata)
    }
}

/// Parser state for collecting item info across boxes.
#[derive(Default)]
struct ParseState {
    exif_item_id: Option<u32>,
    item_locations: std::collections::HashMap<u32, ItemLocation>,
    mdat_offset: Option<u64>,
    idat_offset: Option<u64>,
    idat_data: Option<Vec<u8>>,
}

impl HeicParser {
    /// Parse meta box and its children.
    fn parse_meta_box(
        &self,
        reader: &mut dyn ReadSeek,
        meta_start: u64,
        meta_size: u64,
        metadata: &mut Metadata,
        state: &mut ParseState,
    ) -> Result<()> {
        // Skip version/flags (4 bytes after box header)
        reader.seek(SeekFrom::Start(meta_start + 12))?;
        
        let meta_end = meta_start + meta_size;
        let mut buf = [0u8; 8];
        
        while reader.stream_position()? < meta_end {
            let pos = reader.stream_position()?;
            
            if reader.read_exact(&mut buf).is_err() {
                break;
            }
            
            let box_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let box_type = [buf[4], buf[5], buf[6], buf[7]];
            
            if box_size < 8 || pos + box_size > meta_end {
                break;
            }
            
            match &box_type {
                b"iinf" => {
                    self.parse_iinf_box(reader, pos, box_size, metadata, state)?;
                }
                b"iloc" => {
                    self.parse_iloc_box(reader, pos, box_size, state)?;
                }
                b"iprp" => {
                    self.parse_iprp_box(reader, pos, box_size, metadata)?;
                }
                b"idat" => {
                    // Inline item data - small items can be stored here
                    let data_size = (box_size - 8) as usize;
                    if data_size < 1024 * 1024 {
                        let mut data = vec![0u8; data_size];
                        reader.read_exact(&mut data)?;
                        state.idat_offset = Some(pos + 8);
                        state.idat_data = Some(data);
                    }
                }
                _ => {}
            }
            
            reader.seek(SeekFrom::Start(pos + box_size))?;
        }
        
        Ok(())
    }
    
    /// Parse iinf (item info) box to find Exif item ID.
    fn parse_iinf_box(
        &self,
        reader: &mut dyn ReadSeek,
        box_start: u64,
        box_size: u64,
        metadata: &mut Metadata,
        state: &mut ParseState,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(box_start + 8))?;
        
        // Version and flags
        let mut vf = [0u8; 4];
        reader.read_exact(&mut vf)?;
        let version = vf[0];
        
        // Entry count
        let entry_count = if version == 0 {
            let mut count = [0u8; 2];
            reader.read_exact(&mut count)?;
            u16::from_be_bytes(count) as u32
        } else {
            let mut count = [0u8; 4];
            reader.read_exact(&mut count)?;
            u32::from_be_bytes(count)
        };
        
        metadata.exif.set("ItemCount", AttrValue::UInt(entry_count));
        
        let box_end = box_start + box_size;
        let mut buf = [0u8; 8];
        
        // Parse infe (item info entry) boxes
        while reader.stream_position()? < box_end {
            let pos = reader.stream_position()?;
            
            if reader.read_exact(&mut buf).is_err() {
                break;
            }
            
            let infe_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let infe_type = [buf[4], buf[5], buf[6], buf[7]];
            
            if &infe_type != b"infe" || infe_size < 12 {
                if pos + infe_size > box_end || infe_size == 0 {
                    break;
                }
                reader.seek(SeekFrom::Start(pos + infe_size))?;
                continue;
            }
            
            // Parse infe entry
            let mut infe_vf = [0u8; 4];
            reader.read_exact(&mut infe_vf)?;
            let infe_version = infe_vf[0];
            
            let item_id = if infe_version < 3 {
                let mut id = [0u8; 2];
                reader.read_exact(&mut id)?;
                u16::from_be_bytes(id) as u32
            } else {
                let mut id = [0u8; 4];
                reader.read_exact(&mut id)?;
                u32::from_be_bytes(id)
            };
            
            // Skip item_protection_index (2 bytes)
            let mut _protection = [0u8; 2];
            reader.read_exact(&mut _protection)?;
            
            if infe_version >= 2 {
                // item_type is 4 bytes
                let mut item_type = [0u8; 4];
                reader.read_exact(&mut item_type)?;
                
                if &item_type == b"Exif" {
                    state.exif_item_id = Some(item_id);
                }
            }
            
            reader.seek(SeekFrom::Start(pos + infe_size))?;
        }
        
        Ok(())
    }
    
    /// Parse iloc (item location) box.
    fn parse_iloc_box(
        &self,
        reader: &mut dyn ReadSeek,
        box_start: u64,
        box_size: u64,
        state: &mut ParseState,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(box_start + 8))?;
        
        // Version and flags
        let mut vf = [0u8; 4];
        reader.read_exact(&mut vf)?;
        let version = vf[0];
        
        // Size fields byte
        let mut sizes = [0u8; 2];
        reader.read_exact(&mut sizes)?;
        
        let offset_size = (sizes[0] >> 4) & 0x0F;
        let length_size = sizes[0] & 0x0F;
        let base_offset_size = (sizes[1] >> 4) & 0x0F;
        let index_size = if version >= 1 { sizes[1] & 0x0F } else { 0 };
        
        // Item count
        let item_count = if version < 2 {
            let mut count = [0u8; 2];
            reader.read_exact(&mut count)?;
            u16::from_be_bytes(count) as u32
        } else {
            let mut count = [0u8; 4];
            reader.read_exact(&mut count)?;
            u32::from_be_bytes(count)
        };
        
        let box_end = box_start + box_size;
        
        for _ in 0..item_count {
            if reader.stream_position()? >= box_end {
                break;
            }
            
            // Item ID
            let item_id = if version < 2 {
                let mut id = [0u8; 2];
                reader.read_exact(&mut id)?;
                u16::from_be_bytes(id) as u32
            } else {
                let mut id = [0u8; 4];
                reader.read_exact(&mut id)?;
                u32::from_be_bytes(id)
            };
            
            let mut loc = ItemLocation::default();
            
            // Construction method (version 1, 2)
            if version >= 1 {
                let mut cm = [0u8; 2];
                reader.read_exact(&mut cm)?;
                loc.construction_method = cm[1] & 0x0F;
            }
            
            // Data reference index
            let mut dri = [0u8; 2];
            reader.read_exact(&mut dri)?;
            loc.data_ref_index = u16::from_be_bytes(dri);
            
            // Base offset
            loc.base_offset = self.read_var_int(reader, base_offset_size)?;
            
            // Extent count
            let mut ec = [0u8; 2];
            reader.read_exact(&mut ec)?;
            let extent_count = u16::from_be_bytes(ec);
            
            for _ in 0..extent_count {
                // Extent index (version >= 1, if index_size > 0)
                if version >= 1 && index_size > 0 {
                    let _ = self.read_var_int(reader, index_size)?;
                }
                
                let extent_offset = self.read_var_int(reader, offset_size)?;
                let extent_length = self.read_var_int(reader, length_size)?;
                
                loc.extents.push((extent_offset, extent_length));
            }
            
            state.item_locations.insert(item_id, loc);
        }
        
        Ok(())
    }
    
    /// Read variable-size integer.
    fn read_var_int(&self, reader: &mut dyn ReadSeek, size: u8) -> Result<u64> {
        match size {
            0 => Ok(0),
            4 => {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                Ok(u32::from_be_bytes(buf) as u64)
            }
            8 => {
                let mut buf = [0u8; 8];
                reader.read_exact(&mut buf)?;
                Ok(u64::from_be_bytes(buf))
            }
            _ => {
                // Handle 1, 2, 3 byte sizes
                let mut buf = [0u8; 8];
                reader.read_exact(&mut buf[..size as usize])?;
                let mut val = 0u64;
                for byte in buf.iter().take(size as usize) {
                    val = (val << 8) | *byte as u64;
                }
                Ok(val)
            }
        }
    }
    
    /// Extract and parse EXIF data.
    fn extract_exif(
        &self,
        reader: &mut dyn ReadSeek,
        loc: &ItemLocation,
        state: &ParseState,
        metadata: &mut Metadata,
    ) -> Result<()> {
        if loc.extents.is_empty() {
            return Ok(());
        }
        
        // Calculate total size
        let total_len: u64 = loc.extents.iter().map(|(_, len)| len).sum();
        if !(12..=10 * 1024 * 1024).contains(&total_len) {
            return Ok(()); // Sanity check
        }
        
        let mut exif_data = vec![0u8; total_len as usize];
        let mut write_pos = 0;
        
        for (extent_offset, extent_length) in &loc.extents {
            let abs_offset = match loc.construction_method {
                0 => {
                    // File offset (relative to mdat)
                    loc.base_offset + extent_offset
                }
                1 => {
                    // idat offset
                    if let Some(idat_off) = state.idat_offset {
                        idat_off + loc.base_offset + extent_offset
                    } else {
                        continue;
                    }
                }
                2 => {
                    // Item offset - skip for now
                    continue;
                }
                _ => continue,
            };
            
            if loc.construction_method == 1 {
                // Read from idat_data if available
                if let Some(ref idat) = state.idat_data {
                    let start = (loc.base_offset + extent_offset) as usize;
                    let end = start + *extent_length as usize;
                    if end <= idat.len() {
                        exif_data[write_pos..write_pos + *extent_length as usize]
                            .copy_from_slice(&idat[start..end]);
                        write_pos += *extent_length as usize;
                    }
                }
            } else {
                // Read from file
                reader.seek(SeekFrom::Start(abs_offset))?;
                reader.read_exact(&mut exif_data[write_pos..write_pos + *extent_length as usize])?;
                write_pos += *extent_length as usize;
            }
        }
        
        if exif_data.len() < 10 {
            return Ok(());
        }
        
        // EXIF item format varies:
        // - Some: 4-byte offset prefix, then TIFF data
        // - Some: 2-byte prefix + "Exif\0\0" + TIFF data (iPhone)
        // - Some: "Exif\0\0" + TIFF data
        // Find TIFF header by searching for byte order markers
        let tiff_start = self.find_tiff_header(&exif_data);
        
        if tiff_start >= exif_data.len() {
            return Ok(());
        }
        
        let tiff_data = &exif_data[tiff_start..];
        if tiff_data.len() < 8 {
            return Ok(());
        }
        
        // Parse TIFF/EXIF
        self.parse_tiff_exif(tiff_data, metadata)?;
        metadata.exif_offset = Some(tiff_start);
        
        Ok(())
    }
    
    /// Find TIFF header in EXIF data by searching for byte order markers.
    fn find_tiff_header(&self, data: &[u8]) -> usize {
        // Search for "II" (Intel) or "MM" (Motorola) byte order markers
        // followed by TIFF magic number 0x002A
        for i in 0..data.len().saturating_sub(4) {
            let marker = &data[i..i + 2];
            if (marker == b"II" || marker == b"MM") && data.len() > i + 3 {
                // Check TIFF magic: 0x002A (big) or 0x2A00 (little)
                let magic = if marker == b"MM" {
                    u16::from_be_bytes([data[i + 2], data[i + 3]])
                } else {
                    u16::from_le_bytes([data[i + 2], data[i + 3]])
                };
                if magic == 42 {
                    return i;
                }
            }
        }
        // Fallback: skip 4-byte offset prefix
        4
    }
    
    /// Parse TIFF-format EXIF data.
    fn parse_tiff_exif(&self, tiff_data: &[u8], metadata: &mut Metadata) -> Result<()> {
        let byte_order = ByteOrder::from_marker([tiff_data[0], tiff_data[1]])
            .map_err(Error::Core)?;
        
        let reader = IfdReader::new(tiff_data, byte_order, 0);
        let ifd0_offset = reader.parse_header().map_err(Error::Core)?;
        
        let (entries, _next_ifd) = reader.read_ifd(ifd0_offset).map_err(Error::Core)?;
        
        // Find Make for MakerNotes vendor detection
        let mut vendor = makernotes::Vendor::Unknown;
        for entry in &entries {
            if entry.tag == 0x010F {
                if let RawValue::String(make) = &entry.value {
                    vendor = makernotes::Vendor::from_make(make);
                }
                break;
            }
        }
        
        // Convert IFD0 entries
        for entry in &entries {
            if let Some(name) = lookup_ifd0(entry.tag) {
                metadata.exif.set(name, entry_to_attr(entry));
            }
            
            match entry.tag {
                0x8769 => {
                    // ExifIFD pointer
                    if let Some(offset) = entry.value.as_u32() {
                        if let Ok((exif_entries, _)) = reader.read_ifd(offset) {
                            for e in &exif_entries {
                                if e.tag == 0x927C {
                                    // MakerNotes
                                    if let RawValue::Undefined(bytes) = &e.value {
                                        if let Some(mn_data) = makernotes::parse(bytes, vendor, byte_order) {
                                            for (key, val) in mn_data.iter() {
                                                metadata.exif.set(key.clone(), val.clone());
                                            }
                                        }
                                    }
                                } else if let Some(name) = lookup_exif_subifd(e.tag) {
                                    metadata.exif.set(name, entry_to_attr(e));
                                }
                            }
                        }
                    }
                }
                0x8825 => {
                    // GPS IFD pointer
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
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Parse iprp (item properties) box to find image dimensions.
    fn parse_iprp_box(
        &self,
        reader: &mut dyn ReadSeek,
        box_start: u64,
        box_size: u64,
        metadata: &mut Metadata,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(box_start + 8))?;
        
        let box_end = box_start + box_size;
        let mut buf = [0u8; 8];
        
        while reader.stream_position()? < box_end {
            let pos = reader.stream_position()?;
            
            if reader.read_exact(&mut buf).is_err() {
                break;
            }
            
            let inner_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let inner_type = [buf[4], buf[5], buf[6], buf[7]];
            
            if inner_size < 8 || pos + inner_size > box_end {
                break;
            }
            
            if &inner_type == b"ipco" {
                self.parse_ipco_box(reader, pos, inner_size, metadata)?;
            }
            
            reader.seek(SeekFrom::Start(pos + inner_size))?;
        }
        
        Ok(())
    }
    
    /// Parse ipco (item property container) box.
    fn parse_ipco_box(
        &self,
        reader: &mut dyn ReadSeek,
        box_start: u64,
        box_size: u64,
        metadata: &mut Metadata,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(box_start + 8))?;
        
        let box_end = box_start + box_size;
        let mut buf = [0u8; 8];
        
        while reader.stream_position()? < box_end {
            let pos = reader.stream_position()?;
            
            if reader.read_exact(&mut buf).is_err() {
                break;
            }
            
            let inner_size = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as u64;
            let inner_type = [buf[4], buf[5], buf[6], buf[7]];
            
            if inner_size < 8 || pos + inner_size > box_end {
                break;
            }
            
            match &inner_type {
                b"ispe" => {
                    let mut dim = [0u8; 12];
                    reader.read_exact(&mut dim)?;
                    let width = u32::from_be_bytes([dim[4], dim[5], dim[6], dim[7]]);
                    let height = u32::from_be_bytes([dim[8], dim[9], dim[10], dim[11]]);
                    metadata.exif.set("ImageWidth", AttrValue::UInt(width));
                    metadata.exif.set("ImageHeight", AttrValue::UInt(height));
                }
                b"pixi" => {
                    let mut pixi = [0u8; 5];
                    if reader.read_exact(&mut pixi).is_ok() {
                        let num_channels = pixi[4];
                        metadata.exif.set("ChannelCount", AttrValue::UInt(num_channels as u32));
                    }
                }
                b"colr" => {
                    let mut colr_type = [0u8; 4];
                    if reader.read_exact(&mut colr_type).is_ok() {
                        let color_type = String::from_utf8_lossy(&colr_type).to_string();
                        metadata.exif.set("ColorType", AttrValue::Str(color_type));
                    }
                }
                b"hvcC" => {
                    metadata.exif.set("Codec", AttrValue::Str("HEVC".into()));
                }
                b"av1C" => {
                    metadata.exif.set("Codec", AttrValue::Str("AV1".into()));
                }
                _ => {}
            }
            
            reader.seek(SeekFrom::Start(pos + inner_size))?;
        }
        
        Ok(())
    }
}

// Use shared entry_to_attr from crate::utils
use crate::utils::entry_to_attr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_heic() {
        let parser = HeicParser;
        let header = [
            0x00, 0x00, 0x00, 0x18,
            b'f', b't', b'y', b'p',
            b'h', b'e', b'i', b'c',
        ];
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn detect_avif() {
        let parser = HeicParser;
        let header = [
            0x00, 0x00, 0x00, 0x18,
            b'f', b't', b'y', b'p',
            b'a', b'v', b'i', b'f',
        ];
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn detect_heif() {
        let parser = HeicParser;
        let header = [
            0x00, 0x00, 0x00, 0x18,
            b'f', b't', b'y', b'p',
            b'm', b'i', b'f', b'1',
        ];
        assert!(parser.can_parse(&header));
    }

    #[test]
    fn reject_jpeg() {
        let parser = HeicParser;
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
    }

    #[test]
    fn reject_mp4() {
        let parser = HeicParser;
        let header = [
            0x00, 0x00, 0x00, 0x18,
            b'f', b't', b'y', b'p',
            b'i', b's', b'o', b'm',
        ];
        assert!(!parser.can_parse(&header));
    }
}

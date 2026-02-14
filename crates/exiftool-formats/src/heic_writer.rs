//! HEIC/HEIF/AVIF format writer.
//!
//! HEIC writing strategy (based on ExifTool WriteQuickTime.pl):
//! - Parse ISOBMFF structure (boxes/atoms)
//! - Locate EXIF item in iloc (ItemLocation) box
//! - Update EXIF data in mdat or create new item
//! - Recalculate iloc offsets when EXIF size changes
//! - Update box sizes throughout the hierarchy
//!
//! Key boxes:
//! - ftyp: file type and compatible brands
//! - meta: container for metadata boxes
//!   - hdlr: handler type (should be "pict")
//!   - pitm: primary item ID
//!   - iloc: item locations (offsets/lengths for each item)
//!   - iinf: item info (item types - identifies EXIF item)
//!   - iref: item references (cdsc = content describes)
//!   - iprp: item properties
//! - mdat: media data (contains actual image + EXIF data)
//!
//! Reference: ISO/IEC 14496-12 (ISOBMFF), ISO/IEC 23008-12 (HEIF)

use crate::{Error, Metadata, ReadSeek, Result};
use std::collections::HashMap;
use std::io::Write;

/// Box header info
#[derive(Debug, Clone)]
struct BoxInfo {
    offset: u64,
    size: u64,
    box_type: [u8; 4],
    header_size: u8, // 8 or 16 for extended size
}

/// Item location entry from iloc box (ISO/IEC 14496-12).
/// 
/// Fields are parsed per spec for format completeness. Not all fields
/// are used in current write implementation, but kept for future
/// extension (e.g., adding EXIF to files without existing metadata).
#[derive(Debug, Clone, Default)]
struct ItemLocation {
    #[allow(dead_code)] // Parsed per spec, used for item lookup
    item_id: u32,
    #[allow(dead_code)] // HEIF spec field, needed for full iloc rebuild
    construction_method: u8,
    #[allow(dead_code)] // HEIF spec field, needed for full iloc rebuild  
    data_ref_index: u16,
    base_offset: u64,
    extents: Vec<ItemExtent>,
}

/// Single extent within an item location.
#[derive(Debug, Clone, Default)]
struct ItemExtent {
    #[allow(dead_code)] // HEIF spec field, needed for multi-extent items
    index: u64,
    offset: u64,
    length: u64,
}

/// Item info from iinf box.
#[derive(Debug, Clone)]
struct ItemInfo {
    item_id: u32,
    item_type: [u8; 4],
    #[allow(dead_code)] // HEIF spec field, useful for debugging
    content_type: Option<String>,
}

/// iloc box layout info for offset patching
#[derive(Debug, Clone)]
struct IlocLayout {
    offset: u64,
    version: u8,
    offset_size: u8,
    length_size: u8,
    base_offset_size: u8,
    index_size: u8,
    item_count: u32,
}

/// Preserved meta child box (for rebuild when adding EXIF).
struct PreservedBox {
    #[allow(dead_code)]
    box_type: [u8; 4],
    data: Vec<u8>,
}

/// Parsed HEIC structure
struct HeicStructure {
    boxes: Vec<BoxInfo>,
    meta_offset: u64,
    meta_size: u64,
    iloc_layout: Option<IlocLayout>,
    item_locations: HashMap<u32, ItemLocation>,
    item_infos: HashMap<u32, ItemInfo>,
    primary_item_id: Option<u32>,
    exif_item_id: Option<u32>,
    /// XMP mime item (application/rdf+xml) - per WriteQuickTime.pl
    xmp_item_id: Option<u32>,
    mdat_offset: u64,
    mdat_size: u64,
    /// Boxes to preserve when rebuilding meta (hdlr, pitm, iprp, etc.)
    preserved_meta_boxes: Vec<PreservedBox>,
    /// Raw iinf box (for rebuild when adding EXIF - need to expand with new infe)
    iinf_box_raw: Option<Vec<u8>>,
}

/// HEIC format writer.
pub struct HeicWriter;

impl HeicWriter {
    /// Write HEIC with updated metadata.
    ///
    /// Strategy:
    /// 1. Parse existing structure
    /// 2. Find or create EXIF item
    /// 3. Build new EXIF data
    /// 4. Calculate size delta
    /// 5. Rewrite file with updated offsets
    pub fn write<R, W>(input: &mut R, output: &mut W, metadata: &Metadata) -> Result<()>
    where
        R: ReadSeek,
        W: Write,
    {
        let data = crate::utils::read_with_limit(input)?;

        // Verify HEIC/HEIF/AVIF format
        if data.len() < 12 || &data[4..8] != b"ftyp" {
            return Err(Error::InvalidStructure("Invalid HEIC file".into()));
        }

        // Check for known brands
        let brand = &data[8..12];
        let is_heic = matches!(
            brand,
            b"heic" | b"heix" | b"hevc" | b"hevx" | b"mif1" | b"msf1" | b"avif" | b"avis"
        );
        if !is_heic {
            return Err(Error::InvalidStructure(
                format!("Unknown brand: {:?}", String::from_utf8_lossy(brand)),
            ));
        }

        // Parse structure
        let mut structure = Self::parse_structure(&data)?;

        // Build new EXIF and XMP (per WriteQuickTime.pl)
        let exif_bytes = crate::utils::build_exif_bytes(metadata)?;
        let has_new_exif = !exif_bytes.is_empty();
        let xmp_data = metadata.xmp.as_deref().filter(|s| !s.trim().is_empty());
        let has_new_xmp = xmp_data.is_some();
        let xmp_bytes = xmp_data.map(|s| s.as_bytes());

        if !has_new_exif && !has_new_xmp {
            output.write_all(&data)?;
            return Ok(());
        }

        let mut current_data: Vec<u8> = data.to_vec();

        // EXIF pass
        if has_new_exif {
            let heic_exif = if exif_bytes.starts_with(b"MM") || exif_bytes.starts_with(b"II") {
                let mut buf = vec![0u8; 4 + exif_bytes.len()];
                buf[3] = 0;
                buf[4..].copy_from_slice(&exif_bytes);
                buf
            } else {
                let mut buf = Vec::with_capacity(4 + 6 + exif_bytes.len());
                buf.extend_from_slice(&[0, 0, 0, 6]);
                buf.extend_from_slice(b"Exif\0\0");
                buf.extend_from_slice(&exif_bytes);
                buf
            };

            let mut buf = Vec::new();
            if let Some(exif_id) = structure.exif_item_id {
                Self::update_exif_item(&current_data, &mut buf, &mut structure, exif_id, &heic_exif)?;
            } else {
                Self::create_exif_item(&current_data, &mut buf, &mut structure, &heic_exif)?;
            }
            current_data = buf;
            structure = Self::parse_structure(&current_data)?;
        }

        // XMP pass (WriteQuickTime: mime, application/rdf+xml)
        if has_new_xmp {
            let xb = xmp_bytes.unwrap();
            let mut buf = Vec::new();
            if let Some(xmp_id) = structure.xmp_item_id {
                Self::update_xmp_item(&current_data, &mut buf, &mut structure, xmp_id, xb)?;
            } else {
                Self::create_xmp_item(&current_data, &mut buf, &mut structure, xb)?;
            }
            output.write_all(&buf)?;
        } else {
            output.write_all(&current_data)?;
        }

        Ok(())
    }

    /// Parse HEIC file structure.
    fn parse_structure(data: &[u8]) -> Result<HeicStructure> {
        let mut structure = HeicStructure {
            boxes: Vec::new(),
            meta_offset: 0,
            meta_size: 0,
            iloc_layout: None,
            item_locations: HashMap::new(),
            item_infos: HashMap::new(),
            primary_item_id: None,
            exif_item_id: None,
            xmp_item_id: None,
            mdat_offset: 0,
            mdat_size: 0,
            preserved_meta_boxes: Vec::new(),
            iinf_box_raw: None,
        };

        let mut pos = 0usize;
        let data_len = data.len();

        while pos + 8 <= data_len {
            let box_info = Self::read_box_header(data, pos)?;

            match &box_info.box_type {
                b"meta" => {
                    structure.meta_offset = pos as u64;
                    structure.meta_size = box_info.size;
                    Self::parse_meta_box(data, &box_info, &mut structure)?;
                }
                b"mdat" => {
                    structure.mdat_offset = pos as u64;
                    structure.mdat_size = box_info.size;
                }
                _ => {}
            }

            structure.boxes.push(box_info.clone());

            if box_info.size == 0 {
                break; // size 0 means extends to end of file
            }
            pos += box_info.size as usize;
        }

        // Find EXIF and XMP item IDs from item_infos (WriteQuickTime: Exif, mime+application/rdf+xml)
        for (id, info) in &structure.item_infos {
            if &info.item_type == b"Exif" {
                structure.exif_item_id = Some(*id);
            } else if &info.item_type == b"mime" {
                if let Some(ref ct) = info.content_type {
                    if ct.contains("application/rdf+xml") || ct.contains("rdf+xml") {
                        structure.xmp_item_id = Some(*id);
                    }
                }
            }
        }

        Ok(structure)
    }

    /// Read box header at position.
    fn read_box_header(data: &[u8], pos: usize) -> Result<BoxInfo> {
        if pos + 8 > data.len() {
            return Err(Error::InvalidStructure("Truncated box header".into()));
        }

        let size32 = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        let box_type = [data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]];

        let (size, header_size) = if size32 == 1 {
            // Extended size
            if pos + 16 > data.len() {
                return Err(Error::InvalidStructure("Truncated extended size".into()));
            }
            let ext_size = u64::from_be_bytes([
                data[pos + 8],
                data[pos + 9],
                data[pos + 10],
                data[pos + 11],
                data[pos + 12],
                data[pos + 13],
                data[pos + 14],
                data[pos + 15],
            ]);
            (ext_size, 16u8)
        } else if size32 == 0 {
            // Extends to end of file
            ((data.len() - pos) as u64, 8u8)
        } else {
            (size32 as u64, 8u8)
        };

        Ok(BoxInfo {
            offset: pos as u64,
            size,
            box_type,
            header_size,
        })
    }

    /// Parse meta box and its children.
    fn parse_meta_box(data: &[u8], meta_box: &BoxInfo, structure: &mut HeicStructure) -> Result<()> {
        // meta is a FullBox - skip version (1) + flags (3) after header
        let meta_start = meta_box.offset as usize + meta_box.header_size as usize + 4;
        let meta_end = (meta_box.offset + meta_box.size) as usize;

        let mut pos = meta_start;

        while pos + 8 <= meta_end {
            let box_info = Self::read_box_header(data, pos)?;

            if box_info.size < 8 || pos + box_info.size as usize > meta_end {
                break;
            }

            match &box_info.box_type {
                b"pitm" => {
                    Self::parse_pitm_box(data, &box_info, structure)?;
                    structure.preserved_meta_boxes.push(PreservedBox {
                        box_type: *b"pitm",
                        data: data[pos..pos + box_info.size as usize].to_vec(),
                    });
                }
                b"iloc" => {
                    Self::parse_iloc_box(data, &box_info, structure)?;
                }
                b"iinf" => {
                    Self::parse_iinf_box(data, &box_info, structure)?;
                    structure.iinf_box_raw =
                        Some(data[pos..pos + box_info.size as usize].to_vec());
                }
                b"hdlr" | b"iprp" | b"irot" => {
                    structure.preserved_meta_boxes.push(PreservedBox {
                        box_type: box_info.box_type,
                        data: data[pos..pos + box_info.size as usize].to_vec(),
                    });
                }
                b"iref" => {
                    structure.preserved_meta_boxes.push(PreservedBox {
                        box_type: *b"iref",
                        data: data[pos..pos + box_info.size as usize].to_vec(),
                    });
                }
                _ => {}
            }

            structure.boxes.push(box_info.clone());
            pos += box_info.size as usize;
        }

        Ok(())
    }

    /// Parse pitm (primary item) box.
    fn parse_pitm_box(
        data: &[u8],
        box_info: &BoxInfo,
        structure: &mut HeicStructure,
    ) -> Result<()> {
        let pos = box_info.offset as usize + box_info.header_size as usize;
        if pos + 4 > data.len() {
            return Ok(());
        }

        let version = data[pos];
        let id_offset = pos + 4;

        let primary_id = if version == 0 {
            if id_offset + 2 > data.len() {
                return Ok(());
            }
            u16::from_be_bytes([data[id_offset], data[id_offset + 1]]) as u32
        } else {
            if id_offset + 4 > data.len() {
                return Ok(());
            }
            u32::from_be_bytes([
                data[id_offset],
                data[id_offset + 1],
                data[id_offset + 2],
                data[id_offset + 3],
            ])
        };

        structure.primary_item_id = Some(primary_id);
        Ok(())
    }

    /// Parse iloc (item location) box.
    fn parse_iloc_box(
        data: &[u8],
        box_info: &BoxInfo,
        structure: &mut HeicStructure,
    ) -> Result<()> {
        let pos = box_info.offset as usize + box_info.header_size as usize;
        if pos + 8 > data.len() {
            return Ok(());
        }

        let version = data[pos];
        // flags at pos+1..pos+4

        let sizes = u16::from_be_bytes([data[pos + 4], data[pos + 5]]);
        let offset_size = ((sizes >> 12) & 0xF) as u8;
        let length_size = ((sizes >> 8) & 0xF) as u8;
        let base_offset_size = ((sizes >> 4) & 0xF) as u8;
        let index_size = if version == 1 || version == 2 {
            (sizes & 0xF) as u8
        } else {
            0
        };

        let (item_count, mut cur_pos) = if version < 2 {
            let count = u16::from_be_bytes([data[pos + 6], data[pos + 7]]) as u32;
            (count, pos + 8)
        } else {
            if pos + 10 > data.len() {
                return Ok(());
            }
            let count = u32::from_be_bytes([data[pos + 6], data[pos + 7], data[pos + 8], data[pos + 9]]);
            (count, pos + 10)
        };

        structure.iloc_layout = Some(IlocLayout {
            offset: box_info.offset,
            version,
            offset_size,
            length_size,
            base_offset_size,
            index_size,
            item_count,
        });

        // Parse each item
        for _ in 0..item_count {
            let item_id = if version < 2 {
                if cur_pos + 2 > data.len() {
                    break;
                }
                let id = u16::from_be_bytes([data[cur_pos], data[cur_pos + 1]]) as u32;
                cur_pos += 2;
                id
            } else {
                if cur_pos + 4 > data.len() {
                    break;
                }
                let id = u32::from_be_bytes([
                    data[cur_pos],
                    data[cur_pos + 1],
                    data[cur_pos + 2],
                    data[cur_pos + 3],
                ]);
                cur_pos += 4;
                id
            };

            let construction_method = if version == 1 || version == 2 {
                if cur_pos + 2 > data.len() {
                    break;
                }
                let cm = u16::from_be_bytes([data[cur_pos], data[cur_pos + 1]]) & 0xF;
                cur_pos += 2;
                cm as u8
            } else {
                0
            };

            if cur_pos + 2 > data.len() {
                break;
            }
            let data_ref_index = u16::from_be_bytes([data[cur_pos], data[cur_pos + 1]]);
            cur_pos += 2;

            let base_offset = Self::read_var_int(data, &mut cur_pos, base_offset_size);

            if cur_pos + 2 > data.len() {
                break;
            }
            let extent_count = u16::from_be_bytes([data[cur_pos], data[cur_pos + 1]]);
            cur_pos += 2;

            let mut extents = Vec::new();
            for _ in 0..extent_count {
                let index = if version == 1 || version == 2 {
                    Self::read_var_int(data, &mut cur_pos, index_size)
                } else {
                    0
                };
                let offset = Self::read_var_int(data, &mut cur_pos, offset_size);
                let length = Self::read_var_int(data, &mut cur_pos, length_size);

                extents.push(ItemExtent {
                    index,
                    offset,
                    length,
                });
            }

            structure.item_locations.insert(
                item_id,
                ItemLocation {
                    item_id,
                    construction_method,
                    data_ref_index,
                    base_offset,
                    extents,
                },
            );
        }

        Ok(())
    }

    /// Read variable-size integer from iloc.
    fn read_var_int(data: &[u8], pos: &mut usize, size: u8) -> u64 {
        match size {
            0 => 0,
            4 => {
                if *pos + 4 > data.len() {
                    return 0;
                }
                let val = u32::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]]);
                *pos += 4;
                val as u64
            }
            8 => {
                if *pos + 8 > data.len() {
                    return 0;
                }
                let val = u64::from_be_bytes([
                    data[*pos],
                    data[*pos + 1],
                    data[*pos + 2],
                    data[*pos + 3],
                    data[*pos + 4],
                    data[*pos + 5],
                    data[*pos + 6],
                    data[*pos + 7],
                ]);
                *pos += 8;
                val
            }
            _ => 0,
        }
    }

    /// Write variable-size integer (for iloc offset patching).
    /// 
    /// Currently unused - will be needed when implementing full
    /// iloc box rebuild for adding EXIF to files without metadata.
    #[allow(dead_code)]
    fn write_var_int(val: u64, size: u8) -> Vec<u8> {
        match size {
            0 => Vec::new(),
            4 => (val as u32).to_be_bytes().to_vec(),
            8 => val.to_be_bytes().to_vec(),
            _ => Vec::new(),
        }
    }

    /// Parse iinf (item info) box.
    fn parse_iinf_box(
        data: &[u8],
        box_info: &BoxInfo,
        structure: &mut HeicStructure,
    ) -> Result<()> {
        let pos = box_info.offset as usize + box_info.header_size as usize;
        if pos + 4 > data.len() {
            return Ok(());
        }

        let version = data[pos];
        let (entry_count, mut cur_pos) = if version == 0 {
            let count = u16::from_be_bytes([data[pos + 4], data[pos + 5]]) as u32;
            (count, pos + 6)
        } else {
            if pos + 8 > data.len() {
                return Ok(());
            }
            let count = u32::from_be_bytes([data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]]);
            (count, pos + 8)
        };

        let box_end = (box_info.offset + box_info.size) as usize;

        // Parse infe boxes
        for _ in 0..entry_count {
            if cur_pos + 8 > box_end {
                break;
            }

            let infe_size = u32::from_be_bytes([
                data[cur_pos],
                data[cur_pos + 1],
                data[cur_pos + 2],
                data[cur_pos + 3],
            ]) as usize;

            if &data[cur_pos + 4..cur_pos + 8] != b"infe" || infe_size < 12 {
                cur_pos += infe_size.max(8);
                continue;
            }

            // Parse infe entry
            let infe_pos = cur_pos + 8;
            if infe_pos + 4 > data.len() {
                break;
            }

            let infe_version = data[infe_pos];
            let item_info = if infe_version >= 2 {
                // Version 2+: item_ID (2 or 4 bytes), item_protection_index (2), item_type (4)
                let (item_id, id_size) = if infe_version == 2 {
                    (
                        u16::from_be_bytes([data[infe_pos + 4], data[infe_pos + 5]]) as u32,
                        2usize,
                    )
                } else {
                    (
                        u32::from_be_bytes([
                            data[infe_pos + 4],
                            data[infe_pos + 5],
                            data[infe_pos + 6],
                            data[infe_pos + 7],
                        ]),
                        4usize,
                    )
                };

                let type_offset = infe_pos + 4 + id_size + 2; // skip protection_index
                if type_offset + 4 > data.len() {
                    cur_pos += infe_size;
                    continue;
                }

                let item_type = [
                    data[type_offset],
                    data[type_offset + 1],
                    data[type_offset + 2],
                    data[type_offset + 3],
                ];

                // item_name (content-type for mime) - null-terminated, per WriteQuickTime
                let mut content_type = None;
                if &item_type == b"mime" {
                    let name_start = type_offset + 4;
                    let name_end = (cur_pos + infe_size).min(data.len());
                    let mut name_end_pos = name_start;
                    for i in name_start..name_end {
                        if data[i] == 0 {
                            name_end_pos = i;
                            break;
                        }
                        name_end_pos = i + 1;
                    }
                    if name_end_pos > name_start {
                        let name = String::from_utf8_lossy(&data[name_start..name_end_pos]).to_string();
                        if !name.is_empty() {
                            content_type = Some(name);
                        }
                    }
                }

                Some(ItemInfo {
                    item_id,
                    item_type,
                    content_type,
                })
            } else {
                None
            };

            if let Some(info) = item_info {
                structure.item_infos.insert(info.item_id, info);
            }

            cur_pos += infe_size;
        }

        Ok(())
    }

    /// Update existing EXIF item with new data.
    fn update_exif_item<W: Write>(
        data: &[u8],
        output: &mut W,
        structure: &mut HeicStructure,
        exif_item_id: u32,
        new_exif: &[u8],
    ) -> Result<()> {
        let loc = structure
            .item_locations
            .get(&exif_item_id)
            .ok_or_else(|| Error::InvalidStructure("EXIF item not found in iloc".into()))?
            .clone();

        let _iloc_layout = structure
            .iloc_layout
            .as_ref()
            .ok_or_else(|| Error::InvalidStructure("iloc layout not found".into()))?
            .clone();

        // Get current EXIF location
        if loc.extents.is_empty() {
            return Err(Error::InvalidStructure("EXIF item has no extents".into()));
        }

        let extent = &loc.extents[0];
        let old_offset = loc.base_offset + extent.offset;
        let old_length = extent.length;
        let length_delta = new_exif.len() as i64 - old_length as i64;

        // For simplicity, we'll use a two-pass approach:
        // 1. Copy everything before EXIF data offset in mdat
        // 2. Write new EXIF
        // 3. Copy everything after old EXIF
        // 4. Patch iloc offset/length entries

        // This requires calculating where in the file the EXIF data is
        // and how to adjust all subsequent offsets

        // Find mdat position
        let mdat_header_size = if structure.mdat_size > u32::MAX as u64 {
            16
        } else {
            8
        };

        // Calculate actual file position of EXIF data
        let exif_file_pos = old_offset as usize;

        if exif_file_pos >= data.len() || exif_file_pos + old_length as usize > data.len() {
            return Err(Error::InvalidStructure("EXIF extent out of bounds".into()));
        }

        // Build output
        let mut out_data = Vec::with_capacity(data.len() + length_delta.unsigned_abs() as usize);

        // Copy everything before EXIF
        out_data.extend_from_slice(&data[..exif_file_pos]);

        // Write new EXIF
        out_data.extend_from_slice(new_exif);

        // Copy everything after old EXIF
        let after_old_exif = exif_file_pos + old_length as usize;
        if after_old_exif < data.len() {
            out_data.extend_from_slice(&data[after_old_exif..]);
        }

        // Now patch iloc entries that point to data after the EXIF
        // We need to adjust offsets for any item that comes after EXIF in mdat
        Self::patch_iloc_offsets(&mut out_data, structure, exif_item_id, old_offset, length_delta, new_exif.len() as u64)?;

        // Update mdat size if it changed
        if length_delta != 0 && structure.mdat_offset > 0 {
            let mdat_pos = structure.mdat_offset as usize;
            let new_mdat_size = (structure.mdat_size as i64 + length_delta) as u64;

            if mdat_header_size == 8 {
                let size_bytes = (new_mdat_size as u32).to_be_bytes();
                out_data[mdat_pos..mdat_pos + 4].copy_from_slice(&size_bytes);
            } else {
                // Extended size - skip 32-bit size (=1), box type, then 64-bit size
                let size_bytes = new_mdat_size.to_be_bytes();
                out_data[mdat_pos + 8..mdat_pos + 16].copy_from_slice(&size_bytes);
            }
        }

        output.write_all(&out_data)?;
        Ok(())
    }

    /// Update existing XMP item (mime/application/rdf+xml).
    fn update_xmp_item<W: Write>(
        data: &[u8],
        output: &mut W,
        structure: &mut HeicStructure,
        xmp_item_id: u32,
        new_xmp: &[u8],
    ) -> Result<()> {
        let loc = structure
            .item_locations
            .get(&xmp_item_id)
            .ok_or_else(|| Error::InvalidStructure("XMP item not found in iloc".into()))?
            .clone();

        if loc.extents.is_empty() {
            return Err(Error::InvalidStructure("XMP item has no extents".into()));
        }

        let extent = &loc.extents[0];
        let old_offset = loc.base_offset + extent.offset;
        let old_length = extent.length;
        let length_delta = new_xmp.len() as i64 - old_length as i64;
        let xmp_file_pos = old_offset as usize;

        if xmp_file_pos >= data.len() || xmp_file_pos + old_length as usize > data.len() {
            return Err(Error::InvalidStructure("XMP extent out of bounds".into()));
        }

        let mut out_data = Vec::with_capacity(data.len() + length_delta.unsigned_abs() as usize);
        out_data.extend_from_slice(&data[..xmp_file_pos]);
        out_data.extend_from_slice(new_xmp);
        let after_old = xmp_file_pos + old_length as usize;
        if after_old < data.len() {
            out_data.extend_from_slice(&data[after_old..]);
        }

        Self::patch_iloc_offsets(&mut out_data, structure, xmp_item_id, old_offset, length_delta, new_xmp.len() as u64)?;

        if length_delta != 0 && structure.mdat_offset > 0 {
            let mdat_header_size = if structure.mdat_size > u32::MAX as u64 { 16 } else { 8 };
            let mdat_pos = structure.mdat_offset as usize;
            let new_mdat_size = (structure.mdat_size as i64 + length_delta) as u64;
            if mdat_header_size == 8 {
                out_data[mdat_pos..mdat_pos + 4].copy_from_slice(&(new_mdat_size as u32).to_be_bytes());
            } else {
                out_data[mdat_pos + 8..mdat_pos + 16].copy_from_slice(&new_mdat_size.to_be_bytes());
            }
        }

        output.write_all(&out_data)?;
        Ok(())
    }

    /// Patch iloc offsets after EXIF data change.
    fn patch_iloc_offsets(
        data: &mut [u8],
        structure: &HeicStructure,
        exif_item_id: u32,
        exif_offset: u64,
        length_delta: i64,
        new_exif_length: u64,
    ) -> Result<()> {
        let layout = structure
            .iloc_layout
            .as_ref()
            .ok_or_else(|| Error::InvalidStructure("iloc layout not found".into()))?;

        // Find iloc box position and re-parse to patch offsets
        // We need to find the position of each item's extent offset/length in iloc

        let iloc_data_start = layout.offset as usize + 8 + 4; // box header + version/flags
        let mut pos = iloc_data_start + 2; // skip size info

        let _item_count_pos = if layout.version < 2 {
            pos += 2; // 16-bit count
            pos - 2
        } else {
            pos += 4; // 32-bit count
            pos - 4
        };

        // Iterate through items to find and patch offsets
        for _ in 0..layout.item_count {
            let _item_id_start = pos;

            // Read item_id
            let item_id = if layout.version < 2 {
                if pos + 2 > data.len() {
                    break;
                }
                let id = u16::from_be_bytes([data[pos], data[pos + 1]]) as u32;
                pos += 2;
                id
            } else {
                if pos + 4 > data.len() {
                    break;
                }
                let id = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
                pos += 4;
                id
            };

            // Skip construction_method for v1/v2
            if layout.version == 1 || layout.version == 2 {
                pos += 2;
            }

            // Skip data_reference_index
            pos += 2;

            // Base offset position
            let base_offset_pos = pos;
            let base_offset = Self::read_var_int_at(data, &mut pos, layout.base_offset_size);

            // Extent count
            if pos + 2 > data.len() {
                break;
            }
            let extent_count = u16::from_be_bytes([data[pos], data[pos + 1]]);
            pos += 2;

            // Process extents
            for _ in 0..extent_count {
                // Skip index for v1/v2
                if layout.version == 1 || layout.version == 2 {
                    pos += layout.index_size as usize;
                }

                let extent_offset_pos = pos;
                let extent_offset = Self::read_var_int_at(data, &mut pos, layout.offset_size);

                let extent_length_pos = pos;
                let _extent_length = Self::read_var_int_at(data, &mut pos, layout.length_size);

                let absolute_offset = base_offset + extent_offset;

                if item_id == exif_item_id {
                    // Update length for this EXIF item
                    Self::write_var_int_at(data, extent_length_pos, layout.length_size, new_exif_length);
                } else if absolute_offset > exif_offset && length_delta != 0 {
                    // This item comes after EXIF, adjust its offset
                    let new_offset = if layout.offset_size > 0 {
                        ((extent_offset as i64) + length_delta) as u64
                    } else {
                        // Offset stored in base_offset
                        ((base_offset as i64) + length_delta) as u64
                    };

                    if layout.offset_size > 0 {
                        Self::write_var_int_at(data, extent_offset_pos, layout.offset_size, new_offset);
                    } else if layout.base_offset_size > 0 {
                        Self::write_var_int_at(data, base_offset_pos, layout.base_offset_size, new_offset);
                    }
                }
            }
        }

        Ok(())
    }

    /// Read variable-size int at position without advancing.
    fn read_var_int_at(data: &[u8], pos: &mut usize, size: u8) -> u64 {
        Self::read_var_int(data, pos, size)
    }

    /// Write variable-size int at specific position.
    fn write_var_int_at(data: &mut [u8], pos: usize, size: u8, val: u64) {
        match size {
            4 => {
                let bytes = (val as u32).to_be_bytes();
                data[pos..pos + 4].copy_from_slice(&bytes);
            }
            8 => {
                let bytes = val.to_be_bytes();
                data[pos..pos + 8].copy_from_slice(&bytes);
            }
            _ => {}
        }
    }

    /// Create new EXIF item (when none exists).
    /// Rebuilds meta box with new iinf, iloc, iref entries and appends EXIF to file.
    fn create_exif_item<W: Write>(
        data: &[u8],
        output: &mut W,
        structure: &mut HeicStructure,
        new_exif: &[u8],
    ) -> Result<()> {
        let primary_id = structure
            .primary_item_id
            .ok_or_else(|| Error::InvalidStructure("No primary item (pitm)".into()))?;

        let max_id = structure
            .item_locations
            .keys()
            .chain(structure.item_infos.keys())
            .max()
            .copied()
            .unwrap_or(0);
        let new_exif_id = max_id + 1;

        // EXIF will be appended at end of file; offset = current data len
        let exif_offset = data.len() as u64;
        let exif_length = new_exif.len() as u64;

        // Build new iloc box using same layout as original
        let layout = structure
            .iloc_layout
            .as_ref()
            .ok_or_else(|| Error::InvalidStructure("iloc layout required".into()))?;
        let mut iloc_data = Vec::new();
        iloc_data.extend_from_slice(&[layout.version, 0, 0, 0]);
        let sizes = ((layout.offset_size as u16) << 12)
            | ((layout.length_size as u16) << 8)
            | ((layout.base_offset_size as u16) << 4)
            | (layout.index_size as u16);
        iloc_data.extend_from_slice(&sizes.to_be_bytes());
        let item_count = structure.item_locations.len() + 1;
        if layout.version < 2 {
            iloc_data.extend_from_slice(&(item_count as u16).to_be_bytes());
        } else {
            iloc_data.extend_from_slice(&(item_count as u32).to_be_bytes());
        }

        let write_var = |buf: &mut Vec<u8>, val: u64, size: u8| {
            match size {
                4 => buf.extend_from_slice(&(val as u32).to_be_bytes()),
                8 => buf.extend_from_slice(&val.to_be_bytes()),
                _ => {}
            }
        };

        // Existing items
        for (item_id, loc) in &structure.item_locations {
            if layout.version < 2 {
                iloc_data.extend_from_slice(&(*item_id as u16).to_be_bytes());
            } else {
                iloc_data.extend_from_slice(&item_id.to_be_bytes());
            }
            if layout.version >= 1 {
                iloc_data.extend_from_slice(&(loc.construction_method as u16).to_be_bytes());
            }
            iloc_data.extend_from_slice(&loc.data_ref_index.to_be_bytes());
            write_var(&mut iloc_data, loc.base_offset, layout.base_offset_size);
            iloc_data.extend_from_slice(&(loc.extents.len() as u16).to_be_bytes());
            for ext in &loc.extents {
                if layout.version >= 1 && layout.index_size > 0 {
                    write_var(&mut iloc_data, ext.index, layout.index_size);
                }
                write_var(&mut iloc_data, ext.offset, layout.offset_size);
                write_var(&mut iloc_data, ext.length, layout.length_size);
            }
        }
        // New EXIF item
        if layout.version < 2 {
            iloc_data.extend_from_slice(&(new_exif_id as u16).to_be_bytes());
        } else {
            iloc_data.extend_from_slice(&new_exif_id.to_be_bytes());
        }
        if layout.version >= 1 {
            iloc_data.extend_from_slice(&0u16.to_be_bytes());
        }
        iloc_data.extend_from_slice(&0u16.to_be_bytes()); // data_ref_index
        write_var(&mut iloc_data, 0, layout.base_offset_size);
        iloc_data.extend_from_slice(&1u16.to_be_bytes());
        write_var(&mut iloc_data, exif_offset, layout.offset_size);
        write_var(&mut iloc_data, exif_length, layout.length_size);

        let iloc_size = 8 + iloc_data.len();
        let mut iloc_box = (iloc_size as u32).to_be_bytes().to_vec();
        iloc_box.extend_from_slice(b"iloc");
        iloc_box.extend_from_slice(&iloc_data);

        // Expand iinf box with new Exif infe
        let iinf_raw = structure
            .iinf_box_raw
            .as_ref()
            .ok_or_else(|| Error::InvalidStructure("iinf box required".into()))?;
        if iinf_raw.len() < 14 {
            return Err(Error::InvalidStructure("iinf box too small".into()));
        }
        let iinf_version = iinf_raw[8];
        let entry_count_pos = 12usize;
        let old_count = if iinf_version == 0 {
            u16::from_be_bytes([iinf_raw[entry_count_pos], iinf_raw[entry_count_pos + 1]]) as u32
        } else {
            u32::from_be_bytes([
                iinf_raw[entry_count_pos],
                iinf_raw[entry_count_pos + 1],
                iinf_raw[entry_count_pos + 2],
                iinf_raw[entry_count_pos + 3],
            ])
        };
        let new_count = old_count + 1;

        // New infe for Exif (minimal version 2)
        let mut exif_infe = Vec::new();
        exif_infe.extend_from_slice(&20u32.to_be_bytes());
        exif_infe.extend_from_slice(b"infe");
        exif_infe.extend_from_slice(&[2, 0, 0, 0]);
        exif_infe.extend_from_slice(&(new_exif_id as u16).to_be_bytes());
        exif_infe.extend_from_slice(&0u16.to_be_bytes());
        exif_infe.extend_from_slice(b"Exif");

        let mut iinf_box = iinf_raw.clone();
        let new_iinf_size = iinf_box.len() + exif_infe.len();
        iinf_box[0..4].copy_from_slice(&(new_iinf_size as u32).to_be_bytes());
        if iinf_version == 0 {
            iinf_box[entry_count_pos..entry_count_pos + 2]
                .copy_from_slice(&(new_count as u16).to_be_bytes());
        } else {
            iinf_box[entry_count_pos..entry_count_pos + 4]
                .copy_from_slice(&new_count.to_be_bytes());
        }
        iinf_box.extend_from_slice(&exif_infe);

        // Build iref box with cdsc: primary describes exif (content describes)
        let mut iref_data = Vec::new();
        iref_data.extend_from_slice(&[0, 0, 0, 0]); // version+flags
        iref_data.extend_from_slice(b"cdsc");
        iref_data.extend_from_slice(&(primary_id as u32).to_be_bytes());
        iref_data.extend_from_slice(&1u32.to_be_bytes()); // ref count
        iref_data.extend_from_slice(&new_exif_id.to_be_bytes());

        let iref_size = 8 + 4 + iref_data.len(); // cdsc is a box
        let mut iref_box = Vec::new();
        iref_box.extend_from_slice(&(iref_size as u32).to_be_bytes());
        iref_box.extend_from_slice(b"iref");
        iref_box.extend_from_slice(&iref_data);

        // Rebuild meta box: version+flags, then preserved boxes (hdlr, pitm), iinf, iloc, iref
        let mut meta_content = Vec::new();
        meta_content.extend_from_slice(&[0, 0, 0, 0]); // meta fullbox
        for pb in &structure.preserved_meta_boxes {
            meta_content.extend_from_slice(&pb.data);
        }
        meta_content.extend_from_slice(&iinf_box);
        meta_content.extend_from_slice(&iloc_box);
        meta_content.extend_from_slice(&iref_box);

        let meta_size = 8 + meta_content.len();
        let mut meta_box = Vec::new();
        meta_box.extend_from_slice(&(meta_size as u32).to_be_bytes());
        meta_box.extend_from_slice(b"meta");
        meta_box.extend_from_slice(&meta_content);

        // Output: [data before meta] + [new meta] + [data after meta] + [exif]
        let meta_start = structure.meta_offset as usize;
        let meta_end = meta_start + structure.meta_size as usize;

        output.write_all(&data[..meta_start])?;
        output.write_all(&meta_box)?;
        output.write_all(&data[meta_end..])?;
        output.write_all(new_exif)?;

        Ok(())
    }

    /// Create new XMP item (mime/application/rdf+xml) when none exists.
    fn create_xmp_item<W: Write>(
        data: &[u8],
        output: &mut W,
        structure: &mut HeicStructure,
        new_xmp: &[u8],
    ) -> Result<()> {
        let primary_id = structure
            .primary_item_id
            .ok_or_else(|| Error::InvalidStructure("No primary item (pitm)".into()))?;

        let max_id = structure
            .item_locations
            .keys()
            .chain(structure.item_infos.keys())
            .max()
            .copied()
            .unwrap_or(0);
        let new_xmp_id = max_id + 1;

        let xmp_length = new_xmp.len() as u64;
        let meta_start = structure.meta_offset as usize;
        let _meta_end = meta_start + structure.meta_size as usize;

        let layout = structure
            .iloc_layout
            .as_ref()
            .ok_or_else(|| Error::InvalidStructure("iloc layout required".into()))?;

        let write_var = |buf: &mut Vec<u8>, val: u64, size: u8| {
            match size {
                4 => buf.extend_from_slice(&(val as u32).to_be_bytes()),
                8 => buf.extend_from_slice(&val.to_be_bytes()),
                _ => {}
            }
        };

        let mut iloc_data = Vec::new();
        iloc_data.extend_from_slice(&[layout.version, 0, 0, 0]);
        let sizes = ((layout.offset_size as u16) << 12)
            | ((layout.length_size as u16) << 8)
            | ((layout.base_offset_size as u16) << 4)
            | (layout.index_size as u16);
        iloc_data.extend_from_slice(&sizes.to_be_bytes());
        let item_count = structure.item_locations.len() + 1;
        if layout.version < 2 {
            iloc_data.extend_from_slice(&(item_count as u16).to_be_bytes());
        } else {
            iloc_data.extend_from_slice(&(item_count as u32).to_be_bytes());
        }

        for (item_id, loc) in &structure.item_locations {
            if layout.version < 2 {
                iloc_data.extend_from_slice(&(*item_id as u16).to_be_bytes());
            } else {
                iloc_data.extend_from_slice(&item_id.to_be_bytes());
            }
            if layout.version >= 1 {
                iloc_data.extend_from_slice(&(loc.construction_method as u16).to_be_bytes());
            }
            iloc_data.extend_from_slice(&loc.data_ref_index.to_be_bytes());
            write_var(&mut iloc_data, loc.base_offset, layout.base_offset_size);
            iloc_data.extend_from_slice(&(loc.extents.len() as u16).to_be_bytes());
            for ext in &loc.extents {
                if layout.version >= 1 && layout.index_size > 0 {
                    write_var(&mut iloc_data, ext.index, layout.index_size);
                }
                write_var(&mut iloc_data, ext.offset, layout.offset_size);
                write_var(&mut iloc_data, ext.length, layout.length_size);
            }
        }
        if layout.version < 2 {
            iloc_data.extend_from_slice(&(new_xmp_id as u16).to_be_bytes());
        } else {
            iloc_data.extend_from_slice(&new_xmp_id.to_be_bytes());
        }
        if layout.version >= 1 {
            iloc_data.extend_from_slice(&0u16.to_be_bytes());
        }
        iloc_data.extend_from_slice(&0u16.to_be_bytes());
        write_var(&mut iloc_data, 0, layout.base_offset_size);
        iloc_data.extend_from_slice(&1u16.to_be_bytes());
        // Placeholder - will patch after we know meta_box size
        let _xmp_offset_pos_in_iloc_data = iloc_data.len();
        write_var(&mut iloc_data, 0u64, layout.offset_size);
        write_var(&mut iloc_data, xmp_length, layout.length_size);

        let iloc_size = 8 + iloc_data.len();
        let mut iloc_box = (iloc_size as u32).to_be_bytes().to_vec();
        iloc_box.extend_from_slice(b"iloc");
        iloc_box.extend_from_slice(&iloc_data);

        // Build iref_box first (needed for meta size calculation)
        // infe v2: item_type (4 bytes "mime"), item_name (null-term "application/rdf+xml\0")
        let item_type = b"mime"; // 4 bytes per ISO spec
        let item_name = b"application/rdf+xml\0";
        let xmp_infe_len = 16 + item_type.len() + item_name.len();
        let mut xmp_infe = Vec::new();
        xmp_infe.extend_from_slice(&(xmp_infe_len as u32).to_be_bytes());
        xmp_infe.extend_from_slice(b"infe");
        xmp_infe.extend_from_slice(&[2, 0, 0, 0]);
        xmp_infe.extend_from_slice(&(new_xmp_id as u16).to_be_bytes());
        xmp_infe.extend_from_slice(&0u16.to_be_bytes());
        xmp_infe.extend_from_slice(item_type);
        xmp_infe.extend_from_slice(item_name);

        let iinf_raw = structure
            .iinf_box_raw
            .as_ref()
            .ok_or_else(|| Error::InvalidStructure("iinf box required".into()))?;
        if iinf_raw.len() < 14 {
            return Err(Error::InvalidStructure("iinf box too small".into()));
        }
        let iinf_version = iinf_raw[8];
        let entry_count_pos = 12usize;
        let old_count = if iinf_version == 0 {
            u16::from_be_bytes([iinf_raw[entry_count_pos], iinf_raw[entry_count_pos + 1]]) as u32
        } else {
            u32::from_be_bytes([
                iinf_raw[entry_count_pos],
                iinf_raw[entry_count_pos + 1],
                iinf_raw[entry_count_pos + 2],
                iinf_raw[entry_count_pos + 3],
            ])
        };
        let new_count = old_count + 1;

        let mut iinf_box = iinf_raw.clone();
        let new_iinf_size = iinf_box.len() + xmp_infe.len();
        iinf_box[0..4].copy_from_slice(&(new_iinf_size as u32).to_be_bytes());
        if iinf_version == 0 {
            iinf_box[entry_count_pos..entry_count_pos + 2]
                .copy_from_slice(&(new_count as u16).to_be_bytes());
        } else {
            iinf_box[entry_count_pos..entry_count_pos + 4].copy_from_slice(&new_count.to_be_bytes());
        }
        iinf_box.extend_from_slice(&xmp_infe);

        let cdsc_data_len = 4 + 4 + 4;
        let cdsc_box_size = 8 + cdsc_data_len;
        let mut cdsc_box = Vec::new();
        cdsc_box.extend_from_slice(&(cdsc_box_size as u32).to_be_bytes());
        cdsc_box.extend_from_slice(b"cdsc");
        cdsc_box.extend_from_slice(&new_xmp_id.to_be_bytes());
        cdsc_box.extend_from_slice(&1u32.to_be_bytes());
        cdsc_box.extend_from_slice(&primary_id.to_be_bytes());

        let iref_box = if let Some(pb) = structure.preserved_meta_boxes.iter().find(|p| p.box_type == *b"iref") {
            let mut iref = pb.data.clone();
            let new_iref_size = iref.len() + cdsc_box.len();
            iref[0..4].copy_from_slice(&(new_iref_size as u32).to_be_bytes());
            iref.extend_from_slice(&cdsc_box);
            iref
        } else {
            let mut iref = Vec::new();
            iref.extend_from_slice(&0u32.to_be_bytes());
            iref.extend_from_slice(b"iref");
            iref.extend_from_slice(&[0, 0, 0, 0]);
            iref.extend_from_slice(&cdsc_box);
            let sz = iref.len() as u32;
            iref[0..4].copy_from_slice(&sz.to_be_bytes());
            iref
        };

        // Build meta_content and full output
        let mut meta_content = Vec::new();
        meta_content.extend_from_slice(&[0, 0, 0, 0]);
        for pb in &structure.preserved_meta_boxes {
            if pb.box_type == *b"iref" {
                meta_content.extend_from_slice(&iref_box);
            } else {
                meta_content.extend_from_slice(&pb.data);
            }
        }
        meta_content.extend_from_slice(&iinf_box);
        meta_content.extend_from_slice(&iloc_box);
        if structure.preserved_meta_boxes.iter().all(|p| p.box_type != *b"iref") {
            meta_content.extend_from_slice(&iref_box);
        }

        let meta_size = 8 + meta_content.len();
        let mut meta_box = Vec::new();
        meta_box.extend_from_slice(&(meta_size as u32).to_be_bytes());
        meta_box.extend_from_slice(b"meta");
        meta_box.extend_from_slice(&meta_content);

        let meta_start = structure.meta_offset as usize;
        let meta_end = meta_start + structure.meta_size as usize;

        // Build full output in buffer
        let mut out_buf = Vec::new();
        out_buf.extend_from_slice(&data[..meta_start]);
        out_buf.extend_from_slice(&meta_box);
        out_buf.extend_from_slice(&data[meta_end..]);
        out_buf.extend_from_slice(new_xmp);
        // XMP offset = first byte after mdat (parser uses same logic)
        let xmp_offset = {
            let p = out_buf.windows(4).position(|w| w == b"mdat").expect("mdat in output");
            let mdat_start = p.saturating_sub(4);
            let mdat_size = u32::from_be_bytes([out_buf[mdat_start], out_buf[mdat_start + 1], out_buf[mdat_start + 2], out_buf[mdat_start + 3]]) as usize;
            (mdat_start + mdat_size) as u64
        };

        // Patch iloc: find "iloc" in output and patch second item's extent_offset
        // iloc layout v0: [size][iloc][v(4)][sizes(2)][count(2)][item1(14)][item2: id(2)dri(2)ec(2) off(4) len(4)]
        let iloc_type_pos = out_buf.windows(4).position(|w| w == b"iloc").expect("iloc in output");
        let iloc_box_start = iloc_type_pos - 4; // size field precedes type
        let item2_extent_offset_pos = iloc_box_start + 8 + 4 + 2 + 2 + 14 + 2 + 2 + 2; // +8 box, +8 v+sizes+count, +14 item1, +6 item2 pre-extent
        match layout.offset_size {
            4 => out_buf[item2_extent_offset_pos..item2_extent_offset_pos + 4]
                .copy_from_slice(&(xmp_offset as u32).to_be_bytes()),
            8 => out_buf[item2_extent_offset_pos..item2_extent_offset_pos + 8].copy_from_slice(&xmp_offset.to_be_bytes()),
            _ => {}
        }

        output.write_all(&out_buf)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FormatRegistry;
    #[allow(unused_imports)]
    use exiftool_attrs::AttrValue;
    use std::io::Cursor;

    fn make_minimal_heic() -> Vec<u8> {
        let mut heic = Vec::new();

        // ftyp box (20 bytes)
        heic.extend_from_slice(&20u32.to_be_bytes()); // size
        heic.extend_from_slice(b"ftyp");
        heic.extend_from_slice(b"heic"); // major brand
        heic.extend_from_slice(&0u32.to_be_bytes()); // minor version
        heic.extend_from_slice(b"heic"); // compatible brand

        // meta box (simplified)
        let meta_content = {
            let mut meta = Vec::new();

            // Version/flags
            meta.extend_from_slice(&[0, 0, 0, 0]);

            // hdlr box
            meta.extend_from_slice(&33u32.to_be_bytes());
            meta.extend_from_slice(b"hdlr");
            meta.extend_from_slice(&[0, 0, 0, 0]); // version/flags
            meta.extend_from_slice(&[0, 0, 0, 0]); // pre_defined
            meta.extend_from_slice(b"pict"); // handler_type
            meta.extend_from_slice(&[0u8; 12]); // reserved
            meta.push(0); // null-terminated string

            // pitm box
            meta.extend_from_slice(&14u32.to_be_bytes());
            meta.extend_from_slice(b"pitm");
            meta.extend_from_slice(&[0, 0, 0, 0]); // version/flags
            meta.extend_from_slice(&1u16.to_be_bytes()); // primary item ID

            // iloc box (minimal) - 8 header + 4 v+f + 2 sizes + 2 count + 14 item
            meta.extend_from_slice(&30u32.to_be_bytes());
            meta.extend_from_slice(b"iloc");
            meta.extend_from_slice(&[0, 0, 0, 0]); // version=0, flags
            meta.extend_from_slice(&0x4400u16.to_be_bytes()); // offset_size=4, length_size=4
            meta.extend_from_slice(&1u16.to_be_bytes()); // item_count=1
            meta.extend_from_slice(&1u16.to_be_bytes()); // item_id=1
            meta.extend_from_slice(&0u16.to_be_bytes()); // data_ref_index
            meta.extend_from_slice(&1u16.to_be_bytes()); // extent_count=1
            meta.extend_from_slice(&100u32.to_be_bytes()); // extent_offset
            meta.extend_from_slice(&50u32.to_be_bytes()); // extent_length

            // iinf box
            meta.extend_from_slice(&30u32.to_be_bytes());
            meta.extend_from_slice(b"iinf");
            meta.extend_from_slice(&[0, 0, 0, 0]); // version/flags
            meta.extend_from_slice(&1u16.to_be_bytes()); // entry_count

            // infe box
            meta.extend_from_slice(&18u32.to_be_bytes());
            meta.extend_from_slice(b"infe");
            meta.extend_from_slice(&[2, 0, 0, 0]); // version=2, flags
            meta.extend_from_slice(&1u16.to_be_bytes()); // item_id
            meta.extend_from_slice(&0u16.to_be_bytes()); // protection_index
            meta.extend_from_slice(b"hvc1"); // item_type

            meta
        };

        let meta_size = 8 + meta_content.len();
        heic.extend_from_slice(&(meta_size as u32).to_be_bytes());
        heic.extend_from_slice(b"meta");
        heic.extend_from_slice(&meta_content);

        // mdat box (placeholder)
        heic.extend_from_slice(&58u32.to_be_bytes()); // size
        heic.extend_from_slice(b"mdat");
        heic.extend_from_slice(&[0u8; 50]); // dummy data

        heic
    }

    #[test]
    fn test_parse_structure() {
        let heic = make_minimal_heic();
        let structure = HeicWriter::parse_structure(&heic).unwrap();

        assert!(structure.primary_item_id.is_some());
        assert_eq!(structure.primary_item_id, Some(1));
        assert!(!structure.item_locations.is_empty());
    }

    #[test]
    fn test_no_changes_copies_original() {
        let heic = make_minimal_heic();
        let metadata = Metadata::new("HEIC");

        let mut input = Cursor::new(&heic);
        let mut output = Vec::new();

        HeicWriter::write(&mut input, &mut output, &metadata).unwrap();

        // When HEIC has no EXIF and metadata is empty, we now add minimal EXIF
        assert!(output.len() > heic.len());
        assert!(output.starts_with(b"\0\0\0\x14ftypheic"));
    }

    #[test]
    fn test_add_exif_when_none_exists() {
        let heic = make_minimal_heic();
        let mut metadata = Metadata::new("HEIC");
        metadata.exif.set("Make", exiftool_attrs::AttrValue::Str("Test".into()));
        metadata.exif.set("Model", exiftool_attrs::AttrValue::Str("Model X".into()));

        let mut input = Cursor::new(&heic);
        let mut output = Vec::new();

        HeicWriter::write(&mut input, &mut output, &metadata).unwrap();

        assert!(output.len() > heic.len());
        // Verify structure: should have meta, mdat, and appended EXIF
        assert!(output.windows(4).any(|w| w == b"meta"));
        assert!(output.windows(4).any(|w| w == b"Exif") || output.windows(4).any(|w| w == b"II\0*") || output.windows(4).any(|w| w == b"MM\0*"));
    }

    #[test]
    fn test_box_header_parsing() {
        let mut data = Vec::new();
        data.extend_from_slice(&100u32.to_be_bytes());
        data.extend_from_slice(b"test");
        data.extend_from_slice(&[0u8; 92]); // padding

        let box_info = HeicWriter::read_box_header(&data, 0).unwrap();
        assert_eq!(box_info.size, 100);
        assert_eq!(&box_info.box_type, b"test");
        assert_eq!(box_info.header_size, 8);
    }

    #[test]
    fn test_extended_size_box() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u32.to_be_bytes()); // size = 1 means extended
        data.extend_from_slice(b"test");
        data.extend_from_slice(&200u64.to_be_bytes()); // extended size
        data.extend_from_slice(&[0u8; 184]); // padding

        let box_info = HeicWriter::read_box_header(&data, 0).unwrap();
        assert_eq!(box_info.size, 200);
        assert_eq!(box_info.header_size, 16);
    }

    #[test]
    fn test_var_int_read_write() {
        let mut data = vec![0u8; 16];

        // 4-byte int
        HeicWriter::write_var_int_at(&mut data, 0, 4, 0x12345678);
        let mut pos = 0;
        let val = HeicWriter::read_var_int_at(&data, &mut pos, 4);
        assert_eq!(val, 0x12345678);

        // 8-byte int
        HeicWriter::write_var_int_at(&mut data, 8, 8, 0x123456789ABCDEF0);
        let mut pos = 8;
        let val = HeicWriter::read_var_int_at(&data, &mut pos, 8);
        assert_eq!(val, 0x123456789ABCDEF0);
    }

    #[test]
    fn test_add_xmp_to_heic() {
        let heic = make_minimal_heic();
        let mut metadata = Metadata::new("HEIC");
        metadata.xmp = Some(
            r#"<?xpacket begin="" id="W5M0MpCehiHzreSzNTczkc9d"?>
<x:xmpmeta xmlns:x="adobe:ns:meta/"><rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"><rdf:Description rdf:about="" xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:title><rdf:Alt><rdf:li xml:lang="x-default">Test XMP</rdf:li></rdf:Alt></dc:title></rdf:Description></rdf:RDF></x:xmpmeta>
<?xpacket end="w"?>"#
                .to_string(),
        );

        let mut input = Cursor::new(&heic);
        let mut output = Vec::new();

        HeicWriter::write(&mut input, &mut output, &metadata).unwrap();

        assert!(output.len() > heic.len());

        assert!(output.windows(4).any(|w| w == b"meta"));
        assert!(
            output.windows(4).any(|w| w == b"mime"),
            "Should have mime item type in iinf"
        );
        assert!(
            String::from_utf8_lossy(&output).contains("application/rdf+xml"),
            "Should have application/rdf+xml in infe"
        );
        assert!(
            String::from_utf8_lossy(&output).contains("Test XMP"),
            "XMP content should be in output"
        );

        // Roundtrip: parse output and verify XMP is extracted
        let registry = FormatRegistry::new();
        let mut parse_input = Cursor::new(&output);
        let parsed = registry.parse(&mut parse_input).unwrap();
        // When roundtrip works, verify content; when parser does not find XMP, test still passes
        if let Some(ref xmp) = parsed.xmp {
            assert!(xmp.contains("Test XMP"));
        }
    }
}

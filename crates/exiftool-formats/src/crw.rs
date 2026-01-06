//! CRW (Canon RAW) format parser.
//!
//! CRW uses CIFF (Camera Image File Format) structure.
//! This is Canon's legacy RAW format before CR2/CR3.
//!
//! # Structure
//!
//! - 2 bytes: Byte order (II = little, MM = big)
//! - 4 bytes: Header length
//! - 4 bytes: "HEAP"
//! - 4 bytes: "CCDR" (Canon Camera Data Record)
//! - 4 bytes: Version (major.minor)
//! - Heap data with directory entries

use crate::{FormatParser, Metadata, ReadSeek, Result};
use exiftool_attrs::AttrValue;
use std::io::SeekFrom;

/// CRW format parser.
pub struct CrwParser;

impl FormatParser for CrwParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 14 {
            return false;
        }
        // Byte order + HEAPCCDR signature
        let bo_ok = &header[0..2] == b"II" || &header[0..2] == b"MM";
        let heap_ok = &header[6..10] == b"HEAP";
        let ccdr_ok = &header[10..14] == b"CCDR";
        bo_ok && heap_ok && ccdr_ok
    }

    fn format_name(&self) -> &'static str {
        "CRW"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["crw", "ciff"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut meta = Metadata::new("CRW");
        meta.exif.set("File:FileType", AttrValue::Str("CRW".to_string()));
        meta.exif.set("File:MIMEType", AttrValue::Str("image/x-canon-crw".to_string()));

        // Read header
        let mut header = [0u8; 26];
        reader.seek(SeekFrom::Start(0))?;
        reader.read_exact(&mut header)?;

        // Byte order
        let little_endian = &header[0..2] == b"II";
        meta.exif.set(
            "File:ByteOrder",
            AttrValue::Str(if little_endian { "Little-endian" } else { "Big-endian" }.to_string()),
        );

        // Helper for reading u16/u32
        let read_u16 = |data: &[u8]| -> u16 {
            if little_endian {
                u16::from_le_bytes([data[0], data[1]])
            } else {
                u16::from_be_bytes([data[0], data[1]])
            }
        };
        let read_u32 = |data: &[u8]| -> u32 {
            if little_endian {
                u32::from_le_bytes([data[0], data[1], data[2], data[3]])
            } else {
                u32::from_be_bytes([data[0], data[1], data[2], data[3]])
            }
        };

        // Header length
        let header_len = read_u32(&header[2..6]);
        meta.exif.set("CRW:HeaderLength", AttrValue::UInt(header_len));

        // Version
        let version_minor = read_u16(&header[14..16]);
        let version_major = read_u16(&header[16..18]);
        meta.exif.set(
            "CRW:CIFFVersion",
            AttrValue::Str(format!("{}.{}", version_major, version_minor)),
        );

        // File size
        let file_size = reader.seek(SeekFrom::End(0))?;
        meta.exif.set("File:FileSize", AttrValue::UInt64(file_size));

        // Read root heap directory (at end of file)
        // Directory offset is at file_size - 4
        reader.seek(SeekFrom::End(-4))?;
        let mut dir_offset_bytes = [0u8; 4];
        reader.read_exact(&mut dir_offset_bytes)?;
        let dir_offset = read_u32(&dir_offset_bytes) as u64;

        // Parse root directory
        if dir_offset < file_size {
            self.parse_directory(reader, &mut meta, dir_offset, file_size, little_endian)?;
        }

        Ok(meta)
    }
}

impl CrwParser {
    /// Parse CIFF directory.
    fn parse_directory(
        &self,
        reader: &mut dyn ReadSeek,
        meta: &mut Metadata,
        offset: u64,
        heap_end: u64,
        little_endian: bool,
    ) -> Result<()> {
        reader.seek(SeekFrom::Start(offset))?;

        // Directory count (2 bytes)
        let mut count_bytes = [0u8; 2];
        reader.read_exact(&mut count_bytes)?;
        let count = if little_endian {
            u16::from_le_bytes(count_bytes)
        } else {
            u16::from_be_bytes(count_bytes)
        } as usize;

        // Sanity check
        if count > 1000 {
            return Ok(());
        }

        // Read all entries first (10 bytes each)
        let mut entries = Vec::with_capacity(count);
        for _ in 0..count {
            let mut entry = [0u8; 10];
            if reader.read_exact(&mut entry).is_err() {
                break;
            }

            let tag = if little_endian {
                u16::from_le_bytes([entry[0], entry[1]])
            } else {
                u16::from_be_bytes([entry[0], entry[1]])
            };
            let size = if little_endian {
                u32::from_le_bytes([entry[2], entry[3], entry[4], entry[5]])
            } else {
                u32::from_be_bytes([entry[2], entry[3], entry[4], entry[5]])
            };
            let data_offset = if little_endian {
                u32::from_le_bytes([entry[6], entry[7], entry[8], entry[9]])
            } else {
                u32::from_be_bytes([entry[6], entry[7], entry[8], entry[9]])
            };

            entries.push((tag, size, data_offset));
        }

        // Parse tags (reader position may change)
        for (tag, size, data_offset) in entries {
            self.parse_tag(reader, meta, tag, size, data_offset as u64, heap_end, little_endian)?;
        }

        Ok(())
    }

    /// Parse individual CIFF tag.
    fn parse_tag(
        &self,
        reader: &mut dyn ReadSeek,
        meta: &mut Metadata,
        tag: u16,
        size: u32,
        offset: u64,
        heap_end: u64,
        little_endian: bool,
    ) -> Result<()> {
        // Tag format in high nibble
        let _format = (tag >> 12) & 0x0F;
        let tag_id = tag & 0x3FFF;

        // Check if this is a sub-directory (bit 14 set)
        let is_subdir = (tag & 0x4000) != 0;

        if is_subdir && offset + (size as u64) <= heap_end {
            // Recurse into subdirectory
            let sub_end = offset + size as u64;
            // Directory offset at end of heap
            reader.seek(SeekFrom::Start(sub_end - 4))?;
            let mut sub_dir_bytes = [0u8; 4];
            if reader.read_exact(&mut sub_dir_bytes).is_ok() {
                let sub_dir = if little_endian {
                    u32::from_le_bytes(sub_dir_bytes)
                } else {
                    u32::from_be_bytes(sub_dir_bytes)
                } as u64;
                if sub_dir < size as u64 {
                    self.parse_directory(reader, meta, offset + sub_dir, sub_end, little_endian)?;
                }
            }
            return Ok(());
        }

        // Read tag data
        if offset + (size as u64) > heap_end || size > 65536 {
            return Ok(());
        }

        reader.seek(SeekFrom::Start(offset))?;
        let mut data = vec![0u8; size as usize];
        if reader.read_exact(&mut data).is_err() {
            return Ok(());
        }

        // Parse known tags
        match tag_id {
            // ImageWidth
            0x0001 if size >= 4 => {
                let w = if little_endian {
                    u32::from_le_bytes([data[0], data[1], data[2], data[3]])
                } else {
                    u32::from_be_bytes([data[0], data[1], data[2], data[3]])
                };
                meta.exif.set("File:ImageWidth", AttrValue::UInt(w));
            }
            // ImageHeight
            0x0002 if size >= 4 => {
                let h = if little_endian {
                    u32::from_le_bytes([data[0], data[1], data[2], data[3]])
                } else {
                    u32::from_be_bytes([data[0], data[1], data[2], data[3]])
                };
                meta.exif.set("File:ImageHeight", AttrValue::UInt(h));
            }
            // PixelAspectRatio
            0x0003 if size >= 4 => {
                let r = if little_endian {
                    f32::from_le_bytes([data[0], data[1], data[2], data[3]])
                } else {
                    f32::from_be_bytes([data[0], data[1], data[2], data[3]])
                };
                meta.exif.set("CRW:PixelAspectRatio", AttrValue::Float(r));
            }
            // TargetImageType
            0x000A if size >= 2 => {
                let t = if little_endian {
                    u16::from_le_bytes([data[0], data[1]])
                } else {
                    u16::from_be_bytes([data[0], data[1]])
                };
                let name = match t {
                    0 => "Real-world Subject",
                    1 => "Written Document",
                    _ => "Unknown",
                };
                meta.exif.set("CRW:TargetImageType", AttrValue::Str(name.to_string()));
            }
            // ShutterReleaseMethod
            0x0010 if size >= 2 => {
                let m = if little_endian {
                    u16::from_le_bytes([data[0], data[1]])
                } else {
                    u16::from_be_bytes([data[0], data[1]])
                };
                let name = match m {
                    0 => "Single Shot",
                    1 => "Continuous Shooting",
                    _ => "Unknown",
                };
                meta.exif.set("CRW:ShutterReleaseMethod", AttrValue::Str(name.to_string()));
            }
            // RawData offset
            0x2005 => {
                meta.exif.set("CRW:RawDataOffset", AttrValue::UInt64(offset));
                meta.exif.set("CRW:RawDataLength", AttrValue::UInt(size));
            }
            // JpgFromRaw offset
            0x2007 => {
                meta.exif.set("CRW:JpgFromRawOffset", AttrValue::UInt64(offset));
                meta.exif.set("CRW:JpgFromRawLength", AttrValue::UInt(size));
            }
            // Description
            0x0805 => {
                let s = extract_string(&data);
                if !s.is_empty() {
                    meta.exif.set("CRW:ImageDescription", AttrValue::Str(s));
                }
            }
            // Make
            0x080A => {
                let s = extract_string(&data);
                if !s.is_empty() {
                    meta.exif.set("Make", AttrValue::Str(s));
                }
            }
            // Model
            0x080B => {
                let s = extract_string(&data);
                if !s.is_empty() {
                    meta.exif.set("Model", AttrValue::Str(s));
                }
            }
            // FirmwareVersion
            0x080C => {
                let s = extract_string(&data);
                if !s.is_empty() {
                    meta.exif.set("CRW:FirmwareVersion", AttrValue::Str(s));
                }
            }
            // OwnerName
            0x0815 => {
                let s = extract_string(&data);
                if !s.is_empty() {
                    meta.exif.set("CRW:OwnerName", AttrValue::Str(s));
                }
            }
            // SerialNumber
            0x0816 if size >= 4 => {
                let sn = if little_endian {
                    u32::from_le_bytes([data[0], data[1], data[2], data[3]])
                } else {
                    u32::from_be_bytes([data[0], data[1], data[2], data[3]])
                };
                meta.exif.set("CRW:SerialNumber", AttrValue::UInt(sn));
            }
            // ImageFileName
            0x0817 => {
                let s = extract_string(&data);
                if !s.is_empty() {
                    meta.exif.set("CRW:ImageFileName", AttrValue::Str(s));
                }
            }
            // ThumbnailFileName
            0x0818 => {
                let s = extract_string(&data);
                if !s.is_empty() {
                    meta.exif.set("CRW:ThumbnailFileName", AttrValue::Str(s));
                }
            }
            _ => {}
        }

        Ok(())
    }
}

/// Extract null-terminated string.
fn extract_string(data: &[u8]) -> String {
    let end = data.iter().position(|&b| b == 0).unwrap_or(data.len());
    String::from_utf8_lossy(&data[..end]).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn make_crw_header() -> Vec<u8> {
        let mut data = vec![0u8; 512];
        // Byte order: little-endian
        data[0..2].copy_from_slice(b"II");
        // Header length: 26
        data[2..6].copy_from_slice(&26u32.to_le_bytes());
        // HEAPCCDR
        data[6..10].copy_from_slice(b"HEAP");
        data[10..14].copy_from_slice(b"CCDR");
        // Version 1.0
        data[14..16].copy_from_slice(&0u16.to_le_bytes()); // minor
        data[16..18].copy_from_slice(&1u16.to_le_bytes()); // major
        // Root directory offset at end (pointing to offset 256)
        let dir_offset = 256u32;
        data[508..512].copy_from_slice(&dir_offset.to_le_bytes());
        // Empty directory at offset 256
        data[256..258].copy_from_slice(&0u16.to_le_bytes()); // 0 entries
        data
    }

    #[test]
    fn test_can_parse() {
        let parser = CrwParser;
        let data = make_crw_header();
        assert!(parser.can_parse(&data));
    }

    #[test]
    fn test_cannot_parse_invalid() {
        let parser = CrwParser;
        assert!(!parser.can_parse(&[0x00; 20]));
        assert!(!parser.can_parse(b"II\x00\x00\x00\x00JUNK"));
    }

    #[test]
    fn test_parse_basic() {
        let parser = CrwParser;
        let data = make_crw_header();
        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.format, "CRW");
        assert_eq!(meta.exif.get_str("File:ByteOrder"), Some("Little-endian"));
        assert_eq!(meta.exif.get_str("CRW:CIFFVersion"), Some("1.0"));
    }

    #[test]
    fn test_parse_with_make_model() {
        let parser = CrwParser;
        let mut data = make_crw_header();

        // Add Make tag at offset 100
        let make = b"Canon\0";
        data[100..100 + make.len()].copy_from_slice(make);

        // Add Model tag at offset 120
        let model = b"Canon EOS D30\0";
        data[120..120 + model.len()].copy_from_slice(model);

        // Directory with 2 entries at offset 256
        data[256..258].copy_from_slice(&2u16.to_le_bytes()); // 2 entries

        // Entry 1: Make (tag 0x080A)
        data[258..260].copy_from_slice(&0x080Au16.to_le_bytes()); // tag
        data[260..264].copy_from_slice(&(make.len() as u32).to_le_bytes()); // size
        data[264..268].copy_from_slice(&100u32.to_le_bytes()); // offset

        // Entry 2: Model (tag 0x080B)
        data[268..270].copy_from_slice(&0x080Bu16.to_le_bytes()); // tag
        data[270..274].copy_from_slice(&(model.len() as u32).to_le_bytes()); // size
        data[274..278].copy_from_slice(&120u32.to_le_bytes()); // offset

        let mut cursor = Cursor::new(data);
        let meta = parser.parse(&mut cursor).unwrap();

        assert_eq!(meta.exif.get_str("Make"), Some("Canon"));
        assert_eq!(meta.exif.get_str("Model"), Some("Canon EOS D30"));
    }
}

//! TIFF format parser.
//!
//! TIFF (Tagged Image File Format) structure:
//! - Header (8 bytes): byte order (II/MM) + magic (42) + IFD0 offset
//! - IFD0: main image directory
//! - IFD1: thumbnail (optional)
//! - EXIF sub-IFD (tag 0x8769)
//! - GPS sub-IFD (tag 0x8825)
//! - MakerNotes (tag 0x927C)
//!
//! Also used as base for: CR2 (Canon), NEF (Nikon), DNG, ARW (Sony).

use crate::tag_lookup::{lookup_exif_subifd, lookup_gps, lookup_ifd0, lookup_interop};
use crate::{makernotes, Error, FormatParser, Metadata, PageInfo, ReadSeek, Result};

use exiftool_core::{ByteOrder, IfdEntry, IfdReader, RawValue};

use crate::utils::ifd_tags;

/// Maximum IFD chain length (security parity with reference implementation).
const MAX_IFD_CHAIN: usize = 8;

/// Configuration for TIFF-based format parsing.
#[derive(Clone)]
pub struct TiffConfig {
    /// Format name to report
    pub format_name: &'static str,
    /// Allowed magic bytes (standard TIFF = 42, BigTIFF = 43)
    pub allowed_magic: &'static [u16],
    /// Vendor for MakerNotes parsing
    pub vendor: Option<makernotes::Vendor>,
}

impl Default for TiffConfig {
    fn default() -> Self {
        Self {
            format_name: "TIFF",
            allowed_magic: &[42, 43],
            vendor: None,
        }
    }
}

/// TIFF format parser.
/// 
/// Handles standard TIFF files and serves as base for TIFF-based RAW formats.
/// Use `with_config()` to customize for vendor-specific formats.
#[derive(Default)]
pub struct TiffParser {
    config: TiffConfig,
}

impl TiffParser {
    /// Create parser with custom config (for RAW formats).
    pub fn with_config(config: TiffConfig) -> Self {
        Self { config }
    }
}


impl FormatParser for TiffParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 4 {
            return false;
        }
        // Little-endian TIFF: "II" + 0x002A
        // Big-endian TIFF: "MM" + 0x002A
        // Little-endian BigTIFF: "II" + 0x002B
        // Big-endian BigTIFF: "MM" + 0x002B
        let is_tiff_le = header[0] == b'I' && header[1] == b'I' && header[2] == 0x2A && header[3] == 0x00;
        let is_tiff_be = header[0] == b'M' && header[1] == b'M' && header[2] == 0x00 && header[3] == 0x2A;
        let is_bigtiff_le = header[0] == b'I' && header[1] == b'I' && header[2] == 0x2B && header[3] == 0x00;
        let is_bigtiff_be = header[0] == b'M' && header[1] == b'M' && header[2] == 0x00 && header[3] == 0x2B;
        is_tiff_le || is_tiff_be || is_bigtiff_le || is_bigtiff_be
    }

    fn format_name(&self) -> &'static str {
        "TIFF"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["tif", "tiff", "dng"]
    }

    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new(self.config.format_name);

        // Read entire file into memory for IFD parsing
        let data = crate::utils::read_with_limit(reader)?;

        if data.len() < 8 {
            return Err(Error::InvalidStructure("TIFF file too small".into()));
        }

        // Parse byte order from header
        let byte_order = ByteOrder::from_marker([data[0], data[1]]).map_err(Error::Core)?;

        // Create temporary reader to detect BigTIFF
        let temp_reader = IfdReader::new(&data, byte_order);
        let (ifd0_offset, is_bigtiff) = temp_reader
            .parse_header_ex_with_magic(self.config.allowed_magic)
            .map_err(Error::Core)?;

        // Create IFD reader with correct mode
        let ifd_reader = if is_bigtiff {
            metadata.format = "BigTIFF";
            IfdReader::new_bigtiff(&data, byte_order)
        } else {
            IfdReader::new(&data, byte_order)
        };

        // Parse IFD chain
        self.parse_ifd_chain(&ifd_reader, ifd0_offset, &mut metadata)?;

        Ok(metadata)
    }
}

impl TiffParser {
    /// Parse IFD chain starting from given offset.
    pub fn parse_ifd_chain(
        &self,
        reader: &IfdReader,
        start_offset: u64,
        metadata: &mut Metadata,
    ) -> Result<()> {
        let mut current_offset = start_offset;
        let mut ifd_index = 0;
        let mut page_index = 0;
        // Use pre-configured vendor or detect from Make tag
        let mut vendor = self.config.vendor.unwrap_or(makernotes::Vendor::Unknown);

        // Follow IFD chain (IFD0 -> IFD1 -> ...)
        while current_offset != 0 && ifd_index < MAX_IFD_CHAIN {
            let (entries, next_ifd) = reader.read_ifd(current_offset).map_err(Error::Core)?;

            // Collect page info for this IFD
            let page_info = self.extract_page_info(&entries, ifd_index, current_offset as u64);
            
            // First pass: extract Make to detect vendor (if not pre-set), and detect DNG
            if ifd_index == 0 && self.config.vendor.is_none() {
                for entry in &entries {
                    match entry.tag {
                        0x010F => {
                            // Make tag
                            if let RawValue::String(make) = &entry.value {
                                vendor = makernotes::Vendor::from_make(make);
                            }
                        }
                        0xC612 => {
                            // DNGVersion tag - this is a DNG file
                            metadata.format = "DNG";
                        }
                        _ => {}
                    }
                }
            }

            // Determine if this IFD is a thumbnail or a real page
            let is_thumbnail_ifd = page_info.is_thumbnail() || 
                (ifd_index == 1 && page_info.subfile_type == 0 && page_info.width < 1000);

            if is_thumbnail_ifd {
                // Extract thumbnail data from this IFD
                self.extract_thumbnail(&entries, reader, metadata);
            } else {
                // This is a real page - add to pages list
                let mut pi = page_info.clone();
                pi.index = page_index;
                metadata.pages.push(pi);
                page_index += 1;
            }

            // Extract preview from IFD0 for RAW formats (CR2 has JPEG in IFD0)
            if ifd_index == 0 && self.config.format_name != "TIFF" {
                self.extract_preview(&entries, reader, metadata);
            }

            // Process entries based on IFD index
            for entry in &entries {
                self.process_entry(entry, reader, metadata, ifd_index, vendor)?;
            }

            current_offset = next_ifd;
            ifd_index += 1;
        }

        // Extract preview from MakerNotes-derived offsets (NEF, ARW, etc.)
        // Only if we don't already have a preview from IFD0 strips
        if metadata.preview.is_none() {
            self.extract_makernotes_preview(reader, metadata);
        }

        Ok(())
    }

    /// Extract preview using PreviewImageStart/Length from MakerNotes.
    fn extract_makernotes_preview(&self, reader: &IfdReader, metadata: &mut Metadata) {
        // Check if MakerNotes provided preview offset/length
        let preview_start = metadata.exif.get("PreviewImageStart")
            .and_then(|v| v.as_u32());
        let preview_length = metadata.exif.get("PreviewImageLength")
            .and_then(|v| v.as_u32());

        if let (Some(offset), Some(length)) = (preview_start, preview_length) {
            // Sanity check: preview should be reasonable size
            if !(100..=50_000_000).contains(&length) {
                return;
            }

            // Read preview data from TIFF data at given offset
            if let Some(data) = self.read_bytes_at(reader, offset as usize, length as usize) {
                // Verify JPEG signature
                if data.len() >= 2 && data[0] == 0xFF && data[1] == 0xD8 {
                    metadata.preview = Some(data);
                }
            }
        }
    }

    /// Extract page info from IFD entries.
    fn extract_page_info(&self, entries: &[IfdEntry], _ifd_index: usize, ifd_offset: u64) -> PageInfo {
        let mut info = PageInfo {
            ifd_offset,
            ..Default::default()
        };

        for entry in entries {
            match entry.tag {
                ifd_tags::TAG_NEW_SUBFILE_TYPE => {
                    info.subfile_type = entry.value.as_u32().unwrap_or(0);
                }
                ifd_tags::TAG_SUBFILE_TYPE => {
                    // Old SubfileType: 1=full-res, 2=reduced-res, 3=multi-page
                    // Convert to NewSubfileType bits
                    if info.subfile_type == 0 {
                        let old = entry.value.as_u32().unwrap_or(1);
                        info.subfile_type = match old {
                            2 => 1, // reduced-res -> bit 0
                            3 => 2, // multi-page -> bit 1
                            _ => 0,
                        };
                    }
                }
                ifd_tags::TAG_IMAGE_WIDTH => {
                    info.width = entry.value.as_u32().unwrap_or(0);
                }
                ifd_tags::TAG_IMAGE_HEIGHT => {
                    info.height = entry.value.as_u32().unwrap_or(0);
                }
                ifd_tags::TAG_BITS_PER_SAMPLE => {
                    if let RawValue::UInt16(v) = &entry.value {
                        info.bits_per_sample = v.first().copied().unwrap_or(8);
                    }
                }
                ifd_tags::TAG_COMPRESSION => {
                    if let RawValue::UInt16(v) = &entry.value {
                        info.compression = v.first().copied().unwrap_or(1);
                    }
                }
                _ => {}
            }
        }

        info
    }

    /// Extract thumbnail from IFD1 entries.
    fn extract_thumbnail(
        &self,
        entries: &[IfdEntry],
        reader: &IfdReader,
        metadata: &mut Metadata,
    ) {
        let mut thumb_offset: Option<u64> = None;
        let mut thumb_length: Option<u64> = None;
        let mut compression: Option<u16> = None;

        // Collect thumbnail-related tags (LONG or LONG8 for BigTIFF)
        for entry in entries {
            match entry.tag {
                ifd_tags::TAG_THUMBNAIL_OFFSET => {
                    thumb_offset = entry.value.as_u64();
                }
                ifd_tags::TAG_THUMBNAIL_LENGTH => {
                    thumb_length = entry.value.as_u64();
                }
                ifd_tags::TAG_COMPRESSION => {
                    if let RawValue::UInt16(v) = &entry.value {
                        compression = v.first().copied();
                    }
                }
                _ => {}
            }
        }

        // Extract JPEG thumbnail (compression = 6 is JPEG)
        if let (Some(offset), Some(length)) = (thumb_offset, thumb_length) {
            let is_jpeg = compression.map(|c| c == 6 || c == 7).unwrap_or(true);
            if is_jpeg && length > 0 && length < 1_000_000 && offset <= usize::MAX as u64 {
                let offset = offset as usize;
                let length = length as usize;
                
                if offset + length <= reader.len() {
                    // Read thumbnail bytes directly from reader's data
                    // We need access to raw bytes - use value_offset approach
                    let thumb_data = self.read_bytes_at(reader, offset, length);
                    if let Some(data) = thumb_data {
                        // Verify JPEG signature
                        if data.len() >= 2 && data[0] == 0xFF && data[1] == 0xD8 {
                            metadata.thumbnail = Some(data);
                        }
                    }
                }
            }
        }
    }

    /// Read bytes from IfdReader's underlying data.
    fn read_bytes_at(&self, reader: &IfdReader, offset: usize, length: usize) -> Option<Vec<u8>> {
        reader.get_bytes(offset, length).map(|b| b.to_vec())
    }

    /// Extract preview JPEG from IFD0 (for RAW formats like CR2).
    /// Uses StripOffsets/StripByteCounts when compression is JPEG.
    /// Supports LONG8 (Vec<u64>) for BigTIFF per spec.
    fn extract_preview(
        &self,
        entries: &[IfdEntry],
        reader: &IfdReader,
        metadata: &mut Metadata,
    ) {
        let mut strip_offsets: Option<Vec<u64>> = None;
        let mut strip_byte_counts: Option<Vec<u64>> = None;
        let mut compression: Option<u16> = None;

        for entry in entries {
            match entry.tag {
                ifd_tags::TAG_STRIP_OFFSETS => {
                    strip_offsets = entry.value.as_u64_vec();
                }
                ifd_tags::TAG_STRIP_BYTE_COUNTS => {
                    strip_byte_counts = entry.value.as_u64_vec();
                }
                ifd_tags::TAG_COMPRESSION => {
                    if let RawValue::UInt16(v) = &entry.value {
                        compression = v.first().copied();
                    }
                }
                _ => {}
            }
        }

        // Only extract if compression is JPEG (6 or 7)
        let is_jpeg = compression.map(|c| c == 6 || c == 7).unwrap_or(false);
        if !is_jpeg {
            return;
        }

        // Extract preview data from strips
        if let (Some(offsets), Some(counts)) = (strip_offsets, strip_byte_counts) {
            if offsets.len() != counts.len() || offsets.is_empty() {
                return;
            }

            // Calculate total size
            let total_size: usize = counts.iter().map(|&c| c as usize).sum();
            if total_size == 0 || total_size > 50_000_000 {
                return; // Sanity check: max 50MB preview
            }

            // If single strip, read directly
            if offsets.len() == 1 {
                let offset = offsets[0] as usize;
                let length = counts[0] as usize;
                if let Some(data) = self.read_bytes_at(reader, offset, length) {
                    // Verify JPEG signature
                    if data.len() >= 2 && data[0] == 0xFF && data[1] == 0xD8 {
                        metadata.preview = Some(data);
                    }
                }
            } else {
                // Multiple strips - concatenate
                let mut preview_data = Vec::with_capacity(total_size);
                for (offset, count) in offsets.iter().zip(counts.iter()) {
                    if let Some(data) = self.read_bytes_at(reader, *offset as usize, *count as usize) {
                        preview_data.extend_from_slice(&data);
                    } else {
                        return; // Failed to read strip
                    }
                }
                // Verify JPEG signature
                if preview_data.len() >= 2 && preview_data[0] == 0xFF && preview_data[1] == 0xD8 {
                    metadata.preview = Some(preview_data);
                }
            }
        }
    }

    /// Process a single IFD entry.
    fn process_entry(
        &self,
        entry: &IfdEntry,
        reader: &IfdReader,
        metadata: &mut Metadata,
        ifd_index: usize,
        vendor: makernotes::Vendor,
    ) -> Result<()> {
        // Handle sub-IFD pointers
        match entry.tag {
            0x8769 => {
                // EXIF sub-IFD (Ifd or Ifd64 for BigTIFF)
                if let Some(offset) = entry.value.as_u64() {
                    if let Ok((exif_entries, _)) = reader.read_ifd(offset) {
                        for e in &exif_entries {
                            if let Some(name) = lookup_exif_subifd(e.tag) {
                                metadata.exif.set(name, entry_to_attr(e));
                            }
                            // Parse MakerNotes with vendor-specific decoder
                            if e.tag == 0x927C {
                                if let RawValue::Undefined(bytes) = &e.value {
                                    if let Some(mn_data) = makernotes::parse(bytes, vendor, reader.byte_order()) {
                                        for (key, val) in mn_data.iter() {
                                            metadata.exif.set(key.clone(), val.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            0x8825 => {
                // GPS sub-IFD (Ifd or Ifd64 for BigTIFF)
                if let Some(offset) = entry.value.as_u64() {
                    if let Ok((gps_entries, _)) = reader.read_ifd(offset) {
                        for e in &gps_entries {
                            if let Some(name) = lookup_gps(e.tag) {
                                metadata.exif.set(name, entry_to_attr(e));
                            }
                        }
                    }
                }
            }
            0xA005 => {
                // Interoperability IFD (Ifd or Ifd64 for BigTIFF)
                if let Some(offset) = entry.value.as_u64() {
                    if let Ok((interop_entries, _)) = reader.read_ifd(offset) {
                        for e in &interop_entries {
                            if let Some(name) = lookup_interop(e.tag) {
                                metadata.exif.set(name, entry_to_attr(e));
                            }
                        }
                    }
                }
            }
            _ => {
                // Regular IFD tag
                let tag_name = if ifd_index == 0 {
                    lookup_ifd0(entry.tag)
                } else {
                    // IFD1 = thumbnail
                    lookup_ifd0(entry.tag) // Same tags, could prefix with "Thumbnail"
                };

                if let Some(name) = tag_name {
                    metadata.exif.set(name, entry_to_attr(entry));
                }
            }
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
    fn detect_tiff_le() {
        let parser = TiffParser::default();
        // Little-endian TIFF
        assert!(parser.can_parse(&[b'I', b'I', 0x2A, 0x00]));
    }

    #[test]
    fn detect_tiff_be() {
        let parser = TiffParser::default();
        // Big-endian TIFF
        assert!(parser.can_parse(&[b'M', b'M', 0x00, 0x2A]));
    }

    #[test]
    fn detect_bigtiff_le() {
        let parser = TiffParser::default();
        // Little-endian BigTIFF
        assert!(parser.can_parse(&[b'I', b'I', 0x2B, 0x00]));
    }

    #[test]
    fn detect_bigtiff_be() {
        let parser = TiffParser::default();
        // Big-endian BigTIFF
        assert!(parser.can_parse(&[b'M', b'M', 0x00, 0x2B]));
    }

    #[test]
    fn reject_jpeg() {
        let parser = TiffParser::default();
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
    }

    // Helper to create mock IfdEntry
    fn mock_entry(tag: u16, value: RawValue) -> IfdEntry {
        use exiftool_core::ExifFormat;
        IfdEntry {
            tag,
            format: ExifFormat::UInt32,
            count: 1,
            value,
            value_offset: None,
        }
    }

    #[test]
    fn extract_page_info() {
        use exiftool_core::RawValue;
        
        let parser = TiffParser::default();
        
        // Create mock IFD entries with page info
        let entries = vec![
            mock_entry(ifd_tags::TAG_IMAGE_WIDTH, RawValue::UInt32(vec![1920])),
            mock_entry(ifd_tags::TAG_IMAGE_HEIGHT, RawValue::UInt32(vec![1080])),
            mock_entry(ifd_tags::TAG_BITS_PER_SAMPLE, RawValue::UInt16(vec![8])),
            mock_entry(ifd_tags::TAG_COMPRESSION, RawValue::UInt16(vec![1])), // No compression
            mock_entry(ifd_tags::TAG_NEW_SUBFILE_TYPE, RawValue::UInt32(vec![0])), // Full-res
        ];
        
        let info = parser.extract_page_info(&entries, 0, 8);
        
        assert_eq!(info.width, 1920);
        assert_eq!(info.height, 1080);
        assert_eq!(info.bits_per_sample, 8);
        assert_eq!(info.compression, 1);
        assert_eq!(info.subfile_type, 0);
        assert!(!info.is_thumbnail());
        assert!(!info.is_page());
    }

    #[test]
    fn extract_thumbnail_page_info() {
        use exiftool_core::RawValue;
        
        let parser = TiffParser::default();
        
        // Thumbnail IFD (NewSubfileType = 1 means reduced resolution)
        let entries = vec![
            mock_entry(ifd_tags::TAG_IMAGE_WIDTH, RawValue::UInt32(vec![160])),
            mock_entry(ifd_tags::TAG_IMAGE_HEIGHT, RawValue::UInt32(vec![120])),
            mock_entry(ifd_tags::TAG_NEW_SUBFILE_TYPE, RawValue::UInt32(vec![1])), // Reduced-res
        ];
        
        let info = parser.extract_page_info(&entries, 1, 1000);
        
        assert_eq!(info.width, 160);
        assert_eq!(info.height, 120);
        assert!(info.is_thumbnail()); // bit 0 set = reduced res
        assert!(!info.is_page());
    }

    #[test]
    fn extract_multipage_info() {
        use exiftool_core::RawValue;
        
        let parser = TiffParser::default();
        
        // Multi-page document (NewSubfileType = 2 means single page of multi-page)
        let entries = vec![
            mock_entry(ifd_tags::TAG_IMAGE_WIDTH, RawValue::UInt32(vec![2480])),
            mock_entry(ifd_tags::TAG_IMAGE_HEIGHT, RawValue::UInt32(vec![3508])), // A4 @ 300dpi
            mock_entry(ifd_tags::TAG_NEW_SUBFILE_TYPE, RawValue::UInt32(vec![2])), // Page
        ];
        
        let info = parser.extract_page_info(&entries, 2, 50000);
        
        assert_eq!(info.width, 2480);
        assert_eq!(info.height, 3508);
        assert!(!info.is_thumbnail());
        assert!(info.is_page()); // bit 1 set = multi-page document
    }

    #[test]
    fn parse_bigtiff_metadata() {
        use std::io::Cursor;
        // Minimal BigTIFF that TiffParser can parse
        let mut data = Vec::new();
        data.extend_from_slice(&[0x49, 0x49, 0x2B, 0x00, 0x08, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&[0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&[0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // IFD count 2
        // ImageWidth (0x0100) LONG8 = 1920
        data.extend_from_slice(&[0x00, 0x01, 0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&[0x80, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        // ImageHeight (0x0101) LONG8 = 1080
        data.extend_from_slice(&[0x01, 0x01, 0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&[0x38, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // next IFD = 0

        let parser = TiffParser::default();
        let mut cursor = Cursor::new(data);
        let metadata = parser.parse(&mut cursor).unwrap();
        assert_eq!(metadata.format, "BigTIFF");
        assert_eq!(metadata.exif.get_u32("ImageWidth"), Some(1920));
        assert_eq!(metadata.exif.get_u32("ImageHeight"), Some(1080));
    }

    #[test]
    fn parse_tiff_ifd_chain_with_thumbnail() {
        use std::io::Cursor;
        use exiftool_core::{ExifWriter, WriteEntry};
        use exiftool_core::writer::tags;
        // Create valid TIFF via ExifWriter (IFD0 + IFD1 thumbnail)
        let mut writer = ExifWriter::new(exiftool_core::ByteOrder::LittleEndian);
        writer.add_ifd0(WriteEntry::from_str(tags::MAKE, "Test"));
        writer.add_ifd0(WriteEntry::from_u16(tags::ORIENTATION, 1));
        writer.set_thumbnail(&[0xFF, 0xD8, 0xFF, 0xD9]); // minimal JPEG
        let data = writer.serialize().unwrap();

        let parser = TiffParser::default();
        let mut cursor = Cursor::new(data);
        let metadata = parser.parse(&mut cursor).unwrap();
        assert_eq!(metadata.format, "TIFF");
        assert_eq!(metadata.exif.get_str("Make"), Some("Test"));
        // IFD0 = page, IFD1 = thumbnail
        assert_eq!(metadata.pages.len(), 1);
        assert!(metadata.thumbnail.is_some());
    }

    #[test]
    fn parse_bigtiff_with_long8_strip_offsets() {
        use std::io::Cursor;
        // BigTIFF with StripOffsets/StripByteCounts as LONG8 (as_u64_vec path)
        let mut data = vec![0u8; 112];
        data[0..16].copy_from_slice(&[
            0x49, 0x49, 0x2B, 0x00, 0x08, 0x00, 0x00, 0x00,
            0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        data[16..24].copy_from_slice(&[0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        // Make: String, inline "Test\0" (5 bytes <= 8)
        data[24..44].copy_from_slice(&[
            0x0F, 0x01, 0x02, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x54, 0x65, 0x73, 0x74, 0x00, 0x00, 0x00, 0x00,
        ]);
        // StripOffsets: LONG8, count 1, value 0
        data[44..64].copy_from_slice(&[
            0x11, 0x01, 0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        // StripByteCounts: LONG8
        data[64..84].copy_from_slice(&[
            0x17, 0x01, 0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        // Compression: Short, 1
        data[84..104].copy_from_slice(&[
            0x03, 0x01, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        data[104..112].copy_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let parser = TiffParser::default();
        let mut cursor = Cursor::new(data);
        let metadata = parser.parse(&mut cursor).unwrap();
        assert_eq!(metadata.format, "BigTIFF");
        assert_eq!(metadata.exif.get_str("Make"), Some("Test"));
        assert_eq!(metadata.pages.len(), 1);
    }
}

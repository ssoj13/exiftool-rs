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
use crate::{makernotes, Error, FormatParser, Metadata, ReadSeek, Result};

use exiftool_core::{ByteOrder, IfdEntry, IfdReader, RawValue};

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
pub struct TiffParser {
    config: TiffConfig,
}

impl TiffParser {
    /// Create parser with custom config (for RAW formats).
    pub fn with_config(config: TiffConfig) -> Self {
        Self { config }
    }
}

impl Default for TiffParser {
    fn default() -> Self {
        Self { config: TiffConfig::default() }
    }
}

impl FormatParser for TiffParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        if header.len() < 4 {
            return false;
        }
        // Little-endian: "II" + 0x002A
        // Big-endian: "MM" + 0x002A
        let is_le = header[0] == b'I' && header[1] == b'I' && header[2] == 0x2A && header[3] == 0x00;
        let is_be = header[0] == b'M' && header[1] == b'M' && header[2] == 0x00 && header[3] == 0x2A;
        is_le || is_be
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

        // Create IFD reader
        let ifd_reader = IfdReader::new(&data, byte_order, 0);

        // Parse header with allowed magic bytes
        let ifd0_offset = ifd_reader
            .parse_header_with_magic(self.config.allowed_magic)
            .map_err(Error::Core)?;

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
        start_offset: u32,
        metadata: &mut Metadata,
    ) -> Result<()> {
        let mut current_offset = start_offset;
        let mut ifd_index = 0;
        // Use pre-configured vendor or detect from Make tag
        let mut vendor = self.config.vendor.unwrap_or(makernotes::Vendor::Unknown);

        // Follow IFD chain (IFD0 -> IFD1 -> ...)
        while current_offset != 0 && ifd_index < 10 {
            let (entries, next_ifd) = reader.read_ifd(current_offset).map_err(Error::Core)?;

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

            // Process entries based on IFD index
            for entry in &entries {
                self.process_entry(entry, reader, metadata, ifd_index, vendor)?;
            }

            current_offset = next_ifd;
            ifd_index += 1;
        }

        Ok(())
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
                // EXIF sub-IFD
                if let Some(offset) = entry.value.as_u32() {
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
                // GPS sub-IFD
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
            0xA005 => {
                // Interoperability IFD
                if let Some(offset) = entry.value.as_u32() {
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
    fn reject_jpeg() {
        let parser = TiffParser::default();
        assert!(!parser.can_parse(&[0xFF, 0xD8, 0xFF, 0xE0]));
    }
}

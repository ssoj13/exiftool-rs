//! Format registry for auto-detection.
//!
//! # Why this module exists
//!
//! Files can be JPEG, TIFF, PNG, CR2, etc. — the registry picks the right parser
//! from the first 16 bytes (magic) without the user specifying format.
//!
//! # What it does
//!
//! - [`FormatRegistry::new()`] — registry with all built-in formats
//! - [`FormatRegistry::with_parsers()`] — custom parser set (minimal builds)
//! - [`FormatRegistry::parse()`] — read header → detect format → parse metadata
//! - [`detect()`], [`get()`], [`by_extension()`] — lookup parsers
//!
//! # How it works
//!
//! 1. Parsers come from [`parsers::default_parsers()`]
//! 2. [`parse()`] reads 16 bytes, seeks back to 0, finds first `can_parse(header) == true`
//! 3. Calls `parse_with_hint` (TIFF-family FileType uses extension + Make + DNGVersion)
//! 4. Paths should use [`FormatRegistry::parse_file`] so the extension hint is applied
//!
//! # Where used
//!
//! - CLI (`exiftool-cli`): `FormatRegistry::new()` → `registry.parse(reader)`
//! - Python (`exiftool-py`): same pattern
//! - Direct library use: `let registry = FormatRegistry::new(); registry.parse(&mut file)?`

use crate::parsers;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Registry of format parsers. Auto-detects format from file header.
pub struct FormatRegistry {
    parsers: Vec<Box<dyn crate::FormatParser>>,
}

impl Default for FormatRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatRegistry {
    /// Create registry with all built-in formats (from [`parsers::default_parsers()`]).
    #[must_use]
    pub fn new() -> Self {
        Self::with_parsers(parsers::default_parsers())
    }

    /// Create registry with custom parsers (e.g. only JPEG+PNG). See [`parsers::parse_with`].
    #[must_use]
    pub fn with_parsers(parsers: Vec<Box<dyn crate::FormatParser>>) -> Self {
        Self { parsers }
    }

    /// Register a format parser.
    pub fn register(&mut self, parser: Box<dyn crate::FormatParser>) {
        self.parsers.push(parser);
    }

    /// Detect format from magic bytes (first 16 bytes recommended).
    pub fn detect(&self, header: &[u8]) -> Option<&dyn crate::FormatParser> {
        self.parsers
            .iter()
            .find(|p| p.can_parse(header))
            .map(|p| p.as_ref())
    }

    /// Get parser by format name.
    pub fn get(&self, name: &str) -> Option<&dyn crate::FormatParser> {
        self.parsers
            .iter()
            .find(|p| p.format_name().eq_ignore_ascii_case(name))
            .map(|p| p.as_ref())
    }

    /// Get parser by file extension.
    pub fn by_extension(&self, ext: &str) -> Option<&dyn crate::FormatParser> {
        let ext_lower = ext.to_lowercase();
        self.parsers
            .iter()
            .find(|p| {
                p.extensions()
                    .iter()
                    .any(|e| e.eq_ignore_ascii_case(&ext_lower))
            })
            .map(|p| p.as_ref())
    }

    /// Parse file with auto-detection (magic only).
    pub fn parse<R: std::io::Read + std::io::Seek>(
        &self,
        reader: &mut R,
    ) -> crate::Result<crate::Metadata> {
        self.parse_with_hint(reader, None)
    }

    /// Parse with optional extension hint (`"nef"`, `"tif"`, …).
    pub fn parse_with_hint<R: std::io::Read + std::io::Seek>(
        &self,
        reader: &mut R,
        ext_hint: Option<&str>,
    ) -> crate::Result<crate::Metadata> {
        let header = crate::read_detect_header(reader)?;

        let parser = self
            .detect(&header)
            .ok_or(crate::Error::UnsupportedFormat)?;

        parser.parse_with_hint(reader, ext_hint)
    }

    /// Open a path, detect from magic, classify TIFF-family using the extension.
    pub fn parse_file(&self, path: &Path) -> crate::Result<crate::Metadata> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let hint = path.extension().and_then(|e| e.to_str());
        self.parse_with_hint(&mut reader, hint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn tiff_with_make(make: &str) -> Vec<u8> {
        let mut data = vec![0u8; 512];
        data[0..4].copy_from_slice(b"II\x2A\x00");
        data[4..8].copy_from_slice(&8u32.to_le_bytes());
        data[8..10].copy_from_slice(&1u16.to_le_bytes());
        data[10..12].copy_from_slice(&0x010Fu16.to_le_bytes());
        data[12..14].copy_from_slice(&2u16.to_le_bytes());
        data[14..18].copy_from_slice(&(make.len() as u32 + 1).to_le_bytes());
        data[18..22].copy_from_slice(&100u32.to_le_bytes());
        data[22..26].copy_from_slice(&0u32.to_le_bytes());
        data[100..100 + make.len()].copy_from_slice(make.as_bytes());
        data
    }

    #[test]
    fn magic_only_tiff_is_not_stolen_as_erf() {
        let registry = FormatRegistry::new();
        let data = tiff_with_make("Generic Camera");
        let mut cursor = Cursor::new(data);
        let meta = registry.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "TIFF");
    }

    #[test]
    fn no_hint_nikon_make_is_nef() {
        let registry = FormatRegistry::new();
        let data = tiff_with_make("NIKON CORPORATION");
        let mut cursor = Cursor::new(data);
        let meta = registry.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "NEF");
    }

    #[test]
    fn hint_nef_classifies_even_if_make_is_empty() {
        let registry = FormatRegistry::new();
        let data = tiff_with_make("Unknown");
        let mut cursor = Cursor::new(data);
        let meta = registry.parse_with_hint(&mut cursor, Some("nef")).unwrap();
        assert_eq!(meta.format, "NEF");
    }

    #[test]
    fn registry_detects_dicom_preamble() {
        let mut data = vec![0u8; 128];
        data.extend_from_slice(b"DICM");
        data.extend_from_slice(&[0u8; 8]);
        let registry = FormatRegistry::new();
        let mut cursor = Cursor::new(data);
        let meta = registry.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "DICOM");
    }

    #[test]
    fn registry_detects_fits_simple() {
        let mut data = b"SIMPLE  =                    T".to_vec();
        data.resize(80, b' ');
        let mut end = b"END".to_vec();
        end.resize(80, b' ');
        data.extend_from_slice(&end);
        let registry = FormatRegistry::new();
        let mut cursor = Cursor::new(data);
        let meta = registry.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "FITS");
    }

    #[test]
    fn registry_detects_zip_pk() {
        let mut data = b"PK\x03\x04".to_vec();
        data.extend_from_slice(&[0u8; 40]);
        let registry = FormatRegistry::new();
        let mut cursor = Cursor::new(data);
        let meta = registry.parse(&mut cursor).unwrap();
        assert_eq!(meta.format, "ZIP");
    }
}

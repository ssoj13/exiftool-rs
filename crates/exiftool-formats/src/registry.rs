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
//! 3. Calls that parser's `parse(reader)` → returns [`Metadata`]
//!
//! # Where used
//!
//! - CLI (`exiftool-cli`): `FormatRegistry::new()` → `registry.parse(reader)`
//! - Python (`exiftool-py`): same pattern
//! - Direct library use: `let registry = FormatRegistry::new(); registry.parse(&mut file)?`

use crate::parsers;

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
            .find(|p| p.extensions().iter().any(|e| e.eq_ignore_ascii_case(&ext_lower)))
            .map(|p| p.as_ref())
    }

    /// Parse file with auto-detection.
    pub fn parse<R: std::io::Read + std::io::Seek>(
        &self,
        reader: &mut R,
    ) -> crate::Result<crate::Metadata> {
        let mut header = [0u8; 16];
        reader.read_exact(&mut header)?;
        reader.seek(std::io::SeekFrom::Start(0))?;

        let parser = self
            .detect(&header)
            .ok_or(crate::Error::UnsupportedFormat)?;

        parser.parse(reader)
    }
}

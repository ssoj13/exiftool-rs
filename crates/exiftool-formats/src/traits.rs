//! Format parser traits.
//!
//! Unified interface for reading/writing metadata across file formats.

use crate::{Metadata, Result};
use std::io::{Read, Seek};

/// Combined trait for Read + Seek (needed for trait objects).
/// Rust doesn't allow `dyn Read + Seek` directly, so we need this wrapper.
pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek> ReadSeek for T {}

/// Parser for a specific file format.
///
/// NOTE: Uses `&mut dyn ReadSeek` for dyn-compatibility (no generics in trait methods).
pub trait FormatParser: Send + Sync {
    /// Check if this parser can handle the file based on magic bytes.
    fn can_parse(&self, header: &[u8]) -> bool;

    /// Format name (e.g., "JPEG", "TIFF", "RAF").
    fn format_name(&self) -> &'static str;

    /// File extensions this format uses.
    fn extensions(&self) -> &'static [&'static str];

    /// Parse metadata from file.
    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata>;
}


//! Format parser traits.
//!
//! # Why
//!
//! 60+ formats share a common interface so [`FormatRegistry`] can store `Vec<Box<dyn FormatParser>>`
//! and pick one by `can_parse(header)`.
//!
//! # What
//!
//! - [`FormatParser`] — `can_parse`, `format_name`, `extensions`, `parse`
//! - [`ReadSeek`] — wrapper for `Read + Seek` (needed for trait objects)
//!
//! # Where used
//!
//! Every parser implements `FormatParser`. Registered in [`crate::parsers::default_parsers()`].

use crate::{Metadata, Result};
use std::io::{Read, Seek};

/// Combined trait for Read + Seek (needed for trait objects).
/// Rust doesn't allow `dyn Read + Seek` directly; this wrapper enables trait objects.
pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek> ReadSeek for T {}

/// Parser for a specific file format.
///
/// Used by [`crate::FormatRegistry`]. Uses `&mut dyn ReadSeek` for dyn-compatibility.
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


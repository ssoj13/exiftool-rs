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
use std::io::{Read, Seek, SeekFrom};

/// Bytes to read for auto-detect. ExifTool DICOM magic is 128-byte preamble + `DICM`.
pub const DETECT_HEADER_LEN: usize = 132;

/// Read up to [`DETECT_HEADER_LEN`] and rewind.
pub fn read_detect_header<R: Read + Seek>(reader: &mut R) -> std::io::Result<Vec<u8>> {
    let mut buf = vec![0u8; DETECT_HEADER_LEN];
    let n = reader.read(&mut buf)?;
    buf.truncate(n);
    reader.seek(SeekFrom::Start(0))?;
    Ok(buf)
}

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

    /// Parse with an optional filename extension hint (ExifTool mixes magic + extension).
    ///
    /// Default ignores the hint. [`crate::TiffParser`] classifies TIFF-family FileType.
    fn parse_with_hint(
        &self,
        reader: &mut dyn ReadSeek,
        _ext_hint: Option<&str>,
    ) -> Result<Metadata> {
        self.parse(reader)
    }
}


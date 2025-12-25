//! File format parsers for image metadata extraction.
//!
//! This crate provides a unified interface for parsing EXIF, XMP, and other
//! metadata from various image file formats.
//!
//! # Supported Formats
//!
//! | Format | Extension | Parser | Writer |
//! |--------|-----------|--------|--------|
//! | JPEG | .jpg, .jpeg | [`JpegParser`] | [`JpegWriter`] |
//! | TIFF | .tiff, .tif, .dng | [`TiffParser`] | [`TiffWriter`] |
//! | PNG | .png | [`PngParser`] | [`PngWriter`] |
//! | HEIC/HEIF | .heic, .heif | [`HeicParser`] | - |
//! | Canon CR2 | .cr2 | [`Cr2Parser`] | - |
//! | Canon CR3 | .cr3 | [`Cr3Parser`] | - |
//! | Nikon NEF | .nef | [`NefParser`] | [`NefWriter`] |
//! | Sony ARW | .arw | [`ArwParser`] | - |
//! | Olympus ORF | .orf | [`OrfParser`] | - |
//! | Panasonic RW2 | .rw2 | [`Rw2Parser`] | - |
//! | Pentax PEF | .pef | [`PefParser`] | - |
//! | Fuji RAF | .raf | [`RafParser`] | [`RafWriter`] |
//! | WebP | .webp | [`WebpParser`] | - |
//! | OpenEXR | .exr | [`ExrParser`] | [`ExrWriter`] |
//! | Radiance HDR | .hdr | [`HdrParser`] | [`HdrWriter`] |
//!
//! # Quick Start
//!
//! ```no_run
//! use exiftool_formats::{FormatRegistry, FormatParser};
//! use std::io::Cursor;
//!
//! // Load file and auto-detect format
//! let data = std::fs::read("photo.jpg").unwrap();
//!
//! let registry = FormatRegistry::new();
//! // Detect format from first 16 bytes
//! if let Some(parser) = registry.detect(&data[..16.min(data.len())]) {
//!     let mut cursor = Cursor::new(&data);
//!     let metadata = parser.parse(&mut cursor).unwrap();
//!     println!("Format: {}", metadata.format);
//!     for (tag, value) in metadata.exif.iter() {
//!         println!("{}: {}", tag, value);
//!     }
//! }
//! ```
//!
//! # Direct Parser Usage
//!
//! ```no_run
//! use exiftool_formats::{JpegParser, FormatParser};
//! use std::io::Cursor;
//!
//! let jpeg_data: Vec<u8> = std::fs::read("photo.jpg").unwrap();
//! let mut cursor = Cursor::new(&jpeg_data);
//!
//! let parser = JpegParser;
//! let metadata = parser.parse(&mut cursor).unwrap();
//! println!("Camera: {:?}", metadata.exif.get("Make"));
//! ```

mod arw;
mod cr2;
mod cr3;
mod error;
mod exr;
mod exr_writer;
mod hdr;
mod hdr_writer;
mod heic;
mod jpeg;
mod jpeg_writer;
mod makernotes;
mod nef;
mod nef_writer;
mod orf;
mod pef;
mod png;
mod png_writer;
mod raf;
mod raf_writer;
mod registry;
mod rw2;
mod tag_lookup;
mod tiff;
mod tiff_writer;
mod traits;
mod utils;
mod webp;

pub use arw::ArwParser;
pub use cr2::Cr2Parser;
pub use cr3::Cr3Parser;
pub use error::{Error, Result};
pub use exr::ExrParser;
pub use hdr::HdrParser;
pub use heic::HeicParser;
pub use jpeg::JpegParser;
pub use nef::NefParser;
pub use orf::OrfParser;
pub use pef::PefParser;
pub use png::PngParser;
pub use raf::RafParser;
pub use rw2::Rw2Parser;
pub use tiff::{TiffConfig, TiffParser};
pub use webp::WebpParser;
pub use registry::FormatRegistry;
pub use traits::{FormatParser, FormatWriter, ReadSeek};
pub use jpeg_writer::JpegWriter;
pub use tiff_writer::TiffWriter;
pub use png_writer::PngWriter;
pub use exr_writer::ExrWriter;
pub use hdr_writer::HdrWriter;
pub use raf_writer::RafWriter;
pub use nef_writer::NefWriter;
pub use utils::{build_exif_bytes, entry_to_attr, read_with_limit, MAX_FILE_SIZE};

/// Metadata extracted from a file.
#[derive(Debug, Clone)]
#[must_use]
pub struct Metadata {
    /// File format name.
    pub format: &'static str,
    /// Parsed EXIF attributes.
    pub exif: exiftool_attrs::Attrs,
    /// Raw EXIF data offset in file.
    pub exif_offset: Option<usize>,
    /// XMP data (if present).
    pub xmp: Option<String>,
    /// Thumbnail data (if present).
    pub thumbnail: Option<Vec<u8>>,
}

impl Metadata {
    /// Create new empty metadata.
    pub fn new(format: &'static str) -> Self {
        Self {
            format,
            exif: exiftool_attrs::Attrs::new(),
            exif_offset: None,
            xmp: None,
            thumbnail: None,
        }
    }

    /// Check if this is a camera RAW file (not writable).
    /// 
    /// Detection methods:
    /// 1. By format name (ARW, CR2, CR3, NEF, ORF, RW2, PEF, RAF)
    /// 2. By Make tag for TIFF-based RAW (catches renamed files)
    pub fn is_camera_raw(&self) -> bool {
        // Known RAW format names
        const RAW_FORMATS: &[&str] = &[
            "ARW", "CR2", "CR3", "NEF", "ORF", "RW2", "PEF", "RAF"
        ];
        
        if RAW_FORMATS.contains(&self.format) {
            return true;
        }
        
        // TIFF-based RAW detection via Make tag
        if self.format == "TIFF" {
            if let Some(make) = self.exif.get_str("Make") {
                let make_lower = make.to_lowercase();
                const RAW_VENDORS: &[&str] = &[
                    "sony", "nikon", "canon", "fuji", "olympus",
                    "panasonic", "pentax", "leica", "ricoh", 
                    "hasselblad", "phase one", "samsung"
                ];
                return RAW_VENDORS.iter().any(|v| make_lower.contains(v));
            }
        }
        
        false
    }

    /// Check if this format supports writing.
    ///
    /// Writable: JPEG, PNG, TIFF, DNG, EXR, HDR
    /// Read-only: All RAW formats, HEIC, AVIF, WebP
    pub fn is_writable(&self) -> bool {
        const WRITABLE: &[&str] = &["JPEG", "PNG", "TIFF", "DNG", "EXR", "HDR"];
        WRITABLE.contains(&self.format) && !self.is_camera_raw()
    }
}

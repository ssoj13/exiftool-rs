//! EXIF tag definitions.
//!
//! This crate contains tag definitions for:
//! - Standard EXIF tags (IFD0, ExifIFD, GPS, Interop)
//! - Vendor MakerNotes (Canon, Nikon, Sony, Fuji, etc.)
//!
//! Tag tables are auto-generated from ExifTool Perl sources via xtask.

mod exif;
pub mod generated;
pub mod interp;

pub use exif::{TagDef, TagGroup, EXIF_TAGS, GPS_TAGS, IFD0_TAGS};

/// Well-known tag IDs re-exported from core.
pub use exiftool_core::ifd::tags;

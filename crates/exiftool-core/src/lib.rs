//! Core types and utilities for EXIF metadata parsing.
//!
//! This crate provides low-level primitives for EXIF/TIFF parsing:
//!
//! - [`ByteOrder`] - Big/little endian byte order handling
//! - [`ExifFormat`] - 18 EXIF data format types per specification
//! - [`IfdReader`] - IFD (Image File Directory) parser
//! - [`RawValue`] - Parsed raw values before type conversion
//! - [`ExifWriter`] - TIFF/EXIF structure builder
//!
//! # Example
//!
//! ```
//! use exiftool_core::{ByteOrder, IfdReader};
//!
//! // Parse TIFF header
//! let tiff_data = [
//!     0x49, 0x49,             // "II" = little-endian
//!     0x2A, 0x00,             // TIFF magic (42)
//!     0x08, 0x00, 0x00, 0x00, // IFD0 offset = 8
//! ];
//!
//! let reader = IfdReader::new(&tiff_data, ByteOrder::LittleEndian);
//! let ifd_offset = reader.parse_header().unwrap();
//! assert_eq!(ifd_offset, 8);
//! ```

mod byte_order;
mod error;
mod format;
pub mod ifd;
mod value;
pub mod writer;

pub use byte_order::ByteOrder;
pub use error::{Error, Result};
pub use format::ExifFormat;
pub use ifd::{IfdEntry, IfdReader};
pub use value::{RawValue, SRational, URational};
pub use writer::{ExifWriter, WriteEntry};

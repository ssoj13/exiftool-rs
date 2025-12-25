//! Error types for exiftool-core.

use thiserror::Error;

/// Core parsing errors.
#[derive(Debug, Error)]
#[must_use]
pub enum Error {
    #[error("unexpected end of data: need {need} bytes, have {have}")]
    UnexpectedEof { need: usize, have: usize },

    #[error("invalid TIFF magic: expected 0x002A or 0x002B, got 0x{0:04X}")]
    InvalidTiffMagic(u16),

    #[error("invalid byte order marker: expected 'II' or 'MM', got {0:?}")]
    InvalidByteOrder([u8; 2]),

    #[error("invalid EXIF format type: {0}")]
    InvalidFormat(u16),

    #[error("IFD offset {0} is out of bounds (max {1})")]
    IfdOffsetOutOfBounds(u32, usize),

    #[error("IFD entry count {0} exceeds maximum {1}")]
    TooManyIfdEntries(u16, u16),

    #[error("value offset {0} + size {1} exceeds data length {2}")]
    ValueOutOfBounds(u32, usize, usize),

    #[error("value size overflow: format size {format_size} * count {count} overflows")]
    ValueSizeOverflow { format_size: usize, count: u32 },

    #[error("IFD too large to serialize: size {0} exceeds u32::MAX")]
    IfdTooLarge(usize),

    #[error("recursive IFD reference detected at offset {0}")]
    RecursiveIfd(u32),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type alias for core operations.
pub type Result<T> = std::result::Result<T, Error>;

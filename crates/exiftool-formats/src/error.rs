//! Error types for format parsing.

use thiserror::Error;

/// Format parsing errors.
#[derive(Debug, Error)]
pub enum Error {
    #[error("unsupported format")]
    UnsupportedFormat,

    #[error("invalid file structure: {0}")]
    InvalidStructure(String),

    #[error("missing required segment: {0}")]
    MissingSegment(&'static str),

    #[error("core error: {0}")]
    Core(#[from] exiftool_core::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("file too large: {0} bytes exceeds limit of {1} bytes")]
    FileTooLarge(u64, u64),
}

/// Result type alias.
pub type Result<T> = std::result::Result<T, Error>;

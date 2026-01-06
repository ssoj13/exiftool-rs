//! ICC Profile parsing errors.

use thiserror::Error;

/// ICC Profile parsing errors.
#[derive(Debug, Error)]
pub enum Error {
    #[error("ICC profile too short: {0} bytes (minimum 128)")]
    TooShort(usize),

    #[error("invalid ICC profile signature")]
    InvalidSignature,

    #[error("invalid tag count: {0}")]
    InvalidTagCount(usize),

    #[error("invalid header field: {0}")]
    InvalidHeader(String),
}

/// Result type alias.
pub type Result<T> = std::result::Result<T, Error>;

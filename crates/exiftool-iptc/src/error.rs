//! IPTC error types.

use thiserror::Error;

/// IPTC parsing/writing error.
#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid IPTC structure: {0}")]
    InvalidStructure(String),

    #[error("unknown IPTC tag: {0}")]
    UnknownTag(String),

    #[error("invalid tag value: {0}")]
    InvalidValue(String),

    #[error("encoding error: {0}")]
    Encoding(String),
}

/// Result type for IPTC operations.
pub type Result<T> = std::result::Result<T, Error>;

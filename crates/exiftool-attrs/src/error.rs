//! Error types for exiftool-attrs.

use thiserror::Error;

/// Attribute errors.
#[derive(Debug, Error)]
pub enum Error {
    #[error("type mismatch: expected {expected}, got {actual}")]
    TypeMismatch {
        expected: &'static str,
        actual: &'static str,
    },

    #[error("attribute not found: {0}")]
    NotFound(String),

    #[error("validation failed for attribute '{key}': {message}")]
    Validation { key: String, message: String },

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Result type alias.
pub type Result<T> = std::result::Result<T, Error>;

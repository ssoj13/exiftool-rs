//! XMP parsing errors.

use thiserror::Error;

/// XMP parsing errors.
#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid XMP structure: {0}")]
    InvalidStructure(String),

    #[error("missing RDF root element")]
    MissingRdf,

    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::Error),

    #[error("IO error: {0}")]
    Io(String),
}

/// Result type alias.
pub type Result<T> = std::result::Result<T, Error>;

//! Python exception types.

use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

// Base exception
create_exception!(exiftool_py, ExifError, PyException, "Base exception for exif-tool-rs errors.");

// Specific exceptions
create_exception!(exiftool_py, FormatError, ExifError, "Error parsing image format. File may be corrupted or unsupported.");
create_exception!(exiftool_py, WriteError, ExifError, "Error writing metadata. Format may not support writing.");
create_exception!(exiftool_py, TagError, ExifError, "Error with tag name or value.");

/// Convert Rust error to Python exception with helpful message.
pub fn to_py_err(e: exiftool_formats::Error, path: Option<&str>) -> PyErr {
    let msg = match &e {
        exiftool_formats::Error::Io(io_err) => {
            if let Some(p) = path {
                format!("Cannot read '{}': {}", p, io_err)
            } else {
                format!("IO error: {}", io_err)
            }
        }
        exiftool_formats::Error::InvalidStructure(s) => {
            if let Some(p) = path {
                format!("Cannot parse '{}': {}. File may be corrupted.", p, s)
            } else {
                format!("Invalid structure: {}", s)
            }
        }
        exiftool_formats::Error::UnsupportedFormat => {
            if let Some(p) = path {
                format!(
                    "Unsupported format for '{}'. Supported: JPEG, PNG, TIFF, DNG, HEIC, AVIF, CR2, CR3, NEF, RAF, EXR, HDR",
                    p
                )
            } else {
                "Unsupported format. Supported: JPEG, PNG, TIFF, DNG, HEIC, AVIF, CR2, CR3, NEF, RAF, EXR, HDR".into()
            }
        }
        _ => format!("{}", e),
    };
    FormatError::new_err(msg)
}

/// Create WriteError for unsupported format.
pub fn write_not_supported(format: &str) -> PyErr {
    WriteError::new_err(format!(
        "Cannot write to {} format. Supported for writing: JPEG, PNG, TIFF, DNG",
        format
    ))
}

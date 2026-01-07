//! Python bindings for exiftool-rs.
//!
//! Provides Pythonic object model for reading/writing image metadata.

mod error;
mod gps;
mod image;
mod rational;
mod scan;
mod value;

use pyo3::prelude::*;

/// exif_tool_rs - Fast image metadata library
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Main functions
    m.add_function(wrap_pyfunction!(open, m)?)?;
    m.add_function(wrap_pyfunction!(scan::scan, m)?)?;
    m.add_function(wrap_pyfunction!(scan::scan_dir, m)?)?;

    // Classes
    m.add_class::<image::PyImage>()?;
    m.add_class::<image::PyPageInfo>()?;
    m.add_class::<rational::PyRational>()?;
    m.add_class::<gps::PyGPS>()?;
    m.add_class::<scan::PyScanResult>()?;
    m.add_class::<scan::ScanError>()?;

    // Exceptions
    m.add("ExifError", m.py().get_type::<error::ExifError>())?;
    m.add("FormatError", m.py().get_type::<error::FormatError>())?;
    m.add("WriteError", m.py().get_type::<error::WriteError>())?;
    m.add("TagError", m.py().get_type::<error::TagError>())?;

    Ok(())
}

/// Open an image file and parse its metadata.
///
/// Args:
///     path: Path to the image file
///
/// Returns:
///     PyImage object with metadata
///
/// Raises:
///     FormatError: If the file cannot be parsed
#[pyfunction]
fn open(path: &str) -> PyResult<image::PyImage> {
    image::PyImage::open(path)
}

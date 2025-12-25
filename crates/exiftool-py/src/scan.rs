//! Parallel file scanning with rayon.

use glob::glob;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::path::{Path, PathBuf};

use crate::error::FormatError;
use crate::image::PyImage;

/// Scan files matching glob pattern.
/// Returns iterator of PyImage objects for successfully parsed files.
///
/// Args:
///     pattern: Glob pattern like "photos/**/*.jpg"
///     parallel: Use parallel processing (default: True)
///     ignore_errors: Skip files that fail to parse (default: True)
///
/// Example:
///     for img in exif.scan("photos/**/*.jpg"):
///         print(img.make, img.model)
#[pyfunction]
#[pyo3(signature = (pattern, parallel = true, ignore_errors = true))]
pub fn scan(
    py: Python<'_>,
    pattern: &str,
    parallel: bool,
    ignore_errors: bool,
) -> PyResult<Py<PyScanResult>> {
    // Collect matching paths
    let paths: Vec<PathBuf> = glob(pattern)
        .map_err(|e| FormatError::new_err(format!("Invalid glob pattern: {e}")))?
        .filter_map(|entry| entry.ok())
        .filter(|p| p.is_file())
        .collect();

    // Process files
    let results: Vec<Option<PyImage>> = if parallel {
        paths
            .par_iter()
            .map(|path| try_open_image(path, ignore_errors))
            .collect()
    } else {
        paths
            .iter()
            .map(|path| try_open_image(path, ignore_errors))
            .collect()
    };

    // Filter out None values
    let images: Vec<PyImage> = results.into_iter().flatten().collect();

    Py::new(py, PyScanResult { images, index: 0 })
}

/// Try to open image, returning None on error if ignore_errors is true.
fn try_open_image(path: &Path, ignore_errors: bool) -> Option<PyImage> {
    let path_str = path.to_string_lossy();
    match PyImage::open(&path_str) {
        Ok(img) => Some(img),
        Err(_) if ignore_errors => None,
        Err(_) => None, // TODO: collect errors for reporting
    }
}

/// Iterator over scanned images.
#[pyclass]
pub struct PyScanResult {
    images: Vec<PyImage>,
    index: usize,
}

#[pymethods]
impl PyScanResult {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyImage> {
        if slf.index < slf.images.len() {
            let img = slf.images[slf.index].clone();
            slf.index += 1;
            Some(img)
        } else {
            None
        }
    }

    fn __len__(&self) -> usize {
        self.images.len()
    }

    /// Get all images as list.
    fn to_list(&self) -> Vec<PyImage> {
        self.images.clone()
    }

    /// Number of successfully parsed files.
    #[getter]
    fn count(&self) -> usize {
        self.images.len()
    }
}

/// Scan single directory (non-recursive).
#[pyfunction]
#[pyo3(signature = (directory, extensions = None, parallel = true))]
pub fn scan_dir(
    py: Python<'_>,
    directory: &str,
    extensions: Option<Vec<String>>,
    parallel: bool,
) -> PyResult<Py<PyScanResult>> {
    let exts = extensions.unwrap_or_else(|| {
        vec![
            "jpg".into(), "jpeg".into(), "png".into(), 
            "tiff".into(), "tif".into(), "heic".into(),
            "cr2".into(), "cr3".into(), "nef".into(),
            "arw".into(), "dng".into(),
        ]
    });

    let mut all_paths: Vec<PathBuf> = Vec::new();
    for ext in &exts {
        // Lowercase
        let pattern = format!("{}/*.{}", directory, ext.to_lowercase());
        if let Ok(entries) = glob(&pattern) {
            for entry in entries.flatten() {
                if entry.is_file() {
                    all_paths.push(entry);
                }
            }
        }
        // Uppercase
        let upper = format!("{}/*.{}", directory, ext.to_uppercase());
        if let Ok(entries) = glob(&upper) {
            for entry in entries.flatten() {
                if entry.is_file() && !all_paths.contains(&entry) {
                    all_paths.push(entry);
                }
            }
        }
    }

    // Process
    let results: Vec<Option<PyImage>> = if parallel {
        all_paths
            .par_iter()
            .map(|path| try_open_image(path, true))
            .collect()
    } else {
        all_paths
            .iter()
            .map(|path| try_open_image(path, true))
            .collect()
    };

    let images: Vec<PyImage> = results.into_iter().flatten().collect();
    Py::new(py, PyScanResult { images, index: 0 })
}

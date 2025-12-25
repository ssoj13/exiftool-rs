//! PyImage - main class for image metadata.

use crate::error::{to_py_err, write_not_supported};
use crate::gps::PyGPS;
use crate::rational::PyRational;
use crate::value::{display_value, from_python, to_python};
use exiftool_attrs::AttrValue;
use exiftool_formats::{build_exif_bytes, FormatRegistry, JpegWriter, Metadata, PngWriter, TiffWriter};
use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// Image metadata object.
///
/// Provides dict-like access to EXIF tags and properties for common fields.
#[pyclass(name = "Image")]
#[derive(Clone)]
pub struct PyImage {
    pub metadata: Metadata,
    pub path: Option<PathBuf>,
}

#[pymethods]
impl PyImage {
    // === Properties for common tags ===

    /// File format (JPEG, PNG, TIFF, etc.)
    #[getter]
    fn format(&self) -> &str {
        self.metadata.format
    }

    /// File path (if opened from file).
    #[getter]
    fn path(&self) -> Option<String> {
        self.path.as_ref().map(|p| p.display().to_string())
    }

    /// Camera make.
    #[getter]
    fn make(&self) -> Option<String> {
        self.metadata.exif.get_str("Make").map(|s| s.to_string())
    }

    #[setter]
    fn set_make(&mut self, value: &str) {
        self.metadata.exif.set("Make", AttrValue::Str(value.into()));
    }

    /// Camera model.
    #[getter]
    fn model(&self) -> Option<String> {
        self.metadata.exif.get_str("Model").map(|s| s.to_string())
    }

    #[setter]
    fn set_model(&mut self, value: &str) {
        self.metadata.exif.set("Model", AttrValue::Str(value.into()));
    }

    /// Software used.
    #[getter]
    fn software(&self) -> Option<String> {
        self.metadata.exif.get_str("Software").map(|s| s.to_string())
    }

    #[setter]
    fn set_software(&mut self, value: &str) {
        self.metadata.exif.set("Software", AttrValue::Str(value.into()));
    }

    /// Artist/author.
    #[getter]
    fn artist(&self) -> Option<String> {
        self.metadata.exif.get_str("Artist").map(|s| s.to_string())
    }

    #[setter]
    fn set_artist(&mut self, value: &str) {
        self.metadata.exif.set("Artist", AttrValue::Str(value.into()));
    }

    /// Copyright.
    #[getter]
    fn copyright(&self) -> Option<String> {
        self.metadata.exif.get_str("Copyright").map(|s| s.to_string())
    }

    #[setter]
    fn set_copyright(&mut self, value: &str) {
        self.metadata.exif.set("Copyright", AttrValue::Str(value.into()));
    }

    /// Image description.
    #[getter]
    fn description(&self) -> Option<String> {
        self.metadata.exif.get_str("ImageDescription").map(|s| s.to_string())
    }

    #[setter]
    fn set_description(&mut self, value: &str) {
        self.metadata.exif.set("ImageDescription", AttrValue::Str(value.into()));
    }

    /// ISO sensitivity.
    #[getter]
    fn iso(&self) -> Option<u32> {
        self.metadata.exif.get_u32("ISO")
    }

    /// Exposure time as Rational.
    #[getter]
    fn exposure_time(&self) -> Option<PyRational> {
        self.metadata.exif.get_urational("ExposureTime")
            .map(|(n, d)| PyRational::from_unsigned(n, d))
    }

    /// F-number as Rational.
    #[getter]
    fn fnumber(&self) -> Option<PyRational> {
        self.metadata.exif.get_urational("FNumber")
            .map(|(n, d)| PyRational::from_unsigned(n, d))
    }

    /// Focal length as Rational.
    #[getter]
    fn focal_length(&self) -> Option<PyRational> {
        self.metadata.exif.get_urational("FocalLength")
            .map(|(n, d)| PyRational::from_unsigned(n, d))
    }

    /// Focal length in 35mm equivalent.
    #[getter]
    fn focal_length_35mm(&self) -> Option<u32> {
        self.metadata.exif.get_u32("FocalLengthIn35mmFormat")
    }

    /// Date/time original (when photo was taken).
    #[getter]
    fn date_time_original(&self) -> Option<String> {
        self.metadata.exif.get_str("DateTimeOriginal").map(|s| s.to_string())
    }

    /// Orientation (1-8).
    #[getter]
    fn orientation(&self) -> Option<u32> {
        self.metadata.exif.get_u32("Orientation")
    }

    /// Image width.
    #[getter]
    fn width(&self) -> Option<u32> {
        self.metadata.exif.get_u32("ImageWidth")
            .or_else(|| self.metadata.exif.get_u32("ExifImageWidth"))
    }

    /// Image height.
    #[getter]
    fn height(&self) -> Option<u32> {
        self.metadata.exif.get_u32("ImageHeight")
            .or_else(|| self.metadata.exif.get_u32("ExifImageHeight"))
    }

    /// GPS coordinates (if available).
    #[getter]
    fn gps(&self) -> Option<PyGPS> {
        PyGPS::from_attrs(&self.metadata.exif)
    }

    /// Raw XMP data.
    #[getter]
    fn xmp(&self) -> Option<String> {
        self.metadata.xmp.clone()
    }

    /// Thumbnail bytes.
    #[getter]
    fn thumbnail(&self) -> Option<Vec<u8>> {
        self.metadata.thumbnail.clone()
    }

    /// Check if this is a camera RAW file.
    /// 
    /// RAW files are detected by format (ARW, CR2, NEF, etc.) or by
    /// Make tag for TIFF-based RAW files.
    #[getter]
    fn is_camera_raw(&self) -> bool {
        self.metadata.is_camera_raw()
    }

    /// Check if this format supports writing.
    /// 
    /// Writable: JPEG, PNG, TIFF, DNG, EXR, HDR
    /// Read-only: All RAW formats, HEIC, AVIF, WebP
    #[getter]
    fn is_writable(&self) -> bool {
        self.metadata.is_writable()
    }

    /// Number of tags.
    fn __len__(&self) -> usize {
        self.metadata.exif.len()
    }

    /// Check if tag exists.
    fn __contains__(&self, key: &str) -> bool {
        self.metadata.exif.contains(key)
    }

    /// Get tag by name.
    fn __getitem__(&self, py: Python<'_>, key: &str) -> PyResult<Py<PyAny>> {
        match self.metadata.exif.get(key) {
            Some(v) => to_python(py, v),
            None => Err(PyKeyError::new_err(format!("Tag '{}' not found", key))),
        }
    }

    /// Set tag value.
    fn __setitem__(&mut self, key: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        let attr = from_python(value)?;
        self.metadata.exif.set(key, attr);
        Ok(())
    }

    /// Delete tag.
    fn __delitem__(&mut self, key: &str) -> PyResult<()> {
        let _ = self.metadata.exif.remove(key)
            .ok_or_else(|| PyKeyError::new_err(format!("Tag '{}' not found", key)))?;
        Ok(())
    }

    /// Get tag with default.
    #[pyo3(signature = (key, default=None))]
    fn get(&self, py: Python<'_>, key: &str, default: Option<Py<PyAny>>) -> PyResult<Py<PyAny>> {
        match self.metadata.exif.get(key) {
            Some(v) => to_python(py, v),
            None => Ok(default.unwrap_or_else(|| py.None())),
        }
    }

    /// Get all tag names.
    fn keys(&self) -> Vec<String> {
        self.metadata.exif.iter().map(|(k, _)| k.clone()).collect()
    }

    /// Get all tag values.
    fn values(&self, py: Python<'_>) -> PyResult<Vec<Py<PyAny>>> {
        self.metadata.exif.iter()
            .map(|(_, v)| to_python(py, v))
            .collect()
    }

    /// Get all (key, value) pairs.
    fn items(&self, py: Python<'_>) -> PyResult<Vec<(String, Py<PyAny>)>> {
        self.metadata.exif.iter()
            .map(|(k, v)| Ok((k.clone(), to_python(py, v)?)))
            .collect()
    }

    /// Iterate over tag names.
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<TagIterator>> {
        let keys: Vec<String> = slf.metadata.exif.iter().map(|(k, _)| k.clone()).collect();
        Py::new(slf.py(), TagIterator { keys, index: 0 })
    }

    /// String representation.
    fn __repr__(&self) -> String {
        let path_str = self.path.as_ref()
            .map(|p| format!("'{}'", p.display()))
            .unwrap_or_else(|| "memory".to_string());
        format!("Image({}, format={}, tags={})", path_str, self.metadata.format, self.metadata.exif.len())
    }

    /// Pretty print all tags.
    fn __str__(&self) -> String {
        let mut lines = vec![format!("Format: {}", self.metadata.format)];
        
        let mut entries: Vec<_> = self.metadata.exif.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        
        for (k, v) in entries {
            lines.push(format!("{}: {}", k, display_value(v)));
        }
        
        lines.join("\n")
    }

    /// Convert to dict.
    fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let dict = PyDict::new(py);
        dict.set_item("format", self.metadata.format)?;
        
        for (k, v) in self.metadata.exif.iter() {
            dict.set_item(k, to_python(py, v)?)?;
        }
        
        if let Some(ref xmp) = self.metadata.xmp {
            dict.set_item("_xmp", xmp)?;
        }
        
        Ok(dict.into_any().unbind())
    }

    /// Save metadata to file.
    ///
    /// Args:
    ///     path: Optional output path. If None, overwrites original file.
    #[pyo3(signature = (path=None))]
    fn save(&self, path: Option<&str>) -> PyResult<()> {
        let output_path = match path {
            Some(p) => PathBuf::from(p),
            None => self.path.clone().ok_or_else(|| {
                crate::error::WriteError::new_err("No path specified and image was not opened from file")
            })?,
        };

        let input_path = self.path.as_ref().ok_or_else(|| {
            crate::error::WriteError::new_err("Cannot save: image was not opened from file")
        })?;

        // Check if format is writable
        if !self.metadata.is_writable() {
            let reason = if self.metadata.is_camera_raw() {
                let make = self.metadata.exif.get_str("Make").unwrap_or(self.metadata.format);
                format!("Camera RAW file ({}) is read-only", make.trim())
            } else {
                format!("Format {} does not support writing", self.metadata.format)
            };
            return Err(crate::error::WriteError::new_err(format!(
                "{}. Writable formats: JPEG, PNG, TIFF, DNG, EXR, HDR", reason
            )));
        }

        // Read original file
        let file = File::open(input_path)
            .map_err(|e| crate::error::WriteError::new_err(format!("Cannot read '{}': {}", input_path.display(), e)))?;
        let mut reader = BufReader::new(file);
        let mut output_data = Vec::new();

        match self.metadata.format {
            "JPEG" => {
                let exif = build_exif_bytes(&self.metadata)
                    .map_err(|e| crate::error::WriteError::new_err(format!("EXIF build failed: {}", e)))?;
                JpegWriter::write(&mut reader, &mut output_data, Some(&exif), None)
                    .map_err(|e| crate::error::WriteError::new_err(format!("JPEG write failed: {}", e)))?;
            }
            "PNG" => {
                PngWriter::write(&mut reader, &mut output_data, &self.metadata)
                    .map_err(|e| crate::error::WriteError::new_err(format!("PNG write failed: {}", e)))?;
            }
            "TIFF" | "DNG" => {
                TiffWriter::write(&mut reader, &mut output_data, &self.metadata)
                    .map_err(|e| crate::error::WriteError::new_err(format!("TIFF write failed: {}", e)))?;
            }
            fmt => return Err(write_not_supported(fmt)),
        }

        // Atomic write
        let tmp_path = output_path.with_extension("tmp");
        std::fs::write(&tmp_path, &output_data)
            .map_err(|e| crate::error::WriteError::new_err(format!("Cannot write '{}': {}", tmp_path.display(), e)))?;
        std::fs::rename(&tmp_path, &output_path)
            .map_err(|e| crate::error::WriteError::new_err(format!("Cannot rename to '{}': {}", output_path.display(), e)))?;

        Ok(())
    }

    // Context manager support
    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[pyo3(signature = (_exc_type=None, _exc_val=None, _exc_tb=None))]
    fn __exit__(
        &self,
        _exc_type: Option<Py<PyAny>>,
        _exc_val: Option<Py<PyAny>>,
        _exc_tb: Option<Py<PyAny>>,
    ) -> bool {
        false // Don't suppress exceptions
    }
}

impl PyImage {
    /// Open image from file path.
    pub fn open(path: &str) -> PyResult<Self> {
        let path_buf = PathBuf::from(path);
        let file = File::open(&path_buf)
            .map_err(|e| crate::error::FormatError::new_err(format!("Cannot open '{}': {}", path, e)))?;
        let mut reader = BufReader::new(file);

        let registry = FormatRegistry::new();
        let metadata = registry.parse(&mut reader)
            .map_err(|e| to_py_err(e, Some(path)))?;

        Ok(Self {
            metadata,
            path: Some(path_buf),
        })
    }
}

/// Iterator over tag names.
#[pyclass]
struct TagIterator {
    keys: Vec<String>,
    index: usize,
}

#[pymethods]
impl TagIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<String> {
        if self.index < self.keys.len() {
            let key = self.keys[self.index].clone();
            self.index += 1;
            Some(key)
        } else {
            None
        }
    }
}



//! PyImage - main class for image metadata.

use crate::error::{to_py_err, write_not_supported};
use crate::gps::PyGPS;
use crate::gpx::{parse_exif_datetime, shift_datetime, PyGpxTrack};
use crate::rational::PyRational;
use crate::value::{display_value, from_python, to_python};
use exiftool_attrs::AttrValue;
use exiftool_formats::{
    add_composite_tags, build_exif_bytes, ExrWriter, FormatRegistry, HdrWriter, HeicWriter,
    JpegWriter, Metadata, PageInfo, PngWriter, TiffWriter, WebpWriter,
};
use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::fs::File;
use std::io::{BufReader, Cursor};
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

    /// Thumbnail bytes (small preview, typically 160x120).
    #[getter]
    fn thumbnail(&self) -> Option<Vec<u8>> {
        self.metadata.thumbnail.clone()
    }

    /// Preview image bytes (larger preview from RAW files).
    #[getter]
    fn preview(&self) -> Option<Vec<u8>> {
        self.metadata.preview.clone()
    }

    /// Number of pages/frames in the file.
    #[getter]
    fn page_count(&self) -> usize {
        self.metadata.page_count()
    }

    /// Check if file has multiple pages (multi-page TIFF, etc.).
    #[getter]
    fn is_multi_page(&self) -> bool {
        self.metadata.is_multi_page()
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

    /// Page info for multi-page files (TIFF, etc.).
    #[getter]
    fn pages(&self) -> Vec<PyPageInfo> {
        self.metadata.pages.iter().map(PyPageInfo::from).collect()
    }

    /// Raw EXIF data offset in file (if available).
    #[getter]
    fn exif_offset(&self) -> Option<usize> {
        self.metadata.exif_offset
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

    /// Get human-readable interpretation of a tag value.
    ///
    /// Example:
    ///     img["Orientation"]              # 6 (raw value)
    ///     img.get_interpreted("Orientation")  # "Rotate 90 CW"
    fn get_interpreted(&self, key: &str) -> Option<String> {
        self.metadata.get_interpreted(key)
    }

    /// Get formatted display value of a tag.
    ///
    /// Similar to get_interpreted but with unit formatting.
    fn get_display(&self, key: &str) -> Option<String> {
        self.metadata.get_display(key)
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

    /// Clear all EXIF tags.
    fn clear(&mut self) {
        self.metadata.exif.clear();
    }

    /// Shift all DateTime tags by offset.
    ///
    /// Args:
    ///     offset: Offset string like "+2:30" (hours:minutes) or "-30" (minutes)
    ///
    /// Example:
    ///     img.shift_time("+2:00")  # Add 2 hours
    ///     img.shift_time("-30")    # Subtract 30 minutes
    fn shift_time(&mut self, offset: &str) -> PyResult<()> {
        let offset_secs = parse_shift(offset).ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(
                format!("Invalid offset format: {}. Use +/-HH:MM or +/-MM", offset)
            )
        })?;

        let datetime_tags = [
            "DateTime", "DateTimeOriginal", "CreateDate", "ModifyDate",
            "DateTimeDigitized", "GPSDateTime", "GPSDateStamp",
        ];

        for tag in &datetime_tags {
            if let Some(val) = self.metadata.exif.get(*tag) {
                if let Some(s) = val.as_str() {
                    if let Some(shifted) = shift_datetime(s, offset_secs) {
                        self.metadata.exif.set(*tag, AttrValue::Str(shifted));
                    }
                }
            }
        }

        Ok(())
    }

    /// Add GPS coordinates from a GPX track file.
    ///
    /// Matches photo timestamp (DateTimeOriginal) to track points.
    ///
    /// Args:
    ///     gpx_path: Path to GPX file
    ///
    /// Returns:
    ///     Tuple of (lat, lon) if matched, None if no match
    ///
    /// Example:
    ///     coords = img.geotag("track.gpx")
    ///     if coords:
    ///         print(f"Geotagged to {coords[0]}, {coords[1]}")
    fn geotag(&mut self, gpx_path: &str) -> PyResult<Option<(f64, f64)>> {
        let track = PyGpxTrack::from_file(gpx_path)?;

        // Get photo timestamp
        let timestamp = self.metadata.exif.get("DateTimeOriginal")
            .or_else(|| self.metadata.exif.get("CreateDate"))
            .and_then(|v| v.as_str())
            .and_then(|s| parse_exif_datetime(s));

        let ts = match timestamp {
            Some(t) => t,
            None => return Ok(None),
        };

        if let Some((lat, lon, ele)) = track.find_position(ts) {
            let lat_ref = if lat >= 0.0 { "N" } else { "S" };
            let lon_ref = if lon >= 0.0 { "E" } else { "W" };

            self.metadata.exif.set("GPSLatitude", AttrValue::Double(f64::abs(lat)));
            self.metadata.exif.set("GPSLatitudeRef", AttrValue::Str(lat_ref.to_string()));
            self.metadata.exif.set("GPSLongitude", AttrValue::Double(f64::abs(lon)));
            self.metadata.exif.set("GPSLongitudeRef", AttrValue::Str(lon_ref.to_string()));

            if let Some(altitude) = ele {
                let alt_ref = if altitude >= 0.0 { 0u32 } else { 1u32 };
                self.metadata.exif.set("GPSAltitude", AttrValue::Double(f64::abs(altitude)));
                self.metadata.exif.set("GPSAltitudeRef", AttrValue::UInt(alt_ref));
            }

            Ok(Some((lat, lon)))
        } else {
            Ok(None)
        }
    }

    /// Set ICC color profile.
    ///
    /// Args:
    ///     data: ICC profile bytes, or None to remove
    #[setter]
    fn set_icc(&mut self, data: Option<Vec<u8>>) {
        self.metadata.icc = data;
    }

    /// Get ICC color profile bytes.
    #[getter]
    fn icc(&self) -> Option<Vec<u8>> {
        self.metadata.icc.clone()
    }

    /// Load and set ICC profile from file.
    ///
    /// Args:
    ///     path: Path to .icc or .icm profile file
    fn set_icc_from_file(&mut self, path: &str) -> PyResult<()> {
        let data = std::fs::read(path).map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("Cannot read ICC profile: {}", e))
        })?;
        self.metadata.icc = Some(data);
        Ok(())
    }

    /// Add computed/composite tags (ImageSize, Megapixels, etc.).
    ///
    /// Adds tags like:
    /// - ImageSize: "4000x3000"
    /// - Megapixels: 12.0
    /// - GPSAltitude (combined with ref)
    /// - DateTimeOriginal (with SubSec)
    fn add_composite(&mut self) {
        add_composite_tags(&mut self.metadata);
    }

    /// Create Image from bytes.
    ///
    /// Args:
    ///     data: Raw file bytes
    ///
    /// Returns:
    ///     Image object with parsed metadata
    #[staticmethod]
    fn from_bytes(data: &[u8]) -> PyResult<Self> {
        let mut cursor = Cursor::new(data);
        let registry = FormatRegistry::new();
        let metadata = registry.parse(&mut cursor)
            .map_err(|e| to_py_err(e, None))?;

        Ok(Self {
            metadata,
            path: None,
        })
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
                "{}. Writable formats: JPEG, PNG, TIFF, DNG, WebP, HEIC, EXR, HDR", reason
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
            "WebP" => {
                WebpWriter::write(&mut reader, &mut output_data, &self.metadata)
                    .map_err(|e| crate::error::WriteError::new_err(format!("WebP write failed: {}", e)))?;
            }
            "HEIC" | "HEIF" | "AVIF" => {
                HeicWriter::write(&mut reader, &mut output_data, &self.metadata)
                    .map_err(|e| crate::error::WriteError::new_err(format!("HEIC write failed: {}", e)))?;
            }
            "EXR" => {
                ExrWriter::write(&mut reader, &mut output_data, &self.metadata)
                    .map_err(|e| crate::error::WriteError::new_err(format!("EXR write failed: {}", e)))?;
            }
            "HDR" => {
                HdrWriter::write(&mut reader, &mut output_data, &self.metadata)
                    .map_err(|e| crate::error::WriteError::new_err(format!("HDR write failed: {}", e)))?;
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

/// Page info for multi-page files (TIFF, etc.).
#[pyclass(name = "PageInfo")]
#[derive(Clone)]
pub struct PyPageInfo {
    /// Page index (0-based).
    #[pyo3(get)]
    pub index: usize,
    /// Image width in pixels.
    #[pyo3(get)]
    pub width: u32,
    /// Image height in pixels.
    #[pyo3(get)]
    pub height: u32,
    /// Bits per sample.
    #[pyo3(get)]
    pub bits_per_sample: u16,
    /// Compression type.
    #[pyo3(get)]
    pub compression: u16,
    /// Subfile type.
    #[pyo3(get)]
    pub subfile_type: u32,
}

#[pymethods]
impl PyPageInfo {
    /// Check if this is a thumbnail/reduced resolution image.
    #[getter]
    fn is_thumbnail(&self) -> bool {
        self.subfile_type & 1 != 0
    }

    /// Check if this is a page of a multi-page document.
    #[getter]
    fn is_page(&self) -> bool {
        self.subfile_type & 2 != 0
    }

    fn __repr__(&self) -> String {
        format!("PageInfo(index={}, {}x{}, bits={})",
            self.index, self.width, self.height, self.bits_per_sample)
    }
}

impl From<&PageInfo> for PyPageInfo {
    fn from(p: &PageInfo) -> Self {
        Self {
            index: p.index,
            width: p.width,
            height: p.height,
            bits_per_sample: p.bits_per_sample,
            compression: p.compression,
            subfile_type: p.subfile_type,
        }
    }
}

/// Parse time shift string to seconds.
/// Formats: "+2:30" (hours:minutes), "-30" (minutes), "+1" (minutes)
fn parse_shift(s: &str) -> Option<i64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let (sign, rest) = if s.starts_with('+') {
        (1i64, &s[1..])
    } else if s.starts_with('-') {
        (-1i64, &s[1..])
    } else {
        (1i64, s)
    };

    if rest.contains(':') {
        let parts: Vec<&str> = rest.split(':').collect();
        if parts.len() != 2 {
            return None;
        }
        let hours: i64 = parts[0].parse().ok()?;
        let minutes: i64 = parts[1].parse().ok()?;
        Some(sign * (hours * 3600 + minutes * 60))
    } else {
        let minutes: i64 = rest.parse().ok()?;
        Some(sign * minutes * 60)
    }
}


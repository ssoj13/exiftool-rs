//! XMP sidecar file support (.xmp files).
//!
//! XMP sidecar files store metadata externally alongside image files.
//! Common pattern: photo.jpg -> photo.xmp

use crate::{Error, Result, XmpParser, XmpWriter};
use exiftool_attrs::Attrs;
use std::path::Path;

/// XMP sidecar file operations.
pub struct XmpSidecar;

impl XmpSidecar {
    /// Get sidecar path for an image file.
    ///
    /// # Example
    /// ```
    /// use exiftool_xmp::XmpSidecar;
    /// use std::path::Path;
    ///
    /// let sidecar = XmpSidecar::sidecar_path(Path::new("photo.jpg"));
    /// assert_eq!(sidecar.to_str().unwrap(), "photo.xmp");
    /// ```
    pub fn sidecar_path(image_path: &Path) -> std::path::PathBuf {
        image_path.with_extension("xmp")
    }

    /// Check if sidecar file exists for given image.
    pub fn exists(image_path: &Path) -> bool {
        Self::sidecar_path(image_path).exists()
    }

    /// Read XMP metadata from sidecar file.
    ///
    /// Returns `None` if sidecar doesn't exist.
    pub fn read(image_path: &Path) -> Result<Option<Attrs>> {
        let sidecar_path = Self::sidecar_path(image_path);
        if !sidecar_path.exists() {
            return Ok(None);
        }
        Self::read_file(&sidecar_path).map(Some)
    }

    /// Read XMP from .xmp file directly.
    pub fn read_file(xmp_path: &Path) -> Result<Attrs> {
        let content = std::fs::read_to_string(xmp_path)
            .map_err(|e| Error::Io(e.to_string()))?;
        XmpParser::parse(&content)
    }

    /// Write XMP metadata to sidecar file.
    ///
    /// Creates or overwrites the sidecar file.
    pub fn write(image_path: &Path, attrs: &Attrs) -> Result<()> {
        let sidecar_path = Self::sidecar_path(image_path);
        Self::write_file(&sidecar_path, attrs)
    }

    /// Write XMP to .xmp file directly.
    pub fn write_file(xmp_path: &Path, attrs: &Attrs) -> Result<()> {
        let xmp_xml = XmpWriter::write(attrs)?;
        std::fs::write(xmp_path, xmp_xml)
            .map_err(|e| Error::Io(e.to_string()))?;
        Ok(())
    }

    /// Merge sidecar XMP with embedded metadata.
    ///
    /// Sidecar values override embedded values.
    pub fn merge(embedded: &Attrs, sidecar: &Attrs) -> Attrs {
        let mut result = embedded.clone();
        // Sidecar values take precedence
        for (key, value) in sidecar.iter() {
            result.set(key.clone(), value.clone());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use exiftool_attrs::AttrValue;
    use std::path::PathBuf;

    #[test]
    fn test_sidecar_path() {
        assert_eq!(
            XmpSidecar::sidecar_path(Path::new("photo.jpg")),
            PathBuf::from("photo.xmp")
        );
        assert_eq!(
            XmpSidecar::sidecar_path(Path::new("/path/to/image.CR2")),
            PathBuf::from("/path/to/image.xmp")
        );
        assert_eq!(
            XmpSidecar::sidecar_path(Path::new("no_ext")),
            PathBuf::from("no_ext.xmp")
        );
    }

    #[test]
    fn test_roundtrip() {
        let temp_dir = std::env::temp_dir();
        let xmp_path = temp_dir.join("test_sidecar.xmp");

        let mut attrs = Attrs::new();
        // XMP stores everything as strings in XML
        attrs.set("XMP:Rating".to_string(), AttrValue::Str("5".to_string()));
        attrs.set("XMP:Label".to_string(), AttrValue::Str("Red".to_string()));
        attrs.set(
            "DC:Creator".to_string(),
            AttrValue::List(vec![
                AttrValue::Str("Alice".to_string()),
                AttrValue::Str("Bob".to_string()),
            ]),
        );

        // Write
        XmpSidecar::write_file(&xmp_path, &attrs).unwrap();

        // Read back
        let read_attrs = XmpSidecar::read_file(&xmp_path).unwrap();

        // Check string values (XMP returns everything as strings)
        assert_eq!(read_attrs.get_str("XMP:Rating"), Some("5"));
        assert_eq!(read_attrs.get_str("XMP:Label"), Some("Red"));

        // Cleanup
        let _ = std::fs::remove_file(&xmp_path);
    }

    #[test]
    fn test_merge() {
        let mut embedded = Attrs::new();
        embedded.set("XMP:Rating".to_string(), AttrValue::Int(3));
        embedded.set("DC:Title".to_string(), AttrValue::Str("Original".to_string()));

        let mut sidecar = Attrs::new();
        sidecar.set("XMP:Rating".to_string(), AttrValue::Int(5)); // Override
        sidecar.set("XMP:Label".to_string(), AttrValue::Str("Green".to_string())); // New

        let merged = XmpSidecar::merge(&embedded, &sidecar);

        // Sidecar value wins
        assert_eq!(merged.get("XMP:Rating"), Some(&AttrValue::Int(5)));
        // Original preserved
        assert_eq!(
            merged.get("DC:Title"),
            Some(&AttrValue::Str("Original".to_string()))
        );
        // New from sidecar
        assert_eq!(
            merged.get("XMP:Label"),
            Some(&AttrValue::Str("Green".to_string()))
        );
    }
}

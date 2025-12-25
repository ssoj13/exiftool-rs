//! GPS coordinate wrapper.

use exiftool_attrs::{AttrValue, Attrs};
use pyo3::prelude::*;

/// GPS coordinates extracted from EXIF.
///
/// Provides both decimal degrees (for APIs) and raw DMS values.
#[pyclass(name = "GPS")]
#[derive(Clone)]
pub struct PyGPS {
    lat: Option<f64>,
    lon: Option<f64>,
    alt: Option<f64>,
    lat_dms: Option<(f64, f64, f64, String)>,
    lon_dms: Option<(f64, f64, f64, String)>,
    timestamp: Option<String>,
}

#[pymethods]
impl PyGPS {
    /// Latitude in decimal degrees (positive = North).
    #[getter]
    fn latitude(&self) -> Option<f64> {
        self.lat
    }

    /// Longitude in decimal degrees (positive = East).
    #[getter]
    fn longitude(&self) -> Option<f64> {
        self.lon
    }

    /// Altitude in meters (positive = above sea level).
    #[getter]
    fn altitude(&self) -> Option<f64> {
        self.alt
    }

    /// Latitude as DMS tuple: (degrees, minutes, seconds, ref).
    #[getter]
    fn latitude_dms(&self) -> Option<(f64, f64, f64, String)> {
        self.lat_dms.clone()
    }

    /// Longitude as DMS tuple: (degrees, minutes, seconds, ref).
    #[getter]
    fn longitude_dms(&self) -> Option<(f64, f64, f64, String)> {
        self.lon_dms.clone()
    }

    /// GPS timestamp as string.
    #[getter]
    fn timestamp(&self) -> Option<String> {
        self.timestamp.clone()
    }

    /// Check if GPS data is valid.
    fn __bool__(&self) -> bool {
        self.lat.is_some() && self.lon.is_some()
    }

    fn __repr__(&self) -> String {
        match (self.lat, self.lon) {
            (Some(lat), Some(lon)) => format!("GPS({:.6}, {:.6})", lat, lon),
            _ => "GPS(None)".to_string(),
        }
    }

    /// Get as dict for JSON serialization.
    fn as_dict(&self, py: Python<'_>) -> Py<PyAny> {
        use pyo3::types::PyDict;
        let dict = PyDict::new(py);
        if let Some(lat) = self.lat {
            dict.set_item("latitude", lat).unwrap();
        }
        if let Some(lon) = self.lon {
            dict.set_item("longitude", lon).unwrap();
        }
        if let Some(alt) = self.alt {
            dict.set_item("altitude", alt).unwrap();
        }
        if let Some(ref ts) = self.timestamp {
            dict.set_item("timestamp", ts).unwrap();
        }
        dict.into_any().unbind()
    }
}

impl PyGPS {
    /// Extract GPS from EXIF attrs.
    pub fn from_attrs(attrs: &Attrs) -> Option<Self> {
        let lat_vals = get_dms_values(attrs, "GPSLatitude");
        let lon_vals = get_dms_values(attrs, "GPSLongitude");

        let lat_ref = attrs.get_str("GPSLatitudeRef").unwrap_or("N");
        let lon_ref = attrs.get_str("GPSLongitudeRef").unwrap_or("E");

        let lat = lat_vals.as_ref().map(|dms| {
            let decimal = dms_to_decimal(dms.0, dms.1, dms.2);
            if lat_ref == "S" { -decimal } else { decimal }
        });

        let lon = lon_vals.as_ref().map(|dms| {
            let decimal = dms_to_decimal(dms.0, dms.1, dms.2);
            if lon_ref == "W" { -decimal } else { decimal }
        });

        // Altitude
        let alt = get_rational_f64(attrs, "GPSAltitude").map(|a| {
            let alt_ref = attrs.get_u32("GPSAltitudeRef").unwrap_or(0);
            if alt_ref == 1 { -a } else { a }
        });

        // Timestamp
        let timestamp = attrs.get_str("GPSDateStamp").map(|s| s.to_string());

        // Only return if we have at least lat/lon
        if lat.is_none() && lon.is_none() {
            return None;
        }

        Some(Self {
            lat,
            lon,
            alt,
            lat_dms: lat_vals.map(|(d, m, s)| (d, m, s, lat_ref.to_string())),
            lon_dms: lon_vals.map(|(d, m, s)| (d, m, s, lon_ref.to_string())),
            timestamp,
        })
    }
}

/// Convert DMS to decimal degrees.
fn dms_to_decimal(deg: f64, min: f64, sec: f64) -> f64 {
    deg + min / 60.0 + sec / 3600.0
}

/// Get DMS values from GPS rational array.
fn get_dms_values(attrs: &Attrs, key: &str) -> Option<(f64, f64, f64)> {
    // Try to get as list of rationals
    if let Some(AttrValue::List(list)) = attrs.get(key) {
        if list.len() >= 3 {
            let d = rational_to_f64(&list[0])?;
            let m = rational_to_f64(&list[1])?;
            let s = rational_to_f64(&list[2])?;
            return Some((d, m, s));
        }
    }

    // Try string format "deg min sec"
    if let Some(s) = attrs.get_str(key) {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() >= 3 {
            let d: f64 = parts[0].parse().ok()?;
            let m: f64 = parts[1].parse().ok()?;
            let s: f64 = parts[2].parse().ok()?;
            return Some((d, m, s));
        }
    }

    None
}

/// Convert AttrValue rational to f64.
fn rational_to_f64(v: &AttrValue) -> Option<f64> {
    match v {
        AttrValue::URational(n, d) if *d != 0 => Some(*n as f64 / *d as f64),
        AttrValue::Rational(n, d) if *d != 0 => Some(*n as f64 / *d as f64),
        AttrValue::Float(f) => Some(*f as f64),
        AttrValue::Double(d) => Some(*d),
        AttrValue::UInt(n) => Some(*n as f64),
        AttrValue::Int(n) => Some(*n as f64),
        _ => None,
    }
}

/// Get rational as f64 from attrs.
fn get_rational_f64(attrs: &Attrs, key: &str) -> Option<f64> {
    match attrs.get(key)? {
        AttrValue::URational(n, d) if *d != 0 => Some(*n as f64 / *d as f64),
        AttrValue::Rational(n, d) if *d != 0 => Some(*n as f64 / *d as f64),
        AttrValue::Float(f) => Some(*f as f64),
        AttrValue::Double(d) => Some(*d),
        _ => None,
    }
}

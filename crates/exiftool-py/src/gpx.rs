//! GPX parsing and geotagging support for Python.

use pyo3::prelude::*;
use std::fs;
use std::path::Path;

/// A GPS track point with timestamp.
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyTrackPoint {
    #[pyo3(get)]
    pub lat: f64,
    #[pyo3(get)]
    pub lon: f64,
    #[pyo3(get)]
    pub elevation: Option<f64>,
    #[pyo3(get)]
    pub timestamp: i64,
}

#[pymethods]
impl PyTrackPoint {
    fn __repr__(&self) -> String {
        format!("TrackPoint(lat={}, lon={}, ele={:?}, time={})",
            self.lat, self.lon, self.elevation, self.timestamp)
    }
}

/// Parsed GPX track for geotagging photos.
#[pyclass]
#[derive(Debug, Default)]
pub struct PyGpxTrack {
    points: Vec<PyTrackPoint>,
}

#[pymethods]
impl PyGpxTrack {
    /// Load GPX track from file.
    #[staticmethod]
    pub fn from_file(path: &str) -> PyResult<Self> {
        let p = Path::new(path);
        let content = fs::read_to_string(p)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Cannot read GPX: {}", e)))?;
        Self::parse(&content)
    }
    
    /// Parse GPX XML content.
    #[staticmethod]
    fn parse(xml: &str) -> PyResult<Self> {
        let mut track = PyGpxTrack::default();
        let mut i = 0;
        let bytes = xml.as_bytes();
        
        while i < bytes.len() {
            if let Some(start) = find_tag_start(&xml[i..], "trkpt")
                .or_else(|| find_tag_start(&xml[i..], "wpt"))
            {
                let tag_start = i + start;
                
                if let Some(end) = xml[tag_start..].find('>') {
                    let tag = &xml[tag_start..tag_start + end];
                    
                    let lat = extract_attr(tag, "lat");
                    let lon = extract_attr(tag, "lon");
                    
                    if let (Some(lat), Some(lon)) = (lat, lon) {
                        let close_tag = if xml[tag_start..].contains("trkpt") { "</trkpt>" } else { "</wpt>" };
                        if let Some(close_pos) = xml[tag_start..].find(close_tag) {
                            let element = &xml[tag_start..tag_start + close_pos + close_tag.len()];
                            
                            let elevation = extract_element(element, "ele")
                                .and_then(|s| s.parse().ok());
                            let time = extract_element(element, "time")
                                .and_then(|s| parse_iso_time(&s));
                            
                            if let Some(timestamp) = time {
                                track.points.push(PyTrackPoint { lat, lon, elevation, timestamp });
                            }
                            
                            i = tag_start + close_pos + close_tag.len();
                            continue;
                        }
                    }
                }
                i = tag_start + 1;
            } else {
                break;
            }
        }
        
        track.points.sort_by_key(|p| p.timestamp);
        
        if track.points.is_empty() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "No valid track points found in GPX"
            ));
        }
        
        Ok(track)
    }
    
    /// Number of track points.
    fn __len__(&self) -> usize {
        self.points.len()
    }
    
    /// Get all track points.
    #[getter]
    fn points(&self) -> Vec<PyTrackPoint> {
        self.points.clone()
    }
    
    /// Find GPS coordinates for a Unix timestamp (with interpolation).
    pub fn find_position(&self, timestamp: i64) -> Option<(f64, f64, Option<f64>)> {
        if self.points.is_empty() {
            return None;
        }
        
        // Exact match
        if let Some(pt) = self.points.iter().find(|p| p.timestamp == timestamp) {
            return Some((pt.lat, pt.lon, pt.elevation));
        }
        
        // Find surrounding points
        let mut before: Option<&PyTrackPoint> = None;
        let mut after: Option<&PyTrackPoint> = None;
        
        for pt in &self.points {
            if pt.timestamp <= timestamp {
                before = Some(pt);
            } else {
                after = Some(pt);
                break;
            }
        }
        
        match (before, after) {
            (Some(b), Some(a)) => {
                // Linear interpolation
                let total = (a.timestamp - b.timestamp) as f64;
                let partial = (timestamp - b.timestamp) as f64;
                let ratio = partial / total;
                
                let lat = b.lat + (a.lat - b.lat) * ratio;
                let lon = b.lon + (a.lon - b.lon) * ratio;
                let ele = match (b.elevation, a.elevation) {
                    (Some(be), Some(ae)) => Some(be + (ae - be) * ratio),
                    (Some(e), None) | (None, Some(e)) => Some(e),
                    _ => None,
                };
                
                Some((lat, lon, ele))
            }
            (Some(pt), None) | (None, Some(pt)) => {
                Some((pt.lat, pt.lon, pt.elevation))
            }
            _ => None,
        }
    }
    
    /// Track time range (start_timestamp, end_timestamp).
    #[getter]
    fn time_range(&self) -> Option<(i64, i64)> {
        if self.points.is_empty() {
            return None;
        }
        Some((self.points.first()?.timestamp, self.points.last()?.timestamp))
    }
    
    fn __repr__(&self) -> String {
        format!("GpxTrack(points={})", self.points.len())
    }
}

/// Find start position of an XML tag.
fn find_tag_start(s: &str, tag: &str) -> Option<usize> {
    let pattern = format!("<{}", tag);
    s.find(&pattern)
}

/// Extract attribute value from tag string.
fn extract_attr(tag: &str, attr: &str) -> Option<f64> {
    let pattern = format!("{}=\"", attr);
    let start = tag.find(&pattern)? + pattern.len();
    let end = tag[start..].find('"')? + start;
    tag[start..end].parse().ok()
}

/// Extract element content.
fn extract_element(xml: &str, tag: &str) -> Option<String> {
    let start_tag = format!("<{}>", tag);
    let end_tag = format!("</{}>", tag);
    
    let start = xml.find(&start_tag)? + start_tag.len();
    let end = xml[start..].find(&end_tag)? + start;
    Some(xml[start..end].trim().to_string())
}

/// Parse ISO 8601 timestamp to Unix timestamp.
fn parse_iso_time(s: &str) -> Option<i64> {
    let s = s.trim();
    if s.len() < 19 {
        return None;
    }
    
    let year: i32 = s[0..4].parse().ok()?;
    let month: u32 = s[5..7].parse().ok()?;
    let day: u32 = s[8..10].parse().ok()?;
    let hour: u32 = s[11..13].parse().ok()?;
    let minute: u32 = s[14..16].parse().ok()?;
    let second: u32 = s[17..19].parse().ok()?;
    
    fn days_from_year(y: i32) -> i64 {
        let y = y as i64;
        365 * y + y / 4 - y / 100 + y / 400
    }
    
    const DAYS_BEFORE_MONTH: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let leap_day = if is_leap && month > 2 { 1 } else { 0 };
    
    let days = days_from_year(year) - days_from_year(1970)
        + DAYS_BEFORE_MONTH[(month - 1) as usize] as i64
        + leap_day
        + (day - 1) as i64;
    
    let secs = days * 86400 + hour as i64 * 3600 + minute as i64 * 60 + second as i64;
    
    Some(secs)
}

/// Parse EXIF datetime "YYYY:MM:DD HH:MM:SS" to Unix timestamp.
pub fn parse_exif_datetime(s: &str) -> Option<i64> {
    let s = s.trim();
    if s.len() < 19 {
        return None;
    }
    
    let parts: Vec<&str> = s.split(' ').collect();
    if parts.len() < 2 {
        return None;
    }
    
    let date_parts: Vec<&str> = parts[0].split(':').collect();
    let time_parts: Vec<&str> = parts[1].split(':').collect();
    
    if date_parts.len() != 3 || time_parts.len() < 3 {
        return None;
    }
    
    let year: i32 = date_parts[0].parse().ok()?;
    let month: u32 = date_parts[1].parse().ok()?;
    let day: u32 = date_parts[2].parse().ok()?;
    let hour: u32 = time_parts[0].parse().ok()?;
    let minute: u32 = time_parts[1].parse().ok()?;
    let second: u32 = time_parts[2].split('.').next()?.parse().ok()?;
    
    fn days_from_year(y: i32) -> i64 {
        let y = y as i64;
        365 * y + y / 4 - y / 100 + y / 400
    }
    
    const DAYS_BEFORE_MONTH: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let leap_day = if is_leap && month > 2 { 1 } else { 0 };
    
    let days = days_from_year(year) - days_from_year(1970)
        + DAYS_BEFORE_MONTH[(month - 1) as usize] as i64
        + leap_day
        + (day - 1) as i64;
    
    let secs = days * 86400 + hour as i64 * 3600 + minute as i64 * 60 + second as i64;
    
    Some(secs)
}

/// Shift datetime string by offset seconds.
/// Input format: "YYYY:MM:DD HH:MM:SS"
pub fn shift_datetime(dt: &str, offset_secs: i64) -> Option<String> {
    let dt = dt.trim();
    if dt.len() < 19 {
        return None;
    }
    
    let parts: Vec<&str> = dt.split(' ').collect();
    if parts.len() < 2 {
        return None;
    }
    
    let date_parts: Vec<&str> = parts[0].split(':').collect();
    let time_str = parts[1].split('.').next().unwrap_or(parts[1]);
    let time_parts: Vec<&str> = time_str.split(':').collect();
    
    if date_parts.len() != 3 || time_parts.len() != 3 {
        return None;
    }
    
    let year: i32 = date_parts[0].parse().ok()?;
    let month: u32 = date_parts[1].parse().ok()?;
    let day: u32 = date_parts[2].parse().ok()?;
    let hour: u32 = time_parts[0].parse().ok()?;
    let minute: u32 = time_parts[1].parse().ok()?;
    let second: u32 = time_parts[2].parse().ok()?;
    
    fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 29 } else { 28 },
            _ => 30,
        }
    }
    
    let mut total_secs = (hour * 3600 + minute * 60 + second) as i64;
    let mut d = day as i64;
    let mut m = month as i64;
    let mut y = year as i64;
    
    total_secs += offset_secs;
    
    while total_secs >= 86400 {
        total_secs -= 86400;
        d += 1;
        let dim = days_in_month(y as i32, m as u32) as i64;
        if d > dim {
            d = 1;
            m += 1;
            if m > 12 {
                m = 1;
                y += 1;
            }
        }
    }
    while total_secs < 0 {
        total_secs += 86400;
        d -= 1;
        if d < 1 {
            m -= 1;
            if m < 1 {
                m = 12;
                y -= 1;
            }
            d = days_in_month(y as i32, m as u32) as i64;
        }
    }
    
    let new_hour = (total_secs / 3600) as u32;
    let new_minute = ((total_secs % 3600) / 60) as u32;
    let new_second = (total_secs % 60) as u32;
    
    Some(format!("{:04}:{:02}:{:02} {:02}:{:02}:{:02}", 
        y, m, d, new_hour, new_minute, new_second))
}

//! GPX parsing and geotagging support.

use std::fs;
use std::path::Path;

/// A GPS track point with timestamp.
#[derive(Debug, Clone)]
pub struct TrackPoint {
    pub lat: f64,
    pub lon: f64,
    pub ele: Option<f64>,
    pub time: i64, // Unix timestamp
}

/// Parsed GPX track.
#[derive(Debug, Default)]
pub struct GpxTrack {
    pub points: Vec<TrackPoint>,
}

impl GpxTrack {
    /// Parse GPX file and return track points.
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Cannot read GPX: {}", e))?;
        Self::parse(&content)
    }
    
    /// Parse GPX XML content.
    pub fn parse(xml: &str) -> Result<Self, String> {
        let mut track = GpxTrack::default();
        
        // Simple XML parsing for <trkpt> elements
        let mut i = 0;
        let bytes = xml.as_bytes();
        
        while i < bytes.len() {
            // Find <trkpt or <wpt
            if let Some(start) = find_tag_start(&xml[i..], "trkpt")
                .or_else(|| find_tag_start(&xml[i..], "wpt"))
            {
                let tag_start = i + start;
                
                // Extract lat/lon attributes
                if let Some(end) = xml[tag_start..].find('>') {
                    let tag = &xml[tag_start..tag_start + end];
                    
                    let lat = extract_attr(tag, "lat");
                    let lon = extract_attr(tag, "lon");
                    
                    if let (Some(lat), Some(lon)) = (lat, lon) {
                        // Find closing tag
                        let close_tag = if xml[tag_start..].contains("trkpt") { "</trkpt>" } else { "</wpt>" };
                        if let Some(close_pos) = xml[tag_start..].find(close_tag) {
                            let element = &xml[tag_start..tag_start + close_pos + close_tag.len()];
                            
                            // Extract elevation and time
                            let ele = extract_element(element, "ele")
                                .and_then(|s| s.parse().ok());
                            let time = extract_element(element, "time")
                                .and_then(|s| parse_iso_time(&s));
                            
                            if let Some(time) = time {
                                track.points.push(TrackPoint { lat, lon, ele, time });
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
        
        // Sort by time
        track.points.sort_by_key(|p| p.time);
        
        if track.points.is_empty() {
            return Err("No valid track points found in GPX".to_string());
        }
        
        Ok(track)
    }
    
    /// Find coordinates for a given timestamp (with interpolation).
    pub fn find_position(&self, timestamp: i64) -> Option<(f64, f64, Option<f64>)> {
        if self.points.is_empty() {
            return None;
        }
        
        // Exact match
        if let Some(pt) = self.points.iter().find(|p| p.time == timestamp) {
            return Some((pt.lat, pt.lon, pt.ele));
        }
        
        // Find surrounding points for interpolation
        let mut before: Option<&TrackPoint> = None;
        let mut after: Option<&TrackPoint> = None;
        
        for pt in &self.points {
            if pt.time <= timestamp {
                before = Some(pt);
            } else {
                after = Some(pt);
                break;
            }
        }
        
        match (before, after) {
            (Some(b), Some(a)) => {
                // Linear interpolation
                let total = (a.time - b.time) as f64;
                let partial = (timestamp - b.time) as f64;
                let ratio = partial / total;
                
                let lat = b.lat + (a.lat - b.lat) * ratio;
                let lon = b.lon + (a.lon - b.lon) * ratio;
                let ele = match (b.ele, a.ele) {
                    (Some(be), Some(ae)) => Some(be + (ae - be) * ratio),
                    (Some(e), None) | (None, Some(e)) => Some(e),
                    _ => None,
                };
                
                Some((lat, lon, ele))
            }
            (Some(pt), None) | (None, Some(pt)) => {
                // Use nearest point
                Some((pt.lat, pt.lon, pt.ele))
            }
            _ => None,
        }
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
    // Format: 2024-01-15T10:30:45Z or 2024-01-15T10:30:45+00:00
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
    
    // Simplified conversion to Unix timestamp
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

/// Parse EXIF datetime to Unix timestamp.
pub fn parse_exif_datetime(s: &str) -> Option<i64> {
    // Format: "YYYY:MM:DD HH:MM:SS"
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_gpx() {
        let gpx = r#"<?xml version="1.0"?>
        <gpx>
            <trk><trkseg>
                <trkpt lat="48.8584" lon="2.2945">
                    <ele>35.0</ele>
                    <time>2024-01-15T10:30:00Z</time>
                </trkpt>
                <trkpt lat="48.8600" lon="2.2960">
                    <ele>36.0</ele>
                    <time>2024-01-15T10:35:00Z</time>
                </trkpt>
            </trkseg></trk>
        </gpx>"#;
        
        let track = GpxTrack::parse(gpx).unwrap();
        assert_eq!(track.points.len(), 2);
        assert!((track.points[0].lat - 48.8584).abs() < 0.0001);
    }
    
    #[test]
    fn test_interpolation() {
        let gpx = r#"<?xml version="1.0"?>
        <gpx><trk><trkseg>
            <trkpt lat="0.0" lon="0.0"><time>2024-01-15T10:00:00Z</time></trkpt>
            <trkpt lat="10.0" lon="10.0"><time>2024-01-15T10:10:00Z</time></trkpt>
        </trkseg></trk></gpx>"#;
        
        let track = GpxTrack::parse(gpx).unwrap();
        
        // Midpoint (5 minutes = 300 seconds later)
        let t1 = track.points[0].time;
        let (lat, lon, _) = track.find_position(t1 + 300).unwrap();
        assert!((lat - 5.0).abs() < 0.1);
        assert!((lon - 5.0).abs() < 0.1);
    }
}

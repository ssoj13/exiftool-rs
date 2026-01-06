//! Composite/calculated tags.
//!
//! Derives additional metadata values from existing EXIF data.

use crate::Metadata;
use exiftool_attrs::AttrValue;

/// Add composite tags to metadata.
pub fn add_composite_tags(meta: &mut Metadata) {
    // ImageSize: WxH
    add_image_size(meta);
    
    // Megapixels
    add_megapixels(meta);
    
    // ShutterSpeed: human-readable format
    add_shutter_speed(meta);
    
    // Aperture: f/x.x format
    add_aperture(meta);
    
    // FocalLength35efl: 35mm equivalent
    add_focal_length_35efl(meta);
    
    // GPSPosition: decimal degrees
    add_gps_position(meta);
    
    // LensID: combined lens info
    add_lens_id(meta);
    
    // Duration: HH:MM:SS format for video
    add_duration(meta);
}

/// ImageSize: "WxH" format.
fn add_image_size(meta: &mut Metadata) {
    let width = meta.exif.get_u32("ImageWidth")
        .or_else(|| meta.exif.get_u32("ExifImageWidth"));
    let height = meta.exif.get_u32("ImageHeight")
        .or_else(|| meta.exif.get_u32("ExifImageHeight"));
    
    if let (Some(w), Some(h)) = (width, height) {
        if w > 0 && h > 0 {
            meta.exif.set("ImageSize", AttrValue::Str(format!("{}x{}", w, h)));
        }
    }
}

/// Megapixels: width * height / 1_000_000.
fn add_megapixels(meta: &mut Metadata) {
    let width = meta.exif.get_u32("ImageWidth")
        .or_else(|| meta.exif.get_u32("ExifImageWidth"));
    let height = meta.exif.get_u32("ImageHeight")
        .or_else(|| meta.exif.get_u32("ExifImageHeight"));
    
    if let (Some(w), Some(h)) = (width, height) {
        if w > 0 && h > 0 {
            let mp = (w as f64 * h as f64) / 1_000_000.0;
            meta.exif.set("Megapixels", AttrValue::Double(mp));
        }
    }
}

/// ShutterSpeed: convert ExposureTime to human-readable.
fn add_shutter_speed(meta: &mut Metadata) {
    // Try to get ExposureTime as rational or float
    if let Some(val) = meta.exif.get("ExposureTime") {
        let seconds = match val {
            AttrValue::URational(n, d) if *d > 0 => Some(*n as f64 / *d as f64),
            AttrValue::Rational(n, d) if *d != 0 => Some(*n as f64 / *d as f64),
            AttrValue::Float(f) => Some(*f as f64),
            AttrValue::Double(d) => Some(*d),
            AttrValue::Str(s) => {
                // Try parsing "n/d" format
                if let Some((n, d)) = s.split_once('/') {
                    if let (Ok(num), Ok(den)) = (n.trim().parse::<f64>(), d.trim().parse::<f64>()) {
                        if den > 0.0 { Some(num / den) } else { None }
                    } else { None }
                } else {
                    s.trim().parse::<f64>().ok()
                }
            }
            _ => None,
        };
        
        if let Some(secs) = seconds {
            let display = if secs >= 1.0 {
                format!("{:.1} s", secs)
            } else if secs > 0.0 {
                let recip = (1.0 / secs).round() as u32;
                format!("1/{} s", recip)
            } else {
                return;
            };
            meta.exif.set("ShutterSpeed", AttrValue::Str(display));
        }
    }
}

/// Aperture: convert FNumber to "f/x.x" format.
fn add_aperture(meta: &mut Metadata) {
    if let Some(val) = meta.exif.get("FNumber") {
        let fnum = match val {
            AttrValue::URational(n, d) if *d > 0 => Some(*n as f64 / *d as f64),
            AttrValue::Rational(n, d) if *d != 0 => Some(*n as f64 / *d as f64),
            AttrValue::Float(f) => Some(*f as f64),
            AttrValue::Double(d) => Some(*d),
            AttrValue::UInt(n) => Some(*n as f64),
            AttrValue::Int(n) => Some(*n as f64),
            AttrValue::Str(s) => {
                if let Some((n, d)) = s.split_once('/') {
                    if let (Ok(num), Ok(den)) = (n.trim().parse::<f64>(), d.trim().parse::<f64>()) {
                        if den > 0.0 { Some(num / den) } else { None }
                    } else { None }
                } else {
                    s.trim().parse::<f64>().ok()
                }
            }
            _ => None,
        };
        
        if let Some(f) = fnum {
            if f > 0.0 {
                let display = if f == f.round() {
                    format!("f/{:.0}", f)
                } else {
                    format!("f/{:.1}", f)
                };
                meta.exif.set("Aperture", AttrValue::Str(display));
            }
        }
    }
}

/// FocalLength35efl: 35mm equivalent focal length.
fn add_focal_length_35efl(meta: &mut Metadata) {
    // Get focal length
    let focal = if let Some(val) = meta.exif.get("FocalLength") {
        match val {
            AttrValue::URational(n, d) if *d > 0 => Some(*n as f64 / *d as f64),
            AttrValue::Rational(n, d) if *d != 0 => Some(*n as f64 / *d as f64),
            AttrValue::Float(f) => Some(*f as f64),
            AttrValue::Double(d) => Some(*d),
            AttrValue::UInt(n) => Some(*n as f64),
            AttrValue::Str(s) => {
                if let Some((n, d)) = s.split_once('/') {
                    if let (Ok(num), Ok(den)) = (n.trim().parse::<f64>(), d.trim().parse::<f64>()) {
                        if den > 0.0 { Some(num / den) } else { None }
                    } else { None }
                } else {
                    s.trim().replace("mm", "").trim().parse::<f64>().ok()
                }
            }
            _ => None,
        }
    } else {
        None
    };
    
    // Get crop factor or calculate from sensor size
    let crop_factor = get_crop_factor(meta);
    
    if let (Some(fl), Some(cf)) = (focal, crop_factor) {
        if fl > 0.0 && cf > 0.0 {
            let efl = fl * cf;
            meta.exif.set("FocalLength35efl", AttrValue::Str(format!("{:.0} mm", efl)));
        }
    }
}

/// Get crop factor from FocalLengthIn35mmFormat or estimate from sensor.
fn get_crop_factor(meta: &Metadata) -> Option<f64> {
    // Check if we have 35mm equivalent already
    if let Some(fl35) = meta.exif.get("FocalLengthIn35mmFormat") {
        let efl = match fl35 {
            AttrValue::UInt(n) => Some(*n as f64),
            AttrValue::Int(n) => Some(*n as f64),
            AttrValue::Float(f) => Some(*f as f64),
            AttrValue::Double(d) => Some(*d),
            AttrValue::Str(s) => s.trim().replace("mm", "").trim().parse::<f64>().ok(),
            _ => None,
        };
        
        // Get actual focal length to calculate crop factor
        if let Some(efl) = efl {
            if let Some(fl) = meta.exif.get("FocalLength") {
                let actual = match fl {
                    AttrValue::URational(n, d) if *d > 0 => *n as f64 / *d as f64,
                    AttrValue::Float(f) => *f as f64,
                    AttrValue::Double(d) => *d,
                    _ => return Some(1.0), // Assume full frame
                };
                if actual > 0.0 {
                    return Some(efl / actual);
                }
            }
        }
    }
    
    // Estimate from sensor size or camera model (simplified)
    // Full frame = 36x24mm, diagonal = 43.3mm
    // APS-C Canon = 22.3x14.9mm, crop ~1.6
    // APS-C Nikon/Sony = 23.5x15.6mm, crop ~1.5
    // Micro 4/3 = 17.3x13mm, crop ~2.0
    
    // Default to 1.0 (full frame) if we can't determine
    Some(1.0)
}

/// GPSPosition: decimal degrees from GPS coordinates.
fn add_gps_position(meta: &mut Metadata) {
    let lat = parse_gps_coord(meta.exif.get("GPSLatitude"), meta.exif.get_str("GPSLatitudeRef"));
    let lon = parse_gps_coord(meta.exif.get("GPSLongitude"), meta.exif.get_str("GPSLongitudeRef"));
    
    if let (Some(lat), Some(lon)) = (lat, lon) {
        meta.exif.set("GPSPosition", AttrValue::Str(format!("{:.6}, {:.6}", lat, lon)));
    }
}

/// Parse GPS coordinate from DMS or decimal format.
fn parse_gps_coord(coord: Option<&AttrValue>, ref_val: Option<&str>) -> Option<f64> {
    let degrees = match coord? {
        AttrValue::Str(s) => {
            // Try to parse "deg min sec" or decimal
            parse_dms_string(s)?
        }
        AttrValue::Double(d) => *d,
        AttrValue::Float(f) => *f as f64,
        _ => return None,
    };
    
    // Apply hemisphere (S/W are negative)
    let sign = match ref_val {
        Some("S") | Some("W") => -1.0,
        _ => 1.0,
    };
    
    Some(degrees * sign)
}

/// Parse DMS string like "51 deg 30' 26.13\"" to decimal degrees.
fn parse_dms_string(s: &str) -> Option<f64> {
    // Remove common separators and try to extract numbers
    let s = s.replace("deg", " ").replace(['°', '\'', '"', ','], " ");
    
    let parts: Vec<f64> = s.split_whitespace()
        .filter_map(|p| p.parse::<f64>().ok())
        .collect();
    
    match parts.len() {
        1 => Some(parts[0]), // Already decimal
        2 => Some(parts[0] + parts[1] / 60.0), // deg min
        3 => Some(parts[0] + parts[1] / 60.0 + parts[2] / 3600.0), // deg min sec
        _ => None,
    }
}

/// LensID: combined lens identification.
fn add_lens_id(meta: &mut Metadata) {
    // Check if LensModel or LensID already exists
    if meta.exif.get("LensID").is_some() || meta.exif.get("LensModel").is_some() {
        return;
    }
    
    // Try to build from available lens info
    let mut parts = Vec::new();
    
    if let Some(make) = meta.exif.get_str("LensMake") {
        parts.push(make.to_string());
    }
    
    if let Some(model) = meta.exif.get_str("Lens") {
        parts.push(model.to_string());
    }
    
    if !parts.is_empty() {
        meta.exif.set("LensID", AttrValue::Str(parts.join(" ")));
    }
}

/// Duration: format video duration as HH:MM:SS.
fn add_duration(meta: &mut Metadata) {
    // Check if Duration already exists in readable format
    if meta.exif.get_str("Duration").map(|s| s.contains(':')).unwrap_or(false) {
        return;
    }
    
    // Get duration in seconds
    let seconds = meta.exif.get("Duration")
        .or_else(|| meta.exif.get("MediaDuration"))
        .and_then(|v| match v {
            AttrValue::Float(f) => Some(*f as f64),
            AttrValue::Double(d) => Some(*d),
            AttrValue::UInt(n) => Some(*n as f64),
            AttrValue::Int(n) => Some(*n as f64),
            AttrValue::Str(s) => s.parse::<f64>().ok(),
            _ => None,
        });
    
    if let Some(secs) = seconds {
        if secs > 0.0 {
            let total_secs = secs as u64;
            let hours = total_secs / 3600;
            let mins = (total_secs % 3600) / 60;
            let secs = total_secs % 60;
            
            let display = if hours > 0 {
                format!("{}:{:02}:{:02}", hours, mins, secs)
            } else {
                format!("{}:{:02}", mins, secs)
            };
            meta.exif.set("DurationFormatted", AttrValue::Str(display));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_image_size() {
        let mut meta = Metadata::new("JPEG");
        meta.exif.set("ImageWidth", AttrValue::UInt(4000));
        meta.exif.set("ImageHeight", AttrValue::UInt(3000));
        
        add_composite_tags(&mut meta);
        
        assert_eq!(meta.exif.get_str("ImageSize"), Some("4000x3000"));
        assert!((meta.exif.get_f64("Megapixels").unwrap() - 12.0).abs() < 0.01);
    }
    
    #[test]
    fn test_shutter_speed() {
        let mut meta = Metadata::new("JPEG");
        meta.exif.set("ExposureTime", AttrValue::URational(1, 250));
        
        add_composite_tags(&mut meta);
        
        assert_eq!(meta.exif.get_str("ShutterSpeed"), Some("1/250 s"));
    }
    
    #[test]
    fn test_aperture() {
        let mut meta = Metadata::new("JPEG");
        meta.exif.set("FNumber", AttrValue::URational(28, 10));
        
        add_composite_tags(&mut meta);
        
        assert_eq!(meta.exif.get_str("Aperture"), Some("f/2.8"));
    }
    
    #[test]
    fn test_gps_position() {
        let mut meta = Metadata::new("JPEG");
        meta.exif.set("GPSLatitude", AttrValue::Double(51.5074));
        meta.exif.set("GPSLatitudeRef", AttrValue::Str("N".into()));
        meta.exif.set("GPSLongitude", AttrValue::Double(0.1278));
        meta.exif.set("GPSLongitudeRef", AttrValue::Str("W".into()));
        
        add_composite_tags(&mut meta);
        
        let pos = meta.exif.get_str("GPSPosition").unwrap();
        assert!(pos.contains("51.507"));
        assert!(pos.contains("-0.127"));
    }
    
    #[test]
    fn test_duration() {
        let mut meta = Metadata::new("MP4");
        meta.exif.set("Duration", AttrValue::Double(3725.0)); // 1:02:05
        
        add_composite_tags(&mut meta);
        
        assert_eq!(meta.exif.get_str("DurationFormatted"), Some("1:02:05"));
    }
}

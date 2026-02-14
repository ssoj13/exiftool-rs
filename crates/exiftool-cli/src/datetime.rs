//! DateTime parsing and shifting for metadata.
//!
//! # Why
//!
//! ExifTool supports `--shift +/-HH:MM` to adjust all DateTime tags. This module
//! implements parsing of "YYYY:MM:DD HH:MM:SS" format and arithmetic.
//!
//! # Where used
//!
//! main.rs write path — apply_time_shift() when --shift is given.

use exiftool_attrs::AttrValue;
use exiftool_formats::Metadata;

/// Shift a datetime string by the given offset in seconds.
/// Handles format: "YYYY:MM:DD HH:MM:SS"
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
            2 => {
                if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                    29
                } else {
                    28
                }
            }
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

    Some(format!(
        "{:04}:{:02}:{:02} {:02}:{:02}:{:02}",
        y, m, d, new_hour, new_minute, new_second
    ))
}

/// Apply time shift to all DateTime tags in metadata.
pub fn apply_time_shift(metadata: &mut Metadata, offset_secs: i64) {
    let datetime_tags = [
        "DateTime",
        "DateTimeOriginal",
        "CreateDate",
        "ModifyDate",
        "DateTimeDigitized",
        "GPSDateTime",
        "GPSDateStamp",
    ];

    for tag in &datetime_tags {
        if let Some(val) = metadata.exif.get(*tag) {
            if let Some(s) = val.as_str() {
                if let Some(shifted) = shift_datetime(s, offset_secs) {
                    metadata.exif.set(*tag, AttrValue::Str(shifted));
                }
            }
        }
    }
}

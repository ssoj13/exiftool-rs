//! Validate metadata command (--validate).

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use exiftool_formats::FormatRegistry;

use crate::paths;

/// Validate metadata in files.
pub fn validate_metadata(args: &crate::args::Args, registry: &FormatRegistry) -> Result<()> {
    let files = paths::expand_paths(
        &args.files,
        args.recursive,
        &args.extensions,
        &args.exclude,
        args.newer,
        args.older,
        args.minsize,
        args.maxsize,
    );

    if files.is_empty() {
        anyhow::bail!("No files to validate");
    }

    let mut total_issues = 0;
    let mut files_with_issues = 0;

    for path in &files {
        match validate_metadata_single(path, registry) {
            Ok(issues) => {
                if !issues.is_empty() {
                    println!("-- {} --", path.display());
                    for (tag, severity, msg) in &issues {
                        println!("  [{}] {}: {}", severity, tag, msg);
                    }
                    total_issues += issues.len();
                    files_with_issues += 1;
                }
            }
            Err(e) => {
                eprintln!("Error {}: {}", path.display(), e);
                files_with_issues += 1;
            }
        }
    }

    if files_with_issues > 0 {
        eprintln!("\nFound {} issues in {} files", total_issues, files_with_issues);
        std::process::exit(1);
    } else {
        eprintln!("All {} files valid", files.len());
    }
    Ok(())
}

/// Validate metadata for a single file.
/// Returns list of (tag, severity, message) tuples.
fn validate_metadata_single(
    path: &Path,
    registry: &FormatRegistry,
) -> Result<Vec<(String, String, String)>> {
    let file = File::open(path)
        .with_context(|| format!("Cannot open: {}", path.display()))?;
    let mut reader = BufReader::new(file);
    let metadata = registry
        .parse(&mut reader)
        .with_context(|| format!("Cannot parse: {}", path.display()))?;

    let mut issues = Vec::new();

    if let Some(lat) = metadata.exif.get("GPSLatitude").and_then(|v| v.as_f64()) {
        if !(-90.0..=90.0).contains(&lat) {
            issues.push((
                "GPSLatitude".into(),
                "error".into(),
                format!("Invalid latitude {}: must be -90 to 90", lat),
            ));
        }
    }

    if let Some(lon) = metadata.exif.get("GPSLongitude").and_then(|v| v.as_f64()) {
        if !(-180.0..=180.0).contains(&lon) {
            issues.push((
                "GPSLongitude".into(),
                "error".into(),
                format!("Invalid longitude {}: must be -180 to 180", lon),
            ));
        }
    }

    if let Some(orient) = metadata.exif.get("Orientation").and_then(|v| v.as_u32()) {
        if !(1..=8).contains(&orient) {
            issues.push((
                "Orientation".into(),
                "error".into(),
                format!("Invalid orientation {}: must be 1-8", orient),
            ));
        }
    }

    if let Some(iso) = metadata.exif.get("ISO").and_then(|v| v.as_u32()) {
        if iso == 0 || iso > 10_000_000 {
            issues.push((
                "ISO".into(),
                "warning".into(),
                format!("Suspicious ISO value {}", iso),
            ));
        }
    }

    if let Some(width) = metadata.exif.get("ImageWidth").and_then(|v| v.as_u32()) {
        if width == 0 {
            issues.push(("ImageWidth".into(), "error".into(), "Width is 0".into()));
        }
    }
    if let Some(height) = metadata.exif.get("ImageHeight").and_then(|v| v.as_u32()) {
        if height == 0 {
            issues.push(("ImageHeight".into(), "error".into(), "Height is 0".into()));
        }
    }

    for tag in &[
        "DateTime",
        "DateTimeOriginal",
        "DateTimeDigitized",
        "CreateDate",
        "ModifyDate",
    ] {
        if let Some(dt) = metadata.exif.get(*tag).and_then(|v| v.as_str()) {
            if !is_valid_datetime(dt) {
                issues.push((
                    (*tag).to_string(),
                    "warning".into(),
                    format!("Invalid datetime format: {}", dt),
                ));
            }
        }
    }

    if let Some(exp) = metadata.exif.get("ExposureTime").and_then(|v| v.as_f64()) {
        if exp <= 0.0 {
            issues.push((
                "ExposureTime".into(),
                "error".into(),
                format!("Invalid exposure time: {}", exp),
            ));
        }
    }

    if let Some(f) = metadata.exif.get("FNumber").and_then(|v| v.as_f64()) {
        if f <= 0.0 {
            issues.push((
                "FNumber".into(),
                "error".into(),
                format!("Invalid FNumber: {}", f),
            ));
        }
    }

    Ok(issues)
}

fn is_valid_datetime(s: &str) -> bool {
    let s = s.trim();
    if s.len() < 10 {
        return false;
    }
    let parts: Vec<&str> = s.split(|c| c == ' ' || c == 'T').collect();
    if parts.is_empty() {
        return false;
    }
    let date = parts[0];
    let date_parts: Vec<&str> = date.split(|c| c == ':' || c == '-').collect();
    if date_parts.len() != 3 {
        return false;
    }
    if date_parts[0].parse::<u16>().is_err() {
        return false;
    }
    if let Ok(m) = date_parts[1].parse::<u8>() {
        if !(1..=12).contains(&m) {
            return false;
        }
    } else {
        return false;
    }
    if let Ok(d) = date_parts[2].parse::<u8>() {
        if !(1..=31).contains(&d) {
            return false;
        }
    } else {
        return false;
    }
    true
}

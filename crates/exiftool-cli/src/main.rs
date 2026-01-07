//! exiftool-rs CLI - fast metadata reader/writer for images
//!
//! Supports: JPEG, PNG, TIFF, DNG, CR2, CR3, NEF, ARW, ORF, RW2, PEF, RAF, WebP, HEIC, AVIF, EXR, HDR

mod geotag;
mod html_output;
mod xml_output;

use anyhow::{Context, Result};
use exiftool_attrs::AttrValue;
use exiftool_formats::{
    add_composite_tags, build_exif_bytes, FormatRegistry, JpegWriter, Metadata, 
    PngWriter, TiffWriter, HdrWriter, ExrWriter,
};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::env;
use walkdir::WalkDir;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Simple glob matching for tag names (* = any chars, ? = one char)
fn glob_match(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    glob_match_impl(&p, &t)
}

fn glob_match_impl(p: &[char], t: &[char]) -> bool {
    match (p.first(), t.first()) {
        (None, None) => true,
        (Some('*'), _) => {
            // Try matching * with 0 chars or 1+ chars
            glob_match_impl(&p[1..], t) || (!t.is_empty() && glob_match_impl(p, &t[1..]))
        }
        (Some('?'), Some(_)) => glob_match_impl(&p[1..], &t[1..]),
        (Some(pc), Some(tc)) if pc.eq_ignore_ascii_case(tc) => {
            glob_match_impl(&p[1..], &t[1..])
        }
        _ => false,
    }
}

/// Check if tag matches any of the filter patterns
fn tag_matches(tag: &str, filters: &[String]) -> bool {
    if filters.is_empty() {
        return true;
    }
    filters.iter().any(|f| {
        if f.contains('*') || f.contains('?') {
            glob_match(f, tag)
        } else {
            f.eq_ignore_ascii_case(tag)
        }
    })
}

/// Check if filter is a simple tag name (no wildcards)
fn is_simple_filter(filters: &[String]) -> bool {
    filters.len() == 1 && !filters[0].contains('*') && !filters[0].contains('?')
}

/// Check if any filter has wildcards
fn has_wildcards(filters: &[String]) -> bool {
    filters.iter().any(|f| f.contains('*') || f.contains('?'))
}

/// Expand wildcard patterns to actual tag names from metadata
fn expand_filters(filters: &[String], metadata: &exiftool_formats::Metadata) -> Vec<String> {
    if filters.is_empty() {
        return vec![];
    }
    if !has_wildcards(filters) {
        return filters.to_vec();
    }
    
    let mut result = Vec::new();
    for (tag, _) in metadata.exif.iter() {
        if tag_matches(tag, filters) && !result.contains(tag) {
            result.push(tag.clone());
        }
    }
    result.sort();
    result
}

const HELP: &str = r#"
exif - fast image metadata reader/writer

USAGE:
    exif [OPTIONS] <FILES>...

READ:
    exif photo.jpg                    # show all metadata
    exif -g Model photo.jpg           # get single tag value
    exif -g Model -g Make *.jpg       # get multiple tags
    exif -g Date* photo.jpg           # wildcard: all Date* tags
    exif -g *Original photo.jpg       # wildcard: *Original tags
    exif -f json *.jpg                # JSON output for batch
    exif -f csv photos/*.png          # CSV for spreadsheet
    exif -X photo.jpg                 # XML output (ExifTool compatible)
    exif image.{heic,cr3,nef,arw,orf,rw2,pef,raf,webp}  # RAW formats
    exif -r photos/                   # recursive directory scan
    exif -r -e jpg,png photos/        # recursive with extension filter

OUTPUT:
    exif -f json photo.jpg -o meta.json   # save metadata to file
    exif -f csv *.jpg -o report.csv       # batch export to CSV

WRITE:
    exif -t Artist="John Doe" a.jpg             # set single tag
    exif -t Make=Canon -t Model=EOS a.jpg       # set multiple tags
    exif -w out.jpg -t Copyright="(C) Me" a.jpg # write to new file
    exif -p -t Software=exif a.jpg              # modify in-place (!)
    exif --shift "+2:00" -p photo.jpg           # shift times +2 hours
    exif --shift "-30" -p photo.jpg             # shift times -30 minutes

THUMBNAIL/PREVIEW:
    exif -T photo.jpg                           # extract thumbnail to photo_thumb.jpg
    exif -T -o thumb.jpg photo.jpg              # extract to specific file
    exif -P photo.cr2                           # extract RAW preview to photo_preview.jpg
    exif -P -o preview.jpg photo.raf            # extract preview to specific file

OPTIONS:
    -g, --get <PATTERN>  Get tag(s) matching pattern (* and ? wildcards)
    -f, --format <FMT>   Output: text (default), json, csv, xml, html
    -X, --xml            XML output (shortcut for -f xml)
    -o, --output <FILE>  Save metadata/thumbnail to file
    -t, --tag <T=V>      Set tag (repeatable): -t Tag=Value
    --shift <OFFSET>     Shift all DateTime tags (+/-HH:MM or +/-MM minutes)
    --geotag <GPX>       Add GPS coordinates from GPX track file
    --icc <FILE>         Embed ICC color profile from file
    -w, --write <FILE>   Output image file (for write mode)
    -p, --inplace        Modify original file in-place
    -T, --thumbnail      Extract embedded thumbnail
    -P, --preview        Extract embedded preview (larger, from RAW files)
    -r, --recursive      Recursively scan directories
    -e, --ext <EXTS>     Filter by extensions (comma-separated): jpg,png,cr2
    -x, --exclude <PAT>  Exclude files/dirs matching pattern (glob, repeatable)
    --newer <DATE>       Only files modified after DATE (YYYY-MM-DD or YYYY-MM-DD HH:MM:SS)
    --older <DATE>       Only files modified before DATE
    --minsize <SIZE>     Only files larger than SIZE (e.g., 100, 1K, 1M, 1G)
    --maxsize <SIZE>     Only files smaller than SIZE
    -c, --composite      Add composite/calculated tags (ImageSize, Megapixels, etc.)
    --charset <ENC>      Character encoding for strings (utf8, latin1, default: utf8)
    -a, --all            Include binary/large tags
    -h, --help, /?       Show this help
    -v, --version        Show version

FORMATS (read):  JPEG PNG TIFF DNG CR2 CR3 NEF ARW ORF RW2 PEF RAF WebP HEIC AVIF EXR HDR
FORMATS (write): JPEG PNG TIFF EXR HDR

COMMON TAGS:
    Make, Model, Software, Artist, Copyright, DateTime,
    DateTimeOriginal, CreateDate, ISO, ExposureTime, FNumber,
    FocalLength, Orientation, ImageDescription, GPSLatitude...

EXAMPLES:
    # Extract camera info from all JPEGs
    exif -f json *.jpg | jq '.[].Model'

    # Export metadata to file
    exif -f json *.jpg -o metadata.json

    # Batch set copyright
    for f in *.jpg; do exif -p -t Copyright="2024 Me" "$f"; done

    # Read RAW files
    exif photo.cr3 photo.nef photo.arw photo.orf photo.rw2 photo.pef
"#;

fn main() {
    if let Err(e) = run() {
        // Print error chain without backtrace
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    // No args or help requested
    if args.len() < 2 || matches!(args.get(1).map(|s| s.as_str()), Some("-h" | "--help" | "/?" | "-?" | "help")) {
        print!("{}", HELP.trim_start());
        return Ok(());
    }
    
    // Version
    if matches!(args.get(1).map(|s| s.as_str()), Some("-v" | "--version" | "-V")) {
        println!("exif {}", VERSION);
        return Ok(());
    }
    
    // Parse args manually for flexibility
    let parsed = parse_args(&args[1..])?;
    let registry = FormatRegistry::new();

    // Write mode (modify image tags)
    if !parsed.tags.is_empty() {
        return write_image(&parsed, &registry);
    }

    // Thumbnail extraction mode
    if parsed.thumbnail {
        return extract_thumbnails(&parsed, &registry);
    }

    // Preview extraction mode (RAW files)
    if parsed.preview {
        return extract_previews(&parsed, &registry);
    }

    // Expand paths (handle directories if recursive)
    let files = expand_paths(
        &parsed.files, 
        parsed.recursive, 
        &parsed.extensions, 
        &parsed.exclude,
        parsed.newer,
        parsed.older,
        parsed.minsize,
        parsed.maxsize,
    );
    
    // Read mode
    if files.is_empty() {
        if parsed.files.is_empty() {
            anyhow::bail!("No input files specified.\n\nUsage: exif [OPTIONS] <FILES>...\n       exif --help for more options");
        } else {
            anyhow::bail!("No matching files found.");
        }
    }

    // Show count in recursive mode
    if parsed.recursive && files.len() > 1 {
        eprintln!("Processing {} files...", files.len());
    }

    // CSV mode with multiple files: collect all metadata first for unified headers
    if parsed.format == "csv" && files.len() > 1 {
        return output_csv_unified(&files, &registry, &parsed);
    }

    // Collect output for potential file write
    let mut output_buf = String::new();
    let write_to_file = parsed.output.is_some();

    for path in &files {
        let file = File::open(path)
            .with_context(|| format!("Cannot open: {}", path.display()))?;
        let mut reader = BufReader::new(file);

        match registry.parse(&mut reader) {
            Ok(mut metadata) => {
                // Add composite tags if requested
                if parsed.composite {
                    add_composite_tags(&mut metadata);
                }
                
                if write_to_file {
                    format_metadata(path, &metadata, &parsed, &mut output_buf);
                } else {
                    print_metadata(path, &metadata, &parsed);
                }
            }
            Err(e) => eprintln!("Error {}: {}", path.display(), e),
        }
    }

    // Write to file if -o specified
    if let Some(ref output_path) = parsed.output {
        std::fs::write(output_path, &output_buf)
            .with_context(|| format!("Cannot write: {}", output_path.display()))?;
        eprintln!("Wrote: {}", output_path.display());
    }

    Ok(())
}

#[derive(Debug, Default)]
struct Args {
    files: Vec<PathBuf>,
    format: String,
    get_tags: Vec<String>,            // -g Tag (filter output)
    tags: Vec<(String, String)>,      // -t Tag=Value
    shift: Option<i64>,               // --shift time offset in seconds
    geotag: Option<PathBuf>,          // --geotag GPX file
    icc_profile: Option<PathBuf>,     // --icc profile file
    output: Option<PathBuf>,          // -o metadata output
    write_file: Option<PathBuf>,      // -w image output
    inplace: bool,                    // -p modify in-place
    thumbnail: bool,                  // -T extract thumbnail
    preview: bool,                    // -P extract preview (RAW)
    recursive: bool,                  // -r recursive directory scan
    extensions: Vec<String>,          // -e extension filter
    exclude: Vec<String>,             // -x exclude patterns
    newer: Option<std::time::SystemTime>,  // --newer date filter
    older: Option<std::time::SystemTime>,  // --older date filter
    minsize: Option<u64>,             // --minsize filter
    maxsize: Option<u64>,             // --maxsize filter
    composite: bool,                  // -c add composite tags
    charset: String,                  // --charset encoding
    all: bool,
}

/// Parse date string to SystemTime.
/// Supports: YYYY-MM-DD, YYYY-MM-DD HH:MM:SS, YYYY-MM-DDTHH:MM:SS
fn parse_date(s: &str) -> Option<std::time::SystemTime> {
    use std::time::{Duration, UNIX_EPOCH};
    
    // Try different formats
    let s = s.trim();
    
    // Parse components
    let (date_part, time_part) = if s.contains('T') {
        let parts: Vec<&str> = s.split('T').collect();
        (parts.get(0).copied()?, parts.get(1).map(|s| *s))
    } else if s.contains(' ') {
        let parts: Vec<&str> = s.splitn(2, ' ').collect();
        (parts.get(0).copied()?, parts.get(1).map(|s| *s))
    } else {
        (s, None)
    };
    
    // Parse date
    let date_parts: Vec<&str> = date_part.split('-').collect();
    if date_parts.len() != 3 {
        return None;
    }
    
    let year: i32 = date_parts[0].parse().ok()?;
    let month: u32 = date_parts[1].parse().ok()?;
    let day: u32 = date_parts[2].parse().ok()?;
    
    // Parse time (default to 00:00:00)
    let (hour, minute, second) = if let Some(t) = time_part {
        let t = t.split('.').next().unwrap_or(t); // Strip subseconds
        let time_parts: Vec<&str> = t.split(':').collect();
        (
            time_parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0u32),
            time_parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0u32),
            time_parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0u32),
        )
    } else {
        (0, 0, 0)
    };
    
    // Validate ranges
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    
    // Days from year 0 to year (accounting for leap years)
    fn days_from_year(y: i32) -> i64 {
        let y = y as i64;
        365 * y + y / 4 - y / 100 + y / 400
    }
    
    // Days in months (non-leap year)
    const DAYS_BEFORE_MONTH: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let leap_day = if is_leap && month > 2 { 1 } else { 0 };
    
    let days = days_from_year(year) - days_from_year(1970)
        + DAYS_BEFORE_MONTH[(month - 1) as usize] as i64
        + leap_day
        + (day - 1) as i64;
    
    let secs = days * 86400 + hour as i64 * 3600 + minute as i64 * 60 + second as i64;
    
    if secs >= 0 {
        Some(UNIX_EPOCH + Duration::from_secs(secs as u64))
    } else {
        None
    }
}

/// Shift a datetime string by the given offset in seconds.
/// Handles format: "YYYY:MM:DD HH:MM:SS"
fn shift_datetime(dt: &str, offset_secs: i64) -> Option<String> {
    let dt = dt.trim();
    if dt.len() < 19 {
        return None;
    }
    
    // Parse "YYYY:MM:DD HH:MM:SS"
    let parts: Vec<&str> = dt.split(' ').collect();
    if parts.len() < 2 {
        return None;
    }
    
    let date_parts: Vec<&str> = parts[0].split(':').collect();
    let time_str = parts[1].split('.').next().unwrap_or(parts[1]); // Strip subseconds
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
    
    // Convert to total seconds since epoch (simplified)
    fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 29 } else { 28 },
            _ => 30,
        }
    }
    
    // Calculate total seconds
    let mut total_secs = (hour * 3600 + minute * 60 + second) as i64;
    let mut d = day as i64;
    let mut m = month as i64;
    let mut y = year as i64;
    
    // Add offset
    total_secs += offset_secs;
    
    // Handle day overflow/underflow
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

/// Apply time shift to all DateTime tags in metadata.
fn apply_time_shift(metadata: &mut Metadata, offset_secs: i64) {
    let datetime_tags = [
        "DateTime", "DateTimeOriginal", "CreateDate", "ModifyDate",
        "DateTimeDigitized", "GPSDateTime", "GPSDateStamp",
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
    
    // Check if it's HH:MM format
    if rest.contains(':') {
        let parts: Vec<&str> = rest.split(':').collect();
        if parts.len() != 2 {
            return None;
        }
        let hours: i64 = parts[0].parse().ok()?;
        let minutes: i64 = parts[1].parse().ok()?;
        Some(sign * (hours * 3600 + minutes * 60))
    } else {
        // Just minutes
        let minutes: i64 = rest.parse().ok()?;
        Some(sign * minutes * 60)
    }
}

/// Parse size string (e.g., "100", "1K", "10M", "1G") to bytes.
fn parse_size(s: &str) -> Option<u64> {
    let s = s.trim().to_uppercase();
    if s.is_empty() {
        return None;
    }
    
    let (num_str, multiplier) = if s.ends_with('K') {
        (&s[..s.len()-1], 1024u64)
    } else if s.ends_with("KB") {
        (&s[..s.len()-2], 1024u64)
    } else if s.ends_with('M') {
        (&s[..s.len()-1], 1024u64 * 1024)
    } else if s.ends_with("MB") {
        (&s[..s.len()-2], 1024u64 * 1024)
    } else if s.ends_with('G') {
        (&s[..s.len()-1], 1024u64 * 1024 * 1024)
    } else if s.ends_with("GB") {
        (&s[..s.len()-2], 1024u64 * 1024 * 1024)
    } else {
        (s.as_str(), 1u64)
    };
    
    num_str.trim().parse::<u64>().ok().map(|n| n * multiplier)
}

/// Check if path matches any exclude pattern (file name or full path).
fn matches_exclude(path: &Path, exclude: &[String]) -> bool {
    if exclude.is_empty() {
        return false;
    }
    
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let path_str = path.to_string_lossy();
    
    for pattern in exclude {
        // Match against filename or full path
        if glob_match(pattern, name) || glob_match(pattern, &path_str) {
            return true;
        }
    }
    false
}

/// Check if file size passes size filters.
fn passes_size_filter(path: &Path, minsize: Option<u64>, maxsize: Option<u64>) -> bool {
    let size = match path.metadata().map(|m| m.len()) {
        Ok(s) => s,
        Err(_) => return true, // Can't get size, include file
    };
    
    if let Some(min) = minsize {
        if size < min {
            return false;
        }
    }
    
    if let Some(max) = maxsize {
        if size > max {
            return false;
        }
    }
    
    true
}

/// Check if file modification time passes date filters.
fn passes_date_filter(path: &Path, newer: Option<std::time::SystemTime>, older: Option<std::time::SystemTime>) -> bool {
    let mtime = match path.metadata().and_then(|m| m.modified()) {
        Ok(t) => t,
        Err(_) => return true, // Can't get mtime, include file
    };
    
    if let Some(newer_than) = newer {
        if mtime <= newer_than {
            return false;
        }
    }
    
    if let Some(older_than) = older {
        if mtime >= older_than {
            return false;
        }
    }
    
    true
}

/// Expand paths: if recursive, walk directories; filter by extensions.
fn expand_paths(
    paths: &[PathBuf], 
    recursive: bool, 
    extensions: &[String], 
    exclude: &[String],
    newer: Option<std::time::SystemTime>,
    older: Option<std::time::SystemTime>,
    minsize: Option<u64>,
    maxsize: Option<u64>,
) -> Vec<PathBuf> {
    let mut result = Vec::new();
    
    // Known image/media extensions for recursive mode
    let default_exts: Vec<&str> = vec![
        // Images
        "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp", "heic", "heif", "avif",
        "jxl", "jp2", "j2k", "jpx", "exr", "hdr", "ppm", "pgm", "pbm", "pam", "ico",
        "tga", "pcx", "sgi", "rgb", "svg", "eps", "ai", "psd", "dpx",
        // RAW
        "cr2", "cr3", "nef", "arw", "orf", "rw2", "pef", "raf", "dng", "srw", "srf",
        "sr2", "crw", "dcr", "kdc", "k25", "erf", "mef", "mos", "mrw", "nrw", "rwl",
        "x3f", "3fr", "fff", "iiq", "braw",
        // Video
        "mp4", "mov", "m4v", "3gp", "3g2", "avi", "mkv", "webm", "mxf", "r3d",
        "mts", "m2ts", "ts", "flv", "wmv", "asf",
        // Audio
        "mp3", "flac", "m4a", "aac", "ogg", "opus", "wav", "aiff", "aif", "ape",
        "wv", "dsf", "dff", "tak", "wma", "mid", "midi", "au",
    ];
    
    for path in paths {
        if path.is_dir() {
            if recursive {
                // Walk directory recursively
                for entry in WalkDir::new(path)
                    .follow_links(true)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    let p = entry.path();
                    
                    // Skip excluded paths (check dirs early to skip subtrees)
                    if matches_exclude(p, exclude) {
                        continue;
                    }
                    
                    if p.is_file() {
                        if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                            let ext_lower = ext.to_lowercase();
                            // Check against filter or defaults
                            let matches = if extensions.is_empty() {
                                default_exts.contains(&ext_lower.as_str())
                            } else {
                                extensions.iter().any(|e| e == &ext_lower)
                            };
                            if matches 
                                && passes_date_filter(p, newer, older) 
                                && passes_size_filter(p, minsize, maxsize) 
                            {
                                result.push(p.to_path_buf());
                            }
                        }
                    }
                }
            } else {
                eprintln!("Warning: {} is a directory. Use -r for recursive scan.", path.display());
            }
        } else if path.is_file() {
            // Check exclude patterns
            if matches_exclude(path, exclude) {
                continue;
            }
            
            // Check date filters
            if !passes_date_filter(path, newer, older) {
                continue;
            }
            
            // Check size filters
            if !passes_size_filter(path, minsize, maxsize) {
                continue;
            }
            
            // Check extension filter if specified
            if !extensions.is_empty() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if !extensions.iter().any(|e| e == &ext.to_lowercase()) {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            result.push(path.clone());
        }
    }
    
    // Sort for consistent output
    result.sort();
    result
}

fn parse_args(args: &[String]) -> Result<Args> {
    let mut parsed = Args { 
        format: "text".into(), 
        charset: "utf8".into(),
        ..Default::default() 
    };
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-f" | "--format" => {
                i += 1;
                parsed.format = args.get(i).cloned().unwrap_or_default();
            }
            "-g" | "--get" => {
                i += 1;
                if let Some(tag) = args.get(i) {
                    parsed.get_tags.push(tag.clone());
                }
            }
            "-t" | "--tag" => {
                i += 1;
                if let Some(tv) = args.get(i) {
                    if let Some((t, v)) = tv.split_once('=') {
                        parsed.tags.push((t.to_string(), v.to_string()));
                    } else {
                        anyhow::bail!("Invalid -t format. Use: -t Tag=Value");
                    }
                }
            }
            "--shift" => {
                i += 1;
                if let Some(shift_str) = args.get(i) {
                    parsed.shift = parse_shift(shift_str);
                    if parsed.shift.is_none() {
                        anyhow::bail!("Invalid shift format: {}. Use +/-HH:MM or +/-MM", shift_str);
                    }
                }
            }
            "--geotag" => {
                i += 1;
                if let Some(gpx_path) = args.get(i) {
                    let path = PathBuf::from(gpx_path);
                    if !path.exists() {
                        anyhow::bail!("GPX file not found: {}", gpx_path);
                    }
                    parsed.geotag = Some(path);
                }
            }
            "--icc" | "--icc-profile" => {
                i += 1;
                if let Some(icc_path) = args.get(i) {
                    let path = PathBuf::from(icc_path);
                    if !path.exists() {
                        anyhow::bail!("ICC profile not found: {}", icc_path);
                    }
                    parsed.icc_profile = Some(path);
                }
            }
            "-o" | "--output" => {
                i += 1;
                parsed.output = args.get(i).map(PathBuf::from);
            }
            "-w" | "--write" => {
                i += 1;
                parsed.write_file = args.get(i).map(PathBuf::from);
            }
            "-X" | "--xml" => parsed.format = "xml".into(),
            "-p" | "--inplace" => parsed.inplace = true,
            "-T" | "--thumbnail" => parsed.thumbnail = true,
            "-P" | "--preview" => parsed.preview = true,
            "-r" | "--recursive" => parsed.recursive = true,
            "-e" | "--ext" => {
                i += 1;
                if let Some(exts) = args.get(i) {
                    parsed.extensions = exts.split(',')
                        .map(|s| s.trim().to_lowercase())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
            }
            "-x" | "--exclude" => {
                i += 1;
                if let Some(pat) = args.get(i) {
                    parsed.exclude.push(pat.clone());
                }
            }
            "--newer" => {
                i += 1;
                if let Some(date_str) = args.get(i) {
                    parsed.newer = parse_date(date_str);
                    if parsed.newer.is_none() {
                        anyhow::bail!("Invalid date format: {}. Use YYYY-MM-DD or YYYY-MM-DD HH:MM:SS", date_str);
                    }
                }
            }
            "--older" => {
                i += 1;
                if let Some(date_str) = args.get(i) {
                    parsed.older = parse_date(date_str);
                    if parsed.older.is_none() {
                        anyhow::bail!("Invalid date format: {}. Use YYYY-MM-DD or YYYY-MM-DD HH:MM:SS", date_str);
                    }
                }
            }
            "--minsize" => {
                i += 1;
                if let Some(size_str) = args.get(i) {
                    parsed.minsize = parse_size(size_str);
                    if parsed.minsize.is_none() {
                        anyhow::bail!("Invalid size format: {}. Use bytes or K/M/G suffix (e.g., 100, 1K, 10M)", size_str);
                    }
                }
            }
            "--maxsize" => {
                i += 1;
                if let Some(size_str) = args.get(i) {
                    parsed.maxsize = parse_size(size_str);
                    if parsed.maxsize.is_none() {
                        anyhow::bail!("Invalid size format: {}. Use bytes or K/M/G suffix (e.g., 100, 1K, 10M)", size_str);
                    }
                }
            }
            "-c" | "--composite" => parsed.composite = true,
            "--charset" => {
                i += 1;
                if let Some(enc) = args.get(i) {
                    let enc_lower = enc.to_lowercase();
                    if !matches!(enc_lower.as_str(), "utf8" | "utf-8" | "latin1" | "iso-8859-1" | "ascii") {
                        anyhow::bail!("Unsupported charset: {}. Use utf8, latin1, or ascii", enc);
                    }
                    parsed.charset = enc_lower;
                }
            }
            "-a" | "--all" => parsed.all = true,
            _ if arg.starts_with('-') => {
                // Check for combined -tTag=Value
                if arg.starts_with("-t") && arg.len() > 2 {
                    let rest = &arg[2..];
                    if let Some((t, v)) = rest.split_once('=') {
                        parsed.tags.push((t.to_string(), v.to_string()));
                    }
                } else {
                    anyhow::bail!("Unknown option: {}. Use 'exif --help'", arg);
                }
            }
            _ => parsed.files.push(PathBuf::from(arg)),
        }
        i += 1;
    }

    Ok(parsed)
}

fn write_image(args: &Args, registry: &FormatRegistry) -> Result<()> {
    if args.files.is_empty() {
        anyhow::bail!("No input file specified for write operation.\n\nUsage: exif -t Tag=Value <FILE>");
    }

    for path in &args.files {
        let file = File::open(path)
            .with_context(|| format!("Cannot open: {}", path.display()))?;
        let mut reader = BufReader::new(file);

        let mut metadata = registry.parse(&mut reader)
            .with_context(|| format!("Cannot parse: {}", path.display()))?;

        // Apply time shift if specified
        if let Some(offset) = args.shift {
            apply_time_shift(&mut metadata, offset);
        }
        
        // Apply geotag from GPX if specified
        if let Some(ref gpx_path) = args.geotag {
            match geotag::GpxTrack::from_file(gpx_path) {
                Ok(track) => {
                    // Get photo timestamp
                    let timestamp = metadata.exif.get("DateTimeOriginal")
                        .or_else(|| metadata.exif.get("CreateDate"))
                        .and_then(|v| v.as_str())
                        .and_then(|s| geotag::parse_exif_datetime(s));
                    
                    if let Some(ts) = timestamp {
                        if let Some((lat, lon, ele)) = track.find_position(ts) {
                            // Set GPS coordinates
                            let lat_ref = if lat >= 0.0 { "N" } else { "S" };
                            let lon_ref = if lon >= 0.0 { "E" } else { "W" };
                            
                            metadata.exif.set("GPSLatitude", AttrValue::Double(lat.abs()));
                            metadata.exif.set("GPSLatitudeRef", AttrValue::Str(lat_ref.to_string()));
                            metadata.exif.set("GPSLongitude", AttrValue::Double(lon.abs()));
                            metadata.exif.set("GPSLongitudeRef", AttrValue::Str(lon_ref.to_string()));
                            
                            if let Some(altitude) = ele {
                                let alt_ref = if altitude >= 0.0 { 0u32 } else { 1u32 };
                                metadata.exif.set("GPSAltitude", AttrValue::Double(altitude.abs()));
                                metadata.exif.set("GPSAltitudeRef", AttrValue::UInt(alt_ref));
                            }
                            
                            eprintln!("  Geotagged: {:.6}, {:.6}", lat, lon);
                        } else {
                            eprintln!("  Warning: No GPS position found for photo timestamp");
                        }
                    } else {
                        eprintln!("  Warning: Photo has no DateTimeOriginal for geotagging");
                    }
                }
                Err(e) => eprintln!("Warning: Cannot parse GPX: {}", e),
            }
        }
        
        // Apply ICC profile if specified
        if let Some(ref icc_path) = args.icc_profile {
            match std::fs::read(icc_path) {
                Ok(icc_data) => {
                    metadata.icc = Some(icc_data);
                    eprintln!("  ICC profile: {} ({} bytes)", icc_path.display(), metadata.icc.as_ref().map(|d| d.len()).unwrap_or(0));
                }
                Err(e) => eprintln!("Warning: Cannot read ICC profile: {}", e),
            }
        }
        
        // Apply tag updates
        for (tag, value) in &args.tags {
            metadata.exif.set(tag, AttrValue::Str(value.clone()));
        }

        // Output path: -w > -p > default (_modified)
        let output_path = if let Some(ref out) = args.write_file {
            out.clone()
        } else if args.inplace {
            path.clone()
        } else {
            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
            let ext = path.extension().unwrap_or_default().to_string_lossy();
            path.with_file_name(format!("{}_modified.{}", stem, ext))
        };

        // Check if format is writable (uses Make tag detection for TIFF-based RAW)
        if !metadata.is_writable() {
            let reason = if metadata.is_camera_raw() {
                let make = metadata.exif.get_str("Make").unwrap_or(metadata.format);
                format!("Camera RAW file ({}) is read-only", make.trim())
            } else {
                format!("Format {} does not support writing", metadata.format)
            };
            anyhow::bail!(
                "{}.\n\nWritable formats: JPEG, PNG, TIFF, DNG, EXR, HDR",
                reason
            );
        }

        // Re-read for writer
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut output_data = Vec::new();

        match metadata.format {
            "JPEG" => {
                let exif = build_exif_bytes(&metadata)?;
                JpegWriter::write(&mut reader, &mut output_data, Some(&exif), None)?;
            }
            "PNG" => PngWriter::write(&mut reader, &mut output_data, &metadata)?,
            "TIFF" | "DNG" => TiffWriter::write(&mut reader, &mut output_data, &metadata)?,
            "HDR" => HdrWriter::write(&mut reader, &mut output_data, &metadata)?,
            "EXR" => ExrWriter::write(&mut reader, &mut output_data, &metadata)?,
            fmt => anyhow::bail!("Write not supported for: {}", fmt),
        }

        // Atomic write for inplace mode
        if args.inplace && output_path == *path {
            let tmp = output_path.with_extension("tmp");
            std::fs::write(&tmp, &output_data)?;
            std::fs::rename(&tmp, &output_path)?;
        } else {
            std::fs::write(&output_path, &output_data)?;
        }

        eprintln!("Wrote: {} ({} bytes)", output_path.display(), output_data.len());
    }

    Ok(())
}

fn extract_thumbnails(args: &Args, registry: &FormatRegistry) -> Result<()> {
    if args.files.is_empty() {
        anyhow::bail!("No input file specified for thumbnail extraction.\n\nUsage: exif -T <FILE>");
    }

    for path in &args.files {
        let file = File::open(path)
            .with_context(|| format!("Cannot open: {}", path.display()))?;
        let mut reader = BufReader::new(file);

        let metadata = registry.parse(&mut reader)
            .with_context(|| format!("Cannot parse: {}", path.display()))?;

        if let Some(ref thumb_data) = metadata.thumbnail {
            // Determine output path
            let output_path = if let Some(ref out) = args.output {
                out.clone()
            } else {
                // Default: input_thumb.jpg
                let stem = path.file_stem().unwrap_or_default().to_string_lossy();
                path.with_file_name(format!("{}_thumb.jpg", stem))
            };

            std::fs::write(&output_path, thumb_data)
                .with_context(|| format!("Cannot write: {}", output_path.display()))?;
            
            eprintln!("Thumbnail: {} ({} bytes)", output_path.display(), thumb_data.len());
        } else {
            eprintln!("{}: no embedded thumbnail found", path.display());
        }
    }

    Ok(())
}

fn extract_previews(args: &Args, registry: &FormatRegistry) -> Result<()> {
    if args.files.is_empty() {
        anyhow::bail!("No input file specified for preview extraction.\n\nUsage: exif -P <FILE>");
    }

    for path in &args.files {
        let file = File::open(path)
            .with_context(|| format!("Cannot open: {}", path.display()))?;
        let mut reader = BufReader::new(file);

        let metadata = registry.parse(&mut reader)
            .with_context(|| format!("Cannot parse: {}", path.display()))?;

        if let Some(ref preview_data) = metadata.preview {
            // Determine output path
            let output_path = if let Some(ref out) = args.output {
                out.clone()
            } else {
                // Default: input_preview.jpg
                let stem = path.file_stem().unwrap_or_default().to_string_lossy();
                path.with_file_name(format!("{}_preview.jpg", stem))
            };

            std::fs::write(&output_path, preview_data)
                .with_context(|| format!("Cannot write: {}", output_path.display()))?;
            
            eprintln!("Preview: {} ({} bytes)", output_path.display(), preview_data.len());
        } else {
            eprintln!("{}: no embedded preview found", path.display());
        }
    }

    Ok(())
}

/// Output CSV with unified headers across all files.
/// Collects all metadata first to build superset of columns.
fn output_csv_unified(files: &[PathBuf], registry: &FormatRegistry, args: &Args) -> Result<()> {
    use std::fmt::Write;
    
    // Step 1: Parse all files and collect metadata + all tags
    let mut all_data: Vec<(PathBuf, Metadata)> = Vec::new();
    let mut all_tags: BTreeSet<String> = BTreeSet::new();
    
    for path in files {
        let file = File::open(path)
            .with_context(|| format!("Cannot open: {}", path.display()))?;
        let mut reader = BufReader::new(file);
        
        match registry.parse(&mut reader) {
            Ok(mut metadata) => {
                if args.composite {
                    add_composite_tags(&mut metadata);
                }
                
                // Collect all tag names (filtered or all)
                for (tag, _) in metadata.exif.iter() {
                    if tag_matches(tag, &args.get_tags) {
                        all_tags.insert(tag.clone());
                    }
                }
                
                all_data.push((path.clone(), metadata));
            }
            Err(e) => eprintln!("Error {}: {}", path.display(), e),
        }
    }
    
    if all_data.is_empty() {
        return Ok(());
    }
    
    // Step 2: Build column list (SourceFile first, then sorted tags)
    let columns: Vec<String> = {
        let mut cols = vec!["SourceFile".to_string()];
        cols.extend(all_tags.into_iter());
        cols
    };
    
    // Step 3: Output
    let mut output_buf = String::new();
    let write_to_file = args.output.is_some();
    
    // Header
    let header = columns.join(",");
    if write_to_file {
        writeln!(&mut output_buf, "{}", header).ok();
    } else {
        println!("{}", header);
    }
    
    // Data rows
    for (path, metadata) in &all_data {
        let row: Vec<String> = columns.iter().map(|col| {
            if col == "SourceFile" {
                format!("\"{}\"", path.display())
            } else {
                metadata.exif.get(col)
                    .map(|v| {
                        let s = v.to_string();
                        // Escape quotes in CSV values
                        if s.contains(',') || s.contains('"') || s.contains('\n') {
                            format!("\"{}\"", s.replace('"', "\"\""))
                        } else {
                            format!("\"{}\"", s)
                        }
                    })
                    .unwrap_or_default()
            }
        }).collect();
        
        if write_to_file {
            writeln!(&mut output_buf, "{}", row.join(",")).ok();
        } else {
            println!("{}", row.join(","));
        }
    }
    
    // Write to file if -o specified
    if let Some(ref output_path) = args.output {
        std::fs::write(output_path, &output_buf)
            .with_context(|| format!("Cannot write: {}", output_path.display()))?;
        eprintln!("Wrote: {}", output_path.display());
    }
    
    Ok(())
}

fn print_metadata(path: &Path, m: &Metadata, args: &Args) {
    match args.format.as_str() {
        "json" => print_json(path, m, &args.get_tags),
        "csv" => print_csv(path, m, &args.get_tags),
        "xml" => xml_output::print_xml(path, m, &args.get_tags),
        "html" => html_output::print_html(path, m, &args.get_tags),
        _ => print_text(path, m, args.all, &args.get_tags),
    }
}

fn format_metadata(path: &Path, m: &Metadata, args: &Args, out: &mut String) {
    use std::fmt::Write;
    let filter = &args.get_tags;
    
    match args.format.as_str() {
        "html" => {
            html_output::format_html(path, m, filter, out);
        }
        "xml" => {
            xml_output::format_xml(path, m, filter, out);
        }
        "json" => {
            let mut map = serde_json::Map::new();
            
            if is_simple_filter(filter) {
                if let Some(v) = m.exif.get(&filter[0]) {
                    let _ = writeln!(out, "{}", serde_json::to_string(&val_json(v)).unwrap());
                } else {
                    let _ = writeln!(out, "null");
                }
                return;
            }
            
            if filter.is_empty() {
                map.insert("SourceFile".into(), path.display().to_string().into());
                map.insert("Format".into(), m.format.into());
            }
            for (k, v) in m.exif.iter() {
                if tag_matches(k, filter) {
                    map.insert(k.clone(), val_json(v));
                }
            }
            let _ = writeln!(out, "{}", serde_json::to_string_pretty(
                &serde_json::Value::Array(vec![serde_json::Value::Object(map)])
            ).unwrap());
        }
        "csv" => {
            let keys: Vec<_> = if filter.is_empty() {
                let mut k: Vec<_> = m.exif.iter().map(|(k, _)| k.clone()).collect();
                k.sort();
                k.insert(0, "SourceFile".into());
                k
            } else {
                expand_filters(filter, m)
            };
            let _ = writeln!(out, "{}", keys.join(","));
            let vals: Vec<_> = keys.iter().map(|k| {
                if k == "SourceFile" {
                    format!("\"{}\"", path.display())
                } else {
                    m.exif.get(k).map(|v| format!("\"{}\"", v)).unwrap_or_default()
                }
            }).collect();
            let _ = writeln!(out, "{}", vals.join(","));
        }
        _ => {
            // Single tag, single file: just the value
            if is_simple_filter(filter) && args.files.len() == 1 {
                if let Some(v) = m.exif.get(&filter[0]) {
                    let _ = writeln!(out, "{}", v);
                }
                return;
            }
            
            if filter.is_empty() {
                let _ = writeln!(out, "── {} ──", path.display());
                let _ = writeln!(out, "{:28} {}", "Format", m.format);
            } else if args.files.len() > 1 {
                let _ = writeln!(out, "── {} ──", path.display());
            }
            
            let mut entries: Vec<_> = m.exif.iter()
                .filter(|(k, _)| tag_matches(k, filter))
                .collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            
            for (k, v) in entries {
                let vs = v.to_string();
                if vs.len() > 60 {
                    let _ = writeln!(out, "{:28} {}...", k, &vs[..57]);
                } else {
                    let _ = writeln!(out, "{:28} {}", k, vs);
                }
            }
            if filter.is_empty() {
                if let Some(ref xmp) = m.xmp {
                    let _ = writeln!(out, "{:28} {} bytes", "XMP", xmp.len());
                }
            }
            let _ = writeln!(out);
        }
    }
}

fn print_text(path: &Path, m: &Metadata, _all: bool, filter: &[String]) {
    // Single tag: just print value
    if is_simple_filter(filter) {
        if let Some(v) = m.exif.get(&filter[0]) {
            println!("{}", v);
        }
        return;
    }
    
    if filter.is_empty() {
        println!("── {} ──", path.display());
        println!("{:28} {}", "Format", m.format);
    }
    
    let mut entries: Vec<_> = m.exif.iter()
        .filter(|(k, _)| tag_matches(k, filter))
        .collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));
    
    for (k, v) in entries {
        let vs = v.to_string();
        if vs.len() > 60 {
            println!("{:28} {}...", k, &vs[..57]);
        } else {
            println!("{:28} {}", k, vs);
        }
    }
    
    if filter.is_empty() {
        if let Some(ref xmp) = m.xmp {
            println!("{:28} {} bytes", "XMP", xmp.len());
        }
        // Multi-page info
        if m.pages.len() > 1 {
            println!("{:28} {}", "Pages", m.pages.len());
            for page in &m.pages {
                let desc = if page.is_thumbnail() {
                    "(thumbnail)"
                } else if page.is_page() {
                    "(page)"
                } else {
                    ""
                };
                println!("  Page {:2}: {}x{} {}bpp {}", 
                    page.index, page.width, page.height, page.bits_per_sample, desc);
            }
        }
        // Thumbnail info
        if let Some(ref thumb) = m.thumbnail {
            println!("{:28} {} bytes", "Thumbnail", thumb.len());
        }
        println!();
    }
}

fn print_json(path: &Path, m: &Metadata, filter: &[String]) {
    let mut map = serde_json::Map::new();
    
    // Single tag: just the value
    if is_simple_filter(filter) {
        if let Some(v) = m.exif.get(&filter[0]) {
            println!("{}", serde_json::to_string(&val_json(v)).unwrap());
        } else {
            println!("null");
        }
        return;
    }
    
    if filter.is_empty() {
        map.insert("SourceFile".into(), path.display().to_string().into());
        map.insert("Format".into(), m.format.into());
        // Page count for multi-page TIFF
        if m.pages.len() > 1 {
            map.insert("PageCount".into(), (m.pages.len() as i64).into());
            let pages_arr: Vec<_> = m.pages.iter().map(|p| {
                serde_json::json!({
                    "index": p.index,
                    "width": p.width,
                    "height": p.height,
                    "bitsPerSample": p.bits_per_sample,
                    "compression": p.compression,
                    "subfileType": p.subfile_type
                })
            }).collect();
            map.insert("Pages".into(), serde_json::Value::Array(pages_arr));
        }
        // Thumbnail size
        if let Some(ref thumb) = m.thumbnail {
            map.insert("ThumbnailSize".into(), (thumb.len() as i64).into());
        }
    }
    
    for (k, v) in m.exif.iter() {
        if tag_matches(k, filter) {
            map.insert(k.clone(), val_json(v));
        }
    }
    
    println!("{}", serde_json::to_string_pretty(&serde_json::Value::Array(
        vec![serde_json::Value::Object(map)]
    )).unwrap());
}

fn print_csv(path: &Path, m: &Metadata, filter: &[String]) {
    let keys: Vec<_> = if filter.is_empty() {
        let mut k: Vec<_> = m.exif.iter().map(|(k, _)| k.clone()).collect();
        k.sort();
        k.insert(0, "SourceFile".into());
        k
    } else {
        expand_filters(filter, m)
    };
    
    println!("{}", keys.join(","));
    
    let vals: Vec<_> = keys.iter().map(|k| {
        if k == "SourceFile" {
            format!("\"{}\"", path.display())
        } else {
            m.exif.get(k).map(|v| format!("\"{}\"", v)).unwrap_or_default()
        }
    }).collect();
    println!("{}", vals.join(","));
}

fn val_json(v: &AttrValue) -> serde_json::Value {
    match v {
        AttrValue::Bool(b) => (*b).into(),
        AttrValue::Int(n) => (*n).into(),
        AttrValue::UInt(n) => (*n).into(),
        AttrValue::Float(f) => serde_json::json!(*f),
        AttrValue::Double(d) => serde_json::json!(*d),
        AttrValue::Str(s) => s.clone().into(),
        AttrValue::URational(n, d) if *d != 0 => serde_json::json!(*n as f64 / *d as f64),
        AttrValue::Rational(n, d) if *d != 0 => serde_json::json!(*n as f64 / *d as f64),
        _ => v.to_string().into(),
    }
}

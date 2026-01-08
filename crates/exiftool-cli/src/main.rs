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

IMPORT/COPY:
    exif --json=tags.json -p *.jpg              # import tags from JSON
    exif --csv=meta.csv                         # import tags from CSV
    exif --tagsFromFile src.jpg -p dst.jpg      # copy all tags from src to dst

RENAME:
    exif --rename '%Y%m%d_%H%M%S.%e' *.jpg      # rename by date: 20240115_103045.jpg
    exif --rename '$Make_$Model.%e' *.jpg       # rename by tags: Canon_EOS R5.jpg
    exif --rename '%Y/%m/%d/$Model.%e' *.jpg    # create dirs + rename

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
    --json=<FILE>        Import tags from JSON file
    --csv=<FILE>         Import tags from CSV file
    --tagsFromFile <F>   Copy tags from another image file
    --rename <TMPL>      Rename files using template ($Tag, %Y%m%d, %e=ext)
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
    --delete             Remove all metadata (EXIF, XMP, IPTC, ICC) from files
    --validate           Check metadata for issues (returns exit code 1 if problems)
    -if <COND>           Process only files where CONDITION is true
                         Ops: eq, ne, gt, lt, ge, le, contains, startswith, endswith
                         Examples: -if "Make eq Canon", -if "ISO gt 800"
    -htmlDump            Show file structure with hex preview (to HTML)
    -duplicates [BY]     Find duplicate files (BY: hash, content, datetime, metadata)
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

// =============================================================================
// Condition evaluation for -if
// =============================================================================

/// Condition operator
#[derive(Debug, Clone, Copy, PartialEq)]
enum CondOp {
    Eq,         // eq, =, ==
    Ne,         // ne, !=, <>
    Gt,         // gt, >
    Lt,         // lt, <
    Ge,         // ge, >=
    Le,         // le, <=
    Contains,   // contains, ~
    StartsWith, // startswith
    EndsWith,   // endswith
    Exists,     // tag exists (unary)
}

impl CondOp {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "eq" | "=" | "==" => Some(CondOp::Eq),
            "ne" | "!=" | "<>" => Some(CondOp::Ne),
            "gt" | ">" => Some(CondOp::Gt),
            "lt" | "<" => Some(CondOp::Lt),
            "ge" | ">=" => Some(CondOp::Ge),
            "le" | "<=" => Some(CondOp::Le),
            "contains" | "~" => Some(CondOp::Contains),
            "startswith" | "starts" => Some(CondOp::StartsWith),
            "endswith" | "ends" => Some(CondOp::EndsWith),
            _ => None,
        }
    }
}

/// Parsed condition
struct Condition {
    tag: String,
    op: CondOp,
    value: String,
}

impl Condition {
    /// Parse condition string: "Tag op Value" or just "Tag" for existence check
    fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        
        // Split into parts
        let parts: Vec<&str> = s.split_whitespace().collect();
        
        match parts.len() {
            // Just tag name = existence check
            1 => Some(Condition {
                tag: parts[0].to_string(),
                op: CondOp::Exists,
                value: String::new(),
            }),
            // Tag op Value
            n if n >= 3 => {
                let tag = parts[0].to_string();
                let op = CondOp::from_str(parts[1])?;
                // Value is everything after operator (allows spaces in value)
                let value = parts[2..].join(" ");
                Some(Condition { tag, op, value })
            }
            _ => None,
        }
    }
    
    /// Evaluate condition against metadata
    fn eval(&self, metadata: &Metadata) -> bool {
        // Get tag value from metadata
        let tag_value = metadata.exif.get(&self.tag)
            .map(|v| format_attr_value(v))
            .unwrap_or_default();
        
        // Existence check
        if self.op == CondOp::Exists {
            return metadata.exif.contains(&self.tag);
        }
        
        // No value = condition fails
        if tag_value.is_empty() {
            return false;
        }
        
        // Try numeric comparison
        let tag_num = parse_number(&tag_value);
        let val_num = parse_number(&self.value);
        
        match self.op {
            CondOp::Eq => {
                if let (Some(tn), Some(vn)) = (tag_num, val_num) {
                    (tn - vn).abs() < 0.001
                } else {
                    tag_value.eq_ignore_ascii_case(&self.value)
                }
            }
            CondOp::Ne => {
                if let (Some(tn), Some(vn)) = (tag_num, val_num) {
                    (tn - vn).abs() >= 0.001
                } else {
                    !tag_value.eq_ignore_ascii_case(&self.value)
                }
            }
            CondOp::Gt => {
                if let (Some(tn), Some(vn)) = (tag_num, val_num) {
                    tn > vn
                } else {
                    tag_value > self.value
                }
            }
            CondOp::Lt => {
                if let (Some(tn), Some(vn)) = (tag_num, val_num) {
                    tn < vn
                } else {
                    tag_value < self.value
                }
            }
            CondOp::Ge => {
                if let (Some(tn), Some(vn)) = (tag_num, val_num) {
                    tn >= vn
                } else {
                    tag_value >= self.value
                }
            }
            CondOp::Le => {
                if let (Some(tn), Some(vn)) = (tag_num, val_num) {
                    tn <= vn
                } else {
                    tag_value <= self.value
                }
            }
            CondOp::Contains => {
                tag_value.to_lowercase().contains(&self.value.to_lowercase())
            }
            CondOp::StartsWith => {
                tag_value.to_lowercase().starts_with(&self.value.to_lowercase())
            }
            CondOp::EndsWith => {
                tag_value.to_lowercase().ends_with(&self.value.to_lowercase())
            }
            CondOp::Exists => unreachable!(),
        }
    }
}

/// Parse number from string (handles ratios like "1/200", "f/2.8", "100 mm")
fn parse_number(s: &str) -> Option<f64> {
    let s = s.trim();
    
    // Remove common suffixes
    let s = s.trim_end_matches(|c: char| c.is_alphabetic() || c == ' ');
    
    // Handle f/X notation for aperture
    let s = s.strip_prefix("f/").unwrap_or(s);
    let s = s.strip_prefix("F/").unwrap_or(s);
    
    // Handle ratios like "1/200"
    if let Some((num, den)) = s.split_once('/') {
        let n: f64 = num.trim().parse().ok()?;
        let d: f64 = den.trim().parse().ok()?;
        if d != 0.0 {
            return Some(n / d);
        }
    }
    
    // Plain number
    s.parse().ok()
}

/// Format AttrValue for display/comparison
fn format_attr_value(v: &AttrValue) -> String {
    match v {
        AttrValue::Str(s) => s.clone(),
        AttrValue::Bool(b) => b.to_string(),
        AttrValue::Int8(n) => n.to_string(),
        AttrValue::Int(n) => n.to_string(),
        AttrValue::UInt(n) => n.to_string(),
        AttrValue::Int64(n) => n.to_string(),
        AttrValue::UInt64(n) => n.to_string(),
        AttrValue::Float(f) => format!("{}", f),
        AttrValue::Double(f) => format!("{}", f),
        AttrValue::Rational(n, d) => {
            if *d == 1 { n.to_string() } else { format!("{}/{}", n, d) }
        }
        AttrValue::URational(n, d) => {
            if *d == 1 { n.to_string() } else { format!("{}/{}", n, d) }
        }
        AttrValue::Bytes(b) => format!("({} bytes)", b.len()),
        AttrValue::DateTime(dt) => dt.to_string(),
        AttrValue::Uuid(u) => u.to_string(),
        AttrValue::Json(j) => j.clone(),
        AttrValue::Vec3(v) => format!("{},{},{}", v[0], v[1], v[2]),
        AttrValue::Vec4(v) => format!("{},{},{},{}", v[0], v[1], v[2], v[3]),
        AttrValue::List(l) => format!("[{} items]", l.len()),
        AttrValue::Map(m) => format!("{{{}}} items", m.len()),
        AttrValue::Set(s) => format!("{{{} items}}", s.len()),
        AttrValue::Group(g) => format!("({} attrs)", g.len()),
    }
}

/// Check if file matches -if condition
fn matches_condition(metadata: &Metadata, condition: &str) -> bool {
    if let Some(cond) = Condition::parse(condition) {
        cond.eval(metadata)
    } else {
        eprintln!("Warning: invalid condition syntax: {}", condition);
        true // On parse error, include file
    }
}

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

    // JSON import mode
    if parsed.json_import.is_some() {
        return import_from_json(&parsed, &registry);
    }

    // CSV import mode
    if parsed.csv_import.is_some() {
        return import_from_csv(&parsed, &registry);
    }

    // Rename mode
    if parsed.rename.is_some() {
        return rename_files(&parsed, &registry);
    }

    // Delete metadata mode
    if parsed.delete {
        return delete_metadata(&parsed, &registry);
    }

    // Validate metadata mode
    if parsed.validate {
        return validate_metadata(&parsed, &registry);
    }

    // HTML dump mode (show file structure)
    if parsed.html_dump {
        return html_dump(&parsed, &registry);
    }

    // Duplicates mode
    if parsed.duplicates {
        return find_duplicates(&parsed, &registry);
    }

    // Write mode (modify image tags or copy from file)
    if !parsed.tags.is_empty() || parsed.tags_from_file.is_some() || parsed.shift.is_some() 
        || parsed.geotag.is_some() || parsed.icc_profile.is_some() {
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
                
                // Apply -if condition filter
                if let Some(ref cond) = parsed.if_condition {
                    if !matches_condition(&metadata, cond) {
                        continue; // Skip file that doesn't match condition
                    }
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
    // Import/copy options
    json_import: Option<PathBuf>,     // --json= import tags from JSON
    csv_import: Option<PathBuf>,      // --csv= import tags from CSV
    tags_from_file: Option<PathBuf>,  // --tagsFromFile copy from another image
    copy_tags: Vec<String>,           // tags to copy (empty = all)
    // Rename
    rename: Option<String>,           // --rename template for batch renaming
    // Delete/validate
    delete: bool,                     // --delete remove all metadata
    validate: bool,                   // --validate check metadata
    // Conditional processing
    if_condition: Option<String>,     // -if CONDITION filter by metadata
    // HTML dump
    html_dump: bool,                  // -htmlDump show file structure
    // Duplicates
    duplicates: bool,                 // -duplicates find duplicate files
    dup_by: String,                   // hash, content, metadata, datetime
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
            "--delete" | "--strip" => parsed.delete = true,
            "--validate" => parsed.validate = true,
            "-if" => {
                i += 1;
                if let Some(cond) = args.get(i) {
                    parsed.if_condition = Some(cond.clone());
                } else {
                    anyhow::bail!("-if requires a condition string");
                }
            }
            "-htmlDump" | "--htmlDump" | "--html-dump" => parsed.html_dump = true,
            "-duplicates" | "--duplicates" | "-dup" => {
                parsed.duplicates = true;
                // Check if next arg is a method specifier
                if let Some(next) = args.get(i + 1) {
                    if !next.starts_with('-') && ["hash", "content", "datetime", "metadata"].contains(&next.as_str()) {
                        parsed.dup_by = next.clone();
                        i += 1;
                    }
                }
                if parsed.dup_by.is_empty() {
                    parsed.dup_by = "hash".to_string();
                }
            }
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
            _ if arg.starts_with("--json=") => {
                let path_str = &arg[7..];
                let path = PathBuf::from(path_str);
                if !path.exists() {
                    anyhow::bail!("JSON file not found: {}", path_str);
                }
                parsed.json_import = Some(path);
            }
            _ if arg.starts_with("--csv=") => {
                let path_str = &arg[6..];
                let path = PathBuf::from(path_str);
                if !path.exists() {
                    anyhow::bail!("CSV file not found: {}", path_str);
                }
                parsed.csv_import = Some(path);
            }
            "--tagsFromFile" | "--tagsfromfile" | "-tagsFromFile" => {
                i += 1;
                if let Some(src_path) = args.get(i) {
                    let path = PathBuf::from(src_path);
                    if !path.exists() {
                        anyhow::bail!("Source file not found: {}", src_path);
                    }
                    parsed.tags_from_file = Some(path);
                }
            }
            "--rename" => {
                i += 1;
                if let Some(template) = args.get(i) {
                    parsed.rename = Some(template.to_string());
                }
            }
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
        
        // Copy tags from source file if specified
        if let Some(ref src_path) = args.tags_from_file {
            let src_file = File::open(src_path)
                .with_context(|| format!("Cannot open source: {}", src_path.display()))?;
            let mut src_reader = BufReader::new(src_file);
            
            match registry.parse(&mut src_reader) {
                Ok(src_meta) => {
                    let mut copied = 0;
                    for (tag, value) in src_meta.exif.iter() {
                        // Skip internal/binary tags
                        if tag.starts_with("_") || tag == "ThumbnailImage" || tag == "PreviewImage" {
                            continue;
                        }
                        // If copy_tags specified, only copy those
                        if !args.copy_tags.is_empty() && !args.copy_tags.iter().any(|t| t.eq_ignore_ascii_case(tag)) {
                            continue;
                        }
                        metadata.exif.set(tag, value.clone());
                        copied += 1;
                    }
                    // Also copy XMP if present
                    if src_meta.xmp.is_some() && (args.copy_tags.is_empty() || args.copy_tags.iter().any(|t| t.eq_ignore_ascii_case("XMP"))) {
                        metadata.xmp = src_meta.xmp.clone();
                    }
                    // Also copy ICC if present
                    if src_meta.icc.is_some() && (args.copy_tags.is_empty() || args.copy_tags.iter().any(|t| t.eq_ignore_ascii_case("ICC"))) {
                        metadata.icc = src_meta.icc.clone();
                    }
                    eprintln!("  Copied {} tags from {}", copied, src_path.display());
                }
                Err(e) => eprintln!("Warning: Cannot parse source file: {}", e),
            }
        }
        
        // Apply tag updates (override copied tags)
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
                JpegWriter::write(&mut reader, &mut output_data, Some(&exif), None, None)?;
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
                
                // Apply -if condition filter
                if let Some(ref cond) = args.if_condition {
                    if !matches_condition(&metadata, cond) {
                        continue; // Skip file that doesn't match condition
                    }
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
                let _ = writeln!(out, "{:28} {}", k, v);
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
        println!("{:28} {}", k, v);
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

/// Import tags from JSON file.
/// JSON format:
/// ```json
/// {
///   "photo1.jpg": { "Artist": "John", "Copyright": "2024" },
///   "photo2.jpg": { "Artist": "Jane" }
/// }
/// ```
/// Or for single file (use with -p):
/// ```json
/// { "Artist": "John", "Copyright": "2024" }
/// ```
fn import_from_json(args: &Args, registry: &FormatRegistry) -> Result<()> {
    let json_path = args.json_import.as_ref().unwrap();
    let json_str = std::fs::read_to_string(json_path)
        .with_context(|| format!("Cannot read: {}", json_path.display()))?;
    
    let json: serde_json::Value = serde_json::from_str(&json_str)
        .with_context(|| format!("Invalid JSON in: {}", json_path.display()))?;
    
    let obj = json.as_object()
        .ok_or_else(|| anyhow::anyhow!("JSON must be an object"))?;
    
    // Check if it's file-keyed or direct tags
    let is_file_keyed = obj.values().next()
        .map(|v| v.is_object())
        .unwrap_or(false);
    
    if is_file_keyed {
        // Format: { "file.jpg": { "Tag": "Value" } }
        for (file_path, tags_val) in obj {
            let path = PathBuf::from(file_path);
            if !path.exists() {
                eprintln!("Warning: File not found: {}", file_path);
                continue;
            }
            
            let tags = tags_val.as_object()
                .ok_or_else(|| anyhow::anyhow!("Tags for {} must be an object", file_path))?;
            
            write_tags_to_file(&path, tags, args, registry)?;
        }
    } else {
        // Format: { "Tag": "Value" } - apply to files in args
        if args.files.is_empty() {
            anyhow::bail!("No target files specified. Use: exif --json=tags.json -p photo.jpg");
        }
        
        for path in &args.files {
            write_tags_to_file(path, obj, args, registry)?;
        }
    }
    
    Ok(())
}

/// Import tags from CSV file.
/// CSV format:
/// ```csv
/// SourceFile,Artist,Copyright,Description
/// photo1.jpg,John Doe,2024,My photo
/// photo2.jpg,Jane Doe,2024,Another photo
/// ```
fn import_from_csv(args: &Args, registry: &FormatRegistry) -> Result<()> {
    let csv_path = args.csv_import.as_ref().unwrap();
    let csv_str = std::fs::read_to_string(csv_path)
        .with_context(|| format!("Cannot read: {}", csv_path.display()))?;
    
    let mut lines = csv_str.lines();
    
    // Parse header
    let header_line = lines.next()
        .ok_or_else(|| anyhow::anyhow!("CSV file is empty"))?;
    let headers: Vec<&str> = parse_csv_line(header_line);
    
    // Find SourceFile column
    let source_col = headers.iter().position(|h| h.eq_ignore_ascii_case("SourceFile"))
        .ok_or_else(|| anyhow::anyhow!("CSV must have SourceFile column"))?;
    
    // Process data rows
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        
        let values: Vec<&str> = parse_csv_line(line);
        if values.len() <= source_col {
            continue;
        }
        
        let file_path = values[source_col];
        let path = PathBuf::from(file_path);
        
        if !path.exists() {
            eprintln!("Warning: File not found: {}", file_path);
            continue;
        }
        
        // Build tags map
        let mut tags = serde_json::Map::new();
        for (i, header) in headers.iter().enumerate() {
            if i == source_col || *header == "Format" {
                continue; // Skip SourceFile and Format columns
            }
            if let Some(value) = values.get(i) {
                if !value.is_empty() {
                    tags.insert(header.to_string(), serde_json::Value::String(value.to_string()));
                }
            }
        }
        
        if !tags.is_empty() {
            write_tags_to_file(&path, &tags, args, registry)?;
        }
    }
    
    Ok(())
}

/// Parse a CSV line, handling quoted values.
fn parse_csv_line(line: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;
    let chars: Vec<char> = line.chars().collect();
    
    for (i, &c) in chars.iter().enumerate() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == ',' && !in_quotes {
            let field = &line[start..i];
            result.push(field.trim().trim_matches('"'));
            start = i + 1;
        }
    }
    
    // Last field
    if start <= line.len() {
        result.push(line[start..].trim().trim_matches('"'));
    }
    
    result
}

/// Write tags to a single file.
fn write_tags_to_file(
    path: &Path, 
    tags: &serde_json::Map<String, serde_json::Value>,
    args: &Args,
    registry: &FormatRegistry,
) -> Result<()> {
    let file = File::open(path)
        .with_context(|| format!("Cannot open: {}", path.display()))?;
    let mut reader = BufReader::new(file);

    let mut metadata = registry.parse(&mut reader)
        .with_context(|| format!("Cannot parse: {}", path.display()))?;

    // Check if writable
    if !metadata.is_writable() {
        eprintln!("Warning: {} is not writable, skipping", path.display());
        return Ok(());
    }

    // Apply tags
    for (tag, value) in tags {
        let str_val = match value {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            _ => value.to_string(),
        };
        metadata.exif.set(tag, AttrValue::Str(str_val));
    }

    // Output path
    let output_path = if args.inplace {
        path.to_path_buf()
    } else if let Some(ref out) = args.write_file {
        out.clone()
    } else {
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let ext = path.extension().unwrap_or_default().to_string_lossy();
        path.with_file_name(format!("{}_modified.{}", stem, ext))
    };

    // Re-read and write
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut output_data = Vec::new();

    match metadata.format {
        "JPEG" => {
            let exif = build_exif_bytes(&metadata)?;
            JpegWriter::write(&mut reader, &mut output_data, Some(&exif), None, None)?;
        }
        "PNG" => PngWriter::write(&mut reader, &mut output_data, &metadata)?,
        "TIFF" | "DNG" => TiffWriter::write(&mut reader, &mut output_data, &metadata)?,
        "HDR" => HdrWriter::write(&mut reader, &mut output_data, &metadata)?,
        "EXR" => ExrWriter::write(&mut reader, &mut output_data, &metadata)?,
        fmt => {
            eprintln!("Warning: Write not supported for {}: {}", fmt, path.display());
            return Ok(());
        }
    }

    // Atomic write
    if args.inplace && output_path == *path {
        let tmp = output_path.with_extension("tmp");
        std::fs::write(&tmp, &output_data)?;
        std::fs::rename(&tmp, &output_path)?;
    } else {
        std::fs::write(&output_path, &output_data)?;
    }

    eprintln!("Wrote: {} ({} tags)", output_path.display(), tags.len());
    Ok(())
}

/// Rename files using metadata template.
/// Template syntax:
/// - $Tag - metadata tag value (e.g., $Make, $Model, $DateTimeOriginal)
/// - %Y, %m, %d, %H, %M, %S - date/time from DateTimeOriginal
/// - %e - original extension (without dot)
/// - %c - copy number if file exists (01, 02, ...)
/// - %% - literal %
fn rename_files(args: &Args, registry: &FormatRegistry) -> Result<()> {
    let template = args.rename.as_ref().unwrap();
    
    // Expand paths
    let files = expand_paths(
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
        anyhow::bail!("No files to rename");
    }
    
    let mut renamed = 0;
    let mut errors = 0;
    
    for path in &files {
        match rename_single_file(path, template, registry) {
            Ok(new_path) => {
                if new_path != *path {
                    println!("{} -> {}", path.display(), new_path.display());
                    renamed += 1;
                }
            }
            Err(e) => {
                eprintln!("Error renaming {}: {}", path.display(), e);
                errors += 1;
            }
        }
    }
    
    eprintln!("Renamed {} files, {} errors", renamed, errors);
    Ok(())
}

/// Rename a single file using template.
fn rename_single_file(path: &Path, template: &str, registry: &FormatRegistry) -> Result<PathBuf> {
    // Read metadata
    let file = File::open(path)
        .with_context(|| format!("Cannot open: {}", path.display()))?;
    let mut reader = BufReader::new(file);
    let metadata = registry.parse(&mut reader)
        .with_context(|| format!("Cannot parse: {}", path.display()))?;
    
    // Get original extension
    let ext = path.extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_default();
    
    // Get datetime for strftime formatting
    let datetime = metadata.exif.get("DateTimeOriginal")
        .or_else(|| metadata.exif.get("CreateDate"))
        .or_else(|| metadata.exif.get("ModifyDate"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    // Expand template
    let new_name = expand_rename_template(template, &metadata, &ext, datetime.as_deref())?;
    
    // Determine new path
    let parent = path.parent().unwrap_or(Path::new("."));
    let mut new_path = parent.join(&new_name);
    
    // Handle subdirectories in template (e.g., %Y/%m/%d/name.jpg)
    if new_name.contains('/') || new_name.contains('\\') {
        // Create directories if needed
        if let Some(new_parent) = new_path.parent() {
            std::fs::create_dir_all(new_parent)?;
        }
    }
    
    // Handle duplicate filenames with %c counter
    if new_path.exists() && new_path != *path {
        let stem = new_path.file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let new_ext = new_path.extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();
        let new_parent = new_path.parent().unwrap_or(Path::new("."));
        
        for i in 1..1000 {
            let candidate = new_parent.join(format!("{}_{:02}.{}", stem, i, new_ext));
            if !candidate.exists() {
                new_path = candidate;
                break;
            }
        }
    }
    
    // Rename if different
    if new_path != *path {
        std::fs::rename(path, &new_path)?;
    }
    
    Ok(new_path)
}

/// Expand rename template with metadata values.
fn expand_rename_template(
    template: &str,
    metadata: &Metadata,
    ext: &str,
    datetime: Option<&str>,
) -> Result<String> {
    let mut result = String::with_capacity(template.len() * 2);
    let mut chars = template.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '$' => {
                // Tag substitution: $TagName
                let mut tag_name = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_alphanumeric() || nc == '_' || nc == '-' || nc == ':' {
                        tag_name.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                
                if tag_name.is_empty() {
                    result.push('$');
                } else {
                    // Find tag value
                    let value = metadata.exif.get(&tag_name)
                        .map(|v| sanitize_filename(&v.to_string()))
                        .unwrap_or_default();
                    result.push_str(&value);
                }
            }
            '%' => {
                // Date/time format or special
                if let Some(&nc) = chars.peek() {
                    chars.next();
                    match nc {
                        '%' => result.push('%'),
                        'e' => result.push_str(ext),
                        'Y' | 'm' | 'd' | 'H' | 'M' | 'S' => {
                            if let Some(dt) = datetime {
                                let val = extract_datetime_part(dt, nc);
                                result.push_str(&val);
                            }
                        }
                        _ => {
                            result.push('%');
                            result.push(nc);
                        }
                    }
                } else {
                    result.push('%');
                }
            }
            _ => result.push(c),
        }
    }
    
    // Ensure we have an extension
    if !result.contains('.') && !ext.is_empty() {
        result.push('.');
        result.push_str(ext);
    }
    
    Ok(result)
}

/// Extract datetime component from EXIF datetime string.
/// Format: "YYYY:MM:DD HH:MM:SS"
fn extract_datetime_part(datetime: &str, part: char) -> String {
    let dt = datetime.trim();
    match part {
        'Y' if dt.len() >= 4 => dt[0..4].to_string(),
        'm' if dt.len() >= 7 => dt[5..7].to_string(),
        'd' if dt.len() >= 10 => dt[8..10].to_string(),
        'H' if dt.len() >= 13 => dt[11..13].to_string(),
        'M' if dt.len() >= 16 => dt[14..16].to_string(),
        'S' if dt.len() >= 19 => dt[17..19].to_string(),
        _ => String::new(),
    }
}

/// Sanitize string for use in filename (remove/replace unsafe chars).
fn sanitize_filename(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

// =============================================================================
// Delete Metadata
// =============================================================================

/// Remove all metadata from files.
fn delete_metadata(args: &Args, registry: &FormatRegistry) -> Result<()> {
    if !args.inplace && args.write_file.is_none() {
        anyhow::bail!("--delete requires -p (in-place) or -w <file> (output file)");
    }
    
    let files = expand_paths(
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
        anyhow::bail!("No files to process");
    }
    
    let mut processed = 0;
    let mut errors = 0;
    
    for path in &files {
        match delete_metadata_single(path, args, registry) {
            Ok(()) => {
                println!("Stripped: {}", path.display());
                processed += 1;
            }
            Err(e) => {
                eprintln!("Error {}: {}", path.display(), e);
                errors += 1;
            }
        }
    }
    
    eprintln!("Processed {} files, {} errors", processed, errors);
    if errors > 0 {
        std::process::exit(1);
    }
    Ok(())
}

/// Delete metadata from a single file.
fn delete_metadata_single(path: &Path, args: &Args, registry: &FormatRegistry) -> Result<()> {
    use std::io::{Seek, SeekFrom};
    
    let file = File::open(path)
        .with_context(|| format!("Cannot open: {}", path.display()))?;
    let mut reader = BufReader::new(file);
    let mut metadata = registry.parse(&mut reader)
        .with_context(|| format!("Cannot parse: {}", path.display()))?;
    
    // Check if writable
    if !metadata.is_writable() {
        anyhow::bail!("Format {} does not support writing", metadata.format);
    }
    
    // Clear all metadata
    metadata.exif.clear();
    metadata.xmp = None;
    metadata.icc = None;
    metadata.thumbnail = None;
    metadata.preview = None;
    
    // Seek back to start
    reader.seek(SeekFrom::Start(0))?;
    let mut output = Vec::new();
    
    match metadata.format {
        "JPEG" => {
            // Write JPEG without any metadata segments
            JpegWriter::write(&mut reader, &mut output, None, None, None)?;
        }
        "PNG" => {
            PngWriter::write(&mut reader, &mut output, &metadata)?;
        }
        "TIFF" | "DNG" => {
            TiffWriter::write(&mut reader, &mut output, &metadata)?;
        }
        "EXR" => {
            ExrWriter::write(&mut reader, &mut output, &metadata)?;
        }
        "HDR" => {
            HdrWriter::write(&mut reader, &mut output, &metadata)?;
        }
        fmt => anyhow::bail!("Cannot strip metadata from {}", fmt),
    }
    
    // Write output
    let output_path = args.write_file.as_deref().unwrap_or(path);
    let tmp_path = output_path.with_extension("tmp");
    std::fs::write(&tmp_path, &output)?;
    std::fs::rename(&tmp_path, output_path)?;
    
    Ok(())
}

// =============================================================================
// Find Duplicates
// =============================================================================

use std::collections::HashMap;

/// Find duplicate files.
fn find_duplicates(args: &Args, registry: &FormatRegistry) -> Result<()> {
    let files = expand_paths(
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
        anyhow::bail!("No files specified for -duplicates");
    }

    eprintln!("Scanning {} files for duplicates (by {})...", files.len(), args.dup_by);

    // Group files by their key (hash, datetime, etc.)
    let mut groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for path in &files {
        let key = match args.dup_by.as_str() {
            "hash" | "content" => {
                // Fast hash using file content
                match std::fs::read(path) {
                    Ok(data) => {
                        let hash = simple_hash(&data);
                        format!("{:016x}_{}", hash, data.len())
                    }
                    Err(_) => continue,
                }
            }
            "datetime" => {
                // Use DateTimeOriginal or CreateDate
                let file = match File::open(path) {
                    Ok(f) => f,
                    Err(_) => continue,
                };
                let mut reader = BufReader::new(file);
                match registry.parse(&mut reader) {
                    Ok(metadata) => {
                        metadata.exif.get("DateTimeOriginal")
                            .or_else(|| metadata.exif.get("CreateDate"))
                            .or_else(|| metadata.exif.get("DateTime"))
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    }
                    Err(_) => continue,
                }
            }
            "metadata" => {
                // Use Make + Model + DateTime + dimensions
                let file = match File::open(path) {
                    Ok(f) => f,
                    Err(_) => continue,
                };
                let mut reader = BufReader::new(file);
                match registry.parse(&mut reader) {
                    Ok(metadata) => {
                        let make = metadata.exif.get("Make").map(|v| v.to_string()).unwrap_or_default();
                        let model = metadata.exif.get("Model").map(|v| v.to_string()).unwrap_or_default();
                        let dt = metadata.exif.get("DateTimeOriginal")
                            .or_else(|| metadata.exif.get("CreateDate"))
                            .map(|v| v.to_string())
                            .unwrap_or_default();
                        let w = metadata.exif.get("ImageWidth").map(|v| v.to_string()).unwrap_or_default();
                        let h = metadata.exif.get("ImageHeight").map(|v| v.to_string()).unwrap_or_default();
                        format!("{}|{}|{}|{}x{}", make, model, dt, w, h)
                    }
                    Err(_) => continue,
                }
            }
            _ => {
                eprintln!("Unknown duplicate method: {}. Use: hash, content, datetime, metadata", args.dup_by);
                return Ok(());
            }
        };

        if !key.is_empty() {
            groups.entry(key).or_default().push(path.clone());
        }
    }

    // Find groups with more than one file (duplicates)
    let mut dup_groups: Vec<_> = groups.into_iter()
        .filter(|(_, files)| files.len() > 1)
        .collect();
    dup_groups.sort_by(|a, b| b.1.len().cmp(&a.1.len())); // Sort by count desc

    if dup_groups.is_empty() {
        println!("No duplicates found.");
        return Ok(());
    }

    // Output results
    let total_dups: usize = dup_groups.iter().map(|(_, f)| f.len() - 1).sum();
    println!("Found {} duplicate groups ({} duplicate files):\n", dup_groups.len(), total_dups);

    for (i, (key, files)) in dup_groups.iter().enumerate() {
        let display_key = if key.len() > 60 { &key[..60] } else { key };
        println!("Group {} ({} files) [{}]", i + 1, files.len(), display_key);
        for f in files {
            // Get file size
            let size = std::fs::metadata(f).map(|m| m.len()).unwrap_or(0);
            println!("  {} ({} bytes)", f.display(), size);
        }
        println!();
    }

    // Summary
    let wasted: u64 = dup_groups.iter()
        .flat_map(|(_, files)| files.iter().skip(1)) // Skip first (original)
        .filter_map(|f| std::fs::metadata(f).ok())
        .map(|m| m.len())
        .sum();
    
    println!("Total wasted space: {:.2} MB", wasted as f64 / 1_048_576.0);

    Ok(())
}

/// Simple fast hash for file content.
fn simple_hash(data: &[u8]) -> u64 {
    // FNV-1a hash
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in data {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

// =============================================================================
// HTML Dump - File Structure Visualization
// =============================================================================

/// Generate HTML dump showing file structure.
fn html_dump(args: &Args, registry: &FormatRegistry) -> Result<()> {
    let files = expand_paths(
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
        anyhow::bail!("No files specified for -htmlDump");
    }

    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<title>File Structure Dump</title>\n");
    html.push_str("<style>\n");
    html.push_str("body { font-family: 'SF Mono', Monaco, Consolas, monospace; margin: 20px; background: #1e1e1e; color: #d4d4d4; }\n");
    html.push_str("h1 { color: #569cd6; }\n");
    html.push_str("h2 { color: #4ec9b0; border-bottom: 1px solid #444; padding-bottom: 5px; }\n");
    html.push_str(".file-info { background: #252526; padding: 15px; border-radius: 5px; margin: 10px 0; }\n");
    html.push_str(".hex-dump { background: #1e1e1e; border: 1px solid #444; padding: 10px; overflow-x: auto; }\n");
    html.push_str(".hex-row { display: flex; }\n");
    html.push_str(".hex-offset { color: #608b4e; width: 80px; }\n");
    html.push_str(".hex-bytes { color: #ce9178; flex: 1; }\n");
    html.push_str(".hex-ascii { color: #9cdcfe; width: 180px; }\n");
    html.push_str(".marker { background: #264f78; padding: 2px 6px; border-radius: 3px; margin: 2px; display: inline-block; }\n");
    html.push_str(".marker-exif { background: #4e7a25; }\n");
    html.push_str(".marker-xmp { background: #7a4e25; }\n");
    html.push_str(".marker-icc { background: #4e257a; }\n");
    html.push_str(".meta-table { width: 100%; border-collapse: collapse; margin: 10px 0; }\n");
    html.push_str(".meta-table th, .meta-table td { padding: 8px; text-align: left; border-bottom: 1px solid #444; }\n");
    html.push_str(".meta-table th { background: #333; color: #569cd6; }\n");
    html.push_str(".section { margin: 20px 0; }\n");
    html.push_str("</style>\n</head>\n<body>\n");
    html.push_str("<h1>File Structure Analysis</h1>\n");

    for path in &files {
        html_dump_single(path, registry, &mut html)?;
    }

    html.push_str("</body>\n</html>\n");

    // Output to file or stdout
    if let Some(ref output_path) = args.output {
        std::fs::write(output_path, &html)
            .with_context(|| format!("Cannot write: {}", output_path.display()))?;
        eprintln!("Wrote: {}", output_path.display());
    } else {
        print!("{}", html);
    }

    Ok(())
}

/// Generate HTML dump for a single file.
fn html_dump_single(path: &Path, registry: &FormatRegistry, html: &mut String) -> Result<()> {
    use std::fmt::Write;

    let file_data = std::fs::read(path)
        .with_context(|| format!("Cannot read: {}", path.display()))?;
    
    let file_size = file_data.len();
    let _ = writeln!(html, "<div class=\"file-info\">");
    let _ = writeln!(html, "<h2>{}</h2>", escape_html(&path.display().to_string()));
    let _ = writeln!(html, "<p><strong>Size:</strong> {} bytes ({:.2} KB)</p>", file_size, file_size as f64 / 1024.0);

    // Detect format and show markers
    let format = detect_format(&file_data);
    let _ = writeln!(html, "<p><strong>Format:</strong> {}</p>", format);

    // Show file structure markers
    let _ = writeln!(html, "<div class=\"section\"><h3>Structure</h3>");
    show_structure_markers(&file_data, &format, html);
    let _ = writeln!(html, "</div>");

    // Hex dump of first 256 bytes
    let _ = writeln!(html, "<div class=\"section\"><h3>Header (first 256 bytes)</h3>");
    let _ = writeln!(html, "<div class=\"hex-dump\">");
    let preview_len = file_data.len().min(256);
    for offset in (0..preview_len).step_by(16) {
        let end = (offset + 16).min(preview_len);
        let chunk = &file_data[offset..end];
        
        let _ = write!(html, "<div class=\"hex-row\">");
        let _ = write!(html, "<span class=\"hex-offset\">{:08X}</span>", offset);
        
        // Hex bytes
        let _ = write!(html, "<span class=\"hex-bytes\">");
        for (i, b) in chunk.iter().enumerate() {
            if i == 8 { let _ = write!(html, " "); }
            let _ = write!(html, "{:02X} ", b);
        }
        // Pad if needed
        for i in chunk.len()..16 {
            if i == 8 { let _ = write!(html, " "); }
            let _ = write!(html, "   ");
        }
        let _ = write!(html, "</span>");
        
        // ASCII
        let _ = write!(html, "<span class=\"hex-ascii\">");
        for b in chunk {
            let c = if *b >= 0x20 && *b < 0x7F { *b as char } else { '.' };
            let _ = write!(html, "{}", escape_html(&c.to_string()));
        }
        let _ = writeln!(html, "</span></div>");
    }
    let _ = writeln!(html, "</div></div>");

    // Parse and show metadata summary
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    if let Ok(metadata) = registry.parse(&mut reader) {
        let _ = writeln!(html, "<div class=\"section\"><h3>Metadata Summary ({} tags)</h3>", metadata.exif.len());
        let _ = writeln!(html, "<table class=\"meta-table\">");
        let _ = writeln!(html, "<tr><th>Tag</th><th>Value</th></tr>");
        
        // Show important tags first
        let important = ["Make", "Model", "DateTimeOriginal", "ISO", "ExposureTime", "FNumber", 
                        "FocalLength", "ImageWidth", "ImageHeight", "Software"];
        for tag in &important {
            if let Some(v) = metadata.exif.get(*tag) {
                let _ = writeln!(html, "<tr><td>{}</td><td>{}</td></tr>", tag, escape_html(&v.to_string()));
            }
        }
        let _ = writeln!(html, "</table></div>");
    }

    let _ = writeln!(html, "</div>");
    Ok(())
}

/// Detect file format from magic bytes.
fn detect_format(data: &[u8]) -> &'static str {
    if data.len() < 4 { return "Unknown"; }
    
    match &data[0..4] {
        [0xFF, 0xD8, 0xFF, _] => "JPEG",
        [0x89, 0x50, 0x4E, 0x47] => "PNG",
        [0x49, 0x49, 0x2A, 0x00] => "TIFF (Little Endian)",
        [0x4D, 0x4D, 0x00, 0x2A] => "TIFF (Big Endian)",
        [0x52, 0x49, 0x46, 0x46] => {
            if data.len() >= 12 && &data[8..12] == b"WEBP" {
                "WebP"
            } else if data.len() >= 12 && &data[8..12] == b"AVI " {
                "AVI"
            } else {
                "RIFF"
            }
        }
        _ => {
            // Check for ISOBMFF (ftyp at offset 4)
            if data.len() >= 8 && &data[4..8] == b"ftyp" {
                if data.len() >= 12 {
                    let brand = &data[8..12];
                    if brand == b"heic" || brand == b"heix" || brand == b"mif1" {
                        "HEIC/HEIF"
                    } else if brand == b"avif" {
                        "AVIF"
                    } else if brand == b"mp41" || brand == b"mp42" || brand == b"isom" {
                        "MP4"
                    } else if brand == b"qt  " {
                        "QuickTime MOV"
                    } else {
                        "ISOBMFF"
                    }
                } else {
                    "ISOBMFF"
                }
            } else if data.len() >= 4 && &data[0..4] == b"fLaC" {
                "FLAC"
            } else if data.len() >= 3 && &data[0..3] == b"ID3" {
                "MP3 (ID3)"
            } else if data.len() >= 4 && &data[0..4] == [0x76, 0x2F, 0x31, 0x01] {
                "OpenEXR"
            } else if data.len() >= 10 && &data[0..10] == b"#?RADIANCE" {
                "Radiance HDR"
            } else {
                "Unknown"
            }
        }
    }
}

/// Show structure markers for the file format.
fn show_structure_markers(data: &[u8], format: &str, html: &mut String) {
    use std::fmt::Write;
    
    match format {
        "JPEG" => {
            let _ = write!(html, "<p>Markers: ");
            let mut i = 0;
            while i < data.len() - 1 {
                if data[i] == 0xFF {
                    let marker = data[i + 1];
                    let name = jpeg_marker_name(marker);
                    let class = if name.contains("EXIF") { "marker marker-exif" }
                               else if name.contains("XMP") { "marker marker-xmp" }
                               else if name.contains("ICC") { "marker marker-icc" }
                               else { "marker" };
                    let _ = write!(html, "<span class=\"{}\">{} ({:02X})</span> ", class, name, marker);
                    
                    // Skip marker data
                    if marker >= 0xE0 && marker <= 0xEF || marker == 0xFE || marker == 0xDB || marker == 0xC4 {
                        if i + 3 < data.len() {
                            let len = ((data[i + 2] as usize) << 8) | (data[i + 3] as usize);
                            i += 2 + len;
                            continue;
                        }
                    }
                    if marker == 0xD8 || marker == 0xD9 {
                        i += 2;
                        continue;
                    }
                }
                i += 1;
            }
            let _ = writeln!(html, "</p>");
        }
        "PNG" => {
            let _ = write!(html, "<p>Chunks: ");
            let mut i = 8; // Skip signature
            while i + 8 <= data.len() {
                let len = u32::from_be_bytes([data[i], data[i+1], data[i+2], data[i+3]]) as usize;
                let chunk_type = std::str::from_utf8(&data[i+4..i+8]).unwrap_or("????");
                let class = if chunk_type == "eXIf" || chunk_type == "tEXt" || chunk_type == "iTXt" { "marker marker-exif" }
                           else if chunk_type == "iCCP" { "marker marker-icc" }
                           else { "marker" };
                let _ = write!(html, "<span class=\"{}\">{} ({}B)</span> ", class, chunk_type, len);
                i += 12 + len; // length(4) + type(4) + data + crc(4)
            }
            let _ = writeln!(html, "</p>");
        }
        _ => {
            let _ = writeln!(html, "<p>Structure visualization not available for this format.</p>");
        }
    }
}

/// Get JPEG marker name.
fn jpeg_marker_name(marker: u8) -> &'static str {
    match marker {
        0xD8 => "SOI",
        0xD9 => "EOI",
        0xE0 => "APP0/JFIF",
        0xE1 => "APP1/EXIF",
        0xE2 => "APP2/ICC",
        0xED => "APP13/IPTC",
        0xEE => "APP14",
        0xDB => "DQT",
        0xC0 => "SOF0",
        0xC2 => "SOF2",
        0xC4 => "DHT",
        0xDA => "SOS",
        0xFE => "COM",
        _ => "???",
    }
}

/// Escape HTML special characters.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

// =============================================================================
// Validate Metadata
// =============================================================================

/// Validate metadata in files.
fn validate_metadata(args: &Args, registry: &FormatRegistry) -> Result<()> {
    let files = expand_paths(
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
    registry: &FormatRegistry
) -> Result<Vec<(String, String, String)>> {
    let file = File::open(path)
        .with_context(|| format!("Cannot open: {}", path.display()))?;
    let mut reader = BufReader::new(file);
    let metadata = registry.parse(&mut reader)
        .with_context(|| format!("Cannot parse: {}", path.display()))?;
    
    let mut issues = Vec::new();
    
    // GPS latitude (-90 to 90)
    if let Some(lat) = metadata.exif.get("GPSLatitude").and_then(|v| v.as_f64()) {
        if !(-90.0..=90.0).contains(&lat) {
            issues.push((
                "GPSLatitude".into(),
                "error".into(),
                format!("Invalid latitude {}: must be -90 to 90", lat),
            ));
        }
    }
    
    // GPS longitude (-180 to 180)
    if let Some(lon) = metadata.exif.get("GPSLongitude").and_then(|v| v.as_f64()) {
        if !(-180.0..=180.0).contains(&lon) {
            issues.push((
                "GPSLongitude".into(),
                "error".into(),
                format!("Invalid longitude {}: must be -180 to 180", lon),
            ));
        }
    }
    
    // Orientation (1-8)
    if let Some(orient) = metadata.exif.get("Orientation").and_then(|v| v.as_u32()) {
        if !(1..=8).contains(&orient) {
            issues.push((
                "Orientation".into(),
                "error".into(),
                format!("Invalid orientation {}: must be 1-8", orient),
            ));
        }
    }
    
    // ISO (reasonable range)
    if let Some(iso) = metadata.exif.get("ISO").and_then(|v| v.as_u32()) {
        if iso == 0 || iso > 10_000_000 {
            issues.push((
                "ISO".into(),
                "warning".into(),
                format!("Suspicious ISO value {}", iso),
            ));
        }
    }
    
    // Image dimensions > 0
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
    
    // DateTime format check
    for tag in &["DateTime", "DateTimeOriginal", "DateTimeDigitized", "CreateDate", "ModifyDate"] {
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
    
    // ExposureTime > 0
    if let Some(exp) = metadata.exif.get("ExposureTime").and_then(|v| v.as_f64()) {
        if exp <= 0.0 {
            issues.push((
                "ExposureTime".into(),
                "error".into(),
                format!("Invalid exposure time: {}", exp),
            ));
        }
    }
    
    // FNumber > 0
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

/// Check if datetime string is valid EXIF format.
fn is_valid_datetime(s: &str) -> bool {
    let s = s.trim();
    if s.len() < 10 {
        return false;
    }
    // EXIF format: YYYY:MM:DD HH:MM:SS or YYYY-MM-DD HH:MM:SS
    let parts: Vec<&str> = s.split(|c| c == ' ' || c == 'T').collect();
    if parts.is_empty() {
        return false;
    }
    let date = parts[0];
    let date_parts: Vec<&str> = date.split(|c| c == ':' || c == '-').collect();
    if date_parts.len() != 3 {
        return false;
    }
    // Check year, month, day are numbers
    if date_parts[0].parse::<u16>().is_err() { return false; }
    if let Ok(m) = date_parts[1].parse::<u8>() {
        if !(1..=12).contains(&m) { return false; }
    } else { return false; }
    if let Ok(d) = date_parts[2].parse::<u8>() {
        if !(1..=31).contains(&d) { return false; }
    } else { return false; }
    true
}

//! Argument parsing and HELP text.
//!
//! # Why
//!
//! Manual parsing (not clap) for ExifTool-compatible flexibility. Single source
//! for all CLI options.
//!
//! # Where used
//!
//! main.rs run() — parse_args() then dispatch to commands.

use anyhow::Result;
use std::path::PathBuf;

/// Parsed command-line arguments.
#[derive(Debug, Default)]
pub struct Args {
    pub files: Vec<PathBuf>,
    pub format: String,
    pub get_tags: Vec<String>,
    pub tags: Vec<(String, String)>,
    pub shift: Option<i64>,
    pub geotag: Option<PathBuf>,
    pub icc_profile: Option<PathBuf>,
    pub output: Option<PathBuf>,
    pub write_file: Option<PathBuf>,
    pub inplace: bool,
    pub thumbnail: bool,
    pub preview: bool,
    pub recursive: bool,
    pub extensions: Vec<String>,
    pub exclude: Vec<String>,
    pub newer: Option<std::time::SystemTime>,
    pub older: Option<std::time::SystemTime>,
    pub minsize: Option<u64>,
    pub maxsize: Option<u64>,
    pub composite: bool,
    pub charset: String,
    pub all: bool,
    pub json_import: Option<PathBuf>,
    pub csv_import: Option<PathBuf>,
    pub tags_from_file: Option<PathBuf>,
    pub copy_tags: Vec<String>,
    pub rename: Option<String>,
    pub delete: bool,
    pub validate: bool,
    pub if_condition: Option<String>,
    pub short: bool,
    pub numeric_group: bool,
    pub group: bool,
    pub tabular: bool,
    pub overwrite_original: bool,
    pub html_dump: bool,
    pub duplicates: bool,
    pub dup_by: String,
    pub verbose: u8,
    /// Locale for tag value output (e.g. "de", "fr"). Passed to interpret_value when locales are supported.
    pub lang: Option<String>,
}

/// Parse date string to SystemTime.
/// Supports: YYYY-MM-DD, YYYY-MM-DD HH:MM:SS, YYYY-MM-DDTHH:MM:SS
pub fn parse_date(s: &str) -> Option<std::time::SystemTime> {
    use std::time::{Duration, UNIX_EPOCH};

    let s = s.trim();
    let (date_part, time_part) = if s.contains('T') {
        let parts: Vec<&str> = s.split('T').collect();
        (parts.get(0).copied()?, parts.get(1).map(|s| *s))
    } else if s.contains(' ') {
        let parts: Vec<&str> = s.splitn(2, ' ').collect();
        (parts.get(0).copied()?, parts.get(1).map(|s| *s))
    } else {
        (s, None)
    };

    let date_parts: Vec<&str> = date_part.split('-').collect();
    if date_parts.len() != 3 {
        return None;
    }

    let year: i32 = date_parts[0].parse().ok()?;
    let month: u32 = date_parts[1].parse().ok()?;
    let day: u32 = date_parts[2].parse().ok()?;

    let (hour, minute, second) = if let Some(t) = time_part {
        let t = t.split('.').next().unwrap_or(t);
        let time_parts: Vec<&str> = t.split(':').collect();
        (
            time_parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0u32),
            time_parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0u32),
            time_parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0u32),
        )
    } else {
        (0, 0, 0)
    };

    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }

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

    if secs >= 0 {
        Some(UNIX_EPOCH + Duration::from_secs(secs as u64))
    } else {
        None
    }
}

/// Parse time shift string to seconds.
/// Formats: "+2:30" (hours:minutes), "-30" (minutes), "+1" (minutes)
pub fn parse_shift(s: &str) -> Option<i64> {
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

/// Parse size string (e.g., "100", "1K", "10M", "1G") to bytes.
pub fn parse_size(s: &str) -> Option<u64> {
    let s = s.trim().to_uppercase();
    if s.is_empty() {
        return None;
    }

    let (num_str, multiplier) = if s.ends_with('K') {
        (&s[..s.len() - 1], 1024u64)
    } else if s.ends_with("KB") {
        (&s[..s.len() - 2], 1024u64)
    } else if s.ends_with('M') {
        (&s[..s.len() - 1], 1024u64 * 1024)
    } else if s.ends_with("MB") {
        (&s[..s.len() - 2], 1024u64 * 1024)
    } else if s.ends_with('G') {
        (&s[..s.len() - 1], 1024u64 * 1024 * 1024)
    } else if s.ends_with("GB") {
        (&s[..s.len() - 2], 1024u64 * 1024 * 1024)
    } else {
        (s.as_str(), 1u64)
    };

    num_str.trim().parse::<u64>().ok().map(|n| n * multiplier)
}

pub const HELP: &str = r#"
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
    -s, --short          Print values only (no tag names)
    -G, --numeric        Print group numbers (0:tag or 1:Group:tag)
    --group              Print group name prefix (EXIF:Make, IPTC:Keywords)
    --tabular            Tab-delimited output (machine-parseable)
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
    --overwrite_original No backup: overwrite original (no _original file)
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
    -lang [LANG]         Set output language (e.g. de, fr). Tag names stay English; affects value descriptions.
    -if <COND>           Process only files where CONDITION is true
                         Ops: eq, ne, gt, lt, ge, le, contains, startswith, endswith
                         Examples: -if "Make eq Canon", -if "ISO gt 800"
    -htmlDump            Show file structure with hex preview (to HTML)
    -duplicates [BY]     Find duplicate files (BY: hash, content, datetime, metadata)
    --verbose [N]        Verbose (N=1..3, default 1): file I/O, format detection
    -h, --help, /?       Show this help
    -v, --version        Show version

FORMATS (read):  JPEG PNG TIFF DNG CR2 CR3 NEF ARW ORF RW2 PEF RAF WebP HEIC AVIF EXR HDR GIF WAV JXL MP3 FLAC
FORMATS (write): JPEG PNG TIFF EXR HDR WebP HEIC GIF WAV JXL MP3 FLAC PNM CR2 ARW ORF NEF RAF RW2 PEF SRW...

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

/// Parse command-line arguments.
pub fn parse_args(args: &[String]) -> Result<Args> {
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
            "-s" | "--short" => parsed.short = true,
            "-G" | "--numeric" => parsed.numeric_group = true,
            "--group" | "-group" => parsed.group = true,
            "--tabular" | "-tabular" => parsed.tabular = true,
            "--overwrite_original" => parsed.overwrite_original = true,
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
                if let Some(next) = args.get(i + 1) {
                    if !next.starts_with('-')
                        && ["hash", "content", "datetime", "metadata"].contains(&next.as_str())
                    {
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
                    parsed.extensions = exts
                        .split(',')
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
                        anyhow::bail!(
                            "Invalid date format: {}. Use YYYY-MM-DD or YYYY-MM-DD HH:MM:SS",
                            date_str
                        );
                    }
                }
            }
            "--older" => {
                i += 1;
                if let Some(date_str) = args.get(i) {
                    parsed.older = parse_date(date_str);
                    if parsed.older.is_none() {
                        anyhow::bail!(
                            "Invalid date format: {}. Use YYYY-MM-DD or YYYY-MM-DD HH:MM:SS",
                            date_str
                        );
                    }
                }
            }
            "--minsize" => {
                i += 1;
                if let Some(size_str) = args.get(i) {
                    parsed.minsize = parse_size(size_str);
                    if parsed.minsize.is_none() {
                        anyhow::bail!(
                            "Invalid size format: {}. Use bytes or K/M/G suffix (e.g., 100, 1K, 10M)",
                            size_str
                        );
                    }
                }
            }
            "--maxsize" => {
                i += 1;
                if let Some(size_str) = args.get(i) {
                    parsed.maxsize = parse_size(size_str);
                    if parsed.maxsize.is_none() {
                        anyhow::bail!(
                            "Invalid size format: {}. Use bytes or K/M/G suffix (e.g., 100, 1K, 10M)",
                            size_str
                        );
                    }
                }
            }
            "--verbose" | "-verbose" => {
                i += 1;
                let n = args.get(i).and_then(|s| s.parse::<u8>().ok());
                parsed.verbose = n.unwrap_or(1).min(3);
                if n.is_some() {
                    i += 1;
                }
            }
            "-lang" | "--lang" => {
                i += 1;
                if let Some(lang) = args.get(i) {
                    if !lang.starts_with('-') && lang.len() <= 10 {
                        parsed.lang = Some(lang.to_lowercase());
                    }
                }
            }
            a if a.starts_with("-v") && a.len() > 2 => {
                if let Ok(n) = a[2..].parse::<u8>() {
                    parsed.verbose = n.min(3);
                }
            }
            "-c" | "--composite" => parsed.composite = true,
            "--charset" => {
                i += 1;
                if let Some(enc) = args.get(i) {
                    let enc_lower = enc.to_lowercase();
                    if !matches!(
                        enc_lower.as_str(),
                        "utf8" | "utf-8" | "latin1" | "iso-8859-1" | "ascii"
                    ) {
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

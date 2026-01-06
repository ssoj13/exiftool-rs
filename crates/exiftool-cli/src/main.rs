//! exiftool-rs CLI - fast metadata reader/writer for images
//!
//! Supports: JPEG, PNG, TIFF, DNG, CR2, CR3, NEF, ARW, ORF, RW2, PEF, RAF, WebP, HEIC, AVIF, EXR, HDR

mod xml_output;

use anyhow::{Context, Result};
use exiftool_attrs::AttrValue;
use exiftool_formats::{
    add_composite_tags, build_exif_bytes, FormatRegistry, JpegWriter, Metadata, 
    PngWriter, TiffWriter, HdrWriter, ExrWriter,
};
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

THUMBNAIL/PREVIEW:
    exif -T photo.jpg                           # extract thumbnail to photo_thumb.jpg
    exif -T -o thumb.jpg photo.jpg              # extract to specific file
    exif -P photo.cr2                           # extract RAW preview to photo_preview.jpg
    exif -P -o preview.jpg photo.raf            # extract preview to specific file

OPTIONS:
    -g, --get <PATTERN>  Get tag(s) matching pattern (* and ? wildcards)
    -f, --format <FMT>   Output: text (default), json, csv, xml
    -X, --xml            XML output (shortcut for -f xml)
    -o, --output <FILE>  Save metadata/thumbnail to file
    -t, --tag <T=V>      Set tag (repeatable): -t Tag=Value
    -w, --write <FILE>   Output image file (for write mode)
    -p, --inplace        Modify original file in-place
    -T, --thumbnail      Extract embedded thumbnail
    -P, --preview        Extract embedded preview (larger, from RAW files)
    -r, --recursive      Recursively scan directories
    -e, --ext <EXTS>     Filter by extensions (comma-separated): jpg,png,cr2
    -c, --composite      Add composite/calculated tags (ImageSize, Megapixels, etc.)
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
    let files = expand_paths(&parsed.files, parsed.recursive, &parsed.extensions);
    
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
    output: Option<PathBuf>,          // -o metadata output
    write_file: Option<PathBuf>,      // -w image output
    inplace: bool,                    // -p modify in-place
    thumbnail: bool,                  // -T extract thumbnail
    preview: bool,                    // -P extract preview (RAW)
    recursive: bool,                  // -r recursive directory scan
    extensions: Vec<String>,          // -e extension filter
    composite: bool,                  // -c add composite tags
    all: bool,
}

/// Expand paths: if recursive, walk directories; filter by extensions.
fn expand_paths(paths: &[PathBuf], recursive: bool, extensions: &[String]) -> Vec<PathBuf> {
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
                    if p.is_file() {
                        if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                            let ext_lower = ext.to_lowercase();
                            // Check against filter or defaults
                            let matches = if extensions.is_empty() {
                                default_exts.contains(&ext_lower.as_str())
                            } else {
                                extensions.iter().any(|e| e == &ext_lower)
                            };
                            if matches {
                                result.push(p.to_path_buf());
                            }
                        }
                    }
                }
            } else {
                eprintln!("Warning: {} is a directory. Use -r for recursive scan.", path.display());
            }
        } else if path.is_file() {
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
    let mut parsed = Args { format: "text".into(), ..Default::default() };
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
            "-c" | "--composite" => parsed.composite = true,
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

fn print_metadata(path: &Path, m: &Metadata, args: &Args) {
    match args.format.as_str() {
        "json" => print_json(path, m, &args.get_tags),
        "csv" => print_csv(path, m, &args.get_tags),
        "xml" => xml_output::print_xml(path, m, &args.get_tags),
        _ => print_text(path, m, args.all, &args.get_tags),
    }
}

fn format_metadata(path: &Path, m: &Metadata, args: &Args, out: &mut String) {
    use std::fmt::Write;
    let filter = &args.get_tags;
    
    match args.format.as_str() {
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

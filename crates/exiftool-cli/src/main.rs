//! exiftool-rs CLI - fast metadata reader/writer for images
//!
//! Supports: JPEG, PNG, TIFF, DNG, CR2, CR3, NEF, ARW, ORF, RW2, PEF, RAF, WebP, HEIC, AVIF, EXR, HDR

use anyhow::{Context, Result};
use exiftool_attrs::AttrValue;
use exiftool_formats::{
    build_exif_bytes, FormatRegistry, JpegWriter, Metadata, PngWriter, TiffWriter,
    HdrWriter, ExrWriter,
};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP: &str = r#"
exif - fast image metadata reader/writer

USAGE:
    exif [OPTIONS] <FILES>...

READ:
    exif photo.jpg                    # show all metadata
    exif -g Model photo.jpg           # get single tag value
    exif -g Model -g Make *.jpg       # get multiple tags
    exif -f json *.jpg                # JSON output for batch
    exif -f csv photos/*.png          # CSV for spreadsheet
    exif image.{heic,cr3,nef,arw,orf,rw2,pef,raf,webp}  # RAW formats

OUTPUT:
    exif -f json photo.jpg -o meta.json   # save metadata to file
    exif -f csv *.jpg -o report.csv       # batch export to CSV

WRITE:
    exif -t Artist="John Doe" a.jpg             # set single tag
    exif -t Make=Canon -t Model=EOS a.jpg       # set multiple tags
    exif -w out.jpg -t Copyright="(C) Me" a.jpg # write to new file
    exif -p -t Software=exif a.jpg              # modify in-place (!)

OPTIONS:
    -g, --get <TAG>      Get specific tag(s) only (repeatable)
    -f, --format <FMT>   Output: text (default), json, csv
    -o, --output <FILE>  Save metadata to file (for read mode)
    -t, --tag <T=V>      Set tag (repeatable): -t Tag=Value
    -w, --write <FILE>   Output image file (for write mode)
    -p, --inplace        Modify original file in-place
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

    // Read mode
    if parsed.files.is_empty() {
        anyhow::bail!("No input files specified.\n\nUsage: exif [OPTIONS] <FILES>...\n       exif --help for more options");
    }

    // Collect output for potential file write
    let mut output_buf = String::new();
    let write_to_file = parsed.output.is_some();

    for path in &parsed.files {
        let file = File::open(path)
            .with_context(|| format!("Cannot open: {}", path.display()))?;
        let mut reader = BufReader::new(file);

        match registry.parse(&mut reader) {
            Ok(metadata) => {
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
    all: bool,
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
            "-p" | "--inplace" => parsed.inplace = true,
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

fn print_metadata(path: &Path, m: &Metadata, args: &Args) {
    match args.format.as_str() {
        "json" => print_json(path, m, &args.get_tags),
        "csv" => print_csv(path, m, &args.get_tags),
        _ => print_text(path, m, args.all, &args.get_tags),
    }
}

fn format_metadata(path: &Path, m: &Metadata, args: &Args, out: &mut String) {
    use std::fmt::Write;
    let filter = &args.get_tags;
    
    match args.format.as_str() {
        "json" => {
            let mut map = serde_json::Map::new();
            
            if filter.len() == 1 {
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
                if filter.is_empty() || filter.iter().any(|f| f.eq_ignore_ascii_case(k)) {
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
                filter.clone()
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
            if filter.len() == 1 && args.files.len() == 1 {
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
                .filter(|(k, _)| filter.is_empty() || filter.iter().any(|f| f.eq_ignore_ascii_case(k)))
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
    if filter.len() == 1 {
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
        .filter(|(k, _)| filter.is_empty() || filter.iter().any(|f| f.eq_ignore_ascii_case(k)))
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
        println!();
    }
}

fn print_json(path: &Path, m: &Metadata, filter: &[String]) {
    let mut map = serde_json::Map::new();
    
    // Single tag: just the value
    if filter.len() == 1 {
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
    }
    
    for (k, v) in m.exif.iter() {
        if filter.is_empty() || filter.iter().any(|f| f.eq_ignore_ascii_case(k)) {
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
        filter.to_vec()
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

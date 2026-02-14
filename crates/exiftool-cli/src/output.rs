//! Metadata output formatting (text, JSON, CSV, XML, HTML).
//!
//! # Why
//!
//! Centralizes display logic and ExifTool-compatible output formats. PrintConv
//! (interpretation) applied via format_value_for_display.
//!
//! # Where used
//!
//! main.rs read path — print_metadata() for stdout, format_metadata() for -o file.

use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use exiftool_attrs::AttrValue;
use exiftool_formats::{add_composite_tags, FormatRegistry, Metadata};
use exiftool_tags::interp;

use crate::args::Args;
use crate::condition;
use crate::filters;

/// Format value for display with PrintConv when available (ExifTool-compatible).
/// Prefer Metadata::get_display when available (handles ExposureTime, FNumber, FocalLength, GPS).
pub fn format_value_for_display(tag_name: &str, value: &AttrValue) -> String {
    let i64_val = value.as_i64();
    if let Some(v) = i64_val {
        let base_name = tag_name.rsplit(':').next().unwrap_or(tag_name);
        if let Some(interpreted) = interp::interpret_value(base_name, v) {
            return interpreted;
        }
    }
    value.to_string()
}

/// Format value using Metadata's get_display (ExposureTime→"1/125 sec", FNumber→"f/2.8", etc).
fn format_with_display(m: &Metadata, key: &str, value: &AttrValue) -> String {
    m.get_display(key).unwrap_or_else(|| format_value_for_display(key, value))
}

/// Print metadata to stdout based on format.
pub fn print_metadata(path: &Path, m: &Metadata, args: &Args) {
    match args.format.as_str() {
        "json" => print_json(path, m, &args.get_tags),
        "csv" => print_csv(path, m, &args.get_tags),
        "xml" => crate::xml_output::print_xml(path, m, &args.get_tags),
        "html" => crate::html_output::print_html(path, m, &args.get_tags),
        _ => print_text(path, m, args, &args.get_tags),
    }
}

/// Format metadata into string buffer (for -o file output).
pub fn format_metadata(path: &Path, m: &Metadata, args: &Args, out: &mut String) {
    use std::fmt::Write;
    let filter = &args.get_tags;

    match args.format.as_str() {
        "html" => {
            crate::html_output::format_html(path, m, filter, out);
        }
        "xml" => {
            crate::xml_output::format_xml(path, m, filter, out);
        }
        "json" => {
            let mut map = serde_json::Map::new();

            if filters::is_simple_filter(filter) {
                if let Some(v) = m.exif.get(&filter[0]) {
                    let _ = writeln!(out, "{}", serde_json::to_string(&val_json(v)).unwrap_or_else(|_| "null".into()));
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
                if filters::tag_matches(k, filter) {
                    map.insert(k.clone(), val_json(v));
                }
            }
            let _ = writeln!(
                out,
                "{}",
                serde_json::to_string_pretty(&serde_json::Value::Array(vec![
                    serde_json::Value::Object(map)
                ]))
                .unwrap_or_else(|_| "[]".into())
            );
        }
        "csv" => {
            let keys: Vec<_> = if filter.is_empty() {
                let mut k: Vec<_> = m.exif.iter().map(|(k, _)| k.clone()).collect();
                k.sort();
                k.insert(0, "SourceFile".into());
                k
            } else {
                filters::expand_filters(filter, m)
            };
            let _ = writeln!(out, "{}", keys.join(","));
            let vals: Vec<_> = keys
                .iter()
                .map(|k| {
                    if k == "SourceFile" {
                        format!("\"{}\"", path.display())
                    } else {
                        m.exif
                            .get(k)
                            .map(|v| format!("\"{}\"", v))
                            .unwrap_or_default()
                    }
                })
                .collect();
            let _ = writeln!(out, "{}", vals.join(","));
        }
        _ => {
            if filters::is_simple_filter(filter) && args.files.len() == 1 {
                if let Some(v) = m.exif.get(&filter[0]) {
                    let _ = writeln!(out, "{}", format_with_display(m, &filter[0], v));
                }
                return;
            }

            let sep = if args.tabular { "\t" } else { " " };
            let col_width = if args.tabular { 0 } else { 28 };

            if args.short {
                let mut entries: Vec<_> = m
                    .exif
                    .iter()
                    .filter(|(k, _)| filters::tag_matches(k, filter))
                    .collect();
                entries.sort_by(|a, b| a.0.cmp(b.0));
                for (k, v) in entries {
                    let _ = writeln!(out, "{}", format_with_display(m, k, v));
                }
                return;
            }

            if filter.is_empty() {
                let _ = writeln!(out, "── {} ──", path.display());
                let _ = writeln!(out, "{:28} {}", "Format", m.format);
            } else if args.files.len() > 1 {
                let _ = writeln!(out, "── {} ──", path.display());
            }

            let mut entries: Vec<_> = m
                .exif
                .iter()
                .filter(|(k, _)| filters::tag_matches(k, filter))
                .collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));

            for (k, v) in entries {
                let disp = format_with_display(m, k, v);
                let tag_display = if args.numeric_group {
                    if k.contains(':') {
                        format!("1:{}", k)
                    } else {
                        format!("0:{}", k)
                    }
                } else if args.group && !k.contains(':') {
                    format!("EXIF:{}", k)
                } else {
                    k.clone()
                };
                if args.tabular {
                    let _ = writeln!(out, "{}{}{}", tag_display, sep, disp);
                } else {
                    let _ =
                        writeln!(out, "{:width$}{}{}", tag_display, sep, disp, width = col_width);
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

fn print_text(path: &Path, m: &Metadata, args: &Args, filter: &[String]) {
    let sep = if args.tabular { "\t" } else { " " };
    let col_width = if args.tabular { 0 } else { 28 };

    if filters::is_simple_filter(filter) {
        if let Some(v) = m.exif.get(&filter[0]) {
            let disp = format_with_display(m, &filter[0], v);
            println!("{}", disp);
        }
        return;
    }

    if args.short {
        let mut entries: Vec<_> = m
            .exif
            .iter()
            .filter(|(k, _)| filters::tag_matches(k, filter))
            .collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        for (k, v) in entries {
            println!("{}", format_with_display(m, k, v));
        }
        return;
    }

    if filter.is_empty() {
        println!("── {} ──", path.display());
        println!("{:28} {}", "Format", m.format);
    }

    let mut entries: Vec<_> = m
        .exif
        .iter()
        .filter(|(k, _)| filters::tag_matches(k, filter))
        .collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    for (k, v) in entries {
        let disp = format_with_display(m, k, v);
        let tag_display = if args.numeric_group {
            if k.contains(':') {
                format!("1:{}", k)
            } else {
                format!("0:{}", k)
            }
        } else if args.group && !k.contains(':') {
            format!("EXIF:{}", k)
        } else {
            k.clone()
        };
        if args.tabular {
            println!("{}{}{}", tag_display, sep, disp);
        } else {
            println!("{:width$}{}{}", tag_display, sep, disp, width = col_width);
        }
    }

    if filter.is_empty() {
        if let Some(ref xmp) = m.xmp {
            println!("{:28} {} bytes", "XMP", xmp.len());
        }
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
                println!(
                    "  Page {:2}: {}x{} {}bpp {}",
                    page.index, page.width, page.height, page.bits_per_sample, desc
                );
            }
        }
        if let Some(ref thumb) = m.thumbnail {
            println!("{:28} {} bytes", "Thumbnail", thumb.len());
        }
        println!();
    }
}

fn print_json(path: &Path, m: &Metadata, filter: &[String]) {
    let mut map = serde_json::Map::new();

    if filters::is_simple_filter(filter) {
        if let Some(v) = m.exif.get(&filter[0]) {
            println!("{}", serde_json::to_string(&val_json(v)).unwrap_or_else(|_| "null".into()));
        } else {
            println!("null");
        }
        return;
    }

    if filter.is_empty() {
        map.insert("SourceFile".into(), path.display().to_string().into());
        map.insert("Format".into(), m.format.into());
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
        if let Some(ref thumb) = m.thumbnail {
            map.insert("ThumbnailSize".into(), (thumb.len() as i64).into());
        }
    }

    for (k, v) in m.exif.iter() {
        if filters::tag_matches(k, filter) {
            map.insert(k.clone(), val_json(v));
        }
    }

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::Value::Array(vec![
            serde_json::Value::Object(map)
        ]))
        .unwrap_or_else(|_| "[]".into())
    );
}

fn print_csv(path: &Path, m: &Metadata, filter: &[String]) {
    let keys: Vec<_> = if filter.is_empty() {
        let mut k: Vec<_> = m.exif.iter().map(|(k, _)| k.clone()).collect();
        k.sort();
        k.insert(0, "SourceFile".into());
        k
    } else {
        filters::expand_filters(filter, m)
    };

    println!("{}", keys.join(","));

    let vals: Vec<_> = keys
        .iter()
        .map(|k| {
            if k == "SourceFile" {
                format!("\"{}\"", path.display())
            } else {
                m.exif
                    .get(k)
                    .map(|v| format!("\"{}\"", v))
                    .unwrap_or_default()
            }
        })
        .collect();
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

/// Output CSV with unified headers across all files.
/// Collects all metadata first to build superset of columns.
pub fn output_csv_unified(
    files: &[PathBuf],
    registry: &FormatRegistry,
    args: &Args,
) -> Result<()> {
    use std::fmt::Write;

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
                if let Some(ref cond) = args.if_condition {
                    if !condition::matches_condition(&metadata, cond) {
                        continue;
                    }
                }
                for (tag, _) in metadata.exif.iter() {
                    if filters::tag_matches(tag, &args.get_tags) {
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

    let columns: Vec<String> = {
        let mut cols = vec!["SourceFile".to_string()];
        cols.extend(all_tags.into_iter());
        cols
    };

    let mut output_buf = String::new();
    let write_to_file = args.output.is_some();

    let header = columns.join(",");
    if write_to_file {
        writeln!(&mut output_buf, "{}", header).ok();
    } else {
        println!("{}", header);
    }

    for (path, metadata) in &all_data {
        let row: Vec<String> = columns
            .iter()
            .map(|col| {
                if col == "SourceFile" {
                    format!("\"{}\"", path.display())
                } else {
                    metadata.exif.get(col).map(|v| {
                        let s = v.to_string();
                        if s.contains(',') || s.contains('"') || s.contains('\n') {
                            format!("\"{}\"", s.replace('"', "\"\""))
                        } else {
                            format!("\"{}\"", s)
                        }
                    }).unwrap_or_default()
                }
            })
            .collect();

        if write_to_file {
            writeln!(&mut output_buf, "{}", row.join(",")).ok();
        } else {
            println!("{}", row.join(","));
        }
    }

    if let Some(ref output_path) = args.output {
        std::fs::write(output_path, &output_buf)
            .with_context(|| format!("Cannot write: {}", output_path.display()))?;
        eprintln!("Wrote: {}", output_path.display());
    }

    Ok(())
}

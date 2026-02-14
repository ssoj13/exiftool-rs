//! Rename files using metadata template (--rename).

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use exiftool_formats::{FormatRegistry, Metadata};

use crate::paths;

/// Rename files using metadata template.
pub fn rename_files(args: &crate::args::Args, registry: &FormatRegistry) -> Result<()> {
    let template = args.rename.as_ref().unwrap();

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

fn rename_single_file(path: &Path, template: &str, registry: &FormatRegistry) -> Result<PathBuf> {
    let file = File::open(path)
        .with_context(|| format!("Cannot open: {}", path.display()))?;
    let mut reader = BufReader::new(file);
    let metadata = registry
        .parse(&mut reader)
        .with_context(|| format!("Cannot parse: {}", path.display()))?;

    let ext = path
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_default();

    let datetime = metadata
        .exif
        .get("DateTimeOriginal")
        .or_else(|| metadata.exif.get("CreateDate"))
        .or_else(|| metadata.exif.get("ModifyDate"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let new_name = expand_rename_template(
        template,
        &metadata,
        &ext,
        datetime.as_deref(),
    )?;

    let parent = path.parent().unwrap_or(Path::new("."));
    let mut new_path = parent.join(&new_name);

    if new_name.contains('/') || new_name.contains('\\') {
        if let Some(new_parent) = new_path.parent() {
            std::fs::create_dir_all(new_parent)?;
        }
    }

    if new_path.exists() && new_path != *path {
        let stem = new_path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let new_ext = new_path
            .extension()
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

    if new_path != *path {
        std::fs::rename(path, &new_path)?;
    }

    Ok(new_path)
}

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
                let mut tag_name = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_alphanumeric() || nc == '_' || nc == '-' || nc == ':' {
                        if let Some(nc) = chars.next() {
                            tag_name.push(nc);
                        }
                    } else {
                        break;
                    }
                }
                if tag_name.is_empty() {
                    result.push('$');
                } else {
                    let value = metadata
                        .exif
                        .get(&tag_name)
                        .map(|v| sanitize_filename(&v.to_string()))
                        .unwrap_or_default();
                    result.push_str(&value);
                }
            }
            '%' => {
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

    if !result.contains('.') && !ext.is_empty() {
        result.push('.');
        result.push_str(ext);
    }

    Ok(result)
}

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

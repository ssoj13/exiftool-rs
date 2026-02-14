//! Path expansion and file filtering.
//!
//! # Why
//!
//! `exif -r photos/` must walk directories; `-e jpg,png` filters by extension;
//! `-x "*thumb*"` excludes paths; `--newer`, `--minsize` etc. filter by mtime/size.
//!
//! # Where used
//!
//! All commands that take file lists: read, write, thumbnail, delete, rename, etc.

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::filters::glob_match;

/// Check if path matches any exclude pattern (file name or full path).
pub fn matches_exclude(path: &Path, exclude: &[String]) -> bool {
    if exclude.is_empty() {
        return false;
    }

    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let path_str = path.to_string_lossy();

    for pattern in exclude {
        if glob_match(pattern, name) || glob_match(pattern, &path_str) {
            return true;
        }
    }
    false
}

/// Check if file size passes size filters.
pub fn passes_size_filter(path: &Path, minsize: Option<u64>, maxsize: Option<u64>) -> bool {
    let size = match path.metadata().map(|m| m.len()) {
        Ok(s) => s,
        Err(_) => return true,
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
pub fn passes_date_filter(
    path: &Path,
    newer: Option<std::time::SystemTime>,
    older: Option<std::time::SystemTime>,
) -> bool {
    let mtime = match path.metadata().and_then(|m| m.modified()) {
        Ok(t) => t,
        Err(_) => return true,
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

/// Known image/media extensions for recursive mode.
const DEFAULT_EXTS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp", "heic", "heif", "avif",
    "jxl", "jp2", "j2k", "jpx", "exr", "hdr", "ppm", "pgm", "pbm", "pam", "ico",
    "tga", "pcx", "sgi", "rgb", "svg", "eps", "ai", "psd", "dpx",
    "cr2", "cr3", "nef", "arw", "orf", "rw2", "pef", "raf", "dng", "srw", "srf",
    "sr2", "crw", "dcr", "kdc", "k25", "erf", "mef", "mos", "mrw", "nrw", "rwl",
    "x3f", "3fr", "fff", "iiq", "braw",
    "mp4", "mov", "m4v", "3gp", "3g2", "avi", "mkv", "webm", "mxf", "r3d",
    "mts", "m2ts", "ts", "flv", "wmv", "asf",
    "mp3", "flac", "m4a", "aac", "ogg", "opus", "wav", "aiff", "aif", "ape",
    "wv", "dsf", "dff", "tak", "wma", "mid", "midi", "au",
];

/// Expand paths: if recursive, walk directories; filter by extensions.
pub fn expand_paths(
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

    for path in paths {
        if path.is_dir() {
            if recursive {
                for entry in WalkDir::new(path)
                    .follow_links(true)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    let p = entry.path();

                    if matches_exclude(p, exclude) {
                        continue;
                    }

                    if p.is_file() {
                        if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                            let ext_lower = ext.to_lowercase();
                            let matches = if extensions.is_empty() {
                                DEFAULT_EXTS.contains(&ext_lower.as_str())
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
            if matches_exclude(path, exclude) {
                continue;
            }
            if !passes_date_filter(path, newer, older) {
                continue;
            }
            if !passes_size_filter(path, minsize, maxsize) {
                continue;
            }
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

    result.sort();
    result
}

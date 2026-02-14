//! exiftool-rs CLI - fast metadata reader/writer for images
//!
//! Supports: JPEG, PNG, TIFF, DNG, CR2, CR3, NEF, ARW, ORF, RW2, PEF, RAF, WebP, HEIC, AVIF, EXR, HDR

mod args;
mod commands;
mod condition;
mod datetime;
mod filters;
mod geotag;
mod html_output;
mod output;
mod paths;
mod xml_output;

use anyhow::{Context, Result};
use exiftool_formats::{add_composite_tags, FormatRegistry};
use std::fs::File;
use std::io::BufReader;
use std::env;
const VERSION: &str = env!("CARGO_PKG_VERSION");

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
        print!("{}", args::HELP.trim_start());
        return Ok(());
    }
    
    // Version
    if matches!(args.get(1).map(|s| s.as_str()), Some("-v" | "--version" | "-V")) {
        println!("exif {}", VERSION);
        return Ok(());
    }
    
    // Parse args manually for flexibility
    let parsed = args::parse_args(&args[1..])?;
    let registry = FormatRegistry::new();

    // JSON import mode
    if parsed.json_import.is_some() {
        return commands::import_from_json(&parsed, &registry);
    }

    // CSV import mode
    if parsed.csv_import.is_some() {
        return commands::import_from_csv(&parsed, &registry);
    }

    // Rename mode
    if parsed.rename.is_some() {
        return commands::rename_files(&parsed, &registry);
    }

    // Delete metadata mode
    if parsed.delete {
        return commands::delete_metadata(&parsed, &registry);
    }

    // Validate metadata mode
    if parsed.validate {
        return commands::validate_metadata(&parsed, &registry);
    }

    // HTML dump mode (show file structure)
    if parsed.html_dump {
        return commands::html_dump(&parsed, &registry);
    }

    // Duplicates mode
    if parsed.duplicates {
        return commands::find_duplicates(&parsed, &registry);
    }

    // Write mode (modify image tags or copy from file)
    if !parsed.tags.is_empty() || parsed.tags_from_file.is_some() || parsed.shift.is_some()
        || parsed.geotag.is_some() || parsed.icc_profile.is_some()
    {
        return commands::write_image(&parsed, &registry);
    }

    // Thumbnail extraction mode
    if parsed.thumbnail {
        return commands::extract_thumbnails(&parsed, &registry);
    }

    // Preview extraction mode (RAW files)
    if parsed.preview {
        return commands::extract_previews(&parsed, &registry);
    }

    // Expand paths (handle directories if recursive)
    let files = paths::expand_paths(
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
        return output::output_csv_unified(&files, &registry, &parsed);
    }

    // Collect output for potential file write
    let mut output_buf = String::new();
    let write_to_file = parsed.output.is_some();

    for path in &files {
        if parsed.verbose >= 1 {
            eprintln!("Reading: {}", path.display());
        }
        let file = File::open(path)
            .with_context(|| format!("Cannot open: {}", path.display()))?;
        let mut reader = BufReader::new(file);

        match registry.parse(&mut reader) {
            Ok(mut metadata) => {
                if parsed.verbose >= 2 {
                    eprintln!("  Format: {}", metadata.format);
                }
                // Add composite tags if requested
                if parsed.composite {
                    add_composite_tags(&mut metadata);
                }
                
                // Apply -if condition filter
                if let Some(ref cond) = parsed.if_condition {
                    if !condition::matches_condition(&metadata, cond) {
                        continue; // Skip file that doesn't match condition
                    }
                }
                
                if write_to_file {
                    output::format_metadata(path, &metadata, &parsed, &mut output_buf);
                } else {
                    output::print_metadata(path, &metadata, &parsed);
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



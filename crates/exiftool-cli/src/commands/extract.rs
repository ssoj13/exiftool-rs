//! Extract embedded thumbnail (-T) or preview (-P).

use std::fs::File;
use std::io::BufReader;

use anyhow::{Context, Result};
use exiftool_formats::FormatRegistry;

use crate::args::Args;

/// Extract embedded thumbnails to files.
pub fn extract_thumbnails(args: &Args, registry: &FormatRegistry) -> Result<()> {
    if args.files.is_empty() {
        anyhow::bail!("No input file specified for thumbnail extraction.\n\nUsage: exif -T <FILE>");
    }

    for path in &args.files {
        let file = File::open(path)
            .with_context(|| format!("Cannot open: {}", path.display()))?;
        let mut reader = BufReader::new(file);
        let metadata = registry
            .parse(&mut reader)
            .with_context(|| format!("Cannot parse: {}", path.display()))?;

        if let Some(ref thumb_data) = metadata.thumbnail {
            let output_path = if let Some(ref out) = args.output {
                out.clone()
            } else {
                let stem = path.file_stem().unwrap_or_default().to_string_lossy();
                path.with_file_name(format!("{}_thumb.jpg", stem))
            };

            std::fs::write(&output_path, thumb_data)
                .with_context(|| format!("Cannot write: {}", output_path.display()))?;

            eprintln!(
                "Thumbnail: {} ({} bytes)",
                output_path.display(),
                thumb_data.len()
            );
        } else {
            eprintln!("{}: no embedded thumbnail found", path.display());
        }
    }

    Ok(())
}

/// Extract embedded previews (larger image from RAW files).
pub fn extract_previews(args: &Args, registry: &FormatRegistry) -> Result<()> {
    if args.files.is_empty() {
        anyhow::bail!("No input file specified for preview extraction.\n\nUsage: exif -P <FILE>");
    }

    for path in &args.files {
        let file = File::open(path)
            .with_context(|| format!("Cannot open: {}", path.display()))?;
        let mut reader = BufReader::new(file);
        let metadata = registry
            .parse(&mut reader)
            .with_context(|| format!("Cannot parse: {}", path.display()))?;

        if let Some(ref preview_data) = metadata.preview {
            let output_path = if let Some(ref out) = args.output {
                out.clone()
            } else {
                let stem = path.file_stem().unwrap_or_default().to_string_lossy();
                path.with_file_name(format!("{}_preview.jpg", stem))
            };

            std::fs::write(&output_path, preview_data)
                .with_context(|| format!("Cannot write: {}", output_path.display()))?;

            eprintln!(
                "Preview: {} ({} bytes)",
                output_path.display(),
                preview_data.len()
            );
        } else {
            eprintln!("{}: no embedded preview found", path.display());
        }
    }

    Ok(())
}

//! Find duplicate files (-duplicates).

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Result;
use exiftool_formats::FormatRegistry;

use crate::args::Args;
use crate::paths;

/// Find duplicate files.
pub fn find_duplicates(args: &Args, registry: &FormatRegistry) -> Result<()> {
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
        anyhow::bail!("No files specified for -duplicates");
    }

    eprintln!("Scanning {} files for duplicates (by {})...", files.len(), args.dup_by);

    let mut groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for path in &files {
        let key = match args.dup_by.as_str() {
            "hash" | "content" => match std::fs::read(path) {
                Ok(data) => {
                    let hash = simple_hash(&data);
                    format!("{:016x}_{}", hash, data.len())
                }
                Err(_) => continue,
            },
            "datetime" => {
                let file = match File::open(path) {
                    Ok(f) => f,
                    Err(_) => continue,
                };
                let mut reader = BufReader::new(file);
                match registry.parse(&mut reader) {
                    Ok(metadata) => metadata
                        .exif
                        .get("DateTimeOriginal")
                        .or_else(|| metadata.exif.get("CreateDate"))
                        .or_else(|| metadata.exif.get("DateTime"))
                        .map(|v| v.to_string())
                        .unwrap_or_default(),
                    Err(_) => continue,
                }
            }
            "metadata" => {
                let file = match File::open(path) {
                    Ok(f) => f,
                    Err(_) => continue,
                };
                let mut reader = BufReader::new(file);
                match registry.parse(&mut reader) {
                    Ok(metadata) => {
                        let make = metadata
                            .exif
                            .get("Make")
                            .map(|v| v.to_string())
                            .unwrap_or_default();
                        let model = metadata
                            .exif
                            .get("Model")
                            .map(|v| v.to_string())
                            .unwrap_or_default();
                        let dt = metadata
                            .exif
                            .get("DateTimeOriginal")
                            .or_else(|| metadata.exif.get("CreateDate"))
                            .map(|v| v.to_string())
                            .unwrap_or_default();
                        let w = metadata
                            .exif
                            .get("ImageWidth")
                            .map(|v| v.to_string())
                            .unwrap_or_default();
                        let h = metadata
                            .exif
                            .get("ImageHeight")
                            .map(|v| v.to_string())
                            .unwrap_or_default();
                        format!("{}|{}|{}|{}x{}", make, model, dt, w, h)
                    }
                    Err(_) => continue,
                }
            }
            _ => {
                eprintln!(
                    "Unknown duplicate method: {}. Use: hash, content, datetime, metadata",
                    args.dup_by
                );
                return Ok(());
            }
        };

        if !key.is_empty() {
            groups.entry(key).or_default().push(path.clone());
        }
    }

    let mut dup_groups: Vec<_> = groups
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .collect();
    dup_groups.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    if dup_groups.is_empty() {
        println!("No duplicates found.");
        return Ok(());
    }

    let total_dups: usize = dup_groups.iter().map(|(_, f)| f.len() - 1).sum();
    println!(
        "Found {} duplicate groups ({} duplicate files):\n",
        dup_groups.len(),
        total_dups
    );

    for (i, (key, files)) in dup_groups.iter().enumerate() {
        let display_key = if key.len() > 60 { &key[..60] } else { key };
        println!("Group {} ({} files) [{}]", i + 1, files.len(), display_key);
        for f in files {
            let size = std::fs::metadata(f).map(|m| m.len()).unwrap_or(0);
            println!("  {} ({} bytes)", f.display(), size);
        }
        println!();
    }

    let wasted: u64 = dup_groups
        .iter()
        .flat_map(|(_, files)| files.iter().skip(1))
        .filter_map(|f| std::fs::metadata(f).ok())
        .map(|m| m.len())
        .sum();
    println!("Total wasted space: {:.2} MB", wasted as f64 / 1_048_576.0);

    Ok(())
}

fn simple_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in data {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

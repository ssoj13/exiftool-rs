//! Golden file test framework.
//!
//! Compares parser output against saved expected JSON files.
//! Set UPDATE_GOLDEN=1 to regenerate golden files.
//!
//! # Usage
//!
//! ```ignore
//! // Run tests normally
//! cargo test -p exiftool-formats golden
//!
//! // Update golden files when parser changes
//! UPDATE_GOLDEN=1 cargo test -p exiftool-formats golden
//! ```

use exiftool_formats::{FormatRegistry, Metadata};
use std::collections::BTreeMap;
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

/// Directory containing test images
const TESTDATA_DIR: &str = "tests/testdata";

/// Directory containing golden (expected) JSON files  
const GOLDEN_DIR: &str = "tests/golden/expected";

/// Convert metadata to sorted JSON for stable comparison.
fn metadata_to_json(m: &Metadata) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    
    map.insert("format".into(), m.format.into());
    
    // Sort EXIF tags for stable output
    let mut exif: BTreeMap<String, String> = BTreeMap::new();
    for (k, v) in m.exif.iter() {
        exif.insert(k.clone(), v.to_string());
    }
    map.insert("exif".into(), serde_json::to_value(&exif).unwrap());
    
    // Page info
    if !m.pages.is_empty() {
        let pages: Vec<_> = m.pages.iter().map(|p| {
            serde_json::json!({
                "index": p.index,
                "width": p.width,
                "height": p.height,
                "bits_per_sample": p.bits_per_sample,
            })
        }).collect();
        map.insert("pages".into(), serde_json::Value::Array(pages));
    }
    
    // Thumbnail/preview presence (not content, as it's binary)
    map.insert("has_thumbnail".into(), m.thumbnail.is_some().into());
    map.insert("has_preview".into(), m.preview.is_some().into());
    map.insert("has_xmp".into(), m.xmp.is_some().into());
    
    serde_json::Value::Object(map)
}

/// Get the golden file path for a test image.
fn golden_path(image_name: &str) -> PathBuf {
    let manifest = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest)
        .join(GOLDEN_DIR)
        .join(format!("{}.json", image_name))
}

/// Get the test image path.
fn testdata_path(image_name: &str) -> PathBuf {
    let manifest = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest).join(TESTDATA_DIR).join(image_name)
}

/// Check if golden files should be updated.
fn should_update() -> bool {
    std::env::var("UPDATE_GOLDEN").map(|v| v == "1").unwrap_or(false)
}

/// Run golden test for a single file.
/// Returns Ok if matches, Err with diff if mismatch.
pub fn golden_test(image_name: &str) -> Result<(), String> {
    let registry = FormatRegistry::new();
    let image_path = testdata_path(image_name);
    let golden = golden_path(image_name);
    
    // Parse image
    let file = fs::File::open(&image_path)
        .map_err(|e| format!("Cannot open {}: {}", image_path.display(), e))?;
    let mut reader = BufReader::new(file);
    
    let metadata = registry.parse(&mut reader)
        .map_err(|e| format!("Parse error: {}", e))?;
    
    let actual_json = metadata_to_json(&metadata);
    let actual_str = serde_json::to_string_pretty(&actual_json).unwrap();
    
    // Update mode: write golden file
    if should_update() {
        if let Some(parent) = golden.parent() {
            fs::create_dir_all(parent).ok();
        }
        fs::write(&golden, &actual_str)
            .map_err(|e| format!("Cannot write golden: {}", e))?;
        return Ok(());
    }
    
    // Compare mode: check against golden
    if !golden.exists() {
        return Err(format!(
            "Golden file missing: {}\nRun with UPDATE_GOLDEN=1 to create it.",
            golden.display()
        ));
    }
    
    let expected_str = fs::read_to_string(&golden)
        .map_err(|e| format!("Cannot read golden: {}", e))?;
    
    if actual_str.trim() == expected_str.trim() {
        Ok(())
    } else {
        Err(format!(
            "Golden mismatch for {}:\n\n--- expected ---\n{}\n\n--- actual ---\n{}\n",
            image_name, expected_str.trim(), actual_str.trim()
        ))
    }
}

/// Run golden tests for all files in testdata directory.
pub fn run_all_golden_tests() -> Vec<(String, Result<(), String>)> {
    let testdata = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(TESTDATA_DIR);
    let mut results = Vec::new();
    
    if !testdata.exists() {
        return results;
    }
    
    if let Ok(entries) = fs::read_dir(&testdata) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    // Skip non-image files
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    if !matches!(ext.to_lowercase().as_str(), 
                        "jpg" | "jpeg" | "png" | "tiff" | "tif" | "gif" | "bmp" |
                        "webp" | "heic" | "heif" | "cr2" | "cr3" | "nef" | "arw" |
                        "dng" | "orf" | "rw2" | "pef" | "raf" | "exr" | "hdr"
                    ) {
                        continue;
                    }
                    
                    results.push((name.to_string(), golden_test(name)));
                }
            }
        }
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_golden_files() {
        let results = run_all_golden_tests();
        
        if results.is_empty() {
            eprintln!("No test images found in {}", TESTDATA_DIR);
            return;
        }
        
        let mut failures = Vec::new();
        for (name, result) in results {
            match result {
                Ok(()) => eprintln!("  OK: {}", name),
                Err(e) => {
                    eprintln!("FAIL: {}", name);
                    failures.push((name, e));
                }
            }
        }
        
        if !failures.is_empty() {
            for (name, err) in &failures {
                eprintln!("\n=== {} ===\n{}", name, err);
            }
            panic!("{} golden test(s) failed", failures.len());
        }
    }
}

//! Integration tests for RAW format parsing.
//!
//! Tests use real RAW files downloaded from rawsamples.ch

use exiftool_formats::FormatRegistry;
use std::io::Cursor;

/// Helper to load test file if it exists
fn load_test_file(name: &str) -> Option<Vec<u8>> {
    // Try relative paths from test execution dir
    let paths = [
        format!("../../tests/{}", name),
        format!("../../../tests/{}", name),
        format!("tests/{}", name),
    ];
    
    for path in &paths {
        if let Ok(data) = std::fs::read(path) {
            return Some(data);
        }
    }
    
    // Try absolute path
    let abs_path = format!("C:/projects/projects.rust/exiftool-rs/tests/{}", name);
    std::fs::read(&abs_path).ok()
}

#[test]
fn parse_sony_arw() {
    let Some(data) = load_test_file("test_sony.arw") else {
        eprintln!("Skipping: test_sony.arw not found");
        return;
    };
    
    let registry = FormatRegistry::new();
    // Use by_extension for TIFF-based formats (they share TIFF magic)
    let parser = registry.by_extension("arw").expect("ARW parser should exist");
    let mut cursor = Cursor::new(&data);
    
    let result = parser.parse(&mut cursor);
    assert!(result.is_ok(), "Failed to parse ARW: {:?}", result.err());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.format, "ARW", "Format should be ARW");
    
    // ARW should have basic EXIF tags
    println!("ARW metadata: {} tags", metadata.exif.len());
    for (tag, val) in metadata.exif.iter().take(10) {
        println!("  {}: {:?}", tag, val);
    }
    
    // Check for expected Sony tags
    if let Some(make) = metadata.exif.get_str("Make") {
        assert!(make.contains("SONY") || make.contains("Sony"), 
            "Make should contain SONY, got: {}", make);
    }
}

#[test]
fn parse_olympus_orf() {
    let Some(data) = load_test_file("test_olympus.orf") else {
        eprintln!("Skipping: test_olympus.orf not found");
        return;
    };
    
    let registry = FormatRegistry::new();
    let parser = registry.by_extension("orf").expect("ORF parser should exist");
    let mut cursor = Cursor::new(&data);
    
    let result = parser.parse(&mut cursor);
    assert!(result.is_ok(), "Failed to parse ORF: {:?}", result.err());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.format, "ORF", "Format should be ORF");
    
    println!("ORF metadata: {} tags", metadata.exif.len());
    for (tag, val) in metadata.exif.iter().take(10) {
        println!("  {}: {:?}", tag, val);
    }
    
    // Check for Olympus make
    if let Some(make) = metadata.exif.get_str("Make") {
        assert!(make.contains("OLYMPUS") || make.contains("Olympus"),
            "Make should contain OLYMPUS, got: {}", make);
    }
}

#[test]
fn parse_panasonic_rw2() {
    let Some(data) = load_test_file("test_panasonic.rw2") else {
        eprintln!("Skipping: test_panasonic.rw2 not found");
        return;
    };
    
    let registry = FormatRegistry::new();
    let parser = registry.by_extension("rw2").expect("RW2 parser should exist");
    let mut cursor = Cursor::new(&data);
    
    let result = parser.parse(&mut cursor);
    assert!(result.is_ok(), "Failed to parse RW2: {:?}", result.err());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.format, "RW2", "Format should be RW2");
    
    println!("RW2 metadata: {} tags", metadata.exif.len());
    for (tag, val) in metadata.exif.iter().take(10) {
        println!("  {}: {:?}", tag, val);
    }
    
    // Check for Panasonic make
    if let Some(make) = metadata.exif.get_str("Make") {
        assert!(make.contains("Panasonic") || make.contains("PANASONIC"),
            "Make should contain Panasonic, got: {}", make);
    }
}

#[test]
fn parse_pentax_pef() {
    let Some(data) = load_test_file("test_pentax.pef") else {
        eprintln!("Skipping: test_pentax.pef not found");
        return;
    };
    
    let registry = FormatRegistry::new();
    let parser = registry.by_extension("pef").expect("PEF parser should exist");
    let mut cursor = Cursor::new(&data);
    
    let result = parser.parse(&mut cursor);
    assert!(result.is_ok(), "Failed to parse PEF: {:?}", result.err());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.format, "PEF", "Format should be PEF");
    
    println!("PEF metadata: {} tags", metadata.exif.len());
    for (tag, val) in metadata.exif.iter().take(10) {
        println!("  {}: {:?}", tag, val);
    }
    
    // Check for Pentax make
    if let Some(make) = metadata.exif.get_str("Make") {
        assert!(make.contains("PENTAX") || make.contains("Pentax"),
            "Make should contain PENTAX, got: {}", make);
    }
}

#[test]
fn parse_webp() {
    let Some(data) = load_test_file("test_image.webp") else {
        eprintln!("Skipping: test_image.webp not found");
        return;
    };
    
    let registry = FormatRegistry::new();
    let mut cursor = Cursor::new(&data);
    
    let result = registry.parse(&mut cursor);
    assert!(result.is_ok(), "Failed to parse WebP: {:?}", result.err());
    
    let metadata = result.unwrap();
    assert_eq!(metadata.format, "WebP", "Format should be WebP");
    
    println!("WebP metadata: {} tags", metadata.exif.len());
    for (tag, val) in metadata.exif.iter().take(10) {
        println!("  {}: {:?}", tag, val);
    }
    
    // WebP from gstatic might not have EXIF, just check format detection worked
    println!("WebP parsed successfully");
}

#[test]
fn format_detection_arw() {
    let Some(data) = load_test_file("test_sony.arw") else {
        return;
    };
    
    let registry = FormatRegistry::new();
    
    // ARW uses TIFF magic, detection by extension/content
    if let Some(_parser) = registry.detect(&data[..16.min(data.len())]) {
        // Should detect as some TIFF-based format
        println!("Detected format for ARW bytes: available");
    }
}

#[test]
fn format_detection_orf() {
    let Some(data) = load_test_file("test_olympus.orf") else {
        return;
    };
    
    let registry = FormatRegistry::new();
    
    // ORF has special IIRO magic bytes
    if let Some(_parser) = registry.detect(&data[..16.min(data.len())]) {
        println!("Detected format for ORF bytes: available");
    }
}

#[test]
fn format_detection_rw2() {
    let Some(data) = load_test_file("test_panasonic.rw2") else {
        return;
    };
    
    let registry = FormatRegistry::new();
    
    // RW2 has 0x55 marker
    if let Some(_parser) = registry.detect(&data[..16.min(data.len())]) {
        println!("Detected format for RW2 bytes: available");
    }
}

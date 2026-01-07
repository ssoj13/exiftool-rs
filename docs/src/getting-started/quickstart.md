# Quick Start

## Reading Metadata

The simplest way to read metadata:

```rust
use exiftool_formats::{FormatRegistry, FormatParser};
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("photo.jpg")?;
    let mut reader = BufReader::new(file);
    
    let registry = FormatRegistry::new();
    let metadata = registry.parse(&mut reader)?;
    
    // Basic info
    println!("Format: {}", metadata.format);
    
    // Access tags
    if let Some(make) = metadata.exif.get_str("Make") {
        println!("Camera: {}", make);
    }
    
    // Iterate all tags
    for (tag, value) in metadata.exif.iter() {
        println!("{}: {}", tag, value);
    }
    
    Ok(())
}
```

## Common Tags

```rust
// Camera info
metadata.exif.get_str("Make")           // "Canon"
metadata.exif.get_str("Model")          // "EOS R5"

// Capture settings
metadata.exif.get_u32("ISO")            // 400
metadata.exif.get_f64("ExposureTime")   // 0.008 (1/125)
metadata.exif.get_f64("FNumber")        // 2.8

// Image dimensions
metadata.exif.get_u32("ImageWidth")     // 8192
metadata.exif.get_u32("ImageHeight")    // 5464

// Date/time
metadata.exif.get_str("DateTimeOriginal")  // "2024:01:15 14:30:00"
```

## Format Detection

The registry auto-detects format from file headers:

```rust
let registry = FormatRegistry::new();

// From file
let metadata = registry.parse(&mut reader)?;
println!("Detected: {}", metadata.format);  // "JPEG", "PNG", "CR3", etc.

// Check specific format
if let Some(parser) = registry.detect(&header_bytes) {
    println!("Format: {}", parser.format_name());
}
```

## Error Handling

```rust
use exiftool_formats::{FormatRegistry, Error};

match registry.parse(&mut reader) {
    Ok(metadata) => println!("Success: {}", metadata.format),
    Err(Error::UnsupportedFormat) => println!("Unknown format"),
    Err(Error::InvalidStructure(msg)) => println!("Corrupt file: {}", msg),
    Err(e) => println!("Error: {}", e),
}
```

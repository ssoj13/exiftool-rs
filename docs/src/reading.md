# Reading Metadata

## Basic Reading

```rust
use exiftool_formats::{FormatRegistry, Metadata};
use std::fs::File;
use std::io::BufReader;

let file = File::open("image.jpg")?;
let mut reader = BufReader::new(file);

let registry = FormatRegistry::new();
let metadata = registry.parse(&mut reader)?;
```

## Accessing Values

The `Attrs` struct provides typed accessors:

```rust
// String values
let make: Option<&str> = metadata.exif.get_str("Make");

// Integer values
let iso: Option<u32> = metadata.exif.get_u32("ISO");
let width: Option<i32> = metadata.exif.get_i32("ImageWidth");

// Float values  
let fnumber: Option<f64> = metadata.exif.get_f64("FNumber");

// Rational values (numerator, denominator)
let exposure: Option<(u32, u32)> = metadata.exif.get_urational("ExposureTime");
// Returns (1, 125) for 1/125 sec

// Raw value with full type info
let value: Option<&AttrValue> = metadata.exif.get("CustomTag");
```

## Human-Readable Values

Some tags store numeric codes. Get interpreted values:

```rust
// Raw value
metadata.exif.get_u32("Orientation")  // 6

// Human-readable
metadata.get_interpreted("Orientation")  // "Rotate 90 CW"

// With units
metadata.get_display("FocalLength")  // "50 mm"
metadata.get_display("ExposureTime")  // "1/125 sec"
metadata.get_display("FNumber")  // "f/2.8"
```

## XMP Data

XMP is returned as raw XML string:

```rust
if let Some(xmp) = &metadata.xmp {
    println!("XMP length: {} bytes", xmp.len());
    // Parse with your preferred XML library
}
```

## Thumbnails and Previews

Many formats embed preview images:

```rust
// Small thumbnail (typically 160x120, JPEG)
if let Some(thumb) = &metadata.thumbnail {
    std::fs::write("thumb.jpg", thumb)?;
}

// Larger preview (RAW files often have full-size JPEG)
if let Some(preview) = &metadata.preview {
    std::fs::write("preview.jpg", preview)?;
}
```

## Multi-Page Files

TIFF files can have multiple pages:

```rust
if metadata.is_multi_page() {
    println!("Pages: {}", metadata.page_count());
    
    for page in &metadata.pages {
        println!("Page {}: {}x{}", page.index, page.width, page.height);
        if page.is_thumbnail() {
            println!("  (thumbnail)");
        }
    }
}
```

## RAW File Detection

```rust
if metadata.is_camera_raw() {
    println!("This is a RAW file, read-only");
} else if metadata.is_writable() {
    println!("Can modify this file");
}
```

## Iterating Tags

```rust
// All tags
for (name, value) in metadata.exif.iter() {
    println!("{}: {}", name, value);
}

// Check existence
if metadata.exif.contains("GPSLatitude") {
    println!("Has GPS data");
}

// Count
println!("Total tags: {}", metadata.exif.len());
```

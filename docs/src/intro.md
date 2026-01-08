# exiftool-rs

A fast, dependency-light image metadata library written in Rust.

## Why Another EXIF Library?

The Perl-based ExifTool is the gold standard for metadata extraction. It handles 
every obscure format and camera quirk accumulated over 20+ years. But sometimes 
you need something different:

- **Embeddable** - A library you can link into your app without spawning processes
- **Fast** - Native code that doesn't spin up an interpreter
- **Portable** - Rust compiles to pretty much anything: WASM, mobile, embedded
- **Polyglot** - Rust's FFI means free bindings to Python, Ruby, Node, etc.

This library doesn't aim to replace ExifTool. It covers the common cases - the 
formats and tags you'll actually encounter - with clean, maintainable code.

## What You Get

- **17 formats** - JPEG, PNG, TIFF, HEIC/AVIF, WebP, RAW (CR2, CR3, NEF, ARW, ORF, RW2, PEF, RAF), EXR, HDR
- **Read & write** - Extract metadata, modify tags, save changes
- **Python bindings** - `pip install exiftool-py` and you're done
- **CLI tool** - Drop-in for basic ExifTool usage
- **Zero unsafe** - Pure Rust, no C dependencies (except optional Python bindings)

## Quick Example

**Rust:**
```rust
use exiftool_formats::{FormatRegistry, FormatParser};
use std::fs::File;
use std::io::BufReader;

let file = File::open("photo.jpg")?;
let mut reader = BufReader::new(file);

let registry = FormatRegistry::new();
let metadata = registry.parse(&mut reader)?;

println!("Camera: {:?}", metadata.exif.get("Make"));
println!("Date: {:?}", metadata.exif.get("DateTimeOriginal"));
```

**Python:**
```python
import exiftool_rs as exif

img = exif.open("photo.jpg")
print(f"Camera: {img.make}")
print(f"Date: {img.date_time_original}")

# Modify and save
img.artist = "John Doe"
img.save()
```

## Project Status

Production-ready for common use cases. The library handles the formats and tags 
you'll encounter in real-world applications. Edge cases and exotic formats may 
need work - contributions welcome.

## License

MIT

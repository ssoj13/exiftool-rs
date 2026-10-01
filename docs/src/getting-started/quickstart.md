# Quick start: read metadata in Rust

Set up the local dependencies described in [installation](installation.md).
This example accepts a path and uses the repository's `Writer.jpg` fixture.

## Read a file

Save the following as your application's `src/main.rs`:

```rust
use exiftool_formats::FormatRegistry;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or("expected a file path")?;
    let registry = FormatRegistry::new();
    let metadata = registry.parse_file(Path::new(&path))?;

    println!("Format: {}", metadata.format);
    println!("Make: {:?}", metadata.exif.get_str("Make"));
    println!("Model: {:?}", metadata.exif.get_str("Model"));
    println!("ISO: {:?}", metadata.exif.get_u32("ISO"));
    for (name, value) in metadata.exif.iter() {
        println!("{name}: {value}");
    }
    Ok(())
}
```

From an application next to the checkout, run:

```bash
cargo run -- ../exiftool-rs/crates/exiftool-formats/tests/testdata/Writer.jpg
```

Available tags depend on the input. Typed accessors return `None` when the tag
is absent or its stored type cannot be converted by that accessor.

## Why use a path?

`parse_file` opens a buffered reader, detects the container from its header,
and passes the extension as a classification hint. This matters for TIFF-family
RAW files such as NEF and ARW. An extension does not force a parser to accept
unrelated magic bytes.

## Continue

- [Reading metadata](../reading.md): bytes in memory, interpreted values, and previews.
- [Writing metadata](../writing.md): edit a JPEG and save a separate output file.
- [Parser design](../architecture/parsers.md): detection order and extension hints.

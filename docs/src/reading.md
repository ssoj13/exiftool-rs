# Reading metadata

Set up [local Rust dependencies](getting-started/installation.md) first.
Reuse a `FormatRegistry` across files. Prefer `parse_file` when you have a path
so TIFF-family classification receives the extension hint.

## Files and bytes

```rust
use exiftool_formats::FormatRegistry;
use std::io::Cursor;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("crates/exiftool-formats/tests/testdata/Writer.jpg");
    let registry = FormatRegistry::new();
    let metadata = registry.parse_file(path)?;
    println!("{}: {} tags", metadata.format, metadata.exif.len());

    let bytes = std::fs::read(path)?;
    let mut reader = Cursor::new(bytes);
    let from_bytes = registry.parse(&mut reader)?;
    println!("From bytes: {}", from_bytes.format);
    Ok(())
}
```

`parse` starts detection at the reader's current position and then rewinds to
zero; provide a reader positioned at the start of the file. If you have bytes
and know the extension, call `parse_with_hint(&mut reader, Some("nef"))`.

## Typed and formatted values

Given the `metadata` returned above:

```rust
let make = metadata.exif.get_str("Make");
let iso = metadata.exif.get_u32("ISO");
let fnumber = metadata.exif.get_f64("FNumber");
let exposure = metadata.exif.get_urational("ExposureTime");
let raw_value = metadata.exif.get("Orientation");

println!("{make:?}, ISO {iso:?}, f-number {fnumber:?}");
println!("Exposure fraction: {exposure:?}, orientation: {raw_value:?}");
println!("Orientation: {:?}", metadata.get_interpreted("Orientation"));
println!("Exposure: {:?}", metadata.get_display("ExposureTime"));
```

These accessors return `Option`: absence of a field is normal.
`get_interpreted` handles selected numeric enums and stored strings; not every
numeric tag has an interpretation. `get_display` adds formatting for selected
exposure, focal-length, and GPS fields.

## Metadata payloads

`metadata.xmp` holds raw XML. To decode an extracted packet, use `XmpParser`
from `exiftool-xmp`. `metadata.icc` contains profile bytes when extracted.
The attribute map named `exif` also contains container tags and IPTC attributes.

```rust
if let Some(xmp) = &metadata.xmp {
    println!("XMP: {} bytes", xmp.len());
}
if let Some(icc) = &metadata.icc {
    println!("ICC profile: {} bytes", icc.len());
}
```

## Previews and pages

Thumbnails and previews are optional byte vectors; their presence and encoding
depend on the format. Check the payload before choosing an output extension.
TIFF pages carry dimensions, compression, and subfile flags.

```rust
if let Some(thumbnail) = &metadata.thumbnail {
    println!("Thumbnail: {} bytes", thumbnail.len());
}
if let Some(preview) = &metadata.preview {
    println!("Preview: {} bytes", preview.len());
}
for page in &metadata.pages {
    println!("Page {}: {}x{}", page.index, page.width, page.height);
}
println!("RAW: {}, writable: {}", metadata.is_camera_raw(), metadata.is_writable());
```

See [writing](writing.md) for saving edits, [formats](formats.md) for coverage,
and [performance](performance.md) for resource limits.

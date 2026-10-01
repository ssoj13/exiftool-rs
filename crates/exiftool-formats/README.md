# exiftool-formats

Container parsers and writers for image, camera RAW, audio/video, document,
and archive metadata. This is the main Rust entry point for ordinary file operations.

## Read a file

```rust
use exiftool_formats::FormatRegistry;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = FormatRegistry::new();
    let metadata = registry.parse_file(Path::new(
        "crates/exiftool-formats/tests/testdata/Writer.jpg",
    ))?;
    println!("{}: {} tags", metadata.format, metadata.exif.len());
    Ok(())
}
```

Run from the repository root. See [installation](../../docs/src/getting-started/installation.md)
for local Cargo dependencies and [reading](../../docs/src/reading.md) for memory inputs.

## Main components

| Component | Purpose |
|-----------|---------|
| `FormatRegistry` | Detect and dispatch container parsers |
| `FormatParser` | Shared parser trait over `ReadSeek` |
| `Metadata` | Typed attributes, XMP, ICC, previews, and pages |
| `parsers::default_parsers()` | Central registration list and detection order |
| `utils` | Shared TIFF/EXIF conversion and serialization |
| `makernotes` | Camera/vendor metadata decoding and supported overlays |
| Format-specific writers | Rewrite supported metadata using the original container |

Detection reads up to `DETECT_HEADER_LEN` (132) bytes and rewinds the input.
`parse_file` provides an extension hint for TIFF-family classification.
Custom parser lists change runtime dispatch, not build dependencies.

See [parser design](../../docs/src/architecture/parsers.md),
[writing](../../docs/src/writing.md), and [formats](../../docs/src/formats.md).

## Resource limits

Some readers and writers buffer whole files. The shared reader imposes a
100 MiB limit. See [performance](../../docs/src/performance.md).

## Contribute

Register new parsers in `parsers.rs`, re-export them from `lib.rs` as needed,
and test detection as well as parsing. See
[contributing](../../docs/src/contributing.md) and
[dependency access](../../docs/src/dependency-access.md).

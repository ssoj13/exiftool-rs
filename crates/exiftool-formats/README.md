# exiftool-formats

Format parsers and writers for image/video/audio metadata (EXIF, XMP, IPTC).

## Why this crate exists

Files come in many formats (JPEG, TIFF, PNG, CR2, HEIC, …). This crate provides:

- **Parsers** — read metadata from bytes, auto-detect format from magic
- **Writers** — embed/update metadata in supported formats

## What it contains

| Module/Type | Purpose |
|-------------|---------|
| `parsers` | Central list of format parsers; add/remove by editing `default_parsers()` |
| `registry` | `FormatRegistry` — auto-detect from header, then parse |
| `traits::FormatParser` | Interface: `can_parse(header)`, `parse(reader)` → `Metadata` |
| `utils` | `parse_tiff_exif`, `entry_to_attr`, `build_exif_bytes` — shared TIFF/EXIF logic |
| `makernotes` | Vendor-specific MakerNotes (Canon, Nikon, Sony, …) |
| Writers | `JpegWriter`, `TiffWriter`, `PngWriter`, `WebpWriter`, `HeicWriter`, … |

## How it works

1. **Detection**: `FormatRegistry::parse(reader)` reads 16 bytes, seeks back, finds first parser where `can_parse(header)`.
2. **Parse**: Calls `parser.parse(reader)` → returns `Metadata { exif, xmp, thumbnail, pages }`.
3. **EXIF path**: TIFF-based formats (JPEG APP1, PNG eXIf, WebP EXIF, HEIC, …) use `utils::parse_tiff_exif()` — single implementation, no duplication.
4. **Tag names**: From `exiftool-tags` (generated from ExifTool Perl).

## Where used

- **exiftool-cli** — `FormatRegistry::new()` → parse files, display/write metadata
- **exiftool-py** — same, via PyO3
- **Direct use** — `let registry = FormatRegistry::new(); registry.parse(&mut file)?`

## Adding a new format

1. Create `src/xxx.rs` implementing `FormatParser`.
2. Add `mod xxx;` to `lib.rs`.
3. Add `Box::new(XxxParser)` to `parsers::default_parsers()` (order = detection priority).
4. Re-export in `lib.rs` if public API needs it.

## Minimal builds

Use `FormatRegistry::with_parsers()` and `parsers::default_parsers()` to build a subset:

```rust
use exiftool_formats::{FormatRegistry, JpegParser, PngParser};

let parsers = vec![
    Box::new(JpegParser),
    Box::new(PngParser),
];
let registry = FormatRegistry::with_parsers(parsers);
let metadata = registry.parse(&mut reader)?;
```

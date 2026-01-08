# exiftool-rs

> **Note:** This is an experimental project implementing a small subset of [ExifTool](https://exiftool.org/) functionality in Rust. It is not intended to replace the original ExifTool, which remains the definitive tool for image metadata manipulation. Use this library for learning, experimentation, or when you need a lightweight pure-Rust solution for basic metadata operations.

Fast, pure Rust library for reading and writing image metadata (EXIF, XMP, IPTC).

A native Rust alternative to [ExifTool](https://exiftool.org/) with zero runtime dependencies - no Perl, no external binaries. Parses metadata directly from bytes using auto-generated tag definitions from ExifTool's database (~2500+ tags).

## Features

- **Read/Write EXIF** - Full IFD parsing with MakerNotes support (Canon, Nikon, Sony, Fujifilm, etc.)
- **XMP Support** - Parse rdf:Bag, rdf:Seq, rdf:Alt structures
- **17 Formats** - JPEG, PNG, TIFF, DNG, HEIC/AVIF, CR2, CR3, NEF, ARW, ORF, RW2, PEF, RAF, WebP, EXR, HDR
- **Zero Dependencies** - Pure Rust, no external tools required
- **Fast** - Native code, ~10-100x faster than ExifTool for batch operations
- **Type-Safe** - Strongly typed values (Rational, URational, DateTime, etc.)
- **Geotagging** - Add GPS coordinates from GPX track files
- **Time Shift** - Bulk adjust DateTime tags
- **ICC Profiles** - Read/embed color profiles

## Quick Start

```rust
use exiftool_formats::{FormatRegistry, Metadata};
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Auto-detect format and parse
    let registry = FormatRegistry::new();
    let file = File::open("photo.jpg")?;
    let metadata = registry.parse(&mut BufReader::new(file))?;

    // Access EXIF data
    println!("Format: {}", metadata.format);
    println!("Make: {:?}", metadata.exif.get_str("Make"));
    println!("Model: {:?}", metadata.exif.get_str("Model"));
    println!("ISO: {:?}", metadata.exif.get_u32("ISO"));
    
    // Iterate all tags
    for (tag, value) in metadata.exif.iter() {
        println!("{}: {}", tag, value);
    }

    // XMP data (if present)
    if let Some(xmp) = &metadata.xmp {
        println!("XMP: {} bytes", xmp.len());
    }

    Ok(())
}
```

## Writing Metadata

```rust
use exiftool_formats::{FormatRegistry, JpegWriter};
use exiftool_attrs::AttrValue;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = FormatRegistry::new();
    let file = File::open("input.jpg")?;
    let mut metadata = registry.parse(&mut BufReader::new(file))?;

    // Modify metadata
    metadata.exif.set("Artist", AttrValue::Str("John Doe".into()));
    metadata.exif.set("Copyright", AttrValue::Str("2024 John Doe".into()));
    metadata.exif.set("Software", AttrValue::Str("exiftool-rs".into()));

    // Write to new file
    let mut input = BufReader::new(File::open("input.jpg")?);
    let mut output = Vec::new();
    
    // Build EXIF bytes and write
    let exif_bytes = build_exif(&metadata)?; // see examples/
    JpegWriter::write(&mut input, &mut output, Some(&exif_bytes), None)?;
    
    std::fs::write("output.jpg", output)?;
    Ok(())
}
```

## Python Bindings

```bash
pip install exiftool-py
```

```python
import exiftool_py as exif

# Open and read
img = exif.open("photo.jpg")
print(img.make, img.model)
print(img.iso, img.fnumber, img.exposure_time)

# Dict-like access
print(img["Artist"])
for tag in img:
    print(f"{tag}: {img[tag]}")

# Check if writable before modifying
if img.is_writable:
    img.artist = "John Doe"
    img.save()

# Time shift (+2 hours)
img.shift_time("+2:00")
img.save()

# Geotagging from GPX
coords = img.geotag("track.gpx")
if coords:
    print(f"Geotagged to {coords}")
img.save()

# ICC profile
img.set_icc_from_file("sRGB.icc")
img.save()

# Composite tags
img.add_composite()
print(img["ImageSize"], img["Megapixels"])

# Detect camera RAW files  
if img.is_camera_raw:
    print(f"RAW file from {img.make}")

# Parallel batch scan
for img in exif.scan("photos/**/*.jpg", parallel=True):
    print(img.path, img.make)
```

## CLI Usage

```bash
# Install
cargo install --path crates/exiftool-cli

# Read metadata
exif photo.jpg                        # All tags
exif -g Model photo.jpg               # Single tag (value only)
exif -g Make -g Model *.jpg           # Multiple tags
exif -g "Date*" photo.jpg             # Wildcard: all Date* tags
exif -f json *.jpg                    # JSON output
exif -f csv photos/*.png              # CSV for spreadsheets
exif -f html *.jpg -o report.html     # HTML output
exif -f json *.jpg -o meta.json       # Export to file

# Write metadata
exif -t Artist="John Doe" photo.jpg
exif -t Make=Canon -t Model="EOS R5" photo.jpg
exif -w output.jpg -t Copyright="2024" photo.jpg  # Write to new file
exif -p -t Copyright="2024" photo.jpg             # In-place modify

# Time shift
exif --shift "+2:00" -p photo.jpg     # Add 2 hours
exif --shift "-30" -p photo.jpg       # Subtract 30 minutes

# Geotagging
exif --geotag track.gpx -p photo.jpg  # Add GPS from GPX

# Import from JSON/CSV
exif --json=meta.json -p photo.jpg    # Import tags from JSON
exif --csv=meta.csv -p *.jpg          # Batch import from CSV

# Copy tags between files
exif --tagsFromFile src.jpg -p dst.jpg  # Copy all tags
exif --tagsFromFile src.jpg -t Make -t Model -p dst.jpg  # Copy specific

# Batch rename with templates
exif --rename "$Make_$Model_%Y%m%d" -p *.jpg  # Canon_EOS R5_20240115.jpg
exif --rename "%Y/%m/%d/$filename" -p *.jpg   # Organize by date folders

# Strip all metadata (privacy)
exif --delete -p photo.jpg            # Remove EXIF, XMP, IPTC, ICC
exif --delete -r -p photos/           # Strip entire directory

# Validate metadata
exif --validate photo.jpg             # Check for issues
exif --validate -r photos/            # Validate directory

# Conditional processing
exif -if "Make eq Canon" -r photos/   # Only Canon files
exif -if "ISO gt 800" *.jpg            # High ISO photos
exif -if "Model contains R5" *.jpg     # Model contains "R5"

# File analysis
exif -htmlDump photo.jpg -o dump.html  # Hex dump + structure
exif -htmlDump -r photos/              # Analyze multiple files

# Find duplicates
exif -duplicates hash -r photos/       # Exact duplicates (content hash)
exif -duplicates datetime -r photos/   # Same capture time
exif -duplicates metadata -r photos/   # Same Make/Model/DateTime/Size

# ICC profile
exif --icc sRGB.icc -p photo.jpg      # Embed color profile

# File filtering
exif -r photos/                       # Recursive scan
exif -r -e jpg,png photos/            # Filter by extension
exif -r -x "*_thumb*" photos/         # Exclude pattern
exif -r --newer 2024-01-01 photos/    # Date filter
exif -r --minsize 1M photos/          # Size filter

# Thumbnail/preview extraction
exif -T photo.jpg                     # Extract thumbnail
exif -P photo.cr2                     # Extract RAW preview

# Supported formats
exif image.{jpg,png,tiff,dng,heic,avif,cr2,cr3,nef,arw,orf,rw2,pef,raf,webp,exr,hdr}
```

## Crate Structure

```
exiftool-rs/
  crates/
    exiftool-core/      # IFD reader/writer, byte order, raw values
    exiftool-attrs/     # Typed attribute storage (Attrs, AttrValue)
    exiftool-tags/      # Auto-generated tag tables (~2500+ tags)
    exiftool-formats/   # Format parsers and writers
    exiftool-xmp/       # XMP parser (rdf:Bag/Seq/Alt)
    exiftool-cli/       # Command-line tool
    exiftool-py/        # Python bindings (PyO3)
```

## Supported Formats

| Format | Read | Write | Notes |
|--------|------|-------|-------|
| JPEG   | Yes  | Yes   | APP1 EXIF, APP1 XMP, APP12 Ducky, APP13 IPTC |
| PNG    | Yes  | Yes   | eXIf chunk, tEXt, iTXt, zTXt |
| TIFF   | Yes  | Yes   | Full IFD chain |
| DNG    | Yes  | Yes   | Via TIFF parser, DNGVersion detection |
| HEIC   | Yes  | Yes   | ISOBMFF with EXIF item extraction |
| AVIF   | Yes  | Yes   | Via HEIC parser |
| CR2    | Yes  | -     | Canon RAW (TIFF-based) |
| CR3    | Yes  | -     | Canon RAW (ISOBMFF-based) |
| NEF    | Yes  | Yes   | Nikon RAW |
| ARW    | Yes  | -     | Sony RAW |
| ORF    | Yes  | -     | Olympus RAW (supports IIRO magic) |
| RW2    | Yes  | -     | Panasonic RAW (supports 0x55 magic) |
| PEF    | Yes  | -     | Pentax RAW |
| RAF    | Yes  | Yes   | Fujifilm RAW |
| WebP   | Yes  | Yes   | Google WebP (VP8/VP8L/VP8X) |
| EXR    | Yes  | Yes   | OpenEXR attributes |
| HDR    | Yes  | Yes   | Radiance RGBE |

## AttrValue Types

```rust
pub enum AttrValue {
    Bool(bool),
    Str(String),
    Int(i32),
    UInt(u32),
    Float(f32),
    Double(f64),
    Rational(i32, i32),     // Signed rational (num/den)
    URational(u32, u32),    // Unsigned rational
    Bytes(Vec<u8>),         // Binary data
    DateTime(NaiveDateTime),
    List(Vec<AttrValue>),
    Map(HashMap<String, AttrValue>),
    // ...and more
}
```

## MakerNotes Support

Vendor-specific MakerNotes are parsed using auto-generated tables:

- **Canon** - Camera settings, lens info, AF points
- **Nikon** - Shot info, lens data, NEF settings  
- **Sony** - Camera settings, lens info
- **Fujifilm** - Film simulation, dynamic range
- **Olympus** - Camera settings, equipment
- **Panasonic** - Shooting mode, lens info
- **Pentax** - Camera settings
- **Samsung** - Device info
- **Apple** - HDR info, burst mode

## Tag Database

Tags are auto-generated from ExifTool's Perl source using `cargo xtask codegen`:

```rust
// ~2500 EXIF/TIFF/DNG tags available
use exiftool_tags::generated::exif::EXIF_MAIN;

if let Some(tag_def) = EXIF_MAIN.get(&0x010F) {
    println!("Tag name: {}", tag_def.name); // "Make"
}
```

## Performance

Benchmarks vs ExifTool (reading 1000 JPEGs):

| Tool | Time | Memory |
|------|------|--------|
| exiftool-rs | ~0.8s | ~15MB |
| ExifTool | ~45s | ~120MB |

*Note: ExifTool is more feature-complete. This comparison is for simple read operations.*

## Building

```bash
# Build all crates (release)
cargo build --release

# Or use bootstrap (release by default)
./bootstrap.ps1 build
python bootstrap.py build

# Debug build
./bootstrap.ps1 build --debug
python bootstrap.py build --debug

# Run tests
cargo test

# Run benchmarks
cargo bench -p exiftool-formats

# Regenerate tag tables from ExifTool source
cargo xtask codegen

# Install CLI
cargo install --path crates/exiftool-cli

# Build Python wheel (release)
./bootstrap.ps1 python
python bootstrap.py python

# Build Python wheel (debug)
./bootstrap.ps1 python --debug
python bootstrap.py python --debug
```

## Fuzz Testing

Fuzz testing helps find crashes, panics, and edge cases by feeding random/mutated data to parsers. This project includes fuzz targets for all major format parsers.

### Prerequisites

```bash
# Install cargo-fuzz (requires nightly Rust)
rustup install nightly
cargo +nightly install cargo-fuzz
```

### Available Targets

| Target | Description |
|--------|-------------|
| `fuzz_jpeg` | JPEG parser with APP segments |
| `fuzz_png` | PNG parser with chunks |
| `fuzz_tiff` | TIFF/DNG IFD parser |
| `fuzz_webp` | WebP container parser |
| `fuzz_heic` | HEIC/AVIF ISOBMFF parser |
| `fuzz_cr3` | Canon CR3 parser |
| `fuzz_registry` | Auto-detection across all formats |

### Running Fuzz Tests

```bash
cd fuzz

# Run single target (runs indefinitely until Ctrl+C)
cargo +nightly fuzz run fuzz_jpeg

# Run with timeout (e.g., 60 seconds)
cargo +nightly fuzz run fuzz_jpeg -- -max_total_time=60

# Run with specific number of iterations
cargo +nightly fuzz run fuzz_jpeg -- -runs=10000

# Run all targets sequentially (quick smoke test)
for target in fuzz_jpeg fuzz_png fuzz_tiff fuzz_webp fuzz_heic fuzz_cr3 fuzz_registry; do
    cargo +nightly fuzz run $target -- -max_total_time=30
done
```

### What to Expect

- **Normal output**: Lines showing `#12345` (iteration count), `cov:` (coverage), `ft:` (features)
- **No crashes = good**: Parsers handle malformed input gracefully
- **Crash found**: Fuzzer saves crashing input to `fuzz/artifacts/<target>/`
- **Slow start**: First run builds corpus; subsequent runs are faster

### Reproducing Crashes

If a crash is found:

```bash
# Reproduce crash
cargo +nightly fuzz run fuzz_jpeg fuzz/artifacts/fuzz_jpeg/crash-xxxxx

# Minimize crash input
cargo +nightly fuzz tmin fuzz_jpeg fuzz/artifacts/fuzz_jpeg/crash-xxxxx
```

### Corpus

Fuzzer builds a corpus of interesting inputs in `fuzz/corpus/<target>/`. You can seed it with real files:

```bash
mkdir -p fuzz/corpus/fuzz_jpeg
cp testdata/*.jpg fuzz/corpus/fuzz_jpeg/
```

## Known Limitations

Current version (0.1.0) has some documented limitations:

- **BigTIFF**: Files >4GB with 8-byte offsets not supported
- **Multi-page TIFF**: Only first IFD processed
- **Thumbnail extraction**: Not implemented yet
- **Some RAW formats**: Leica, Sigma, Phase One not yet supported
- **Value interpretation**: Enums like Orientation/Flash returned as numbers, not strings

See [AGENTS.md](AGENTS.md) for full architecture documentation.

## License

MIT OR Apache-2.0

## Acknowledgments

Tag definitions derived from [ExifTool](https://exiftool.org/) by Phil Harvey.

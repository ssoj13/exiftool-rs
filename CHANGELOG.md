# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added
- **MakerNotes field write**: FujiFilm IFD rebuild; IFD overlay for camera vendors plus Xiaomi/Oppo/OnePlus/Vivo/Realme/Huawei/Google, Phase One/Leaf, Leica (`LEICA CAMERA AG` and headerless), Hasselblad (own tags, not Sony), DJI FLOAT tags, GoPro GPMF same-size leaves. Overlay names match each vendor parser table. Olympus Equipment/CameraSettings/ImageProcessing/FocusInfo and Pentax LensInfo/AFInfo nested IFDs overlay in place. Canon CameraSettings/ShotInfo/FileInfo/ProcessingInfo int16 and AFInfo uint16 arrays overlay in place. Panasonic FaceDetect `FaceCount` overlays the first u16. Nikon encrypted overlay: ShotInfo Full-crypt and `NIKON_OFFSETS`, ColorBalance levels, LensData `LensIDNumber`. JPEG/RAF `write_metadata` rewrites original APP1 TIFF so MakerNotes are not dropped.
- **CR3 write** (`Cr3Writer`): ExifTool WriteQuickTime CR3 map — rewrite CMT1/CMT2/CMT4, optional XMP UUID, patch CTBO and stco/co64; keep `mdat` largesize header; CMT3 blob copy. `is_writable` true.
- **RAF write** (`RafWriter`): ExifTool `WriteRAF` — rewrite preview JPEG EXIF, 4-byte pad, fix header pointers, copy RAF directory + CFA from `nextPtr` at 0x5C. `is_writable` true.
- **NEF/NRW write** (`tiff_rewrite` + `NefWriter`): overlay IFD0 / ExifIFD / GPS; copy SubIFD trees, strips/tiles, MakerNotes/ICC blobs. Does not use `TiffWriter` (that path drops SubIFD/raw). CLI and Python `save()` dispatch both formats.
- **Nikon MakerNotes**: Type-3 IFD, decrypt (`ProcessNikonEncrypted` / `@xlat` including `NIKON_OFFSETS` piecewise), LensData 0100/0101/02xx, ColorBalance, ShotInfo per-model dispatch (D40–Z9). PreviewIFD offsets relocated.
- **EXR** via `exr-core` (`ssh://git@github.com/ssoj13/exr-rs.git`); crates.io `exr` removed.
- **JPEG 2000** via `jpg-rs` / HTJ2K `jph-rs` (SSH git).
- **JPEG** post-EOI trailer detect (AFCP / FotoStation / PhotoMechanic / Samsung / CanonVRD). **PNG** `zXIf` (zlib EXIF).
- **DICOM**, **FITS**, **7z** (unencoded + LZMA encoded header id 23), **ZIP/OOXML/ODF** readers.
- Golden fixtures from ExifTool `t/images` + `xtask/parity.py` vs Perl FileType.

### Changed
- TIFF-family FileType: unique magics first; generic TIFF classified by `DNGVersion`, extension, Make (`tiff_family`). Wrappers no longer steal TIFF magic.
- `Metadata::is_writable`: TIFF-family RAW, RAF, CR3, MP4/MOV, WAV/FLAC/MP3.
- Bootstrap build defaults to release; use `--debug` for debug builds.
- Removed `python release` subcommand from bootstrap scripts.

### Fixed
- RAF writer no longer treats 0x5C/0x60 as CFA offset/length.
- **TIFF-family RAW write** via `tiff_rewrite` (also used by `TiffWriter::write`): CR2 16-byte header (`WriteCR2`), ORF/RW2 magics, strip padding, BigTIFF (16-byte header / 20-byte IFDs), Sony A100 `FinishARW` (MRW + `A100DataOffset`).
- **HEIC write**: create an EXIF item when the file has none (`HeicWriter`).
- **MP4/MOV, WAV, FLAC, MP3**: `is_writable` matches existing writers (XMP UUID / tags).

### Docs
- Docs: `writing.md`, `formats.md`, `formats/raw.md`, `reading.md`, `python.md`, `AGENTS.md`, `plan2.md`, README.

### Added
- **5 New RAW Formats**: Sony ARW, Olympus ORF, Panasonic RW2, Pentax PEF, WebP
- **TiffConfig**: Unified configuration for TIFF-based RAW parsers
  - `format_name`, `allowed_magic`, `vendor` fields
  - All RAW parsers use `TiffParser::with_config()` - no code duplication
- **IfdReader::parse_header_with_magic()**: Support for non-standard TIFF magic bytes
  - Olympus ORF: IIRO (0x4F52), IIRS (0x5352)
  - Panasonic RW2: 0x55 magic
- **AttrValue::Group**: Nested metadata structures for MakerNotes sub-IFDs
- **Modular MakerNotes**: 9 vendor parsers with sub-IFD support
  - Canon, Nikon, Sony, Fujifilm, Olympus, Panasonic, Pentax, Samsung, Apple
- **Integration tests**: Real RAW file parsing tests (from rawsamples.ch)
- **Metadata API**: `is_camera_raw()` and `is_writable()` methods
  - Detects RAW by format name or Make tag (catches renamed files)
  - Used in CLI and Python for write protection
- **Python bindings**: `img.is_camera_raw` and `img.is_writable` properties

### Fixed
- **CLI error handling**: Clean error messages without stack traces
  - User-friendly messages with usage hints
  - Write protection for RAW formats with informative errors
- Integer overflow protection in IFD parsing with checked arithmetic
- PyO3 `.unwrap()` replaced with proper error propagation (`?` operator)
- Duplicate `build_exif_bytes` consolidated to `exiftool_formats::utils`
- Error message in `write_not_supported` now lists correct formats (JPEG, PNG, TIFF, DNG)
- File size limits (100MB) to prevent OOM attacks in format parsers/writers
- Dead code removed from hdr_writer.rs
- All Clippy warnings fixed across workspace:
  - Codegen template needless_borrow (1380+ generated warnings)
  - ptr_arg: `&PathBuf` → `&Path` in CLI and Python bindings
  - Unnecessary parentheses and needless_range_loop in HEIC parser

### Added
- `#[must_use]` annotations on core types (ByteOrder, RawValue, URational, SRational, AttrValue)
- MSRV (Minimum Supported Rust Version): 1.82
- AGENTS.md - Comprehensive architecture documentation for contributors
- Shared utilities module (`exiftool_formats::utils`) with:
  - `entry_to_attr()` - unified IFD to AttrValue conversion
  - `build_exif_bytes()` - unified EXIF serialization
  - `read_with_limit()` - safe file reading with size limits

### Documentation
- README.md - Added Known Limitations section
- plan1.md - Bug hunt report with prioritized fixes

## [0.1.0] - 2025-12-25

### Added

#### Python Bindings (`exiftool-py`)
- New crate `exiftool-py` with PyO3/maturin for Python integration
- `Image` class with Pythonic API:
  - Properties: `make`, `model`, `iso`, `fnumber`, `exposure_time`, `focal_length`, etc.
  - Dict-like access: `img["Tag"]`, `img.keys()`, `img.values()`, `img.items()`
  - Direct dict conversion: `dict(img)`
  - Context manager support: `with exif.open("photo.jpg") as img:`
  - Iteration: `for tag in img:`
- `Rational` class for EXIF rational values with `numerator`, `denominator`, `as_tuple()`
- `GPS` class with decimal degrees (`latitude`, `longitude`) and raw DMS access
- Exception hierarchy: `ExifError`, `FormatError`, `WriteError`, `TagError`
- `scan()` function for parallel batch processing with rayon
- `scan_dir()` for single directory scanning
- Type stubs (`.pyi`) for IDE autocomplete
- Python package structure with `__init__.py` re-exports

#### Build System
- `bootstrap.ps1` script for Windows:
  - `.\bootstrap.ps1 build` - Build Rust crates
  - `.\bootstrap.ps1 python` - Build Python wheel
  - `.\bootstrap.ps1 python dev` - Install in development mode
  - `.\bootstrap.ps1 python release` - Build release wheel
  - `.\bootstrap.ps1 test` - Run tests
  - `.\bootstrap.ps1 codegen` - Regenerate tag tables

#### Documentation
- Updated main `README.md` with Python bindings section
- Created `crates/exiftool-py/README.md` with usage examples
- Added crate to workspace structure diagram

### Core Library (existing)
- EXIF/XMP/IPTC metadata parsing
- 17 format support: JPEG, PNG, TIFF, DNG, HEIC, AVIF, CR2, CR3, NEF, ARW, ORF, RW2, PEF, RAF, WebP, EXR, HDR
- MakerNotes support (Canon, Nikon, Sony, Fujifilm, etc.)
- XMP parser with rdf:Bag/Seq/Alt
- CLI tool for reading/writing metadata
- Auto-generated tag tables from ExifTool (~2500+ tags)

## Installation

### Rust
```bash
cargo add exiftool-formats
```

### Python
```bash
pip install exiftool-py
```

### CLI
```bash
cargo install --path crates/exiftool-cli
```

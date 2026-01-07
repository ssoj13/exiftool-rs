# PLAN.md - Verified Implementation Status

Generated: 2026-01-06
Based on: Code audit against TODO.md and TODO8.md

## Summary

| Category | Claimed | Verified | Actual |
|----------|---------|----------|--------|
| Tests | 419 | 442 | **DONE** |
| RAW Preview | Not done | Done | **DONE** (CR3, CRW, RAF, NEF, ARW, PEF, ORF, RW2 via TIFF) |
| Composite Tags | Partial | Done | **DONE** (8 tags implemented) |
| CLI XML Output | Done | Done | **DONE** |
| CLI CSV Output | Done | Partial | Header per file, not unified |
| CLI Recursive | Done | Done | **DONE** |
| Thumbnail Write | Not done | Done | **DONE** (ExifWriter.set_thumbnail) |
| Fuzz Testing | Not started | Not done | **NOT DONE** |
| Benchmarks | Not started | Not done | **NOT DONE** |

---

## Phase 1: Core Infrastructure

### 1.1 RAW Preview/Thumbnail Extraction - **DONE**
- [x] CR2: via TiffParser IFD0 strips - **VERIFIED**
- [x] CR3: PRVW/THMB boxes in cr3.rs:258 - **VERIFIED**
- [x] CRW: tag parsing in crw.rs:292 - **VERIFIED**
- [x] RAF: embedded JPEG in raf.rs:71 - **VERIFIED**
- [x] NEF: PreviewImageStart/Length via Nikon MakerNotes - **VERIFIED** (makernotes/nikon.rs:172-181)
- [x] ARW: PreviewImageStart/Length via Sony MakerNotes - **VERIFIED** (makernotes/sony.rs:84-96)
- [x] PEF: PreviewImageStart/Length via Pentax MakerNotes - **VERIFIED** (makernotes/pentax.rs:190-191)
- [x] ORF/RW2: via TiffParser IFD0 strips - **VERIFIED** (tiff.rs handles all TIFF-based RAW)
- [x] CLI `-P` flag for preview extraction - **VERIFIED** (main.rs:extract_previews)
- [x] CLI `-T` flag for thumbnail extraction - **VERIFIED** (main.rs:extract_thumbnails)

### 1.2 Thumbnail Write/Update - **DONE**
- [x] ExifWriter.set_thumbnail() in exiftool-core/src/writer.rs:519+ - **VERIFIED**
- [x] IFD1 with JPEGInterchangeFormat/Length - **VERIFIED**
- [ ] PNG tEXt chunk embedding - **NOT VERIFIED** (no evidence in png_writer.rs)
- [ ] WebP EXIF chunk thumbnail - **NOT VERIFIED**
- [ ] HEIC thumbnail in meta box - **NOT VERIFIED**

### 1.3 ICC Profile Write/Embed - **NOT DONE**
- [x] ICC Profile parser exists (exiftool-icc crate) - **VERIFIED**
- [ ] JPEG APP2 ICC_PROFILE write - **NOT IMPLEMENTED**
- [ ] PNG iCCP chunk write - **NOT IMPLEMENTED**
- [ ] TIFF InterColorProfile tag write - **NOT IMPLEMENTED**
- [ ] CLI `--icc-profile` flag - **NOT IMPLEMENTED**

### 1.4 Composite Tags - **DONE** (8 of 10)
Location: crates/exiftool-formats/src/composite.rs

- [x] ImageSize: "WxH" format - **VERIFIED**
- [x] Megapixels: calculation - **VERIFIED**
- [x] ShutterSpeed: "1/xxx s" format - **VERIFIED**
- [x] Aperture: "f/x.x" format - **VERIFIED**
- [x] FocalLength35efl: 35mm equivalent - **VERIFIED**
- [x] GPSPosition: decimal degrees - **VERIFIED**
- [x] LensID: combined lens info - **VERIFIED**
- [x] Duration: HH:MM:SS format - **VERIFIED** (as DurationFormatted)
- [ ] GPSAltitude: with ref - **NOT IMPLEMENTED**
- [ ] DateTimeOriginal with SubSec - **NOT IMPLEMENTED**

CLI `-c/--composite` flag - **VERIFIED**

---

## Phase 3: RAW Format Enhancements - **NOT DONE**

### 3.1 CR3 Write Support - **NOT DONE**
- [ ] HEIF box modification
- [ ] XMP in uuid box update
- [ ] EXIF in Exif box update

### 3.2 NEF Encrypted Data - **NOT DONE**
- [ ] SerialNumber-based decryption key
- [ ] WhiteBalance decryption
- [ ] ColorBalance decryption

### 3.3 ARW Write Support - **NOT DONE**
- [ ] IFD0 EXIF update
- [ ] Sony MakerNotes preservation

### 3.4 DNG 1.6 Features - **NOT DONE**
- [ ] ProfileGainTableMap tag
- [ ] ProfileGainTableMap2 tag
- [ ] New OpcodeList3 operations
- [ ] Semantic masks support

---

## Phase 10: Infrastructure Enhancements

### 10.1 CLI Output Formats - **PARTIAL**
- [x] XML output (`-X`, `--xml`) - **VERIFIED** (xml_output.rs)
  - [x] RDF structure
  - [x] Namespace handling
  - [x] XML escaping
- [x] CSV output (`--csv`) - **PARTIAL**
  - [x] Header row
  - [x] Proper quoting
  - [ ] Multi-file unified header - **NOT IMPLEMENTED**
- [ ] HTML output (`--html`) - **NOT IMPLEMENTED**
- [ ] Tab-delimited output - **NOT IMPLEMENTED**

### 10.2 CLI Recursive Processing - **PARTIAL**
- [x] `-r/--recursive` flag - **VERIFIED**
- [x] Directory traversal (walkdir) - **VERIFIED**
- [x] Extension filtering (`-e`) - **VERIFIED**
- [x] Default extension list - **VERIFIED**
- [x] Progress indicator (file count) - **VERIFIED**
- [ ] Date filtering (`--newer`, `--older`) - **NOT IMPLEMENTED**
- [ ] Size filtering (`--minsize`, `--maxsize`) - **NOT IMPLEMENTED**
- [ ] Exclude patterns (`--exclude`) - **NOT IMPLEMENTED**
- [ ] Summary statistics - **NOT IMPLEMENTED**

### 10.3 CLI Geotag Support - **NOT DONE**
- [ ] `--geotag <gpx_file>` flag
- [ ] GPX file parsing
- [ ] KML file parsing
- [ ] Time matching algorithm
- [ ] GPS tag writing

### 10.4 CLI Time Operations - **NOT DONE**
- [ ] `--datetimeshift <delta>` flag
- [ ] Delta format parsing
- [ ] DateTime shifting
- [ ] `--filetimefrommeta` flag

### 10.5 Charset Support - **NOT DONE**
- [ ] exiftool-charset crate
- [ ] Latin-1 (ISO-8859-1)
- [ ] Windows-1252
- [ ] Shift-JIS
- [ ] UTF-16 LE/BE
- [ ] `--charset` CLI flag

### 10.6 API Enhancements - **NOT DONE**
- [ ] Async read support
- [ ] Memory-mapped file support
- [ ] Streaming parser mode
- [ ] Progress callbacks
- [ ] Cancellation tokens

---

## Phase 11: Testing & Quality

### 11.1 Test Infrastructure - **PARTIAL**
Current: **442 tests** (verified via `cargo test`)

| Crate | Tests |
|-------|-------|
| exiftool-cli | 2 |
| exiftool-core | 11 + 9 (proptest) |
| exiftool-formats | 354 + 14 (proptest) + 8 (raw) + 3 (round-trip) |
| exiftool-icc | 5 |
| exiftool-iptc | 4 |
| exiftool-tags | 7 |
| exiftool-xmp | 15 |
| doc tests | ~9 |

- [x] Unit tests - **DONE**
- [x] Property-based tests (proptest) - **VERIFIED** (proptest_parsers.rs, proptest_formats.rs)
- [x] Round-trip tests - **VERIFIED** (round_trip.rs - 3 tests)
- [ ] Golden file tests - **NOT DONE**
- [ ] Fuzz testing (cargo-fuzz) - **NOT DONE** (no fuzz/ directory)
- [ ] Benchmark suite (criterion) - **NOT DONE** (not in Cargo.toml)

### 11.2 Test File Collection - **NOT DONE**
- [ ] Download script
- [ ] RAW samples from raw.pixls.us
- [ ] Video/audio samples

### 11.3 Documentation - **PARTIAL**
Location: docs/src/

- [x] mdBook structure - **VERIFIED**
- [x] intro.md, cli.md, reading.md, writing.md
- [x] formats/ directory (images, raw, video, audio)
- [x] architecture/ directory
- [x] python/ directory
- [ ] Complete API reference - **PARTIAL**
- [ ] Tag reference documentation - **NOT DONE**
- [ ] Migration guide from ExifTool - **NOT DONE**

---

## Phase 12: Python Bindings - **PARTIAL**

Location: crates/exiftool-py/

- [x] PyO3 bindings - **VERIFIED**
- [x] Type stubs (.pyi) - **VERIFIED** (python/exiftool_py/__init__.pyi)
- [x] Basic read support - **VERIFIED**
- [ ] Write support - **NOT VERIFIED**
- [ ] Batch operations - **NOT VERIFIED**
- [ ] Async support - **NOT DONE**

---

## Priority Implementation Order

### P0 - Quick Fixes (1-2 days each)
1. [ ] GPSAltitude composite tag (composite.rs)
2. [ ] DateTimeOriginal with SubSec composite tag
3. [ ] Multi-file unified CSV header
4. [ ] `--exclude` pattern for recursive

### P1 - Medium Effort (1 week each)
5. [ ] Fuzz testing setup (cargo-fuzz targets)
6. [ ] Benchmark suite (criterion)
7. [ ] Golden file test framework
8. [ ] `--newer/--older` date filters
9. [ ] `--minsize/--maxsize` size filters

### P2 - Larger Features (2+ weeks)
10. [ ] Geotag from GPX
11. [ ] Time shift operations
12. [ ] Charset support (encoding_rs integration)
13. [ ] HTML output format
14. [ ] ICC Profile write support

### P3 - RAW Write (Complex)
15. [ ] ARW write support
16. [ ] CR3 write support
17. [ ] NEF encrypted data decryption
18. [ ] DNG 1.6 features

---

## Discrepancies Found

| Item | TODO.md Status | Actual |
|------|----------------|--------|
| Phase 1 | 100% | ~95% (missing ICC write, 2 composite tags) |
| Tests | 419 | 442 |
| RAW Preview | "Sprint 2: TODO" | Actually DONE |
| Thumbnail Write | "TODO" in Phase 1.2 | ExifWriter.set_thumbnail EXISTS |
| Fuzz Testing | "Not started" | Correct |
| Benchmarks | "Not started" | Correct |

---

## Files Changed Analysis

Key implementation files:
- `crates/exiftool-cli/src/main.rs` - CLI with -T, -P, -r, -e, -c, -X flags
- `crates/exiftool-cli/src/xml_output.rs` - XML output implementation
- `crates/exiftool-formats/src/composite.rs` - 8 composite tags
- `crates/exiftool-formats/src/tiff.rs` - Preview extraction for TIFF-based RAW
- `crates/exiftool-formats/src/cr3.rs` - CR3 preview extraction
- `crates/exiftool-formats/src/raf.rs` - RAF preview extraction
- `crates/exiftool-core/src/writer.rs` - Thumbnail write support

---

## Recommended Next Steps

1. **Update TODO.md** - Mark RAW preview extraction as DONE
2. **Add missing composite tags** - GPSAltitude, DateTimeOriginal with SubSec
3. **Setup fuzz testing** - Create fuzz/ directory with cargo-fuzz targets
4. **Add benchmarks** - Add criterion to dev-dependencies
5. **Unified CSV header** - Track all columns across multi-file output

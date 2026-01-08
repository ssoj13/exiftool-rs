# exiftool-rs - Audit & Improvement Plan

## Overview

Comparison with Perl ExifTool v13.44 reference implementation.

**Current Status:**
- Rust LOC: ~26K across 9 crates
- Formats: ~90 (vs 200+ in Perl)
- MakerNotes vendors: 20 (vs 50+ in Perl)
- EXIF tags: ~2500

---

## 1. Core Algorithms & Data Types

### 1.1 EXIF Format Types (exiftool-core/src/format.rs)

- [x] Add EXIF 3.0 UTF8 type (type ID 129)
- [x] Types 1-18 implemented
- [x] BigTIFF types (16-18) implemented

### 1.2 IFD Parser (exiftool-core/src/ifd.rs)

- [x] Standard TIFF IFD parsing
- [x] BigTIFF header detection (magic 43)
- [x] BigTIFF IFD reading (8-byte offsets, 20-byte entries)
- [ ] Multi-page TIFF navigation (follow IFD chain beyond IFD0/IFD1)
- [ ] Thumbnail extraction from IFD1
- [ ] Sub-IFD recursive parsing improvements

### 1.3 Byte Order (exiftool-core/src/byte_order.rs)

- [x] Little-endian (II)
- [x] Big-endian (MM)
- [x] All numeric types supported

### 1.4 Value Interpretation (exiftool-tags/src/interp.rs)

- [x] Basic enum lookups (Orientation, MeteringMode, etc.)
- [x] Complete all EXIF enum interpretations (20+ tags covered)
- [x] Add unit formatting (focal length "mm", aperture "f/x.x")
- [x] Bitfield decoding for complex tags (Flash)

---

## 2. File Format Coverage

### 2.1 Image Formats - Present

- [x] JPEG (R/W)
- [x] PNG (R/W)
- [x] TIFF (R/W)
- [x] GIF (R)
- [x] BMP (R)
- [x] ICO (R)
- [x] TGA (R)
- [x] PCX (R)
- [x] SGI (R)
- [x] PNM (R)
- [x] SVG (R)
- [x] EPS (R)
- [x] AI (R)

### 2.2 Image Formats - Missing

- [ ] PSD/PSB (Adobe Photoshop) - HIGH PRIORITY
- [ ] PDF (metadata extraction) - HIGH PRIORITY
- [ ] DjVu
- [ ] BPG (Better Portable Graphics)
- [ ] FLIF (Free Lossless Image Format)
- [ ] Flash/SWF

### 2.3 Modern Image Formats - Present

- [x] HEIC/HEIF (R/W)
- [x] AVIF (R/W via HEIC)
- [x] WebP (R/W)
- [x] JXL (JPEG XL) (R)
- [x] JP2 (JPEG 2000) (R)
- [x] EXR (R/W)
- [x] HDR (R/W)
- [x] DPX (R)

### 2.4 RAW Formats - Present

- [x] CR2 (Canon)
- [x] CR3 (Canon)
- [x] CRW (Canon legacy)
- [x] NEF (Nikon)
- [x] NRW (Nikon)
- [x] ARW (Sony)
- [x] SRF (Sony)
- [x] ORF (Olympus)
- [x] RW2 (Panasonic)
- [x] RWL (Leica via Panasonic)
- [x] PEF (Pentax)
- [x] RAF (Fujifilm)
- [x] SRW (Samsung)
- [x] ERF (Epson)
- [x] MEF (Mamiya)
- [x] MRW (Minolta)
- [x] MOS (Leaf)
- [x] X3F (Sigma)
- [x] IIQ (Phase One)
- [x] DCR (Kodak)
- [x] BRAW (Blackmagic)
- [x] FFF (Hasselblad)
- [x] R3D (RED)

### 2.5 RAW Formats - Missing

- [ ] DNG improvements (separate detailed module)
- [ ] 3FR (Hasselblad legacy)
- [ ] KDC (Kodak legacy)
- [ ] RWZ (Rawzor compressed)

### 2.6 Video Formats - Present

- [x] MP4/M4V
- [x] MKV/WebM
- [x] AVI
- [x] FLV
- [x] MXF
- [x] RM (RealMedia)
- [x] MPEG-TS
- [x] ASF/WMV

### 2.7 Video Formats - Missing

- [ ] QuickTime improvements (detailed atom parsing)
- [ ] H264/H265 NAL unit parsing
- [ ] DV (Digital Video)

### 2.8 Audio Formats - Present

- [x] MP3/ID3 (R/W)
- [x] FLAC (R)
- [x] WAV (R)
- [x] AIFF (R)
- [x] OGG (R)
- [x] AAC (R)
- [x] ALAC (R)
- [x] APE (R)
- [x] WavPack (R)
- [x] DSF/DFF (R)
- [x] TAK (R)
- [x] MIDI (R)
- [x] Audible (R)
- [x] AU (R)

### 2.9 Archive/Document Formats - Missing

- [ ] ZIP (metadata)
- [ ] 7Z (metadata)
- [ ] RAR (metadata)
- [ ] Torrent (metadata)
- [ ] GZIP (metadata)

### 2.10 Specialized Formats - Missing

- [ ] DICOM (medical imaging)
- [ ] FITS (astronomy)
- [ ] EXE/DLL (PE resources)
- [ ] Font files (TTF/OTF)
- [ ] HTML (meta tags)
- [ ] InDesign

---

## 3. MakerNotes Coverage

### 3.1 Implemented Vendors (20)

- [x] Apple
- [x] Canon
- [x] Casio
- [x] DJI
- [x] Fujifilm
- [x] Google
- [x] GoPro
- [x] Hasselblad
- [x] Huawei
- [x] Kodak
- [x] Minolta
- [x] Nikon
- [x] Olympus
- [x] Panasonic (+ Leica)
- [x] Pentax
- [x] Phase One
- [x] Ricoh
- [x] Samsung
- [x] Sigma
- [x] Sony
- [x] Xiaomi

### 3.2 Missing Vendors (30+)

HIGH PRIORITY:
- [ ] Leica (dedicated, not via Panasonic)
- [ ] Motorola (mobile)
- [ ] OnePlus (mobile)
- [ ] Oppo (mobile)
- [ ] Vivo (mobile)

MEDIUM PRIORITY:
- [ ] GE (General Electric)
- [ ] HP (Hewlett-Packard)
- [ ] Nintendo (Switch screenshots)
- [ ] Parrot (drones)
- [ ] Reconyx (trail cameras)
- [ ] Sanyo

LOW PRIORITY:
- [ ] Agfa
- [ ] Epson
- [ ] Foveon
- [ ] JVC
- [ ] Kyocera
- [ ] Leaf
- [ ] Mamiya
- [ ] Noritsu
- [ ] Polaroid
- [ ] Rollei
- [ ] SeaLife
- [ ] Sinar
- [ ] Vivitar

THERMAL/SPECIALIZED:
- [ ] FLIR (thermal imaging)
- [ ] InfiRay (thermal)

---

## 4. Metadata Standards

### 4.1 EXIF

- [x] EXIF 2.32 tags
- [ ] EXIF 3.0 new tags and UTF8 type
- [x] GPS IFD
- [x] Interoperability IFD
- [ ] Complete PrintConv for all enums

### 4.2 XMP

- [x] Basic XMP parsing (quick-xml)
- [x] RDF structures (Bag, Seq, Alt)
- [x] Common namespaces (dc, exif, photoshop, xmp)
- [ ] XMP sidecar file support
- [ ] XMP-IPTC Extension schema
- [ ] Custom namespace registration

### 4.3 IPTC

- [x] IPTC-IIM parsing
- [x] Common fields (caption, keywords, copyright)
- [ ] Extended IPTC fields
- [ ] IPTC digest validation

### 4.4 ICC Profiles

- [x] ICC profile embedding/extraction
- [x] Profile description reading
- [ ] Color space conversion info
- [ ] Profile validation

---

## 5. Character Set Support

### 5.1 Currently Supported

- [x] UTF-8
- [x] ASCII
- [x] UTF-16 (basic, in Unicode EXIF type)

### 5.2 Missing Charsets

HIGH PRIORITY (Japanese cameras):
- [ ] ShiftJIS (Shift_JIS)
- [ ] EUC-JP

MEDIUM PRIORITY:
- [ ] Latin1 (ISO-8859-1)
- [ ] Latin2 (ISO-8859-2)
- [ ] Windows-1252

LOW PRIORITY:
- [ ] Cyrillic variants
- [ ] Chinese (GB2312, Big5)
- [ ] Korean (EUC-KR)
- [ ] Arabic
- [ ] Hebrew
- [ ] Thai

---

## 6. Python API Improvements

### 6.1 Current Features (Complete)

- [x] open(path) -> Image
- [x] scan(pattern, parallel, ignore_errors) -> ScanResult
- [x] scan_dir(directory, extensions, parallel) -> ScanResult
- [x] Dict-like access (img[tag], img.get(), del img[tag])
- [x] Properties (make, model, iso, fnumber, exposure_time, etc.)
- [x] Setters for common tags
- [x] get_interpreted() - human-readable values
- [x] get_display() - formatted with units
- [x] to_dict() - convert to Python dict
- [x] from_bytes() - parse from memory
- [x] save() - write changes
- [x] shift_time() - adjust timestamps
- [x] geotag() - GPS from GPX
- [x] add_composite() - calculated tags
- [x] set_icc_from_file() - embed ICC profile
- [x] Context manager support
- [x] Iteration support

### 6.2 Missing Features

HIGH PRIORITY:
- [x] Type stubs (.pyi files) for IDE support
- [x] set_gps(lat, lon, alt) - direct GPS setting
- [x] copy_tags(source, tags) - copy tags between images
- [x] strip_metadata() / clear_all() - remove all metadata

MEDIUM PRIORITY:
- [x] extract_thumbnail() -> bytes - explicit thumbnail extraction (via .thumbnail property)
- [x] extract_preview() -> bytes - explicit preview extraction (via .preview property)
- [ ] validate() -> List[Warning] - metadata validation
- [ ] batch_write(images, parallel) - batch save operation
- [ ] async API (open_async, scan_async)

LOW PRIORITY:
- [ ] compare(other) - diff two images' metadata
- [ ] to_json() - direct JSON export
- [ ] to_csv_row() - for batch exports
- [ ] Plugin/extension system

### 6.3 Classes to Add

- [ ] ExifTag class (name, value, interpreted, group)
- [ ] ValidationWarning class
- [ ] MetadataDiff class

---

## 7. CLI Improvements

### 7.1 Current Features

- [x] Read metadata (all tags or specific)
- [x] Output formats (JSON, CSV, HTML)
- [x] Write metadata (-t Tag=Value)
- [x] Time shifting (--shift)
- [x] Geotagging (--geotag)
- [x] ICC profiles (--icc)
- [x] Recursive (-r)
- [x] Extension filter (-e)
- [x] Exclude patterns (-x)
- [x] Thumbnail extraction (-T)
- [x] Preview extraction (-P)

### 7.2 Missing Features

- [ ] Copy tags between files (--copy-from)
- [ ] Remove all metadata (--strip)
- [ ] Rename by metadata pattern (--rename)
- [ ] Conditional processing (--if)
- [ ] Config file support
- [ ] Verbose mode with progress

---

## 8. Testing & Quality

### 8.1 Current Testing

- [x] Unit tests in each crate
- [x] Property-based tests (proptest) in exiftool-core
- [x] Fuzz targets (6 parsers)
- [x] Criterion benchmarks

### 8.2 Testing Improvements

- [ ] Test images for each MakerNotes vendor
- [ ] Roundtrip tests (read -> write -> read)
- [ ] Comparison tests against Perl ExifTool output
- [ ] Edge case test suite (corrupted files, truncated data)
- [ ] Performance regression tests
- [ ] Memory usage tests

---

## 9. Documentation

### 9.1 Current Docs

- [x] README.md
- [x] Rustdoc comments
- [x] Python docstrings

### 9.2 Documentation Improvements

- [ ] Differences from Perl ExifTool document
- [ ] Supported formats matrix
- [ ] MakerNotes coverage table
- [ ] Migration guide from pyexiftool
- [ ] API reference website
- [ ] Examples repository

---

## 10. Implementation Priority

### Phase 1 - Critical (This Sprint)

1. [x] EXIF 3.0 UTF8 type (format.rs)
2. [x] Python .pyi type stubs
3. [x] set_gps() Python method
4. [x] strip_metadata() Python method

### Phase 2 - High Priority

5. [ ] PSD/PSB format parser
6. [ ] PDF metadata extraction
7. [ ] Leica MakerNotes (dedicated)
8. [ ] ShiftJIS charset support
9. [ ] Multi-page TIFF navigation

### Phase 3 - Medium Priority

10. [ ] Mobile vendor MakerNotes (Motorola, OnePlus, Oppo, Vivo)
11. [ ] copy_tags() Python method
12. [ ] validate() Python method
13. [ ] Complete EXIF enum interpretations
14. [ ] XMP sidecar support

### Phase 4 - Low Priority

15. [ ] Archive formats (ZIP, 7Z)
16. [ ] Specialized formats (DICOM, FITS)
17. [ ] Remaining charsets
18. [ ] Async Python API
19. [ ] Remaining MakerNotes vendors

---

## Progress Tracking

**Last Updated:** 2026-01-07 (Phase 1 Complete)

| Category | Done | Total | % |
|----------|------|-------|---|
| Core Types | 18 | 19 | 95% |
| Image Formats | 13 | 19 | 68% |
| Modern Formats | 8 | 8 | 100% |
| RAW Formats | 23 | 27 | 85% |
| Video Formats | 8 | 11 | 73% |
| Audio Formats | 14 | 14 | 100% |
| MakerNotes | 20 | 50+ | 40% |
| Python API | 25 | 30 | 83% |

---

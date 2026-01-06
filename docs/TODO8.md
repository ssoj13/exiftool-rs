# TODO8: Phases 1, 3, 10, 11 - Detailed Implementation Plan

## Legend
- [ ] Not started
- [~] In progress
- [x] Completed

---

## Phase 1: Core Infrastructure Completion

### 1.1 RAW Preview/Thumbnail Extraction
- [ ] CR2: Extract embedded JPEG from StripOffsets/StripByteCounts in IFD1
- [ ] NEF: Extract JpgFromRaw tag (0x0201) from SubIFD
- [ ] ARW: Extract PreviewImage from MakerNotes or IFD1
- [ ] RAF: Extract embedded JPEG after header
- [ ] ORF: Extract PreviewImage from MakerNotes
- [ ] RW2: Extract JpgFromRaw from Panasonic tags
- [ ] PEF: Extract PreviewImage from IFD1
- [ ] DNG: Extract PreviewImage from SubIFD
- [ ] CR3: Extract preview from PRVW box (HEIF)
- [ ] Create unified `extract_preview()` API in traits.rs
- [ ] Add CLI flag: `--preview` to extract preview image
- [ ] Add CLI flag: `--thumbnail` to extract thumbnail

### 1.2 Thumbnail Write/Update
- [ ] JPEG: Update APP1 EXIF IFD1 thumbnail
- [ ] TIFF: Update IFD1 thumbnail strips
- [ ] PNG: Embed thumbnail in tEXt chunk (base64)
- [ ] WebP: Update EXIF chunk thumbnail
- [ ] HEIC: Update thumbnail in meta box
- [ ] Create `write_thumbnail()` API

### 1.3 ICC Profile Write/Embed
- [ ] JPEG: Write APP2 ICC_PROFILE marker
- [ ] PNG: Write iCCP chunk
- [ ] TIFF: Write InterColorProfile tag (0x8773)
- [ ] WebP: Write ICCP chunk
- [ ] HEIC: Write colr box with ICC profile
- [ ] Create `write_icc_profile()` API
- [ ] Add CLI flag: `--icc-profile <file>` to embed profile

### 1.4 Composite Tags
- [x] Create composite.rs module
- [x] ImageSize: Combine ImageWidth + ImageHeight to "WxH"
- [x] Megapixels: Calculate from ImageWidth * ImageHeight / 1000000
- [x] ShutterSpeed: Convert ExposureTime to "1/xxx s" format
- [x] Aperture: Convert FNumber to "f/x.x" format
- [x] FocalLength35efl: Calculate 35mm equivalent from FocalLength + CropFactor
- [x] GPSPosition: Combine GPSLatitude + GPSLongitude to decimal degrees
- [x] LensID: Combine LensType + LensModel + FocalLength
- [x] Duration: Format video duration as HH:MM:SS
- [x] CLI flag `-c/--composite` to enable
- [ ] GPSAltitude: Combine GPSAltitude + GPSAltitudeRef
- [ ] DateTimeOriginal: Combine date + SubSecTimeOriginal

---

## Phase 3: RAW Format Enhancements

### 3.1 CR3 Write Support
- [ ] Research CR3 structure (HEIF-based with CRAW box)
- [ ] Implement HEIF box modification
- [ ] Update XMP in uuid box
- [ ] Update EXIF in Exif box
- [ ] Preserve CRAW/mdat structure
- [ ] Test with Canon EOS R/R5/R6 files

### 3.2 NEF Encrypted Data
- [ ] Research Nikon encryption (D3/D300 and later)
- [ ] Implement SerialNumber-based decryption key
- [ ] Decrypt WhiteBalance data
- [ ] Decrypt ColorBalance data
- [ ] Add `--decrypt-nef` flag

### 3.3 ARW Write Support
- [ ] Analyze ARW structure (TIFF-based)
- [ ] Implement IFD0 EXIF update
- [ ] Preserve Sony MakerNotes structure
- [ ] Test with Sony A7/A9 files

### 3.4 DNG 1.6 Features
- [ ] Research DNG 1.6 spec additions
- [ ] Support ProfileGainTableMap tag
- [ ] Support ProfileGainTableMap2 tag
- [ ] Support new OpcodeList3 operations
- [ ] Semantic masks support
- [ ] Update DNG version detection

---

## Phase 10: Infrastructure Enhancements

### 10.1 CLI Output Formats
- [x] XML output format (`-X` or `--xml`)
  - [x] Create xml_output.rs module
  - [x] XML declaration and root element
  - [x] Tag grouping by category (EXIF, IPTC, XMP)
  - [x] Proper XML escaping
  - [x] UTF-8 encoding declaration
- [x] CSV output format (`--csv`) - basic implementation exists
  - [x] Header row with tag names
  - [x] Proper CSV escaping (quotes, commas)
  - [ ] Multi-file unified header (needs superset of columns)
- [ ] HTML output format (`--html`)
  - [ ] Create html_output.rs module
  - [ ] Table-based layout
  - [ ] CSS styling
  - [ ] Image thumbnail embedding (optional)
- [ ] Tab-delimited output (`--tab`)
- [ ] PHP serialized output (`--php`) - for compatibility

### 10.2 CLI Recursive Processing
- [x] Add `-r` / `--recursive` flag
- [x] Directory traversal with walkdir crate
- [x] File filtering by extension (`-e jpg,png`)
- [x] Default extension list for known media formats
- [x] Progress indicator for batch operations
- [ ] File filtering by date (`--newer`, `--older`)
- [ ] File filtering by size (`--minsize`, `--maxsize`)
- [ ] Exclude patterns (`--exclude`)
- [ ] Summary statistics at end

### 10.3 CLI Geotag Support
- [ ] Add `--geotag <gpx_file>` flag
- [ ] GPX file parsing
  - [ ] Parse <trkpt> elements
  - [ ] Extract lat, lon, ele, time
- [ ] KML file parsing (optional)
- [ ] Time matching algorithm
  - [ ] Exact match
  - [ ] Interpolation between points
  - [ ] Time offset (`--geotime`)
- [ ] Write GPS tags to images
  - [ ] GPSLatitude, GPSLatitudeRef
  - [ ] GPSLongitude, GPSLongitudeRef
  - [ ] GPSAltitude, GPSAltitudeRef
  - [ ] GPSDateStamp, GPSTimeStamp

### 10.4 CLI Time Operations
- [ ] Add `--datetimeshift <delta>` flag
- [ ] Parse delta format: "+HH:MM:SS" or "-HH:MM:SS"
- [ ] Shift DateTimeOriginal
- [ ] Shift CreateDate
- [ ] Shift ModifyDate
- [ ] Add `--datetimefromfilename` flag
- [ ] Add `--filetimefrommeta` flag (set file mtime from EXIF)

### 10.5 Charset Support
- [ ] Create exiftool-charset crate
- [ ] UTF-8 (default, already works)
- [ ] Latin-1 (ISO-8859-1) encoding/decoding
- [ ] Windows-1252 encoding/decoding
- [ ] Shift-JIS (Japanese) via encoding_rs
- [ ] EUC-JP (Japanese) via encoding_rs
- [ ] GB2312/GBK (Chinese) via encoding_rs
- [ ] Big5 (Chinese Traditional) via encoding_rs
- [ ] EUC-KR (Korean) via encoding_rs
- [ ] UTF-16 LE/BE encoding/decoding
- [ ] MacRoman encoding/decoding
- [ ] Cyrillic (Windows-1251, KOI8-R)
- [ ] Add `--charset <encoding>` CLI flag
- [ ] Auto-detect encoding from BOM

### 10.6 API Enhancements
- [ ] Async read support (tokio feature flag)
- [ ] Memory-mapped file support (memmap2 crate)
- [ ] Streaming parser mode for large files
- [ ] Progress callbacks for long operations
- [ ] Cancellation tokens

---

## Phase 11: Testing & Quality

### 11.1 Test File Collection
- [ ] Create tests/samples/ directory structure
- [ ] Download RAW samples from raw.pixls.us:
  - [ ] Canon CR2 (5D, 6D, 7D series)
  - [ ] Canon CR3 (EOS R series)
  - [ ] Nikon NEF (D850, Z6, Z7)
  - [ ] Sony ARW (A7, A9 series)
  - [ ] Fuji RAF (X-T series)
  - [ ] Olympus ORF (E-M1, E-M5)
  - [ ] Panasonic RW2 (GH5, S1)
  - [ ] Pentax PEF (K-1, K-3)
  - [ ] Leica RWL (M series)
  - [ ] Hasselblad 3FR/FFF
  - [ ] Phase One IIQ
  - [ ] Sigma X3F
- [ ] Download image samples:
  - [ ] JPEG with full EXIF/IPTC/XMP
  - [ ] PNG with tEXt/iTXt
  - [ ] WebP with EXIF/XMP
  - [ ] HEIC from iPhone
  - [ ] AVIF samples
  - [ ] JPEG XL samples
  - [ ] JPEG 2000 samples
- [ ] Download video samples:
  - [ ] MP4 with GPS (GoPro, DJI)
  - [ ] MOV from iPhone
  - [ ] MKV with chapters
  - [ ] MXF broadcast
  - [ ] R3D from RED camera
  - [ ] BRAW from Blackmagic
- [ ] Download audio samples:
  - [ ] MP3 with ID3v2.4
  - [ ] FLAC with Vorbis comments
  - [ ] M4A with iTunes tags
  - [ ] WAV with INFO chunk
- [ ] Create download_samples.ps1 script

### 11.2 Golden File Tests
- [ ] Create tests/golden/ directory
- [ ] Generate expected output with exiftool:
  - [ ] `exiftool -j <file> > expected.json`
- [ ] Create golden_test.rs
- [ ] Compare our output with exiftool output
- [ ] Document known differences
- [ ] Threshold for acceptable differences
- [ ] CI integration for golden tests

### 11.3 Round-trip Tests
- [ ] JPEG: read -> modify -> write -> read -> verify
- [ ] TIFF: read -> modify -> write -> read -> verify
- [ ] PNG: read -> modify -> write -> read -> verify
- [ ] WebP: read -> modify -> write -> read -> verify
- [ ] HEIC: read -> modify -> write -> read -> verify
- [ ] NEF: read -> modify -> write -> read -> verify
- [ ] RAF: read -> modify -> write -> read -> verify
- [ ] Verify no data corruption
- [ ] Verify all tags preserved

### 11.4 Fuzz Testing
- [ ] Add cargo-fuzz to project
- [ ] Create fuzz targets:
  - [ ] fuzz_jpeg_parser
  - [ ] fuzz_tiff_parser
  - [ ] fuzz_png_parser
  - [ ] fuzz_heic_parser
  - [ ] fuzz_mp4_parser
  - [ ] fuzz_id3_parser
  - [ ] fuzz_xmp_parser
- [ ] Run fuzz tests for 1M+ iterations
- [ ] Fix any crashes found
- [ ] Add crash inputs to regression tests

### 11.5 Property-based Tests
- [ ] Add proptest to dev-dependencies
- [ ] Test EXIF round-trip with arbitrary values
- [ ] Test XMP round-trip with arbitrary strings
- [ ] Test GPS coordinate encoding/decoding
- [ ] Test rational number encoding
- [ ] Test Unicode string handling

### 11.6 Benchmark Suite
- [ ] Add criterion to dev-dependencies
- [ ] Benchmark JPEG parsing (various sizes)
- [ ] Benchmark TIFF parsing
- [ ] Benchmark RAW parsing (CR2, NEF, ARW)
- [ ] Benchmark video parsing (MP4, MKV)
- [ ] Benchmark XMP parsing
- [ ] Benchmark metadata writing
- [ ] Create benchmark comparison chart
- [ ] CI integration for performance regression

### 11.7 Documentation
- [ ] Complete rustdoc for all public APIs
- [ ] Create docs/USER_GUIDE.md
  - [ ] Installation instructions
  - [ ] Basic usage examples
  - [ ] Advanced usage
  - [ ] Tag reference
- [ ] Create docs/MIGRATION.md (from ExifTool)
  - [ ] Command-line flag mapping
  - [ ] Output format differences
  - [ ] Missing features
- [ ] Create docs/FORMAT_SUPPORT.md
  - [ ] Complete format matrix
  - [ ] Read/write capabilities
  - [ ] Known limitations
- [ ] Create docs/API_REFERENCE.md
  - [ ] Rust API examples
  - [ ] Python API examples
- [ ] Generate tag reference from code

---

## Execution Order

### Sprint 1: Quick Wins
1. [x] CLI XML output
2. [x] CLI CSV output (basic)
3. [x] CLI recursive processing
4. [x] Composite tags (basic set)

### Sprint 2: RAW Previews
5. [ ] CR2 preview extraction
6. [ ] NEF preview extraction
7. [ ] ARW preview extraction
8. [ ] Other RAW previews
9. [ ] Unified preview API

### Sprint 3: Testing Infrastructure
10. [ ] Download test samples script
11. [ ] Golden file test framework
12. [ ] Fuzz testing setup
13. [ ] Benchmark suite

### Sprint 4: Advanced Features
14. [ ] Geotag from GPX
15. [ ] Time shift operations
16. [ ] Charset support
17. [ ] ICC profile write

### Sprint 5: RAW Write
18. [ ] ARW write support
19. [ ] CR3 write support (complex)
20. [ ] NEF encrypted data
21. [ ] DNG 1.6 features

### Sprint 6: Documentation
22. [ ] User guide
23. [ ] Migration guide
24. [ ] API reference
25. [ ] Format support matrix

---

## Notes

- All changes must pass existing 339 tests
- Add tests for every new feature
- Update main TODO.md after each section complete
- Commit after each completed item

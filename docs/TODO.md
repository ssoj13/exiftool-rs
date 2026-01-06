# ExifTool-RS: Full Port TODO

## Legend
- [ ] Not started
- [x] Completed
- [~] Partial / In Progress

---

## Phase 0: Current State Audit

### 0.1 Existing Crates
- [x] `exiftool-core` - IFD reader/writer, byte order, formats
- [x] `exiftool-attrs` - Typed attribute storage
- [x] `exiftool-tags` - Auto-generated tag definitions
- [x] `exiftool-formats` - Format parsers
- [x] `exiftool-xmp` - XMP parser
- [x] `exiftool-cli` - CLI tool
- [x] `exiftool-py` - Python bindings
- [x] `xtask` - Build automation

### 0.2 Existing Format Support
- [x] JPEG (read/write)
- [x] PNG (read/write)
- [x] TIFF (read/write)
- [x] DNG (read/write via TIFF)
- [x] HEIC/HEIF (read)
- [x] AVIF (read via HEIC)
- [x] CR2 Canon (read)
- [x] CR3 Canon (read)
- [x] NEF Nikon (read/write)
- [x] ARW Sony (read)
- [x] ORF Olympus (read)
- [x] RW2 Panasonic (read)
- [x] PEF Pentax (read)
- [x] RAF Fujifilm (read/write)
- [x] WebP (read)
- [x] OpenEXR (read/write)
- [x] Radiance HDR (read/write)

### 0.3 Existing MakerNotes
- [x] Canon
- [x] Nikon
- [x] Sony
- [x] Fujifilm
- [x] Olympus
- [x] Panasonic
- [x] Pentax
- [x] Samsung
- [x] Apple

---

## Phase 1: Core Infrastructure (Priority: CRITICAL)

### 1.1 IPTC Support
- [x] Create `exiftool-iptc` crate
- [x] Parse IPTC-IIM (APP13 in JPEG)
- [x] IPTC record types (1:xx, 2:xx, etc.)
- [x] IPTC tag definitions (~70 tags)
- [x] IPTC write support
- [x] Integration with Metadata struct

### 1.2 XMP Write Support
- [x] XMP serialization (Attrs -> XML)
- [x] RDF structure generation
- [x] Namespace handling (dc, xmp, photoshop, etc.)
- [x] XMP sidecar file support (.xmp)
- [x] XmpSidecar: read/write/merge operations

### 1.3 ICC Profile Support
- [x] Create `exiftool-icc` crate
- [x] Parse ICC profile header (128 bytes)
- [x] Tag table parsing (all tag types)
- [x] Color space detection
- [x] Profile description extraction (desc, mluc)
- [x] Integrated into JPEG parser
- [ ] Profile write/embed support (future)

### 1.4 Value Interpretation System
- [x] Create value interpretation framework (interp.rs)
- [x] Enum value -> string mapping (~25 tag types)
- [x] Units conversion (exposure time, f-number, focal length)
- [x] GPS coordinate formatting
- [x] Flash bitmask interpretation
- [ ] Composite tag calculation (future)
- [ ] Full PrintConv coverage (future)

### 1.5 Thumbnail Support
- [x] EXIF thumbnail extraction (IFD1)
- [x] JPEG embedded thumbnail
- [ ] RAW preview extraction
- [ ] Thumbnail write/update

### 1.6 BigTIFF Support
- [x] 8-byte offset handling
- [x] BigTIFF header detection (0x002B)
- [x] Large file (>4GB) support
- [x] Update TiffParser for BigTIFF

### 1.7 Multi-page TIFF
- [x] IFD chain traversal
- [x] Page enumeration
- [x] Per-page metadata access

---

## Phase 2: Image Formats (Priority: HIGH)

### 2.1 Simple Image Formats
- [x] GIF parser
  - [x] GIF87a/GIF89a detection
  - [x] Comment extension
  - [x] Application extension (XMP)
  - [x] Animation metadata
- [x] BMP parser
  - [x] BITMAPFILEHEADER
  - [x] BITMAPINFOHEADER variants
  - [x] Color depth detection
- [x] ICO/CUR parser
  - [x] Icon directory
  - [x] Multiple resolutions
- [x] PPM/PGM/PBM/PAM (Netpbm)
  - [x] P1-P7 formats (ASCII/binary)
  - [x] Comment parsing
  - [x] PAM (P7) with TUPLTYPE
- [x] PCX parser (legacy)
- [x] TGA parser
- [x] SGI/RGB parser

### 2.2 Modern Image Formats
- [x] JPEG XL (.jxl)
  - [x] Container format (ISOBMFF)
  - [x] Codestream format
  - [x] EXIF box (full TIFF parsing)
  - [x] XMP box (full parsing)
- [x] JPEG 2000 (.jp2, .jpx, .j2k)
  - [x] JP2 container
  - [x] ihdr, colr, ftyp boxes
  - [x] SIZ marker (codestream)
  - [x] XML box (XMP parsing)
  - [x] UUID box (XMP parsing)
- [x] WebP write support
- [x] HEIC write support
- [x] AVIF write support (via HEIC writer)

### 2.3 Vector/Special Formats
- [x] SVG metadata
  - [x] XML parsing
  - [x] Metadata element
  - [x] Dublin Core
- [x] EPS/PS metadata
  - [x] DSC comments
  - [x] XMP packet
- [x] AI (Adobe Illustrator)
  - [x] PDF-based AI
  - [x] Legacy AI (EPS-based)

---

## Phase 3: RAW Formats (Priority: HIGH)

### 3.1 Canon Additional
- [x] CRW parser (legacy Canon)
  - [x] CIFF structure
  - [x] Heap parsing
- [ ] CR3 write support

### 3.2 Nikon Additional
- [x] NRW parser (Nikon coolpix)
- [ ] NEF encrypted data handling

### 3.3 Sony Additional
- [x] SRF parser (Sony RAW Format)
- [x] SR2 parser (Sony RAW 2)
- [ ] ARW write support

### 3.4 Other Manufacturers
- [x] X3F parser (Sigma/Foveon)
  - [x] Directory structure
  - [x] Property list
- [x] 3FR/FFF parser (Hasselblad)
  - [x] TIFF-based structure
- [x] IIQ parser (Phase One)
  - [x] TIFF variant
- [x] RWL parser (Leica)
- [x] DCR parser (Kodak)
- [x] KDC parser (Kodak)
- [x] K25 parser (Kodak)
- [x] MRW parser (Minolta)
- [x] ERF parser (Epson)
- [x] MEF parser (Mamiya)
- [x] SRW parser (Samsung)
- [x] RWL parser (Leica)
- [x] MOS parser (Leaf)
- [~] RWZ parser (Rawzor) - skipped, obsolete format

### 3.5 DNG Extensions
- [ ] DNG 1.6 features
- [ ] Lossy DNG
- [ ] Linear DNG
- [ ] DNG SDK integration (optional)

---

## Phase 4: MakerNotes Expansion (Priority: MEDIUM)

### 4.1 Major Manufacturers (Missing)
- [ ] Minolta MakerNotes
  - [ ] Main tags
  - [ ] CameraSettings
  - [ ] MinoltaRaw specifics
- [ ] Sigma MakerNotes
  - [ ] Main tags
  - [ ] X3F specifics
- [ ] Kodak MakerNotes
  - [ ] Main tags
  - [ ] IFD structure
- [ ] Casio MakerNotes
  - [ ] Type 1 (QV series)
  - [ ] Type 2 (EX series)
- [ ] Ricoh MakerNotes
  - [ ] Main tags
  - [ ] GR series specifics

### 4.2 Action Cameras/Drones
- [x] DJI MakerNotes
  - [x] Drone metadata
  - [x] GPS/altitude
  - [x] Gimbal data
- [x] GoPro MakerNotes
  - [x] GPMF data
  - [x] Telemetry
- [ ] Insta360 MakerNotes

### 4.3 Medium Format
- [ ] Phase One MakerNotes
- [ ] Hasselblad MakerNotes
- [ ] Mamiya MakerNotes
- [ ] Leaf MakerNotes

### 4.4 Legacy/Other
- [ ] HP MakerNotes
- [ ] Sanyo MakerNotes
- [ ] JVC MakerNotes
- [ ] GE MakerNotes
- [ ] Reconyx MakerNotes
- [ ] FLIR MakerNotes (thermal)
- [ ] Parrot MakerNotes (drones)
- [ ] Qualcomm MakerNotes (phones)
- [ ] Google MakerNotes (Pixel)
- [ ] Motorola MakerNotes
- [ ] LG MakerNotes
- [ ] Huawei MakerNotes
- [ ] Xiaomi MakerNotes
- [ ] OnePlus MakerNotes

### 4.5 MakerNotes Infrastructure
- [ ] Encrypted MakerNotes decoding
- [ ] Unknown tag preservation
- [ ] MakerNotes write support (per vendor)
- [ ] Sub-IFD deep parsing

---

## Phase 5: Video Formats (Priority: MEDIUM)

### 5.1 QuickTime/MP4 Family
- [x] MP4/MOV/M4A parser (in exiftool-formats)
- [x] QuickTime atom parser
  - [x] moov/trak/mdia hierarchy
  - [x] udta (user data)
  - [x] meta box
  - [x] XMP uuid
- [x] MP4 support (ISO base media)
- [x] M4V support (iTunes video)
- [x] MOV support (Apple QuickTime)
- [x] 3GP/3G2 support
- [ ] HEVC/H.265 metadata (partial)

### 5.2 AVI/RIFF Family
- [x] AVI parser
  - [x] RIFF structure
  - [x] avih/strh headers
  - [x] INFO chunk metadata
  - [x] EXIF chunk (full TIFF parsing)
  - [x] XMP chunk
- [ ] WAV metadata (shared with audio)

### 5.3 Matroska Family
- [x] MKV parser
  - [x] EBML structure
  - [x] Info element (duration, title, apps)
  - [x] Tracks element (video/audio codecs)
  - [x] Tags element
  - [x] Chapters (EditionEntry, ChapterAtom)
  - [x] Attachments (file name, MIME, size)
- [x] WebM parser (via MKV)

### 5.4 MPEG Family
- [x] MPEG-2 TS (.mts, .m2ts)
- [ ] MPEG-4 Part 2
- [ ] MPEG-1/2 system streams

### 5.5 Professional Video
- [x] MXF parser (broadcast)
- [x] DPX parser (film scan)
- [x] R3D parser (RED camera)
- [x] BRAW parser (Blackmagic)
- [x] ProRes metadata (via Mp4Parser)

### 5.6 Other Video
- [x] ASF/WMV parser
- [x] FLV parser
- [x] Real Media parser

---

## Phase 6: Audio Formats (Priority: LOW)

### 6.1 ID3 Tags
- [x] ID3 parser (in exiftool-formats)
- [x] ID3v1 parser
- [x] ID3v2.2/2.3/2.4 parser
- [x] ID3 frame types (TIT2, TPE1, etc.)
- [ ] ID3 write support
- [x] APIC (embedded image) extraction

### 6.2 Lossless Audio
- [x] FLAC parser
  - [x] METADATA_BLOCK
  - [x] VORBIS_COMMENT
  - [x] PICTURE block
- [x] ALAC (Apple Lossless via CAF)
- [x] APE parser (Monkey's Audio)
  - [x] MAC header
  - [x] APEv2 tags
- [x] WavPack parser
  - [x] wvpk header
  - [x] Hybrid/lossless mode
- [x] TAK parser

### 6.3 Compressed Audio
- [x] MP3 parser (via Id3Parser)
- [x] AAC parser (ADTS)
- [x] OGG Vorbis parser
  - [x] OGG page structure
  - [x] Vorbis comments
- [x] Opus parser (via OGG)
  - [x] OpusHead header
  - [x] OpusTags
- [x] WMA parser (via ASF)
- [x] M4A parser (via Mp4Parser)

### 6.4 Uncompressed Audio
- [x] WAV parser
  - [x] RIFF structure
  - [x] INFO chunk
  - [x] BEXT chunk (broadcast)
- [x] AIFF/AIFC parser
  - [x] FORM container
  - [x] COMM chunk (80-bit extended float)
  - [x] NAME/AUTH/ANNO chunks
- [x] AU parser (Sun Audio)
  - [x] .snd magic
  - [x] Encoding types (mu-law, PCM, etc.)
  - [x] Annotation field

### 6.5 Specialized Audio
- [x] DSF/DFF (DSD audio)
  - [x] DSF parser (DSD Stream File)
  - [x] DFF parser (DSDIFF)
  - [x] DSD64/128/256/512 rate detection
- [x] MIDI metadata
- [x] Audible (.aa, .aax)

---

## Phase 7: Document Formats (Priority: LOW)

### 7.1 PDF
- [ ] Create `exiftool-docs` crate
- [ ] PDF structure parsing
- [ ] Info dictionary
- [ ] XMP metadata stream
- [ ] PDF write support

### 7.2 Office Formats
- [ ] OOXML parser (.docx, .xlsx, .pptx)
  - [ ] ZIP container
  - [ ] docProps/core.xml
  - [ ] docProps/app.xml
  - [ ] Custom properties
- [ ] ODF parser (.odt, .ods, .odp)
  - [ ] meta.xml
- [ ] Legacy Office (.doc, .xls, .ppt)
  - [ ] OLE compound document
  - [ ] SummaryInformation

### 7.3 Adobe Formats
- [ ] PSD parser
  - [ ] Image resources section
  - [ ] IPTC block
  - [ ] XMP block
  - [ ] ICC profile
- [ ] PSB parser (large PSD)
- [ ] AI parser (Illustrator)
- [ ] INDD parser (InDesign)

### 7.4 Other Documents
- [ ] HTML metadata
- [ ] RTF metadata
- [ ] EPUB metadata
- [ ] DjVu metadata

---

## Phase 8: Archive/Container Formats (Priority: LOW)

### 8.1 Archives
- [ ] ZIP metadata
- [ ] 7Z metadata
- [ ] RAR metadata
- [ ] TAR metadata
- [ ] GZIP metadata

### 8.2 Disk Images
- [ ] ISO 9660 metadata
- [ ] DMG metadata

---

## Phase 9: Scientific/Specialized Formats (Priority: LOW)

### 9.1 Medical Imaging
- [ ] DICOM parser
  - [ ] File meta information
  - [ ] Data set parsing
  - [ ] Transfer syntax
- [ ] NIFTI parser (neuroimaging)

### 9.2 Astronomy
- [ ] FITS parser
  - [ ] Header units
  - [ ] Keyword=value pairs

### 9.3 GIS/Mapping
- [ ] GeoTIFF extensions
- [ ] NITF parser (military imagery)

### 9.4 Thermal Imaging
- [ ] FLIR radiometric data
- [ ] SEEK thermal
- [ ] InfiRay data

---

## Phase 10: Infrastructure Enhancements (Priority: MEDIUM)

### 10.1 Charset Support
- [ ] Create `exiftool-charset` crate
- [ ] UTF-8 (default)
- [ ] Latin-1 (ISO-8859-1)
- [ ] Windows-1252
- [ ] Shift-JIS (Japanese)
- [ ] EUC-JP (Japanese)
- [ ] GB2312/GBK (Chinese)
- [ ] Big5 (Chinese Traditional)
- [ ] EUC-KR (Korean)
- [ ] UTF-16 LE/BE
- [ ] MacRoman
- [ ] Cyrillic variants

### 10.2 Localization
- [ ] Create `exiftool-i18n` crate
- [ ] Tag name translations
- [ ] Value translations
- [ ] Language files (de, fr, es, ja, zh, ru, etc.)

### 10.3 Tag System Enhancements
- [ ] Composite tags (calculated values)
- [ ] Tag shortcuts/aliases
- [ ] Tag groups hierarchy
- [ ] Writable flag per tag
- [ ] Protected tag handling

### 10.4 CLI Enhancements
- [ ] Output format: XML
- [ ] Output format: HTML
- [ ] Output format: CSV
- [ ] Output format: PHP
- [ ] Output format: Perl (for compatibility)
- [ ] Recursive directory processing
- [ ] File filtering by date/type
- [ ] Conditional tag writing
- [ ] Geotag from GPX/KML
- [ ] Time shift operations
- [ ] Duplicate detection

### 10.5 API Enhancements
- [ ] Async read support
- [ ] Memory-mapped file support
- [ ] Streaming parser mode
- [ ] Progress callbacks
- [ ] Cancellation tokens

---

## Phase 11: Testing & Quality (Priority: HIGH)

### 11.1 Test Infrastructure
- [ ] Golden file test suite
- [ ] Round-trip test framework
- [ ] Fuzz testing (cargo-fuzz)
- [ ] Property-based tests (proptest)
- [ ] Benchmark suite (criterion)

### 11.2 Compatibility Testing
- [ ] Compare output with exiftool
- [ ] Test with ExifTool test images
- [ ] Real-world sample collection
- [ ] Edge case database

### 11.3 Documentation
- [ ] API documentation (rustdoc)
- [ ] User guide
- [ ] Migration guide from ExifTool
- [ ] Tag reference documentation
- [ ] Format support matrix

---

## Phase 12: Python Bindings Enhancement (Priority: MEDIUM)

### 12.1 API Completeness
- [ ] All tag access methods
- [ ] Write support
- [ ] Batch operations
- [ ] Async support
- [ ] Type stubs (.pyi)

### 12.2 Pythonic Interface
- [ ] Context managers
- [ ] Iterator protocol
- [ ] Dict-like access
- [ ] Dataclass integration

---

## Phase 13: Additional Language Bindings (Priority: LOW)

### 13.1 C/C++ Bindings
- [ ] C header generation (cbindgen)
- [ ] Stable ABI
- [ ] Memory management docs

### 13.2 JavaScript/WASM
- [ ] wasm-bindgen setup
- [ ] Browser-compatible build
- [ ] Node.js package
- [ ] TypeScript definitions

### 13.3 Other Languages
- [ ] Ruby bindings (magnus)
- [ ] Go bindings (cgo)
- [ ] Java bindings (JNI)

---

## Appendix A: ExifTool Perl Modules -> Rust Mapping

| Perl Module | Rust Equivalent | Status |
|-------------|-----------------|--------|
| ExifTool.pm | exiftool-formats/lib.rs | [~] |
| Exif.pm | exiftool-core/ifd.rs | [x] |
| TIFF.pm | exiftool-formats/tiff.rs | [x] |
| JPEG.pm | exiftool-formats/jpeg.rs | [x] |
| PNG.pm | exiftool-formats/png.rs | [x] |
| XMP.pm | exiftool-xmp/ | [~] |
| IPTC.pm | exiftool-formats/iptc.rs | [x] |
| ICC_Profile.pm | exiftool-icc/ | [x] |
| GPS.pm | exiftool-tags/generated/gps.rs | [x] |
| Canon.pm | exiftool-formats/makernotes/canon.rs | [x] |
| Nikon.pm | exiftool-formats/makernotes/nikon.rs | [x] |
| Sony.pm | exiftool-formats/makernotes/sony.rs | [x] |
| FujiFilm.pm | exiftool-formats/makernotes/fujifilm.rs | [x] |
| Olympus.pm | exiftool-formats/makernotes/olympus.rs | [x] |
| Panasonic.pm | exiftool-formats/makernotes/panasonic.rs | [x] |
| Pentax.pm | exiftool-formats/makernotes/pentax.rs | [x] |
| Samsung.pm | exiftool-formats/makernotes/samsung.rs | [x] |
| Apple.pm | exiftool-formats/makernotes/apple.rs | [x] |
| QuickTime.pm | exiftool-formats/mp4.rs | [x] |
| ID3.pm | exiftool-formats/id3.rs | [x] |
| RIFF.pm | exiftool-formats/avi.rs | [x] |
| Matroska.pm | exiftool-formats/mkv.rs | [x] |
| Jpeg2000.pm | exiftool-formats/jp2.rs | [x] |
| JXL.pm | exiftool-formats/jxl.rs | [x] |
| PPM.pm | exiftool-formats/pnm.rs | [x] |
| RIFF.pm (WAV) | exiftool-formats/wav.rs | [x] |
| TGA.pm | exiftool-formats/tga.rs | [x] |
| PCX.pm | exiftool-formats/pcx.rs | [x] |
| SGI.pm | exiftool-formats/sgi.rs | [x] |
| CanonRaw.pm | exiftool-formats/crw.rs | [x] |
| NikonCapture.pm | exiftool-formats/nrw.rs | [x] |
| Sony.pm (SRF/SR2) | exiftool-formats/srf.rs | [x] |
| SigmaRaw.pm | exiftool-formats/x3f.rs | [x] |
| AIFF.pm | exiftool-formats/aiff.rs | [x] |
| MinoltaRaw.pm | exiftool-formats/mrw.rs | [x] |
| Hasselblad.pm | exiftool-formats/fff.rs | [x] |
| Epson.pm | exiftool-formats/erf.rs | [x] |
| MakerNotes/Mamiya.pm | exiftool-formats/mef.rs | [x] |
| Samsung.pm (SRW) | exiftool-formats/srw.rs | [x] |
| Panasonic.pm (RWL) | exiftool-formats/rwl.rs | [x] |
| Kodak.pm (DCR) | exiftool-formats/dcr.rs | [x] |
| Kodak.pm (KDC) | exiftool-formats/dcr.rs | [x] |
| Kodak.pm (K25) | exiftool-formats/dcr.rs | [x] |
| Leaf.pm | exiftool-formats/mos.rs | [x] |
| PhaseOne.pm | exiftool-formats/iiq.rs | [x] |
| AU.pm | exiftool-formats/au.rs | [x] |
| Vorbis.pm | exiftool-formats/ogg.rs | [x] |
| Opus.pm | exiftool-formats/ogg.rs | [x] |
| APE.pm | exiftool-formats/ape.rs | [x] |
| WavPack.pm | exiftool-formats/wv.rs | [x] |
| DSD.pm (DSF) | exiftool-formats/dsf.rs | [x] |
| DSD.pm (DFF) | exiftool-formats/dsf.rs | [x] |
| PDF.pm | - | [ ] |
| ... | ... | ... |

---

## Appendix B: Priority Matrix

### P0 - Critical (blocks other work)
- IPTC support
- XMP write
- Value interpretation

### P1 - High (commonly requested)
- GIF/BMP parsers
- WebP/HEIC write
- Thumbnail extraction
- More RAW formats

### P2 - Medium (nice to have)
- Video metadata
- Audio metadata
- Additional MakerNotes

### P3 - Low (completeness)
- Documents
- Archives
- Localization
- Additional bindings

---

## Appendix C: Estimated Lines of Code

| Component | Estimated LOC |
|-----------|---------------|
| IPTC crate | 800 |
| XMP write | 600 |
| ICC crate | 700 |
| Value interp | 1500 |
| Simple images (GIF, BMP, etc.) | 1000 |
| Modern images (JXL, JP2) | 1500 |
| RAW formats (all remaining) | 4000 |
| MakerNotes (all remaining) | 8000 |
| Video crate | 5000 |
| Audio crate | 3000 |
| Documents crate | 3000 |
| Charset crate | 500 |
| i18n crate | 300 |
| Tests & docs | 3000 |
| **TOTAL** | **~32,000** |

---

## Progress Tracking

Last updated: 2025-01-05

| Phase | Progress | Notes |
|-------|----------|-------|
| Phase 0 | 100% | Audit complete |
| Phase 1 | 90% | IPTC, XMP, ICC, ValueInterp, Thumbnail, BigTIFF, Multi-page done |
| Phase 2 | 100% | All image formats complete |
| Phase 3 | 100% | All RAW formats complete (RWZ skipped) |
| Phase 4 | 25% | 11/44 vendors (added DJI, GoPro) |
| Phase 5 | 100% | All video formats complete |
| Phase 6 | 95% | All audio formats complete, only ID3 write pending |
| Phase 7 | 0% | Not started |
| Phase 8 | 0% | Not started |
| Phase 9 | 0% | Not started |
| Phase 10 | 10% | Basic CLI done |
| Phase 11 | 65% | 363 tests (335 formats + 28 other) |
| Phase 12 | 60% | PyO3 works |
| Phase 13 | 0% | Not started |

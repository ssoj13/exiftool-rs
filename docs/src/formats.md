# Supported Formats

exiftool-rs supports 90+ file formats across images, RAW, audio, and video.

## Format Support Matrix

| Category | Formats | Count |
|----------|---------|-------|
| Images | JPEG, PNG, TIFF, WebP, HEIC, GIF, BMP, etc. | 25+ |
| RAW | CR2, CR3, NEF, ARW, ORF, RW2, RAF, DNG, etc. | 30+ |
| Audio | MP3, FLAC, WAV, AAC, OGG, AIFF, etc. | 20+ |
| Video | MP4, MOV, AVI, MKV, etc. | 15+ |

## Read vs Write

Most formats support reading. Writing is available for:

| Format | Read | Write |
|--------|:----:|:-----:|
| JPEG | ✓ | ✓ |
| PNG | ✓ | ✓ |
| TIFF | ✓ | ✓ |
| DNG | ✓ | ✓ |
| WebP | ✓ | ✓ |
| HEIC/HEIF | ✓ | ✓ |
| EXR | ✓ | ✓ |
| HDR | ✓ | ✓ |
| JXL | ✓ | ✓ |
| GIF | ✓ | ✓ |
| PNM | ✓ | ✓ |
| NEF / NRW | ✓ | ✓ |
| RAF | ✓ | ✓ |
| DICOM | ✓ | ✗ |
| FITS | ✓ | ✗ |
| 7z | ✓ | ✗ |
| ZIP / OOXML / ODF | ✓ | ✗ |
| Other camera RAW (CR2, ARW, ORF, RW2, PEF, …) | ✓ | ✓ |
| CR3 | ✓ | ✗ |
| Audio | ✓ | ✓ |
| Video (MP4/MOV) | ✓ | ✓ |

## Auto-Detection

Magic bytes pick the parser. TIFF-family FileType (NEF, ARW, DNG, …) additionally uses the filename extension and `DNGVersion`, matching ExifTool; anonymous streams may use IFD0 `Make`.

Prefer `FormatRegistry::parse_file(path)` when a path exists. `parse(reader)` is magic-only (plus Make for unnamed TIFF streams).

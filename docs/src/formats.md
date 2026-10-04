# Supported Formats

The registry reads metadata across image, RAW, audio/video, document, and archive
containers. A registered parser may recognize several related formats; tag
coverage and writable fields vary by container and camera model.

## Format Support Matrix

| Category | Examples | Details |
|----------|----------|---------|
| Images | JPEG, PNG, TIFF, WebP, HEIC/AVIF, GIF, EXR | [Images](formats/images.md) |
| Camera RAW | CR2, CR3, NEF, ARW, ORF, RW2, RAF, DNG | [RAW](formats/raw.md) |
| Audio | MP3, FLAC, WAV, AAC, Ogg, AIFF | [Audio](formats/audio.md) |
| Video | MP4, MOV, AVI, MKV, MXF | [Video](formats/video.md) |
| Documents and archives | PDF, DICOM, FITS, ZIP, 7z, ZIP-based office formats | Container metadata; read-only |

## Read vs Write

Most formats support reading. Writing is available for:

| Format | Read | Write |
|--------|:----:|:-----:|
| JPEG | ✓ | ✓ |
| PNG | ✓ | ✓ |
| TIFF | ✓ | ✓ |
| DNG | ✓ | ✓ |
| WebP | ✓ | ✓ |
| HEIC/HEIF/AVIF | ✓ | ✓ |
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
| CR3 | ✓ | ✓ |
| WAV / FLAC / MP3 | ✓ | ✓ |
| Other audio containers | ✓ | ✗ |
| Video (MP4/MOV) | ✓ | ✓ |

## Auto-Detection

Magic bytes pick the parser. TIFF-family FileType (NEF, ARW, DNG, …) additionally uses the filename extension and `DNGVersion`, matching ExifTool; anonymous streams may use IFD0 `Make`.

Prefer `FormatRegistry::parse_file(path)` when a path exists. `parse(reader)` is magic-only (plus Make for unnamed TIFF streams).

## Coverage limits

A check mark indicates a parser or writer path, not complete tag coverage.
`Metadata::is_writable()` is the programmatic format check; see
[writing metadata](writing.md) for payload replacement and RAW preservation.

Some operations buffer inputs and enforce the shared 100 MiB limit. BigTIFF
support does not guarantee files over 4 GiB can be processed. See
[performance](performance.md).

7z encoded headers use LZMA decoding; AES-encrypted headers cannot be decoded
and LZMA2-encoded headers are not supported by this path.

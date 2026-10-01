# Changelog

All notable changes to this project are documented in this file.

## [0.1.0] - Unreleased

Initial release: a native Rust port of ExifTool that reads and writes metadata without a Perl runtime.

### Reading
- EXIF, XMP (rdf:Bag/Seq/Alt), IPTC and ICC parsing.
- Image formats: JPEG, PNG, TIFF, DNG, HEIC, AVIF, WebP, GIF, OpenEXR, HDR, JPEG 2000.
- Camera RAW: CR2, CR3, NEF/NRW, ARW, ORF, RW2, PEF, RAF and other TIFF-based RAW layouts, with unique magics detected first and generic TIFF classified by `DNGVersion`, extension and Make.
- Containers and scientific formats: DICOM, FITS, 7z (including LZMA-encoded headers), ZIP/OOXML/ODF.
- JPEG post-EOI trailer detection (AFCP, FotoStation, PhotoMechanic, Samsung, CanonVRD); PNG `zXIf` (zlib EXIF).
- MakerNotes for Canon, Nikon, Sony, FujiFilm, Olympus, Panasonic, Pentax, Samsung, Apple, Kodak, DJI, GoPro and more, with nested sub-IFDs (`AttrValue::Group`).
- Nikon MakerNotes: Type-3 IFD, decryption, LensData, ColorBalance and per-model ShotInfo dispatch (D40 to Z9).
- Tag tables generated from ExifTool's Perl sources (~2500+ tags), including Mask/`0.1` indices, ProcessBinaryData and conditional SubDirectory tables.

### Writing
- Writers for JPEG, PNG, TIFF, DNG, HEIC (creates an EXIF item when missing), WebP, GIF, MP4/MOV, WAV, FLAC, MP3, EXR and HDR.
- RAW writers: CR2, CR3, NEF/NRW, ARW, ORF, RW2, PEF, RAF and others. SubIFD trees, strips/tiles, MakerNotes and ICC blobs are preserved, as are A100-style MRW/CFA trailers.
- MakerNotes field write for the major camera vendors and phone makers, overlaying values in place and re-encrypting Nikon data where needed.
- `Metadata::is_camera_raw()` and `Metadata::is_writable()` for write protection, which also catches renamed RAW files by their Make tag.

### Python bindings (`exiftool-py`)
- PyO3/maturin package with an `Image` class: properties (`make`, `model`, `iso`, `fnumber`, ...), dict-like access, context manager and iteration.
- `Rational` and `GPS` (decimal degrees and raw DMS) classes.
- Exception hierarchy: `ExifError`, `FormatError`, `WriteError`, `TagError`.
- Parallel batch processing with `scan()` and `scan_dir()`; type stubs for IDE autocomplete.

### CLI
- `exif` tool for reading and writing metadata, time shift, geotagging, JSON/CSV import, tag copy, templated renaming, metadata stripping, validation, conditional processing and duplicate search.

### Tooling and safety
- `cargo xtask dump` / `codegen` to regenerate tag tables from ExifTool.
- `bootstrap.py` for build, test, lint, CLI install, Python wheel and docs.
- Golden test fixtures from ExifTool's `t/images` and a parity check against Perl FileType (`xtask/parity.py`).
- Fuzz targets seeded from the test fixtures.
- Checked arithmetic in IFD parsing and a 100 MB file size limit in parsers and writers.
- Licensed under the same terms as ExifTool (`Artistic-1.0-Perl OR GPL-1.0-or-later`); see `NOTICE.md`.

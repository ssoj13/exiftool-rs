# Python Bindings

The `exiftool-rs` Python package provides a Pythonic interface to the Rust library.

## Why Python Bindings?

Rust gives us:
- **Speed** - Native code, no interpreter overhead
- **Free bindings** - PyO3 makes Python integration trivial
- **No dependencies** - Single wheel, no runtime requirements

Compared to calling ExifTool via subprocess:
- 10-100x faster for batch operations
- No process spawn overhead
- Direct memory access to thumbnails/previews

## Features

- Dict-like access to EXIF tags
- Properties for common fields (make, model, gps, etc.)
- Read/write support for JPEG, PNG, TIFF, WebP, HEIC
- Thumbnail and preview extraction
- Multi-page TIFF support
- Context manager protocol

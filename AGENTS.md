# exiftool-rs Architecture & Agent Guide

> Reference for AI agents and developers. Contains dataflows, codepaths, and architectural decisions.

## Project Overview

exiftool-rs is a **pure Rust** library for reading and writing image metadata (EXIF, XMP, IPTC, ICC). It is inspired by [ExifTool](https://exiftool.org/) (Perl) and uses tag definitions derived from ExifTool's database (~2500+ tags).

**Reference implementation:** `_ref/exif-rs/` (kamadak/exif-rs, BSD-2-Clause) — used for TIFF/EXIF parsing logic parity checks. Our project extends far beyond the reference with multi-crate architecture, 60+ format parsers, writers, and ExifTool-style features.

---

## ASCII Dataflow Diagrams

### High-Level Read Path

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  File/Reader    │────>│ FormatRegistry    │────>│ FormatParser    │
│  (bytes)       │     │ .detect(header)   │     │ .parse(reader)   │
└─────────────────┘     └──────────────────┘     └────────┬────────┘
         │                          │                       │
         │  16 bytes                │ can_parse()            │ Metadata
         │  header                  │ returns first match   │ (exif, xmp,
         v                          v                       │  thumbnail...)
┌─────────────────┐     ┌──────────────────┐     ┌────────┴────────┐
│ Parser order    │     │ TIFF-based:      │     │ Metadata.exif    │
│ (registry.rs):  │     │ IfdReader        │     │ = Attrs (k/v)    │
│ 1. JPEG, PNG    │     │ -> read_ifd()    │     │                  │
│ 2. GIF, BMP     │     │ -> sub-IFDs      │     │ Tag names from   │
│ 3. WebP, RAF    │     │   (0x8769,8825,  │     │ exiftool-tags    │
│ 4. Exr, HDR     │     │   0xA005)        │     │ (generated)      │
│ 5. CR3, HEIC    │     └──────────────────┘     └─────────────────┘
│ 6. MP4, ID3     │
│ ...             │
│ 7. TIFF-based   │
│    (last!)      │
└─────────────────┘
```

### TIFF/EXIF Parse Codepath (Single Source Pattern)

```
TIFF bytes ──> IfdReader::new(data, byte_order)
                    │
                    v
              parse_header() ──> ifd0_offset
                    │
                    v
              read_ifd(offset) ──> (entries, next_ifd)
                    │
                    ├── FOR each entry:
                    │       ├── lookup_ifd0(tag) ──> name, entry_to_attr() ──> metadata.exif.set()
                    │       ├── tag 0x8769 ──> read_ifd(offset) ──> ExifIFD + MakerNotes
                    │       ├── tag 0x8825 ──> read_ifd(offset) ──> GPS IFD
                    │       └── tag 0xA005 ──> read_ifd(offset) ──> Interop IFD (TIFF only)
                    │
                    └── next_ifd != 0 ──> read_ifd(next_ifd) ──> IFD1 (thumbnail)
```

### Format-Specific EXIF Extraction

```
Container                    EXIF location                Parser method
────────────────────────────────────────────────────────────────────────
JPEG      APP1 0xFFE1 "Exif\0\0" + TIFF     jpeg::parse_exif() [module fn]
PNG       eXIf chunk (TIFF)                 png::PngParser::parse_exif()
WebP      EXIF chunk (TIFF)                 utils::parse_tiff_exif (via parse_exif_chunk)
HEIC      meta/iloc/iinf/idat, Exif item    heic::HeicParser::parse_tiff_exif()
CR3       meta/Exif in moov                 cr3::parse_exif_from_moov()
AVI       RIFF chunks                       avi::AviParser::parse_tiff_exif()
JXL       jumb box (Exif)                   jxl::JxlParser::parse_tiff_exif()
```

### Write Path

```
Metadata (Attrs) ──> build_exif_bytes() [utils.rs]
                          │
                          v
                    ExifWriter (exiftool-core)
                          │
                          v
                    TIFF bytes (LE)
                          │
         ┌────────────────┼────────────────┐
         v                v                 v
    JpegWriter      PngWriter         TiffWriter
    (APP1 replace)  (eXIf chunk)      (IFD rewrite)
         │                │                 │
         v                v                 v
    output.jpg      output.png        output.tiff
```

---

## Codepath Summary

| Component          | Location                | Responsibility                              |
|--------------------|-------------------------|---------------------------------------------|
| FormatRegistry     | formats/registry.rs     | Parser registration, detect(), parse()      |
| FormatParser       | formats/traits.rs       | can_parse, format_name, extensions, parse   |
| IfdReader          | core/ifd.rs             | TIFF IFD parsing, BigTIFF                   |
| tag_lookup         | formats/tag_lookup.rs   | lookup_ifd0, lookup_exif_subifd, lookup_gps|
| entry_to_attr      | formats/utils.rs       | IfdEntry → AttrValue (single source)        |
| build_exif_bytes   | formats/utils.rs       | Metadata → TIFF bytes (single source)        |
| ExifWriter         | core/writer.rs          | TIFF structure builder                     |

---

## Key Design Decisions

1. **Single source for EXIF→Attr conversion**: `utils::entry_to_attr()` used by all parsers.
2. **Tag names from codegen**: `exiftool-tags` generated from ExifTool Perl (`cargo xtask codegen`).
3. **Attrs vs Field**: We use `Attrs` (HashMap<String, AttrValue), not ref's `Field` (tag, ifd_num, Value). Our model is flatter, ExifTool-style.
4. **Parser order matters**: Registry tries parsers in sequence; TIFF-based formats last to avoid false positives.
5. **Vendor detection**: Make tag (0x010F) used for MakerNotes decoder selection (Canon, Nikon, Sony, etc.).

---

## Known Issues & TODOs

| File:Line       | Issue | Status |
|-----------------|-------|--------|
| heic_writer.rs:869 | TODO: Full HEIC write with EXIF add — currently copies file as-is when no EXIF present | Documented in README |
| webp.rs           | FIXED: Now uses shared `parse_tiff_exif` with full sub-IFD support | Done |
| jpeg/heic/jxl/avi | FIXED: All use shared `parse_tiff_exif` with InteropIFD (0xA005) | Done |

---

## File References (Important Paths)

```
_ref/exif-rs/src/          # Reference (exif-rs, single crate)
├── tiff.rs, value.rs      # TIFF/EXIF parse logic
├── jpeg.rs, png.rs        # Container EXIF extraction
├── webp.rs, isobmff.rs    # HEIF/WebP
└── reader.rs              # read_from_container()

crates/
├── exiftool-core/         # IFD, ByteOrder, RawValue, ExifWriter
├── exiftool-formats/      # All format parsers, registry, utils
├── exiftool-attrs/        # Attrs, AttrValue
├── exiftool-tags/         # Generated tag tables, interp
├── exiftool-xmp/          # XMP parse/write
├── exiftool-iptc/         # IPTC
├── exiftool-icc/          # ICC profile
└── exiftool-cli/          # CLI tool
```

# DIAGRAMS.md — exiftool-rs formats

Reference: ExifTool 13.59 (`vfx.ref/exiftool`).

## Registry dispatch (as implemented)

```mermaid
flowchart TD
  A[CLI / Python / library] --> B{path known?}
  B -->|yes| C[FormatRegistry.parse_file]
  B -->|no| D[FormatRegistry.parse reader]
  C --> E[Read DETECT_HEADER_LEN 132, rewind]
  D --> E
  E --> F{First parser where can_parse}
  F -->|JPEG PNG RAF CR2 ORF RW2 CR3 HEIC MP4 ZIP DICOM FITS ...| G[Format-specific parse]
  F -->|TIFF / BigTIFF magic| H[TiffParser]
  H --> I[tiff_family::classify]
  I -->|DNGVersion| J[format DNG]
  I -->|hint nef/arw/...| K[vendor FileType]
  I -->|Make Nikon, no generic tif hint| L[format NEF]
  I -->|else| M[format TIFF]
  F -->|no match| N[UnsupportedFormat]
  G --> O[Metadata]
  J --> O
  K --> O
  L --> O
  M --> O
```

TIFF-family wrappers (ERF, MEF, SRW, RWL, DCR, NEF, ARW, …) stay in `default_parsers()` for `by_extension` / `get`, but **`can_parse` is false** so they cannot steal TIFF magic.

## IFD walker

```mermaid
flowchart LR
  IFD0 --> ExifIFD
  IFD0 --> GPS
  IFD0 --> Interop
  IFD0 --> SubIFD
  IFD0 --> XMP
  IFD0 --> IPTC
  ExifIFD --> MakerNotes
```

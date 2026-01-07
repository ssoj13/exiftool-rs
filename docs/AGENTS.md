# ExifTool-RS: Architecture & Dataflow Documentation

> Auto-generated documentation for AI agents and developers.
> Last updated: 2026-01-06 (Bug Hunt #2)
>
> **STATUS**: Most issues from plan1.md have been FIXED. See plan2.md for current state.

## Table of Contents

1. [Project Overview](#project-overview)
2. [Crate Architecture](#crate-architecture)
3. [Dataflow Diagrams](#dataflow-diagrams)
4. [Trait Hierarchy](#trait-hierarchy)
5. [Error Propagation](#error-propagation)
6. [Key Code Paths](#key-code-paths)
7. [Critical Files Reference](#critical-files-reference)
8. [Known Issues & TODOs](#known-issues--todos)

---

## Project Overview

ExifTool-RS is a Rust port of the Perl ExifTool, providing metadata extraction and writing capabilities for 90+ file formats.

### Workspace Structure

```
exiftool-rs/
├── crates/
│   ├── exiftool-core/      # Core TIFF/EXIF parsing primitives
│   ├── exiftool-formats/   # Format parsers & writers (90+ formats)
│   ├── exiftool-attrs/     # Attribute storage system
│   ├── exiftool-tags/      # Generated tag definitions
│   ├── exiftool-xmp/       # XMP parsing
│   ├── exiftool-icc/       # ICC profile parsing
│   ├── exiftool-iptc/      # IPTC metadata parsing
│   ├── exiftool-cli/       # Command-line interface
│   └── exiftool-py/        # Python bindings (PyO3)
├── docs/                   # Documentation
└── xtask/                  # Build utilities
```

---

## Crate Architecture

### Dependency Graph

```
                    ┌─────────────┐
                    │ exiftool-cli│ (binary)
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
              ▼            ▼            ▼
       ┌──────────┐ ┌─────────────┐ ┌──────────┐
       │ exiftool │ │   exiftool  │ │ exiftool │
       │   -py    │ │  -formats   │ │   -cli   │
       │  (PyO3)  │ │   (main)    │ │ outputs  │
       └────┬─────┘ └──────┬──────┘ └──────────┘
            │              │
            │   ┌──────────┼──────────┬──────────┐
            │   │          │          │          │
            │   ▼          ▼          ▼          ▼
            │ ┌─────┐  ┌───────┐  ┌──────┐  ┌──────┐
            │ │ xmp │  │  icc  │  │ iptc │  │ tags │
            │ └──┬──┘  └───┬───┘  └──┬───┘  └──────┘
            │    │         │         │
            └────┼─────────┼─────────┼─────────────┐
                 │         │         │             │
                 └─────────┴─────────┴─────────────┤
                                                   │
                           ┌───────────────────────┘
                           ▼
                    ┌─────────────┐
                    │exiftool-core│
                    └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │exiftool-attrs│
                    └─────────────┘
```

### Crate Responsibilities

| Crate | Purpose | Key Types |
|-------|---------|-----------|
| `exiftool-core` | Low-level TIFF/IFD parsing | `IfdReader`, `IfdEntry`, `ByteOrder`, `Error` |
| `exiftool-attrs` | Attribute storage with dirty tracking | `Attrs`, `AttrValue`, `AttrKey` |
| `exiftool-formats` | Format detection & parsing | `FormatParser`, `FormatWriter`, `Metadata`, `FormatRegistry` |
| `exiftool-tags` | Generated tag name mappings | `lookup()`, `TagInfo` |
| `exiftool-xmp` | XMP/RDF parsing | `XmpParser`, `XmpValue` |
| `exiftool-icc` | ICC color profile parsing | `IccParser`, `IccProfile` |
| `exiftool-iptc` | IPTC-IIM record parsing | `IptcParser`, `IptcRecord` |
| `exiftool-cli` | CLI application | `main()`, output formatters |
| `exiftool-py` | Python bindings | `ExifReader`, `PyMetadata` |

---

## Dataflow Diagrams

### 1. Read Operation (CLI)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                              READ DATAFLOW                                    │
└──────────────────────────────────────────────────────────────────────────────┘

      User Input                          CLI                         Output
    ┌───────────┐                  ┌──────────────┐              ┌────────────┐
    │ file.jpg  │ ──────────────▶  │    main()    │ ──────────▶  │   stdout   │
    │ -json     │                  │  main.rs:1   │              │   (JSON)   │
    └───────────┘                  └──────┬───────┘              └────────────┘
                                          │
                    ┌─────────────────────┼─────────────────────┐
                    │ Parse args (clap)   │ Open file           │
                    ▼                     ▼                     │
            ┌───────────────┐     ┌───────────────┐             │
            │   Cli Args    │     │ BufReader<F>  │             │
            │ (Args struct) │     │ (ReadSeek)    │             │
            └───────────────┘     └───────┬───────┘             │
                                          │                     │
                    ┌─────────────────────┘                     │
                    │ registry.detect_format()                  │
                    ▼                                           │
            ┌───────────────────────────────────────┐           │
            │         FormatRegistry                 │           │
            │     registry.rs:1-200                  │           │
            │                                        │           │
            │  ┌─────────────────────────────────┐  │           │
            │  │ 1. Read header (512 bytes)      │  │           │
            │  │ 2. Try can_parse() on each      │  │           │
            │  │ 3. Fallback: by_extension()     │  │           │
            │  └─────────────────────────────────┘  │           │
            └──────────────────┬────────────────────┘           │
                               │ parser                         │
                               ▼                                │
            ┌───────────────────────────────────────┐           │
            │       dyn FormatParser                 │           │
            │    (JpegParser, PngParser, etc.)       │           │
            │                                        │           │
            │  parser.parse(reader) -> Metadata      │           │
            └──────────────────┬────────────────────┘           │
                               │                                │
           ┌───────────────────┼───────────────────┐            │
           │                   │                   │            │
           ▼                   ▼                   ▼            │
    ┌─────────────┐    ┌─────────────┐     ┌─────────────┐      │
    │    EXIF     │    │     XMP     │     │     ICC     │      │
    │   (Attrs)   │    │   (Attrs)   │     │   (Attrs)   │      │
    │  core::ifd  │    │  xmp::parse │     │ icc::parse  │      │
    └──────┬──────┘    └──────┬──────┘     └──────┬──────┘      │
           │                  │                   │             │
           └──────────────────┴───────────────────┘             │
                               │                                │
                               ▼                                │
                      ┌───────────────┐                         │
                      │   Metadata    │                         │
                      │ (lib.rs:266)  │                         │
                      │               │                         │
                      │ .exif: Attrs  │                         │
                      │ .xmp: Attrs   │                         │
                      │ .icc: Attrs   │                         │
                      │ .iptc: Attrs  │                         │
                      │ .format: str  │                         │
                      └───────┬───────┘                         │
                              │                                 │
                              │ Output formatting               │
                              ▼                                 │
                      ┌───────────────────────────┐             │
                      │    Output Formatters      │◀────────────┘
                      │                           │
                      │ • json_output.rs          │
                      │ • xml_output.rs           │
                      │ • table_output.rs         │
                      │ • text_output.rs          │
                      └───────────────────────────┘
```

### 2. Write Operation (CLI)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                              WRITE DATAFLOW                                   │
└──────────────────────────────────────────────────────────────────────────────┘

      User Input                          CLI                        Output
    ┌───────────┐                  ┌──────────────┐              ┌────────────┐
    │ file.jpg  │ ──────────────▶  │    main()    │ ──────────▶  │ file_new   │
    │ -Artist=X │                  │ main.rs:400+ │              │   .jpg     │
    └───────────┘                  └──────┬───────┘              └────────────┘
                                          │
                    ┌─────────────────────┼─────────────────────┐
                    │                     │                     │
                    ▼                     ▼                     ▼
            ┌───────────────┐     ┌───────────────┐     ┌───────────────┐
            │  Parse tags   │     │ Read existing │     │ Check writable│
            │  to modify    │     │   metadata    │     │ is_writable() │
            └───────────────┘     └───────┬───────┘     └───────────────┘
                                          │
                    ┌─────────────────────┘
                    │ Apply modifications
                    ▼
            ┌───────────────────────────────────────┐
            │         Metadata (modified)           │
            │                                        │
            │  metadata.exif.set(tag, value)        │
            │  metadata.xmp.set(tag, value)         │
            └──────────────────┬────────────────────┘
                               │
                               │ match metadata.format
                               ▼
            ┌───────────────────────────────────────┐
            │            Writer Selection           │
            │            (main.rs:450-470)          │
            │                                        │
            │  match format {                       │
            │    "JPEG" => JpegWriter::write()     │
            │    "PNG"  => PngWriter::write()      │
            │    "TIFF" => TiffWriter::write()     │
            │    "EXR"  => ExrWriter::write()      │
            │    "HDR"  => HdrWriter::write()      │
            │    "HEIC" => HeicWriter::write()     │
            │    "WebP" => WebpWriter::write()     │
            │    _ => error!("Not supported")      │
            │  }                                    │
            └──────────────────┬────────────────────┘
                               │
                               ▼
            ┌───────────────────────────────────────┐
            │        Specific Writer                 │
            │     (e.g., jpeg_writer.rs)             │
            │                                        │
            │  1. Copy non-metadata segments         │
            │  2. Rebuild APP1 (EXIF) segment        │
            │  3. Rebuild APP1 (XMP) segment         │
            │  4. Copy image data unchanged          │
            │  5. Write to output                    │
            └──────────────────┬────────────────────┘
                               │
                               ▼
                      ┌───────────────┐
                      │  Output File  │
                      │ (Write + Sync)│
                      └───────────────┘
```

### 3. Format Detection Pipeline

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                         FORMAT DETECTION PIPELINE                             │
└──────────────────────────────────────────────────────────────────────────────┘

   Input Stream                  Detection Phases                    Result
  ┌──────────────┐         ┌─────────────────────────┐         ┌─────────────┐
  │ ReadSeek     │ ──────▶ │   Phase 1: Magic       │ ──────▶ │ FormatParser│
  │ (file/memory)│         │                         │         │ (detected)  │
  └──────────────┘         └───────────┬─────────────┘         └─────────────┘
                                       │
       ┌───────────────────────────────┼───────────────────────────────┐
       │                               │                               │
       ▼                               ▼                               ▼
  ┌─────────────┐             ┌─────────────────┐             ┌─────────────┐
  │ Read header │             │ Try all parsers │             │  Found?     │
  │ (512 bytes) │             │ can_parse(hdr)  │             │  Yes → use  │
  └─────────────┘             └─────────────────┘             │  No → next  │
                                                              └──────┬──────┘
                                                                     │
       ┌─────────────────────────────────────────────────────────────┘
       │ If no magic match...
       ▼
  ┌─────────────────────────────────────────────────────────────────────────┐
  │                          Phase 2: Extension                             │
  │                                                                         │
  │   by_extension(ext) → search parsers where ext in extensions()          │
  │                                                                         │
  │   Examples:                                                             │
  │   • ".arw" → ArwParser (Sony RAW)                                       │
  │   • ".nef" → NefParser (Nikon RAW)                                      │
  │   • ".cr2" → Cr2Parser (Canon RAW)                                      │
  │                                                                         │
  │   NOTE: All RAW formats have TIFF magic, distinguished by extension     │
  └─────────────────────────────────────────────────────────────────────────┘


  Magic Bytes Reference (first 12 bytes):
  ┌────────────┬─────────────────────────────────────┬─────────────────┐
  │ Format     │ Magic Signature                     │ Detection       │
  ├────────────┼─────────────────────────────────────┼─────────────────┤
  │ JPEG       │ FF D8 FF                            │ can_parse()     │
  │ PNG        │ 89 50 4E 47 0D 0A 1A 0A             │ can_parse()     │
  │ GIF        │ 47 49 46 38 [37|39] 61              │ can_parse()     │
  │ WebP       │ 52 49 46 46 ?? ?? ?? ?? 57 45 42 50 │ can_parse()     │
  │ TIFF LE    │ 49 49 2A 00                         │ can_parse()     │
  │ TIFF BE    │ 4D 4D 00 2A                         │ can_parse()     │
  │ MP4/MOV    │ ?? ?? ?? ?? 66 74 79 70             │ can_parse()     │
  │ HEIC       │ ?? ?? ?? ?? 66 74 79 70 68 65 69 63 │ can_parse()     │
  │ PDF        │ 25 50 44 46                         │ can_parse()     │
  │ EXR        │ 76 2F 31 01                         │ can_parse()     │
  │ RAW (Sony) │ 49 49 2A 00 (TIFF)                  │ by_extension()  │
  │ RAW (Nikon)│ 4D 4D 00 2A (TIFF)                  │ by_extension()  │
  └────────────┴─────────────────────────────────────┴─────────────────┘
```

### 4. JPEG Parsing Internals

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                          JPEG PARSING DATAFLOW                                │
└──────────────────────────────────────────────────────────────────────────────┘

  JPEG File Structure                   Parsing Steps                Metadata
  ┌────────────────┐            ┌──────────────────────┐        ┌────────────┐
  │ FFD8           │───────────▶│ SOI Marker           │        │            │
  ├────────────────┤            └──────────────────────┘        │            │
  │ FFE0 (APP0)    │───────────▶│ JFIF (skip)          │        │            │
  ├────────────────┤            └──────────────────────┘        │            │
  │ FFE1 (APP1)    │───────────▶│ Check: "Exif\0\0"    │───────▶│   EXIF     │
  │  "Exif\0\0"    │            │ → parse_tiff_header  │        │  (Attrs)   │
  │  TIFF header   │            │ → read_ifd(0)        │        │            │
  │  IFD0          │            │ → read_ifd(ExifIFD)  │        │            │
  │  ExifIFD       │            │ → read_makernotes    │        │            │
  │  MakerNotes    │            │                      │        │            │
  ├────────────────┤            └──────────────────────┘        ├────────────┤
  │ FFE1 (APP1)    │───────────▶│ Check: "http://ns.."│───────▶│    XMP     │
  │  XMP/RDF       │            │ → XmpParser::parse   │        │  (Attrs)   │
  ├────────────────┤            └──────────────────────┘        ├────────────┤
  │ FFE2 (APP2)    │───────────▶│ Check: "ICC_PROFILE"│───────▶│    ICC     │
  │  ICC Profile   │            │ → IccParser::parse   │        │  (Attrs)   │
  ├────────────────┤            └──────────────────────┘        ├────────────┤
  │ FFED (APP13)   │───────────▶│ Check: "Photoshop"  │───────▶│   IPTC     │
  │  IPTC-IIM      │            │ → IptcParser::parse  │        │  (Attrs)   │
  ├────────────────┤            └──────────────────────┘        ├────────────┤
  │ FFC0/FFC2(SOF) │───────────▶│ Image dimensions     │───────▶│  width/    │
  │                │            │ width, height, depth │        │  height    │
  ├────────────────┤            └──────────────────────┘        ├────────────┤
  │ FFDA (SOS)     │───────────▶│ Start of Scan        │        │            │
  │ Image Data     │            │ (skip for metadata)  │        │            │
  ├────────────────┤            └──────────────────────┘        │            │
  │ FFD9 (EOI)     │───────────▶│ End of Image         │        │            │
  └────────────────┘            └──────────────────────┘        └────────────┘


  IFD Parsing Detail (EXIF):
  ┌──────────────────────────────────────────────────────────────────────────┐
  │                                                                          │
  │   TIFF Header     IFD0 (Main)        ExifIFD           MakerNotes        │
  │   ┌─────────┐    ┌───────────┐     ┌───────────┐     ┌───────────┐      │
  │   │ByteOrder│───▶│ Make      │────▶│DateTimeOrig────▶│ Vendor    │      │
  │   │ II / MM │    │ Model     │     │ ExposureTime    │ specific  │      │
  │   │ 0x002A  │    │ DateTime  │     │ FNumber    │    │ tags      │      │
  │   │IFD0 off │    │*ExifIFD*──│     │ ISO        │    │           │      │
  │   └─────────┘    │ GPS IFD   │     │ Lens       │    │ Canon     │      │
  │                  │ ...       │     │*MakerNotes*│    │ Nikon     │      │
  │                  └───────────┘     └───────────┘    │ Sony      │      │
  │                                                      │ etc (21)  │      │
  │                                                      └───────────┘      │
  └──────────────────────────────────────────────────────────────────────────┘
```

### 5. MakerNotes Dispatch

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                         MAKERNOTES VENDOR DISPATCH                            │
└──────────────────────────────────────────────────────────────────────────────┘

  Detection Input              Vendor Detection              Vendor Parser
  ┌─────────────────┐    ┌───────────────────────────┐    ┌─────────────────┐
  │ Make: "Canon"   │───▶│ make_to_vendor(make)      │───▶│ CanonParser     │
  │ MakerNotes data │    │                           │    │ makernotes/     │
  └─────────────────┘    │ match make {              │    │ canon.rs        │
                         │   "Canon" => Canon        │    └─────────────────┘
                         │   "NIKON" => Nikon        │
                         │   "Sony"  => Sony         │
                         │   "Apple" => Apple        │
                         │   "DJI"   => DJI          │
                         │   ...                     │
                         │ }                         │
                         └───────────────────────────┘

  VendorParser Trait:
  ┌──────────────────────────────────────────────────────────────────────────┐
  │                                                                          │
  │  pub trait VendorParser {                                                │
  │      fn parse(&self, data: &[u8], byte_order: ByteOrder) -> Option<Attrs>│
  │  }                                                                       │
  │                                                                          │
  │  Implementations (21 vendors):                                           │
  │  ┌──────────┬──────────┬──────────┬──────────┬──────────┬──────────┐    │
  │  │ Canon    │ Nikon    │ Sony     │ Fujifilm │ Olympus  │ Panasonic│    │
  │  ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤    │
  │  │ Pentax   │ Samsung  │ Apple    │ DJI      │ GoPro    │ Hasselblad   │
  │  ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤    │
  │  │ Leica    │ Sigma    │ Ricoh    │ Kodak    │ Minolta  │ Casio    │    │
  │  ├──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤    │
  │  │ Epson    │ Motorola │ Sanyo    │          │          │          │    │
  │  └──────────┴──────────┴──────────┴──────────┴──────────┴──────────┘    │
  │                                                                          │
  └──────────────────────────────────────────────────────────────────────────┘
```

---

## Trait Hierarchy

### Core Traits

```rust
// crates/exiftool-formats/src/traits.rs

/// Marker trait for read+seek capability
pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek> ReadSeek for T {}

/// Format parser interface - implement for each file format
pub trait FormatParser: Send + Sync {
    /// Check if parser can handle this file by magic bytes
    fn can_parse(&self, header: &[u8]) -> bool;
    
    /// Human-readable format name (e.g., "JPEG", "PNG")
    fn format_name(&self) -> &'static str;
    
    /// File extensions this parser handles
    fn extensions(&self) -> &'static [&'static str];
    
    /// Parse file and extract metadata
    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata>;
}

/// Format writer interface - extends FormatParser
/// NOTE: Currently NOT IMPLEMENTED by any writer!
pub trait FormatWriter: FormatParser {
    fn can_write(&self) -> bool { true }
    fn write(&self, source: &mut dyn ReadSeek, 
             dest: &mut dyn Write, 
             metadata: &Metadata) -> Result<()>;
}
```

### Implementation Pattern

```rust
// Typical parser implementation pattern:

pub struct JpegParser;  // Unit struct (stateless)

impl FormatParser for JpegParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 3 && header[0..2] == [0xFF, 0xD8]
    }
    
    fn format_name(&self) -> &'static str { "JPEG" }
    
    fn extensions(&self) -> &'static [&'static str] {
        &["jpg", "jpeg", "jpe", "jfif"]
    }
    
    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        // Implementation...
    }
}

// For TIFF-based formats (with state):

pub struct ArwParser {
    tiff_parser: TiffParser,  // Embedded parser
}

impl ArwParser {
    pub fn new() -> Self {
        Self {
            tiff_parser: TiffParser::with_config(Config {
                vendor: Some(Vendor::Sony),
                ..Default::default()
            }),
        }
    }
}
```

---

## Error Propagation

### Error Type Hierarchy

```
                    ┌────────────────────┐
                    │  std::io::Error    │
                    └─────────┬──────────┘
                              │ #[from]
              ┌───────────────┼───────────────┐
              │               │               │
              ▼               ▼               ▼
    ┌─────────────────┐ ┌───────────┐ ┌──────────────┐
    │exiftool_core::  │ │quick_xml::│ │serde_json::  │
    │     Error       │ │   Error   │ │    Error     │
    └────────┬────────┘ └─────┬─────┘ └──────┬───────┘
             │                │               │
             │ #[from]        │ #[from]       │ #[from]
             │                │               │
             ▼                ▼               ▼
    ┌─────────────────────────────────────────────────┐
    │           exiftool_formats::Error               │
    │                                                 │
    │   • UnsupportedFormat                           │
    │   • InvalidStructure(String)                    │
    │   • MissingSegment(&'static str)                │
    │   • Core(exiftool_core::Error)     ← #[from]    │
    │   • Io(std::io::Error)             ← #[from]    │
    │   • FileTooLarge(u64, u64)                      │
    │   • Xmp(exiftool_xmp::Error)       ← #[from] ✓ FIXED
    │   • Icc(exiftool_icc::Error)       ← #[from] ✓ FIXED
    │   • Iptc(exiftool_iptc::Error)     ← #[from] ✓ FIXED
    └─────────────────────────────────────────────────┘
                              │
                              │ (CLI consumes)
                              ▼
                    ┌─────────────────────┐
                    │   anyhow::Error     │
                    │    (CLI only)       │
                    └─────────────────────┘


  ✅ FIXED ISSUES (from plan1.md):

  1. ✓ exiftool_xmp::Error::Io - NOW uses #[from] std::io::Error
  2. ✓ From impl: exiftool_xmp::Error → exiftool_formats::Error (line 27)
  3. ✓ From impl: exiftool_icc::Error → exiftool_formats::Error (line 30)
  4. ✓ From impl: exiftool_iptc::Error → exiftool_formats::Error (line 33)
```

### Error Handling Best Practices

```rust
// GOOD: Use ? operator with proper From impl
fn parse_jpeg(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
    reader.seek(SeekFrom::Start(0))?;  // io::Error → Error via From
    let tiff = parse_tiff_header(data)?;  // core::Error → Error via From
    Ok(metadata)
}

// BAD: Silent error swallowing
dict.set_item("key", value).unwrap();  // PyO3 can fail!

// BETTER: Handle or propagate
dict.set_item("key", value).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
```

---

## Key Code Paths

### CLI Entry Point

```
main.rs:main()
    │
    ├─► parse_args() → Cli struct
    │
    ├─► if args.files.is_empty() → print_help()
    │
    ├─► for file in args.files:
    │       │
    │       ├─► process_file(file, &args)
    │       │       │
    │       │       ├─► open_file(path) → BufReader
    │       │       │
    │       │       ├─► detect_format(&registry, reader)
    │       │       │
    │       │       ├─► parser.parse(reader) → Metadata
    │       │       │
    │       │       └─► if args.write:
    │       │               write_metadata(metadata, writer)
    │       │           else:
    │       │               format_output(metadata, &args)
    │       │
    │       └─► handle_error(e)
    │
    └─► exit(0)
```

### Format Registration

```
registry.rs:FormatRegistry::default()
    │
    ├─► register(Box::new(JpegParser))
    ├─► register(Box::new(PngParser))
    ├─► register(Box::new(TiffParser::default()))
    ├─► register(Box::new(WebpParser::new()))
    ├─► register(Box::new(GifParser))
    ├─► register(Box::new(BmpParser))
    ├─► register(Box::new(Mp4Parser))
    ├─► register(Box::new(HeicParser))
    ├─► register(Box::new(Cr2Parser::new()))
    ├─► register(Box::new(ArwParser::new()))
    ├─► ... (90+ formats total)
    │
    └─► FormatRegistry { parsers: Vec<Box<dyn FormatParser>> }
```

---

## Critical Files Reference

### Must-Know Files

| File | Purpose | Lines | Complexity |
|------|---------|-------|------------|
| `formats/src/traits.rs` | Core trait definitions | ~50 | Low |
| `formats/src/lib.rs` | Metadata struct, exports | ~400 | Medium |
| `formats/src/registry.rs` | Format detection | ~200 | Medium |
| `formats/src/jpeg.rs` | JPEG parser | ~600 | High |
| `formats/src/tiff.rs` | TIFF/RAW parser | ~800 | High |
| `formats/src/mp4.rs` | MP4/MOV parser | ~900 | High |
| `formats/src/makernotes/mod.rs` | MakerNotes dispatch | ~300 | Medium |
| `core/src/ifd.rs` | IFD/TIFF parsing | ~400 | High |
| `attrs/src/lib.rs` | Attribute storage | ~300 | Medium |
| `cli/src/main.rs` | CLI application | ~600 | Medium |

### Writers (Separate from Parsers)

| File | Supports |
|------|----------|
| `jpeg_writer.rs` | JPEG (APP1 EXIF/XMP) |
| `png_writer.rs` | PNG (tEXt, iTXt chunks) |
| `tiff_writer.rs` | TIFF, DNG |
| `webp_writer.rs` | WebP (EXIF chunk) |
| `heic_writer.rs` | HEIC/HEIF/AVIF |
| `exr_writer.rs` | OpenEXR |
| `hdr_writer.rs` | Radiance HDR |

---

## Known Issues & TODOs

### Active TODOs in Code (2 items)

| Location | Issue | Status |
|----------|-------|--------|
| `exiftool-py/src/scan.rs:62` | `TODO: collect errors for reporting` | OPEN |
| `exiftool-formats/src/heic_writer.rs:861` | `TODO: Full implementation would...` | LOW PRIORITY |

### Architecture Issues

1. **FormatWriter trait not implemented** - Writers use standalone functions
2. **Inconsistent writer signatures** - JpegWriter differs from others
3. **No writer registry** - CLI uses match statement
4. **set_file_type() method unused** - Added at `lib.rs:399` but 90+ parsers use old pattern
5. **Dead code with #[allow(dead_code)]** - 13 locations (some justified, see plan2.md)

### ✅ FIXED Issues (plan1.md)

1. ✓ **XMP: Io(String)** - NOW uses `#[from] std::io::Error`
2. ✓ **PyO3 unwrap()** - NOW uses `let _ = dict.set_item()` pattern
3. ✓ **Missing From impls** - XMP/ICC/IPTC errors NOW convert to formats::Error
4. ✓ **MakerNotes IFD parsing** - `parse_ifd_entries()` helper added, used by 26+ parsers
5. ✓ **get_file_size()** helper added to utils.rs, used by 27+ parsers
6. ✓ **RIFF parsing** - `riff.rs` module created with common parsing

### Remaining Code Patterns to Unify

1. **File type setting** - `set("File:FileType", ...)` (90+ times) → should use `set_file_type()`
2. **nikon.rs** - Line 154 still uses `IfdReader::new` directly instead of helper

---

## Appendix: Format Support Matrix

### Read Support (90+ formats)

| Category | Formats |
|----------|---------|
| **Images** | JPEG, PNG, GIF, BMP, TIFF, WebP, HEIC/HEIF/AVIF, EXR, HDR, PSD, DNG, SVG |
| **RAW** | CR2, CR3, ARW, NEF, ORF, RAF, RW2, PEF, SRW, ERF, 3FR, FFF, DCR, KDC, MRW |
| **Video** | MP4, MOV, AVI, MKV, WebM, FLV, MXF, MPEG-TS, R3D, BRAW |
| **Audio** | MP3, WAV, FLAC, OGG, AAC, ALAC, APE, DSF, DFF, TAK, WV, MID |
| **Documents** | PDF |
| **Other** | ICC profiles, XMP sidecar |

### Write Support (10 formats)

| Format | EXIF | XMP | Notes |
|--------|------|-----|-------|
| JPEG | ✓ | ✓ | Full support |
| PNG | ✓ | ✓ | tEXt/iTXt chunks |
| TIFF | ✓ | ✓ | Full IFD rewrite |
| DNG | ✓ | ✓ | Via TIFF writer |
| WebP | ✓ | - | EXIF chunk only |
| HEIC/HEIF | ✓ | ✓ | Limited (TODO) |
| AVIF | ✓ | ✓ | Via HEIC writer |
| EXR | ✓ | - | Header attributes |
| HDR | ✓ | - | Header comments |

---

*Document generated for AI agents working on exiftool-rs codebase.*

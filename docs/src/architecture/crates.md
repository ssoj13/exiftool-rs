# Crate Structure

## Dependency Graph

```
exiftool-cli ─────┐
                  ▼
exiftool-py ──► exiftool-formats
                  │
        ┌─────────┼─────────┬─────────┐
        ▼         ▼         ▼         ▼
   exiftool-  exiftool-  exiftool-  exiftool-
     xmp       iptc       icc       tags
        │         │                   │
        └─────────┴─────────┬─────────┘
                            ▼
                      exiftool-core
                            │
                            ▼
                      exiftool-attrs
```

## exiftool-attrs

Foundation crate. Defines `Attrs` (attribute map) and `AttrValue` (typed values).

```rust
use exiftool_attrs::{Attrs, AttrValue};

let mut attrs = Attrs::new();
attrs.set("Make", AttrValue::Str("Canon".into()));
attrs.set("ISO", AttrValue::UInt(400));
```

**Key types:**
- `Attrs` - HashMap-like container with typed accessors
- `AttrValue` - Enum of possible value types (Str, Int, UInt, Float, Rational, List, Bytes)

## exiftool-core

Low-level TIFF/EXIF primitives.

```rust
use exiftool_core::{ByteOrder, IfdReader};

let reader = IfdReader::new(data, ByteOrder::LittleEndian);
let ifd_offset = reader.parse_header()?;
let (entries, next_ifd) = reader.read_ifd(ifd_offset)?;
```

**Key types:**
- `IfdReader` - Parse IFD structures from bytes
- `ExifWriter` - Build EXIF byte streams
- `ByteOrder` - Big/Little endian handling
- `IfdEntry`, `RawValue` - Parsed IFD data

## exiftool-tags

Tag definitions and value interpretation.

```rust
use exiftool_tags::interp;

// Interpret numeric values
interp::interpret_value("Orientation", 6)  // "Rotate 90 CW"

// Format with units
interp::format_focal_length(50.0)  // "50 mm"
interp::format_fnumber(2.8)        // "f/2.8"
```

**Features:**
- Tag ID to name mapping
- Value interpretation (enums, bitfields)
- Display formatting

## exiftool-formats

All format parsers and writers. This is the main crate most users need.

```rust
use exiftool_formats::{FormatRegistry, FormatParser, JpegWriter};

let registry = FormatRegistry::new();
let metadata = registry.parse(&mut reader)?;
```

**Key types:**
- `FormatRegistry` - Auto-detect and parse any format
- `FormatParser` trait - Common interface for all parsers
- `Metadata` - Parsed result with EXIF, XMP, thumbnails
- `*Parser` - Format-specific parsers (JpegParser, etc.)
- `*Writer` - Format-specific writers

## exiftool-xmp

XMP (XML) metadata parsing and writing.

```rust
use exiftool_xmp::{parse_xmp, XmpWriter};

let attrs = parse_xmp(xml_string)?;
let xml = XmpWriter::build(&attrs)?;
```

## exiftool-iptc

IPTC-IIM metadata for JPEG APP13 segments.

```rust
use exiftool_iptc::{IptcParser, IptcWriter};

let attrs = IptcParser::parse(data)?;
let bytes = IptcWriter::build(&attrs);
```

## exiftool-icc

ICC color profile parsing.

```rust
use exiftool_icc::parse_icc_profile;

let attrs = parse_icc_profile(data)?;
// ProfileDescription, ColorSpace, etc.
```

## exiftool-py

Python bindings via PyO3.

```python
import exiftool_rs as exif
img = exif.open("photo.jpg")
```

## exiftool-cli

Command-line tool.

```bash
exif photo.jpg
exif -j *.jpg
```

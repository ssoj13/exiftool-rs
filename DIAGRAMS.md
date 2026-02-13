# exiftool-rs Diagrams (Mermaid)

## Crate Dependency Graph

```mermaid
graph TD
    subgraph "Application Layer"
        CLI[exiftool-cli]
        PY[exiftool-py]
    end

    subgraph "Format Layer"
        FORMATS[exiftool-formats]
    end

    subgraph "Core Layer"
        CORE[exiftool-core]
        ATTRS[exiftool-attrs]
        TAGS[exiftool-tags]
        XMP[exiftool-xmp]
        IPTC[exiftool-iptc]
        ICC[exiftool-icc]
    end

    CLI --> FORMATS
    PY --> FORMATS
    FORMATS --> CORE
    FORMATS --> ATTRS
    FORMATS --> TAGS
    FORMATS --> XMP
    FORMATS --> IPTC
    FORMATS --> ICC
    CORE --> ATTRS
```

## Parse Flow (Sequence)

```mermaid
sequenceDiagram
    participant User
    participant Registry
    participant Parser
    participant IfdReader
    participant Utils

    User->>Registry: parse(reader)
    Registry->>Registry: detect(header)
    Registry->>Parser: parse(reader)
    
    alt TIFF-based format
        Parser->>IfdReader: new(data, byte_order)
        Parser->>IfdReader: read_ifd(offset)
        loop For each IFD entry
            IfdReader-->>Parser: IfdEntry
            Parser->>Utils: entry_to_attr(entry)
            Parser->>Parser: metadata.exif.set(name, attr)
        end
    end
    
    Parser-->>Registry: Metadata
    Registry-->>User: Metadata
```

## Metadata Structure

```mermaid
classDiagram
    class Metadata {
        +format: &'static str
        +exif: Attrs
        +exif_offset: Option~usize~
        +xmp: Option~String~
        +thumbnail: Option~Vec~u8~~
        +preview: Option~Vec~u8~~
        +icc: Option~Vec~u8~~
        +pages: Vec~PageInfo~
        +is_camera_raw()
        +is_writable()
        +get_interpreted()
        +get_display()
    }

    class Attrs {
        +get()
        +get_str()
        +get_u32()
        +set()
        +iter()
    }

    class AttrValue {
        <<enum>>
        Str
        Int
        UInt
        Float
        Rational
        URational
        Bytes
        DateTime
        List
        Map
    }

    Metadata *-- Attrs : contains
    Attrs *-- AttrValue : values
```

## Format Parser Hierarchy

```mermaid
flowchart TB
    subgraph "Image Formats"
        JPEG[JpegParser]
        PNG[PngParser]
        TIFF[TiffParser]
        WEBP[WebpParser]
        HEIC[HeicParser]
        EXR[ExrParser]
    end

    subgraph "RAW Formats"
        CR2[Cr2Parser]
        CR3[Cr3Parser]
        NEF[NefParser]
        ARW[ArwParser]
        RAF[RafParser]
    end

    subgraph "Audio/Video"
        MP4[Mp4Parser]
        ID3[Id3Parser]
        FLAC[FlacParser]
    end

    Registry[FormatRegistry] --> JPEG
    Registry --> PNG
    Registry --> TIFF
    Registry --> WEBP
    Registry --> HEIC
    Registry --> CR2
    Registry --> CR3
    Registry --> NEF
    Registry --> ARW
    Registry --> RAF
    Registry --> MP4
    Registry --> ID3
```

## EXIF Sub-IFD Structure

```mermaid
graph TB
    IFD0[IFD0 Primary] --> |0x8769| EXIF[ExifIFD]
    IFD0 --> |0x8825| GPS[GPS IFD]
    IFD0 --> |0xA005| INTEROP[Interop IFD]
    IFD0 --> |next| IFD1[IFD1 Thumbnail]
    
    EXIF --> |0x927C| MN[MakerNotes]
    
    EXIF --- E1[ExposureTime, FNumber, ISO...]
    GPS --- G1[Latitude, Longitude, Altitude...]
    INTEROP --- I1[InteropIndex, InteropVersion]
```

## Write Path (JPEG Example)

```mermaid
sequenceDiagram
    participant User
    participant Metadata
    participant Utils
    participant ExifWriter
    participant JpegWriter

    User->>Metadata: exif.set("Artist", "John")
    User->>Utils: build_exif_bytes(metadata)
    Utils->>ExifWriter: add_ifd0(), add_exif()
    ExifWriter->>ExifWriter: serialize()
    ExifWriter-->>Utils: Vec~u8~
    Utils-->>User: exif_bytes
    
    User->>JpegWriter: write(input, output, exif_bytes, None)
    JpegWriter->>JpegWriter: Copy segments, replace APP1 EXIF
    JpegWriter-->>User: Ok
```

## Tag Lookup Flow

```mermaid
flowchart LR
    TagID[u16 tag id] --> lookup_ifd0
    lookup_ifd0 --> |0x8769| ExifOffset
    lookup_ifd0 --> |0x8825| GPSInfo
    lookup_ifd0 --> |0xA005| InteropOffset
    lookup_ifd0 --> |_| lookup_exif
    
    lookup_exif --> EXIF_MAIN[exiftool-tags generated]
    EXIF_MAIN --> Name[tag name string]
    
    GPS_MAIN[GPS_MAIN table] --> lookup_gps
```

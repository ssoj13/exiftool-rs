# ExifTool Source Reference

Quick navigation for ExifTool Perl source code in ./_ref/exiftool

## Module Size Overview (TOP 25)

| Module | Lines | Purpose |
|--------|-------|---------|
| Nikon.pm | 14234 | Nikon MakerNotes |
| TagLookup.pm | 13957 | Auto-generated tag index |
| Sony.pm | 11878 | Sony MakerNotes |
| NikonCustom.pm | 11411 | Nikon custom settings |
| Canon.pm | 10766 | Canon MakerNotes |
| QuickTime.pm | 10727 | MP4/MOV/HEIC container |
| Exif.pm | 7221 | Core EXIF tags |
| Pentax.pm | 6930 | Pentax MakerNotes |
| XMP.pm | 4656 | XMP (Adobe XML metadata) |
| Olympus.pm | 4462 | Olympus MakerNotes |
| DICOM.pm | 3881 | Medical imaging |
| Kodak.pm | 3280 | Kodak MakerNotes |
| MXF.pm | 3031 | Pro video format |
| Panasonic.pm | 2994 | Panasonic MakerNotes |
| Minolta.pm | 2960 | Minolta/Sony legacy |
| CanonCustom.pm | 2900 | Canon custom functions |

---

## Directory Structure

```
_ref/exiftool/
├── exiftool                    # CLI entry point (Perl script)
├── lib/
│   ├── File/
│   │   └── RandomAccess.pm     # Buffered file I/O
│   └── Image/
│       ├── ExifTool.pm         # CORE: Main module (~8000 lines)
│       └── ExifTool/
│           ├── [Core]
│           │   ├── Exif.pm         # EXIF standard (IFD, TIFF tags)
│           │   ├── TagLookup.pm    # Auto-generated tag index
│           │   ├── MakerNotes.pm   # MakerNotes dispatcher
│           │   ├── Writer.pl       # Write logic
│           │   ├── Fixup.pm        # Offset fixups after write
│           │   └── Shortcuts.pm    # Tag shortcuts/aliases
│           │
│           ├── [Metadata Standards]
│           │   ├── XMP.pm          # Adobe XMP (XML)
│           │   ├── XMP2.pl         # XMP namespaces
│           │   ├── IPTC.pm         # Press/news metadata
│           │   ├── ICC_Profile.pm  # Color profiles
│           │   ├── GPS.pm          # GPS tags
│           │   ├── GeoTiff.pm      # GeoTIFF tags
│           │   └── Geotag.pm       # GPS track matching
│           │
│           ├── [Image Formats]
│           │   ├── JPEG.pm         # JPEG segments (APP0-15)
│           │   ├── PNG.pm          # PNG chunks
│           │   ├── GIF.pm          # GIF metadata
│           │   ├── BMP.pm          # Windows bitmap
│           │   ├── TIFF.pm         # (uses Exif.pm mostly)
│           │   ├── PSD.pm          # Photoshop (see Photoshop.pm)
│           │   ├── Photoshop.pm    # PSD/PSB
│           │   ├── PDF.pm          # PDF documents
│           │   ├── PostScript.pm   # EPS/AI
│           │   └── ...
│           │
│           ├── [RAW Formats]
│           │   ├── CanonRaw.pm     # CRW/CR2/CR3
│           │   ├── FujiFilm.pm     # RAF (+ MakerNotes)
│           │   ├── Nikon.pm        # NEF/NRW (+ MakerNotes)
│           │   ├── Sony.pm         # ARW (+ MakerNotes)
│           │   ├── Olympus.pm      # ORF (+ MakerNotes)
│           │   ├── PanasonicRaw.pm # RW2
│           │   ├── Pentax.pm       # PEF (+ MakerNotes)
│           │   ├── SigmaRaw.pm     # X3F
│           │   ├── MinoltaRaw.pm   # MRW
│           │   ├── KyoceraRaw.pm   # Contax RAW
│           │   └── DNG.pm          # Adobe DNG
│           │
│           ├── [Video/Audio]
│           │   ├── QuickTime.pm    # MP4/MOV/M4A/HEIC
│           │   ├── QuickTimeStream.pl # Streaming data
│           │   ├── RIFF.pm         # AVI/WAV/WebP
│           │   ├── Matroska.pm     # MKV/WebM
│           │   ├── MPEG.pm         # MPEG video
│           │   ├── ASF.pm          # WMV/WMA
│           │   ├── FLAC.pm         # FLAC audio
│           │   ├── ID3.pm          # MP3 tags
│           │   ├── Ogg.pm          # Ogg container
│           │   ├── Vorbis.pm       # Vorbis comments
│           │   └── ...
│           │
│           ├── [MakerNotes - Camera Vendors]
│           │   ├── Canon.pm            # Canon EXIF maker notes
│           │   ├── CanonCustom.pm      # Canon custom functions
│           │   ├── CanonVRD.pm         # Canon DPP recipes
│           │   ├── Nikon.pm            # Nikon maker notes
│           │   ├── NikonCustom.pm      # Nikon menu settings
│           │   ├── NikonCapture.pm     # Nikon NX settings
│           │   ├── NikonSettings.pm    # More Nikon settings
│           │   ├── Sony.pm             # Sony maker notes
│           │   ├── SonyIDC.pm          # Sony IDC data
│           │   ├── FujiFilm.pm         # Fuji maker notes
│           │   ├── Olympus.pm          # Olympus maker notes
│           │   ├── Panasonic.pm        # Panasonic maker notes
│           │   ├── Pentax.pm           # Pentax maker notes
│           │   ├── Minolta.pm          # Minolta/Konica
│           │   ├── Samsung.pm          # Samsung
│           │   ├── Ricoh.pm            # Ricoh/GR
│           │   ├── Casio.pm            # Casio
│           │   ├── Kodak.pm            # Kodak
│           │   ├── HP.pm               # HP
│           │   ├── GoPro.pm            # GoPro
│           │   ├── DJI.pm              # DJI drones
│           │   └── ...
│           │
│           ├── [Writers]
│           │   ├── Writer.pl           # Main write logic
│           │   ├── WriteExif.pl        # EXIF/TIFF writer
│           │   ├── WriteXMP.pl         # XMP writer
│           │   ├── WriteIPTC.pl        # IPTC writer
│           │   ├── WritePNG.pl         # PNG writer
│           │   ├── WritePDF.pl         # PDF writer
│           │   ├── WritePhotoshop.pl   # PSD writer
│           │   ├── WritePostScript.pl  # EPS writer
│           │   ├── WriteQuickTime.pl   # MOV/MP4 writer
│           │   ├── WriteRIFF.pl        # AVI/WebP writer
│           │   └── WriteCanonRaw.pl    # CRW writer
│           │
│           ├── [Charset Encodings]
│           │   └── Charset/
│           │       ├── Latin.pm
│           │       ├── Cyrillic.pm
│           │       ├── ShiftJIS.pm
│           │       ├── MacJapanese.pm
│           │       └── ... (32 encodings)
│           │
│           └── [Localization]
│               └── Lang/
│                   ├── en.pm (default)
│                   ├── ru.pm
│                   ├── de.pm
│                   ├── ja.pm
│                   └── ... (18 languages)
```

---

## Key Concepts

### 1. Tag Tables

Every tag is defined in a Perl hash like this:

```perl
%Image::ExifTool::Exif::Main = (
    GROUPS => { 0 => 'EXIF', 1 => 'IFD0', 2 => 'Image' },
    WRITE_PROC => \&WriteExif,
    CHECK_PROC => \&CheckExif,
    
    0x010f => {               # Tag ID (hex)
        Name => 'Make',       # Tag name
        Writable => 'string', # Can write, type is string
        Groups => { 2 => 'Camera' },
    },
    0x0110 => {
        Name => 'Model',
        Writable => 'string',
    },
    0x8769 => {
        Name => 'ExifOffset',
        SubDirectory => {     # Points to nested IFD
            TagTable => 'Image::ExifTool::Exif::Main',
            DirName => 'ExifIFD',
        },
    },
);
```

### 2. EXIF Data Types (Exif.pm:75)

```perl
@formatSize = (undef,1,1,2,4,8,1,1,2,4,8,4,8,4,2,8,8,8,8);
@formatName = (
    undef,         # 0
    'int8u',       # 1  = BYTE
    'string',      # 2  = ASCII  
    'int16u',      # 3  = SHORT
    'int32u',      # 4  = LONG
    'rational64u', # 5  = RATIONAL (2x uint32)
    'int8s',       # 6  = SBYTE
    'undef',       # 7  = UNDEFINED
    'int16s',      # 8  = SSHORT
    'int32s',      # 9  = SLONG
    'rational64s', # 10 = SRATIONAL
    'float',       # 11 = FLOAT
    'double',      # 12 = DOUBLE
    'ifd',         # 13 = IFD pointer
    'unicode',     # 14
    'complex',     # 15
    'int64u',      # 16 = BigTIFF
    'int64s',      # 17 = BigTIFF
    'ifd64',       # 18 = BigTIFF
);
```

### 3. IFD Structure (Image File Directory)

```
+----------------+
| IFD Header     |
| - Entry Count  | (2 bytes)
+----------------+
| Entry 0        | (12 bytes each)
| - Tag ID       | (2 bytes)
| - Type         | (2 bytes) 
| - Count        | (4 bytes)
| - Value/Offset | (4 bytes)
+----------------+
| Entry 1        |
| ...            |
+----------------+
| Next IFD Offset| (4 bytes, 0 = end)
+----------------+
```

### 4. MakerNotes

**What:** Proprietary binary blobs in EXIF, each vendor invents their own format.

**Where:** EXIF tag 0x927c (MakerNote) contains binary data.

**Detection:** MakerNotes.pm checks headers:
- Canon: starts with IFD directly
- Nikon: `"Nikon\x00\x02"` header + TIFF
- Fuji: `"FUJIFILM"` header + custom IFD
- Sony: varies by model

**Known formats:**
- Most use standard IFD structure (Canon, Sony, Pentax)
- Some have custom headers (Nikon, Fuji, Olympus)
- Some are encrypted (Nikon D4+, some Canon)
- Some are undocumented binary blobs

### 5. XMP (Extensible Metadata Platform)

**What:** Adobe's XML-based metadata format, embedded in files.

**Where:**
- JPEG: APP1 segment with `"http://ns.adobe.com/xap/1.0/"` prefix
- TIFF: Tag 0x02bc
- PNG: iTXt chunk with keyword "XML:com.adobe.xmp"
- PDF: XMP stream

**Structure:** RDF/XML with namespaces:
```xml
<x:xmpmeta xmlns:x="adobe:ns:meta/">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
    <rdf:Description rdf:about=""
      xmlns:dc="http://purl.org/dc/elements/1.1/"
      xmlns:exif="http://ns.adobe.com/exif/1.0/">
      <dc:creator>John Doe</dc:creator>
      <exif:DateTimeOriginal>2024-01-15</exif:DateTimeOriginal>
    </rdf:Description>
  </rdf:RDF>
</x:xmpmeta>
```

### 6. RAW Format Containers

| Format | Extension | Container | MakerNotes | Notes |
|--------|-----------|-----------|------------|-------|
| Canon CR2 | .cr2 | TIFF | Canon IFD | Standard TIFF with extra IFDs |
| Canon CR3 | .cr3 | ISO BMFF | Canon CRAW | Same container as HEIC |
| Canon CRW | .crw | CIFF | Canon CIFF | Old proprietary format |
| Nikon NEF | .nef | TIFF | Nikon IFD | Sometimes encrypted |
| Fuji RAF | .raf | Custom | FUJIFILM | Custom header + JPEG preview + raw data |
| Sony ARW | .arw | TIFF | Sony IFD | Standard TIFF |
| Olympus ORF | .orf | TIFF | Olympus IFD | Has own byte order |
| Panasonic RW2 | .rw2 | TIFF | Panasonic IFD | Standard TIFF |

---

## CLI Output Format

```bash
# Default output (human readable)
$ exiftool photo.jpg
ExifTool Version Number         : 13.44
File Name                       : photo.jpg
Directory                       : .
File Size                       : 3.2 MB
...
Make                            : Canon
Model                           : Canon EOS R5
...

# JSON output
$ exiftool -json photo.jpg
[{
  "SourceFile": "photo.jpg",
  "ExifToolVersion": 13.44,
  "FileName": "photo.jpg",
  "Make": "Canon",
  "Model": "Canon EOS R5"
}]

# Specific tags
$ exiftool -Make -Model -ISO photo.jpg

# Write tags
$ exiftool -Artist="John" photo.jpg

# Copy tags between files
$ exiftool -TagsFromFile src.jpg dst.jpg
```

---

## Important Functions (ExifTool.pm)

| Function | Purpose |
|----------|---------|
| `ImageInfo()` | Main entry: read metadata |
| `SetNewValue()` | Queue tag for writing |
| `WriteInfo()` | Write queued changes |
| `GetValue()` | Get tag value (raw/converted) |
| `GetTagList()` | List all found tags |
| `ReadValue()` | Read bytes in format |
| `WriteValue()` | Write bytes in format |
| `GetByteOrder()` | Current endianness |
| `SetByteOrder()` | Set endianness |
| `ProcessDirectory()` | Parse IFD structure |
| `WriteDirectory()` | Write IFD structure |

---

## Tag Sources (Priority Order)

1. **EXIF** - Binary, embedded in file
2. **XMP** - XML, can be embedded or sidecar
3. **IPTC** - Legacy press metadata
4. **MakerNotes** - Vendor proprietary
5. **File metadata** - OS level (name, size, dates)

---

## Files You'll Touch Most

For MVP (JPEG + EXIF + XMP):

1. `ExifTool.pm` - Core logic, understand this first
2. `Exif.pm` - EXIF tag definitions and IFD parsing
3. `JPEG.pm` - JPEG segment parsing
4. `XMP.pm` - XMP parsing (XML)
5. `MakerNotes.pm` - Dispatcher for vendor data
6. `Writer.pl` + `WriteExif.pl` - Writing logic

For RAW (RAF/NEF/CR2):

1. `FujiFilm.pm` - RAF container + Fuji MakerNotes
2. `Nikon.pm` - NEF parsing + Nikon MakerNotes  
3. `CanonRaw.pm` - CR2/CR3/CRW containers
4. `Canon.pm` - Canon MakerNotes

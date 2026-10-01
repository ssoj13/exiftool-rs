# Image Formats

## JPEG

JPEG stores EXIF and XMP in APP1 and IPTC in Photoshop APP13 segments.
Support depends on the embedded fields and segment layout.

| Feature | Support |
|---------|---------|
| Read | ✓ |
| Write | ✓ |
| EXIF | Supported fields |
| XMP | Packet parsing / writing |
| IPTC | Supported IPTC-IIM records |
| Thumbnail | ✓ |

**Extensions:** `.jpg`, `.jpeg`

## PNG

Portable Network Graphics with text chunks.

| Feature | Support |
|---------|---------|
| Read | ✓ |
| Write | ✓ |
| EXIF | Via eXIf chunk |
| XMP | Via iTXt chunk |
| Text | tEXt/iTXt chunks |

**Extensions:** `.png`

## TIFF

Tagged Image File Format. Basis for many RAW formats.

| Feature | Support |
|---------|---------|
| Read | ✓ |
| Write | ✓ |
| EXIF | Native |
| XMP | Packet parsing / writing |
| Multi-page | ✓ |
| BigTIFF | ✓ |

**Extensions:** `.tif`, `.tiff`

## WebP

Google's modern image format.

| Feature | Support |
|---------|---------|
| Read | ✓ |
| Write | ✓ |
| EXIF | Via EXIF chunk |
| XMP | Via XMP chunk |
| ICC | Via ICCP chunk |

**Extensions:** `.webp`

## HEIC/HEIF/AVIF

Modern container formats using ISOBMFF structure.

| Feature | Support |
|---------|---------|
| Read | ✓ |
| Write | ✓ (update existing EXIF or add EXIF item) |
| EXIF | Via meta box |
| XMP | Via meta box |

**Extensions:** `.heic`, `.heif`, `.avif`

## GIF

Graphics Interchange Format.

| Feature | Support |
|---------|---------|
| Read | ✓ |
| Write | ✓ (supported comment metadata) |
| Comments | ✓ |
| Animation | Frame count |

**Extensions:** `.gif`

## BMP

Windows Bitmap.

| Feature | Support |
|---------|---------|
| Read | ✓ |
| Write | ✗ |
| Dimensions | ✓ |
| Bit depth | ✓ |

**Extensions:** `.bmp`

## Other Image Formats

| Format | Extensions | Notes |
|--------|------------|-------|
| ICO | .ico | Windows icon |
| TGA | .tga | Truevision |
| PCX | .pcx | PC Paintbrush |
| PNM | .ppm, .pgm, .pbm | Netpbm |
| SGI | .sgi, .rgb | Silicon Graphics |
| DPX | .dpx | Digital Picture Exchange |
| EXR | .exr | OpenEXR (HDR) |
| HDR | .hdr | Radiance RGBE |
| JPEG XL | .jxl | Modern JPEG replacement |
| JPEG 2000 | .jp2, .j2k | Wavelet compression |
| SVG | .svg | Vector (XML metadata) |

See [supported formats](../formats.md) and [writing metadata](../writing.md)
for field support and resource limits.

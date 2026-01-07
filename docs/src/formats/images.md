# Image Formats

## JPEG

The most common image format. Full EXIF and XMP support.

| Feature | Support |
|---------|---------|
| Read | ✓ |
| Write | ✓ |
| EXIF | Full |
| XMP | Full |
| IPTC | Full |
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
| XMP | Full |
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
| Write | ✓ (existing EXIF) |
| EXIF | Via meta box |
| XMP | Via meta box |

**Extensions:** `.heic`, `.heif`, `.avif`

## GIF

Graphics Interchange Format.

| Feature | Support |
|---------|---------|
| Read | ✓ |
| Write | ✗ |
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

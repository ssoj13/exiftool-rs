# Video Formats

Video containers store metadata in various structures depending on the format.

## MP4/MOV

ISOBMFF-based containers used by most modern cameras and phones.

| Feature | Support |
|---------|---------|
| moov/mvhd | ✓ |
| udta atoms | ✓ |
| XMP | ✓ |
| GPS | ✓ |

**Extensions:** `.mp4`, `.m4v`, `.mov`, `.3gp`

**Common metadata:**
- Duration, creation date
- Video/audio codec info
- GPS track (action cameras)
- Camera make/model

## AVI

Microsoft Audio Video Interleave.

| Feature | Support |
|---------|---------|
| INFO chunk | ✓ |
| Stream info | ✓ |

**Extensions:** `.avi`

## MKV/WebM

Matroska container format.

| Feature | Support |
|---------|---------|
| EBML header | ✓ |
| Segment info | ✓ |
| Tags | ✓ |

**Extensions:** `.mkv`, `.webm`

## FLV

Adobe Flash Video.

| Feature | Support |
|---------|---------|
| onMetaData | ✓ |
| Script tags | ✓ |

**Extensions:** `.flv`

## MXF

Material Exchange Format (broadcast).

| Feature | Support |
|---------|---------|
| Header metadata | ✓ |
| Descriptors | ✓ |

**Extensions:** `.mxf`

## MPEG-TS

Transport stream for broadcast.

| Feature | Support |
|---------|---------|
| PAT/PMT | ✓ |
| Stream info | ✓ |

**Extensions:** `.ts`, `.m2ts`, `.mts`

## Common Video Metadata

```rust
// Duration
metadata.exif.get_str("Duration")  // "1:30:45"

// Dimensions
metadata.exif.get_u32("ImageWidth")   // 1920
metadata.exif.get_u32("ImageHeight")  // 1080

// Frame rate
metadata.exif.get_f64("VideoFrameRate")  // 29.97

// Codec
metadata.exif.get_str("CompressorID")  // "avc1"

// Creation date
metadata.exif.get_str("CreateDate")

// GPS (action cameras)
metadata.exif.get_f64("GPSLatitude")
metadata.exif.get_f64("GPSLongitude")
```

## Action Camera Support

GoPro, DJI, and similar cameras embed extensive metadata:

```rust
// GoPro GPMF data
metadata.exif.get_str("DeviceName")  // "HERO12 Black"

// DJI drone data  
metadata.exif.get_str("Make")  // "DJI"
metadata.exif.get_f64("AbsoluteAltitude")
metadata.exif.get_f64("FlightYawDegree")
```

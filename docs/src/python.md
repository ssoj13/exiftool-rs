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

## Installation

```bash
pip install exiftool-py
```

Or build from source:

```bash
cd crates/exiftool-py
maturin develop --release
```

## Basic Usage

```python
import exiftool_py as exif

# Open image
img = exif.open("photo.jpg")

# Properties for common fields
print(img.make, img.model)
print(img.iso, img.fnumber, img.exposure_time)
print(img.date_time_original)
print(img.width, img.height)

# Dict-like access
print(img["Artist"])
print(img.get("Copyright", "Unknown"))

# Iterate all tags
for tag in img:
    print(f"{tag}: {img[tag]}")

# Convert to dict
metadata = dict(img)
```

## Writing Metadata

```python
# Modify tags
img["Artist"] = "John Doe"
img["Copyright"] = "2024 John Doe"
img.artist = "John Doe"  # Property setter

# Save changes
if img.is_writable:
    img.save()              # Overwrite original
    img.save("output.jpg")  # Save to new file
```

## Time Operations

### Shift DateTime

Shift all DateTime tags by a fixed offset:

```python
# Add 2 hours
img.shift_time("+2:00")

# Subtract 30 minutes
img.shift_time("-30")

# Add 1 hour 45 minutes
img.shift_time("+1:45")

img.save()
```

## Geotagging

### From GPX Track

Add GPS coordinates from a GPX track file:

```python
# Geotag single photo
coords = img.geotag("track.gpx")
if coords:
    lat, lon = coords
    print(f"Geotagged to {lat}, {lon}")
img.save()

# Work with GPX tracks directly
track = exif.GpxTrack.from_file("track.gpx")
print(f"Track has {len(track)} points")
print(f"Time range: {track.time_range}")

# Find position for timestamp
lat, lon, ele = track.find_position(unix_timestamp)
```

## ICC Color Profiles

```python
# Get current ICC profile
if img.icc:
    print(f"Has ICC profile: {len(img.icc)} bytes")

# Set ICC profile from bytes
img.icc = profile_bytes

# Load from file
img.set_icc_from_file("sRGB.icc")

img.save()
```

## Composite Tags

Add computed tags like ImageSize and Megapixels:

```python
img.add_composite()
print(img["ImageSize"])    # "4000x3000"
print(img["Megapixels"])   # 12.0
```

## GPS Coordinates

```python
if img.gps:
    print(f"Location: {img.gps.latitude}, {img.gps.longitude}")
    print(f"Altitude: {img.gps.altitude}m")
    
    # Decimal degrees
    lat, lon = img.gps.as_decimal()
```

## Batch Processing

### Parallel Scan

```python
# Scan with glob pattern
for img in exif.scan("photos/**/*.jpg", parallel=True):
    print(img.path, img.make, img.model)

# Access errors
results = exif.scan("photos/**/*.jpg")
print(f"Parsed: {results.count}, Errors: {results.error_count}")
for err in results.errors:
    print(f"Failed: {err.path} - {err.error}")

# Scan directory
for img in exif.scan_dir("photos/", extensions=["jpg", "png"]):
    print(img.path)
```

## Multi-Page Files

```python
# Check for multi-page TIFF
if img.is_multi_page:
    print(f"Pages: {img.page_count}")
    for page in img.pages:
        print(f"  Page {page.index}: {page.width}x{page.height}")
        if page.is_thumbnail:
            print("    (thumbnail)")
```

## RAW Files

```python
# Detect camera RAW
if img.is_camera_raw:
    print(f"RAW file from {img.make}")
    
    # Extract preview
    if img.preview:
        with open("preview.jpg", "wb") as f:
            f.write(img.preview)

# Extract thumbnail
if img.thumbnail:
    with open("thumb.jpg", "wb") as f:
        f.write(img.thumbnail)
```

## Context Manager

```python
with exif.open("photo.jpg") as img:
    img["Artist"] = "John Doe"
    img.save()
```

## Error Handling

```python
from exiftool_py import ExifError, FormatError, WriteError, TagError

try:
    img = exif.open("photo.jpg")
except FormatError as e:
    print(f"Cannot parse: {e}")

try:
    img["Artist"] = "John"
    img.save()
except WriteError as e:
    print(f"Cannot write: {e}")
```

## API Reference

### Image Class

| Property | Type | Description |
|----------|------|-------------|
| `format` | str | File format (JPEG, PNG, etc.) |
| `path` | str | File path (if opened from file) |
| `make` | str | Camera make |
| `model` | str | Camera model |
| `software` | str | Software used |
| `artist` | str | Artist/author |
| `copyright` | str | Copyright |
| `description` | str | Image description |
| `iso` | int | ISO sensitivity |
| `exposure_time` | Rational | Exposure time |
| `fnumber` | Rational | F-number |
| `focal_length` | Rational | Focal length |
| `focal_length_35mm` | int | 35mm equivalent |
| `date_time_original` | str | Date/time taken |
| `orientation` | int | Orientation (1-8) |
| `width` | int | Image width |
| `height` | int | Image height |
| `gps` | GPS | GPS coordinates |
| `xmp` | str | Raw XMP data |
| `thumbnail` | bytes | Thumbnail image |
| `preview` | bytes | Preview image |
| `icc` | bytes | ICC color profile |
| `page_count` | int | Number of pages |
| `is_multi_page` | bool | Has multiple pages |
| `is_camera_raw` | bool | Is RAW file |
| `is_writable` | bool | Supports writing |

### Image Methods

| Method | Description |
|--------|-------------|
| `open(path)` | Open image file |
| `from_bytes(data)` | Create from bytes |
| `save(path=None)` | Save changes |
| `get(key, default)` | Get tag with default |
| `get_interpreted(key)` | Get human-readable value |
| `get_display(key)` | Get formatted display value |
| `keys()` | Get all tag names |
| `values()` | Get all tag values |
| `items()` | Get (key, value) pairs |
| `clear()` | Remove all tags |
| `to_dict()` | Convert to dictionary |
| `shift_time(offset)` | Shift DateTime tags |
| `geotag(gpx_path)` | Add GPS from GPX |
| `set_icc_from_file(path)` | Load ICC profile |
| `add_composite()` | Add computed tags |

### GpxTrack Class

| Method/Property | Description |
|-----------------|-------------|
| `from_file(path)` | Load GPX file |
| `parse(xml)` | Parse GPX XML |
| `points` | List of TrackPoint |
| `time_range` | (start, end) timestamps |
| `find_position(ts)` | Find coords at timestamp |

# Python Usage

## Opening Files

```python
import exiftool_rs as exif

# From file path
img = exif.open("photo.jpg")

# From bytes
with open("photo.jpg", "rb") as f:
    data = f.read()
img = exif.Image.from_bytes(data)
```

## Reading Tags

```python
img = exif.open("photo.jpg")

# Common properties
print(img.format)           # "JPEG"
print(img.make)             # "Canon"
print(img.model)            # "EOS R5"
print(img.width, img.height)  # 8192 5464

# Date/time
print(img.date_time_original)  # "2024:01:15 14:30:00"

# Exposure settings
print(img.iso)              # 400
print(img.exposure_time)    # Rational(1, 125)
print(img.fnumber)          # Rational(28, 10)
print(img.focal_length)     # Rational(50, 1)

# GPS
if img.gps:
    print(img.gps.latitude)   # 37.7749
    print(img.gps.longitude)  # -122.4194
    print(img.gps.altitude)   # 10.5
```

## Dict-Like Access

```python
# Get any tag
print(img["Make"])          # "Canon"
print(img["ISO"])           # 400

# With default
print(img.get("Rating", 0))  # 0

# Check existence
if "GPSLatitude" in img:
    print("Has GPS")

# Iterate
for tag in img:
    print(f"{tag}: {img[tag]}")

# All keys/values
print(img.keys())
print(img.values())
print(img.items())

# Count
print(len(img))  # number of tags
```

## Human-Readable Values

```python
# Raw value
print(img["Orientation"])  # 6

# Interpreted
print(img.get_interpreted("Orientation"))  # "Rotate 90 CW"

# With units
print(img.get_display("FocalLength"))   # "50 mm"
print(img.get_display("ExposureTime"))  # "1/125 sec"
print(img.get_display("FNumber"))       # "f/2.8"
```

## Thumbnails and Previews

```python
# Small thumbnail
if img.thumbnail:
    with open("thumb.jpg", "wb") as f:
        f.write(img.thumbnail)

# Larger preview (RAW files)
if img.preview:
    with open("preview.jpg", "wb") as f:
        f.write(img.preview)
```

## Modifying Tags

```python
img = exif.open("photo.jpg")

# Set via property
img.artist = "John Doe"
img.copyright = "2024 John Doe"

# Set via dict
img["Software"] = "My App 1.0"
img["Rating"] = 5

# Delete
del img["GPS:GPSLatitude"]

# Clear all
img.clear()
```

## Saving Changes

```python
# Overwrite original
img.save()

# Save to new file
img.save("modified.jpg")
```

## Context Manager

```python
with exif.open("photo.jpg") as img:
    print(img.make)
    img.artist = "John Doe"
    img.save()
```

## Batch Processing

```python
from pathlib import Path

for path in Path("photos").glob("*.jpg"):
    img = exif.open(str(path))
    print(f"{path.name}: {img.make} {img.model}")
```

## Multi-Page Files

```python
img = exif.open("multipage.tiff")

print(img.page_count)      # 3
print(img.is_multi_page)   # True

for page in img.pages:
    print(f"Page {page.index}: {page.width}x{page.height}")
    print(f"  Thumbnail: {page.is_thumbnail}")
```

## Error Handling

```python
from exiftool_rs import FormatError, WriteError

try:
    img = exif.open("unknown.xyz")
except FormatError as e:
    print(f"Cannot parse: {e}")

try:
    img.save()
except WriteError as e:
    print(f"Cannot save: {e}")
```

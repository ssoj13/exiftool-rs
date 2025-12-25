# exiftool-py

Fast image metadata library for Python, built in Rust.

## Installation

```bash
pip install exiftool-py
```

## Usage

```python
import exiftool_py as exif

# Open image
img = exif.open("photo.jpg")

# Read common properties
print(img.make, img.model)
print(img.iso, img.fnumber)

# GPS (if available)
if img.gps:
    print(img.gps.latitude, img.gps.longitude)

# Dict-like access
print(img["Artist"])
for tag in img:
    print(f"{tag}: {img[tag]}")

# Convert to dict
d = dict(img)

# Check format capabilities
if img.is_writable:
    img.artist = "John Doe"
    img.save()
else:
    print(f"Cannot write: {img.format} is read-only")

# Detect camera RAW files
if img.is_camera_raw:
    print(f"RAW file from {img.make}")

# Parallel batch processing
for img in exif.scan("photos/**/*.jpg", parallel=True):
    print(img.path, img.make)
```

## Supported Formats

- JPEG, PNG, TIFF, DNG, WebP
- HEIC, AVIF
- Canon CR2, CR3
- Nikon NEF
- Sony ARW
- Olympus ORF
- Panasonic RW2
- Pentax PEF
- Fujifilm RAF
- OpenEXR, Radiance HDR

## License

MIT OR Apache-2.0

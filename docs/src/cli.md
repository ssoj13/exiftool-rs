# CLI Tool

The `exif` command-line tool provides quick access to image metadata.

## Installation

```bash
cargo install --path crates/exiftool-cli
```

## Basic Usage

```bash
# Show all metadata
exif photo.jpg

# Show specific tags
exif -t Make,Model,ISO photo.jpg

# JSON output
exif -j photo.jpg

# XML output
exif -x photo.jpg

# Process multiple files
exif *.jpg

# Recursive directory scan
exif -r photos/
```

## Output Formats

### Default (Human-Readable)

```
$ exif photo.jpg
File: photo.jpg
Format: JPEG
Make: Canon
Model: EOS R5
DateTimeOriginal: 2024:01:15 14:30:00
ISO: 400
ExposureTime: 1/125
FNumber: 2.8
FocalLength: 50 mm
ImageWidth: 8192
ImageHeight: 5464
```

### JSON

```bash
$ exif -j photo.jpg
{
  "File": "photo.jpg",
  "Format": "JPEG",
  "Make": "Canon",
  "Model": "EOS R5",
  ...
}
```

### XML

```bash
$ exif -x photo.jpg
<?xml version="1.0"?>
<image file="photo.jpg">
  <Format>JPEG</Format>
  <Make>Canon</Make>
  ...
</image>
```

## Options

| Option | Description |
|--------|-------------|
| `-t, --tags <TAGS>` | Show only specified tags (comma-separated) |
| `-j, --json` | Output as JSON |
| `-x, --xml` | Output as XML |
| `-r, --recursive` | Process directories recursively |
| `-q, --quiet` | Suppress errors for unreadable files |
| `-v, --verbose` | Show additional debug info |
| `-h, --help` | Show help |
| `-V, --version` | Show version |

## Examples

```bash
# Camera info only
exif -t Make,Model,LensModel photo.jpg

# GPS coordinates
exif -t GPSLatitude,GPSLongitude photo.jpg

# All files in directory as JSON
exif -j -r photos/ > metadata.json

# Find photos by camera
exif -r photos/ | grep "Model: EOS R5"
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Some files failed to parse |
| 2 | Invalid arguments |

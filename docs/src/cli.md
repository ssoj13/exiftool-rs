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
exif -g Model photo.jpg           # Single tag (value only)
exif -g Make -g Model *.jpg       # Multiple tags
exif -g "Date*" photo.jpg         # Wildcard: all Date* tags

# Output formats
exif -f json *.jpg                # JSON output
exif -f csv photos/*.png          # CSV for spreadsheets
exif -f xml photo.jpg             # XML output
exif -f html photo.jpg            # HTML with styling
exif -X photo.jpg                 # XML shortcut

# Process multiple files
exif *.jpg
exif -r photos/                   # Recursive directory scan
exif -r -e jpg,png photos/        # Filter by extension
```

## Output Formats

### Default (Human-Readable)

```
$ exif photo.jpg
-- photo.jpg --
Format                       JPEG
Make                         Canon
Model                        EOS R5
DateTimeOriginal             2024:01:15 14:30:00
ISO                          400
ExposureTime                 1/125
FNumber                      2.8
FocalLength                  50 mm
ImageWidth                   8192
ImageHeight                  5464
```

### JSON

```bash
$ exif -f json photo.jpg
[
  {
    "SourceFile": "photo.jpg",
    "Format": "JPEG",
    "Make": "Canon",
    "Model": "EOS R5",
    ...
  }
]
```

### CSV

Multi-file CSV output uses unified headers across all files:

```bash
$ exif -f csv *.jpg
SourceFile,DateTimeOriginal,Make,Model,ISO
"photo1.jpg","2024:01:15 14:30:00","Canon","EOS R5","400"
"photo2.jpg","2024:01:16 09:15:00","Nikon","Z8","200"
```

### XML

```bash
$ exif -X photo.jpg
<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
  <rdf:Description rdf:about="photo.jpg">
    <Format>JPEG</Format>
    <Make>Canon</Make>
    ...
  </rdf:Description>
</rdf:RDF>
```

### HTML

```bash
$ exif -f html photo.jpg > report.html
```

Generates styled HTML with CSS for easy viewing in browser.

## Writing Metadata

```bash
# Set single tag
exif -t Artist="John Doe" photo.jpg

# Set multiple tags
exif -t Make=Canon -t Model="EOS R5" photo.jpg

# Write to new file
exif -w output.jpg -t Copyright="2024 Me" photo.jpg

# Modify in-place (overwrites original!)
exif -p -t Software=exif photo.jpg
```

## Time Operations

### Shift DateTime Tags

Shift all DateTime tags by a fixed offset:

```bash
# Add 2 hours
exif --shift "+2:00" -p photo.jpg

# Subtract 30 minutes
exif --shift "-30" -p photo.jpg

# Add 1 hour 30 minutes
exif --shift "+1:30" -p photo.jpg
```

Format: `+/-HH:MM` (hours:minutes) or `+/-MM` (minutes only)

## Geotagging

### From GPX Track

Add GPS coordinates from a GPX track file:

```bash
exif --geotag track.gpx -p photo.jpg
```

The tool matches photo timestamps (DateTimeOriginal) to GPX track points with linear interpolation between points.

## Color Profiles

### Embed ICC Profile

```bash
exif --icc sRGB.icc -p photo.jpg
exif --icc "Adobe RGB.icc" -w output.jpg photo.jpg
```

## File Filtering

### Recursive Scan

```bash
# All supported formats
exif -r photos/

# Filter by extension
exif -r -e jpg,png,cr2 photos/

# Exclude patterns
exif -r -x "*.tmp" -x "*_backup*" photos/
exif -r -x "cache" -x "thumbs" photos/
```

### Date Filters

```bash
# Files modified after date
exif -r --newer 2024-01-01 photos/
exif -r --newer "2024-01-15 10:00:00" photos/

# Files modified before date
exif -r --older 2024-06-01 photos/

# Date range
exif -r --newer 2024-01-01 --older 2024-02-01 photos/
```

### Size Filters

```bash
# Files larger than 1MB
exif -r --minsize 1M photos/

# Files smaller than 10MB
exif -r --maxsize 10M photos/

# Size range
exif -r --minsize 100K --maxsize 5M photos/
```

Size suffixes: K (kilobytes), M (megabytes), G (gigabytes)

## Thumbnail/Preview Extraction

```bash
# Extract thumbnail
exif -T photo.jpg                    # Creates photo_thumb.jpg
exif -T -o thumb.jpg photo.jpg       # Specific output file

# Extract preview (larger, from RAW files)
exif -P photo.cr2                    # Creates photo_preview.jpg
exif -P -o preview.jpg photo.raf
```

## Composite Tags

Add computed tags like ImageSize and Megapixels:

```bash
exif -c photo.jpg
```

Adds:
- `ImageSize`: "4000x3000"
- `Megapixels`: 12.0
- `GPSAltitude`: Combined value with reference
- `DateTimeOriginal`: With SubSecTimeOriginal

## All Options

| Option | Description |
|--------|-------------|
| `-g, --get <PATTERN>` | Get tag(s) matching pattern (* and ? wildcards) |
| `-f, --format <FMT>` | Output format: text, json, csv, xml, html |
| `-X, --xml` | XML output (shortcut for -f xml) |
| `-o, --output <FILE>` | Save metadata/thumbnail to file |
| `-t, --tag <T=V>` | Set tag (repeatable): -t Tag=Value |
| `--shift <OFFSET>` | Shift DateTime tags (+/-HH:MM or +/-MM) |
| `--geotag <GPX>` | Add GPS from GPX track file |
| `--icc <FILE>` | Embed ICC color profile |
| `-w, --write <FILE>` | Output image file (for write mode) |
| `-p, --inplace` | Modify original file in-place |
| `-T, --thumbnail` | Extract embedded thumbnail |
| `-P, --preview` | Extract embedded preview (RAW files) |
| `-r, --recursive` | Recursively scan directories |
| `-e, --ext <EXTS>` | Filter by extensions (comma-separated) |
| `-x, --exclude <PAT>` | Exclude files/dirs matching pattern (glob) |
| `--newer <DATE>` | Only files modified after DATE |
| `--older <DATE>` | Only files modified before DATE |
| `--minsize <SIZE>` | Only files larger than SIZE (K/M/G suffix) |
| `--maxsize <SIZE>` | Only files smaller than SIZE |
| `-c, --composite` | Add composite/calculated tags |
| `--charset <ENC>` | Character encoding (utf8, latin1, ascii) |
| `-a, --all` | Include binary/large tags |
| `-h, --help` | Show help |
| `-v, --version` | Show version |

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Some files failed to parse or other error |

## Examples

```bash
# Camera info from all JPEGs as JSON
exif -f json *.jpg | jq '.[].Model'

# Export metadata to file
exif -f json *.jpg -o metadata.json
exif -f csv *.jpg -o report.csv

# Batch set copyright
for f in *.jpg; do exif -p -t Copyright="2024 Me" "$f"; done

# Process RAW files
exif photo.cr3 photo.nef photo.arw photo.orf

# Find large photos from 2024
exif -r --newer 2024-01-01 --minsize 5M -f csv photos/ -o large_2024.csv

# Geotag vacation photos
exif --geotag vacation.gpx -p photos/*.jpg
```

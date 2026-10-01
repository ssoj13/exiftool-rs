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

## Import Metadata

### From JSON

Import tags from a JSON file:

```bash
exif --json=meta.json -p photo.jpg
exif --json=tags.json -p *.jpg
```

JSON format (object or array of objects):
```json
{
  "Artist": "John Doe",
  "Copyright": "2024 John Doe"
}
```

### From CSV

Batch import from CSV (first column = SourceFile):

```bash
exif --csv=metadata.csv -p *.jpg
```

CSV format:
```csv
SourceFile,Artist,Copyright,Software
photo1.jpg,John Doe,2024,exif
photo2.jpg,Jane Doe,2024,exif
```

### Copy from Another File

Copy metadata from source file:

```bash
# Copy all metadata
exif --tagsFromFile source.jpg -p target.jpg

# Copy specific tags only
exif --tagsFromFile source.jpg -t Make -t Model -t Artist -p target.jpg
```

## Batch Rename

Rename files using metadata templates:

```bash
# Basic rename with camera info
exif --rename "$Make_$Model" -p *.jpg
# Result: Canon_EOS R5.jpg

# With date/time (strftime format)
exif --rename "%Y%m%d_%H%M%S" -p *.jpg
# Result: 20240115_143000.jpg

# Combined template
exif --rename "$Make_$Model_%Y%m%d" -p *.jpg
# Result: Canon_EOS R5_20240115.jpg

# Directory organization
exif --rename "%Y/%m/%d/$filename" -p *.jpg
# Result: 2024/01/15/photo.jpg (creates directories)
```

### Template Variables

| Variable | Description | Example |
|----------|-------------|--------|
| `$TagName` | Any EXIF tag value | `$Make` -> "Canon" |
| `$filename` | Original filename (no ext) | `$filename` -> "IMG_1234" |
| `%Y` | Year (4 digit) | `%Y` -> "2024" |
| `%m` | Month (2 digit) | `%m` -> "01" |
| `%d` | Day (2 digit) | `%d` -> "15" |
| `%H` | Hour (2 digit, 24h) | `%H` -> "14" |
| `%M` | Minute (2 digit) | `%M` -> "30" |
| `%S` | Second (2 digit) | `%S` -> "00" |
| `%e` | Original extension | `%e` -> "jpg" |

Date/time values come from DateTimeOriginal or CreateDate tag.

## Strip Metadata

Remove all metadata from files for privacy:

```bash
# Strip single file
exif --delete -p photo.jpg

# Strip to new file
exif --delete -w clean.jpg photo.jpg

# Strip entire directory
exif --delete -r -p photos/
```

Removes: EXIF, XMP, IPTC, ICC profiles, thumbnails.

## Validate Metadata

Check metadata for common issues:

```bash
# Validate single file
exif --validate photo.jpg

# Validate directory
exif --validate -r photos/
```

Checks:
- GPS coordinates in valid range (-90/90 lat, -180/180 lon)
- Orientation value (1-8)
- ISO reasonable range
- DateTime format validity
- Image dimensions > 0
- ExposureTime > 0
- FNumber > 0

Returns exit code 1 if issues found.

## Conditional Processing

Process only files matching a condition:

```bash
# Only Canon cameras
exif -if "Make eq Canon" -r photos/

# High ISO photos
exif -if "ISO gt 800" *.jpg

# Model contains "R5"
exif -if "Model contains R5" *.jpg

# Files with GPS data
exif -if "GPSLatitude" -r photos/

# Combined with other options
exif -if "Make eq Canon" -f json -r photos/
```

### Condition Operators

| Operator | Description | Example |
|----------|-------------|--------|
| `eq` | Equals (case-insensitive) | `Make eq Canon` |
| `ne` | Not equals | `Make ne Canon` |
| `gt` | Greater than | `ISO gt 800` |
| `lt` | Less than | `ISO lt 200` |
| `ge` | Greater or equal | `ISO ge 400` |
| `le` | Less or equal | `ISO le 1600` |
| `contains` | Contains substring | `Model contains R5` |
| `startswith` | Starts with | `Make startswith Can` |
| `endswith` | Ends with | `Software endswith GIMP` |
| (tag only) | Tag exists | `GPSLatitude` |

Numeric comparisons work with ratios (1/200, f/2.8) and units (100 mm).

## HTML Dump

Visualize file structure with hex preview:

```bash
# Single file to stdout
exif -htmlDump photo.jpg

# Save to file
exif -htmlDump photo.jpg -o dump.html

# Multiple files
exif -htmlDump -r photos/ -o structure.html
```

Output includes:
- File format detection
- Structure markers (JPEG markers, PNG chunks)
- Hex dump of first 256 bytes
- Metadata summary table

## Find Duplicates

Find duplicate files by various criteria:

```bash
# Exact content duplicates (hash)
exif -duplicates hash -r photos/
exif -duplicates -r photos/           # hash is default

# Same capture DateTime
exif -duplicates datetime -r photos/

# Same camera + datetime + dimensions
exif -duplicates metadata -r photos/
```

### Duplicate Methods

| Method | Description |
|--------|-------------|
| `hash` | Exact content match (FNV-1a hash + size) |
| `content` | Same as hash |
| `datetime` | Same DateTimeOriginal/CreateDate |
| `metadata` | Same Make + Model + DateTime + Dimensions |

Output shows duplicate groups with file sizes and total wasted space.

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
| `--json=<FILE>` | Import tags from JSON file |
| `--csv=<FILE>` | Import tags from CSV file |
| `--tagsFromFile <SRC>` | Copy tags from source file |
| `--rename <TEMPLATE>` | Rename files using template |
| `--charset <ENC>` | Character encoding (utf8, latin1, ascii) |
| `--delete` | Remove all metadata (EXIF, XMP, IPTC, ICC) |
| `--validate` | Check metadata for issues |
| `-if <COND>` | Process only files matching condition |
| `-htmlDump` | Show file structure with hex preview |
| `-duplicates [BY]` | Find duplicates (hash/datetime/metadata) |
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

# Organize photos by date
exif --rename "%Y/%m/$Make_%Y%m%d_%H%M%S" -p photos/*.jpg

# Import metadata from spreadsheet
exif --csv=metadata.csv -p *.jpg

# Copy EXIF from original to edited
exif --tagsFromFile original.jpg -p edited.jpg
```

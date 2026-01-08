# TODO3 - Final Features

## High Priority (Simple & Useful)

- [x] `--delete` / `--strip` - Remove all metadata from files via CLI
- [x] `--validate` - Check metadata for issues via CLI

## Medium Priority (Expand Capabilities)

- [ ] Video metadata - MP4/MOV/AVI (QuickTime atoms)
- [ ] Audio metadata - MP3 ID3v1/ID3v2, FLAC Vorbis comments
- [ ] `-if "CONDITION"` - Conditional processing (`exif -if "$ISO > 800" ...`)

## Low Priority (Nice to Have)

- [ ] `-htmlDump` - Visualize binary file structure
- [ ] `-duplicates` - Find duplicates by hash/metadata

---

## Progress

### --delete / --strip
- CLI flag to remove EXIF, XMP, IPTC, ICC from files
- Works with -p (in-place) or -w (new file)
- Example: `exif --delete -p photo.jpg`

### --validate
- CLI flag to check metadata validity
- Returns exit code 0 if valid, 1 if issues found
- Example: `exif --validate photo.jpg`

### Video metadata
- MP4/MOV: Parse moov/meta atoms
- AVI: Parse RIFF INFO chunks
- Tags: Duration, CreateDate, GPSCoordinates, Make, Model

### Audio metadata
- MP3: ID3v1 (last 128 bytes), ID3v2 (header)
- FLAC: Vorbis comments in metadata block
- Tags: Artist, Album, Title, Year, Genre

### -if CONDITION
- Simple expression parser
- Operators: ==, !=, <, >, <=, >=, =~
- Logic: and, or, not
- Example: `exif -if "$ISO > 800 and $Make eq 'Canon'" ...`

### -htmlDump
- Visual representation of file structure
- Shows offsets, sizes, segment types
- Output as standalone HTML with CSS

### -duplicates
- Hash-based (MD5/SHA256 of pixel data or full file)
- Metadata-based (same DateTimeOriginal + dimensions)
- Output: groups of duplicate files

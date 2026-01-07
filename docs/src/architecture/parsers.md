# Parser Design

## FormatParser Trait

All parsers implement a common trait:

```rust
pub trait FormatParser {
    /// Check if this parser can handle the data (from header bytes).
    fn can_parse(&self, header: &[u8]) -> bool;
    
    /// Human-readable format name.
    fn format_name(&self) -> &'static str;
    
    /// Supported file extensions.
    fn extensions(&self) -> &'static [&'static str];
    
    /// Parse metadata from reader.
    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata>;
}
```

## Format Detection

The registry tests each parser's `can_parse()` against file headers:

```rust
impl FormatRegistry {
    pub fn detect(&self, header: &[u8]) -> Option<&dyn FormatParser> {
        self.parsers.iter()
            .find(|p| p.can_parse(header))
            .map(|p| p.as_ref())
    }
}
```

Detection is based on magic bytes, not file extensions:

| Format | Magic Bytes |
|--------|-------------|
| JPEG | `FF D8 FF` |
| PNG | `89 50 4E 47 0D 0A 1A 0A` |
| TIFF LE | `49 49 2A 00` |
| TIFF BE | `4D 4D 00 2A` |
| HEIC | `....ftyp` + brand check |
| MP4 | `....ftyp` + brand check |
| GIF | `GIF87a` or `GIF89a` |

## Parser Implementation Pattern

Typical parser structure:

```rust
pub struct JpegParser;

impl FormatParser for JpegParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        header.len() >= 3 && header[0..2] == [0xFF, 0xD8]
    }
    
    fn format_name(&self) -> &'static str { "JPEG" }
    
    fn extensions(&self) -> &'static [&'static str] {
        &["jpg", "jpeg"]
    }
    
    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("JPEG");
        
        // Parse APP1 (EXIF)
        if let Some(exif_data) = self.find_app1(reader)? {
            self.parse_exif(&exif_data, &mut metadata)?;
        }
        
        // Parse APP13 (IPTC)
        // Parse XMP
        // Extract thumbnail
        
        Ok(metadata)
    }
}
```

## TIFF-Based Formats

Many formats are TIFF variants (CR2, NEF, DNG, etc.):

```rust
pub struct Cr2Parser;

impl FormatParser for Cr2Parser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // TIFF + CR2 magic at offset 8
        TiffParser::is_tiff(header) && 
        header.len() >= 10 && 
        &header[8..10] == b"CR"
    }
    
    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        // Use TiffParser for structure, add CR2-specific handling
        let mut metadata = TiffParser::parse_internal(reader)?;
        metadata.format = "CR2";
        
        // Parse Canon MakerNotes
        self.parse_makernotes(&mut metadata)?;
        
        Ok(metadata)
    }
}
```

## Streaming Design

Parsers accept `&mut dyn ReadSeek`, allowing:

- Files via `BufReader<File>`
- Memory via `Cursor<&[u8]>`
- Network streams (with buffering)
- Memory-mapped files

```rust
// From file
let file = File::open("photo.jpg")?;
let mut reader = BufReader::new(file);
let metadata = parser.parse(&mut reader)?;

// From bytes
let mut cursor = Cursor::new(&data);
let metadata = parser.parse(&mut cursor)?;
```

## Error Handling

Parsers return `Result<Metadata, Error>`:

```rust
pub enum Error {
    Io(std::io::Error),
    UnsupportedFormat,
    InvalidStructure(String),
    // ...
}
```

Partial parsing is preferred - extract what's possible, skip corrupt sections:

```rust
// Don't fail on bad MakerNotes
if let Ok(maker) = self.parse_makernotes(data) {
    metadata.exif.merge(maker);
}
// Continue even if MakerNotes failed
```

# Writing Metadata

## Supported Formats

Writing is supported for:

| Format | Extension | Notes |
|--------|-----------|-------|
| JPEG | .jpg, .jpeg | Full EXIF + XMP |
| PNG | .png | tEXt/iTXt chunks |
| TIFF | .tif, .tiff | Full EXIF |
| DNG | .dng | Full EXIF |
| WebP | .webp | EXIF + XMP chunks |
| HEIC/HEIF | .heic, .heif | EXIF in meta box |
| EXR | .exr | Header attributes |
| HDR | .hdr | Header comments |

RAW formats (CR2, NEF, ARW, etc.) are read-only.

## Basic Writing

```rust
use exiftool_formats::{JpegWriter, build_exif_bytes};
use std::fs::File;
use std::io::{BufReader, BufWriter};

// Read original
let mut reader = BufReader::new(File::open("input.jpg")?);
let registry = FormatRegistry::new();
let mut metadata = registry.parse(&mut reader)?;

// Modify tags
metadata.exif.set("Artist", AttrValue::Str("John Doe".into()));
metadata.exif.set("Copyright", AttrValue::Str("2024 John Doe".into()));

// Build EXIF bytes
let exif_bytes = build_exif_bytes(&metadata)?;

// Write to new file
reader.seek(SeekFrom::Start(0))?;
let mut output = Vec::new();
JpegWriter::write(&mut reader, &mut output, Some(&exif_bytes), None)?;

std::fs::write("output.jpg", output)?;
```

## Modifying Tags

```rust
use exiftool_attrs::AttrValue;

// Set string
metadata.exif.set("Artist", AttrValue::Str("Name".into()));

// Set integer
metadata.exif.set("Orientation", AttrValue::UInt(1));

// Set rational (numerator, denominator)
metadata.exif.set("ExposureTime", AttrValue::URational(1, 125));

// Set list
metadata.exif.set("Keywords", AttrValue::List(vec![
    "landscape".into(),
    "mountains".into(),
]));

// Remove tag
metadata.exif.remove("GPSLatitude");

// Clear all
metadata.exif.clear();
```

## Format-Specific Writers

Each writable format has its own writer:

```rust
// JPEG
JpegWriter::write(&mut reader, &mut output, Some(&exif), Some(&xmp))?;

// PNG  
PngWriter::write(&mut reader, &mut output, &metadata)?;

// TIFF
TiffWriter::write(&mut reader, &mut output, &metadata)?;

// WebP
WebpWriter::write(&mut reader, &mut output, &metadata)?;

// HEIC
HeicWriter::write(&mut reader, &mut output, &metadata)?;
```

## XMP Writing

```rust
// Set raw XMP
metadata.xmp = Some(r#"<?xpacket begin="..." ?>
<x:xmpmeta xmlns:x="adobe:ns:meta/">
  ...
</x:xmpmeta>
<?xpacket end="w"?>"#.to_string());
```

## Preserving Original Data

Writers preserve image data and non-EXIF chunks. Only metadata sections 
are modified. The image pixels remain untouched.

```rust
// Original file structure is preserved
// Only EXIF/XMP segments are replaced
JpegWriter::write(&mut reader, &mut output, Some(&new_exif), None)?;
```

## Error Handling

```rust
match TiffWriter::write(&mut reader, &mut output, &metadata) {
    Ok(()) => println!("Success"),
    Err(Error::UnsupportedFormat) => println!("Format doesn't support writing"),
    Err(Error::InvalidStructure(msg)) => println!("Corrupt file: {}", msg),
    Err(e) => println!("Write error: {}", e),
}
```

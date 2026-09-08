# Writing Metadata

## Supported Formats

Writing is supported for:

| Format | Extension | Notes |
|--------|-----------|-------|
| JPEG | .jpg, .jpeg | Full EXIF + XMP + IPTC |
| PNG | .png | tEXt/iTXt chunks |
| TIFF | .tif, .tiff | Full EXIF (classic + BigTIFF preserve rewrite) |
| DNG | .dng | Full EXIF |
| WebP | .webp | EXIF + XMP chunks |
| HEIC/HEIF | .heic, .heif | EXIF in meta box (creates EXIF item if missing) |
| EXR | .exr | Header attributes (`exr-core`) |
| HDR | .hdr | Header comments |
| GIF | .gif | Comment / metadata |
| PNM | .pbm/.pgm/.ppm/.pam | Header comments |
| JXL | .jxl | EXIF box |
| RAF | .raf | Preview JPEG EXIF (WriteRAF); RAF directory + CFA copied from nextPtr at 0x5C |
| CR3 | .cr3 | ISOBMFF: CMT1/2/4 TIFF rewrite, XMP UUID, CTBO + stco/co64 (ExifTool WriteQuickTime CR3 map) |
| NEF / NRW | .nef, .nrw | TIFF rewrite: IFD0/Exif/GPS overlay; SubIFD + strips/tiles + MakerNotes blob copied |
| CR2 / ARW / ORF / RW2 / … | TIFF-RAW | Same `tiff_rewrite` path (`TiffWriter::write`); CR2 16-byte header; A100 ARW uses FinishARW (MRW + CFA trailer) |
| MP4 / MOV | .mp4, .mov, … | XMP UUID box |
| WAV / FLAC / MP3 | | Existing tag writers |

Do not rebuild camera RAW with a short IFD0-only writer. MakerNotes field write overlays existing IFD tags (FujiFilm rebuild; other IFD vendors in-place, including headerless Kodak and Motorola). Nikon ShotInfo (`0x0091`) Full-crypt and `NIKON_OFFSETS` fields, ColorBalance levels, and LensData `LensIDNumber` overlay after decrypt; `KDK*` binary magics stay blob-copied.

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
JpegWriter::write(&mut reader, &mut output, Some(&exif_bytes), None, None)?;

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
// JPEG (exif, xmp, iptc)
JpegWriter::write(&mut reader, &mut output, Some(&exif), Some(&xmp), Some(&iptc))?;

// PNG  
PngWriter::write(&mut reader, &mut output, &metadata)?;

// TIFF
TiffWriter::write(&mut reader, &mut output, &metadata)?;

// WebP
WebpWriter::write(&mut reader, &mut output, &metadata)?;

// HEIC
HeicWriter::write(&mut reader, &mut output, &metadata)?;

// NEF/NRW (preserve SubIFD/raw)
NefWriter::write(&mut reader, &mut output, &metadata)?;

// CR2 / ORF (same preserve rewrite; CR2 keeps 16-byte header)
Cr2Writer::write(&mut reader, &mut output, &metadata)?;

// RAF (preview JPEG EXIF; CFA copied)
RafWriter::write(&mut reader, &mut output, &metadata)?;
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
JpegWriter::write(&mut reader, &mut output, Some(&new_exif), None, None)?;
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

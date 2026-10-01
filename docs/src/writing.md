# Writing metadata

Writing uses a format-specific writer and the original file. `is_writable()`
indicates a supported format, not that every attribute in the map can be stored.
Read the output back to verify the fields you changed.

## Edit a JPEG and save a copy

With the [Rust dependencies](getting-started/installation.md) configured,
run this program from the repository root:

```rust
use exiftool_attrs::AttrValue;
use exiftool_formats::{FormatRegistry, JpegWriter};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("crates/exiftool-formats/tests/testdata/Writer.jpg");
    let registry = FormatRegistry::new();
    let mut metadata = registry.parse_file(path)?;
    metadata.exif.set("Artist", AttrValue::Str("Alex".into()));

    // Reopen the source so the writer starts at byte zero.
    let mut input = BufReader::new(File::open(path)?);
    let mut output = Vec::new();
    JpegWriter::write_metadata(&mut input, &mut output, &metadata)?;
    std::fs::write("output.jpg", &output)?;

    let saved = registry.parse_file(Path::new("output.jpg"))?;
    assert_eq!(saved.exif.get_str("Artist"), Some("Alex"));
    Ok(())
}
```

`write_metadata` uses existing EXIF TIFF data for the preservation-oriented
rewrite when available, builds EXIF for a file without it, supplies the current
XMP, and builds IPTC APP13 from the attributes. JPEG image data is copied without
recompression. This does not promise byte-for-byte preservation of every
metadata block in every JPEG variant.

## Replacement payloads and removal

The lower-level `JpegWriter::write` takes five arguments:
`input`, `output`, optional EXIF TIFF bytes, optional XMP text, and optional
IPTC APP13 bytes. **`None` removes an existing recognized block; it does not
mean preserve it.** Supply the payload when you want to keep it.

`build_exif_bytes(&metadata)` builds a fresh TIFF EXIF payload for a supported
set of fields. It is not a general serializer for every attribute or a
replacement for preservation-oriented RAW rewriting.

Clearing `metadata.exif` is different from clearing XMP and ICC fields, and
writers may retain existing container structures. Use the CLI's explicit
`--delete` operation for its supported removal behavior. Neither clearing a map
nor a generic rewrite proves that all identifying information has disappeared.

## Writable containers

| Container family | Writer path | Scope |
|------------------|-------------|-------|
| JPEG | `JpegWriter` | EXIF, XMP, IPTC payloads |
| PNG | `PngWriter` | EXIF and supported text/XML chunks |
| TIFF / DNG | `TiffWriter` | Preserve and overlay supported IFD fields |
| WebP | `WebpWriter` | EXIF / XMP chunks |
| HEIC / HEIF / AVIF | `HeicWriter` | Metadata items in the ISOBMFF container |
| EXR / HDR | `ExrWriter` / `HdrWriter` | Header attributes / comments |
| GIF / PNM / JXL | Corresponding writers | Supported comments or metadata blocks |
| TIFF-family RAW | Vendor writers / TIFF rewrite | IFD overlay with RAW structure preservation |
| RAF | `RafWriter` | Preview JPEG EXIF with CFA data copied |
| CR3 | `Cr3Writer` | CMT TIFF blocks, XMP UUID and affected offset tables |
| MP4 / MOV family | `Mp4Writer` | XMP UUID metadata |
| WAV / FLAC / MP3 | `WavWriter` / `FlacWriter` / `Id3Writer` | Supported container tags |

Audio and video reading support is broader than these writing paths.
Consult [formats](formats.md) and each writer's Rust API docs for exact signatures.

## Camera RAW and MakerNotes

Do not replace a RAW container with a fresh minimal TIFF/EXIF header. Use its
writer so SubIFDs, strips/tiles, camera-specific trailers, and data offsets follow
the appropriate preservation path.

MakerNotes updates depend on known vendor layouts. Supported paths include
IFD fields, selected indexed blobs, masks/bitfields, Nikon encrypted fields,
FujiFilm rebuilding, and same-size GoPro GPMF leaves. Unknown structures may
remain opaque blobs. A vendor name in a tag table is not a guarantee that all
fields or camera variants can be edited.

## Errors and resource use

Writers can report malformed structures, I/O errors, oversized input, and
metadata-standard errors. Several writers buffer the full input; the shared
reader's limit is 100 MiB. See [performance](performance.md).

For command-line editing use the [CLI guide](cli.md); for Python use
[Python usage](python/usage.md).

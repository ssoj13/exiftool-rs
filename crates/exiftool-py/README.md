# exiftool-py

Python bindings for the Rust metadata library. Distribution name: `exiftool-py`.
Import name: `exiftool_py`.

## Install from this checkout

Use Rust 1.96+, Python, an activated virtual environment, and access to the
[Git dependencies](../../docs/src/dependency-access.md). From the repository root:

```bash
python -m pip install maturin
maturin develop --release --manifest-path crates/exiftool-py/Cargo.toml
```

See [Python installation](../../docs/src/python/installation.md) for environment
activation and wheel builds. The repository's release workflow publishes CLI
archives rather than Python wheels.

## Read and edit

Run from the repository root:

```python
import exiftool_py as exif

img = exif.open("crates/exiftool-formats/tests/testdata/Writer.jpg")
print(img.format, img.make, img.model)
print(img.get("Artist", "Unknown"))
if img.is_writable:
    img.artist = "Alex"
    img.save("output.jpg")
```

`save()` without an output path overwrites the original file. Writing support
varies by container and tag. See [usage](../../docs/src/python/usage.md),
[API reference](../../docs/src/python/api.md), and
[formats](../../docs/src/formats.md).

## License

`Artistic-1.0-Perl OR GPL-1.0-or-later`, matching ExifTool and Perl.
See [LICENSE](../../LICENSE) and [NOTICE.md](../../NOTICE.md).

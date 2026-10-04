# Python bindings

The distribution **`exiftool-py`** provides the import **`exiftool_py`**.
It wraps the Rust parsers through PyO3 and works with metadata directly,
without an ExifTool subprocess.

## Guides

- [Installation](python/installation.md): virtual environments and source/wheel builds.
- [Usage](python/usage.md): reading, editing, previews, and batch scans.
- [API reference](python/api.md): supported functions, properties, and errors.

## Read and edit

After installation, run this from the repository root:

```python
import exiftool_py as exif

img = exif.open("crates/exiftool-formats/tests/testdata/Writer.jpg")
print(img.format, img.make, img.model)
print(img.get("Artist", "Unknown"))

if img.is_writable:
    img.artist = "Alex"
    img.save("output.jpg")
    saved = exif.open("output.jpg")
    assert saved.artist == "Alex"
```

`save()` with no path overwrites the source file. Use an output path for the
first edit. Writable format support does not mean every tag is serializable;
see [writing metadata](writing.md).

Parallel scans return an iterator over already collected results and expose
failed files separately. Async helpers require Python 3.9+ and run synchronous
operations in a worker thread; they are not nonblocking native parsers.
See [usage](python/usage.md) and [performance](performance.md).

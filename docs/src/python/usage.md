# Python usage

Install the extension using [Python installation](installation.md).
The examples use the included JPEG fixture and run from the repository root.

## Read a file or bytes

```python
from pathlib import Path
import exiftool_py as exif

path = "crates/exiftool-formats/tests/testdata/Writer.jpg"
img = exif.open(path)
print(img.format, img.make, img.model)
print(img.get("Artist", "Unknown"))
print(img.get_display("ExposureTime"))

for name, value in img.items():
    print(name, value)

from_bytes = exif.Image.from_bytes(Path(path).read_bytes())
print(from_bytes.format)
```

A file-backed image carries its source path. An image created from bytes has no
source path, and saving requires source-file information in the current writer
implementation. Use `open` for editing; see [API reference](api.md).

## Save an edit and verify it

```python
img = exif.open("crates/exiftool-formats/tests/testdata/Writer.jpg")
if img.is_writable:
    img.artist = "Alex"
    img["Copyright"] = "2026 Alex"
    img.save("output.jpg")
    saved = exif.open("output.jpg")
    assert saved.artist == "Alex"
```

`save()` without a path overwrites the original file. `clear()` clears the
attribute map; `strip_metadata()` also clears payload fields in memory.
Actual removal depends on the writer. Check the result rather than assuming
that an empty map proves a file contains no identifying metadata.

## Batch scans and errors

```python
result = exif.scan("crates/exiftool-formats/tests/testdata/*.jpg", parallel=True)
for img in result:
    print(img.path, img.format, img.make)
print("Parsed:", result.count, "Failed:", result.error_count)
for error in result.errors:
    print(error.path, error.error)
```

`scan` accepts a glob, parses matching files, and returns a `ScanResult` iterator.
All results are collected before the function returns. `scan_dir` is
non-recursive and accepts an optional list of extensions.

The `ignore_errors` argument currently does not make failed parses raise:
failures are collected in `result.errors` in either case. Check `error_count`
when a complete scan is required.

## Async use

Python 3.9+:

```python
import asyncio
import exiftool_py as exif

async def main():
    img = await exif.open_async("crates/exiftool-formats/tests/testdata/Writer.jpg")
    print(img.format, img.make)

asyncio.run(main())
```

The helpers use `asyncio.to_thread`. They move the synchronous call into a
worker thread; scan results still consume memory as described in
[performance](../performance.md).

## Payloads and other operations

Read optional `thumbnail`, `preview`, `xmp`, and `icc` fields before using them.
Binary previews should be identified before choosing an output file extension.
The `Image` API also exposes time shifts, GPX geotagging, direct GPS setters,
copying tags, ICC replacement, sidecars, composite tags, and validation.
See the [API reference](api.md) for entry points and the
[writing guide](../writing.md) for container limitations.

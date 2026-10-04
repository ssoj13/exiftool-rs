# Python API reference

Import the public package with `import exiftool_py as exif`.
This page describes the shipped wrapper and extension; the typing stub can
contain declarations beyond what is exported by the public package.

## Module functions

| Function | Result | Behavior |
|----------|--------|----------|
| `open(path)` | `Image` | Open a string path and parse metadata |
| `scan(pattern, parallel=True, ignore_errors=True)` | `ScanResult` | Parse files matching a glob; failures are collected |
| `scan_dir(directory, extensions=None, parallel=True)` | `ScanResult` | Scan one directory, non-recursively |
| `await open_async(path)` | `Image` | Run `open` in a Python worker thread |
| `await scan_async(pattern, parallel=True, ignore_errors=True)` | `ScanResult` | Run `scan` in a worker thread |
| `await scan_dir_async(directory, extensions=None, parallel=True)` | `ScanResult` | Run `scan_dir` in a worker thread |

Use strings for file paths in the native functions. Async helpers require
Python 3.9+. `scan` currently collects errors for both values of `ignore_errors`.

## Image

Create an image with `exif.open(path)` or `exif.Image.from_bytes(data)`.
`from_bytes` parses without a filename extension hint and has no source path.
The current `save` implementation needs a source path to read the original
container, even when an output path is provided.

| Property | Purpose |
|----------|---------|
| `format`, `path` | Parsed format and optional source path |
| `make`, `model`, `software`, `artist`, `copyright`, `description` | Common editable string fields |
| `iso`, `exposure_time`, `fnumber`, `focal_length`, `focal_length_35mm` | Capture settings, if present |
| `date_time_original`, `orientation`, `width`, `height` | Capture date and image geometry |
| `gps` | Optional `GPS` value |
| `xmp`, `icc`, `thumbnail`, `preview` | Optional text or binary payloads |
| `page_count`, `is_multi_page`, `pages`, `exif_offset` | TIFF/page information and EXIF location |
| `is_camera_raw`, `is_writable` | Format classification and writer availability |

Properties for absent metadata return `None` where applicable. The value type
of a tag depends on how it was stored and decoded.

| Method | Purpose |
|--------|---------|
| `get(key, default=None)` | Read a tag with a default |
| `get_interpreted(key)`, `get_display(key)` | Selected enum interpretation / formatted values |
| `keys()`, `values()`, `items()`, `to_dict()` | Attribute map views/copy |
| `clear()` | Clear the attribute map |
| `strip_metadata()` | Clear attributes and metadata payloads in memory |
| `save(path=None)` | Write using the original file; no path means overwrite it |
| `copy_tags(source, tags=None)` | Copy all or selected attributes from an `Image` |
| `shift_time(offset)` | Shift supported datetime attributes |
| `geotag(gpx_path)` | Assign coordinates from a GPX track |
| `set_gps(lat, lon, alt=None)` | Set GPS coordinates |
| `set_icc_from_file(path)` | Load ICC bytes |
| `add_composite()` | Add computed attributes |
| `validate()` | Return validation issues |
| `has_sidecar()`, `sidecar_path()`, `load_sidecar()`, `save_sidecar(path=None)` | Work with XMP sidecars |

`img[key]`, assignment, deletion, membership, `len(img)`, and iteration over tag
names are supported. Use `img.items()` for `(name, value)` pairs.
Writable formats and individual writable fields differ; see [writing](../writing.md).

## ScanResult

| Member | Meaning |
|--------|---------|
| Iteration | Successfully parsed `Image` objects |
| `count`, `len(result)` | Number of successfully parsed images |
| `errors` | Objects with `path` and `error` strings |
| `error_count` | Number of failed parses |
| `to_list()` | Copy successfully parsed images to a list |

Scanning is eager. A large result retains metadata payloads for all successful
files. See [performance](../performance.md).

## Supporting values

- `Rational(num, den)` exposes **`num`** and **`den`**. Use `float(value)` or
  `int(value)` for conversion. The current implementation returns zero for a
  zero denominator during these conversions.
- `GPS` exposes latitude, longitude, optional altitude, and coordinate helpers.
- `img.pages` contains page objects with `index`, `width`, `height`,
  `bits_per_sample`, `compression`, `subfile_type`, `is_thumbnail`, and `is_page`.
- `ValidationIssue` is exported by the package for validation results.

`GpxTrack`, `TrackPoint`, and `PageInfo` are not exported at the top level by
`exiftool_py.__init__`. Use `Image.geotag` for the public GPX operation and
`img.pages` for page information.

## Exceptions

`ExifError`, `FormatError`, `WriteError`, and `TagError` are exported by the
package. Filesystem failures may also map to Python built-in exceptions.
Catch the error appropriate to the operation and inspect its message.

See [installation](installation.md) and [usage](usage.md).

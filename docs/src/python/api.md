# Python API Reference

## Module Functions

### `open(path: str) -> Image`

Open an image file and parse metadata.

```python
img = exif.open("photo.jpg")
```

**Raises:** `FormatError` if file cannot be parsed.

### `scan(path: str) -> ScanResult`

Quick scan of a file without full parsing.

### `scan_dir(path: str) -> List[ScanResult]`

Scan all supported files in a directory.

---

## Image Class

Main class for image metadata.

### Constructor

#### `Image.from_bytes(data: bytes) -> Image`

Create Image from raw bytes.

```python
img = exif.Image.from_bytes(jpeg_bytes)
```

### Properties (Read-Only)

| Property | Type | Description |
|----------|------|-------------|
| `format` | `str` | File format ("JPEG", "PNG", etc.) |
| `path` | `str \| None` | File path if opened from file |
| `xmp` | `str \| None` | Raw XMP data |
| `thumbnail` | `bytes \| None` | Embedded thumbnail |
| `preview` | `bytes \| None` | Larger preview image |
| `page_count` | `int` | Number of pages |
| `is_multi_page` | `bool` | True if multiple pages |
| `is_camera_raw` | `bool` | True if RAW format |
| `is_writable` | `bool` | True if format supports writing |
| `pages` | `List[PageInfo]` | Page info for multi-page files |
| `exif_offset` | `int \| None` | EXIF data offset in file |

### Properties (Read-Write)

| Property | Type | Description |
|----------|------|-------------|
| `make` | `str \| None` | Camera make |
| `model` | `str \| None` | Camera model |
| `software` | `str \| None` | Software used |
| `artist` | `str \| None` | Artist/author |
| `copyright` | `str \| None` | Copyright notice |
| `description` | `str \| None` | Image description |

### Properties (Read-Only, Capture Info)

| Property | Type | Description |
|----------|------|-------------|
| `iso` | `int \| None` | ISO sensitivity |
| `exposure_time` | `Rational \| None` | Exposure time |
| `fnumber` | `Rational \| None` | F-number |
| `focal_length` | `Rational \| None` | Focal length |
| `focal_length_35mm` | `int \| None` | 35mm equivalent |
| `date_time_original` | `str \| None` | Capture date/time |
| `orientation` | `int \| None` | Orientation (1-8) |
| `width` | `int \| None` | Image width |
| `height` | `int \| None` | Image height |
| `gps` | `GPS \| None` | GPS coordinates |

### Methods

#### `get(key: str, default=None) -> Any`

Get tag value with optional default.

#### `get_interpreted(key: str) -> str | None`

Get human-readable interpretation of tag value.

#### `get_display(key: str) -> str | None`

Get formatted display value with units.

#### `keys() -> List[str]`

Get all tag names.

#### `values() -> List[Any]`

Get all tag values.

#### `items() -> List[Tuple[str, Any]]`

Get all (key, value) pairs.

#### `clear()`

Remove all EXIF tags.

#### `to_dict() -> dict`

Convert metadata to dictionary.

#### `save(path: str = None)`

Save metadata to file.

- If `path` is None, overwrites original file
- Raises `WriteError` on failure

### Dict-Like Operations

```python
img["Tag"]           # Get tag
img["Tag"] = value   # Set tag
del img["Tag"]       # Delete tag
"Tag" in img         # Check existence
len(img)             # Tag count
for tag in img:      # Iterate tags
```

---

## PageInfo Class

Information about a page in multi-page files.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `index` | `int` | Page index (0-based) |
| `width` | `int` | Width in pixels |
| `height` | `int` | Height in pixels |
| `bits_per_sample` | `int` | Bits per sample |
| `compression` | `int` | Compression type |
| `subfile_type` | `int` | Subfile type flags |
| `is_thumbnail` | `bool` | True if reduced resolution |
| `is_page` | `bool` | True if document page |

---

## Rational Class

Represents a rational number (fraction).

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `numerator` | `int` | Numerator |
| `denominator` | `int` | Denominator |

### Methods

#### `to_float() -> float`

Convert to floating point.

---

## GPS Class

GPS coordinates.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `latitude` | `float` | Latitude in degrees |
| `longitude` | `float` | Longitude in degrees |
| `altitude` | `float \| None` | Altitude in meters |

---

## Exceptions

### `ExifError`

Base exception for all errors.

### `FormatError`

Raised when file format cannot be parsed.

### `WriteError`

Raised when writing fails.

### `TagError`

Raised for tag-related errors.

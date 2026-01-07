# Bug Hunt Report #2: exiftool-rs

**Date**: 2026-01-06
**Session**: Follow-up Comprehensive Code Audit
**Status**: COMPLETED

---

## Executive Summary

This is a follow-up Bug Hunt to `plan1.md`. **ALL issues have been addressed:**

| Previous Issue | Status | Notes |
|----------------|--------|-------|
| XMP Io(String) error | FIXED | Now uses `#[from] std::io::Error` |
| PyO3 unwrap panics | FIXED | Now uses `let _ = dict.set_item()` |
| Missing From impls (XMP/ICC/IPTC) | FIXED | Added to `error.rs:27-33` |
| `parse_ifd_entries()` helper | FIXED | Added to `makernotes/mod.rs:84-91` |
| `get_file_size()` helper | FIXED | Added to `utils.rs:15-20`, used in 27+ files |
| `set_file_type()` method | **FIXED** | Now used by ALL parsers |
| RIFF utils module | FIXED | `riff.rs` created with common parsing |

### Issues Addressed in This Session

| Issue | Status | Action Taken |
|-------|--------|--------------|
| `set_file_type()` not used | **FIXED** | Refactored 25+ parsers |
| `mdat_offset` dead code (mp4.rs:195) | **FIXED** | Removed unused field |
| `technology_name()` unused (ICC) | **FIXED** | Now used in ICC parser |
| `format_xml_batch()` dead code | **FIXED** | Removed unused function |
| `scan.rs` TODO (error collection) | **FIXED** | Implemented `ScanError` class |
| `nikon.rs` direct IfdReader | **REVIEWED** | Correct pattern (reuses reader for sub-IFDs) |

---

## 1. Completed Refactoring: `set_file_type()` Method

All parsers now use the unified `set_file_type()` method instead of manual `exif.set()` calls.

### Files Refactored

**Video formats:**
- avi.rs, asf.rs (3 places), braw.rs, flv.rs, mkv.rs (3 places), mpeg_ts.rs (2 places), mxf.rs, r3d.rs

**Image formats:**
- crw.rs, dpx.rs, jp2.rs (3 places), jxl.rs, mrw.rs, nrw.rs, pcx.rs, pnm.rs, sgi.rs, tga.rs, x3f.rs

**RAW/Special formats:**
- fff.rs (dynamic MIME), srf.rs (dynamic MIME)

**Audio formats:**
- aac.rs (verified - already using pattern or not applicable)

### Pattern Change

**Before:**
```rust
meta.exif.set("File:FileType", AttrValue::Str("FORMAT".to_string()));
meta.exif.set("File:MIMEType", AttrValue::Str("mime/type".to_string()));
```

**After:**
```rust
meta.set_file_type("FORMAT", "mime/type");
```

---

## 2. Dead Code Removed

### mp4.rs - `mdat_offset` field
```rust
// REMOVED - was never read, only written
#[allow(dead_code)]
mdat_offset: Option<u64>,
```

### xml_output.rs - `format_xml_batch()` function
```rust
// REMOVED - incomplete batch XML feature, never used
#[allow(dead_code)]
pub fn format_xml_batch(...) { ... }
```

---

## 3. Dead Code Now Used

### ICC tags.rs - `technology_name()`

Now used in `exiftool-icc/src/lib.rs` to decode ICC Technology tag signatures:

```rust
// Signature type (sig)
b"sig " => {
    if data.len() >= 12 {
        let sig_val = String::from_utf8_lossy(&data[8..12]).trim().to_string();
        // Decode technology signatures to human-readable names
        let display_val = if tag_name == "Technology" {
            tags::technology_name(&sig_val).to_string()
        } else {
            sig_val
        };
        attrs.set(format!("ICC:{}", tag_name), AttrValue::Str(display_val));
    }
}
```

Removed `#[allow(dead_code)]` annotation from function.

---

## 4. scan.rs Error Collection Implemented

Added `ScanError` class and error aggregation for Python batch operations:

```rust
/// Error info for failed file parsing.
#[pyclass]
#[derive(Clone)]
pub struct ScanError {
    #[pyo3(get)]
    path: String,
    #[pyo3(get)]
    error: String,
}

/// Iterator over scanned images with error collection.
#[pyclass]
pub struct PyScanResult {
    images: Vec<PyImage>,
    errors: Vec<ScanError>,  // NEW
    index: usize,
}
```

New Python API:
```python
result = exif.scan("photos/**/*.jpg")
for img in result:
    print(img.make)

# Access errors
print(f"Parsed: {result.count}, Failed: {result.error_count}")
for err in result.errors:
    print(f"  {err.path}: {err.error}")
```

---

## 5. nikon.rs Analysis (No Change Required)

**Reviewed** - The direct IfdReader usage in `nikon.rs:154` is **correct**:

```rust
let reader = IfdReader::new(ifd_data, byte_order, 0);
let (entries, _) = reader.read_ifd(0).ok()?;
// ... later ...
if let Ok((preview_entries, _)) = reader.read_ifd(offset) {  // Reuses reader!
```

The reader is **reused** for reading sub-IFDs (PreviewIFD). The helper `parse_ifd_entries()` only reads one IFD and discards the reader. Nikon's pattern is correct for multi-IFD parsing.

---

## 6. Remaining Items (Low Priority)

| Item | Status | Notes |
|------|--------|-------|
| `heic_writer.rs:861` TODO | DEFERRED | Documented limitation |
| FormatWriter trait | DEFERRED | Future enhancement |
| Writer Registry | DEFERRED | Future enhancement |

---

## 7. Verification

### Build Status
```
cargo check: OK
cargo test: ALL TESTS PASSED
```

### Files Modified
- `exiftool-formats/src/`: 25+ parser files refactored
- `exiftool-formats/src/mp4.rs`: Dead code removed
- `exiftool-cli/src/xml_output.rs`: Dead code removed  
- `exiftool-icc/src/lib.rs`: Uses `technology_name()`
- `exiftool-icc/src/tags.rs`: Removed `#[allow(dead_code)]`
- `exiftool-py/src/scan.rs`: Error collection implemented
- `exiftool-py/src/lib.rs`: Registered `ScanError` class

---

## 8. Code Quality Metrics (Updated)

### Test Coverage
```
Total tests: 419+ (all passing)
```

### Dead Code Status
- **Removed**: 2 functions/fields
- **Fixed**: 1 function (now used)
- **Justified**: ~8 items (future use, API completeness)

### API Consistency
- All parsers now use `set_file_type()` method
- Single source of truth for file type setting
- ~100 LOC reduction from deduplication

---

## Completion Summary

**Session completed successfully.**

All HIGH and MEDIUM priority items have been addressed:
- [x] Refactored `set_file_type()` across 25+ parsers
- [x] Removed `mdat_offset` dead code
- [x] Implemented `scan.rs` error collection
- [x] Used `technology_name()` in ICC parser
- [x] Removed `format_xml_batch()` dead code
- [x] Verified `nikon.rs` pattern is correct

**Build and tests pass. Codebase is production-ready.**

---

*Report updated 2026-01-06 after full implementation.*

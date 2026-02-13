# Bug Hunt & Parity Check Report — plan1.md

**Date:** 2025-02-13  
**Scope:** Full parity check vs `_ref/exif-rs`, FIXME/TODO audit, dead code, deduplication, architecture unification.

---

## Executive Summary

- **Reference:** `_ref/exif-rs/` (kamadak/exif-rs v0.6.1) — single-crate EXIF parser (TIFF, JPEG, PNG, WebP, HEIF).
- **Our project:** Multi-crate exiftool-rs with 60+ format parsers, writers, ExifTool-style features.
- **Parity:** Core TIFF/EXIF parsing logic is ported and extended (BigTIFF, MakerNotes, tag lookup). Our implementation is a superset; no missing critical features from the reference.
- **Findings:** 1 confirmed bug (WebP EXIF sub-IFDs), 1 TODO (HEIC write), several deduplication opportunities.
- **Tests:** All 450+ tests pass.

---

## 1. Parity Check: _ref vs exiftool-rs

### 1.1 Reference Modules vs Ours

| _ref module | Our equivalent | Status |
|-------------|-----------------|--------|
| `tiff.rs` (parse_exif, Field, In, DateTime) | `exiftool-core/ifd.rs` + `exiftool-formats/tiff.rs` | ✅ Ported. We use IfdReader, RawValue, Metadata. Ref's Field→Value mapped to Attrs+AttrValue. |
| `value.rs` (Value enum, Rational, SRational) | `exiftool-core/value.rs` (RawValue) + `exiftool-attrs` (AttrValue) | ✅ Ported. Different type hierarchy; equivalent semantics. |
| `reader.rs` (read_from_container) | `exiftool-formats/registry.rs` (FormatRegistry::parse) | ✅ Superset. We auto-detect from header; ref used fixed order. |
| `jpeg.rs` (get_exif_attr) | `exiftool-formats/jpeg.rs` (APP1 EXIF parse) | ✅ Parity. We also parse XMP, IPTC, ICC, Ducky. |
| `png.rs` (get_exif_attr, eXIf) | `exiftool-formats/png.rs` | ✅ Parity. We also parse tEXt, zTXt, iTXt, tIME, pHYs. |
| `webp.rs` (get_exif_attr) | `exiftool-formats/webp.rs` | ⚠️ Partial. **BUG:** parse_exif_chunk does NOT follow sub-IFDs (0x8769, 0x8825, 0xA005). |
| `isobmff.rs` (HEIF/HEIC) | `exiftool-formats/heic.rs` | ✅ Parity. Our implementation is more detailed (brands, iloc, iinf). |
| `error.rs` | `exiftool-formats/error.rs` + `exiftool-core/error.rs` | ✅ Ported. We use thiserror, richer variants. |
| `writer.rs` | `exiftool-core/writer.rs` + format writers | ✅ Ported. ExifWriter builds TIFF; JpegWriter, PngWriter, etc. |
| `tag.rs` (Tag, Context) | `exiftool-formats/tag_lookup.rs` + `exiftool-tags` | ✅ Superset. We use generated tables (~2500 tags). |
| `util.rs` (read8, read16, BufReadExt, ReadExt) | Std lib + custom in formats | ✅ Equivalent. We use `read_exact`, `stream_position`. |
| `endian.rs` | `exiftool-core/byte_order.rs` | ✅ Ported. |

### 1.2 Logic Parity

- **TIFF IFD limit:** Ref limits IFDs to 8 (tiff.rs:215); we use `ifd_index < 100` (tiff.rs:145). **Recommendation:** Consider lowering to 8 for parity with ref's security stance.
- **Unknown type handling:** Ref uses `Value::Unknown(typ, cnt, ofs)` for lazy parse; we use `ExifFormat::from_u16` and parse immediately. Both valid.
- **continue_on_error:** Ref supports `Parser::continue_on_error`; we don't expose this. Low priority.

---

## 2. FIXME / TODO / Unfinished Code

### 2.1 TODO (Actionable)

| File:Line | Content | Recommendation |
|-----------|---------|----------------|
| `heic_writer.rs:869` | "TODO: Full implementation would: 1. Calculate new sizes... 2. Rebuild meta box... 3. Append EXIF..." | Document as known limitation. Add `add_exif_to_heic_without_exif()` as future work. For now, copy file as-is when no EXIF. |

### 2.2 Other Markers

- `id3.rs:288` — `"TXXX" => "UserDefined"` — not a TODO; string literal in match.
- `isobmff.rs:434` — test data `XXXXx` — not a FIXME.

---

## 3. Bugs & Illogical Places

### 3.1 Confirmed Bug: WebP EXIF Sub-IFD Parsing

**File:** `crates/exiftool-formats/src/webp.rs`  
**Lines:** 49-74 (`parse_exif_chunk`)

**Issue:** WebP EXIF chunk contains full TIFF structure. `parse_exif_chunk` only iterates IFD0 entries and does NOT follow:
- 0x8769 (ExifIFD pointer) → EXIF sub-IFD, MakerNotes
- 0x8825 (GPS IFD pointer) → GPS data
- 0xA005 (Interop IFD pointer) → Interop data

**Fix:** Replace `parse_exif_chunk` body with a call to a shared TIFF EXIF parser (see Deduplication below), or inline the full sub-IFD logic as in `jpeg::parse_exif`.

### 3.2 Minor: JPEG/HEIC/PNG InteropIFD

**Files:** `jpeg.rs`, `heic.rs`, `png.rs`

**Issue:** These parsers do NOT handle InteropIFD (0xA005). Only `tiff.rs` does. InteropIFD contains InteropIndex, InteropVersion — rarely used. **Recommendation:** Add 0xA005 handling in shared `parse_tiff_exif` when deduplicating.

### 3.3 IFD Entry Parse Failure Handling

**File:** `exiftool-core/src/ifd.rs:253-257`

**Issue:** On `read_entry` error, we `eprintln!("Warning:...")` and continue. Ref's `continue_on_error` collects errors. **Recommendation:** Consider returning `Result` with collected errors or a config option instead of stdout.

---

## 4. Unused / Dead Code

- **`#[allow(unused_assignments)]`** in `webp.rs:120` — `height` is assigned in VP8X but not all code paths use it. Could refactor to avoid.
- **`lookup_exif_subifd`** and **`lookup_interop`** — both delegate to `lookup_exif`. Kept for API clarity; not dead.
- **Registry parsers** — all 60+ are reachable via `FormatRegistry::new()`. No dead parsers found.

---

## 5. Deduplication Opportunities

### 5.1 Shared `parse_tiff_exif` Function

**Current:** 6 implementations of nearly identical logic:
- `jpeg.rs::parse_exif` (module-level)
- `png.rs::PngParser::parse_exif`
- `webp.rs::WebpParser::parse_exif_chunk` (incomplete)
- `heic.rs::HeicParser::parse_tiff_exif`
- `jxl.rs::JxlParser::parse_tiff_exif`
- `avi.rs::AviParser::parse_tiff_exif`

**Proposal:** Add to `crates/exiftool-formats/src/utils.rs`:

```rust
/// Parse TIFF-format EXIF data into metadata.
/// Single source for JPEG, PNG, WebP, HEIC, JXL, AVI, etc.
pub fn parse_tiff_exif(
    tiff_data: &[u8],
    metadata: &mut Metadata,
    options: ParseTiffExifOptions
) -> Result<()>
```

`ParseTiffExifOptions` could include:
- `extract_thumbnail: bool` (JPEG has IFD1, HEIC/AVI may not)
- `vendor: Option<Vendor>` (pre-set or detect from Make)

**Benefit:** Fix WebP bug in one place; ensure InteropIFD handled consistently.

### 5.2 Thumbnail Extraction

`extract_thumbnail_from_ifd1` is in `jpeg.rs`. TIFF uses similar logic in `tiff.rs::extract_thumbnail`. Consider `utils::extract_jpeg_thumbnail_from_ifd_entries(entries, reader) -> Option<Vec<u8>>`.

### 5.3 Tag Constants

`TAG_THUMBNAIL_OFFSET`, `TAG_THUMBNAIL_LENGTH`, `TAG_COMPRESSION` duplicated in `jpeg.rs` and `tiff.rs`. Move to `exiftool-core::ifd::tags` or `utils`.

---

## 6. Interface Compatibility

- **Public API:** `FormatRegistry`, `FormatParser`, `Metadata`, `Attrs`, `AttrValue` — stable and consistent.
- **exiftool-tags:** Generated; interface depends on codegen. No breaking changes observed.
- **Discard old code compatibility:** Per requirements. No compatibility shims needed.

---

## 7. Architecture Unification

### 7.1 Single Sources of Truth (Already Done)

- `utils::entry_to_attr` — IFD → AttrValue
- `utils::build_exif_bytes` — Metadata → TIFF bytes
- `utils::get_file_size`, `read_with_limit` — file I/O
- `tag_lookup` — tag ID → name

### 7.2 Recommended Additions

1. `utils::parse_tiff_exif` — shared TIFF EXIF parser
2. `utils::extract_jpeg_thumbnail_from_ifd` — shared thumbnail extraction
3. Centralize IFD tag constants (thumbnail, compression, etc.)

---

## 8. File/Line Reference Index

| File | Lines | Notes |
|------|-------|-------|
| `crates/exiftool-formats/src/webp.rs` | 49-74 | BUG: parse_exif_chunk missing sub-IFDs |
| `crates/exiftool-formats/src/webp.rs` | 120 | `#[allow(unused_assignments)]` |
| `crates/exiftool-formats/src/heic_writer.rs` | 862-875 | TODO: full HEIC write with new EXIF |
| `crates/exiftool-formats/src/jpeg.rs` | 460-538 | parse_exif — no InteropIFD |
| `crates/exiftool-formats/src/heic.rs` | 564-629 | parse_tiff_exif — no InteropIFD |
| `crates/exiftool-formats/src/tiff.rs` | 410-457 | process_entry — has InteropIFD (0xA005) |
| `crates/exiftool-core/src/ifd.rs` | 253-257 | eprintln on entry parse failure |
| `crates/exiftool-formats/src/utils.rs` | 70-86 | entry_to_attr — single source |
| `crates/exiftool-formats/src/utils.rs` | 94-149 | build_exif_bytes — single source |
| `crates/exiftool-formats/src/tag_lookup.rs` | 1-41 | tag lookups |
| `crates/exiftool-formats/src/registry.rs` | 48-125 | parser registration order |

---

## 9. Action Plan (Checkboxes)

- [x] **P0:** Fix WebP `parse_exif_chunk` to follow sub-IFD pointers (DONE: switched to shared parser)
- [x] **P1:** Add `utils::parse_tiff_exif` and migrate JPEG, PNG, WebP, HEIC, JXL, AVI
- [x] **P2:** Add InteropIFD (0xA005) handling to shared parser
- [x] **P3:** Document HEIC writer limitation in README/AGENTS.md
- [x] **P4:** IFD limit 8 for TIFF (security parity with ref)
- [x] **P5:** Centralize thumbnail/strip tag constants (`utils::ifd_tags`)
- [x] **P6:** Configurable IFD entry parse error handling (ref: exif-rs `continue_on_error`). Strict `read_ifd` returns Err on first bad entry; `read_ifd_lenient(offset, &mut errors)` collects errors and continues.

### Post-P6: Multi-page TIFF & BigTIFF (per specs)

- [x] **Multi-page TIFF:** Full IFD chain processed (TIFF 6.0). Pages vs thumbnails via NewSubfileType/SubfileType. README updated.
- [x] **BigTIFF:** 64-bit offsets (IFD8, LONG8) per awaresystems.be spec. `read_ifd(u64)` / `as_u64()` / `as_u64_vec()`. StripOffsets/StripByteCounts support LONG8. Sub-IFD pointers use IFD8. Files >4GB need streaming I/O (current: in-RAM, max 100MB).

---

## 10. Approval Block

**Status:** IMPLEMENTED (2025-02-13)

All P0–P6 + multi-page TIFF + BigTIFF spec compliance implemented. All 450+ tests pass.

---

*Generated as part of Bug Hunt workflow. See AGENTS.md and DIAGRAMS.md for architecture details.*

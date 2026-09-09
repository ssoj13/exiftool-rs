# plan1.md — Format parity bug hunt vs ExifTool 13.59

**Status:** investigation complete — **awaiting approval before any code changes.**

**Reference:** `C:/projects/projects.rust.cg/vfx.ref/exiftool` (`Image::ExifTool` VERSION 13.59, `lib/Image/ExifTool.pm:32`).

**Goals:** (1) bugs in formats already ported; (2) formats still worth porting.

Do not implement until approved. Fixes must be systemic (single TIFF-family path, shared EXIF walk).

## Evidence

Read registry, parsers list, traits, TIFF/JPEG/PNG/CR3/MP4, CLI. Compared to ExifTool file-type tables. Did not run live ExifTool vs `exif` (golden testdata empty). Runtime confirmation is Phase 0 after approval.

## Architecture today

CLI `exiftool-cli/src/main.rs:145` and Python call `FormatRegistry::parse` (`registry.rs:84-96`): 16-byte magic, first `can_parse` wins (`parsers.rs:54-128`). `FormatParser::extensions()` exists (`traits.rs:36-37`) but parse never uses path/extension. ExifTool mixes magic + extension.

Docs vs code: README says 17 formats; `docs/src/formats.md` says 90+ and RAW write no; `Metadata::is_writable` (`lib.rs:415-428`) lists CR2/ARW/NEF and MP4/GIF.

## 1. Bugs in already-ported formats

### P0 — TIFF-family detection is wrong (systemic)

**Root cause:** several TIFF-RAW wrappers set `can_parse` = `TiffParser::can_parse` (any TIFF/BigTIFF). First of those in `default_parsers()` is **ErfParser** (`parsers.rs:119`, `erf.rs:28-31`).

Effects (code, not fixtures):

- Every standard TIFF/DNG/NEF/ARW/PEF via `registry.parse` is handled by ErfParser, which delegates to `TiffParser::default()` (`erf.rs:41-48`).
- `format` becomes ERF only if Make contains EPSON; else TIFF, or DNG if tag 0xC612 (`tiff.rs:140-153`). **NEF/ARW/PEF are never named as such** on auto-detect.
- Wrappers with `can_parse` always false are dead for detect: `nef.rs:46-67`, `arw.rs:35-50`, `pef.rs:35-50`, plus NRW/SRF/FFF.
- Later stealers never run: MEF/SRW/RWL/DCR/MOS/IIQ (`mef.rs:28-30`, `srw.rs:28-30`).
- CLI cannot recover (`main.rs:145`). Tests hide it (`raw_formats.rs` uses `by_extension`; `golden/mod.rs:171-173` passes if testdata empty).

**Correct design (SSOT):** one TIFF-family parser.

1. Unique magics stay dedicated (CR2 CR at +8, ORF IIRO, RW2 II 0x55, RAF ASCII, CR3 crx).
2. Standard TIFF/BigTIFF: parse once, then classify from DNGVersion, Make, optional path extension.
3. `FormatRegistry::parse` takes optional path hint (or `parse_path`) — ExifTool model.
4. Collapse dead wrappers into that classifier. Do not make NefParser return true on TIFF magic (same steal as ERF).

**Verify after:** `.nef` -> format NEF; `.tif` non-Epson -> TIFF; `.dng` with 0xC612 -> DNG; `.erf` Epson -> ERF.

### P0 — CR3 CMT3 MakerNotes dropped

`cr3.rs:217-219` CMT3 prefix MakerNotes. `cr3.rs:309-313` match only IFD0/EXIF/GPS; `_ => None` drops all CMT3 tags. Perl: QuickTime CMT3 + Canon.pm. Fix: reuse `makernotes::parse` (same as TIFF 0x927C in `tiff.rs:425-432`).

### P1 — TIFF SubIFD / XMP / IPTC not walked

`tiff.rs:401-468` only 0x8769, 0x8825, 0xA005. Missing 0x014A SubIFD (Exif.pm), 0x02BC XMP, 0x83BB IPTC. Same gap in `utils::parse_tiff_exif`. Extend the one IFD walker; do not copy per format.

### P1 — JPEG vs JPEG.pm / MPF.pm / Trailer.pm

Stop at SOS/EOI (`jpeg.rs:48,67-69`). APP2 ICC only (`jpeg.rs:133`), no MPF. No Extended XMP. No post-EOI trailers.

### P1 — PNG iTXt stub

Non-XMP iTXt sets placeholder `<iTXt data>` (`png.rs:308-312`). Perl ProcessPNG_iTXt extracts UTF-8. No zXIf / post-IEND trailer.

### P1 — MP4/MOV Keys

`mp4.rs` has udta + ilst (`mp4.rs:235-236,862-863`). No keys/mdta (QuickTime.pm ProcessKeys). iPhone MOV GPS/creation Keys missing. HEIC parser is real ISOBMFF items; still narrower than full QuickTime.pm.

### P2 — Incomplete but real

RAF (`raf.rs`): JPEG-preview EXIF, not full FujiFilm RAF. PDF (`pdf.rs`): Info/XMP scan, not xref/ObjStm. RAW writers: thin TiffWriter; docs say no write. NEF comments (`nef.rs:10-21`) stale vs vendor MakerNotes.

### P2 — Tests

Golden empty-pass. Need parity harness vs cloned `exiftool` on fixtures.

## 2. Formats still to port

Already real in Rust (not todo!): DPX, OpenEXR, Radiance/HDR, MXF, R3D, BRAW (no ExifTool BRAW.pm), JXL/JP2, SVG, PDF, EPS/AI, Matroska, M2TS, ASF, FLAC/Ogg/AAC/ID3, BigTIFF-in-TIFF, QuickTime family.

ExifTool has ~100+ processable types. Remaining file modules (not MakerNotes/Lang/Charset):

| Pri | ExifTool | Why | Rust |
|-----|----------|-----|------|
| A | DICOM.pm | medical / volume | none |
| A | FITS.pm | astronomy / sci-vis | none |
| A | XISF.pm | PixInsight | none |
| A | ZISRAW.pm | CZI microscopy | none |
| A | InDesign.pm | print | none |
| A | ZIP.pm + OOXML.pm + 7Z.pm | packages, IDML | none |
| A | FlashPix.pm | OLE compound | none |
| B | MPEG.pm | elementary/program != M2TS | none |
| B | DV.pm | tape ingest | none |
| B | H264.pm | SEI into M2TS | none |
| B | Flash.pm SWF | flv.rs only | partial |
| B | GIMP.pm XCF | source art | none |
| B | MIFF.pm | ImageMagick | none |
| B | MIE.pm | ExifTool sidecar | none |
| B | Font.pm + standalone .icc | color/typo QC | ICC embedded only |
| C | DjVu FLIF BPG PGF PICT PhotoCD WPG PSP Lytro LIF MRC | niche | none |
| C | EXE LNK ISO PCAP Torrent WTV HTML JSON Text RTF VCard PLIST CBOR iWork TNEF | ops | none |

Do not start new formats until TIFF-family detect + CMT3 + SubIFD walker are honest.

## Phases after approval

- [x] **0.** Fixtures + tests proving ERF-steal; stop golden empty-pass.
- [x] **1.** TIFF-family SSOT: classify after parse; optional path hint; collapse wrappers.
- [x] **2.** Shared IFD walker SubIFD/XMP/IPTC; CR3 CMT3 -> makernotes::parse.
- [x] **3.** JPEG MPF + Extended XMP; PNG real iTXt; MP4 keys/mdta.
- [x] **4.** Align is_writable + docs + RAW writer policy with ExifTool.
- [x] **5.** Parity script vs vfx.ref/exiftool/exiftool (`xtask/parity.py`; FileType on testdata).
- [x] **6a.** DICOM + ZIP/OOXML/ODF (ExifTool `DICOM.pm` / `ZIP.pm`).
- [x] **6b.** FITS (ExifTool `FITS.pm`). XISF, CZI, native InDesign still skipped (IDML-via-ZIP already).

## Out of scope until asked

1:1 MakerNotes tag-count. Full PDF engine. Full Fuji RAF CFA. Editing Perl.

## Approve

Approve Phase 0-1 (detect SSOT + tests) first.

# Parity Report: exiftool-rs vs ExifTool (Perl)

**Date:** 2025-02-13  
**Scope:** Full parity assessment vs ExifTool 13.50 (Phil Harvey)  
**Reference:** https://exiftool.org/

---

## 1. Executive Summary

| Dimension | ExifTool Perl | exiftool-rs | Parity |
|-----------|---------------|-------------|--------|
| **File formats (read)** | 150+ types | ~60 parsers | **Partial** (~40%) |
| **File formats (write)** | 90+ types | ~32 writers | **Partial** (~36%) |
| **Metadata types** | EXIF, IPTC, XMP, ICC, MakerNotes, GeoTIFF, AFCP, etc. | EXIF, XMP, IPTC, ICC | **Partial** |
| **MakerNotes vendors** | 25+ | 21 vendors | **Good** |
| **Tag count** | Tens of thousands | ~2500 (generated) | **Partial** |
| **Value conversion** | PrintConv, ValueConv, conditional | Raw + basic interp | **Partial** |
| **CLI** | 100+ options | ~30 options | **Partial** |
| **Python bindings** | Via wrapper | Native (exiftool-py) | **Different** |

**Conclusion:** exiftool-rs achieves **partial parity** in core EXIF reading and a subset of formats. Full parity with ExifTool is not a design goal; exiftool-rs targets a **lightweight pure-Rust subset** for common use cases.

---

## 2. Format Support

### 2.1 Formats: ExifTool vs exiftool-rs

**ExifTool supports (R=Read, W=Write, C=Create):** 3FR, 3GP, 7z, A, AA, AAC, AAE, AAX, ACR, AFM, AI, AIFF, APE, ARQ, ARW, ASF, AVI, AVIF, BMP, BPG, BTF, CAP, C2PA, CHM, COS, CR2, CR3, CRM, CS1, CSV, CZI, DCM, DCP, DCR, DFONT, DIVX, DJVU, DNG, DOC, DOCX, DPX, DR4, DSF, DSS, DYLIB, DV, DVB, DVR-MS, EIP, EPUB, ERF, EXE, EXIF, EXR, EXV, F4A/F4B/F4P/F4V, FFF, FLIR FFF, FITS, FLA, FLAC, FLIF, FLV, FPF, FPX, GIF, GLV, GPR, GZ, HDP/WDP/JXR, HDR, HEIC/HEIF/HIF, HTML, ICC, ICO, ICS, IDML, IIQ, IND/INDD/INDT, INSP, INSV, INX, ISO, ITC, J2C/J2K/JPC, JP2/JPF/JPM/JPX/JPH, JPEG, JSON, JXL, K25, KDC, KEY/KTH, KVAR, LA, LFP/LFR, LIF, LNK, LRV/LRF, M2TS, M4A/M4B/M4P/M4V, MACOS, MAX, MEF, MIE, MIFF, MKA/MKV/MKS, MOBI/AZW, MODD, MOI, MOS, MOV, MP3, MP4, MPG, MPO, MRW, MSI, MXF, NEF/NRW, ORF, OGG, PDF, PEF, PGM, PICT, PLIST, PNG, PSD, QT, RAF, RAW, RM, RSRC, RTF, RW2, SR2/SRF, SRW, THM, TIFF, VOB, VRD, WAV, WebP, WMA/WMV, WV, X3F, XLS/XLSX, XML, XMP, ZIP, ... (150+ total)

**exiftool-rs supports (read):**

| Category | Formats |
|----------|---------|
| **Images** | JPEG, PNG, GIF, BMP, ICO, TIFF, DNG, WebP, HEIC/AVIF, EXR, HDR, SVG, PNM, TGA, PCX, SGI, DPX |
| **RAW** | CR2, CR3, CRW, NEF, NRW, ARW, SRF, ORF, RW2, PEF, RAF, DCR, KDC, K25, ERF, MEF, SRW, RWL, X3F, IIQ, MOS, FFF, BRAW, R3D, MRW |
| **Audio** | MP3 (ID3), FLAC, WAV, AIFF, AU, APE, WV, OGG, TAK, ALAC, AAC, DSD/DFF, Audible, MIDI |
| **Video** | MP4/MOV, AVI, MKV, FLV, ASF, MPEG-TS, MXF |
| **Document** | PDF, EPS, AI (Illustrator), PSD |

### 2.2 Format Gaps (ExifTool has, we don't)

- **Notable missing:** 3GP, 7z, Audible AA (read only), BPG, C2PA/JUMBF, DICOM, DjVu, DOCX, EPUB, EXE/CHM, FITS, FLIR, FPX, GM PDR, HTML, ICC standalone R/W, INDD, Insta360, ISO, ITC, Lytro, MIE, MOBI, OOXML, Palm, PLIST, QuickTime standalone, RSRC, ZIP metadata, many more
- **ExifTool write-only or Create:** EXV, MIE, sidecar formats — we have none

---

## 3. Metadata Types

| Type | ExifTool | exiftool-rs |
|------|----------|-------------|
| **EXIF** | R/W/C full | R/W (TIFF-based) |
| **IPTC (IIM)** | R/W | R (parse), limited write |
| **XMP** | R/W, structured | R (parse), W (basic) |
| **ICC Profile** | R/W/C | R (embed), W (set from file) |
| **MakerNotes** | R/W, 25+ vendors | R only, 21 vendors |
| **GeoTIFF** | R/W | R (via EXIF GPS) |
| **JFIF** | R/W | R |
| **Photoshop IRB** | R/W | R (PSD) |
| **FlashPix** | R | ❌ |
| **AFCP** | R/W | ❌ |
| **ID3** | R/W | R, W (id3_writer) |
| **Lyrics3** | R | ❌ |
| **C2PA/JUMBF** | R | ❌ |
| **QuickTime** | R/W | R (MP4/MOV) |
| **GM PDR** | R | ❌ |

---

## 4. MakerNotes / Vendor Parity

**ExifTool:** Apple, Canon, Casio, DJI, FLIR, FujiFilm, GE, Google, GoPro, HP, JVC, Kodak, Leaf, Minolta, Motorola, Nikon, Nintendo, Olympus, Panasonic, Pentax, Phase One, Reconyx, Ricoh, Samsung, Sanyo, Sigma, Sony

**exiftool-rs:** Apple, Canon, Casio, DJI, Fujifilm, Google, GoPro, Hasselblad, Huawei, Kodak, Leica, Minolta, Motorola, Nikon, Olympus, OnePlus, Oppo, Panasonic, Pentax, Phase One, Realme, Ricoh, Samsung, Sigma, Sony, Vivo, Xiaomi

**Gaps:** FLIR, GE, HP, JVC, Leaf (as separate), Nintendo, Reconyx, Sanyo

---

## 5. Tag & Value Conversion

| Feature | ExifTool | exiftool-rs |
|---------|----------|-------------|
| **Tag count** | Tens of thousands | ~2500 (exiftool-tags generated) |
| **Tag groups** | EXIF:*, IPTC:*, XMP:*, etc. | Flat namespace, some prefixes |
| **ValueConv** | Type conversion, conditional | Raw → AttrValue, basic |
| **PrintConv** | Enum labels (e.g. Orientation → "Rotate 90 CW") | Numbers only, some interp |
| **Conditional** | `$format eq "RAW"` | ❌ |
| **Composite tags** | ImageSize, Megapixels, etc. | Limited (composite.rs) |
| **MWG recommendations** | Supported | ❌ |

---

## 6. CLI Features

| Option | ExifTool | exiftool-rs |
|--------|----------|-------------|
| `-s` (short output) | ✓ | ✓ |
| `-g` (group) | ✓ | ✓ |
| `-G` (numeric group) | ✓ | ✓ |
| `-f` (format: csv, json, html, xml) | ✓ | json, csv, html, xml |
| `-t` (tabular) | ✓ | ✓ |
| `-p` (in-place write) | ✓ | ✓ |
| `-w` (write to new file) | ✓ | ✓ |
| `--shift` (date/time shift) | ✓ | ✓ |
| `--geotag` (GPX) | ✓ | ✓ |
| `--tagsFromFile` | ✓ | ✓ |
| `--rename` (template) | ✓ | ✓ |
| `-r` (recursive) | ✓ | ✓ |
| `-e` / `-ext` (extension filter) | ✓ | ✓ |
| `-if` (conditional) | ✓ | ✓ |
| `-delete` (strip) | ✓ | ✓ |
| `-overwrite_original` | ✓ | ✓ |
| `-lang` (locale) | ✓ | ❌ |
| `-v` (verbose) | ✓ | ✓ |
| `-ee` (extract embedded) | ✓ | Limited |
| Batch processing | ✓ | ✓ |
| `-stay_open` (daemon) | ✓ | ❌ |

---

## 7. Write Support

| Format | ExifTool | exiftool-rs |
|--------|----------|-------------|
| JPEG | R/W/C | R/W |
| TIFF | R/W/C | R/W |
| PNG | R/W/C | R/W |
| WebP | R/W | R/W |
| HEIC | R/W | R/W (add EXIF when missing; update existing) |
| CR2, CR3, NEF, ARW, RW2, PEF, SRW, etc. | R/W | R/W (TIFF-based RAW) |
| EXR, HDR | R | R/W |
| ID3 (MP3) | R/W | R/W |
| FLAC | R/W | R/W (Vorbis comments) |
| PNM (PBM/PGM/PPM/PAM) | R/W | R/W (comment lines) |
| GIF | R/W | R/W (Comment extension) |
| WAV | R/W | R/W (RIFF LIST INFO) |
| JXL | R/W | R/W (container Exif box) |
| ICC | R/W/C | Set from file |

---

## 8. Architecture Differences

| Aspect | ExifTool | exiftool-rs |
|--------|----------|-------------|
| **Runtime** | Perl | Pure Rust |
| **Dependencies** | Perl 5 | Zero runtime |
| **Tag definitions** | .pd file hand-maintained | Generated from ExifTool DB (tags.json) |
| **Config** | .ExifTool_config | ❌ |
| **Plugin API** | UserDefined | ❌ |

---

## 9. Known Limitations (exiftool-rs)

- **BigTIFF >4GB:** Parsing supports 64-bit; files loaded in RAM (max 100MB). True >4GB needs streaming.
- **HEIC write:** Add EXIF when missing now supported (meta box rebuild).
- **Value display:** Orientation, Flash, etc. as numbers, not "Rotate 90 CW", "Fired" etc.
- **Multi-page TIFF:** Full IFD chain processed; per-page EXIF prefix (e.g. `IFD2:Make`) not exposed.
- **RAW writers:** CR2, ARW, ORF, NEF, RAF supported.
- **C2PA:** Not supported.
- **Locale:** No multi-language output.

---

## 10. Recommendations for Parity Improvement

### High priority
1. **PrintConv for common tags:** Orientation, Flash, ExposureProgram → human-readable strings.
2. **HEIC add EXIF:** Implement meta box rebuild for files without EXIF. ✅ Done.
3. **More format writers:** CR2, ARW, ORF done. DNG, MP4 if feasible.

### Medium priority
4. **Tag groups:** Expose `EXIF:Make`, `IPTC:ObjectName`-style namespacing.
5. **Composite tags:** Expand ImageSize, ShutterSpeed, FNumber formatting.
6. **CLI `-ext`:** Filter by extension.
7. **Backup on write:** `-overwrite_original`-style option.

### Low priority
8. **C2PA/JUMBF:** Read-only if demand exists.
9. **Locale:** Add `-lang` for tag/value strings.
10. **Config file:** User-defined tag mappings.

---

## 11. Summary Table

| Area | Parity % | Notes |
|------|----------|-------|
| EXIF read (TIFF-based) | **95** | Core complete; edge cases possible |
| EXIF write | **70** | TIFF/JPEG/PNG/WebP solid |
| XMP | **60** | Parse OK; write basic |
| IPTC | **40** | Parse; limited write |
| MakerNotes | **85** | 21 vendors; no write |
| Format count (read) | **40** | 60 vs 150+ |
| Format count (write) | **11** | ~10 vs 90+ |
| CLI | **30** | Core; missing many options |
| Value conversion | **30** | Raw only; no PrintConv |

---

---

# PART II: MEGA-DETAILED IMPLEMENTATION PLAN

**Purpose:** Point-by-point list of all formats, bugs, glitches, and features to implement.  
**Last updated:** 2025-02-13

---

## 12. FORMATS: FULL INVENTORY (Read)

### 12.1 Image formats (18)

| # | Format | Parser | Extensions | Status | Issues / TODOs |
|---|--------|--------|------------|--------|----------------|
| 1 | JPEG | JpegParser | jpg, jpeg, jpe | ✓ | Parse OK; APP1 EXIF, XMP, IPTC, ICC, Ducky. InteropIFD handled in shared parser. |
| 2 | PNG | PngParser | png | ✓ | eXIf, tEXt, zTXt, iTXt, tIME, pHYs |
| 3 | GIF | GifParser | gif | ✓ | GIF87a/GIF89a; limited metadata |
| 4 | BMP | BmpParser | bmp | ✓ | Basic header |
| 5 | ICO | IcoParser | ico | ✓ | Icon/cursor |
| 6 | WebP | WebpParser | webp | ✓ | FIXED: now uses shared parse_tiff_exif (sub-IFDs). RIFF container |
| 7 | TIFF | TiffParser | tif, tiff, dng | ✓ | BigTIFF, multi-page, IFD chain. Max 100MB in RAM |
| 8 | EXR | ExrParser | exr | ✓ | OpenEXR attributes |
| 9 | HDR | HdrParser | hdr | ✓ | Radiance RGBE |
| 10 | HEIC/HEIF/AVIF | HeicParser | heic, heif, avif | ✓ | ISOBMFF, brands, iloc, iinf |
| 11 | SVG | SvgParser | svg | ✓ | XML-based, limited |
| 12 | PNM | PnmParser | ppm, pgm, pbm, pnm, pam | ✓ | P1–P6, PAM |
| 13 | TGA | TgaParser | tga, tpic, vda, icb, vst | ✓ | Truevision Targa |
| 14 | PCX | PcxParser | pcx, pcc, dcx | ✓ | ZSoft |
| 15 | SGI | SgiParser | sgi, rgb, rgba, bw, iris, int | ✓ | Silicon Graphics |
| 16 | DPX | DpxParser | dpx | ✓ | Film scan SMPTE |
| 17 | JXL | JxlParser | jxl | ✓ | JPEG XL, uses shared parse_tiff_exif |
| 18 | JP2 | Jp2Parser | jp2, jpx, jpf, j2k, jpc, j2c | ✓ | JPEG 2000 |

### 12.2 RAW formats (24)

| # | Format | Parser | Extensions | TIFF-based | MakerNotes | Status |
|---|--------|--------|------------|------------|------------|--------|
| 1 | CR2 | Cr2Parser | cr2 | ✓ | Canon | ✓ |
| 2 | CR3 | Cr3Parser | cr3 | ❌ ISOBMFF | Canon | ✓ |
| 3 | CRW | CrwParser | crw, ciff | ❌ CIFF | Canon | ✓ |
| 4 | NEF/NRW | NefParser, NrwParser | nef, nrw | ✓ | Nikon | ✓ |
| 5 | ARW | ArwParser | arw | ✓ | Sony | ✓ |
| 6 | SRF/SR2 | SrfParser | srf, sr2 | ✓ | Sony | ✓ |
| 7 | ORF | OrfParser | orf, ori | ✓ (magic 0x4F52/0x5352) | Olympus | ✓ |
| 8 | RW2 | Rw2Parser | rw2 | ✓ (magic 0x55) | Panasonic | ✓ |
| 9 | PEF | PefParser | pef | ✓ | Pentax | ✓ |
| 10 | RAF | RafParser | raf | ❌ FUJIFILM magic | Fujifilm | ✓ |
| 11 | DCR | DcrParser | dcr | ✓ | Kodak | ✓ |
| 12 | KDC | KdcParser | kdc | ✓ | Kodak | ✓ |
| 13 | K25 | K25Parser | k25 | ✓ | Kodak | ✓ |
| 14 | ERF | ErfParser | erf | ✓ | Epson | ✓ |
| 15 | MEF | MefParser | mef | ✓ | Mamiya | ✓ |
| 16 | SRW | SrwParser | srw | ✓ | Samsung | ✓ |
| 17 | RWL | RwlParser | rwl | ✓ | Leica | ✓ |
| 18 | FFF/3FR | FffParser | 3fr, fff | ✓ | Hasselblad | ✓ |
| 19 | IIQ | IiqParser | iiq | ✓ | Phase One | ✓ |
| 20 | MOS | MosParser | mos | ✓ | Leaf | ✓ |
| 21 | X3F | X3fParser | x3f | ❌ Sigma | Sigma | ✓ |
| 22 | BRAW | BrawParser | braw | ❌ | Blackmagic | ✓ |
| 23 | R3D | R3dParser | r3d | ❌ | RED | ✓ |
| 24 | MRW | MrwParser | mrw | ❌ | Minolta | ✓ |

### 12.3 Audio formats (14)

| # | Format | Parser | Extensions | Status |
|---|--------|--------|------------|--------|
| 1 | MP3 | Id3Parser | mp3 | ✓ ID3v1/v2 |
| 2 | FLAC | FlacParser | flac | ✓ |
| 3 | WAV | WavParser | wav, wave, bwf | ✓ |
| 4 | AIFF | AiffParser | aiff, aif, aifc | ✓ |
| 5 | AU | AuParser | au, snd | ✓ |
| 6 | OGG | OggParser | ogg, oga, ogv, ogx | ✓ Vorbis/Opus |
| 7 | APE | ApeParser | ape | ✓ |
| 8 | WV | WvParser | wv | ✓ WavPack |
| 9 | DSD/DSF | DsfParser | dsf | ✓ |
| 10 | DSDIFF/DFF | DffParser | dff | ✓ |
| 11 | CAF/ALAC | CafParser | caf | ✓ |
| 12 | TAK | TakParser | tak | ✓ |
| 13 | MIDI | MidiParser | mid, midi, smf, kar | ✓ |
| 14 | Audible | AudibleParser | aa, aax, aaxc | ✓ |
| 15 | AAC | AacParser | aac | ✓ ADTS |

### 12.4 Video formats (7)

| # | Format | Parser | Extensions | Status |
|---|--------|--------|------------|--------|
| 1 | MP4/MOV | Mp4Parser | mp4, m4v, m4a, m4b, m4p, mov, 3gp, 3g2, f4v | ✓ ISOBMFF |
| 2 | AVI | AviParser | avi, divx | ✓ RIFF, EXIF in hdrl |
| 3 | MKV | MkvParser | mkv, mka, mks, mk3d, webm | ✓ Matroska |
| 4 | FLV | FlvParser | flv, f4v | ✓ Flash Video |
| 5 | ASF | AsfParser | wma, wmv, asf | ✓ |
| 6 | MPEG-TS | MpegTsParser | ts, mts, m2ts, mxts | ✓ |
| 7 | MXF | MxfParser | mxf | ✓ Broadcast |

### 12.5 Document formats (4)

| # | Format | Parser | Extensions | Status |
|---|--------|--------|------------|--------|
| 1 | PDF | PdfParser | pdf | ✓ |
| 2 | EPS | EpsParser | eps | ✓ PostScript |
| 3 | AI | AiParser | ai, ait | ✓ Illustrator |
| 4 | PSD | PsdParser | psd, psb | ✓ Photoshop IRB |

### 12.6 Other formats (2)

| # | Format | Parser | Extensions | Status |
|---|--------|--------|------------|--------|
| 1 | RM | RmParser | rm | ✓ Real Media |
| 2 | Audible AA | AudibleParser | aa, aax, aaxc | ✓ |

---

## 13. WRITERS: FULL INVENTORY

| # | Format | Writer | Status | Limitations |
|---|--------|-------|--------|-------------|
| 1 | JPEG | JpegWriter | ✓ | APP1 EXIF update |
| 2 | TIFF | TiffWriter | ✓ | Standard TIFF; BigTIFF write not tested |
| 3 | PNG | PngWriter | ✓ | eXIf chunk |
| 4 | WebP | WebpWriter | ✓ | EXIF chunk |
| 5 | HEIC | HeicWriter | ✓ | Add EXIF when missing; update existing |
| 6 | CR2 | Cr2Writer | ✓ | Canon TIFF-based RAW |
| 7 | ARW | ArwWriter | ✓ | Sony TIFF-based RAW |
| 8 | ORF | OrfWriter | ✓ | Olympus TIFF-based RAW |
| 9 | NEF | NefWriter | ✓ | Nikon TIFF-based RAW |
| 10 | RAF | RafWriter | ✓ | Fujifilm RAF |
| 11 | EXR | ExrWriter | ✓ | OpenEXR |
| 12 | HDR | HdrWriter | ✓ | Radiance |
| 13 | ID3 | Id3Writer | ✓ | MP3 tags |
| 14 | IPTC | IptcWriter | ✓ | Limited (separate from JPEG flow) |

**Missing writers (high value):** CR3, RW2, PEF, DNG, MP4/MOV.

---

## 14. KNOWN BUGS & GLITCHES (Point-by-Point)

### 14.1 Confirmed bugs (fixed in plan1)

| # | Bug | File | Status |
|---|-----|------|--------|
| 1 | WebP EXIF sub-IFD not followed | webp.rs | ✅ FIXED (shared parse_tiff_exif) |
| 2 | IFD entry parse failure → eprintln | ifd.rs | ✅ FIXED (P6: read_ifd strict/lenient) |

### 14.2 Minor / edge cases

| # | Issue | Location | Recommendation |
|---|-------|---------|----------------|
| 1 | `#[allow(unused_assignments)]` height in WebP VP8X | webp.rs:120 | Refactor to use height consistently |
| 2 | IFD limit: ref (exif-rs) uses 8, we use 100 | tiff.rs | Consider 8 for security parity (optional) |
| 3 | BigTIFF files >4GB need streaming I/O | tiff.rs | Current: in-RAM max 100MB; document limit |
| 4 | unwrap/expect in formats | multiple | Audit and replace with proper error handling where possible |
| 5 | DFF format_name in dsf.rs returns "DFF" not "DSF" for DFF parser | dsf.rs | Cosmetic; both formats in one file |

### 14.3 Per-format quirks

| Format | Quirk |
|--------|-------|
| JPEG | Thumbnail extraction from IFD1 works |
| TIFF | Multi-page: per-page EXIF prefix (IFD2:Make) not exposed in flat namespace |
| HEIC | Add EXIF to file without EXIF: NOT supported |
| CR3 | ISOBMFF; different structure from CR2 |
| CRW | CIFF, not TIFF |
| X3F | Sigma proprietary |
| BRAW | Blackmagic proprietary |
| R3D | RED proprietary |
| MRW | Minolta proprietary |

---

## 15. FEATURES TO IMPLEMENT (Checklist)

### 15.1 Value interpretation (PrintConv)

**Already implemented (interp.rs):**

- Orientation (1–8)
- ResolutionUnit, YCbCrPositioning
- ExposureProgram, MeteringMode, LightSource
- Flash (bitmask), SensingMethod
- FileSource, SceneType, CustomRendered, ExposureMode, WhiteBalance
- SceneCaptureType, GainControl, Contrast/Saturation/Sharpness
- SubjectDistanceRange, ColorSpace, Compression, SensitivityType
- GPSAltitudeRef, GPSStatus, GPSMeasureMode, GPSDifferential

**Format helpers:** format_exposure_time, format_fnumber, format_focal_length, format_gps_coord

**Missing (ExifTool has, we need):**

| Tag | Type | Action |
|-----|------|--------|
| PhotometricInterpretation | enum | Add to interp |
| PlanarConfiguration | enum | Add to interp |
| ExposureProgram (extended) | enum | Some values may be missing |
| ComponentsConfiguration | 4-byte | "Y Cb Cr -" style |
| FileSource (extended) | enum | Add more |
| RecommendedExposureIndex | - | Format as ISO |
| GPSLatitudeRef/LongitudeRef | N/S E/W | Simple |
| GPSSpeedRef, GPSTrackRef | enum | Add |
| InteropIndex, InteropVersion | string | Add |

### 15.2 Composite tags (composite.rs)

**Already implemented:**

- ImageSize, Megapixels
- ShutterSpeed, Aperture
- FocalLength35efl
- GPSPosition, GPSAltitude
- LensID
- Duration (video)
- DateTimeOriginal

**Missing / to expand:**

- PreviewImage, Thumbnail (extraction path)
- CircleOfConfusion ✅
- HyperfocalDistance ✅
- LightValue ✅
- ScaleFactor35efl ✅

### 15.3 Metadata types

| Type | Read | Write | Action |
|------|------|-------|--------|
| EXIF | ✓ | ✓ | - |
| XMP | ✓ | Partial | Expand structured write |
| IPTC | ✓ | Limited | Improve IptcWriter integration |
| ICC | ✓ | Set from file | Standalone ICC R/W not supported |
| MakerNotes | ✓ | ❌ | Read-only; no MakerNotes write |
| GeoTIFF | ✓ (via GPS) | - | - |
| FlashPix | ❌ | ❌ | Low priority |
| AFCP | ❌ | ❌ | - |
| C2PA/JUMBF | ❌ | ❌ | Future if demand |
| Lyrics3 | ❌ | ❌ | MP3 only; low priority |

### 15.4 CLI options

| Option | ExifTool | exiftool-rs | Status |
|--------|-----------|-------------|--------|
| -s (short) | ✓ | ✓ | ✓ |
| -g (get tag) | ✓ | ✓ | ✓ |
| -G (numeric group) | ✓ | ✓ | ✓ |
| -f (format) | ✓ | ✓ | json, csv, html, xml |
| -t (tabular) | ✓ | ✓ | ✓ |
| -p (in-place) | ✓ | ✓ | ✓ |
| -w (write to file) | ✓ | ✓ | ✓ |
| --shift | ✓ | ✓ | ✓ |
| --geotag | ✓ | ✓ | ✓ |
| --tagsFromFile | ✓ | ✓ | ✓ |
| --rename | ✓ | ✓ | ✓ |
| -r (recursive) | ✓ | ✓ | ✓ |
| -e (extensions) | ✓ | ✓ | ✓ (as -e/--ext) |
| -x (exclude) | ✓ | ✓ | ✓ |
| -if (conditional) | ✓ | ✓ | eq, ne, gt, lt, contains, etc. |
| --delete | ✓ | ✓ | ✓ |
| -overwrite_original | ✓ | ✓ | ✓ |
| -lang | ✓ | ✓ | Option added; locale tables TBD |
| -v (verbose) | ✓ | ✓ | ✓ |
| -ee (extract embedded) | ✓ | Limited | Expand |
| -T (thumbnail) | - | ✓ | ✓ |
| -P (preview) | - | ✓ | ✓ |
| -stay_open | ✓ | ❌ | Daemon mode; low priority |

### 15.5 MakerNotes vendors

**Present (21):** Apple, Canon, Casio, DJI, Fujifilm, Google, GoPro, Hasselblad, Huawei, Kodak, Leica, Minolta, Motorola, Nikon, Olympus, OnePlus, Oppo, Panasonic, Pentax, Phase One, Realme, Ricoh, Samsung, Sigma, Sony, Vivo, Xiaomi

**Missing vs ExifTool:** FLIR, GE, HP, JVC, Leaf (as separate), Nintendo, Reconyx, Sanyo

---

## 16. DEDUPLICATION & ARCHITECTURE (Done / Pending)

| Item | Status |
|------|--------|
| utils::parse_tiff_exif | ✅ Done; used by JPEG, PNG, WebP, HEIC, JXL, AVI |
| utils::entry_to_attr | ✅ Done |
| utils::build_exif_bytes | ✅ Done |
| utils::ifd_tags (StripOffsets, etc.) | ✅ Done |
| InteropIFD (0xA005) in parse_tiff_exif | ✅ Done |
| Thumbnail extraction shared fn | Partial; JPEG has extract_thumbnail_from_ifd1; TIFF has extract_thumbnail |
| Tag constants centralization | ✅ ifd_tags |

---

## 17. PRIORITIZED ACTION LIST

### P0 (Critical)

1. **HEIC add EXIF:** Implement meta box rebuild for HEIC without EXIF. ✅ Done.
2. **Error handling:** Reduce unwrap/expect in hot paths; return Result where appropriate. (Partial: CLI output, rename.)

### P1 (High)

3. **PrintConv integration:** Wire interpret_value into CLI output (values shown as strings, not numbers).
4. **More RAW writers:** CR2, ARW, ORF, RW2, PEF, SRW, etc. ✅
5. **CLI -G (numeric group):** Output group numbers. ✅
6. **CLI -overwrite_original:** Backup before overwrite option. ✅

### P2 (Medium)

7. **CLI -t (tabular):** Tab-separated output.
8. **CLI -if:** Conditional processing.
9. **Tag groups:** EXIF:*, IPTC:* namespacing in output.
10. **Composite tags:** CircleOfConfusion, LightValue, HyperfocalDistance, ScaleFactor35efl ✅
11. **XMP write:** More structured XMP write support.
12. **MakerNotes write:** At least Canon/Nikon for critical tags.

### P3 (Low)

13. **CLI -v (verbose):** Debug output. ✅
14. **CLI -lang:** Option added; locale translation tables TBD.
15. **C2PA/JUMBF:** Read-only parser if needed.
16. **New MakerNotes:** FLIR, JVC, Reconyx, etc.
17. **Config file:** User-defined tag mappings (.ExifTool_config style).
18. **-stay_open daemon:** Batch mode via stay-open pipe.

---

## 18. TEST COVERAGE CHECKLIST

| Area | Tests |
|------|-------|
| RawValue as_u64, as_u64_vec | exiftool-core |
| read_ifd strict/lenient | exiftool-core |
| BigTIFF IFD with LONG8 | exiftool-core |
| BigTIFF IFD chain | exiftool-core |
| parse_bigtiff_metadata | exiftool-formats |
| parse_tiff_ifd_chain_with_thumbnail | exiftool-formats |
| WebP EXIF sub-IFD | Via shared parser |
| HEIC parse | - |
| CR3 parse | - |
| Writers (JPEG, TIFF, PNG, WebP, HEIC, NEF, RAF) | - |

---

## 19. FILE REFERENCE INDEX

| File | Key functions / notes |
|------|------------------------|
| exiftool-core/ifd.rs | read_ifd, read_ifd_lenient, BigTIFF IFD8 |
| exiftool-core/value.rs | RawValue, as_u64, as_u64_vec |
| exiftool-formats/utils.rs | parse_tiff_exif, entry_to_attr, build_exif_bytes, ifd_tags |
| exiftool-formats/tiff.rs | TiffParser, BigTIFF, multi-page, extract_preview |
| exiftool-formats/heic_writer.rs | HEIC write, add EXIF when missing ✅ |
| exiftool-formats/webp.rs | Uses parse_tiff_exif |
| exiftool-formats/jpeg.rs | Uses parse_tiff_exif |
| exiftool-tags/interp.rs | interpret_value, format_* helpers |
| exiftool-formats/composite.rs | add_composite_tags |
| exiftool-formats/makernotes/ | 21 vendor parsers |
| exiftool-cli/main.rs | All CLI logic |

---

*Generated for exiftool-rs mega-detailed plan. See plan1.md for bug hunt report and Part I for parity summary.*

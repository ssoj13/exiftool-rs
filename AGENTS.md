# AGENTS.md — exiftool-rs

Pure-Rust metadata stack. Reference: ExifTool **13.59** at `C:/projects/projects.rust.cg/vfx.ref/exiftool`.

## Dependencies

- **Our repos** (`ssoj13/*`, private forks): Cargo `git` over **SSH**, e.g. `{ git = "ssh://git@github.com/ssoj13/exr-rs.git" }`. Same for submodules: `git@github.com:ssoj13/...`. Do not pin sibling crates with a local `path = "C:/projects/..."` except inside a single workspace.
- **Public crates**: crates.io versions, or `https://github.com/...` git when we need a public fork/rev.
- OpenEXR: **`exr-core`** from `ssh://git@github.com/ssoj13/exr-rs.git`.
- JPEG 2000 Part-1: **`jpg-rs`** from `ssh://git@github.com/ssoj13/jpg-rs.git` (not baseline JPEG).
- HTJ2K / JPH: **`jph-rs`** from `ssh://git@github.com/ssoj13/jph-rs.git`.

## Layout

```
exiftool-cli / exiftool-py
        |
        v
FormatRegistry.parse_file(path)   # magic + extension hint (TIFF-family FileType)
FormatRegistry.parse(reader)      # magic only; unnamed TIFF may use IFD0 Make
        |
        v
parsers::default_parsers()        # first can_parse(header) wins
        |
        +-- unique magic (JPEG, PNG, RAF, CR2, ORF, RW2, CR3, HEIC, MP4, ZIP, DICOM, FITS, …)
        +-- TIFF-family stealers can_parse = false (ERF, MEF, SRW, NEF, …)
        +-- TiffParser then tiff_family::classify (DNGVersion, extension, Make)
                |
                v
         Metadata { format, exif, xmp, icc, thumbnail, preview, pages }
```

| Crate | Role |
|-------|------|
| `exiftool-core` | Byte order, IFD reader, raw values |
| `exiftool-tags` | Generated tag names from ExifTool Perl |
| `exiftool-formats` | File parsers/writers + MakerNotes |
| `exiftool-xmp` / `exiftool-iptc` / `exiftool-icc` | Sidecar blobs |
| `exiftool-cli` / `exiftool-py` | Front ends — `parse_file` when a path exists |
| `xtask` | `codegen` / `dump` / DICOM table gen / parity vs Perl |

## Detection (as implemented)

Header length: `DETECT_HEADER_LEN` = 132 (DICOM preamble + `DICM`).

1. Unique magics first (JPEG `FF D8`, PNG, RAF `FUJIFILMCCD-RAW`, CR2 `CR` at +8, ORF, RW2, ZIP `PK`, FITS `SIMPLE  =`+20 spaces+`T`, DICOM, 7z `7z\xbc\xaf\x27\x1c`, …).
2. TIFF-RAW wrappers that used to steal TIFF magic (`ErfParser`, …) have `can_parse = false`; FileType comes from `tiff_family` after `TiffParser`.
3. Path hint: `.nef` → NEF even if Make is empty; generic `.tif` stays TIFF.

## TIFF / EXIF walk

IFD0 + ExifIFD + GPS + Interop. Shared `utils::apply_subifd_xmp_iptc`: SubIFD `0x014A`, XMP `0x02BC`, IPTC `0x83BB`. MakerNotes `0x927C` via `makernotes::parse`. CR3 CMT3 is a TIFF blob into Canon MakerNotes. Nikon ShotInfo `0x0091`: version/size dispatch; `NIKON_OFFSETS` tables use piecewise decrypt (`PrepareNikonOffsets`).

## Tests

- Unit tests in each parser.
- Golden: `crates/exiftool-formats/tests/testdata` + `tests/golden/expected`. Empty testdata is a **failure**. `UPDATE_GOLDEN=1` regenerates JSON.
- Parity: `python xtask/parity.py` vs `vfx.ref/exiftool/exiftool` (Perl).

## Writing

- RAF: ExifTool WriteRAF -- rewrite preview JPEG EXIF, 4-byte pad, fix header pointers, copy from nextPtr at 0x5C. Do not treat 0x5C/0x60 as CFA offset/length.
- TIFF-family RAW (NEF/NRW/CR2/ARW/ORF/RW2/…): `tiff_rewrite` — IFD0/Exif/GPS overlay; copy SubIFD, strips/tiles (with original padding), MakerNotes blobs. CR2 keeps 16-byte header (`WriteCR2`). BigTIFF uses 16-byte header / 20-byte entries / 8-byte offsets. A100 ARW: `FinishARW` (do not treat 0x14a as SubIFD; append Minolta MRW then CFA). Do not use the old short `TiffWriter` rebuild.
- HEIC: update existing EXIF item, or create one when missing (`HeicWriter::create_exif_item`).
- CR3: rewrite CMT1/CMT2/CMT4 TIFF boxes (IFD0/Exif/GPS), optional XMP UUID, patch CTBO + stco/co64. Keep CMT3 MakerNotes as a blob. Preserve `mdat` largesize header so chunk offsets stay valid.
- MakerNotes write: FujiFilm IFD fields overlay existing tags only (ExifTool Permanent), then AFCSettings int32u. Other IFD vendors patch in place. Nested Olympus/Pentax IFDs, Canon/Sony/Nikon index blobs, Sony Tag9405a (including int16s CorrParams), Canon HDRInfo/VignettingCorr2, Kodak KDK Main scalars overlay when the parser table has the field. Unknown magics stay blobs.
- MP4/MOV: XMP UUID box. WAV/FLAC/MP3: existing writers; `is_writable` true.
- 7z read: unencoded header or LZMA encoded header (id 23). AES headers warn.

## Language

Chat with the owner in Russian when they write Russian. Code, identifiers, and repo `.md` (this file, README, plans) stay **English**.

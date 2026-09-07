# plan2.md — remaining format work (ordered)

**Status:** phases A–H landed. **Still open** (see below).

**Reference:** ExifTool 13.59 `C:/projects/projects.rust.cg/vfx.ref/exiftool`. FujiFilm.pm / Nikon.pm / 7Z.pm / Write* as needed. EXR: `git@github.com:ssoj13/exr-rs.git` (`exr-core`).

## Dependency rule

```toml
# ours
exr-core = { git = "ssh://git@github.com/ssoj13/exr-rs.git" }
# public
flate2 = "1.1"
```

No `path = "C:/projects/..."` for sibling git repos.

## Phases

- [x] **A.** Docs match code: `AGENTS.md`, `DIAGRAMS.md`; this plan. `plan1.md` checkboxes 0–4 were already done in code (TIFF-family, IFD walk, JPEG/PNG/MP4, is_writable).
- [x] **B.** Golden testdata must not empty-pass. Copy ExifTool `t/images` samples. `python xtask/parity.py` vs Perl for FileType on those files.
- [x] **C.** 7z (`7Z.pm`): magic `7z\xbc\xaf\x27\x1c`; unencoded header file list; encoded header (id 23) LZMA (`lzma-rs`).
- [x] **D.** Fuji RAF to ExifTool FujiFilm.pm depth: RAF directory, CFA tags, MakerNotes from preview JPEG — not JPEG-only stub.
- [x] **E.** Nikon NEF/NRW: Type-3 MakerNotes IFD, decrypt (`ProcessNikonEncrypted` / `@xlat`), LensData 0100/0101/02xx, ColorBalance layouts, ShotInfo version dispatch (D40–Z9 / `NIKON_OFFSETS` piecewise decrypt), PreviewIFD offsets relocated.
- [x] **F.** Switch EXR parser/writer from crates.io `exr` to `exr-core` SSH git. Keep public `exr` out after switch.
- [x] **F2.** JPEG 2000: `jpg-rs` + HTJ2K `jph-rs` via SSH (not local path).
- [x] **G.** RAW writers: RAF = WriteRAF. TIFF-family RAW (NEF/CR2/ARW/ORF/…) = `tiff_rewrite` via `TiffWriter::write`; `is_writable` true except CR3.
- [x] **H.** JPEG post-EOI trailers (AFCP / FotoStation / PhotoMechanic / Samsung / CanonVRD detect). PNG `zXIf` (zlib EXIF, same as `eXIf`).

## Still open

1. MakerNotes field write (not only blob copy).
2. CR3 write.
3. 1:1 remaining MakerNotes vendors (NikonCustom ShotInfo subdirs, other brands).

## Out of scope unless asked

XISF, CZI, native InDesign, FlashPix, 1:1 every MakerNotes vendor besides Fuji+Nikon, full PDF engine.

## Verify per phase

`cargo test -p exiftool-formats --lib` plus that phase’s fixtures / parity. No empty golden.

# Fuzz Testing

LibFuzzer targets for parser robustness. Keep this crate at repo-root `fuzz/` (`cargo fuzz` looks for that name). Do not copy seeds into `fuzz/corpus/` — pass existing sample dirs.

## Requirements

- Rust nightly
- `cargo install cargo-fuzz`
- Unix-like host for libFuzzer (Linux or WSL). Windows MSVC is not supported.

## Running (from repository root)

```bash
cargo +nightly fuzz list

# JPEG seeds: root `tests/` samples + golden testdata
cargo +nightly fuzz run fuzz_jpeg -- tests crates/exiftool-formats/tests/testdata

# Time-boxed smoke
cargo +nightly fuzz run fuzz_jpeg -- tests crates/exiftool-formats/tests/testdata -max_total_time=60
```

Generated coverage corpus and crashes stay in `fuzz/corpus/<target>/` and `fuzz/artifacts/` (gitignored).

## Targets

| Target | Description |
|--------|-------------|
| `fuzz_jpeg` | JPEG/JFIF parser |
| `fuzz_png` | PNG parser |
| `fuzz_tiff` | TIFF/DNG parser |
| `fuzz_webp` | WebP parser |
| `fuzz_heic` | HEIC/HEIF parser |
| `fuzz_cr3` | Canon CR3 parser |
| `fuzz_registry` | Format auto-detection |

`fuzz_png` / `fuzz_tiff` / `fuzz_heic` / `fuzz_cr3` can use the same two dirs; libFuzzer ignores files the target does not exercise.

## Reproducing crashes

```bash
cargo +nightly fuzz run fuzz_jpeg fuzz/artifacts/fuzz_jpeg/crash-xxxxx
```

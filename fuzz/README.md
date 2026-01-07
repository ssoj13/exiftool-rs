# Fuzz Testing

This directory contains fuzz targets for testing exiftool-rs parsers with arbitrary input data.

## Requirements

- Rust nightly toolchain
- `cargo-fuzz` (`cargo install cargo-fuzz`)
- Linux or WSL (libFuzzer requires Unix-like environment)

## Running Fuzz Tests

```bash
# Install cargo-fuzz (one-time)
cargo install cargo-fuzz

# List available targets
cargo +nightly fuzz list

# Run a specific fuzz target
cargo +nightly fuzz run fuzz_jpeg

# Run with timeout per test case
cargo +nightly fuzz run fuzz_jpeg -- -timeout=5

# Run for a limited time (60 seconds)
cargo +nightly fuzz run fuzz_jpeg -- -max_total_time=60

# Run with initial corpus
cargo +nightly fuzz run fuzz_jpeg corpus/jpeg/
```

## Available Targets

| Target | Description |
|--------|-------------|
| `fuzz_jpeg` | JPEG/JFIF parser |
| `fuzz_png` | PNG parser |
| `fuzz_tiff` | TIFF/DNG parser |
| `fuzz_webp` | WebP parser |
| `fuzz_heic` | HEIC/HEIF parser |
| `fuzz_cr3` | Canon CR3 parser |
| `fuzz_registry` | Format auto-detection |

## Creating Seed Corpus

For better fuzzing coverage, provide sample files:

```bash
mkdir -p fuzz/corpus/jpeg
cp test_images/*.jpg fuzz/corpus/jpeg/
cargo +nightly fuzz run fuzz_jpeg fuzz/corpus/jpeg/
```

## Reproducing Crashes

When a crash is found, reproduce it:

```bash
cargo +nightly fuzz run fuzz_jpeg artifacts/fuzz_jpeg/crash-xxxxx
```

## Coverage-Guided Fuzzing Tips

1. Start with valid sample files in corpus
2. Run for extended periods (hours/days) for best coverage
3. Use `-jobs=N` for parallel fuzzing
4. Check `fuzz/artifacts/` for crash inputs

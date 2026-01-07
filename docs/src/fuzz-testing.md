# Fuzz Testing

Fuzz testing (fuzzing) is an automated testing technique that feeds random or semi-random data to a program to find bugs, crashes, and security vulnerabilities. For a metadata parser like exiftool-rs, fuzzing is essential because:

- Parsers handle untrusted input (arbitrary image files)
- Binary formats have complex structures prone to edge cases
- Memory safety issues can lead to security vulnerabilities

This project uses [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) with libFuzzer for coverage-guided fuzzing.

## Prerequisites

```bash
# Install nightly Rust (required for cargo-fuzz)
rustup install nightly

# Install cargo-fuzz
cargo +nightly install cargo-fuzz
```

## Available Fuzz Targets

| Target | Format | Description |
|--------|--------|-------------|
| `fuzz_jpeg` | JPEG | APP1/APP13 segments, EXIF, XMP, IPTC |
| `fuzz_png` | PNG | eXIf, tEXt, iTXt, zTXt chunks |
| `fuzz_tiff` | TIFF/DNG | IFD chains, tag parsing |
| `fuzz_webp` | WebP | VP8/VP8L/VP8X containers |
| `fuzz_heic` | HEIC/AVIF | ISOBMFF boxes, item extraction |
| `fuzz_cr3` | Canon CR3 | ISOBMFF with Canon-specific boxes |
| `fuzz_registry` | All | Format auto-detection |

## Running Fuzz Tests

### Basic Usage

```bash
cd fuzz

# Run a single target (runs until Ctrl+C)
cargo +nightly fuzz run fuzz_jpeg

# Run with time limit (60 seconds)
cargo +nightly fuzz run fuzz_jpeg -- -max_total_time=60

# Run with iteration limit
cargo +nightly fuzz run fuzz_jpeg -- -runs=100000

# Run with multiple parallel jobs
cargo +nightly fuzz run fuzz_jpeg -- -jobs=4 -workers=4
```

### Quick Smoke Test

Run all targets for 30 seconds each:

```bash
cd fuzz

# Bash/PowerShell
for target in fuzz_jpeg fuzz_png fuzz_tiff fuzz_webp fuzz_heic fuzz_cr3 fuzz_registry
do
    echo "=== Running $target ==="
    cargo +nightly fuzz run $target -- -max_total_time=30
done
```

### CI Integration

For continuous integration, run with a fixed number of iterations:

```bash
cargo +nightly fuzz run fuzz_jpeg -- -runs=10000
cargo +nightly fuzz run fuzz_png -- -runs=10000
# etc.
```

## Understanding Output

### Normal Operation

```
#12345  INITED cov: 1234 ft: 5678 corp: 100/50Kb exec/s: 1000
#12400  NEW    cov: 1240 ft: 5700 corp: 101/51Kb exec/s: 1000
```

- `#12345` - Iteration number
- `INITED` - Fuzzer initialized
- `NEW` - Found new interesting input (increases coverage)
- `cov:` - Number of code edges covered
- `ft:` - Number of features (coverage counters)
- `corp:` - Corpus size (inputs / total bytes)
- `exec/s:` - Executions per second

### What "Good" Looks Like

- High `exec/s` (thousands per second)
- Growing `cov:` and `ft:` numbers (finding new code paths)
- No crashes or timeouts
- Eventually stabilizes when coverage is saturated

### Crash Found

If fuzzer finds a crash:

```
==12345== ERROR: libFuzzer: deadly signal
artifact_prefix='./fuzz/artifacts/fuzz_jpeg/'; 
Test unit written to ./fuzz/artifacts/fuzz_jpeg/crash-abc123def456
```

The crashing input is saved for reproduction.

## Handling Crashes

### Reproduce a Crash

```bash
# Re-run with the crash file
cargo +nightly fuzz run fuzz_jpeg fuzz/artifacts/fuzz_jpeg/crash-abc123def456
```

### Minimize Crash Input

Reduce the crash file to minimal reproducer:

```bash
cargo +nightly fuzz tmin fuzz_jpeg fuzz/artifacts/fuzz_jpeg/crash-abc123def456
```

### Debug the Crash

```bash
# Build with debug info
cargo +nightly fuzz build

# Run under debugger
rust-lldb target/x86_64-unknown-linux-gnu/release/fuzz_jpeg fuzz/artifacts/fuzz_jpeg/crash-abc123def456
```

### Report and Fix

1. Create a test case from the minimized input
2. Add to `tests/` for regression testing
3. Fix the bug in the parser
4. Verify fix with the crash input

## Corpus Management

The fuzzer builds a corpus of interesting inputs that maximize code coverage.

### Corpus Location

```
fuzz/corpus/<target>/
```

### Seeding with Real Files

Improve fuzzing effectiveness by seeding with real image files:

```bash
# Create corpus directory
mkdir -p fuzz/corpus/fuzz_jpeg

# Copy test files
cp testdata/*.jpg fuzz/corpus/fuzz_jpeg/
cp ~/photos/sample.jpg fuzz/corpus/fuzz_jpeg/

# Fuzzer will mutate these as starting points
cargo +nightly fuzz run fuzz_jpeg
```

### Corpus Minimization

Remove redundant inputs while preserving coverage:

```bash
cargo +nightly fuzz cmin fuzz_jpeg
```

## Best Practices

### Regular Fuzzing

- Run fuzzing regularly (nightly builds, pre-release)
- Track coverage over time
- Keep corpus in version control (or artifact storage)

### After Code Changes

When parser code changes:

```bash
# Re-run fuzzing to test new code paths
cargo +nightly fuzz run fuzz_jpeg -- -max_total_time=300
```

### Performance Tips

- Use release builds (default with cargo-fuzz)
- Run on fast machines with multiple cores
- Use `-jobs=N -workers=N` for parallelism
- SSD helps with corpus I/O

## Fuzz Target Structure

Each fuzz target is a simple harness:

```rust
// fuzz/fuzz_targets/fuzz_jpeg.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use std::io::Cursor;
use exiftool_formats::JpegParser;

fuzz_target!(|data: &[u8]| {
    // Parser should handle any input without panicking
    let mut cursor = Cursor::new(data);
    let _ = JpegParser.parse(&mut cursor);
});
```

The goal: **no panics, no crashes, no undefined behavior** regardless of input.

## Troubleshooting

### "error: could not find `fuzz`"

```bash
# Make sure you're in the fuzz directory
cd fuzz
cargo +nightly fuzz list
```

### Slow Execution

- Check corpus isn't too large: `ls fuzz/corpus/fuzz_jpeg | wc -l`
- Run corpus minimization: `cargo +nightly fuzz cmin fuzz_jpeg`

### Out of Memory

```bash
# Limit memory usage
cargo +nightly fuzz run fuzz_jpeg -- -rss_limit_mb=2048
```

### Timeout Issues

```bash
# Increase timeout for slow inputs
cargo +nightly fuzz run fuzz_jpeg -- -timeout=30
```

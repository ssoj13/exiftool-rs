# Installation

## Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
exiftool-formats = "0.1"
```

For just the core EXIF parsing (smaller dependency footprint):

```toml
[dependencies]
exiftool-core = "0.1"
```

## Python

```bash
pip install exiftool-rs
```

Requires Python 3.8+. Wheels are provided for:
- Linux (x86_64, aarch64)
- macOS (x86_64, arm64)
- Windows (x86_64)

## CLI

Build from source:

```bash
cargo install --path crates/exiftool-cli
```

Or download pre-built binaries from the releases page.

## Building from Source

Requirements:
- Rust 1.70+
- For Python bindings: Python 3.8+ with maturin

```bash
git clone https://github.com/user/exiftool-rs
cd exiftool-rs

# Build everything
cargo build --release

# Run tests
cargo test --workspace

# Build Python wheel
cd crates/exiftool-py
maturin build --release
```

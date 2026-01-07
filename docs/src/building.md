# Building from Source

## Requirements

- Rust 1.70 or later
- For Python bindings: Python 3.8+ and maturin

## Clone and Build

```bash
git clone https://github.com/user/exiftool-rs
cd exiftool-rs

# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

## Build Specific Crates

```bash
# Just the core library
cargo build -p exiftool-formats

# Just the CLI
cargo build -p exiftool-cli --release

# Just Python bindings
cd crates/exiftool-py
maturin build --release
```

## Run Tests

```bash
# All tests
cargo test --workspace

# With verbose output
cargo test --workspace -- --nocapture

# Specific test
cargo test -p exiftool-formats jpeg
```

## Python Bindings

```bash
# Install maturin
pip install maturin

# Build wheel
cd crates/exiftool-py
maturin build --release

# Install locally for development
maturin develop

# Build and install
pip install .
```

## Documentation

```bash
# Rust API docs
cargo doc --workspace --open

# mdbook
cd docs/book
mdbook build
mdbook serve  # Local server at localhost:3000
```

## Cross-Compilation

Rust makes cross-compilation straightforward:

```bash
# Add target
rustup target add aarch64-unknown-linux-gnu

# Build
cargo build --release --target aarch64-unknown-linux-gnu
```

## Workspace Structure

```
exiftool-rs/
├── Cargo.toml           # Workspace root
├── crates/
│   ├── exiftool-attrs/  # Attribute types
│   ├── exiftool-core/   # TIFF/IFD primitives
│   ├── exiftool-tags/   # Tag definitions
│   ├── exiftool-formats/# All format parsers
│   ├── exiftool-xmp/    # XMP handling
│   ├── exiftool-iptc/   # IPTC handling
│   ├── exiftool-icc/    # ICC profiles
│   ├── exiftool-py/     # Python bindings
│   └── exiftool-cli/    # CLI tool
├── docs/
│   └── book/            # mdbook source
└── tests/               # Integration tests
```

## Troubleshooting

### Compilation Errors

Make sure you have the latest Rust:

```bash
rustup update
```

### Python Binding Issues

Ensure maturin matches your Python version:

```bash
pip install --upgrade maturin
maturin build --release
```

### Missing Features

Some features require specific dependencies. Check `Cargo.toml` for feature flags.

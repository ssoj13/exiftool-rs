# Installation

## Requirements

- Rust **1.96 or later**, with Cargo and a native linker.
- Git and access to the [Git dependencies](../dependency-access.md).
- Python **3.8+** for bindings, plus a Python version supported by the pinned PyO3
  dependency. Async wrappers use `asyncio.to_thread`, which requires Python 3.9+.
- Python 3 for the optional `bootstrap.py` helper.

The CI/release workflows publish neither crates.io packages nor PyPI wheels.
The instructions below use a source checkout.

## Clone and check access

```bash
git clone https://github.com/ssoj13/exiftool-rs.git
cd exiftool-rs
cargo fetch --locked
```

A fetch failure mentioning authentication or a missing Git revision can mean
that the dependency is private. Follow [dependency access](../dependency-access.md)
before changing the lockfile.

## CLI

From the repository root:

```bash
cargo install --path crates/exiftool-cli --locked
exif --version
exif --help
```

Cargo installs `exif` into its binary directory (`~/.cargo/bin` by default).
On Windows the executable is `exif.exe`.

For tagged releases, see the repository's
[GitHub Releases](https://github.com/ssoj13/exiftool-rs/releases).
Availability depends on successful release builds; see [CI/CD](../ci-cd.md).

## Rust library

For a Rust application next to your `exiftool-rs` checkout, add local dependencies:

```toml
[dependencies]
exiftool-formats = { path = "../exiftool-rs/crates/exiftool-formats" }
exiftool-attrs = { path = "../exiftool-rs/crates/exiftool-attrs" }
```

Adjust the relative paths to your checkout. `exiftool-formats` provides
`FormatRegistry`, `Metadata`, and format-specific parsers/writers.
`exiftool-attrs` provides `AttrValue` for editing typed values.
Use `exiftool-core` directly only when you need low-level TIFF/IFD primitives.

Continue with the [quick start](quickstart.md).

## Python and development builds

Follow [Python installation](../python/installation.md) for virtual environments
and maturin, or [building from source](../building.md) for workspace commands,
Rust API docs, and the documentation book.

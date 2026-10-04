# Building from source

## Prepare the environment

Use Rust **1.96+**, Cargo, Git, and a native linker. Bindings also need Python and
maturin; see [Python installation](python/installation.md).
Complete [dependency access](dependency-access.md) before the first Cargo fetch.

```bash
git clone https://github.com/ssoj13/exiftool-rs.git
cd exiftool-rs
cargo fetch --locked
cargo build -p exiftool-cli --release --locked
```

The CLI is `target/release/exif` (`exif.exe` on Windows).

## Build and check the workspace

```bash
cargo build --workspace --all-targets --locked
cargo test --workspace --exclude exiftool-py --locked
cargo check -p exiftool-py --locked
cargo fmt --all -- --check
cargo clippy --workspace --exclude exiftool-py --all-targets --locked -- -D warnings
```

The ordinary Rust test suite excludes `exiftool-py`, whose library is a Python
extension. Check it separately and use maturin for a Python runtime build.
CI currently treats formatting, Clippy, and Rustdoc findings as advisory while
the existing baseline is cleaned up. See [CI/CD](ci-cd.md).

## Bootstrap helper

The optional Python 3 helper runs commands from the repository root:

| Command | Operation |
|---------|-----------|
| `python bootstrap.py b` | Release workspace build, including all targets |
| `python bootstrap.py b -d` | Debug workspace build |
| `python bootstrap.py t` | Workspace tests, including the Python crate |
| `python bootstrap.py c` | Formatting check and strict workspace Clippy |
| `python bootstrap.py i` | Install `exif` |
| `python bootstrap.py p b` | Build Python wheel |
| `python bootstrap.py p d` | Editable Python install |
| `python bootstrap.py p i` | Build/install Python package |
| `python bootstrap.py m b` | Build the documentation book |
| `python bootstrap.py m s` | Serve the documentation book |

The helper's broad workspace checks are not identical to the CI commands above.

## Documentation

```bash
cargo doc --workspace --exclude exiftool-py --no-deps --locked
cargo install mdbook --locked
cargo install mdbook-mermaid --locked
mdbook build docs
mdbook serve docs
```

The book configuration is `docs/book.toml`, sources are under `docs/src`, and
HTML output goes to `docs/book`. Mermaid assets are included in the repository;
the `mdbook-mermaid` preprocessor must be on `PATH`. Do not run
`mdbook-mermaid install` again for an ordinary book build.

## Generated tag tables

Tag extraction needs Perl and an ExifTool source tree. The CLI itself does not
need either at runtime. Inspect available xtask options before regenerating:

```bash
cargo xtask --help
python bootstrap.py codegen
```

Review generated diffs along with parser changes, and keep ExifTool attribution.

## Troubleshooting

| Symptom | Check |
|---------|-------|
| Dependency fetch authentication error | Access to every Git repository in [dependency access](dependency-access.md) |
| Unsupported Rust syntax or MSRV error | `rustc --version` and Rust 1.96+ |
| `exif` is not found after install | Cargo's binary directory on `PATH` |
| Python extension does not import | Active environment, maturin build, and `exiftool_py` import name |
| Book cannot find `mdbook-mermaid` | Install the preprocessor and verify `PATH` |

For cross-compilation you also need the target linker and any target-specific
system dependencies; installing a Rust target alone does not provide them.
See [contributing](contributing.md) and [releasing](../releasing.md).

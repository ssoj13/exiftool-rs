# Python Installation

## From PyPI

```bash
pip install exiftool-rs
```

## Requirements

- Python 3.8 or later
- No runtime dependencies

## Platform Support

Pre-built wheels are available for:

| Platform | Architecture |
|----------|--------------|
| Linux | x86_64, aarch64 |
| macOS | x86_64 (Intel), arm64 (Apple Silicon) |
| Windows | x86_64 |

## Building from Source

If no wheel is available for your platform:

```bash
# Install build tool
pip install maturin

# Clone and build
git clone https://github.com/user/exiftool-rs
cd exiftool-rs/crates/exiftool-py
maturin build --release

# Install the wheel
pip install target/wheels/exiftool_rs-*.whl
```

## Verify Installation

```python
import exiftool_rs as exif

print(exif.__version__)
```

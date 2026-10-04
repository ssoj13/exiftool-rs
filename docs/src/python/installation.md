# Python installation

The package is named `exiftool-py`, and Python imports it as `exiftool_py`.
The repository release workflow builds CLI archives; it does not publish PyPI
wheels. Build the bindings from source using the instructions below.

## Requirements

- Rust 1.96+, Cargo, Git, and access to [Git dependencies](../dependency-access.md).
- Python 3.8+ as declared by the package, with a version supported by its pinned
  PyO3 dependency. Async helpers require Python 3.9+.
- A virtual environment and maturin.

## Install for development

From the repository root, create a virtual environment:

```bash
python -m venv .venv
```

Activate it for your shell:

```bash
# Bash / zsh
source .venv/bin/activate
```

```powershell
# PowerShell
.\.venv\Scripts\Activate.ps1
```

Then install the build tool and extension:

```bash
python -m pip install maturin
maturin develop --release --manifest-path crates/exiftool-py/Cargo.toml
python -c "import exiftool_py; print(exiftool_py.__version__)"
```

## Build a wheel

From the repository root:

```bash
maturin build --release --manifest-path crates/exiftool-py/Cargo.toml
```

Wheels are placed in the workspace's `target/wheels/` directory. Install the
actual generated wheel path with `python -m pip install`, choosing a wheel that
matches your Python version, OS, and architecture. A local build does not produce
wheels for every platform.

Continue with [usage](usage.md), or see [building](../building.md) for the
bootstrap helper.

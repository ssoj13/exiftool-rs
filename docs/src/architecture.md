# Architecture

exiftool-rs is organized as a Rust workspace with multiple crates, each with 
a focused responsibility.

## Design Principles

1. **Separation of concerns** - Core parsing logic separate from format-specific code
2. **Zero unsafe** - Pure safe Rust (except PyO3 bindings)
3. **Minimal dependencies** - Only what's necessary
4. **Streaming** - Parse from any `Read + Seek` source
5. **Lazy loading** - Don't read what you don't need

## Why a Workspace?

Multiple crates allow:

- **Cherry-picking** - Use only what you need
- **Parallel compilation** - Faster builds
- **Clear boundaries** - Enforced API contracts
- **Independent versioning** - Update parts separately

```
exiftool-rs/
├── crates/
│   ├── exiftool-core/      # TIFF/IFD parsing primitives
│   ├── exiftool-attrs/     # Attribute storage and types
│   ├── exiftool-tags/      # Tag definitions and interpretation
│   ├── exiftool-formats/   # All format parsers
│   ├── exiftool-xmp/       # XMP parsing/writing
│   ├── exiftool-iptc/      # IPTC parsing/writing
│   ├── exiftool-icc/       # ICC profile parsing
│   ├── exiftool-py/        # Python bindings
│   └── exiftool-cli/       # Command-line tool
```

# Contributing

Contributions welcome! Here's how to help.

## Ways to Contribute

- **Bug reports** - Found something broken? Open an issue
- **Format support** - Add parsing for new formats
- **MakerNotes** - Camera-specific metadata decoding
- **Documentation** - Improve docs, add examples
- **Tests** - More test coverage is always good

## Development Setup

```bash
git clone https://github.com/user/exiftool-rs
cd exiftool-rs

# Build
cargo build

# Test
cargo test --workspace

# Lint
cargo clippy --workspace
```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add tests for new functionality
- Document public APIs

## Adding a New Format

1. Create parser in `crates/exiftool-formats/src/`
2. Implement `FormatParser` trait
3. Register in `registry.rs`
4. Add tests
5. Document in mdbook

Example skeleton:

```rust
// newformat.rs
pub struct NewFormatParser;

impl FormatParser for NewFormatParser {
    fn can_parse(&self, header: &[u8]) -> bool {
        // Check magic bytes
        header.starts_with(b"MAGIC")
    }
    
    fn format_name(&self) -> &'static str { "NewFormat" }
    
    fn extensions(&self) -> &'static [&'static str] {
        &["new", "nf"]
    }
    
    fn parse(&self, reader: &mut dyn ReadSeek) -> Result<Metadata> {
        let mut metadata = Metadata::new("NewFormat");
        // Parse...
        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn detect_format() {
        let parser = NewFormatParser;
        assert!(parser.can_parse(b"MAGIC..."));
        assert!(!parser.can_parse(b"OTHER"));
    }
}
```

## Testing

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p exiftool-formats

# Specific test
cargo test -p exiftool-formats jpeg::tests

# With output
cargo test -- --nocapture
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `cargo fmt` and `cargo clippy`
5. Submit PR with clear description

## Questions?

Open a discussion or issue on GitHub.

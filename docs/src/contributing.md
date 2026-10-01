# Contributing

Bug reports, parser fixes, camera-specific metadata samples, and documentation
improvements are welcome. Start with [building](building.md) and
[dependency access](dependency-access.md).

## Report a reproducible issue

Include the commit or version, operating system, command or API call, expected
behavior, and observed result. For parsing/writing issues, identify the format
and relevant tags. Share a minimal redistributable sample when possible; camera
files can contain personal data and copyrighted content.

## Validate a change

From the repository root:

```bash
cargo test --workspace --exclude exiftool-py --locked
cargo check -p exiftool-py --locked
cargo fmt --all -- --check
cargo clippy --workspace --exclude exiftool-py --all-targets --locked -- -D warnings
python -m unittest discover -s .github/scripts -p 'test_*.py'
mdbook build docs
```

Test the affected parser or writer as well as detection. For writing changes,
read the output back, assert edited fields, and check that required original
payloads and offsets remain valid. See [fuzz testing](fuzz-testing.md) for
malformed-input coverage.

## Add a format parser

1. Add a module under `crates/exiftool-formats/src/` implementing
   `FormatParser: Send + Sync`.
2. Declare and, if public, re-export it in `lib.rs`.
3. Register it in **`parsers::default_parsers()`**, not `registry.rs`.
   Put specific signatures before overlapping generic containers.
4. Test valid detection, rejection of unrelated headers, malformed input,
   and representative metadata. Include extension-hint behavior when relevant.
5. Update the [format reference](formats.md) and the relevant category page.

Read [parser design](architecture/parsers.md) for the trait contract and TIFF-family
classification. Custom registries select parsers at runtime; they do not
remove dependencies from the build.

## Documentation changes

Keep the README as the entry point and detailed guidance under `docs/src`.
Add book chapters to `SUMMARY.md`, link related pages, and use relative paths.
Examples should use real APIs and include any setup needed to run them.

Use Mermaid for architecture or process diagrams when it clarifies the text.
Build the book with its preprocessor installed; GitHub renders fenced Mermaid
blocks directly, while mdBook needs the configured assets and preprocessor.

## Submit a pull request

Describe the concrete behavior changed, the reason, and validation performed.
Keep generated tables, fixtures, documentation, and tests aligned with the
implementation. Preserve the license and upstream attribution.

Fork workflows do not receive repository secrets. If dependencies remain
private, dependency-based CI cannot run for an unauthenticated fork. See
[CI/CD](ci-cd.md) and [dependency access](dependency-access.md).

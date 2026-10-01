# exiftool-rs

Read and edit file metadata through a native Rust library, Python bindings,
or the `exif` command-line tool. The project parses files directly rather than
starting an ExifTool subprocess.

This is an experimental port of a subset of [ExifTool](https://exiftool.org/).
Support depends on the container, tag, and camera model. Use the
[format reference](formats.md) to choose a parser and the
[writing guide](writing.md) to understand edit behavior.

## Start here

| Goal | Guide |
|------|-------|
| Install or build | [Installation](getting-started/installation.md) |
| Read your first file in Rust | [Quick start](getting-started/quickstart.md) |
| Inspect and batch-edit files | [CLI](cli.md) |
| Integrate with Python | [Python](python.md) |
| Understand the internals | [Architecture](architecture.md) |
| Contribute a parser or fix | [Contributing](contributing.md) |

## Before building

The workspace requires Rust 1.96+ and access to its Git dependencies.
The Python distribution is `exiftool-py`, imported as `exiftool_py`.
GitHub release automation builds CLI archives; it does not publish Rust crates
or Python wheels. Follow [dependency access](dependency-access.md) before a first
source build.

## What the library returns

`FormatRegistry` detects a container and returns `Metadata`: typed attributes,
raw XMP, optional ICC data, thumbnails, previews, and TIFF page information.
The attribute map is named `exif`, but can also contain non-EXIF container tags.

Reading support is broader than writing support. A writer dispatches according
to the parsed format; changing an attribute in memory alone does not write it
to disk. See [reading](reading.md) and [writing](writing.md).

## License

The project uses the same licensing choice as ExifTool and Perl:
`Artistic-1.0-Perl OR GPL-1.0-or-later`.
ExifTool is Copyright © 2003–2026 Phil Harvey; the Rust port is
Copyright © 2026 Alex Khalyavin. Attribution and license texts are included at
the repository root.

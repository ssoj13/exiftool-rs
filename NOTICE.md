# Copyright and third-party notices

## License

exiftool-rs is distributed under the same terms as ExifTool and Perl itself: either the Perl Artistic License ([LICENSE-ARTISTIC](LICENSE-ARTISTIC)) or the GNU General Public License, version 1 or later ([LICENSE-GPL](LICENSE-GPL)). See [LICENSE](LICENSE).

## Original work: ExifTool

[ExifTool](https://exiftool.org/) is Copyright © 2003–2026 Phil Harvey, the author of the original work.
This project is a Rust port and a derivative work of [ExifTool 13.59](https://github.com/exiftool/exiftool/tree/13.59).
It includes tag definitions generated from ExifTool's Perl sources (`cargo xtask codegen`) and format behavior ported from them.

## Derivative work: exiftool-rs

The Rust implementation is Copyright © 2026 Alex Khalyavin, the author of the derivative work.
It is licensed under the same terms as the original work.

## Test data

Most fixtures in `crates/exiftool-formats/tests/testdata` are byte-identical copies of files from ExifTool 13.59 (`t/images`), distributed with ExifTool.
Any file-specific copyright remains with the respective rights holders; see the [test-data README](crates/exiftool-formats/tests/testdata/README.md).
The two small 7z archives there are synthetic files created for this project.

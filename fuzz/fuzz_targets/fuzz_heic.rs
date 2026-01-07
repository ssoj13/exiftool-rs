#![no_main]

use libfuzzer_sys::fuzz_target;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    // Fuzz HEIC/HEIF parser with arbitrary data
    let mut cursor = Cursor::new(data);
    let _ = exiftool_formats::HeicParser::parse(&mut cursor);
});

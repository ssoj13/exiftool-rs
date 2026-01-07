#![no_main]

use libfuzzer_sys::fuzz_target;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    // Fuzz JPEG parser with arbitrary data
    let mut cursor = Cursor::new(data);
    let _ = exiftool_formats::JpegParser::parse(&mut cursor);
});

#![no_main]

use libfuzzer_sys::fuzz_target;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    // Fuzz format registry (auto-detection) with arbitrary data
    let registry = exiftool_formats::FormatRegistry::new();
    let mut cursor = Cursor::new(data);
    let _ = registry.parse(&mut cursor);
});

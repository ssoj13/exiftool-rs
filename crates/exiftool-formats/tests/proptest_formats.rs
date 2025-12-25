//! Property-based tests for format parsers.
//!
//! Verifies that parsers handle arbitrary input without panicking.

use exiftool_formats::{
    Cr2Parser, Cr3Parser, ExrParser, HdrParser, HeicParser,
    JpegParser, NefParser, PngParser, RafParser, TiffParser,
    FormatParser,
};
use proptest::prelude::*;
use std::io::Cursor;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    /// JPEG parser should not panic on arbitrary data.
    #[test]
    fn jpeg_parse_no_panic(data in prop::collection::vec(any::<u8>(), 0..2048)) {
        let parser = JpegParser;
        let mut cursor = Cursor::new(&data);
        let _ = parser.parse(&mut cursor);
    }

    /// PNG parser should not panic on arbitrary data.
    #[test]
    fn png_parse_no_panic(data in prop::collection::vec(any::<u8>(), 0..2048)) {
        let parser = PngParser;
        let mut cursor = Cursor::new(&data);
        let _ = parser.parse(&mut cursor);
    }

    /// TIFF parser should not panic on arbitrary data.
    #[test]
    fn tiff_parse_no_panic(data in prop::collection::vec(any::<u8>(), 0..2048)) {
        let parser = TiffParser::default();
        let mut cursor = Cursor::new(&data);
        let _ = parser.parse(&mut cursor);
    }

    /// HEIC parser should not panic on arbitrary data.
    #[test]
    fn heic_parse_no_panic(data in prop::collection::vec(any::<u8>(), 0..2048)) {
        let parser = HeicParser;
        let mut cursor = Cursor::new(&data);
        let _ = parser.parse(&mut cursor);
    }

    /// CR2 parser should not panic on arbitrary data.
    #[test]
    fn cr2_parse_no_panic(data in prop::collection::vec(any::<u8>(), 0..2048)) {
        let parser = Cr2Parser::default();
        let mut cursor = Cursor::new(&data);
        let _ = parser.parse(&mut cursor);
    }

    /// CR3 parser should not panic on arbitrary data.
    #[test]
    fn cr3_parse_no_panic(data in prop::collection::vec(any::<u8>(), 0..2048)) {
        let parser = Cr3Parser;
        let mut cursor = Cursor::new(&data);
        let _ = parser.parse(&mut cursor);
    }

    /// NEF parser should not panic on arbitrary data.
    #[test]
    fn nef_parse_no_panic(data in prop::collection::vec(any::<u8>(), 0..2048)) {
        let parser = NefParser::default();
        let mut cursor = Cursor::new(&data);
        let _ = parser.parse(&mut cursor);
    }

    /// RAF parser should not panic on arbitrary data.
    #[test]
    fn raf_parse_no_panic(data in prop::collection::vec(any::<u8>(), 0..2048)) {
        let parser = RafParser;
        let mut cursor = Cursor::new(&data);
        let _ = parser.parse(&mut cursor);
    }

    /// HDR parser should not panic on arbitrary data.
    #[test]
    fn hdr_parse_no_panic(data in prop::collection::vec(any::<u8>(), 0..2048)) {
        let parser = HdrParser;
        let mut cursor = Cursor::new(&data);
        let _ = parser.parse(&mut cursor);
    }

    /// EXR parser should not panic on arbitrary data.
    #[test]
    fn exr_parse_no_panic(data in prop::collection::vec(any::<u8>(), 0..2048)) {
        let parser = ExrParser;
        let mut cursor = Cursor::new(&data);
        let _ = parser.parse(&mut cursor);
    }
}

/// Test with magic bytes prefix to exercise deeper parsing paths.
mod with_magic {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(200))]

        /// JPEG with valid magic should not panic.
        #[test]
        fn jpeg_with_magic(tail in prop::collection::vec(any::<u8>(), 0..1024)) {
            let mut data = vec![0xFF, 0xD8, 0xFF];
            data.extend(tail);
            let parser = JpegParser;
            let mut cursor = Cursor::new(&data);
            let _ = parser.parse(&mut cursor);
        }

        /// PNG with valid magic should not panic.
        #[test]
        fn png_with_magic(tail in prop::collection::vec(any::<u8>(), 0..1024)) {
            let mut data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
            data.extend(tail);
            let parser = PngParser;
            let mut cursor = Cursor::new(&data);
            let _ = parser.parse(&mut cursor);
        }

        /// TIFF LE with valid magic should not panic.
        #[test]
        fn tiff_le_with_magic(tail in prop::collection::vec(any::<u8>(), 0..1024)) {
            let mut data = vec![0x49, 0x49, 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00];
            data.extend(tail);
            let parser = TiffParser::default();
            let mut cursor = Cursor::new(&data);
            let _ = parser.parse(&mut cursor);
        }

        /// HEIC/HEIF with valid magic should not panic.
        #[test]
        fn heic_with_magic(tail in prop::collection::vec(any::<u8>(), 0..1024)) {
            // ftyp box with heic brand
            let mut data = vec![
                0x00, 0x00, 0x00, 0x18, // size = 24
                0x66, 0x74, 0x79, 0x70, // 'ftyp'
                0x68, 0x65, 0x69, 0x63, // 'heic'
                0x00, 0x00, 0x00, 0x00, // minor version
                0x68, 0x65, 0x69, 0x63, // compatible brand
                0x6D, 0x69, 0x66, 0x31, // 'mif1'
            ];
            data.extend(tail);
            let parser = HeicParser;
            let mut cursor = Cursor::new(&data);
            let _ = parser.parse(&mut cursor);
        }
    }
}

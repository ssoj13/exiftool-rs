//! Property-based tests for core parsers.
//!
//! These tests verify that parsers handle arbitrary input without panicking.

use exiftool_core::{ByteOrder, ExifFormat, IfdReader};
use proptest::prelude::*;

proptest! {
    /// IFD parser should never panic on arbitrary byte sequences.
    #[test]
    fn ifd_parser_no_panic(data in prop::collection::vec(any::<u8>(), 0..1024)) {
        let reader = IfdReader::new(&data, ByteOrder::LittleEndian);
        // Should return Ok or Err, never panic
        let _ = reader.parse_header();
    }

    /// IFD parser handles both byte orders without panic.
    #[test]
    fn ifd_parser_both_endians(
        data in prop::collection::vec(any::<u8>(), 0..512),
        big_endian in any::<bool>()
    ) {
        let order = if big_endian {
            ByteOrder::BigEndian
        } else {
            ByteOrder::LittleEndian
        };
        let reader = IfdReader::new(&data, order);
        let _ = reader.parse_header();
    }

    /// ByteOrder::parse should handle any 2-byte input.
    #[test]
    fn byte_order_parse_no_panic(b0 in any::<u8>(), b1 in any::<u8>()) {
        let _ = ByteOrder::from_marker([b0, b1]);
    }

    /// ExifFormat::from_u16 should handle any u16 value.
    #[test]
    fn exif_format_from_u16_no_panic(val in any::<u16>()) {
        let _ = ExifFormat::from_u16(val);
    }

    /// Valid TIFF headers should parse correctly.
    #[test]
    fn valid_tiff_header_parses(offset in 8u32..0xFFFF_u32) {
        // Little-endian header
        let mut data = vec![0x49, 0x49, 0x2A, 0x00];
        data.extend_from_slice(&offset.to_le_bytes());
        // Pad to offset
        data.resize(offset as usize + 2, 0);

        let reader = IfdReader::new(&data, ByteOrder::LittleEndian);
        let result = reader.parse_header();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), offset);
    }
}

#[cfg(test)]
mod edge_cases {
    use exiftool_core::{ByteOrder, IfdReader};

    #[test]
    fn empty_data() {
        let data: &[u8] = &[];
        let reader = IfdReader::new(data, ByteOrder::LittleEndian);
        assert!(reader.parse_header().is_err());
    }

    #[test]
    fn too_short_header() {
        let data = [0x49, 0x49, 0x2A]; // Missing offset
        let reader = IfdReader::new(&data, ByteOrder::LittleEndian);
        assert!(reader.parse_header().is_err());
    }

    #[test]
    fn invalid_magic() {
        let data = [0x49, 0x49, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00];
        let reader = IfdReader::new(&data, ByteOrder::LittleEndian);
        assert!(reader.parse_header().is_err());
    }

    #[test]
    fn offset_beyond_data() {
        let data = [0x49, 0x49, 0x2A, 0x00, 0xFF, 0xFF, 0x00, 0x00];
        let reader = IfdReader::new(&data, ByteOrder::LittleEndian);
        // Header parses, but offset is invalid
        let result = reader.parse_header();
        // Parser should accept header, further reading will fail
        assert!(result.is_ok());
    }
}

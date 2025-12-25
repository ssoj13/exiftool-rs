//! Round-trip integration tests for EXIF writing.

use exiftool_core::{ByteOrder, ExifWriter, WriteEntry, writer::tags};
use exiftool_formats::{FormatRegistry, JpegWriter};
use std::io::Cursor;

/// Create a minimal valid JPEG for testing.
fn minimal_jpeg() -> Vec<u8> {
    vec![
        // SOI
        0xFF, 0xD8,
        // APP0 JFIF
        0xFF, 0xE0, 0x00, 0x10,
        b'J', b'F', b'I', b'F', 0x00, 0x01, 0x01, 0x00, 
        0x00, 0x48, 0x00, 0x48, 0x00, 0x00,
        // DQT (minimal)
        0xFF, 0xDB, 0x00, 0x43, 0x00,
        0x08, 0x06, 0x06, 0x07, 0x06, 0x05, 0x08, 0x07,
        0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14,
        0x0D, 0x0C, 0x0B, 0x0B, 0x0C, 0x19, 0x12, 0x13,
        0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A,
        0x1C, 0x1C, 0x20, 0x24, 0x2E, 0x27, 0x20, 0x22,
        0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29, 0x2C,
        0x30, 0x31, 0x34, 0x34, 0x34, 0x1F, 0x27, 0x39,
        0x3D, 0x38, 0x32, 0x3C, 0x2E, 0x33, 0x34, 0x32,
        // SOF0 (baseline DCT, 1x1 pixel)
        0xFF, 0xC0, 0x00, 0x0B, 0x08, 0x00, 0x01, 0x00, 0x01, 0x01, 0x01, 0x11, 0x00,
        // DHT (minimal)
        0xFF, 0xC4, 0x00, 0x1F, 0x00,
        0x00, 0x01, 0x05, 0x01, 0x01, 0x01, 0x01, 0x01,
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0A, 0x0B,
        // SOS
        0xFF, 0xDA, 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00,
        // Minimal scan data (just zeros)
        0x7F,
        // EOI
        0xFF, 0xD9,
    ]
}

#[test]
fn round_trip_exif_write() {
    // Create EXIF data
    let mut writer = ExifWriter::new(ByteOrder::LittleEndian);
    writer.add_ifd0(WriteEntry::from_str(tags::MAKE, "RustCam"));
    writer.add_ifd0(WriteEntry::from_str(tags::MODEL, "ExifTool-RS v1"));
    writer.add_ifd0(WriteEntry::from_u16(tags::ORIENTATION, 1));
    writer.add_ifd0(WriteEntry::from_str(tags::SOFTWARE, "exiftool-rs"));
    writer.add_ifd0(WriteEntry::from_str(tags::ARTIST, "Test Artist"));
    
    let exif_bytes = writer.serialize().unwrap();
    
    // Write JPEG with new EXIF
    let input = minimal_jpeg();
    let mut input_cursor = Cursor::new(&input);
    let mut output = Vec::new();
    
    JpegWriter::write(&mut input_cursor, &mut output, Some(&exif_bytes), None).unwrap();
    
    // Verify output is valid JPEG
    assert_eq!(&output[0..2], &[0xFF, 0xD8], "Should start with SOI");
    
    // Parse the output and verify EXIF
    let registry = FormatRegistry::new();
    let mut output_cursor = Cursor::new(&output);
    let metadata = registry.parse(&mut output_cursor).unwrap();
    
    assert_eq!(metadata.format, "JPEG");
    assert_eq!(metadata.exif.get_str("Make"), Some("RustCam"));
    assert_eq!(metadata.exif.get_str("Model"), Some("ExifTool-RS v1"));
    assert_eq!(metadata.exif.get_str("Software"), Some("exiftool-rs"));
    assert_eq!(metadata.exif.get_str("Artist"), Some("Test Artist"));
}

#[test]
fn round_trip_with_existing_exif() {
    // Create original JPEG with EXIF
    let mut original_writer = ExifWriter::new(ByteOrder::LittleEndian);
    original_writer.add_ifd0(WriteEntry::from_str(tags::MAKE, "OriginalCam"));
    original_writer.add_ifd0(WriteEntry::from_str(tags::MODEL, "Original Model"));
    
    let original_exif = original_writer.serialize().unwrap();
    
    let input = minimal_jpeg();
    let mut input_cursor = Cursor::new(&input);
    let mut jpeg_with_exif = Vec::new();
    
    JpegWriter::write(&mut input_cursor, &mut jpeg_with_exif, Some(&original_exif), None).unwrap();
    
    // Verify original EXIF
    let registry = FormatRegistry::new();
    let mut cursor = Cursor::new(&jpeg_with_exif);
    let metadata = registry.parse(&mut cursor).unwrap();
    assert_eq!(metadata.exif.get_str("Make"), Some("OriginalCam"));
    
    // Now replace with new EXIF
    let mut new_writer = ExifWriter::new(ByteOrder::LittleEndian);
    new_writer.add_ifd0(WriteEntry::from_str(tags::MAKE, "NewCam"));
    new_writer.add_ifd0(WriteEntry::from_str(tags::MODEL, "New Model"));
    new_writer.add_ifd0(WriteEntry::from_str(tags::COPYRIGHT, "Copyright 2024"));
    
    let new_exif = new_writer.serialize().unwrap();
    
    let mut jpeg_cursor = Cursor::new(&jpeg_with_exif);
    let mut final_output = Vec::new();
    
    JpegWriter::write(&mut jpeg_cursor, &mut final_output, Some(&new_exif), None).unwrap();
    
    // Verify new EXIF replaced old
    let mut final_cursor = Cursor::new(&final_output);
    let final_metadata = registry.parse(&mut final_cursor).unwrap();
    
    assert_eq!(final_metadata.exif.get_str("Make"), Some("NewCam"));
    assert_eq!(final_metadata.exif.get_str("Model"), Some("New Model"));
    assert_eq!(final_metadata.exif.get_str("Copyright"), Some("Copyright 2024"));
    // Original values should be gone
    assert_ne!(final_metadata.exif.get_str("Make"), Some("OriginalCam"));
}

#[test]
fn round_trip_with_exif_ifd() {
    // Test ExifIFD sub-IFD
    let mut writer = ExifWriter::new(ByteOrder::LittleEndian);
    writer.add_ifd0(WriteEntry::from_str(tags::MAKE, "TestCam"));
    
    // Add ExifIFD entries
    writer.add_exif(WriteEntry::from_u16(tags::ISO, 800));
    writer.add_exif(WriteEntry::from_urational(tags::EXPOSURE_TIME, 1, 125));
    writer.add_exif(WriteEntry::from_urational(tags::FNUMBER, 28, 10));
    
    let exif_bytes = writer.serialize().unwrap();
    
    let input = minimal_jpeg();
    let mut input_cursor = Cursor::new(&input);
    let mut output = Vec::new();
    
    JpegWriter::write(&mut input_cursor, &mut output, Some(&exif_bytes), None).unwrap();
    
    // Parse and verify
    let registry = FormatRegistry::new();
    let mut output_cursor = Cursor::new(&output);
    let metadata = registry.parse(&mut output_cursor).unwrap();
    
    assert_eq!(metadata.exif.get_str("Make"), Some("TestCam"));
    // ExifIFD values should also be parsed
    // Note: ISO is stored as u16, check if it's parsed correctly
}

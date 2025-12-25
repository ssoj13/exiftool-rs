//! Unified tag lookup using generated tables from ExifTool.
//!
//! Provides dynamic tag name resolution for all EXIF, GPS, and IFD tags
//! using the auto-generated tables (~2500+ tags).

use exiftool_tags::generated::exif::EXIF_MAIN;
use exiftool_tags::generated::gps::GPS_MAIN;

/// Lookup any EXIF/IFD tag by ID from generated table.
/// Covers ~2500 tags including TIFF, EXIF, DNG, and more.
pub fn lookup_exif(tag: u16) -> Option<&'static str> {
    EXIF_MAIN.get(&tag).map(|def| def.name)
}

/// Lookup GPS tag by ID from generated table.
pub fn lookup_gps(tag: u16) -> Option<&'static str> {
    GPS_MAIN.get(&tag).map(|def| def.name)
}

/// IFD0 tag lookup - handles special pointer tags.
pub fn lookup_ifd0(tag: u16) -> Option<&'static str> {
    match tag {
        0x8769 => Some("ExifOffset"),  // EXIF sub-IFD pointer
        0x8825 => Some("GPSInfo"),     // GPS sub-IFD pointer
        0x014A => Some("SubIFDs"),     // Sub-IFD pointers
        0xA005 => Some("InteropOffset"), // Interoperability pointer
        _ => lookup_exif(tag),
    }
}

/// EXIF sub-IFD tag lookup.
#[inline]
pub fn lookup_exif_subifd(tag: u16) -> Option<&'static str> {
    lookup_exif(tag)
}

/// Interoperability tag lookup (uses same table as EXIF).
#[inline]
pub fn lookup_interop(tag: u16) -> Option<&'static str> {
    lookup_exif(tag)
}

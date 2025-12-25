//! MakerNotes parsers for vendor-specific EXIF data.
//!
//! Each camera vendor embeds proprietary metadata in EXIF tag 0x927C.
//! This module provides modular parsers for each vendor that extract
//! nested sub-IFD structures into hierarchical Attrs.
//!
//! # Architecture
//!
//! Each vendor parser implements [`VendorParser`] trait:
//! - Detects vendor from Make string
//! - Parses vendor-specific header format
//! - Extracts main IFD and known sub-IFDs
//! - Returns nested [`Attrs`] with vendor prefix
//!
//! # Supported Vendors
//!
//! - Canon: AFInfo, ShotInfo, CameraSettings, ColorBalance, etc.
//! - Nikon: AFInfo, VRInfo, PictureControl, WorldTime, etc.
//! - Sony: AFInfo, Tag9405, Tag2010, etc.
//! - Fujifilm: Main tags + AFCSettings
//! - Olympus: CameraSettings, Equipment, FocusInfo, etc.
//! - Panasonic: Main tags + FaceDetection
//! - Pentax: Main tags + LensInfo
//! - Samsung: Main tags
//! - Apple: Main tags

mod canon;
mod nikon;
mod sony;
mod fujifilm;
mod olympus;
mod panasonic;
mod pentax;
mod samsung;
mod apple;

pub use canon::CanonParser;
pub use nikon::NikonParser;
pub use sony::SonyParser;
pub use fujifilm::FujifilmParser;
pub use olympus::OlympusParser;
pub use panasonic::PanasonicParser;
pub use pentax::PentaxParser;
pub use samsung::SamsungParser;
pub use apple::AppleParser;

use exiftool_attrs::{Attrs, AttrValue};
use exiftool_core::ByteOrder;

/// Detected camera vendor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vendor {
    Canon,
    Nikon,
    Sony,
    Fujifilm,
    Olympus,
    Panasonic,
    Pentax,
    Samsung,
    Apple,
    Unknown,
}

impl Vendor {
    /// Detect vendor from EXIF Make string.
    pub fn from_make(make: &str) -> Self {
        let make_lower = make.to_lowercase();
        if make_lower.contains("canon") {
            Vendor::Canon
        } else if make_lower.contains("nikon") {
            Vendor::Nikon
        } else if make_lower.contains("sony") {
            Vendor::Sony
        } else if make_lower.contains("fuji") {
            Vendor::Fujifilm
        } else if make_lower.contains("olympus") || make_lower.contains("om digital") {
            Vendor::Olympus
        } else if make_lower.contains("panasonic") || make_lower.contains("leica") {
            Vendor::Panasonic
        } else if make_lower.contains("pentax") || make_lower.contains("ricoh") {
            Vendor::Pentax
        } else if make_lower.contains("samsung") {
            Vendor::Samsung
        } else if make_lower.contains("apple") {
            Vendor::Apple
        } else {
            Vendor::Unknown
        }
    }

    /// Get vendor name as string.
    pub fn name(&self) -> &'static str {
        match self {
            Vendor::Canon => "Canon",
            Vendor::Nikon => "Nikon",
            Vendor::Sony => "Sony",
            Vendor::Fujifilm => "Fujifilm",
            Vendor::Olympus => "Olympus",
            Vendor::Panasonic => "Panasonic",
            Vendor::Pentax => "Pentax",
            Vendor::Samsung => "Samsung",
            Vendor::Apple => "Apple",
            Vendor::Unknown => "Unknown",
        }
    }
}

/// Trait for vendor-specific MakerNotes parsing.
///
/// Implementations handle:
/// 1. Header detection and parsing
/// 2. Byte order determination  
/// 3. Main IFD extraction
/// 4. Sub-IFD extraction (vendor-specific)
pub trait VendorParser: Send + Sync {
    /// Get the vendor this parser handles.
    fn vendor(&self) -> Vendor;

    /// Parse MakerNotes data into nested Attrs.
    ///
    /// # Arguments
    /// * `data` - Raw MakerNotes bytes (tag 0x927C value)
    /// * `parent_byte_order` - Byte order from parent TIFF
    ///
    /// # Returns
    /// Nested Attrs with vendor-prefixed groups, e.g.:
    /// ```text
    /// Canon:
    ///   AFInfo:
    ///     AFPointsUsed: 15
    ///   ShotInfo:
    ///     ShutterCount: 12345
    /// ```
    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs>;
}

/// Parse MakerNotes with auto-detected vendor.
///
/// This is the main entry point for MakerNotes parsing.
pub fn parse(data: &[u8], vendor: Vendor, parent_byte_order: ByteOrder) -> Option<Attrs> {
    if data.is_empty() {
        return None;
    }

    let parser: &dyn VendorParser = match vendor {
        Vendor::Canon => &CanonParser,
        Vendor::Nikon => &NikonParser,
        Vendor::Sony => &SonyParser,
        Vendor::Fujifilm => &FujifilmParser,
        Vendor::Olympus => &OlympusParser,
        Vendor::Panasonic => &PanasonicParser,
        Vendor::Pentax => &PentaxParser,
        Vendor::Samsung => &SamsungParser,
        Vendor::Apple => &AppleParser,
        Vendor::Unknown => return None,
    };

    // Validate parser matches expected vendor
    debug_assert_eq!(parser.vendor(), vendor, "Parser vendor mismatch");

    let mut attrs = parser.parse(data, parent_byte_order)?;
    
    // Add vendor identification to result
    attrs.set("Vendor", AttrValue::Str(parser.vendor().name().to_string()));
    
    Some(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vendor_detection() {
        assert_eq!(Vendor::from_make("Canon"), Vendor::Canon);
        assert_eq!(Vendor::from_make("NIKON CORPORATION"), Vendor::Nikon);
        assert_eq!(Vendor::from_make("SONY"), Vendor::Sony);
        assert_eq!(Vendor::from_make("FUJIFILM"), Vendor::Fujifilm);
        assert_eq!(Vendor::from_make("OLYMPUS CORPORATION"), Vendor::Olympus);
        assert_eq!(Vendor::from_make("OM Digital Solutions"), Vendor::Olympus);
        assert_eq!(Vendor::from_make("Panasonic"), Vendor::Panasonic);
        assert_eq!(Vendor::from_make("LEICA"), Vendor::Panasonic);
        assert_eq!(Vendor::from_make("PENTAX"), Vendor::Pentax);
        assert_eq!(Vendor::from_make("RICOH IMAGING"), Vendor::Pentax);
        assert_eq!(Vendor::from_make("SAMSUNG"), Vendor::Samsung);
        assert_eq!(Vendor::from_make("Apple"), Vendor::Apple);
        assert_eq!(Vendor::from_make("Unknown Brand"), Vendor::Unknown);
    }
}

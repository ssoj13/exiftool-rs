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
//! - DJI: Drone telemetry (SpeedX/Y/Z, Pitch/Yaw/Roll, CameraPitch/Yaw/Roll)
//! - GoPro: GPMF format (Accelerometer, Gyroscope, GPS, Temperature, etc.)

mod apple;
mod canon;
mod casio;
mod dji;
mod fujifilm;
mod google;
mod gopro;
mod hasselblad;
mod huawei;
mod kodak;
mod minolta;
mod nikon;
mod olympus;
mod panasonic;
mod pentax;
mod phaseone;
mod ricoh;
mod samsung;
mod sigma;
mod sony;
mod xiaomi;

pub use apple::AppleParser;
pub use canon::CanonParser;
pub use casio::CasioParser;
pub use dji::DjiParser;
pub use fujifilm::FujifilmParser;
pub use google::GoogleParser;
pub use gopro::GoProParser;
pub use hasselblad::HasselbladParser;
pub use huawei::HuaweiParser;
pub use kodak::KodakParser;
pub use minolta::MinoltaParser;
pub use nikon::NikonParser;
pub use olympus::OlympusParser;
pub use panasonic::PanasonicParser;
pub use pentax::PentaxParser;
pub use phaseone::PhaseOneParser;
pub use ricoh::RicohParser;
pub use samsung::SamsungParser;
pub use sigma::SigmaParser;
pub use sony::SonyParser;
pub use xiaomi::XiaomiParser;

use exiftool_attrs::{Attrs, AttrValue};
use exiftool_core::{ByteOrder, IfdEntry, IfdReader};

/// Parse IFD entries from raw data.
///
/// Common helper for vendor parsers that read IFD structures.
/// - `data`: Raw bytes containing IFD
/// - `byte_order`: Endianness for parsing
/// - `ifd_offset`: Offset to IFD within data (usually 0)
///
/// Returns None if data is too small or IFD parsing fails.
pub fn parse_ifd_entries(data: &[u8], byte_order: ByteOrder, ifd_offset: u32) -> Option<Vec<IfdEntry>> {
    if data.len() < 2 {
        return None;
    }
    let reader = IfdReader::new(data, byte_order, 0);
    let (entries, _) = reader.read_ifd(ifd_offset).ok()?;
    Some(entries)
}

/// Detected camera vendor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vendor {
    Apple,
    Canon,
    Casio,
    Dji,
    Fujifilm,
    Google,
    GoPro,
    Hasselblad,
    Huawei,
    Kodak,
    Minolta,
    Nikon,
    Olympus,
    Panasonic,
    Pentax,
    PhaseOne,
    Ricoh,
    Samsung,
    Sigma,
    Sony,
    Xiaomi,
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
        } else if make_lower.contains("pentax") {
            Vendor::Pentax
        } else if make_lower.contains("ricoh") {
            // Ricoh cameras (not Pentax-based)
            Vendor::Ricoh
        } else if make_lower.contains("samsung") {
            Vendor::Samsung
        } else if make_lower.contains("apple") {
            Vendor::Apple
        } else if make_lower.contains("dji") {
            Vendor::Dji
        } else if make_lower.contains("gopro") {
            Vendor::GoPro
        } else if make_lower.contains("minolta") || make_lower.contains("konica") {
            Vendor::Minolta
        } else if make_lower.contains("sigma") || make_lower.contains("foveon") {
            Vendor::Sigma
        } else if make_lower.contains("kodak") {
            Vendor::Kodak
        } else if make_lower.contains("casio") {
            Vendor::Casio
        } else if make_lower.contains("hasselblad") {
            Vendor::Hasselblad
        } else if make_lower.contains("phase one") || make_lower.contains("phaseone") || make_lower.contains("leaf") || make_lower.contains("mamiya") {
            Vendor::PhaseOne
        } else if make_lower.contains("huawei") || make_lower.contains("honor") {
            Vendor::Huawei
        } else if make_lower.contains("xiaomi") || make_lower.contains("redmi") || make_lower.contains("poco") {
            Vendor::Xiaomi
        } else if make_lower.contains("google") {
            Vendor::Google
        } else {
            Vendor::Unknown
        }
    }

    /// Get vendor name as string.
    pub fn name(&self) -> &'static str {
        match self {
            Vendor::Apple => "Apple",
            Vendor::Canon => "Canon",
            Vendor::Casio => "Casio",
            Vendor::Dji => "DJI",
            Vendor::Fujifilm => "Fujifilm",
            Vendor::Google => "Google",
            Vendor::GoPro => "GoPro",
            Vendor::Hasselblad => "Hasselblad",
            Vendor::Huawei => "Huawei",
            Vendor::Kodak => "Kodak",
            Vendor::Minolta => "Minolta",
            Vendor::Nikon => "Nikon",
            Vendor::Olympus => "Olympus",
            Vendor::Panasonic => "Panasonic",
            Vendor::Pentax => "Pentax",
            Vendor::PhaseOne => "Phase One",
            Vendor::Ricoh => "Ricoh",
            Vendor::Samsung => "Samsung",
            Vendor::Sigma => "Sigma",
            Vendor::Sony => "Sony",
            Vendor::Xiaomi => "Xiaomi",
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
        Vendor::Apple => &AppleParser,
        Vendor::Canon => &CanonParser,
        Vendor::Casio => &CasioParser,
        Vendor::Dji => &DjiParser,
        Vendor::Fujifilm => &FujifilmParser,
        Vendor::Google => &GoogleParser,
        Vendor::GoPro => &GoProParser,
        Vendor::Hasselblad => &HasselbladParser,
        Vendor::Huawei => &HuaweiParser,
        Vendor::Kodak => &KodakParser,
        Vendor::Minolta => &MinoltaParser,
        Vendor::Nikon => &NikonParser,
        Vendor::Olympus => &OlympusParser,
        Vendor::Panasonic => &PanasonicParser,
        Vendor::Pentax => &PentaxParser,
        Vendor::PhaseOne => &PhaseOneParser,
        Vendor::Ricoh => &RicohParser,
        Vendor::Samsung => &SamsungParser,
        Vendor::Sigma => &SigmaParser,
        Vendor::Sony => &SonyParser,
        Vendor::Xiaomi => &XiaomiParser,
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
        assert_eq!(Vendor::from_make("RICOH IMAGING"), Vendor::Ricoh);
        assert_eq!(Vendor::from_make("MINOLTA"), Vendor::Minolta);
        assert_eq!(Vendor::from_make("KONICA MINOLTA"), Vendor::Minolta);
        assert_eq!(Vendor::from_make("SIGMA"), Vendor::Sigma);
        assert_eq!(Vendor::from_make("KODAK"), Vendor::Kodak);
        assert_eq!(Vendor::from_make("CASIO"), Vendor::Casio);
        assert_eq!(Vendor::from_make("HASSELBLAD"), Vendor::Hasselblad);
        assert_eq!(Vendor::from_make("Phase One"), Vendor::PhaseOne);
        assert_eq!(Vendor::from_make("Leaf"), Vendor::PhaseOne);
        assert_eq!(Vendor::from_make("HUAWEI"), Vendor::Huawei);
        assert_eq!(Vendor::from_make("HONOR"), Vendor::Huawei);
        assert_eq!(Vendor::from_make("Xiaomi"), Vendor::Xiaomi);
        assert_eq!(Vendor::from_make("Redmi"), Vendor::Xiaomi);
        assert_eq!(Vendor::from_make("Google"), Vendor::Google);
        assert_eq!(Vendor::from_make("SAMSUNG"), Vendor::Samsung);
        assert_eq!(Vendor::from_make("Apple"), Vendor::Apple);
        assert_eq!(Vendor::from_make("DJI"), Vendor::Dji);
        assert_eq!(Vendor::from_make("GoPro"), Vendor::GoPro);
        assert_eq!(Vendor::from_make("Unknown Brand"), Vendor::Unknown);
    }
}

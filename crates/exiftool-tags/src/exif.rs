//! Standard EXIF tag definitions.
//!
//! Based on EXIF 2.32 / TIFF 6.0 specifications.
//!
//! STUB: Contains basic tag tables (IFD0, EXIF, GPS).
//! Need to implement:
//! - Complete IFD0/IFD1 tags (~50 more tags)
//! - Complete EXIF tags (~100 more tags)
//! - Complete GPS tags (~30 more tags)
//! - Interoperability IFD tags
//! - TIFF/EP extension tags
//! - Value interpretation (e.g., Orientation 1=Normal, Flash bits)
//! - Codegen from ExifTool Perl source via xtask

use exiftool_core::ExifFormat;
use phf::phf_map;

/// Tag group classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagGroup {
    /// IFD0 - main image tags.
    Ifd0,
    /// IFD1 - thumbnail tags.
    Ifd1,
    /// EXIF sub-IFD.
    ExifIfd,
    /// GPS sub-IFD.
    GpsIfd,
    /// Interoperability sub-IFD.
    InteropIfd,
    /// MakerNotes (vendor-specific).
    MakerNotes,
}

/// Tag definition.
#[derive(Debug, Clone)]
pub struct TagDef {
    /// Tag name (e.g., "Make", "Model").
    pub name: &'static str,
    /// Human-readable description.
    pub description: Option<&'static str>,
    /// Expected data format.
    pub format: Option<ExifFormat>,
    /// Tag group.
    pub group: TagGroup,
    /// Can be written.
    pub writable: bool,
}

impl TagDef {
    /// Create new tag definition.
    pub const fn new(name: &'static str, group: TagGroup) -> Self {
        Self {
            name,
            description: None,
            format: None,
            group,
            writable: false,
        }
    }

    /// Create with format.
    pub const fn with_format(name: &'static str, group: TagGroup, format: ExifFormat) -> Self {
        Self {
            name,
            description: None,
            format: Some(format),
            group,
            writable: false,
        }
    }

    /// Create writable tag.
    pub const fn writable(name: &'static str, group: TagGroup, format: ExifFormat) -> Self {
        Self {
            name,
            description: None,
            format: Some(format),
            group,
            writable: true,
        }
    }
}

/// IFD0 (main image) tags.
pub static IFD0_TAGS: phf::Map<u16, TagDef> = phf_map! {
    0x010E_u16 => TagDef::new("ImageDescription", TagGroup::Ifd0),
    0x010F_u16 => TagDef::new("Make", TagGroup::Ifd0),
    0x0110_u16 => TagDef::new("Model", TagGroup::Ifd0),
    0x0112_u16 => TagDef::new("Orientation", TagGroup::Ifd0),
    0x011A_u16 => TagDef::new("XResolution", TagGroup::Ifd0),
    0x011B_u16 => TagDef::new("YResolution", TagGroup::Ifd0),
    0x0128_u16 => TagDef::new("ResolutionUnit", TagGroup::Ifd0),
    0x0131_u16 => TagDef::new("Software", TagGroup::Ifd0),
    0x0132_u16 => TagDef::new("DateTime", TagGroup::Ifd0),
    0x013B_u16 => TagDef::new("Artist", TagGroup::Ifd0),
    0x013E_u16 => TagDef::new("WhitePoint", TagGroup::Ifd0),
    0x013F_u16 => TagDef::new("PrimaryChromaticities", TagGroup::Ifd0),
    0x0211_u16 => TagDef::new("YCbCrCoefficients", TagGroup::Ifd0),
    0x0213_u16 => TagDef::new("YCbCrPositioning", TagGroup::Ifd0),
    0x0214_u16 => TagDef::new("ReferenceBlackWhite", TagGroup::Ifd0),
    0x8298_u16 => TagDef::new("Copyright", TagGroup::Ifd0),
    0x8769_u16 => TagDef::new("ExifOffset", TagGroup::Ifd0),
    0x8825_u16 => TagDef::new("GPSInfo", TagGroup::Ifd0),
};

/// EXIF sub-IFD tags.
pub static EXIF_TAGS: phf::Map<u16, TagDef> = phf_map! {
    0x829A_u16 => TagDef::new("ExposureTime", TagGroup::ExifIfd),
    0x829D_u16 => TagDef::new("FNumber", TagGroup::ExifIfd),
    0x8822_u16 => TagDef::new("ExposureProgram", TagGroup::ExifIfd),
    0x8824_u16 => TagDef::new("SpectralSensitivity", TagGroup::ExifIfd),
    0x8827_u16 => TagDef::new("ISO", TagGroup::ExifIfd),
    0x8828_u16 => TagDef::new("OECF", TagGroup::ExifIfd),
    0x8830_u16 => TagDef::new("SensitivityType", TagGroup::ExifIfd),
    0x9000_u16 => TagDef::new("ExifVersion", TagGroup::ExifIfd),
    0x9003_u16 => TagDef::new("DateTimeOriginal", TagGroup::ExifIfd),
    0x9004_u16 => TagDef::new("CreateDate", TagGroup::ExifIfd),
    0x9010_u16 => TagDef::new("OffsetTime", TagGroup::ExifIfd),
    0x9011_u16 => TagDef::new("OffsetTimeOriginal", TagGroup::ExifIfd),
    0x9012_u16 => TagDef::new("OffsetTimeDigitized", TagGroup::ExifIfd),
    0x9101_u16 => TagDef::new("ComponentsConfiguration", TagGroup::ExifIfd),
    0x9102_u16 => TagDef::new("CompressedBitsPerPixel", TagGroup::ExifIfd),
    0x9201_u16 => TagDef::new("ShutterSpeedValue", TagGroup::ExifIfd),
    0x9202_u16 => TagDef::new("ApertureValue", TagGroup::ExifIfd),
    0x9203_u16 => TagDef::new("BrightnessValue", TagGroup::ExifIfd),
    0x9204_u16 => TagDef::new("ExposureCompensation", TagGroup::ExifIfd),
    0x9205_u16 => TagDef::new("MaxApertureValue", TagGroup::ExifIfd),
    0x9206_u16 => TagDef::new("SubjectDistance", TagGroup::ExifIfd),
    0x9207_u16 => TagDef::new("MeteringMode", TagGroup::ExifIfd),
    0x9208_u16 => TagDef::new("LightSource", TagGroup::ExifIfd),
    0x9209_u16 => TagDef::new("Flash", TagGroup::ExifIfd),
    0x920A_u16 => TagDef::new("FocalLength", TagGroup::ExifIfd),
    0x9214_u16 => TagDef::new("SubjectArea", TagGroup::ExifIfd),
    0x927C_u16 => TagDef::new("MakerNote", TagGroup::ExifIfd),
    0x9286_u16 => TagDef::new("UserComment", TagGroup::ExifIfd),
    0x9290_u16 => TagDef::new("SubSecTime", TagGroup::ExifIfd),
    0x9291_u16 => TagDef::new("SubSecTimeOriginal", TagGroup::ExifIfd),
    0x9292_u16 => TagDef::new("SubSecTimeDigitized", TagGroup::ExifIfd),
    0xA000_u16 => TagDef::new("FlashpixVersion", TagGroup::ExifIfd),
    0xA001_u16 => TagDef::new("ColorSpace", TagGroup::ExifIfd),
    0xA002_u16 => TagDef::new("ExifImageWidth", TagGroup::ExifIfd),
    0xA003_u16 => TagDef::new("ExifImageHeight", TagGroup::ExifIfd),
    0xA004_u16 => TagDef::new("RelatedSoundFile", TagGroup::ExifIfd),
    0xA005_u16 => TagDef::new("InteropOffset", TagGroup::ExifIfd),
    0xA20E_u16 => TagDef::new("FocalPlaneXResolution", TagGroup::ExifIfd),
    0xA20F_u16 => TagDef::new("FocalPlaneYResolution", TagGroup::ExifIfd),
    0xA210_u16 => TagDef::new("FocalPlaneResolutionUnit", TagGroup::ExifIfd),
    0xA215_u16 => TagDef::new("ExposureIndex", TagGroup::ExifIfd),
    0xA217_u16 => TagDef::new("SensingMethod", TagGroup::ExifIfd),
    0xA300_u16 => TagDef::new("FileSource", TagGroup::ExifIfd),
    0xA301_u16 => TagDef::new("SceneType", TagGroup::ExifIfd),
    0xA302_u16 => TagDef::new("CFAPattern", TagGroup::ExifIfd),
    0xA401_u16 => TagDef::new("CustomRendered", TagGroup::ExifIfd),
    0xA402_u16 => TagDef::new("ExposureMode", TagGroup::ExifIfd),
    0xA403_u16 => TagDef::new("WhiteBalance", TagGroup::ExifIfd),
    0xA404_u16 => TagDef::new("DigitalZoomRatio", TagGroup::ExifIfd),
    0xA405_u16 => TagDef::new("FocalLengthIn35mmFormat", TagGroup::ExifIfd),
    0xA406_u16 => TagDef::new("SceneCaptureType", TagGroup::ExifIfd),
    0xA407_u16 => TagDef::new("GainControl", TagGroup::ExifIfd),
    0xA408_u16 => TagDef::new("Contrast", TagGroup::ExifIfd),
    0xA409_u16 => TagDef::new("Saturation", TagGroup::ExifIfd),
    0xA40A_u16 => TagDef::new("Sharpness", TagGroup::ExifIfd),
    0xA40C_u16 => TagDef::new("SubjectDistanceRange", TagGroup::ExifIfd),
    0xA420_u16 => TagDef::new("ImageUniqueID", TagGroup::ExifIfd),
    0xA430_u16 => TagDef::new("OwnerName", TagGroup::ExifIfd),
    0xA431_u16 => TagDef::new("SerialNumber", TagGroup::ExifIfd),
    0xA432_u16 => TagDef::new("LensInfo", TagGroup::ExifIfd),
    0xA433_u16 => TagDef::new("LensMake", TagGroup::ExifIfd),
    0xA434_u16 => TagDef::new("LensModel", TagGroup::ExifIfd),
    0xA435_u16 => TagDef::new("LensSerialNumber", TagGroup::ExifIfd),
};

/// GPS sub-IFD tags.
pub static GPS_TAGS: phf::Map<u16, TagDef> = phf_map! {
    0x0000_u16 => TagDef::new("GPSVersionID", TagGroup::GpsIfd),
    0x0001_u16 => TagDef::new("GPSLatitudeRef", TagGroup::GpsIfd),
    0x0002_u16 => TagDef::new("GPSLatitude", TagGroup::GpsIfd),
    0x0003_u16 => TagDef::new("GPSLongitudeRef", TagGroup::GpsIfd),
    0x0004_u16 => TagDef::new("GPSLongitude", TagGroup::GpsIfd),
    0x0005_u16 => TagDef::new("GPSAltitudeRef", TagGroup::GpsIfd),
    0x0006_u16 => TagDef::new("GPSAltitude", TagGroup::GpsIfd),
    0x0007_u16 => TagDef::new("GPSTimeStamp", TagGroup::GpsIfd),
    0x0008_u16 => TagDef::new("GPSSatellites", TagGroup::GpsIfd),
    0x0009_u16 => TagDef::new("GPSStatus", TagGroup::GpsIfd),
    0x000A_u16 => TagDef::new("GPSMeasureMode", TagGroup::GpsIfd),
    0x000B_u16 => TagDef::new("GPSDOP", TagGroup::GpsIfd),
    0x000C_u16 => TagDef::new("GPSSpeedRef", TagGroup::GpsIfd),
    0x000D_u16 => TagDef::new("GPSSpeed", TagGroup::GpsIfd),
    0x000E_u16 => TagDef::new("GPSTrackRef", TagGroup::GpsIfd),
    0x000F_u16 => TagDef::new("GPSTrack", TagGroup::GpsIfd),
    0x0010_u16 => TagDef::new("GPSImgDirectionRef", TagGroup::GpsIfd),
    0x0011_u16 => TagDef::new("GPSImgDirection", TagGroup::GpsIfd),
    0x0012_u16 => TagDef::new("GPSMapDatum", TagGroup::GpsIfd),
    0x0013_u16 => TagDef::new("GPSDestLatitudeRef", TagGroup::GpsIfd),
    0x0014_u16 => TagDef::new("GPSDestLatitude", TagGroup::GpsIfd),
    0x0015_u16 => TagDef::new("GPSDestLongitudeRef", TagGroup::GpsIfd),
    0x0016_u16 => TagDef::new("GPSDestLongitude", TagGroup::GpsIfd),
    0x0017_u16 => TagDef::new("GPSDestBearingRef", TagGroup::GpsIfd),
    0x0018_u16 => TagDef::new("GPSDestBearing", TagGroup::GpsIfd),
    0x0019_u16 => TagDef::new("GPSDestDistanceRef", TagGroup::GpsIfd),
    0x001A_u16 => TagDef::new("GPSDestDistance", TagGroup::GpsIfd),
    0x001B_u16 => TagDef::new("GPSProcessingMethod", TagGroup::GpsIfd),
    0x001C_u16 => TagDef::new("GPSAreaInformation", TagGroup::GpsIfd),
    0x001D_u16 => TagDef::new("GPSDateStamp", TagGroup::GpsIfd),
    0x001E_u16 => TagDef::new("GPSDifferential", TagGroup::GpsIfd),
    0x001F_u16 => TagDef::new("GPSHPositioningError", TagGroup::GpsIfd),
};

/// Lookup tag name by ID in any table.
#[allow(dead_code)] // Public API for future use
pub fn lookup_tag(tag_id: u16, group: TagGroup) -> Option<&'static TagDef> {
    match group {
        TagGroup::Ifd0 | TagGroup::Ifd1 => IFD0_TAGS.get(&tag_id),
        TagGroup::ExifIfd => EXIF_TAGS.get(&tag_id),
        TagGroup::GpsIfd => GPS_TAGS.get(&tag_id),
        _ => None,
    }
}

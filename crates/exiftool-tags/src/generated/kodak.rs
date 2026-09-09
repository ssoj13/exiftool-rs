//! Kodak MakerNotes tag definitions.
//! Auto-generated from ExifTool - DO NOT EDIT

#![allow(dead_code)]

/// Tag definition with name and optional value mappings.
#[derive(Debug, Clone)]
pub struct TagDef {
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}

/// Bit-sliced ProcessBinaryData tag (ExifTool `0.1` index + Mask).
#[derive(Debug, Clone)]
pub struct MaskDef {
    pub index: u16,
    pub mask: u32,
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}

/// ProcessBinaryData integer index (ExifTool FORMAT, default int8u).
#[derive(Debug, Clone, Copy)]
pub struct BinDef {
    pub index: u16,
    pub width: u8,
    pub signed: bool,
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}

/// Kodak::Main tags
pub static KODAK_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "KodakModel", values: None },
    10u16 => TagDef { name: "BurstMode", values: Some(KODAK_MAIN_BURSTMODE_VALUES) },
    100u16 => TagDef { name: "DateTimeStamp", values: None },
    102u16 => TagDef { name: "ColorMode", values: Some(KODAK_MAIN_COLORMODE_VALUES) },
    104u16 => TagDef { name: "DigitalZoom", values: None },
    107u16 => TagDef { name: "Sharpness", values: Some(KODAK_MAIN_SHARPNESS_VALUES) },
    12u16 => TagDef { name: "KodakImageWidth", values: None },
    14u16 => TagDef { name: "KodakImageHeight", values: None },
    16u16 => TagDef { name: "YearCreated", values: None },
    18u16 => TagDef { name: "MonthDayCreated", values: None },
    20u16 => TagDef { name: "TimeCreated", values: None },
    24u16 => TagDef { name: "BurstMode2", values: None },
    27u16 => TagDef { name: "ShutterMode", values: Some(KODAK_MAIN_SHUTTERMODE_VALUES) },
    28u16 => TagDef { name: "MeteringMode", values: Some(KODAK_MAIN_METERINGMODE_VALUES) },
    30u16 => TagDef { name: "FNumber", values: None },
    32u16 => TagDef { name: "ExposureTime", values: None },
    36u16 => TagDef { name: "ExposureCompensation", values: None },
    38u16 => TagDef { name: "VariousModes", values: None },
    40u16 => TagDef { name: "Distance1", values: None },
    44u16 => TagDef { name: "Distance2", values: None },
    48u16 => TagDef { name: "Distance3", values: None },
    52u16 => TagDef { name: "Distance4", values: None },
    56u16 => TagDef { name: "FocusMode", values: Some(KODAK_MAIN_FOCUSMODE_VALUES) },
    58u16 => TagDef { name: "VariousModes2", values: None },
    60u16 => TagDef { name: "PanoramaMode", values: None },
    62u16 => TagDef { name: "SubjectDistance", values: None },
    64u16 => TagDef { name: "WhiteBalance", values: Some(KODAK_MAIN_WHITEBALANCE_VALUES) },
    9u16 => TagDef { name: "Quality", values: Some(KODAK_MAIN_QUALITY_VALUES) },
    92u16 => TagDef { name: "FlashMode", values: Some(KODAK_MAIN_FLASHMODE_VALUES) },
    93u16 => TagDef { name: "FlashFired", values: Some(KODAK_MAIN_FLASHFIRED_VALUES) },
    94u16 => TagDef { name: "ISOSetting", values: None },
    96u16 => TagDef { name: "ISO", values: None },
    98u16 => TagDef { name: "TotalZoom", values: None },
};

/// Kodak::Main ProcessBinaryData integers (table FORMAT or int8u)
pub static KODAK_MAIN_BIN: &[BinDef] = &[
    BinDef { index: 10, width: 1, signed: false, name: "BurstMode", values: Some(KODAK_MAIN_BURSTMODE_VALUES) },
    BinDef { index: 100, width: 2, signed: false, name: "DateTimeStamp", values: None },
    BinDef { index: 102, width: 2, signed: false, name: "ColorMode", values: Some(KODAK_MAIN_COLORMODE_VALUES) },
    BinDef { index: 104, width: 2, signed: false, name: "DigitalZoom", values: None },
    BinDef { index: 107, width: 1, signed: true, name: "Sharpness", values: Some(KODAK_MAIN_SHARPNESS_VALUES) },
    BinDef { index: 12, width: 2, signed: false, name: "KodakImageWidth", values: None },
    BinDef { index: 14, width: 2, signed: false, name: "KodakImageHeight", values: None },
    BinDef { index: 16, width: 2, signed: false, name: "YearCreated", values: None },
    BinDef { index: 24, width: 2, signed: false, name: "BurstMode2", values: None },
    BinDef { index: 27, width: 1, signed: false, name: "ShutterMode", values: Some(KODAK_MAIN_SHUTTERMODE_VALUES) },
    BinDef { index: 28, width: 1, signed: false, name: "MeteringMode", values: Some(KODAK_MAIN_METERINGMODE_VALUES) },
    BinDef { index: 30, width: 2, signed: false, name: "FNumber", values: None },
    BinDef { index: 32, width: 4, signed: false, name: "ExposureTime", values: None },
    BinDef { index: 36, width: 2, signed: true, name: "ExposureCompensation", values: None },
    BinDef { index: 38, width: 2, signed: false, name: "VariousModes", values: None },
    BinDef { index: 40, width: 4, signed: false, name: "Distance1", values: None },
    BinDef { index: 44, width: 4, signed: false, name: "Distance2", values: None },
    BinDef { index: 48, width: 4, signed: false, name: "Distance3", values: None },
    BinDef { index: 52, width: 4, signed: false, name: "Distance4", values: None },
    BinDef { index: 56, width: 1, signed: false, name: "FocusMode", values: Some(KODAK_MAIN_FOCUSMODE_VALUES) },
    BinDef { index: 58, width: 2, signed: false, name: "VariousModes2", values: None },
    BinDef { index: 60, width: 2, signed: false, name: "PanoramaMode", values: None },
    BinDef { index: 62, width: 2, signed: false, name: "SubjectDistance", values: None },
    BinDef { index: 64, width: 1, signed: false, name: "WhiteBalance", values: Some(KODAK_MAIN_WHITEBALANCE_VALUES) },
    BinDef { index: 9, width: 1, signed: false, name: "Quality", values: Some(KODAK_MAIN_QUALITY_VALUES) },
    BinDef { index: 92, width: 1, signed: false, name: "FlashMode", values: Some(KODAK_MAIN_FLASHMODE_VALUES) },
    BinDef { index: 93, width: 1, signed: false, name: "FlashFired", values: Some(KODAK_MAIN_FLASHFIRED_VALUES) },
    BinDef { index: 94, width: 2, signed: false, name: "ISOSetting", values: None },
    BinDef { index: 96, width: 2, signed: false, name: "ISO", values: None },
    BinDef { index: 98, width: 2, signed: false, name: "TotalZoom", values: None },
];

pub static KODAK_MAIN_BURSTMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static KODAK_MAIN_COLORMODE_VALUES: &[(i64, &str)] = &[
    (1, "B&W"),
    (16384, "Sepia"),
    (2, "Sepia"),
    (256, "Saturated Color"),
    (3, "B&W Yellow Filter"),
    (32, "Saturated Color"),
    (4, "B&W Red Filter"),
    (512, "Neutral Color"),
    (64, "Neutral Color"),
    (8192, "B&W"),
];

pub static KODAK_MAIN_SHARPNESS_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
];

pub static KODAK_MAIN_SHUTTERMODE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (32, "Manual?"),
    (8, "Aperture Priority"),
];

pub static KODAK_MAIN_METERINGMODE_VALUES: &[(i64, &str)] = &[
    (0, "Multi-segment"),
    (1, "Center-weighted average"),
    (2, "Spot"),
];

pub static KODAK_MAIN_FOCUSMODE_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (2, "Macro"),
];

pub static KODAK_MAIN_WHITEBALANCE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Flash?"),
    (2, "Tungsten"),
    (3, "Daylight"),
];

pub static KODAK_MAIN_QUALITY_VALUES: &[(i64, &str)] = &[
    (1, "Fine"),
    (2, "Normal"),
];

pub static KODAK_MAIN_FLASHMODE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Fill Flash"),
    (16, "Fill Flash"),
    (2, "Off"),
    (3, "Red-Eye"),
    (32, "Off"),
    (64, "Red-Eye?"),
];

pub static KODAK_MAIN_FLASHFIRED_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];


/// Look up a tag by ID in the main table.
pub fn lookup(_tag_id: u16) -> Option<&'static TagDef> {
    // Default to main table - override in specific modules
    None
}

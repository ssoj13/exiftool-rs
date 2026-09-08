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

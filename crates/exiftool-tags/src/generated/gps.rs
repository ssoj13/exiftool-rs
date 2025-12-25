//! GPS MakerNotes tag definitions.
//! Auto-generated from ExifTool - DO NOT EDIT

#![allow(dead_code)]

/// Tag definition with name and optional value mappings.
#[derive(Debug, Clone)]
pub struct TagDef {
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}

/// GPS::Main tags
pub static GPS_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "GPSVersionID", values: None },
    1u16 => TagDef { name: "GPSLatitudeRef", values: None },
    10u16 => TagDef { name: "GPSMeasureMode", values: Some(GPS_MAIN_GPSMEASUREMODE_VALUES) },
    11u16 => TagDef { name: "GPSDOP", values: None },
    12u16 => TagDef { name: "GPSSpeedRef", values: None },
    13u16 => TagDef { name: "GPSSpeed", values: None },
    14u16 => TagDef { name: "GPSTrackRef", values: None },
    15u16 => TagDef { name: "GPSTrack", values: None },
    16u16 => TagDef { name: "GPSImgDirectionRef", values: None },
    17u16 => TagDef { name: "GPSImgDirection", values: None },
    18u16 => TagDef { name: "GPSMapDatum", values: None },
    19u16 => TagDef { name: "GPSDestLatitudeRef", values: None },
    2u16 => TagDef { name: "GPSLatitude", values: None },
    20u16 => TagDef { name: "GPSDestLatitude", values: None },
    21u16 => TagDef { name: "GPSDestLongitudeRef", values: None },
    22u16 => TagDef { name: "GPSDestLongitude", values: None },
    23u16 => TagDef { name: "GPSDestBearingRef", values: None },
    24u16 => TagDef { name: "GPSDestBearing", values: None },
    25u16 => TagDef { name: "GPSDestDistanceRef", values: None },
    26u16 => TagDef { name: "GPSDestDistance", values: None },
    27u16 => TagDef { name: "GPSProcessingMethod", values: None },
    28u16 => TagDef { name: "GPSAreaInformation", values: None },
    29u16 => TagDef { name: "GPSDateStamp", values: None },
    3u16 => TagDef { name: "GPSLongitudeRef", values: None },
    30u16 => TagDef { name: "GPSDifferential", values: Some(GPS_MAIN_GPSDIFFERENTIAL_VALUES) },
    31u16 => TagDef { name: "GPSHPositioningError", values: None },
    4u16 => TagDef { name: "GPSLongitude", values: None },
    5u16 => TagDef { name: "GPSAltitudeRef", values: Some(GPS_MAIN_GPSALTITUDEREF_VALUES) },
    6u16 => TagDef { name: "GPSAltitude", values: None },
    7u16 => TagDef { name: "GPSTimeStamp", values: None },
    8u16 => TagDef { name: "GPSSatellites", values: None },
    9u16 => TagDef { name: "GPSStatus", values: None },
};

pub static GPS_MAIN_GPSMEASUREMODE_VALUES: &[(i64, &str)] = &[
    (2, "2-Dimensional Measurement"),
    (3, "3-Dimensional Measurement"),
];

pub static GPS_MAIN_GPSDIFFERENTIAL_VALUES: &[(i64, &str)] = &[
    (0, "No Correction"),
    (1, "Differential Corrected"),
];

pub static GPS_MAIN_GPSALTITUDEREF_VALUES: &[(i64, &str)] = &[
    (0, "Above Sea Level"),
    (1, "Below Sea Level"),
    (2, "Positive Sea Level (sea-level ref)"),
    (3, "Negative Sea Level (sea-level ref)"),
];


/// Look up a tag by ID in the main table.
pub fn lookup(_tag_id: u16) -> Option<&'static TagDef> {
    // Default to main table - override in specific modules
    None
}

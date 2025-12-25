//! Apple MakerNotes tag definitions.
//! Auto-generated from ExifTool - DO NOT EDIT

#![allow(dead_code)]

/// Tag definition with name and optional value mappings.
#[derive(Debug, Clone)]
pub struct TagDef {
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}

/// Apple::Main tags
pub static APPLE_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "MakerNoteVersion", values: None },
    10u16 => TagDef { name: "HDRImageType", values: Some(APPLE_MAIN_HDRIMAGETYPE_VALUES) },
    11u16 => TagDef { name: "BurstUUID", values: None },
    12u16 => TagDef { name: "FocusDistanceRange", values: None },
    15u16 => TagDef { name: "OISMode", values: None },
    17u16 => TagDef { name: "ContentIdentifier", values: None },
    2u16 => TagDef { name: "AEMatrix", values: None },
    20u16 => TagDef { name: "ImageCaptureType", values: Some(APPLE_MAIN_IMAGECAPTURETYPE_VALUES) },
    21u16 => TagDef { name: "ImageUniqueID", values: None },
    23u16 => TagDef { name: "LivePhotoVideoIndex", values: None },
    25u16 => TagDef { name: "ImageProcessingFlags", values: None },
    26u16 => TagDef { name: "QualityHint", values: None },
    29u16 => TagDef { name: "LuminanceNoiseAmplitude", values: None },
    3u16 => TagDef { name: "RunTime", values: None },
    31u16 => TagDef { name: "PhotosAppFeatureFlags", values: None },
    32u16 => TagDef { name: "ImageCaptureRequestID", values: None },
    33u16 => TagDef { name: "HDRHeadroom", values: None },
    35u16 => TagDef { name: "AFPerformance", values: None },
    37u16 => TagDef { name: "SceneFlags", values: None },
    38u16 => TagDef { name: "SignalToNoiseRatioType", values: None },
    39u16 => TagDef { name: "SignalToNoiseRatio", values: None },
    4u16 => TagDef { name: "AEStable", values: Some(APPLE_MAIN_AESTABLE_VALUES) },
    43u16 => TagDef { name: "PhotoIdentifier", values: None },
    45u16 => TagDef { name: "ColorTemperature", values: None },
    46u16 => TagDef { name: "CameraType", values: Some(APPLE_MAIN_CAMERATYPE_VALUES) },
    47u16 => TagDef { name: "FocusPosition", values: None },
    48u16 => TagDef { name: "HDRGain", values: None },
    5u16 => TagDef { name: "AETarget", values: None },
    56u16 => TagDef { name: "AFMeasuredDepth", values: None },
    6u16 => TagDef { name: "AEAverage", values: None },
    61u16 => TagDef { name: "AFConfidence", values: None },
    62u16 => TagDef { name: "ColorCorrectionMatrix", values: None },
    63u16 => TagDef { name: "GreenGhostMitigationStatus", values: None },
    64u16 => TagDef { name: "SemanticStyle", values: None },
    65u16 => TagDef { name: "SemanticStyleRenderingVer", values: None },
    66u16 => TagDef { name: "SemanticStylePreset", values: None },
    7u16 => TagDef { name: "AFStable", values: Some(APPLE_MAIN_AFSTABLE_VALUES) },
    78u16 => TagDef { name: "Apple_0x004e", values: None },
    79u16 => TagDef { name: "Apple_0x004f", values: None },
    8u16 => TagDef { name: "AccelerationVector", values: None },
    84u16 => TagDef { name: "Apple_0x0054", values: None },
    90u16 => TagDef { name: "Apple_0x005a", values: None },
};

pub static APPLE_MAIN_HDRIMAGETYPE_VALUES: &[(i64, &str)] = &[
    (3, "HDR Image"),
    (4, "Original Image"),
];

pub static APPLE_MAIN_IMAGECAPTURETYPE_VALUES: &[(i64, &str)] = &[
    (1, "ProRAW"),
    (10, "Photo"),
    (11, "Manual Focus"),
    (12, "Scene"),
    (2, "Portrait"),
];

pub static APPLE_MAIN_AESTABLE_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];

pub static APPLE_MAIN_CAMERATYPE_VALUES: &[(i64, &str)] = &[
    (0, "Back Wide Angle"),
    (1, "Back Normal"),
    (6, "Front"),
];

pub static APPLE_MAIN_AFSTABLE_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];


/// Look up a tag by ID in the main table.
pub fn lookup(_tag_id: u16) -> Option<&'static TagDef> {
    // Default to main table - override in specific modules
    None
}

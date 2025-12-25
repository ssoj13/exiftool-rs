//! Canon MakerNotes tag definitions.
//! Auto-generated from ExifTool - DO NOT EDIT

#![allow(dead_code)]

/// Tag definition with name and optional value mappings.
#[derive(Debug, Clone)]
pub struct TagDef {
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}

/// Canon::AFConfig tags
pub static CANON_AFCONFIG: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "AFConfigTool", values: Some(CANON_AFCONFIG_AFCONFIGTOOL_VALUES) },
    10u16 => TagDef { name: "AutoAFPointSelEOSiTRAF", values: Some(CANON_AFCONFIG_AUTOAFPOINTSELEOSITRAF_VALUES) },
    11u16 => TagDef { name: "LensDriveWhenAFImpossible", values: Some(CANON_AFCONFIG_LENSDRIVEWHENAFIMPOSSIBLE_VALUES) },
    12u16 => TagDef { name: "SelectAFAreaSelectionMode", values: None },
    13u16 => TagDef { name: "AFAreaSelectionMethod", values: Some(CANON_AFCONFIG_AFAREASELECTIONMETHOD_VALUES) },
    14u16 => TagDef { name: "OrientationLinkedAF", values: Some(CANON_AFCONFIG_ORIENTATIONLINKEDAF_VALUES) },
    15u16 => TagDef { name: "ManualAFPointSelPattern", values: Some(CANON_AFCONFIG_MANUALAFPOINTSELPATTERN_VALUES) },
    16u16 => TagDef { name: "AFPointDisplayDuringFocus", values: Some(CANON_AFCONFIG_AFPOINTDISPLAYDURINGFOCUS_VALUES) },
    17u16 => TagDef { name: "VFDisplayIllumination", values: Some(CANON_AFCONFIG_VFDISPLAYILLUMINATION_VALUES) },
    18u16 => TagDef { name: "AFStatusViewfinder", values: Some(CANON_AFCONFIG_AFSTATUSVIEWFINDER_VALUES) },
    19u16 => TagDef { name: "InitialAFPointInServo", values: Some(CANON_AFCONFIG_INITIALAFPOINTINSERVO_VALUES) },
    2u16 => TagDef { name: "AFTrackingSensitivity", values: Some(CANON_AFCONFIG_AFTRACKINGSENSITIVITY_VALUES) },
    20u16 => TagDef { name: "SubjectToDetect", values: Some(CANON_AFCONFIG_SUBJECTTODETECT_VALUES) },
    21u16 => TagDef { name: "SubjectSwitching", values: Some(CANON_AFCONFIG_SUBJECTSWITCHING_VALUES) },
    24u16 => TagDef { name: "EyeDetection", values: Some(CANON_AFCONFIG_EYEDETECTION_VALUES) },
    26u16 => TagDef { name: "WholeAreaTracking", values: Some(CANON_AFCONFIG_WHOLEAREATRACKING_VALUES) },
    27u16 => TagDef { name: "ServoAFCharacteristics", values: Some(CANON_AFCONFIG_SERVOAFCHARACTERISTICS_VALUES) },
    28u16 => TagDef { name: "CaseAutoSetting", values: Some(CANON_AFCONFIG_CASEAUTOSETTING_VALUES) },
    29u16 => TagDef { name: "ActionPriority", values: Some(CANON_AFCONFIG_ACTIONPRIORITY_VALUES) },
    3u16 => TagDef { name: "AFAccelDecelTracking", values: Some(CANON_AFCONFIG_AFACCELDECELTRACKING_VALUES) },
    30u16 => TagDef { name: "SportEvents", values: Some(CANON_AFCONFIG_SPORTEVENTS_VALUES) },
    4u16 => TagDef { name: "AFPointSwitching", values: Some(CANON_AFCONFIG_AFPOINTSWITCHING_VALUES) },
    5u16 => TagDef { name: "AIServoFirstImage", values: Some(CANON_AFCONFIG_AISERVOFIRSTIMAGE_VALUES) },
    6u16 => TagDef { name: "AIServoSecondImage", values: Some(CANON_AFCONFIG_AISERVOSECONDIMAGE_VALUES) },
    7u16 => TagDef { name: "USMLensElectronicMF", values: Some(CANON_AFCONFIG_USMLENSELECTRONICMF_VALUES) },
    8u16 => TagDef { name: "AFAssistBeam", values: Some(CANON_AFCONFIG_AFASSISTBEAM_VALUES) },
    9u16 => TagDef { name: "OneShotAFRelease", values: Some(CANON_AFCONFIG_ONESHOTAFRELEASE_VALUES) },
};

pub static CANON_AFCONFIG_AFCONFIGTOOL_VALUES: &[(i64, &str)] = &[
    (11, "Case A"),
    (2147483648, "n/a"),
];

pub static CANON_AFCONFIG_AUTOAFPOINTSELEOSITRAF_VALUES: &[(i64, &str)] = &[
    (0, "Enable"),
    (1, "Disable"),
];

pub static CANON_AFCONFIG_LENSDRIVEWHENAFIMPOSSIBLE_VALUES: &[(i64, &str)] = &[
    (0, "Continue Focus Search"),
    (1, "Stop Focus Search"),
];

pub static CANON_AFCONFIG_AFAREASELECTIONMETHOD_VALUES: &[(i64, &str)] = &[
    (0, "M-Fn Button"),
    (1, "Main Dial"),
];

pub static CANON_AFCONFIG_ORIENTATIONLINKEDAF_VALUES: &[(i64, &str)] = &[
    (0, "Same for Vert/Horiz Points"),
    (1, "Separate Vert/Horiz Points"),
    (2, "Separate Area+Points"),
];

pub static CANON_AFCONFIG_MANUALAFPOINTSELPATTERN_VALUES: &[(i64, &str)] = &[
    (0, "Stops at AF Area Edges"),
    (1, "Continuous"),
];

pub static CANON_AFCONFIG_AFPOINTDISPLAYDURINGFOCUS_VALUES: &[(i64, &str)] = &[
    (0, "Selected (constant)"),
    (1, "All (constant)"),
    (2, "Selected (pre-AF, focused)"),
    (3, "Selected (focused)"),
    (4, "Disabled"),
];

pub static CANON_AFCONFIG_VFDISPLAYILLUMINATION_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Enable"),
    (2, "Disable"),
];

pub static CANON_AFCONFIG_AFSTATUSVIEWFINDER_VALUES: &[(i64, &str)] = &[
    (0, "Show in Field of View"),
    (1, "Show Outside View"),
];

pub static CANON_AFCONFIG_INITIALAFPOINTINSERVO_VALUES: &[(i64, &str)] = &[
    (0, "Initial AF Point Selected"),
    (1, "Manual AF Point"),
    (2, "Auto"),
];

pub static CANON_AFCONFIG_AFTRACKINGSENSITIVITY_VALUES: &[(i64, &str)] = &[
    (127, "Auto"),
    (2147483647, "n/a"),
];

pub static CANON_AFCONFIG_SUBJECTTODETECT_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "People"),
    (2, "Animals"),
    (3, "Vehicles"),
    (4, "Auto"),
];

pub static CANON_AFCONFIG_SUBJECTSWITCHING_VALUES: &[(i64, &str)] = &[
    (0, "Initial Priority"),
    (1, "On Subject"),
    (2, "Switch Subject"),
];

pub static CANON_AFCONFIG_EYEDETECTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Auto"),
    (2, "Left Eye"),
    (3, "Right Eye"),
];

pub static CANON_AFCONFIG_WHOLEAREATRACKING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANON_AFCONFIG_SERVOAFCHARACTERISTICS_VALUES: &[(i64, &str)] = &[
    (0, "Case Auto"),
    (1, "Case Manual"),
];

pub static CANON_AFCONFIG_CASEAUTOSETTING_VALUES: &[(i64, &str)] = &[
    (-1, "Locked On"),
    (0, "Standard"),
    (1, "Responsive"),
    (2147483647, "n/a"),
];

pub static CANON_AFCONFIG_ACTIONPRIORITY_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANON_AFCONFIG_AFACCELDECELTRACKING_VALUES: &[(i64, &str)] = &[
    (127, "Auto"),
    (2147483647, "n/a"),
];

pub static CANON_AFCONFIG_SPORTEVENTS_VALUES: &[(i64, &str)] = &[
    (0, "Soccer"),
    (1, "Basketball"),
    (2, "Volleyball"),
];

pub static CANON_AFCONFIG_AFPOINTSWITCHING_VALUES: &[(i64, &str)] = &[
    (2147483647, "n/a"),
];

pub static CANON_AFCONFIG_AISERVOFIRSTIMAGE_VALUES: &[(i64, &str)] = &[
    (0, "Equal Priority"),
    (1, "Release Priority"),
    (2, "Focus Priority"),
];

pub static CANON_AFCONFIG_AISERVOSECONDIMAGE_VALUES: &[(i64, &str)] = &[
    (0, "Equal Priority"),
    (1, "Release Priority"),
    (2, "Focus Priority"),
    (3, "Release High Priority"),
    (4, "Focus High Priority"),
];

pub static CANON_AFCONFIG_USMLENSELECTRONICMF_VALUES: &[(i64, &str)] = &[
    (0, "Disable After One-Shot"),
    (1, "One-Shot -> Enabled"),
    (2, "One-Shot -> Enabled (magnify)"),
    (3, "Disable in AF Mode"),
];

pub static CANON_AFCONFIG_AFASSISTBEAM_VALUES: &[(i64, &str)] = &[
    (0, "Enable"),
    (1, "Disable"),
    (2, "IR AF Assist Beam Only"),
    (3, "LED AF Assist Beam Only"),
];

pub static CANON_AFCONFIG_ONESHOTAFRELEASE_VALUES: &[(i64, &str)] = &[
    (0, "Focus Priority"),
    (1, "Release Priority"),
];

/// Canon::AFInfo tags
pub static CANON_AFINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "NumAFPoints", values: None },
    1u16 => TagDef { name: "ValidAFPoints", values: None },
    10u16 => TagDef { name: "AFPointsInFocus", values: None },
    11u16 => TagDef { name: "PrimaryAFPoint", values: None },
    2u16 => TagDef { name: "CanonImageWidth", values: None },
    3u16 => TagDef { name: "CanonImageHeight", values: None },
    4u16 => TagDef { name: "AFImageWidth", values: None },
    8u16 => TagDef { name: "AFAreaXPositions", values: None },
    9u16 => TagDef { name: "AFAreaYPositions", values: None },
};

/// Canon::AFInfo2 tags
pub static CANON_AFINFO2: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "AFInfoSize", values: None },
    1u16 => TagDef { name: "AFAreaMode", values: Some(CANON_AFINFO2_AFAREAMODE_VALUES) },
    10u16 => TagDef { name: "AFAreaXPositions", values: None },
    11u16 => TagDef { name: "AFAreaYPositions", values: None },
    12u16 => TagDef { name: "AFPointsInFocus", values: None },
    13u16 => TagDef { name: "AFPointsSelected", values: None },
    14u16 => TagDef { name: "PrimaryAFPoint", values: None },
    2u16 => TagDef { name: "NumAFPoints", values: None },
    3u16 => TagDef { name: "ValidAFPoints", values: None },
    4u16 => TagDef { name: "CanonImageWidth", values: None },
    5u16 => TagDef { name: "CanonImageHeight", values: None },
    6u16 => TagDef { name: "AFImageWidth", values: None },
    8u16 => TagDef { name: "AFAreaWidths", values: None },
    9u16 => TagDef { name: "AFAreaHeights", values: None },
};

pub static CANON_AFINFO2_AFAREAMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off (Manual Focus)"),
    (1, "AF Point Expansion (surround)"),
    (10, "AF Point Expansion (8 point)"),
    (11, "Flexizone Multi (49 point)"),
    (12, "Flexizone Multi (9 point)"),
    (13, "Flexizone Single"),
    (14, "Large Zone AF"),
    (16, "Large Zone AF (vertical)"),
    (17, "Large Zone AF (horizontal)"),
    (19, "Flexible Zone AF 1"),
    (2, "Single-point AF"),
    (20, "Flexible Zone AF 2"),
    (21, "Flexible Zone AF 3"),
    (22, "Whole Area AF"),
    (4, "Auto"),
    (5, "Face Detect AF"),
    (6, "Face + Tracking"),
    (7, "Zone AF"),
    (8, "AF Point Expansion (4 point)"),
    (9, "Spot AF"),
];

/// Canon::AFMicroAdj tags
pub static CANON_AFMICROADJ: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "AFMicroAdjMode", values: Some(CANON_AFMICROADJ_AFMICROADJMODE_VALUES) },
    2u16 => TagDef { name: "AFMicroAdjValue", values: None },
};

pub static CANON_AFMICROADJ_AFMICROADJMODE_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Adjust all by the same amount"),
    (2, "Adjust by lens"),
];

/// Canon::Ambience tags
pub static CANON_AMBIENCE: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "AmbienceSelection", values: Some(CANON_AMBIENCE_AMBIENCESELECTION_VALUES) },
};

pub static CANON_AMBIENCE_AMBIENCESELECTION_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (1, "Vivid"),
    (2, "Warm"),
    (3, "Soft"),
    (4, "Cool"),
    (5, "Intense"),
    (6, "Brighter"),
    (7, "Darker"),
    (8, "Monochrome"),
];

/// Canon::AspectInfo tags
pub static CANON_ASPECTINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "AspectRatio", values: Some(CANON_ASPECTINFO_ASPECTRATIO_VALUES) },
};

pub static CANON_ASPECTINFO_ASPECTRATIO_VALUES: &[(i64, &str)] = &[
    (0, "3:2"),
    (1, "1:1"),
    (12, "3:2 (APS-H crop)"),
    (13, "3:2 (APS-C crop)"),
    (2, "4:3"),
    (258, "4:3 crop"),
    (7, "16:9"),
    (8, "4:5"),
];

/// Canon::CameraInfo1D tags
pub static CANON_CAMERAINFO1D: phf::Map<u16, TagDef> = phf::phf_map! {
    10u16 => TagDef { name: "FocalLength", values: None },
    13u16 => TagDef { name: "LensType", values: Some(CANON_CAMERAINFO1D_LENSTYPE_VALUES) },
    14u16 => TagDef { name: "MinFocalLength", values: None },
    16u16 => TagDef { name: "MaxFocalLength", values: None },
    4u16 => TagDef { name: "ExposureTime", values: None },
    65u16 => TagDef { name: "SharpnessFrequency", values: Some(CANON_CAMERAINFO1D_SHARPNESSFREQUENCY_VALUES) },
    66u16 => TagDef { name: "Sharpness", values: None },
    68u16 => TagDef { name: "WhiteBalance", values: Some(CANON_CAMERAINFO1D_WHITEBALANCE_VALUES) },
    71u16 => TagDef { name: "SharpnessFrequency", values: Some(CANON_CAMERAINFO1D_SHARPNESSFREQUENCY_VALUES) },
    72u16 => TagDef { name: "ColorTemperature", values: None },
    74u16 => TagDef { name: "WhiteBalance", values: Some(CANON_CAMERAINFO1D_WHITEBALANCE_VALUES) },
    75u16 => TagDef { name: "PictureStyle", values: Some(CANON_CAMERAINFO1D_PICTURESTYLE_VALUES) },
    78u16 => TagDef { name: "ColorTemperature", values: None },
    81u16 => TagDef { name: "PictureStyle", values: Some(CANON_CAMERAINFO1D_PICTURESTYLE_VALUES) },
};

pub static CANON_CAMERAINFO1D_LENSTYPE_VALUES: &[(i64, &str)] = &[
    (-1, "n/a"),
    (1, "Canon EF 50mm f/1.8"),
    (10, "Canon EF 50mm f/2.5 Macro or Sigma Lens"),
    (103, "Samyang AF 14mm f/2.8 EF or Rokinon Lens"),
    (106, "Rokinon SP / Samyang XP 35mm f/1.2"),
    (11, "Canon EF 35mm f/2"),
    (112, "Sigma 28mm f/1.5 FF High-speed Prime or other Sigma Lens"),
    (1136, "Sigma 24-70mm f/2.8 DG OS HSM | A"),
    (117, "Tamron 35-150mm f/2.8-4.0 Di VC OSD (A043) or other Tamron Lens"),
    (124, "Canon MP-E 65mm f/2.8 1-5x Macro Photo"),
    (125, "Canon TS-E 24mm f/3.5L"),
    (126, "Canon TS-E 45mm f/2.8"),
    (127, "Canon TS-E 90mm f/2.8 or Tamron Lens"),
    (129, "Canon EF 300mm f/2.8L USM"),
    (13, "Canon EF 15mm f/2.8 Fisheye"),
    (130, "Canon EF 50mm f/1.0L USM"),
    (131, "Canon EF 28-80mm f/2.8-4L USM or Sigma Lens"),
    (132, "Canon EF 1200mm f/5.6L USM"),
    (134, "Canon EF 600mm f/4L IS USM"),
    (135, "Canon EF 200mm f/1.8L USM"),
    (136, "Canon EF 300mm f/2.8L USM"),
    (137, "Canon EF 85mm f/1.2L USM or Sigma or Tamron Lens"),
    (138, "Canon EF 28-80mm f/2.8-4L"),
    (139, "Canon EF 400mm f/2.8L USM"),
    (14, "Canon EF 50-200mm f/3.5-4.5L"),
    (140, "Canon EF 500mm f/4.5L USM"),
    (141, "Canon EF 500mm f/4.5L USM"),
    (142, "Canon EF 300mm f/2.8L IS USM"),
    (143, "Canon EF 500mm f/4L IS USM or Sigma Lens"),
    (144, "Canon EF 35-135mm f/4-5.6 USM"),
    (145, "Canon EF 100-300mm f/4.5-5.6 USM"),
    (146, "Canon EF 70-210mm f/3.5-4.5 USM"),
    (147, "Canon EF 35-135mm f/4-5.6 USM"),
    (148, "Canon EF 28-80mm f/3.5-5.6 USM"),
    (149, "Canon EF 100mm f/2 USM"),
    (15, "Canon EF 50-200mm f/3.5-4.5"),
    (150, "Canon EF 14mm f/2.8L USM or Sigma Lens"),
    (151, "Canon EF 200mm f/2.8L USM"),
    (152, "Canon EF 300mm f/4L IS USM or Sigma Lens"),
    (153, "Canon EF 35-350mm f/3.5-5.6L USM or Sigma or Tamron Lens"),
    (154, "Canon EF 20mm f/2.8 USM or Zeiss Lens"),
    (155, "Canon EF 85mm f/1.8 USM or Sigma Lens"),
    (156, "Canon EF 28-105mm f/3.5-4.5 USM or Tamron Lens"),
    (16, "Canon EF 35-135mm f/3.5-4.5"),
    (160, "Canon EF 20-35mm f/3.5-4.5 USM or Tamron or Tokina Lens"),
    (161, "Canon EF 28-70mm f/2.8L USM or Other Lens"),
    (162, "Canon EF 200mm f/2.8L USM"),
    (163, "Canon EF 300mm f/4L"),
    (164, "Canon EF 400mm f/5.6L"),
    (165, "Canon EF 70-200mm f/2.8L USM"),
    (166, "Canon EF 70-200mm f/2.8L USM + 1.4x"),
    (167, "Canon EF 70-200mm f/2.8L USM + 2x"),
    (168, "Canon EF 28mm f/1.8 USM or Sigma Lens"),
    (169, "Canon EF 17-35mm f/2.8L USM or Sigma Lens"),
    (17, "Canon EF 35-70mm f/3.5-4.5A"),
    (170, "Canon EF 200mm f/2.8L II USM or Sigma Lens"),
    (171, "Canon EF 300mm f/4L USM"),
    (172, "Canon EF 400mm f/5.6L USM or Sigma Lens"),
    (173, "Canon EF 180mm Macro f/3.5L USM or Sigma Lens"),
    (174, "Canon EF 135mm f/2L USM or Other Lens"),
    (175, "Canon EF 400mm f/2.8L USM"),
    (176, "Canon EF 24-85mm f/3.5-4.5 USM"),
    (177, "Canon EF 300mm f/4L IS USM"),
    (178, "Canon EF 28-135mm f/3.5-5.6 IS"),
    (179, "Canon EF 24mm f/1.4L USM"),
    (18, "Canon EF 28-70mm f/3.5-4.5"),
    (180, "Canon EF 35mm f/1.4L USM or Other Lens"),
    (181, "Canon EF 100-400mm f/4.5-5.6L IS USM + 1.4x or Sigma Lens"),
    (182, "Canon EF 100-400mm f/4.5-5.6L IS USM + 2x or Sigma Lens"),
    (183, "Canon EF 100-400mm f/4.5-5.6L IS USM or Sigma Lens"),
    (184, "Canon EF 400mm f/2.8L USM + 2x"),
    (185, "Canon EF 600mm f/4L IS USM"),
    (186, "Canon EF 70-200mm f/4L USM"),
    (187, "Canon EF 70-200mm f/4L USM + 1.4x"),
    (188, "Canon EF 70-200mm f/4L USM + 2x"),
    (189, "Canon EF 70-200mm f/4L USM + 2.8x"),
    (190, "Canon EF 100mm f/2.8 Macro USM"),
    (191, "Canon EF 400mm f/4 DO IS or Sigma Lens"),
    (193, "Canon EF 35-80mm f/4-5.6 USM"),
    (194, "Canon EF 80-200mm f/4.5-5.6 USM"),
    (195, "Canon EF 35-105mm f/4.5-5.6 USM"),
    (196, "Canon EF 75-300mm f/4-5.6 USM"),
    (197, "Canon EF 75-300mm f/4-5.6 IS USM or Sigma Lens"),
    (198, "Canon EF 50mm f/1.4 USM or Other Lens"),
    (199, "Canon EF 28-80mm f/3.5-5.6 USM"),
    (2, "Canon EF 28mm f/2.8 or Sigma Lens"),
    (20, "Canon EF 100-200mm f/4.5A"),
    (200, "Canon EF 75-300mm f/4-5.6 USM"),
    (201, "Canon EF 28-80mm f/3.5-5.6 USM"),
    (202, "Canon EF 28-80mm f/3.5-5.6 USM IV"),
    (208, "Canon EF 22-55mm f/4-5.6 USM"),
    (209, "Canon EF 55-200mm f/4.5-5.6"),
    (21, "Canon EF 80-200mm f/2.8L"),
    (210, "Canon EF 28-90mm f/4-5.6 USM"),
    (211, "Canon EF 28-200mm f/3.5-5.6 USM"),
    (212, "Canon EF 28-105mm f/4-5.6 USM"),
    (213, "Canon EF 90-300mm f/4.5-5.6 USM or Tamron Lens"),
    (214, "Canon EF-S 18-55mm f/3.5-5.6 USM"),
    (215, "Canon EF 55-200mm f/4.5-5.6 II USM"),
    (217, "Tamron AF 18-270mm f/3.5-6.3 Di II VC PZD"),
    (22, "Canon EF 20-35mm f/2.8L or Tokina Lens"),
    (220, "Yongnuo YN 50mm f/1.8"),
    (224, "Canon EF 70-200mm f/2.8L IS USM"),
    (225, "Canon EF 70-200mm f/2.8L IS USM + 1.4x"),
    (226, "Canon EF 70-200mm f/2.8L IS USM + 2x"),
    (227, "Canon EF 70-200mm f/2.8L IS USM + 2.8x"),
    (228, "Canon EF 28-105mm f/3.5-4.5 USM"),
    (229, "Canon EF 16-35mm f/2.8L USM"),
    (23, "Canon EF 35-105mm f/3.5-4.5"),
    (230, "Canon EF 24-70mm f/2.8L USM"),
    (231, "Canon EF 17-40mm f/4L USM or Sigma Lens"),
    (232, "Canon EF 70-300mm f/4.5-5.6 DO IS USM"),
    (233, "Canon EF 28-300mm f/3.5-5.6L IS USM"),
    (234, "Canon EF-S 17-85mm f/4-5.6 IS USM or Tokina Lens"),
    (235, "Canon EF-S 10-22mm f/3.5-4.5 USM"),
    (236, "Canon EF-S 60mm f/2.8 Macro USM"),
    (237, "Canon EF 24-105mm f/4L IS USM"),
    (238, "Canon EF 70-300mm f/4-5.6 IS USM"),
    (239, "Canon EF 85mm f/1.2L II USM or Rokinon Lens"),
    (24, "Canon EF 35-80mm f/4-5.6 Power Zoom"),
    (240, "Canon EF-S 17-55mm f/2.8 IS USM or Sigma Lens"),
    (241, "Canon EF 50mm f/1.2L USM"),
    (242, "Canon EF 70-200mm f/4L IS USM"),
    (243, "Canon EF 70-200mm f/4L IS USM + 1.4x"),
    (244, "Canon EF 70-200mm f/4L IS USM + 2x"),
    (245, "Canon EF 70-200mm f/4L IS USM + 2.8x"),
    (246, "Canon EF 16-35mm f/2.8L II USM"),
    (247, "Canon EF 14mm f/2.8L II USM"),
    (248, "Canon EF 200mm f/2L IS USM or Sigma Lens"),
    (249, "Canon EF 800mm f/5.6L IS USM"),
    (25, "Canon EF 35-80mm f/4-5.6 Power Zoom"),
    (250, "Canon EF 24mm f/1.4L II USM or Sigma Lens"),
    (251, "Canon EF 70-200mm f/2.8L IS II USM"),
    (252, "Canon EF 70-200mm f/2.8L IS II USM + 1.4x"),
    (253, "Canon EF 70-200mm f/2.8L IS II USM + 2x"),
    (254, "Canon EF 100mm f/2.8L Macro IS USM or Tamron Lens"),
    (255, "Sigma 24-105mm f/4 DG OS HSM | A or Other Lens"),
    (26, "Canon EF 100mm f/2.8 Macro or Other Lens"),
    (27, "Canon EF 35-80mm f/4-5.6"),
    (28, "Canon EF 80-200mm f/4.5-5.6 or Tamron Lens"),
    (29, "Canon EF 50mm f/1.8 II"),
    (3, "Canon EF 135mm f/2.8 Soft"),
    (30, "Canon EF 35-105mm f/4.5-5.6"),
    (31, "Canon EF 75-300mm f/4-5.6 or Tamron Lens"),
    (32, "Canon EF 24mm f/2.8 or Sigma Lens"),
    (33, "Voigtlander or Carl Zeiss Lens"),
    (35, "Canon EF 35-80mm f/4-5.6"),
    (36, "Canon EF 38-76mm f/4.5-5.6"),
    (368, "Sigma 14-24mm f/2.8 DG HSM | A or other Sigma Lens"),
    (36910, "Canon EF 70-300mm f/4-5.6 IS II USM"),
    (36912, "Canon EF-S 18-135mm f/3.5-5.6 IS USM"),
    (37, "Canon EF 35-80mm f/4-5.6 or Tamron Lens"),
    (38, "Canon EF 80-200mm f/4.5-5.6 II"),
    (39, "Canon EF 75-300mm f/4-5.6"),
    (4, "Canon EF 35-105mm f/3.5-4.5 or Sigma Lens"),
    (40, "Canon EF 28-80mm f/3.5-5.6"),
    (41, "Canon EF 28-90mm f/4-5.6"),
    (4142, "Canon EF-S 18-135mm f/3.5-5.6 IS STM"),
    (4143, "Canon EF-M 18-55mm f/3.5-5.6 IS STM or Tamron Lens"),
    (4144, "Canon EF 40mm f/2.8 STM"),
    (4145, "Canon EF-M 22mm f/2 STM"),
    (4146, "Canon EF-S 18-55mm f/3.5-5.6 IS STM"),
    (4147, "Canon EF-M 11-22mm f/4-5.6 IS STM"),
    (4148, "Canon EF-S 55-250mm f/4-5.6 IS STM"),
    (4149, "Canon EF-M 55-200mm f/4.5-6.3 IS STM"),
    (4150, "Canon EF-S 10-18mm f/4.5-5.6 IS STM"),
    (4152, "Canon EF 24-105mm f/3.5-5.6 IS STM"),
    (4153, "Canon EF-M 15-45mm f/3.5-6.3 IS STM"),
    (4154, "Canon EF-S 24mm f/2.8 STM"),
    (4155, "Canon EF-M 28mm f/3.5 Macro IS STM"),
    (4156, "Canon EF 50mm f/1.8 STM"),
    (4157, "Canon EF-M 18-150mm f/3.5-6.3 IS STM"),
    (4158, "Canon EF-S 18-55mm f/4-5.6 IS STM"),
    (4159, "Canon EF-M 32mm f/1.4 STM"),
    (4160, "Canon EF-S 35mm f/2.8 Macro IS STM"),
    (42, "Canon EF 28-200mm f/3.5-5.6 or Tamron Lens"),
    (4208, "Sigma 56mm f/1.4 DC DN | C or other Sigma Lens"),
    (43, "Canon EF 28-105mm f/4-5.6"),
    (44, "Canon EF 90-300mm f/4.5-5.6"),
    (45, "Canon EF-S 18-55mm f/3.5-5.6 [II]"),
    (46, "Canon EF 28-90mm f/4-5.6"),
    (47, "Zeiss Milvus 35mm f/2 or 50mm f/2"),
    (48, "Canon EF-S 18-55mm f/3.5-5.6 IS"),
    (488, "Canon EF-S 15-85mm f/3.5-5.6 IS USM"),
    (489, "Canon EF 70-300mm f/4-5.6L IS USM"),
    (49, "Canon EF-S 55-250mm f/4-5.6 IS"),
    (490, "Canon EF 8-15mm f/4L Fisheye USM"),
    (491, "Canon EF 300mm f/2.8L IS II USM or Tamron Lens"),
    (492, "Canon EF 400mm f/2.8L IS II USM"),
    (493, "Canon EF 500mm f/4L IS II USM or EF 24-105mm f4L IS USM"),
    (494, "Canon EF 600mm f/4L IS II USM"),
    (495, "Canon EF 24-70mm f/2.8L II USM or Sigma Lens"),
    (496, "Canon EF 200-400mm f/4L IS USM"),
    (4976, "Sigma 16-300mm F3.5-6.7 DC OS | C (025)"),
    (499, "Canon EF 200-400mm f/4L IS USM + 1.4x"),
    (5, "Canon EF 35-70mm f/3.5-4.5"),
    (50, "Canon EF-S 18-200mm f/3.5-5.6 IS"),
    (502, "Canon EF 28mm f/2.8 IS USM or Tamron Lens"),
    (503, "Canon EF 24mm f/2.8 IS USM"),
    (504, "Canon EF 24-70mm f/4L IS USM"),
    (505, "Canon EF 35mm f/2 IS USM"),
    (506, "Canon EF 400mm f/4 DO IS II USM"),
    (507, "Canon EF 16-35mm f/4L IS USM"),
    (508, "Canon EF 11-24mm f/4L USM or Tamron Lens"),
    (51, "Canon EF-S 18-135mm f/3.5-5.6 IS"),
    (52, "Canon EF-S 18-55mm f/3.5-5.6 IS II"),
    (53, "Canon EF-S 18-55mm f/3.5-5.6 III"),
    (54, "Canon EF-S 55-250mm f/4-5.6 IS II"),
    (6, "Canon EF 28-70mm f/3.5-4.5 or Sigma or Tokina Lens"),
    (60, "Irix 11mm f/4 or 15mm f/2.4"),
    (61182, "Canon RF 50mm F1.2L USM or other Canon RF Lens"),
    (61491, "Canon CN-E 14mm T3.1 L F"),
    (61492, "Canon CN-E 24mm T1.5 L F"),
    (61494, "Canon CN-E 85mm T1.3 L F"),
    (61495, "Canon CN-E 135mm T2.2 L F"),
    (61496, "Canon CN-E 35mm T1.5 L F"),
    (624, "Sigma 70-200mm f/2.8 DG OS HSM | S or other Sigma Lens"),
    (63, "Irix 30mm F1.4 Dragonfly"),
    (6512, "Sigma 12mm F1.4 DC | C"),
    (65535, "n/a"),
    (7, "Canon EF 100-300mm f/5.6L"),
    (747, "Canon EF 100-400mm f/4.5-5.6L IS II USM or Tamron Lens"),
    (748, "Canon EF 100-400mm f/4.5-5.6L IS II USM + 1.4x or Tamron Lens"),
    (749, "Canon EF 100-400mm f/4.5-5.6L IS II USM + 2x or Tamron Lens"),
    (750, "Canon EF 35mm f/1.4L II USM or Tamron Lens"),
    (751, "Canon EF 16-35mm f/2.8L III USM"),
    (752, "Canon EF 24-105mm f/4L IS II USM"),
    (753, "Canon EF 85mm f/1.4L IS USM"),
    (754, "Canon EF 70-200mm f/4L IS II USM"),
    (757, "Canon EF 400mm f/2.8L IS III USM"),
    (758, "Canon EF 600mm f/4L IS III USM"),
    (8, "Canon EF 100-300mm f/5.6 or Sigma or Tokina Lens"),
    (80, "Canon TS-E 50mm f/2.8L Macro"),
    (81, "Canon TS-E 90mm f/2.8L Macro"),
    (82, "Canon TS-E 135mm f/4L Macro"),
    (9, "Canon EF 70-210mm f/4"),
    (94, "Canon TS-E 17mm f/4L"),
    (95, "Canon TS-E 24mm f/3.5L II"),
];

pub static CANON_CAMERAINFO1D_SHARPNESSFREQUENCY_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "Lowest"),
    (2, "Low"),
    (3, "Standard"),
    (4, "High"),
    (5, "Highest"),
];

pub static CANON_CAMERAINFO1D_WHITEBALANCE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Daylight"),
    (10, "PC Set1"),
    (11, "PC Set2"),
    (12, "PC Set3"),
    (14, "Daylight Fluorescent"),
    (15, "Custom 1"),
    (16, "Custom 2"),
    (17, "Underwater"),
    (18, "Custom 3"),
    (19, "Custom 4"),
    (2, "Cloudy"),
    (20, "PC Set4"),
    (21, "PC Set5"),
    (23, "Auto (ambience priority)"),
    (3, "Tungsten"),
    (4, "Fluorescent"),
    (5, "Flash"),
    (6, "Custom"),
    (7, "Black & White"),
    (8, "Shade"),
    (9, "Manual Temperature (Kelvin)"),
];

pub static CANON_CAMERAINFO1D_PICTURESTYLE_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "Standard"),
    (129, "Standard"),
    (130, "Portrait"),
    (131, "Landscape"),
    (132, "Neutral"),
    (133, "Faithful"),
    (134, "Monochrome"),
    (135, "Auto"),
    (136, "Fine Detail"),
    (2, "Portrait"),
    (255, "n/a"),
    (3, "High Saturation"),
    (33, "User Def. 1"),
    (34, "User Def. 2"),
    (35, "User Def. 3"),
    (4, "Adobe RGB"),
    (5, "Low Saturation"),
    (6, "CM Set 1"),
    (65, "PC 1"),
    (65535, "n/a"),
    (66, "PC 2"),
    (67, "PC 3"),
    (7, "CM Set 2"),
];

/// Canon::CameraSettings tags
pub static CANON_CAMERASETTINGS: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "MacroMode", values: Some(CANON_CAMERASETTINGS_MACROMODE_VALUES) },
    10u16 => TagDef { name: "CanonImageSize", values: Some(CANON_CAMERASETTINGS_CANONIMAGESIZE_VALUES) },
    11u16 => TagDef { name: "EasyMode", values: Some(CANON_CAMERASETTINGS_EASYMODE_VALUES) },
    12u16 => TagDef { name: "DigitalZoom", values: Some(CANON_CAMERASETTINGS_DIGITALZOOM_VALUES) },
    13u16 => TagDef { name: "Contrast", values: Some(CANON_CAMERASETTINGS_CONTRAST_VALUES) },
    14u16 => TagDef { name: "Saturation", values: Some(CANON_CAMERASETTINGS_SATURATION_VALUES) },
    15u16 => TagDef { name: "Sharpness", values: None },
    16u16 => TagDef { name: "CameraISO", values: None },
    17u16 => TagDef { name: "MeteringMode", values: Some(CANON_CAMERASETTINGS_METERINGMODE_VALUES) },
    18u16 => TagDef { name: "FocusRange", values: Some(CANON_CAMERASETTINGS_FOCUSRANGE_VALUES) },
    19u16 => TagDef { name: "AFPoint", values: Some(CANON_CAMERASETTINGS_AFPOINT_VALUES) },
    2u16 => TagDef { name: "SelfTimer", values: None },
    20u16 => TagDef { name: "CanonExposureMode", values: Some(CANON_CAMERASETTINGS_CANONEXPOSUREMODE_VALUES) },
    22u16 => TagDef { name: "LensType", values: Some(CANON_CAMERASETTINGS_LENSTYPE_VALUES) },
    23u16 => TagDef { name: "MaxFocalLength", values: None },
    24u16 => TagDef { name: "MinFocalLength", values: None },
    25u16 => TagDef { name: "FocalUnits", values: None },
    26u16 => TagDef { name: "MaxAperture", values: None },
    27u16 => TagDef { name: "MinAperture", values: None },
    28u16 => TagDef { name: "FlashActivity", values: None },
    29u16 => TagDef { name: "FlashBits", values: Some(CANON_CAMERASETTINGS_FLASHBITS_VALUES) },
    3u16 => TagDef { name: "Quality", values: Some(CANON_CAMERASETTINGS_QUALITY_VALUES) },
    32u16 => TagDef { name: "FocusContinuous", values: Some(CANON_CAMERASETTINGS_FOCUSCONTINUOUS_VALUES) },
    33u16 => TagDef { name: "AESetting", values: Some(CANON_CAMERASETTINGS_AESETTING_VALUES) },
    34u16 => TagDef { name: "ImageStabilization", values: Some(CANON_CAMERASETTINGS_IMAGESTABILIZATION_VALUES) },
    35u16 => TagDef { name: "DisplayAperture", values: None },
    39u16 => TagDef { name: "SpotMeteringMode", values: Some(CANON_CAMERASETTINGS_SPOTMETERINGMODE_VALUES) },
    4u16 => TagDef { name: "CanonFlashMode", values: Some(CANON_CAMERASETTINGS_CANONFLASHMODE_VALUES) },
    40u16 => TagDef { name: "PhotoEffect", values: Some(CANON_CAMERASETTINGS_PHOTOEFFECT_VALUES) },
    41u16 => TagDef { name: "ManualFlashOutput", values: Some(CANON_CAMERASETTINGS_MANUALFLASHOUTPUT_VALUES) },
    42u16 => TagDef { name: "ColorTone", values: Some(CANON_CAMERASETTINGS_COLORTONE_VALUES) },
    46u16 => TagDef { name: "SRAWQuality", values: Some(CANON_CAMERASETTINGS_SRAWQUALITY_VALUES) },
    5u16 => TagDef { name: "ContinuousDrive", values: Some(CANON_CAMERASETTINGS_CONTINUOUSDRIVE_VALUES) },
    50u16 => TagDef { name: "FocusBracketing", values: Some(CANON_CAMERASETTINGS_FOCUSBRACKETING_VALUES) },
    51u16 => TagDef { name: "Clarity", values: Some(CANON_CAMERASETTINGS_CLARITY_VALUES) },
    52u16 => TagDef { name: "HDR-PQ", values: Some(CANON_CAMERASETTINGS_HDR_PQ_VALUES) },
    7u16 => TagDef { name: "FocusMode", values: Some(CANON_CAMERASETTINGS_FOCUSMODE_VALUES) },
    9u16 => TagDef { name: "RecordMode", values: Some(CANON_CAMERASETTINGS_RECORDMODE_VALUES) },
};

pub static CANON_CAMERASETTINGS_MACROMODE_VALUES: &[(i64, &str)] = &[
    (1, "Macro"),
    (2, "Normal"),
];

pub static CANON_CAMERASETTINGS_CANONIMAGESIZE_VALUES: &[(i64, &str)] = &[
    (-1, "n/a"),
    (0, "Large"),
    (1, "Medium"),
    (10, "Medium Widescreen"),
    (128, "640x480 Movie"),
    (129, "Medium Movie"),
    (130, "Small Movie"),
    (137, "1280x720 Movie"),
    (14, "Small 1"),
    (142, "1920x1080 Movie"),
    (143, "4096x2160 Movie"),
    (15, "Small 2"),
    (16, "Small 3"),
    (2, "Small"),
    (5, "Medium 1"),
    (6, "Medium 2"),
    (7, "Medium 3"),
    (8, "Postcard"),
    (9, "Widescreen"),
];

pub static CANON_CAMERASETTINGS_EASYMODE_VALUES: &[(i64, &str)] = &[
    (0, "Full auto"),
    (1, "Manual"),
    (10, "Macro"),
    (11, "Black & White"),
    (12, "Pan focus"),
    (13, "Vivid"),
    (14, "Neutral"),
    (15, "Flash Off"),
    (16, "Long Shutter"),
    (17, "Super Macro"),
    (18, "Foliage"),
    (19, "Indoor"),
    (2, "Landscape"),
    (20, "Fireworks"),
    (21, "Beach"),
    (22, "Underwater"),
    (23, "Snow"),
    (24, "Kids & Pets"),
    (25, "Night Snapshot"),
    (257, "Spotlight"),
    (258, "Night 2"),
    (259, "Night+"),
    (26, "Digital Macro"),
    (260, "Super Night"),
    (261, "Sunset"),
    (263, "Night Scene"),
    (264, "Surface"),
    (265, "Low Light 2"),
    (27, "My Colors"),
    (28, "Movie Snap"),
    (29, "Super Macro 2"),
    (3, "Fast shutter"),
    (30, "Color Accent"),
    (31, "Color Swap"),
    (32, "Aquarium"),
    (33, "ISO 3200"),
    (34, "ISO 6400"),
    (35, "Creative Light Effect"),
    (36, "Easy"),
    (37, "Quick Shot"),
    (38, "Creative Auto"),
    (39, "Zoom Blur"),
    (4, "Slow shutter"),
    (40, "Low Light"),
    (41, "Nostalgic"),
    (42, "Super Vivid"),
    (43, "Poster Effect"),
    (44, "Face Self-timer"),
    (45, "Smile"),
    (46, "Wink Self-timer"),
    (47, "Fisheye Effect"),
    (48, "Miniature Effect"),
    (49, "High-speed Burst"),
    (5, "Night"),
    (50, "Best Image Selection"),
    (51, "High Dynamic Range"),
    (52, "Handheld Night Scene"),
    (53, "Movie Digest"),
    (54, "Live View Control"),
    (55, "Discreet"),
    (56, "Blur Reduction"),
    (57, "Monochrome"),
    (58, "Toy Camera Effect"),
    (59, "Scene Intelligent Auto"),
    (6, "Gray Scale"),
    (60, "High-speed Burst HQ"),
    (61, "Smooth Skin"),
    (62, "Soft Focus"),
    (68, "Food"),
    (7, "Sepia"),
    (8, "Portrait"),
    (84, "HDR Art Standard"),
    (85, "HDR Art Vivid"),
    (9, "Sports"),
    (93, "HDR Art Bold"),
];

pub static CANON_CAMERASETTINGS_DIGITALZOOM_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "2x"),
    (2, "4x"),
    (3, "Other"),
];

pub static CANON_CAMERASETTINGS_CONTRAST_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
];

pub static CANON_CAMERASETTINGS_SATURATION_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
];

pub static CANON_CAMERASETTINGS_METERINGMODE_VALUES: &[(i64, &str)] = &[
    (0, "Default"),
    (1, "Spot"),
    (2, "Average"),
    (3, "Evaluative"),
    (4, "Partial"),
    (5, "Center-weighted average"),
];

pub static CANON_CAMERASETTINGS_FOCUSRANGE_VALUES: &[(i64, &str)] = &[
    (0, "Manual"),
    (1, "Auto"),
    (10, "Infinity"),
    (2, "Not Known"),
    (3, "Macro"),
    (4, "Very Close"),
    (5, "Close"),
    (6, "Middle Range"),
    (7, "Far Range"),
    (8, "Pan Focus"),
    (9, "Super Macro"),
];

pub static CANON_CAMERASETTINGS_AFPOINT_VALUES: &[(i64, &str)] = &[
    (12288, "None (MF)"),
    (12289, "Auto AF point selection"),
    (12290, "Right"),
    (12291, "Center"),
    (12292, "Left"),
    (16385, "Auto AF point selection"),
    (16390, "Face Detect"),
    (8197, "Manual AF point selection"),
];

pub static CANON_CAMERASETTINGS_CANONEXPOSUREMODE_VALUES: &[(i64, &str)] = &[
    (0, "Easy"),
    (1, "Program AE"),
    (2, "Shutter speed priority AE"),
    (3, "Aperture-priority AE"),
    (4, "Manual"),
    (5, "Depth-of-field AE"),
    (6, "M-Dep"),
    (7, "Bulb"),
    (8, "Flexible-priority AE"),
];

pub static CANON_CAMERASETTINGS_LENSTYPE_VALUES: &[(i64, &str)] = &[
    (-1, "n/a"),
    (1, "Canon EF 50mm f/1.8"),
    (10, "Canon EF 50mm f/2.5 Macro or Sigma Lens"),
    (103, "Samyang AF 14mm f/2.8 EF or Rokinon Lens"),
    (106, "Rokinon SP / Samyang XP 35mm f/1.2"),
    (11, "Canon EF 35mm f/2"),
    (112, "Sigma 28mm f/1.5 FF High-speed Prime or other Sigma Lens"),
    (1136, "Sigma 24-70mm f/2.8 DG OS HSM | A"),
    (117, "Tamron 35-150mm f/2.8-4.0 Di VC OSD (A043) or other Tamron Lens"),
    (124, "Canon MP-E 65mm f/2.8 1-5x Macro Photo"),
    (125, "Canon TS-E 24mm f/3.5L"),
    (126, "Canon TS-E 45mm f/2.8"),
    (127, "Canon TS-E 90mm f/2.8 or Tamron Lens"),
    (129, "Canon EF 300mm f/2.8L USM"),
    (13, "Canon EF 15mm f/2.8 Fisheye"),
    (130, "Canon EF 50mm f/1.0L USM"),
    (131, "Canon EF 28-80mm f/2.8-4L USM or Sigma Lens"),
    (132, "Canon EF 1200mm f/5.6L USM"),
    (134, "Canon EF 600mm f/4L IS USM"),
    (135, "Canon EF 200mm f/1.8L USM"),
    (136, "Canon EF 300mm f/2.8L USM"),
    (137, "Canon EF 85mm f/1.2L USM or Sigma or Tamron Lens"),
    (138, "Canon EF 28-80mm f/2.8-4L"),
    (139, "Canon EF 400mm f/2.8L USM"),
    (14, "Canon EF 50-200mm f/3.5-4.5L"),
    (140, "Canon EF 500mm f/4.5L USM"),
    (141, "Canon EF 500mm f/4.5L USM"),
    (142, "Canon EF 300mm f/2.8L IS USM"),
    (143, "Canon EF 500mm f/4L IS USM or Sigma Lens"),
    (144, "Canon EF 35-135mm f/4-5.6 USM"),
    (145, "Canon EF 100-300mm f/4.5-5.6 USM"),
    (146, "Canon EF 70-210mm f/3.5-4.5 USM"),
    (147, "Canon EF 35-135mm f/4-5.6 USM"),
    (148, "Canon EF 28-80mm f/3.5-5.6 USM"),
    (149, "Canon EF 100mm f/2 USM"),
    (15, "Canon EF 50-200mm f/3.5-4.5"),
    (150, "Canon EF 14mm f/2.8L USM or Sigma Lens"),
    (151, "Canon EF 200mm f/2.8L USM"),
    (152, "Canon EF 300mm f/4L IS USM or Sigma Lens"),
    (153, "Canon EF 35-350mm f/3.5-5.6L USM or Sigma or Tamron Lens"),
    (154, "Canon EF 20mm f/2.8 USM or Zeiss Lens"),
    (155, "Canon EF 85mm f/1.8 USM or Sigma Lens"),
    (156, "Canon EF 28-105mm f/3.5-4.5 USM or Tamron Lens"),
    (16, "Canon EF 35-135mm f/3.5-4.5"),
    (160, "Canon EF 20-35mm f/3.5-4.5 USM or Tamron or Tokina Lens"),
    (161, "Canon EF 28-70mm f/2.8L USM or Other Lens"),
    (162, "Canon EF 200mm f/2.8L USM"),
    (163, "Canon EF 300mm f/4L"),
    (164, "Canon EF 400mm f/5.6L"),
    (165, "Canon EF 70-200mm f/2.8L USM"),
    (166, "Canon EF 70-200mm f/2.8L USM + 1.4x"),
    (167, "Canon EF 70-200mm f/2.8L USM + 2x"),
    (168, "Canon EF 28mm f/1.8 USM or Sigma Lens"),
    (169, "Canon EF 17-35mm f/2.8L USM or Sigma Lens"),
    (17, "Canon EF 35-70mm f/3.5-4.5A"),
    (170, "Canon EF 200mm f/2.8L II USM or Sigma Lens"),
    (171, "Canon EF 300mm f/4L USM"),
    (172, "Canon EF 400mm f/5.6L USM or Sigma Lens"),
    (173, "Canon EF 180mm Macro f/3.5L USM or Sigma Lens"),
    (174, "Canon EF 135mm f/2L USM or Other Lens"),
    (175, "Canon EF 400mm f/2.8L USM"),
    (176, "Canon EF 24-85mm f/3.5-4.5 USM"),
    (177, "Canon EF 300mm f/4L IS USM"),
    (178, "Canon EF 28-135mm f/3.5-5.6 IS"),
    (179, "Canon EF 24mm f/1.4L USM"),
    (18, "Canon EF 28-70mm f/3.5-4.5"),
    (180, "Canon EF 35mm f/1.4L USM or Other Lens"),
    (181, "Canon EF 100-400mm f/4.5-5.6L IS USM + 1.4x or Sigma Lens"),
    (182, "Canon EF 100-400mm f/4.5-5.6L IS USM + 2x or Sigma Lens"),
    (183, "Canon EF 100-400mm f/4.5-5.6L IS USM or Sigma Lens"),
    (184, "Canon EF 400mm f/2.8L USM + 2x"),
    (185, "Canon EF 600mm f/4L IS USM"),
    (186, "Canon EF 70-200mm f/4L USM"),
    (187, "Canon EF 70-200mm f/4L USM + 1.4x"),
    (188, "Canon EF 70-200mm f/4L USM + 2x"),
    (189, "Canon EF 70-200mm f/4L USM + 2.8x"),
    (190, "Canon EF 100mm f/2.8 Macro USM"),
    (191, "Canon EF 400mm f/4 DO IS or Sigma Lens"),
    (193, "Canon EF 35-80mm f/4-5.6 USM"),
    (194, "Canon EF 80-200mm f/4.5-5.6 USM"),
    (195, "Canon EF 35-105mm f/4.5-5.6 USM"),
    (196, "Canon EF 75-300mm f/4-5.6 USM"),
    (197, "Canon EF 75-300mm f/4-5.6 IS USM or Sigma Lens"),
    (198, "Canon EF 50mm f/1.4 USM or Other Lens"),
    (199, "Canon EF 28-80mm f/3.5-5.6 USM"),
    (2, "Canon EF 28mm f/2.8 or Sigma Lens"),
    (20, "Canon EF 100-200mm f/4.5A"),
    (200, "Canon EF 75-300mm f/4-5.6 USM"),
    (201, "Canon EF 28-80mm f/3.5-5.6 USM"),
    (202, "Canon EF 28-80mm f/3.5-5.6 USM IV"),
    (208, "Canon EF 22-55mm f/4-5.6 USM"),
    (209, "Canon EF 55-200mm f/4.5-5.6"),
    (21, "Canon EF 80-200mm f/2.8L"),
    (210, "Canon EF 28-90mm f/4-5.6 USM"),
    (211, "Canon EF 28-200mm f/3.5-5.6 USM"),
    (212, "Canon EF 28-105mm f/4-5.6 USM"),
    (213, "Canon EF 90-300mm f/4.5-5.6 USM or Tamron Lens"),
    (214, "Canon EF-S 18-55mm f/3.5-5.6 USM"),
    (215, "Canon EF 55-200mm f/4.5-5.6 II USM"),
    (217, "Tamron AF 18-270mm f/3.5-6.3 Di II VC PZD"),
    (22, "Canon EF 20-35mm f/2.8L or Tokina Lens"),
    (220, "Yongnuo YN 50mm f/1.8"),
    (224, "Canon EF 70-200mm f/2.8L IS USM"),
    (225, "Canon EF 70-200mm f/2.8L IS USM + 1.4x"),
    (226, "Canon EF 70-200mm f/2.8L IS USM + 2x"),
    (227, "Canon EF 70-200mm f/2.8L IS USM + 2.8x"),
    (228, "Canon EF 28-105mm f/3.5-4.5 USM"),
    (229, "Canon EF 16-35mm f/2.8L USM"),
    (23, "Canon EF 35-105mm f/3.5-4.5"),
    (230, "Canon EF 24-70mm f/2.8L USM"),
    (231, "Canon EF 17-40mm f/4L USM or Sigma Lens"),
    (232, "Canon EF 70-300mm f/4.5-5.6 DO IS USM"),
    (233, "Canon EF 28-300mm f/3.5-5.6L IS USM"),
    (234, "Canon EF-S 17-85mm f/4-5.6 IS USM or Tokina Lens"),
    (235, "Canon EF-S 10-22mm f/3.5-4.5 USM"),
    (236, "Canon EF-S 60mm f/2.8 Macro USM"),
    (237, "Canon EF 24-105mm f/4L IS USM"),
    (238, "Canon EF 70-300mm f/4-5.6 IS USM"),
    (239, "Canon EF 85mm f/1.2L II USM or Rokinon Lens"),
    (24, "Canon EF 35-80mm f/4-5.6 Power Zoom"),
    (240, "Canon EF-S 17-55mm f/2.8 IS USM or Sigma Lens"),
    (241, "Canon EF 50mm f/1.2L USM"),
    (242, "Canon EF 70-200mm f/4L IS USM"),
    (243, "Canon EF 70-200mm f/4L IS USM + 1.4x"),
    (244, "Canon EF 70-200mm f/4L IS USM + 2x"),
    (245, "Canon EF 70-200mm f/4L IS USM + 2.8x"),
    (246, "Canon EF 16-35mm f/2.8L II USM"),
    (247, "Canon EF 14mm f/2.8L II USM"),
    (248, "Canon EF 200mm f/2L IS USM or Sigma Lens"),
    (249, "Canon EF 800mm f/5.6L IS USM"),
    (25, "Canon EF 35-80mm f/4-5.6 Power Zoom"),
    (250, "Canon EF 24mm f/1.4L II USM or Sigma Lens"),
    (251, "Canon EF 70-200mm f/2.8L IS II USM"),
    (252, "Canon EF 70-200mm f/2.8L IS II USM + 1.4x"),
    (253, "Canon EF 70-200mm f/2.8L IS II USM + 2x"),
    (254, "Canon EF 100mm f/2.8L Macro IS USM or Tamron Lens"),
    (255, "Sigma 24-105mm f/4 DG OS HSM | A or Other Lens"),
    (26, "Canon EF 100mm f/2.8 Macro or Other Lens"),
    (27, "Canon EF 35-80mm f/4-5.6"),
    (28, "Canon EF 80-200mm f/4.5-5.6 or Tamron Lens"),
    (29, "Canon EF 50mm f/1.8 II"),
    (3, "Canon EF 135mm f/2.8 Soft"),
    (30, "Canon EF 35-105mm f/4.5-5.6"),
    (31, "Canon EF 75-300mm f/4-5.6 or Tamron Lens"),
    (32, "Canon EF 24mm f/2.8 or Sigma Lens"),
    (33, "Voigtlander or Carl Zeiss Lens"),
    (35, "Canon EF 35-80mm f/4-5.6"),
    (36, "Canon EF 38-76mm f/4.5-5.6"),
    (368, "Sigma 14-24mm f/2.8 DG HSM | A or other Sigma Lens"),
    (36910, "Canon EF 70-300mm f/4-5.6 IS II USM"),
    (36912, "Canon EF-S 18-135mm f/3.5-5.6 IS USM"),
    (37, "Canon EF 35-80mm f/4-5.6 or Tamron Lens"),
    (38, "Canon EF 80-200mm f/4.5-5.6 II"),
    (39, "Canon EF 75-300mm f/4-5.6"),
    (4, "Canon EF 35-105mm f/3.5-4.5 or Sigma Lens"),
    (40, "Canon EF 28-80mm f/3.5-5.6"),
    (41, "Canon EF 28-90mm f/4-5.6"),
    (4142, "Canon EF-S 18-135mm f/3.5-5.6 IS STM"),
    (4143, "Canon EF-M 18-55mm f/3.5-5.6 IS STM or Tamron Lens"),
    (4144, "Canon EF 40mm f/2.8 STM"),
    (4145, "Canon EF-M 22mm f/2 STM"),
    (4146, "Canon EF-S 18-55mm f/3.5-5.6 IS STM"),
    (4147, "Canon EF-M 11-22mm f/4-5.6 IS STM"),
    (4148, "Canon EF-S 55-250mm f/4-5.6 IS STM"),
    (4149, "Canon EF-M 55-200mm f/4.5-6.3 IS STM"),
    (4150, "Canon EF-S 10-18mm f/4.5-5.6 IS STM"),
    (4152, "Canon EF 24-105mm f/3.5-5.6 IS STM"),
    (4153, "Canon EF-M 15-45mm f/3.5-6.3 IS STM"),
    (4154, "Canon EF-S 24mm f/2.8 STM"),
    (4155, "Canon EF-M 28mm f/3.5 Macro IS STM"),
    (4156, "Canon EF 50mm f/1.8 STM"),
    (4157, "Canon EF-M 18-150mm f/3.5-6.3 IS STM"),
    (4158, "Canon EF-S 18-55mm f/4-5.6 IS STM"),
    (4159, "Canon EF-M 32mm f/1.4 STM"),
    (4160, "Canon EF-S 35mm f/2.8 Macro IS STM"),
    (42, "Canon EF 28-200mm f/3.5-5.6 or Tamron Lens"),
    (4208, "Sigma 56mm f/1.4 DC DN | C or other Sigma Lens"),
    (43, "Canon EF 28-105mm f/4-5.6"),
    (44, "Canon EF 90-300mm f/4.5-5.6"),
    (45, "Canon EF-S 18-55mm f/3.5-5.6 [II]"),
    (46, "Canon EF 28-90mm f/4-5.6"),
    (47, "Zeiss Milvus 35mm f/2 or 50mm f/2"),
    (48, "Canon EF-S 18-55mm f/3.5-5.6 IS"),
    (488, "Canon EF-S 15-85mm f/3.5-5.6 IS USM"),
    (489, "Canon EF 70-300mm f/4-5.6L IS USM"),
    (49, "Canon EF-S 55-250mm f/4-5.6 IS"),
    (490, "Canon EF 8-15mm f/4L Fisheye USM"),
    (491, "Canon EF 300mm f/2.8L IS II USM or Tamron Lens"),
    (492, "Canon EF 400mm f/2.8L IS II USM"),
    (493, "Canon EF 500mm f/4L IS II USM or EF 24-105mm f4L IS USM"),
    (494, "Canon EF 600mm f/4L IS II USM"),
    (495, "Canon EF 24-70mm f/2.8L II USM or Sigma Lens"),
    (496, "Canon EF 200-400mm f/4L IS USM"),
    (4976, "Sigma 16-300mm F3.5-6.7 DC OS | C (025)"),
    (499, "Canon EF 200-400mm f/4L IS USM + 1.4x"),
    (5, "Canon EF 35-70mm f/3.5-4.5"),
    (50, "Canon EF-S 18-200mm f/3.5-5.6 IS"),
    (502, "Canon EF 28mm f/2.8 IS USM or Tamron Lens"),
    (503, "Canon EF 24mm f/2.8 IS USM"),
    (504, "Canon EF 24-70mm f/4L IS USM"),
    (505, "Canon EF 35mm f/2 IS USM"),
    (506, "Canon EF 400mm f/4 DO IS II USM"),
    (507, "Canon EF 16-35mm f/4L IS USM"),
    (508, "Canon EF 11-24mm f/4L USM or Tamron Lens"),
    (51, "Canon EF-S 18-135mm f/3.5-5.6 IS"),
    (52, "Canon EF-S 18-55mm f/3.5-5.6 IS II"),
    (53, "Canon EF-S 18-55mm f/3.5-5.6 III"),
    (54, "Canon EF-S 55-250mm f/4-5.6 IS II"),
    (6, "Canon EF 28-70mm f/3.5-4.5 or Sigma or Tokina Lens"),
    (60, "Irix 11mm f/4 or 15mm f/2.4"),
    (61182, "Canon RF 50mm F1.2L USM or other Canon RF Lens"),
    (61491, "Canon CN-E 14mm T3.1 L F"),
    (61492, "Canon CN-E 24mm T1.5 L F"),
    (61494, "Canon CN-E 85mm T1.3 L F"),
    (61495, "Canon CN-E 135mm T2.2 L F"),
    (61496, "Canon CN-E 35mm T1.5 L F"),
    (624, "Sigma 70-200mm f/2.8 DG OS HSM | S or other Sigma Lens"),
    (63, "Irix 30mm F1.4 Dragonfly"),
    (6512, "Sigma 12mm F1.4 DC | C"),
    (65535, "n/a"),
    (7, "Canon EF 100-300mm f/5.6L"),
    (747, "Canon EF 100-400mm f/4.5-5.6L IS II USM or Tamron Lens"),
    (748, "Canon EF 100-400mm f/4.5-5.6L IS II USM + 1.4x or Tamron Lens"),
    (749, "Canon EF 100-400mm f/4.5-5.6L IS II USM + 2x or Tamron Lens"),
    (750, "Canon EF 35mm f/1.4L II USM or Tamron Lens"),
    (751, "Canon EF 16-35mm f/2.8L III USM"),
    (752, "Canon EF 24-105mm f/4L IS II USM"),
    (753, "Canon EF 85mm f/1.4L IS USM"),
    (754, "Canon EF 70-200mm f/4L IS II USM"),
    (757, "Canon EF 400mm f/2.8L IS III USM"),
    (758, "Canon EF 600mm f/4L IS III USM"),
    (8, "Canon EF 100-300mm f/5.6 or Sigma or Tokina Lens"),
    (80, "Canon TS-E 50mm f/2.8L Macro"),
    (81, "Canon TS-E 90mm f/2.8L Macro"),
    (82, "Canon TS-E 135mm f/4L Macro"),
    (9, "Canon EF 70-210mm f/4"),
    (94, "Canon TS-E 17mm f/4L"),
    (95, "Canon TS-E 24mm f/3.5L II"),
];

pub static CANON_CAMERASETTINGS_FLASHBITS_VALUES: &[(i64, &str)] = &[
    (0, "(none)"),
];

pub static CANON_CAMERASETTINGS_QUALITY_VALUES: &[(i64, &str)] = &[
    (-1, "n/a"),
    (1, "Economy"),
    (130, "Light (RAW)"),
    (131, "Standard (RAW)"),
    (2, "Normal"),
    (3, "Fine"),
    (4, "RAW"),
    (5, "Superfine"),
    (7, "CRAW"),
];

pub static CANON_CAMERASETTINGS_FOCUSCONTINUOUS_VALUES: &[(i64, &str)] = &[
    (0, "Single"),
    (1, "Continuous"),
    (8, "Manual"),
];

pub static CANON_CAMERASETTINGS_AESETTING_VALUES: &[(i64, &str)] = &[
    (0, "Normal AE"),
    (1, "Exposure Compensation"),
    (2, "AE Lock"),
    (3, "AE Lock + Exposure Comp."),
    (4, "No AE"),
];

pub static CANON_CAMERASETTINGS_IMAGESTABILIZATION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
    (2, "Shoot Only"),
    (256, "Off (2)"),
    (257, "On (2)"),
    (258, "Shoot Only (2)"),
    (259, "Panning (2)"),
    (260, "Dynamic (2)"),
    (3, "Panning"),
    (4, "Dynamic"),
];

pub static CANON_CAMERASETTINGS_SPOTMETERINGMODE_VALUES: &[(i64, &str)] = &[
    (0, "Center"),
    (1, "AF Point"),
];

pub static CANON_CAMERASETTINGS_CANONFLASHMODE_VALUES: &[(i64, &str)] = &[
    (-1, "n/a"),
    (0, "Off"),
    (1, "Auto"),
    (16, "External flash"),
    (2, "On"),
    (3, "Red-eye reduction"),
    (4, "Slow-sync"),
    (5, "Red-eye reduction (Auto)"),
    (6, "Red-eye reduction (On)"),
];

pub static CANON_CAMERASETTINGS_PHOTOEFFECT_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Vivid"),
    (100, "My Color Data"),
    (2, "Neutral"),
    (3, "Smooth"),
    (4, "Sepia"),
    (5, "B&W"),
    (6, "Custom"),
];

pub static CANON_CAMERASETTINGS_MANUALFLASHOUTPUT_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1280, "Full"),
    (1282, "Medium"),
    (1284, "Low"),
    (32767, "n/a"),
];

pub static CANON_CAMERASETTINGS_COLORTONE_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
];

pub static CANON_CAMERASETTINGS_SRAWQUALITY_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "sRAW1 (mRAW)"),
    (2, "sRAW2 (sRAW)"),
];

pub static CANON_CAMERASETTINGS_CONTINUOUSDRIVE_VALUES: &[(i64, &str)] = &[
    (0, "Single"),
    (1, "Continuous"),
    (10, "Continuous, Silent"),
    (2, "Movie"),
    (3, "Continuous, Speed Priority"),
    (4, "Continuous, Low"),
    (5, "Continuous, High"),
    (6, "Silent Single"),
    (8, "Continuous, High+"),
    (9, "Single, Silent"),
];

pub static CANON_CAMERASETTINGS_FOCUSBRACKETING_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Enable"),
];

pub static CANON_CAMERASETTINGS_CLARITY_VALUES: &[(i64, &str)] = &[
    (32767, "n/a"),
];

pub static CANON_CAMERASETTINGS_HDR_PQ_VALUES: &[(i64, &str)] = &[
    (-1, "n/a"),
    (0, "Off"),
    (1, "On"),
];

pub static CANON_CAMERASETTINGS_FOCUSMODE_VALUES: &[(i64, &str)] = &[
    (0, "One-shot AF"),
    (1, "AI Servo AF"),
    (16, "Pan Focus"),
    (2, "AI Focus AF"),
    (256, "One-shot AF (Live View)"),
    (257, "AI Servo AF (Live View)"),
    (258, "AI Focus AF (Live View)"),
    (3, "Manual Focus (3)"),
    (4, "Single"),
    (5, "Continuous"),
    (512, "Movie Snap Focus"),
    (519, "Movie Servo AF"),
    (6, "Manual Focus (6)"),
];

pub static CANON_CAMERASETTINGS_RECORDMODE_VALUES: &[(i64, &str)] = &[
    (1, "JPEG"),
    (10, "MP4"),
    (11, "CRM"),
    (12, "CR3"),
    (13, "CR3+JPEG"),
    (14, "HIF"),
    (15, "CR3+HIF"),
    (2, "CRW+THM"),
    (3, "AVI+THM"),
    (4, "TIF"),
    (5, "TIF+JPEG"),
    (6, "CR2"),
    (7, "CR2+JPEG"),
    (9, "MOV"),
];

/// Canon::ColorBalance tags
pub static CANON_COLORBALANCE: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "WB_RGGBLevelsAuto", values: None },
    13u16 => TagDef { name: "WB_RGGBLevelsCloudy", values: None },
    17u16 => TagDef { name: "WB_RGGBLevelsTungsten", values: None },
    21u16 => TagDef { name: "WB_RGGBLevelsFluorescent", values: None },
    25u16 => TagDef { name: "WB_RGGBLevelsFlash", values: None },
    29u16 => TagDef { name: "WB_RGGBLevelsCustom", values: None },
    33u16 => TagDef { name: "WB_RGGBLevelsKelvin", values: None },
    37u16 => TagDef { name: "WB_RGGBBlackLevels", values: None },
    5u16 => TagDef { name: "WB_RGGBLevelsDaylight", values: None },
    9u16 => TagDef { name: "WB_RGGBLevelsShade", values: None },
};

/// Canon::ColorCalib tags
pub static CANON_COLORCALIB: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "CameraColorCalibration01", values: None },
    12u16 => TagDef { name: "CameraColorCalibration04", values: None },
    16u16 => TagDef { name: "CameraColorCalibration05", values: None },
    20u16 => TagDef { name: "CameraColorCalibration06", values: None },
    24u16 => TagDef { name: "CameraColorCalibration07", values: None },
    28u16 => TagDef { name: "CameraColorCalibration08", values: None },
    32u16 => TagDef { name: "CameraColorCalibration09", values: None },
    36u16 => TagDef { name: "CameraColorCalibration10", values: None },
    4u16 => TagDef { name: "CameraColorCalibration02", values: None },
    40u16 => TagDef { name: "CameraColorCalibration11", values: None },
    44u16 => TagDef { name: "CameraColorCalibration12", values: None },
    48u16 => TagDef { name: "CameraColorCalibration13", values: None },
    52u16 => TagDef { name: "CameraColorCalibration14", values: None },
    56u16 => TagDef { name: "CameraColorCalibration15", values: None },
    8u16 => TagDef { name: "CameraColorCalibration03", values: None },
};

/// Canon::ColorData1 tags
pub static CANON_COLORDATA1: phf::Map<u16, TagDef> = phf::phf_map! {
    25u16 => TagDef { name: "WB_RGGBLevelsAsShot", values: None },
    30u16 => TagDef { name: "WB_RGGBLevelsAuto", values: None },
    35u16 => TagDef { name: "WB_RGGBLevelsDaylight", values: None },
    40u16 => TagDef { name: "WB_RGGBLevelsShade", values: None },
    45u16 => TagDef { name: "WB_RGGBLevelsCloudy", values: None },
    50u16 => TagDef { name: "WB_RGGBLevelsTungsten", values: None },
    55u16 => TagDef { name: "WB_RGGBLevelsFluorescent", values: None },
    60u16 => TagDef { name: "WB_RGGBLevelsFlash", values: None },
    65u16 => TagDef { name: "WB_RGGBLevelsCustom1", values: None },
    70u16 => TagDef { name: "WB_RGGBLevelsCustom2", values: None },
    75u16 => TagDef { name: "ColorCalib", values: None },
};

/// Canon::ColorInfo tags
pub static CANON_COLORINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "Saturation", values: Some(CANON_COLORINFO_SATURATION_VALUES) },
    2u16 => TagDef { name: "ColorTone", values: Some(CANON_COLORINFO_COLORTONE_VALUES) },
    3u16 => TagDef { name: "ColorSpace", values: Some(CANON_COLORINFO_COLORSPACE_VALUES) },
};

pub static CANON_COLORINFO_SATURATION_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
];

pub static CANON_COLORINFO_COLORTONE_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
];

pub static CANON_COLORINFO_COLORSPACE_VALUES: &[(i64, &str)] = &[
    (1, "sRGB"),
    (2, "Adobe RGB"),
];

/// Canon::ContrastInfo tags
pub static CANON_CONTRASTINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    4u16 => TagDef { name: "IntelligentContrast", values: Some(CANON_CONTRASTINFO_INTELLIGENTCONTRAST_VALUES) },
};

pub static CANON_CONTRASTINFO_INTELLIGENTCONTRAST_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (65535, "n/a"),
    (8, "On"),
];

/// Canon::FaceDetect1 tags
pub static CANON_FACEDETECT1: phf::Map<u16, TagDef> = phf::phf_map! {
    10u16 => TagDef { name: "Face2Position", values: None },
    12u16 => TagDef { name: "Face3Position", values: None },
    14u16 => TagDef { name: "Face4Position", values: None },
    16u16 => TagDef { name: "Face5Position", values: None },
    18u16 => TagDef { name: "Face6Position", values: None },
    2u16 => TagDef { name: "FacesDetected", values: None },
    20u16 => TagDef { name: "Face7Position", values: None },
    22u16 => TagDef { name: "Face8Position", values: None },
    24u16 => TagDef { name: "Face9Position", values: None },
    3u16 => TagDef { name: "FaceDetectFrameSize", values: None },
    8u16 => TagDef { name: "Face1Position", values: None },
};

/// Canon::FileInfo tags
pub static CANON_FILEINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "FileNumber", values: None },
    14u16 => TagDef { name: "FilterEffect", values: Some(CANON_FILEINFO_FILTEREFFECT_VALUES) },
    15u16 => TagDef { name: "ToningEffect", values: Some(CANON_FILEINFO_TONINGEFFECT_VALUES) },
    16u16 => TagDef { name: "MacroMagnification", values: None },
    19u16 => TagDef { name: "LiveViewShooting", values: Some(CANON_FILEINFO_LIVEVIEWSHOOTING_VALUES) },
    20u16 => TagDef { name: "FocusDistanceUpper", values: None },
    21u16 => TagDef { name: "FocusDistanceLower", values: None },
    23u16 => TagDef { name: "ShutterMode", values: Some(CANON_FILEINFO_SHUTTERMODE_VALUES) },
    25u16 => TagDef { name: "FlashExposureLock", values: Some(CANON_FILEINFO_FLASHEXPOSURELOCK_VALUES) },
    3u16 => TagDef { name: "BracketMode", values: Some(CANON_FILEINFO_BRACKETMODE_VALUES) },
    32u16 => TagDef { name: "AntiFlicker", values: Some(CANON_FILEINFO_ANTIFLICKER_VALUES) },
    6u16 => TagDef { name: "RawJpgQuality", values: Some(CANON_FILEINFO_RAWJPGQUALITY_VALUES) },
    61u16 => TagDef { name: "RFLensType", values: Some(CANON_FILEINFO_RFLENSTYPE_VALUES) },
    7u16 => TagDef { name: "RawJpgSize", values: Some(CANON_FILEINFO_RAWJPGSIZE_VALUES) },
    8u16 => TagDef { name: "LongExposureNoiseReduction2", values: Some(CANON_FILEINFO_LONGEXPOSURENOISEREDUCTION2_VALUES) },
    9u16 => TagDef { name: "WBBracketMode", values: Some(CANON_FILEINFO_WBBRACKETMODE_VALUES) },
};

pub static CANON_FILEINFO_FILTEREFFECT_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "Yellow"),
    (2, "Orange"),
    (3, "Red"),
    (4, "Green"),
];

pub static CANON_FILEINFO_TONINGEFFECT_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "Sepia"),
    (2, "Blue"),
    (3, "Purple"),
    (4, "Green"),
];

pub static CANON_FILEINFO_LIVEVIEWSHOOTING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANON_FILEINFO_SHUTTERMODE_VALUES: &[(i64, &str)] = &[
    (0, "Mechanical"),
    (1, "Electronic First Curtain"),
    (2, "Electronic"),
];

pub static CANON_FILEINFO_FLASHEXPOSURELOCK_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANON_FILEINFO_BRACKETMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "AEB"),
    (2, "FEB"),
    (3, "ISO"),
    (4, "WB"),
];

pub static CANON_FILEINFO_ANTIFLICKER_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANON_FILEINFO_RAWJPGQUALITY_VALUES: &[(i64, &str)] = &[
    (-1, "n/a"),
    (1, "Economy"),
    (130, "Light (RAW)"),
    (131, "Standard (RAW)"),
    (2, "Normal"),
    (3, "Fine"),
    (4, "RAW"),
    (5, "Superfine"),
    (7, "CRAW"),
];

pub static CANON_FILEINFO_RFLENSTYPE_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (257, "Canon RF 50mm F1.2L USM"),
    (258, "Canon RF 24-105mm F4L IS USM"),
    (259, "Canon RF 28-70mm F2L USM"),
    (260, "Canon RF 35mm F1.8 MACRO IS STM"),
    (261, "Canon RF 85mm F1.2L USM"),
    (262, "Canon RF 85mm F1.2L USM DS"),
    (263, "Canon RF 24-70mm F2.8L IS USM"),
    (264, "Canon RF 15-35mm F2.8L IS USM"),
    (265, "Canon RF 24-240mm F4-6.3 IS USM"),
    (266, "Canon RF 70-200mm F2.8L IS USM"),
    (267, "Canon RF 85mm F2 MACRO IS STM"),
    (268, "Canon RF 600mm F11 IS STM"),
    (269, "Canon RF 600mm F11 IS STM + RF1.4x"),
    (270, "Canon RF 600mm F11 IS STM + RF2x"),
    (271, "Canon RF 800mm F11 IS STM"),
    (272, "Canon RF 800mm F11 IS STM + RF1.4x"),
    (273, "Canon RF 800mm F11 IS STM + RF2x"),
    (274, "Canon RF 24-105mm F4-7.1 IS STM"),
    (275, "Canon RF 100-500mm F4.5-7.1L IS USM"),
    (276, "Canon RF 100-500mm F4.5-7.1L IS USM + RF1.4x"),
    (277, "Canon RF 100-500mm F4.5-7.1L IS USM + RF2x"),
    (278, "Canon RF 70-200mm F4L IS USM"),
    (279, "Canon RF 100mm F2.8L MACRO IS USM"),
    (280, "Canon RF 50mm F1.8 STM"),
    (281, "Canon RF 14-35mm F4L IS USM"),
    (282, "Canon RF-S 18-45mm F4.5-6.3 IS STM"),
    (283, "Canon RF 100-400mm F5.6-8 IS USM"),
    (284, "Canon RF 100-400mm F5.6-8 IS USM + RF1.4x"),
    (285, "Canon RF 100-400mm F5.6-8 IS USM + RF2x"),
    (286, "Canon RF-S 18-150mm F3.5-6.3 IS STM"),
    (287, "Canon RF 24mm F1.8 MACRO IS STM"),
    (288, "Canon RF 16mm F2.8 STM"),
    (289, "Canon RF 400mm F2.8L IS USM"),
    (290, "Canon RF 400mm F2.8L IS USM + RF1.4x"),
    (291, "Canon RF 400mm F2.8L IS USM + RF2x"),
    (292, "Canon RF 600mm F4L IS USM"),
    (293, "Canon RF 600mm F4L IS USM + RF1.4x"),
    (294, "Canon RF 600mm F4L IS USM + RF2x"),
    (295, "Canon RF 800mm F5.6L IS USM"),
    (296, "Canon RF 800mm F5.6L IS USM + RF1.4x"),
    (297, "Canon RF 800mm F5.6L IS USM + RF2x"),
    (298, "Canon RF 1200mm F8L IS USM"),
    (299, "Canon RF 1200mm F8L IS USM + RF1.4x"),
    (300, "Canon RF 1200mm F8L IS USM + RF2x"),
    (301, "Canon RF 5.2mm F2.8L Dual Fisheye 3D VR"),
    (302, "Canon RF 15-30mm F4.5-6.3 IS STM"),
    (303, "Canon RF 135mm F1.8 L IS USM"),
    (304, "Canon RF 24-50mm F4.5-6.3 IS STM"),
    (305, "Canon RF-S 55-210mm F5-7.1 IS STM"),
    (306, "Canon RF 100-300mm F2.8L IS USM"),
    (307, "Canon RF 100-300mm F2.8L IS USM + RF1.4x"),
    (308, "Canon RF 100-300mm F2.8L IS USM + RF2x"),
    (309, "Canon RF 200-800mm F6.3-9 IS USM"),
    (310, "Canon RF 200-800mm F6.3-9 IS USM + RF1.4x"),
    (311, "Canon RF 200-800mm F6.3-9 IS USM + RF2x"),
    (312, "Canon RF 10-20mm F4 L IS STM"),
    (313, "Canon RF 28mm F2.8 STM"),
    (314, "Canon RF 24-105mm F2.8 L IS USM Z"),
    (315, "Canon RF-S 10-18mm F4.5-6.3 IS STM"),
    (316, "Canon RF 35mm F1.4 L VCM"),
    (317, "Canon RF-S 3.9mm F3.5 STM DUAL FISHEYE"),
    (318, "Canon RF 28-70mm F2.8 IS STM"),
    (319, "Canon RF 70-200mm F2.8 L IS USM Z"),
    (320, "Canon RF 70-200mm F2.8 L IS USM Z + RF1.4x"),
    (321, "Canon RF 70-200mm F2.8 L IS USM Z + RF2x"),
    (323, "Canon RF 16-28mm F2.8 IS STM"),
    (324, "Canon RF-S 14-30mm F4-6.3 IS STM PZ"),
    (325, "Canon RF 50mm F1.4 L VCM"),
    (326, "Canon RF 24mm F1.4 L VCM"),
    (327, "Canon RF 20mm F1.4 L VCM"),
    (328, "Canon RF 85mm F1.4 L VCM"),
    (330, "Canon RF 45mm F1.2 STM"),
];

pub static CANON_FILEINFO_RAWJPGSIZE_VALUES: &[(i64, &str)] = &[
    (-1, "n/a"),
    (0, "Large"),
    (1, "Medium"),
    (10, "Medium Widescreen"),
    (128, "640x480 Movie"),
    (129, "Medium Movie"),
    (130, "Small Movie"),
    (137, "1280x720 Movie"),
    (14, "Small 1"),
    (142, "1920x1080 Movie"),
    (143, "4096x2160 Movie"),
    (15, "Small 2"),
    (16, "Small 3"),
    (2, "Small"),
    (5, "Medium 1"),
    (6, "Medium 2"),
    (7, "Medium 3"),
    (8, "Postcard"),
    (9, "Widescreen"),
];

pub static CANON_FILEINFO_LONGEXPOSURENOISEREDUCTION2_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On (1D)"),
    (3, "On"),
    (4, "Auto"),
];

pub static CANON_FILEINFO_WBBRACKETMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On (shift AB)"),
    (2, "On (shift GM)"),
];

/// Canon::FilterInfo tags
pub static CANON_FILTERINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    1025u16 => TagDef { name: "MiniatureFilter", values: Some(CANON_FILTERINFO_MINIATUREFILTER_VALUES) },
    1026u16 => TagDef { name: "MiniatureFilterOrientation", values: Some(CANON_FILTERINFO_MINIATUREFILTERORIENTATION_VALUES) },
    1281u16 => TagDef { name: "FisheyeFilter", values: Some(CANON_FILTERINFO_FISHEYEFILTER_VALUES) },
    1537u16 => TagDef { name: "PaintingFilter", values: Some(CANON_FILTERINFO_PAINTINGFILTER_VALUES) },
    1793u16 => TagDef { name: "WatercolorFilter", values: Some(CANON_FILTERINFO_WATERCOLORFILTER_VALUES) },
    257u16 => TagDef { name: "GrainyBWFilter", values: Some(CANON_FILTERINFO_GRAINYBWFILTER_VALUES) },
    513u16 => TagDef { name: "SoftFocusFilter", values: Some(CANON_FILTERINFO_SOFTFOCUSFILTER_VALUES) },
    769u16 => TagDef { name: "ToyCameraFilter", values: Some(CANON_FILTERINFO_TOYCAMERAFILTER_VALUES) },
};

pub static CANON_FILTERINFO_MINIATUREFILTER_VALUES: &[(i64, &str)] = &[
    (-1, "Off"),
];

pub static CANON_FILTERINFO_MINIATUREFILTERORIENTATION_VALUES: &[(i64, &str)] = &[
    (0, "Horizontal"),
    (1, "Vertical"),
];

pub static CANON_FILTERINFO_FISHEYEFILTER_VALUES: &[(i64, &str)] = &[
    (-1, "Off"),
];

pub static CANON_FILTERINFO_PAINTINGFILTER_VALUES: &[(i64, &str)] = &[
    (-1, "Off"),
];

pub static CANON_FILTERINFO_WATERCOLORFILTER_VALUES: &[(i64, &str)] = &[
    (-1, "Off"),
];

pub static CANON_FILTERINFO_GRAINYBWFILTER_VALUES: &[(i64, &str)] = &[
    (-1, "Off"),
];

pub static CANON_FILTERINFO_SOFTFOCUSFILTER_VALUES: &[(i64, &str)] = &[
    (-1, "Off"),
];

pub static CANON_FILTERINFO_TOYCAMERAFILTER_VALUES: &[(i64, &str)] = &[
    (-1, "Off"),
];

/// Canon::FocalLength tags
pub static CANON_FOCALLENGTH: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "FocalType", values: Some(CANON_FOCALLENGTH_FOCALTYPE_VALUES) },
    1u16 => TagDef { name: "FocalLength", values: None },
    2u16 => TagDef { name: "FocalPlaneXSize", values: None },
    3u16 => TagDef { name: "FocalPlaneYSize", values: None },
};

pub static CANON_FOCALLENGTH_FOCALTYPE_VALUES: &[(i64, &str)] = &[
    (1, "Fixed"),
    (2, "Zoom"),
];

/// Canon::HDRInfo tags
pub static CANON_HDRINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "HDR", values: Some(CANON_HDRINFO_HDR_VALUES) },
    2u16 => TagDef { name: "HDREffect", values: Some(CANON_HDRINFO_HDREFFECT_VALUES) },
};

pub static CANON_HDRINFO_HDR_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Auto"),
    (2, "On"),
];

pub static CANON_HDRINFO_HDREFFECT_VALUES: &[(i64, &str)] = &[
    (0, "Natural"),
    (1, "Art (standard)"),
    (2, "Art (vivid)"),
    (3, "Art (bold)"),
    (4, "Art (embossed)"),
];

/// Canon::LensInfo tags
pub static CANON_LENSINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "LensSerialNumber", values: None },
};

/// Canon::LevelInfo tags
pub static CANON_LEVELINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    4u16 => TagDef { name: "RollAngle", values: None },
    5u16 => TagDef { name: "PitchAngle", values: None },
    7u16 => TagDef { name: "FocalLength", values: None },
    8u16 => TagDef { name: "MinFocalLength2", values: None },
    9u16 => TagDef { name: "MaxFocalLength2", values: None },
};

/// Canon::LightingOpt tags
pub static CANON_LIGHTINGOPT: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "PeripheralIlluminationCorr", values: Some(CANON_LIGHTINGOPT_PERIPHERALILLUMINATIONCORR_VALUES) },
    10u16 => TagDef { name: "DigitalLensOptimizer", values: Some(CANON_LIGHTINGOPT_DIGITALLENSOPTIMIZER_VALUES) },
    11u16 => TagDef { name: "DualPixelRaw", values: Some(CANON_LIGHTINGOPT_DUALPIXELRAW_VALUES) },
    2u16 => TagDef { name: "AutoLightingOptimizer", values: Some(CANON_LIGHTINGOPT_AUTOLIGHTINGOPTIMIZER_VALUES) },
    3u16 => TagDef { name: "HighlightTonePriority", values: Some(CANON_LIGHTINGOPT_HIGHLIGHTTONEPRIORITY_VALUES) },
    4u16 => TagDef { name: "LongExposureNoiseReduction", values: Some(CANON_LIGHTINGOPT_LONGEXPOSURENOISEREDUCTION_VALUES) },
    5u16 => TagDef { name: "HighISONoiseReduction", values: Some(CANON_LIGHTINGOPT_HIGHISONOISEREDUCTION_VALUES) },
};

pub static CANON_LIGHTINGOPT_PERIPHERALILLUMINATIONCORR_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANON_LIGHTINGOPT_DIGITALLENSOPTIMIZER_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Standard"),
    (2, "High"),
];

pub static CANON_LIGHTINGOPT_DUALPIXELRAW_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANON_LIGHTINGOPT_AUTOLIGHTINGOPTIMIZER_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (1, "Low"),
    (2, "Strong"),
    (3, "Off"),
];

pub static CANON_LIGHTINGOPT_HIGHLIGHTTONEPRIORITY_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
    (2, "Enhanced"),
];

pub static CANON_LIGHTINGOPT_LONGEXPOSURENOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Auto"),
    (2, "On"),
];

pub static CANON_LIGHTINGOPT_HIGHISONOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (1, "Low"),
    (2, "Strong"),
    (3, "Off"),
];

/// Canon::LogInfo tags
pub static CANON_LOGINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    10u16 => TagDef { name: "ColorMatrix", values: Some(CANON_LOGINFO_COLORMATRIX_VALUES) },
    11u16 => TagDef { name: "CanonLogVersion", values: Some(CANON_LOGINFO_CANONLOGVERSION_VALUES) },
    4u16 => TagDef { name: "CompressionFormat", values: Some(CANON_LOGINFO_COMPRESSIONFORMAT_VALUES) },
    6u16 => TagDef { name: "Sharpness", values: None },
    7u16 => TagDef { name: "Saturation", values: Some(CANON_LOGINFO_SATURATION_VALUES) },
    8u16 => TagDef { name: "ColorTone", values: Some(CANON_LOGINFO_COLORTONE_VALUES) },
    9u16 => TagDef { name: "ColorSpace2", values: Some(CANON_LOGINFO_COLORSPACE2_VALUES) },
};

pub static CANON_LOGINFO_COLORMATRIX_VALUES: &[(i64, &str)] = &[
    (0, "EOS Original"),
    (1, "Neutral"),
];

pub static CANON_LOGINFO_CANONLOGVERSION_VALUES: &[(i64, &str)] = &[
    (0, "OFF"),
    (1, "CLogV1"),
    (2, "CLogV2"),
    (3, "CLogV3"),
];

pub static CANON_LOGINFO_COMPRESSIONFORMAT_VALUES: &[(i64, &str)] = &[
    (0, "Editing (ALL-I)"),
    (1, "Standard (IPB)"),
    (2, "Light (IPB)"),
    (3, "Motion JPEG"),
    (4, "RAW"),
];

pub static CANON_LOGINFO_SATURATION_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
];

pub static CANON_LOGINFO_COLORTONE_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
];

pub static CANON_LOGINFO_COLORSPACE2_VALUES: &[(i64, &str)] = &[
    (0, "BT.709"),
    (1, "BT.2020"),
    (2, "CinemaGamut"),
];

/// Canon::Main tags
pub static CANON_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "CanonCameraSettings", values: None },
    10u16 => TagDef { name: "UnknownD30", values: None },
    12u16 => TagDef { name: "SerialNumber", values: None },
    129u16 => TagDef { name: "RawDataOffset", values: None },
    13u16 => TagDef { name: "CanonCameraInfo1D", values: None },
    130u16 => TagDef { name: "RawDataLength", values: None },
    131u16 => TagDef { name: "OriginalDecisionDataOffset", values: None },
    14u16 => TagDef { name: "CanonFileLength", values: None },
    144u16 => TagDef { name: "CustomFunctions1D", values: None },
    145u16 => TagDef { name: "PersonalFunctions", values: None },
    146u16 => TagDef { name: "PersonalFunctionValues", values: None },
    147u16 => TagDef { name: "CanonFileInfo", values: None },
    148u16 => TagDef { name: "AFPointsInFocus1D", values: None },
    149u16 => TagDef { name: "LensModel", values: None },
    15u16 => TagDef { name: "CustomFunctions1D", values: None },
    150u16 => TagDef { name: "SerialInfo", values: None },
    151u16 => TagDef { name: "DustRemovalData", values: None },
    152u16 => TagDef { name: "CropInfo", values: None },
    153u16 => TagDef { name: "CustomFunctions2", values: None },
    154u16 => TagDef { name: "AspectInfo", values: None },
    16u16 => TagDef { name: "CanonModelID", values: Some(CANON_MAIN_CANONMODELID_VALUES) },
    160u16 => TagDef { name: "ProcessingInfo", values: None },
    161u16 => TagDef { name: "ToneCurveTable", values: None },
    162u16 => TagDef { name: "SharpnessTable", values: None },
    163u16 => TagDef { name: "SharpnessFreqTable", values: None },
    16385u16 => TagDef { name: "ColorData1", values: None },
    16386u16 => TagDef { name: "CRWParam", values: None },
    16387u16 => TagDef { name: "ColorInfo", values: None },
    16389u16 => TagDef { name: "Flavor", values: None },
    16392u16 => TagDef { name: "PictureStyleUserDef", values: None },
    16393u16 => TagDef { name: "PictureStylePC", values: None },
    164u16 => TagDef { name: "WhiteBalanceTable", values: None },
    16400u16 => TagDef { name: "CustomPictureStyleFileName", values: None },
    16403u16 => TagDef { name: "AFMicroAdj", values: None },
    16405u16 => TagDef { name: "VignettingCorr", values: None },
    16406u16 => TagDef { name: "VignettingCorr2", values: None },
    16408u16 => TagDef { name: "LightingOpt", values: None },
    16409u16 => TagDef { name: "LensInfo", values: None },
    16416u16 => TagDef { name: "AmbienceInfo", values: None },
    16417u16 => TagDef { name: "MultiExp", values: None },
    16420u16 => TagDef { name: "FilterInfo", values: None },
    16421u16 => TagDef { name: "HDRInfo", values: None },
    16422u16 => TagDef { name: "LogInfo", values: None },
    16424u16 => TagDef { name: "AFConfig", values: None },
    16447u16 => TagDef { name: "RawBurstModeRoll", values: None },
    16473u16 => TagDef { name: "LevelInfo", values: None },
    169u16 => TagDef { name: "ColorBalance", values: None },
    17u16 => TagDef { name: "MovieInfo", values: None },
    170u16 => TagDef { name: "MeasuredColor", values: None },
    174u16 => TagDef { name: "ColorTemperature", values: None },
    176u16 => TagDef { name: "CanonFlags", values: None },
    177u16 => TagDef { name: "ModifiedInfo", values: None },
    178u16 => TagDef { name: "ToneCurveMatching", values: None },
    179u16 => TagDef { name: "WhiteBalanceMatching", values: None },
    18u16 => TagDef { name: "CanonAFInfo", values: None },
    180u16 => TagDef { name: "ColorSpace", values: Some(CANON_MAIN_COLORSPACE_VALUES) },
    182u16 => TagDef { name: "PreviewImageInfo", values: None },
    19u16 => TagDef { name: "ThumbnailImageValidArea", values: None },
    2u16 => TagDef { name: "CanonFocalLength", values: None },
    208u16 => TagDef { name: "VRDOffset", values: None },
    21u16 => TagDef { name: "SerialNumberFormat", values: Some(CANON_MAIN_SERIALNUMBERFORMAT_VALUES) },
    224u16 => TagDef { name: "SensorInfo", values: None },
    26u16 => TagDef { name: "SuperMacro", values: Some(CANON_MAIN_SUPERMACRO_VALUES) },
    28u16 => TagDef { name: "DateStampMode", values: Some(CANON_MAIN_DATESTAMPMODE_VALUES) },
    29u16 => TagDef { name: "MyColors", values: None },
    3u16 => TagDef { name: "CanonFlashInfo", values: None },
    30u16 => TagDef { name: "FirmwareRevision", values: None },
    35u16 => TagDef { name: "Categories", values: Some(CANON_MAIN_CATEGORIES_VALUES) },
    36u16 => TagDef { name: "FaceDetect1", values: None },
    37u16 => TagDef { name: "FaceDetect2", values: None },
    38u16 => TagDef { name: "CanonAFInfo2", values: None },
    39u16 => TagDef { name: "ContrastInfo", values: None },
    4u16 => TagDef { name: "CanonShotInfo", values: None },
    40u16 => TagDef { name: "ImageUniqueID", values: None },
    41u16 => TagDef { name: "WBInfo", values: None },
    47u16 => TagDef { name: "FaceDetect3", values: None },
    5u16 => TagDef { name: "CanonPanorama", values: None },
    53u16 => TagDef { name: "TimeInfo", values: None },
    56u16 => TagDef { name: "BatteryType", values: None },
    6u16 => TagDef { name: "CanonImageType", values: None },
    60u16 => TagDef { name: "AFInfo3", values: None },
    7u16 => TagDef { name: "CanonFirmwareVersion", values: None },
    8u16 => TagDef { name: "FileNumber", values: None },
    9u16 => TagDef { name: "OwnerName", values: None },
};

pub static CANON_MAIN_CANONMODELID_VALUES: &[(i64, &str)] = &[
    (100925440, "PowerShot S100 / Digital IXUS / IXY Digital"),
    (1042, "EOS M50 / Kiss M"),
    (1073742375, "EOS C50"),
    (1074255475, "DC19/DC21/DC22"),
    (1074255476, "XH A1"),
    (1074255477, "HV10"),
    (1074255478, "MD130/MD140/MD150/MD160/ZR850"),
    (1074255735, "DC50"),
    (1074255736, "HV20"),
    (1074255737, "DC211"),
    (1074255738, "HG10"),
    (1074255739, "HR10"),
    (1074255741, "MD255/ZR950"),
    (1074255900, "HF11"),
    (1074255992, "HV30"),
    (1074255996, "XH A1S"),
    (1074255998, "DC301/DC310/DC311/DC320/DC330"),
    (1074255999, "FS100"),
    (1074256000, "HF10"),
    (1074256002, "HG20/HG21"),
    (1074256165, "HF21"),
    (1074256166, "HF S11"),
    (1074256248, "HV40"),
    (1074256263, "DC410/DC411/DC420"),
    (1074256264, "FS19/FS20/FS21/FS22/FS200"),
    (1074256265, "HF20/HF200"),
    (1074256266, "HF S10/S100"),
    (1074256526, "HF R10/R16/R17/R18/R100/R106"),
    (1074256527, "HF M30/M31/M36/M300/M306"),
    (1074256528, "HF S20/S21/S200"),
    (1074256530, "FS31/FS36/FS37/FS300/FS305/FS306/FS307"),
    (1074257056, "EOS C300"),
    (1074257321, "HF G25"),
    (1074257844, "XC10"),
    (1074258371, "EOS C200"),
    (16842752, "PowerShot A30"),
    (17039360, "PowerShot S300 / Digital IXUS 300 / IXY Digital 300"),
    (17170432, "PowerShot A20"),
    (17301504, "PowerShot A10"),
    (17367040, "PowerShot S110 / Digital IXUS v / IXY Digital 200"),
    (17825792, "PowerShot G2"),
    (17891328, "PowerShot S40"),
    (17956864, "PowerShot S30"),
    (18022400, "PowerShot A40"),
    (18087936, "EOS D30"),
    (18153472, "PowerShot A100"),
    (18219008, "PowerShot S200 / Digital IXUS v2 / IXY Digital 200a"),
    (18284544, "PowerShot A200"),
    (18350080, "PowerShot S330 / Digital IXUS 330 / IXY Digital 300a"),
    (18415616, "PowerShot G3"),
    (18939904, "PowerShot S45"),
    (19070976, "PowerShot SD100 / Digital IXUS II / IXY Digital 30"),
    (19136512, "PowerShot S230 / Digital IXUS v3 / IXY Digital 320"),
    (19202048, "PowerShot A70"),
    (19267584, "PowerShot A60"),
    (19333120, "PowerShot S400 / Digital IXUS 400 / IXY Digital 400"),
    (19464192, "PowerShot G5"),
    (19922944, "PowerShot A300"),
    (19988480, "PowerShot S50"),
    (20185088, "PowerShot A80"),
    (20250624, "PowerShot SD10 / Digital IXUS i / IXY Digital L"),
    (20316160, "PowerShot S1 IS"),
    (20381696, "PowerShot Pro1"),
    (20447232, "PowerShot S70"),
    (2049, "PowerShot SX740 HS"),
    (20512768, "PowerShot S60"),
    (2052, "PowerShot G5 X Mark II"),
    (2053, "PowerShot SX70 HS"),
    (2056, "PowerShot G7 X Mark III"),
    (2065, "EOS M6 Mark II"),
    (2066, "EOS M200"),
    (20971520, "PowerShot G6"),
    (21037056, "PowerShot S500 / Digital IXUS 500 / IXY Digital 500"),
    (21102592, "PowerShot A75"),
    (21233664, "PowerShot SD110 / Digital IXUS IIs / IXY Digital 30a"),
    (21299200, "PowerShot A400"),
    (21430272, "PowerShot A310"),
    (2147483649, "EOS-1D"),
    (2147484007, "EOS-1DS"),
    (2147484008, "EOS 10D"),
    (2147484009, "EOS-1D Mark III"),
    (2147484016, "EOS Digital Rebel / 300D / Kiss Digital"),
    (2147484020, "EOS-1D Mark II"),
    (2147484021, "EOS 20D"),
    (2147484022, "EOS Digital Rebel XSi / 450D / Kiss X2"),
    (2147484040, "EOS-1Ds Mark II"),
    (2147484041, "EOS Digital Rebel XT / 350D / Kiss Digital N"),
    (2147484048, "EOS 40D"),
    (2147484179, "EOS 5D"),
    (2147484181, "EOS-1Ds Mark III"),
    (2147484184, "EOS 5D Mark II"),
    (2147484185, "WFT-E1"),
    (2147484210, "EOS-1D Mark II N"),
    (2147484212, "EOS 30D"),
    (2147484214, "EOS Digital Rebel XTi / 400D / Kiss Digital X"),
    (2147484225, "WFT-E2"),
    (2147484230, "WFT-E3"),
    (2147484240, "EOS 7D"),
    (2147484242, "EOS Rebel T1i / 500D / Kiss X3"),
    (2147484244, "EOS Rebel XS / 1000D / Kiss F"),
    (2147484257, "EOS 50D"),
    (2147484265, "EOS-1D X"),
    (2147484272, "EOS Rebel T2i / 550D / Kiss X4"),
    (2147484273, "WFT-E4"),
    (2147484275, "WFT-E5"),
    (2147484289, "EOS-1D Mark IV"),
    (2147484293, "EOS 5D Mark III"),
    (2147484294, "EOS Rebel T3i / 600D / Kiss X5"),
    (2147484295, "EOS 60D"),
    (2147484296, "EOS Rebel T3 / 1100D / Kiss X50"),
    (2147484297, "EOS 7D Mark II"),
    (2147484311, "WFT-E2 II"),
    (2147484312, "WFT-E4 II"),
    (2147484417, "EOS Rebel T4i / 650D / Kiss X6i"),
    (2147484418, "EOS 6D"),
    (2147484452, "EOS-1D C"),
    (2147484453, "EOS 70D"),
    (2147484454, "EOS Rebel T5i / 700D / Kiss X7i"),
    (2147484455, "EOS Rebel T5 / 1200D / Kiss X70 / Hi"),
    (2147484456, "EOS-1D X Mark II"),
    (2147484465, "EOS M"),
    (2147484486, "EOS Rebel SL1 / 100D / Kiss X7"),
    (2147484487, "EOS Rebel T6s / 760D / 8000D"),
    (2147484489, "EOS 5D Mark IV"),
    (2147484496, "EOS 80D"),
    (2147484501, "EOS M2"),
    (2147484546, "EOS 5DS"),
    (2147484563, "EOS Rebel T6i / 750D / Kiss X8i"),
    (2147484673, "EOS 5DS R"),
    (2147484676, "EOS Rebel T6 / 1300D / Kiss X80"),
    (2147484677, "EOS Rebel T7i / 800D / Kiss X9i"),
    (2147484678, "EOS 6D Mark II"),
    (2147484680, "EOS 77D / 9000D"),
    (2147484695, "EOS Rebel SL2 / 200D / Kiss X9"),
    (2147484705, "EOS R5"),
    (2147484706, "EOS Rebel T100 / 4000D / 3000D"),
    (2147484708, "EOS R"),
    (2147484712, "EOS-1D X Mark III"),
    (2147484722, "EOS Rebel T7 / 2000D / 1500D / Kiss X90"),
    (2147484723, "EOS RP"),
    (2147484725, "EOS Rebel T8i / 850D / X10i"),
    (2147484726, "EOS SL3 / 250D / Kiss X10"),
    (2147484727, "EOS 90D"),
    (2147484752, "EOS R3"),
    (2147484755, "EOS R6"),
    (2147484772, "EOS R7"),
    (2147484773, "EOS R10"),
    (2147484775, "PowerShot ZOOM"),
    (2147484776, "EOS M50 Mark II / Kiss M2"),
    (2147484800, "EOS R50"),
    (2147484801, "EOS R6 Mark II"),
    (2147484807, "EOS R8"),
    (2147484817, "PowerShot V10"),
    (2147484821, "EOS R1"),
    (2147484822, "EOS R5 Mark II"),
    (2147484823, "PowerShot V1"),
    (2147484824, "EOS R100"),
    (2147484950, "EOS R50 V"),
    (2147484952, "EOS R6 Mark III"),
    (2147484960, "EOS D2000C"),
    (2147485024, "EOS D6000C"),
    (21561344, "PowerShot A85"),
    (22151168, "PowerShot S410 / Digital IXUS 430 / IXY Digital 450"),
    (22216704, "PowerShot A95"),
    (22282240, "PowerShot SD300 / Digital IXUS 40 / IXY Digital 50"),
    (22347776, "PowerShot SD200 / Digital IXUS 30 / IXY Digital 40"),
    (22413312, "PowerShot A520"),
    (22478848, "PowerShot A510"),
    (22609920, "PowerShot SD20 / Digital IXUS i5 / IXY Digital L2"),
    (23330816, "PowerShot S2 IS"),
    (23396352, "PowerShot SD430 / Digital IXUS Wireless / IXY Digital Wireless"),
    (23461888, "PowerShot SD500 / Digital IXUS 700 / IXY Digital 600"),
    (23494656, "EOS D60"),
    (24117248, "PowerShot SD30 / Digital IXUS i Zoom / IXY Digital L3"),
    (24379392, "PowerShot A430"),
    (24444928, "PowerShot A410"),
    (24510464, "PowerShot S80"),
    (24641536, "PowerShot A620"),
    (24707072, "PowerShot A610"),
    (25165824, "PowerShot SD630 / Digital IXUS 65 / IXY Digital 80"),
    (25231360, "PowerShot SD450 / Digital IXUS 55 / IXY Digital 60"),
    (25296896, "PowerShot TX1"),
    (25624576, "PowerShot SD400 / Digital IXUS 50 / IXY Digital 55"),
    (25690112, "PowerShot A420"),
    (25755648, "PowerShot SD900 / Digital IXUS 900 Ti / IXY Digital 1000"),
    (26214400, "PowerShot SD550 / Digital IXUS 750 / IXY Digital 700"),
    (26345472, "PowerShot A700"),
    (26476544, "PowerShot SD700 IS / Digital IXUS 800 IS / IXY Digital 800 IS"),
    (26542080, "PowerShot S3 IS"),
    (26607616, "PowerShot A540"),
    (26673152, "PowerShot SD600 / Digital IXUS 60 / IXY Digital 70"),
    (26738688, "PowerShot G7"),
    (26804224, "PowerShot A530"),
    (33554432, "PowerShot SD800 IS / Digital IXUS 850 IS / IXY Digital 900 IS"),
    (33619968, "PowerShot SD40 / Digital IXUS i7 / IXY Digital L4"),
    (33685504, "PowerShot A710 IS"),
    (33751040, "PowerShot A640"),
    (33816576, "PowerShot A630"),
    (34144256, "PowerShot S5 IS"),
    (34603008, "PowerShot A460"),
    (34734080, "PowerShot SD850 IS / Digital IXUS 950 IS / IXY Digital 810 IS"),
    (34799616, "PowerShot A570 IS"),
    (34865152, "PowerShot A560"),
    (34930688, "PowerShot SD750 / Digital IXUS 75 / IXY Digital 90"),
    (34996224, "PowerShot SD1000 / Digital IXUS 70 / IXY Digital 10"),
    (35127296, "PowerShot A550"),
    (35192832, "PowerShot A450"),
    (35848192, "PowerShot G9"),
    (35913728, "PowerShot A650 IS"),
    (36044800, "PowerShot A720 IS"),
    (36241408, "PowerShot SX100 IS"),
    (36700160, "PowerShot SD950 IS / Digital IXUS 960 IS / IXY Digital 2000 IS"),
    (36765696, "PowerShot SD870 IS / Digital IXUS 860 IS / IXY Digital 910 IS"),
    (36831232, "PowerShot SD890 IS / Digital IXUS 970 IS / IXY Digital 820 IS"),
    (37093376, "PowerShot SD790 IS / Digital IXUS 90 IS / IXY Digital 95 IS"),
    (37158912, "PowerShot SD770 IS / Digital IXUS 85 IS / IXY Digital 25 IS"),
    (37224448, "PowerShot A590 IS"),
    (37289984, "PowerShot A580"),
    (37879808, "PowerShot A470"),
    (37945344, "PowerShot SD1100 IS / Digital IXUS 80 IS / IXY Digital 20 IS"),
    (38141952, "PowerShot SX1 IS"),
    (38207488, "PowerShot SX10 IS"),
    (38273024, "PowerShot A1000 IS"),
    (38338560, "PowerShot G10"),
    (38862848, "PowerShot A2000 IS"),
    (38928384, "PowerShot SX110 IS"),
    (38993920, "PowerShot SD990 IS / Digital IXUS 980 IS / IXY Digital 3000 IS"),
    (39059456, "PowerShot SD880 IS / Digital IXUS 870 IS / IXY Digital 920 IS"),
    (39124992, "PowerShot E1"),
    (39190528, "PowerShot D10"),
    (39256064, "PowerShot SD960 IS / Digital IXUS 110 IS / IXY Digital 510 IS"),
    (39321600, "PowerShot A2100 IS"),
    (39387136, "PowerShot A480"),
    (39845888, "PowerShot SX200 IS"),
    (39911424, "PowerShot SD970 IS / Digital IXUS 990 IS / IXY Digital 830 IS"),
    (39976960, "PowerShot SD780 IS / Digital IXUS 100 IS / IXY Digital 210 IS"),
    (40042496, "PowerShot A1100 IS"),
    (40108032, "PowerShot SD1200 IS / Digital IXUS 95 IS / IXY Digital 110 IS"),
    (40894464, "PowerShot G11"),
    (40960000, "PowerShot SX120 IS"),
    (41025536, "PowerShot S90"),
    (41222144, "PowerShot SX20 IS"),
    (41287680, "PowerShot SD980 IS / Digital IXUS 200 IS / IXY Digital 930 IS"),
    (41353216, "PowerShot SD940 IS / Digital IXUS 120 IS / IXY Digital 220 IS"),
    (41943040, "PowerShot A495"),
    (42008576, "PowerShot A490"),
    (42074112, "PowerShot A3100/A3150 IS"),
    (42139648, "PowerShot A3000 IS"),
    (42205184, "PowerShot SD1400 IS / IXUS 130 / IXY 400F"),
    (42270720, "PowerShot SD1300 IS / IXUS 105 / IXY 200F"),
    (42336256, "PowerShot SD3500 IS / IXUS 210 / IXY 10S"),
    (42401792, "PowerShot SX210 IS"),
    (42467328, "PowerShot SD4000 IS / IXUS 300 HS / IXY 30S"),
    (42532864, "PowerShot SD4500 IS / IXUS 1000 HS / IXY 50S"),
    (43122688, "PowerShot G12"),
    (43188224, "PowerShot SX30 IS"),
    (43253760, "PowerShot SX130 IS"),
    (43319296, "PowerShot S95"),
    (43515904, "PowerShot A3300 IS"),
    (43581440, "PowerShot A3200 IS"),
    (50331648, "PowerShot ELPH 500 HS / IXUS 310 HS / IXY 31S"),
    (50397184, "PowerShot Pro90 IS"),
    (50397185, "PowerShot A800"),
    (50462720, "PowerShot ELPH 100 HS / IXUS 115 HS / IXY 210F"),
    (50528256, "PowerShot SX230 HS"),
    (50593792, "PowerShot ELPH 300 HS / IXUS 220 HS / IXY 410F"),
    (50659328, "PowerShot A2200"),
    (50724864, "PowerShot A1200"),
    (50790400, "PowerShot SX220 HS"),
    (50855936, "PowerShot G1 X"),
    (50921472, "PowerShot SX150 IS"),
    (51380224, "PowerShot ELPH 510 HS / IXUS 1100 HS / IXY 51S"),
    (51445760, "PowerShot S100 (new)"),
    (51511296, "PowerShot ELPH 310 HS / IXUS 230 HS / IXY 600F"),
    (51576832, "PowerShot SX40 HS"),
    (51642368, "IXY 32S"),
    (51773440, "PowerShot A1300"),
    (51838976, "PowerShot A810"),
    (51904512, "PowerShot ELPH 320 HS / IXUS 240 HS / IXY 420F"),
    (51970048, "PowerShot ELPH 110 HS / IXUS 125 HS / IXY 220F"),
    (52428800, "PowerShot D20"),
    (52494336, "PowerShot A4000 IS"),
    (52559872, "PowerShot SX260 HS"),
    (52625408, "PowerShot SX240 HS"),
    (52690944, "PowerShot ELPH 530 HS / IXUS 510 HS / IXY 1"),
    (52756480, "PowerShot ELPH 520 HS / IXUS 500 HS / IXY 3"),
    (52822016, "PowerShot A3400 IS"),
    (52887552, "PowerShot A2400 IS"),
    (52953088, "PowerShot A2300"),
    (53608448, "PowerShot S100V"),
    (53673984, "PowerShot G15"),
    (53739520, "PowerShot SX50 HS"),
    (53805056, "PowerShot SX160 IS"),
    (53870592, "PowerShot S110 (new)"),
    (53936128, "PowerShot SX500 IS"),
    (54001664, "PowerShot N"),
    (54067200, "IXUS 245 HS / IXY 430F"),
    (54525952, "PowerShot SX280 HS"),
    (54591488, "PowerShot SX270 HS"),
    (54657024, "PowerShot A3500 IS"),
    (54722560, "PowerShot A2600"),
    (54788096, "PowerShot SX275 HS"),
    (54853632, "PowerShot A1400"),
    (54919168, "PowerShot ELPH 130 IS / IXUS 140 / IXY 110F"),
    (54984704, "PowerShot ELPH 115/120 IS / IXUS 132/135 / IXY 90F/100F"),
    (55115776, "PowerShot ELPH 330 HS / IXUS 255 HS / IXY 610F"),
    (55640064, "PowerShot A2500"),
    (55836672, "PowerShot G16"),
    (55902208, "PowerShot S120"),
    (55967744, "PowerShot SX170 IS"),
    (56098816, "PowerShot SX510 HS"),
    (56164352, "PowerShot S200 (new)"),
    (56623104, "IXY 620F"),
    (56688640, "PowerShot N100"),
    (56885248, "PowerShot G1 X Mark II"),
    (56950784, "PowerShot D30"),
    (57016320, "PowerShot SX700 HS"),
    (57081856, "PowerShot SX600 HS"),
    (57147392, "PowerShot ELPH 140 IS / IXUS 150 / IXY 130"),
    (57212928, "PowerShot ELPH 135 / IXUS 145 / IXY 120"),
    (57671680, "PowerShot ELPH 340 HS / IXUS 265 HS / IXY 630"),
    (57737216, "PowerShot ELPH 150 IS / IXUS 155 / IXY 140"),
    (57933824, "EOS M3"),
    (57999360, "PowerShot SX60 HS"),
    (58064896, "PowerShot SX520 HS"),
    (58130432, "PowerShot SX400 IS"),
    (58195968, "PowerShot G7 X"),
    (58261504, "PowerShot N2"),
    (58720256, "PowerShot SX530 HS"),
    (58851328, "PowerShot SX710 HS"),
    (58916864, "PowerShot SX610 HS"),
    (58982400, "EOS M10"),
    (59047936, "PowerShot G3 X"),
    (59113472, "PowerShot ELPH 165 HS / IXUS 165 / IXY 160"),
    (59179008, "PowerShot ELPH 160 / IXUS 160"),
    (59244544, "PowerShot ELPH 350 HS / IXUS 275 HS / IXY 640"),
    (59310080, "PowerShot ELPH 170 IS / IXUS 170"),
    (59834368, "PowerShot SX410 IS"),
    (59965440, "PowerShot G9 X"),
    (60030976, "EOS M5"),
    (60096512, "PowerShot G5 X"),
    (60227584, "PowerShot G7 X Mark II"),
    (60293120, "EOS M100"),
    (60358656, "PowerShot ELPH 360 HS / IXUS 285 HS / IXY 650"),
    (67174400, "PowerShot SX540 HS"),
    (67239936, "PowerShot SX420 IS"),
    (67305472, "PowerShot ELPH 190 IS / IXUS 180 / IXY 190"),
    (67371008, "PowerShot G1"),
    (67371009, "PowerShot ELPH 180 IS / IXUS 175 / IXY 180"),
    (67436544, "PowerShot SX720 HS"),
    (67502080, "PowerShot SX620 HS"),
    (67567616, "EOS M6"),
    (68157440, "PowerShot G9 X Mark II"),
    (68485120, "PowerShot ELPH 185 / IXUS 185 / IXY 200"),
    (68550656, "PowerShot SX430 IS"),
    (68616192, "PowerShot SX730 HS"),
    (68681728, "PowerShot G1 X Mark III"),
];

pub static CANON_MAIN_COLORSPACE_VALUES: &[(i64, &str)] = &[
    (1, "sRGB"),
    (2, "Adobe RGB"),
    (65535, "n/a"),
];

pub static CANON_MAIN_SERIALNUMBERFORMAT_VALUES: &[(i64, &str)] = &[
    (2415919104, "Format 1"),
    (2684354560, "Format 2"),
];

pub static CANON_MAIN_SUPERMACRO_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On (1)"),
    (2, "On (2)"),
];

pub static CANON_MAIN_DATESTAMPMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Date"),
    (2, "Date & Time"),
];

pub static CANON_MAIN_CATEGORIES_VALUES: &[(i64, &str)] = &[
    (0, "(none)"),
];

/// Canon::MeasuredColor tags
pub static CANON_MEASUREDCOLOR: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "MeasuredRGGB", values: None },
};

/// Canon::ModifiedInfo tags
pub static CANON_MODIFIEDINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "ModifiedToneCurve", values: Some(CANON_MODIFIEDINFO_MODIFIEDTONECURVE_VALUES) },
    10u16 => TagDef { name: "ModifiedPictureStyle", values: Some(CANON_MODIFIEDINFO_MODIFIEDPICTURESTYLE_VALUES) },
    11u16 => TagDef { name: "ModifiedDigitalGain", values: None },
    2u16 => TagDef { name: "ModifiedSharpness", values: None },
    3u16 => TagDef { name: "ModifiedSharpnessFreq", values: Some(CANON_MODIFIEDINFO_MODIFIEDSHARPNESSFREQ_VALUES) },
    8u16 => TagDef { name: "ModifiedWhiteBalance", values: Some(CANON_MODIFIEDINFO_MODIFIEDWHITEBALANCE_VALUES) },
};

pub static CANON_MODIFIEDINFO_MODIFIEDTONECURVE_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (1, "Manual"),
    (2, "Custom"),
];

pub static CANON_MODIFIEDINFO_MODIFIEDPICTURESTYLE_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "Standard"),
    (129, "Standard"),
    (130, "Portrait"),
    (131, "Landscape"),
    (132, "Neutral"),
    (133, "Faithful"),
    (134, "Monochrome"),
    (135, "Auto"),
    (136, "Fine Detail"),
    (2, "Portrait"),
    (255, "n/a"),
    (3, "High Saturation"),
    (33, "User Def. 1"),
    (34, "User Def. 2"),
    (35, "User Def. 3"),
    (4, "Adobe RGB"),
    (5, "Low Saturation"),
    (6, "CM Set 1"),
    (65, "PC 1"),
    (65535, "n/a"),
    (66, "PC 2"),
    (67, "PC 3"),
    (7, "CM Set 2"),
];

pub static CANON_MODIFIEDINFO_MODIFIEDSHARPNESSFREQ_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "Lowest"),
    (2, "Low"),
    (3, "Standard"),
    (4, "High"),
    (5, "Highest"),
];

pub static CANON_MODIFIEDINFO_MODIFIEDWHITEBALANCE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Daylight"),
    (10, "PC Set1"),
    (11, "PC Set2"),
    (12, "PC Set3"),
    (14, "Daylight Fluorescent"),
    (15, "Custom 1"),
    (16, "Custom 2"),
    (17, "Underwater"),
    (18, "Custom 3"),
    (19, "Custom 4"),
    (2, "Cloudy"),
    (20, "PC Set4"),
    (21, "PC Set5"),
    (23, "Auto (ambience priority)"),
    (3, "Tungsten"),
    (4, "Fluorescent"),
    (5, "Flash"),
    (6, "Custom"),
    (7, "Black & White"),
    (8, "Shade"),
    (9, "Manual Temperature (Kelvin)"),
];

/// Canon::MovieInfo tags
pub static CANON_MOVIEINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "FrameRate", values: None },
    106u16 => TagDef { name: "Duration", values: None },
    108u16 => TagDef { name: "AudioBitrate", values: None },
    110u16 => TagDef { name: "AudioSampleRate", values: None },
    112u16 => TagDef { name: "AudioChannels", values: None },
    116u16 => TagDef { name: "VideoCodec", values: None },
    2u16 => TagDef { name: "FrameCount", values: None },
    4u16 => TagDef { name: "FrameCount", values: None },
    6u16 => TagDef { name: "FrameRate", values: None },
};

/// Canon::MultiExp tags
pub static CANON_MULTIEXP: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "MultiExposure", values: Some(CANON_MULTIEXP_MULTIEXPOSURE_VALUES) },
    2u16 => TagDef { name: "MultiExposureControl", values: Some(CANON_MULTIEXP_MULTIEXPOSURECONTROL_VALUES) },
};

pub static CANON_MULTIEXP_MULTIEXPOSURE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
    (2, "On (RAW)"),
];

pub static CANON_MULTIEXP_MULTIEXPOSURECONTROL_VALUES: &[(i64, &str)] = &[
    (0, "Additive"),
    (1, "Average"),
    (2, "Bright (comparative)"),
    (3, "Dark (comparative)"),
];

/// Canon::MyColors tags
pub static CANON_MYCOLORS: phf::Map<u16, TagDef> = phf::phf_map! {
    2u16 => TagDef { name: "MyColorMode", values: Some(CANON_MYCOLORS_MYCOLORMODE_VALUES) },
};

pub static CANON_MYCOLORS_MYCOLORMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Positive Film"),
    (12, "Vivid"),
    (13, "Neutral"),
    (14, "Sepia"),
    (15, "B&W"),
    (2, "Light Skin Tone"),
    (3, "Dark Skin Tone"),
    (4, "Vivid Blue"),
    (5, "Vivid Green"),
    (6, "Vivid Red"),
    (7, "Color Accent"),
    (8, "Color Swap"),
    (9, "Custom"),
];

/// Canon::Panorama tags
pub static CANON_PANORAMA: phf::Map<u16, TagDef> = phf::phf_map! {
    5u16 => TagDef { name: "PanoramaDirection", values: Some(CANON_PANORAMA_PANORAMADIRECTION_VALUES) },
};

pub static CANON_PANORAMA_PANORAMADIRECTION_VALUES: &[(i64, &str)] = &[
    (0, "Left to Right"),
    (1, "Right to Left"),
    (2, "Bottom to Top"),
    (3, "Top to Bottom"),
    (4, "2x2 Matrix (Clockwise)"),
];

/// Canon::PreviewImageInfo tags
pub static CANON_PREVIEWIMAGEINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "PreviewQuality", values: Some(CANON_PREVIEWIMAGEINFO_PREVIEWQUALITY_VALUES) },
    2u16 => TagDef { name: "PreviewImageLength", values: None },
    5u16 => TagDef { name: "PreviewImageStart", values: None },
};

pub static CANON_PREVIEWIMAGEINFO_PREVIEWQUALITY_VALUES: &[(i64, &str)] = &[
    (-1, "n/a"),
    (1, "Economy"),
    (130, "Light (RAW)"),
    (131, "Standard (RAW)"),
    (2, "Normal"),
    (3, "Fine"),
    (4, "RAW"),
    (5, "Superfine"),
    (7, "CRAW"),
];

/// Canon::Processing tags
pub static CANON_PROCESSING: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "ToneCurve", values: Some(CANON_PROCESSING_TONECURVE_VALUES) },
    10u16 => TagDef { name: "PictureStyle", values: Some(CANON_PROCESSING_PICTURESTYLE_VALUES) },
    11u16 => TagDef { name: "DigitalGain", values: None },
    12u16 => TagDef { name: "WBShiftAB", values: None },
    13u16 => TagDef { name: "WBShiftGM", values: None },
    2u16 => TagDef { name: "Sharpness", values: None },
    3u16 => TagDef { name: "SharpnessFrequency", values: Some(CANON_PROCESSING_SHARPNESSFREQUENCY_VALUES) },
    8u16 => TagDef { name: "WhiteBalance", values: Some(CANON_PROCESSING_WHITEBALANCE_VALUES) },
};

pub static CANON_PROCESSING_TONECURVE_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (1, "Manual"),
    (2, "Custom"),
];

pub static CANON_PROCESSING_PICTURESTYLE_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "Standard"),
    (129, "Standard"),
    (130, "Portrait"),
    (131, "Landscape"),
    (132, "Neutral"),
    (133, "Faithful"),
    (134, "Monochrome"),
    (135, "Auto"),
    (136, "Fine Detail"),
    (2, "Portrait"),
    (255, "n/a"),
    (3, "High Saturation"),
    (33, "User Def. 1"),
    (34, "User Def. 2"),
    (35, "User Def. 3"),
    (4, "Adobe RGB"),
    (5, "Low Saturation"),
    (6, "CM Set 1"),
    (65, "PC 1"),
    (65535, "n/a"),
    (66, "PC 2"),
    (67, "PC 3"),
    (7, "CM Set 2"),
];

pub static CANON_PROCESSING_SHARPNESSFREQUENCY_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "Lowest"),
    (2, "Low"),
    (3, "Standard"),
    (4, "High"),
    (5, "Highest"),
];

pub static CANON_PROCESSING_WHITEBALANCE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Daylight"),
    (10, "PC Set1"),
    (11, "PC Set2"),
    (12, "PC Set3"),
    (14, "Daylight Fluorescent"),
    (15, "Custom 1"),
    (16, "Custom 2"),
    (17, "Underwater"),
    (18, "Custom 3"),
    (19, "Custom 4"),
    (2, "Cloudy"),
    (20, "PC Set4"),
    (21, "PC Set5"),
    (23, "Auto (ambience priority)"),
    (3, "Tungsten"),
    (4, "Fluorescent"),
    (5, "Flash"),
    (6, "Custom"),
    (7, "Black & White"),
    (8, "Shade"),
    (9, "Manual Temperature (Kelvin)"),
];

/// Canon::SensorInfo tags
pub static CANON_SENSORINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    9u16 => TagDef { name: "BlackMaskLeftBorder", values: None },
};

/// Canon::SerialInfo tags
pub static CANON_SERIALINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    9u16 => TagDef { name: "InternalSerialNumber", values: None },
};

/// Canon::ShotInfo tags
pub static CANON_SHOTINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "AutoISO", values: None },
    10u16 => TagDef { name: "OpticalZoomCode", values: None },
    12u16 => TagDef { name: "CameraTemperature", values: None },
    13u16 => TagDef { name: "FlashGuideNumber", values: None },
    14u16 => TagDef { name: "AFPointsInFocus", values: Some(CANON_SHOTINFO_AFPOINTSINFOCUS_VALUES) },
    15u16 => TagDef { name: "FlashExposureComp", values: None },
    16u16 => TagDef { name: "AutoExposureBracketing", values: Some(CANON_SHOTINFO_AUTOEXPOSUREBRACKETING_VALUES) },
    17u16 => TagDef { name: "AEBBracketValue", values: None },
    18u16 => TagDef { name: "ControlMode", values: Some(CANON_SHOTINFO_CONTROLMODE_VALUES) },
    19u16 => TagDef { name: "FocusDistanceUpper", values: None },
    2u16 => TagDef { name: "BaseISO", values: None },
    20u16 => TagDef { name: "FocusDistanceLower", values: None },
    21u16 => TagDef { name: "FNumber", values: None },
    22u16 => TagDef { name: "ExposureTime", values: None },
    23u16 => TagDef { name: "MeasuredEV2", values: None },
    24u16 => TagDef { name: "BulbDuration", values: None },
    26u16 => TagDef { name: "CameraType", values: Some(CANON_SHOTINFO_CAMERATYPE_VALUES) },
    27u16 => TagDef { name: "AutoRotate", values: Some(CANON_SHOTINFO_AUTOROTATE_VALUES) },
    28u16 => TagDef { name: "NDFilter", values: Some(CANON_SHOTINFO_NDFILTER_VALUES) },
    29u16 => TagDef { name: "SelfTimer2", values: None },
    3u16 => TagDef { name: "MeasuredEV", values: None },
    33u16 => TagDef { name: "FlashOutput", values: None },
    4u16 => TagDef { name: "TargetAperture", values: None },
    5u16 => TagDef { name: "TargetExposureTime", values: None },
    6u16 => TagDef { name: "ExposureCompensation", values: None },
    7u16 => TagDef { name: "WhiteBalance", values: Some(CANON_SHOTINFO_WHITEBALANCE_VALUES) },
    8u16 => TagDef { name: "SlowShutter", values: Some(CANON_SHOTINFO_SLOWSHUTTER_VALUES) },
    9u16 => TagDef { name: "SequenceNumber", values: None },
};

pub static CANON_SHOTINFO_AFPOINTSINFOCUS_VALUES: &[(i64, &str)] = &[
    (12288, "None (MF)"),
    (12289, "Right"),
    (12290, "Center"),
    (12291, "Center+Right"),
    (12292, "Left"),
    (12293, "Left+Right"),
    (12294, "Left+Center"),
    (12295, "All"),
];

pub static CANON_SHOTINFO_AUTOEXPOSUREBRACKETING_VALUES: &[(i64, &str)] = &[
    (-1, "On"),
    (0, "Off"),
    (1, "On (shot 1)"),
    (2, "On (shot 2)"),
    (3, "On (shot 3)"),
];

pub static CANON_SHOTINFO_CONTROLMODE_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "Camera Local Control"),
    (3, "Computer Remote Control"),
];

pub static CANON_SHOTINFO_CAMERATYPE_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (248, "EOS High-end"),
    (250, "Compact"),
    (252, "EOS Mid-range"),
    (255, "DV Camera"),
];

pub static CANON_SHOTINFO_AUTOROTATE_VALUES: &[(i64, &str)] = &[
    (-1, "n/a"),
    (0, "None"),
    (1, "Rotate 90 CW"),
    (2, "Rotate 180"),
    (3, "Rotate 270 CW"),
];

pub static CANON_SHOTINFO_NDFILTER_VALUES: &[(i64, &str)] = &[
    (-1, "n/a"),
    (0, "Off"),
    (1, "On"),
];

pub static CANON_SHOTINFO_WHITEBALANCE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Daylight"),
    (10, "PC Set1"),
    (11, "PC Set2"),
    (12, "PC Set3"),
    (14, "Daylight Fluorescent"),
    (15, "Custom 1"),
    (16, "Custom 2"),
    (17, "Underwater"),
    (18, "Custom 3"),
    (19, "Custom 4"),
    (2, "Cloudy"),
    (20, "PC Set4"),
    (21, "PC Set5"),
    (23, "Auto (ambience priority)"),
    (3, "Tungsten"),
    (4, "Fluorescent"),
    (5, "Flash"),
    (6, "Custom"),
    (7, "Black & White"),
    (8, "Shade"),
    (9, "Manual Temperature (Kelvin)"),
];

pub static CANON_SHOTINFO_SLOWSHUTTER_VALUES: &[(i64, &str)] = &[
    (-1, "n/a"),
    (0, "Off"),
    (1, "Night Scene"),
    (2, "On"),
    (3, "None"),
];

/// Canon::TimeInfo tags
pub static CANON_TIMEINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "TimeZone", values: None },
    2u16 => TagDef { name: "TimeZoneCity", values: Some(CANON_TIMEINFO_TIMEZONECITY_VALUES) },
    3u16 => TagDef { name: "DaylightSavings", values: Some(CANON_TIMEINFO_DAYLIGHTSAVINGS_VALUES) },
};

pub static CANON_TIMEINFO_TIMEZONECITY_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "Chatham Islands"),
    (10, "Dhaka"),
    (11, "Kathmandu"),
    (12, "Delhi"),
    (13, "Karachi"),
    (14, "Kabul"),
    (15, "Dubai"),
    (16, "Tehran"),
    (17, "Moscow"),
    (18, "Cairo"),
    (19, "Paris"),
    (2, "Wellington"),
    (20, "London"),
    (21, "Azores"),
    (22, "Fernando de Noronha"),
    (23, "Sao Paulo"),
    (24, "Newfoundland"),
    (25, "Santiago"),
    (26, "Caracas"),
    (27, "New York"),
    (28, "Chicago"),
    (29, "Denver"),
    (3, "Solomon Islands"),
    (30, "Los Angeles"),
    (31, "Anchorage"),
    (32, "Honolulu"),
    (32766, "(not set)"),
    (33, "Samoa"),
    (4, "Sydney"),
    (5, "Adelaide"),
    (6, "Tokyo"),
    (7, "Hong Kong"),
    (8, "Bangkok"),
    (9, "Yangon"),
];

pub static CANON_TIMEINFO_DAYLIGHTSAVINGS_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (60, "On"),
];

/// Canon::VignettingCorr tags
pub static CANON_VIGNETTINGCORR: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "VignettingCorrVersion", values: None },
    11u16 => TagDef { name: "OriginalImageWidth", values: None },
    2u16 => TagDef { name: "PeripheralLighting", values: Some(CANON_VIGNETTINGCORR_PERIPHERALLIGHTING_VALUES) },
    3u16 => TagDef { name: "DistortionCorrection", values: Some(CANON_VIGNETTINGCORR_DISTORTIONCORRECTION_VALUES) },
    4u16 => TagDef { name: "ChromaticAberrationCorr", values: Some(CANON_VIGNETTINGCORR_CHROMATICABERRATIONCORR_VALUES) },
    5u16 => TagDef { name: "ChromaticAberrationCorr", values: Some(CANON_VIGNETTINGCORR_CHROMATICABERRATIONCORR_VALUES) },
};

pub static CANON_VIGNETTINGCORR_PERIPHERALLIGHTING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANON_VIGNETTINGCORR_DISTORTIONCORRECTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANON_VIGNETTINGCORR_CHROMATICABERRATIONCORR_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

/// Canon::VignettingCorr2 tags
pub static CANON_VIGNETTINGCORR2: phf::Map<u16, TagDef> = phf::phf_map! {
    5u16 => TagDef { name: "PeripheralLightingSetting", values: Some(CANON_VIGNETTINGCORR2_PERIPHERALLIGHTINGSETTING_VALUES) },
    6u16 => TagDef { name: "ChromaticAberrationSetting", values: Some(CANON_VIGNETTINGCORR2_CHROMATICABERRATIONSETTING_VALUES) },
    7u16 => TagDef { name: "DistortionCorrectionSetting", values: Some(CANON_VIGNETTINGCORR2_DISTORTIONCORRECTIONSETTING_VALUES) },
    9u16 => TagDef { name: "DigitalLensOptimizerSetting", values: Some(CANON_VIGNETTINGCORR2_DIGITALLENSOPTIMIZERSETTING_VALUES) },
};

pub static CANON_VIGNETTINGCORR2_PERIPHERALLIGHTINGSETTING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANON_VIGNETTINGCORR2_CHROMATICABERRATIONSETTING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANON_VIGNETTINGCORR2_DISTORTIONCORRECTIONSETTING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANON_VIGNETTINGCORR2_DIGITALLENSOPTIMIZERSETTING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

/// Canon::WBInfo tags
pub static CANON_WBINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    10u16 => TagDef { name: "WB_GRBGLevelsDaylight", values: None },
    18u16 => TagDef { name: "WB_GRBGLevelsCloudy", values: None },
    2u16 => TagDef { name: "WB_GRBGLevelsAuto", values: None },
    26u16 => TagDef { name: "WB_GRBGLevelsTungsten", values: None },
    34u16 => TagDef { name: "WB_GRBGLevelsFluorescent", values: None },
    42u16 => TagDef { name: "WB_GRBGLevelsFluorHigh", values: None },
    50u16 => TagDef { name: "WB_GRBGLevelsFlash", values: None },
    58u16 => TagDef { name: "WB_GRBGLevelsUnderwater", values: None },
    66u16 => TagDef { name: "WB_GRBGLevelsCustom1", values: None },
    74u16 => TagDef { name: "WB_GRBGLevelsCustom2", values: None },
};

/// CanonCustom::Functions1D tags
pub static CANONCUSTOM_FUNCTIONS1D: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "FocusingScreen", values: Some(CANONCUSTOM_FUNCTIONS1D_FOCUSINGSCREEN_VALUES) },
    1u16 => TagDef { name: "FinderDisplayDuringExposure", values: Some(CANONCUSTOM_FUNCTIONS1D_FINDERDISPLAYDURINGEXPOSURE_VALUES) },
    10u16 => TagDef { name: "AFPointIllumination", values: Some(CANONCUSTOM_FUNCTIONS1D_AFPOINTILLUMINATION_VALUES) },
    11u16 => TagDef { name: "AFPointSelection", values: Some(CANONCUSTOM_FUNCTIONS1D_AFPOINTSELECTION_VALUES) },
    12u16 => TagDef { name: "MirrorLockup", values: Some(CANONCUSTOM_FUNCTIONS1D_MIRRORLOCKUP_VALUES) },
    13u16 => TagDef { name: "AFPointSpotMetering", values: Some(CANONCUSTOM_FUNCTIONS1D_AFPOINTSPOTMETERING_VALUES) },
    14u16 => TagDef { name: "FillFlashAutoReduction", values: Some(CANONCUSTOM_FUNCTIONS1D_FILLFLASHAUTOREDUCTION_VALUES) },
    15u16 => TagDef { name: "ShutterCurtainSync", values: Some(CANONCUSTOM_FUNCTIONS1D_SHUTTERCURTAINSYNC_VALUES) },
    16u16 => TagDef { name: "SafetyShiftInAvOrTv", values: Some(CANONCUSTOM_FUNCTIONS1D_SAFETYSHIFTINAVORTV_VALUES) },
    17u16 => TagDef { name: "AFPointActivationArea", values: Some(CANONCUSTOM_FUNCTIONS1D_AFPOINTACTIVATIONAREA_VALUES) },
    18u16 => TagDef { name: "SwitchToRegisteredAFPoint", values: Some(CANONCUSTOM_FUNCTIONS1D_SWITCHTOREGISTEREDAFPOINT_VALUES) },
    19u16 => TagDef { name: "LensAFStopButton", values: Some(CANONCUSTOM_FUNCTIONS1D_LENSAFSTOPBUTTON_VALUES) },
    2u16 => TagDef { name: "ShutterReleaseNoCFCard", values: Some(CANONCUSTOM_FUNCTIONS1D_SHUTTERRELEASENOCFCARD_VALUES) },
    20u16 => TagDef { name: "AIServoTrackingSensitivity", values: Some(CANONCUSTOM_FUNCTIONS1D_AISERVOTRACKINGSENSITIVITY_VALUES) },
    21u16 => TagDef { name: "AIServoContinuousShooting", values: Some(CANONCUSTOM_FUNCTIONS1D_AISERVOCONTINUOUSSHOOTING_VALUES) },
    3u16 => TagDef { name: "ISOSpeedExpansion", values: Some(CANONCUSTOM_FUNCTIONS1D_ISOSPEEDEXPANSION_VALUES) },
    4u16 => TagDef { name: "ShutterAELButton", values: Some(CANONCUSTOM_FUNCTIONS1D_SHUTTERAELBUTTON_VALUES) },
    5u16 => TagDef { name: "ManualTv", values: Some(CANONCUSTOM_FUNCTIONS1D_MANUALTV_VALUES) },
    6u16 => TagDef { name: "ExposureLevelIncrements", values: Some(CANONCUSTOM_FUNCTIONS1D_EXPOSURELEVELINCREMENTS_VALUES) },
    7u16 => TagDef { name: "USMLensElectronicMF", values: Some(CANONCUSTOM_FUNCTIONS1D_USMLENSELECTRONICMF_VALUES) },
    8u16 => TagDef { name: "LCDPanels", values: Some(CANONCUSTOM_FUNCTIONS1D_LCDPANELS_VALUES) },
    9u16 => TagDef { name: "AEBSequenceAutoCancel", values: Some(CANONCUSTOM_FUNCTIONS1D_AEBSEQUENCEAUTOCANCEL_VALUES) },
};

pub static CANONCUSTOM_FUNCTIONS1D_FOCUSINGSCREEN_VALUES: &[(i64, &str)] = &[
    (0, "Ec-N, R"),
    (1, "Ec-A,B,C,CII,CIII,D,H,I,L"),
];

pub static CANONCUSTOM_FUNCTIONS1D_FINDERDISPLAYDURINGEXPOSURE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANONCUSTOM_FUNCTIONS1D_AFPOINTILLUMINATION_VALUES: &[(i64, &str)] = &[
    (0, "On"),
    (1, "Off"),
    (2, "On without dimming"),
    (3, "Brighter"),
];

pub static CANONCUSTOM_FUNCTIONS1D_AFPOINTSELECTION_VALUES: &[(i64, &str)] = &[
    (0, "H=AF+Main/V=AF+Command"),
    (1, "H=Comp+Main/V=Comp+Command"),
    (2, "H=Command only/V=Assist+Main"),
    (3, "H=FEL+Main/V=FEL+Command"),
];

pub static CANONCUSTOM_FUNCTIONS1D_MIRRORLOCKUP_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Enable"),
];

pub static CANONCUSTOM_FUNCTIONS1D_AFPOINTSPOTMETERING_VALUES: &[(i64, &str)] = &[
    (0, "45/Center AF point"),
    (1, "11/Active AF point"),
    (2, "11/Center AF point"),
    (3, "9/Active AF point"),
];

pub static CANONCUSTOM_FUNCTIONS1D_FILLFLASHAUTOREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "Enable"),
    (1, "Disable"),
];

pub static CANONCUSTOM_FUNCTIONS1D_SHUTTERCURTAINSYNC_VALUES: &[(i64, &str)] = &[
    (0, "1st-curtain sync"),
    (1, "2nd-curtain sync"),
];

pub static CANONCUSTOM_FUNCTIONS1D_SAFETYSHIFTINAVORTV_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Enable"),
];

pub static CANONCUSTOM_FUNCTIONS1D_AFPOINTACTIVATIONAREA_VALUES: &[(i64, &str)] = &[
    (0, "Single AF point"),
    (1, "Expanded (TTL. of 7 AF points)"),
    (2, "Automatic expanded (max. 13)"),
];

pub static CANONCUSTOM_FUNCTIONS1D_SWITCHTOREGISTEREDAFPOINT_VALUES: &[(i64, &str)] = &[
    (0, "Assist + AF"),
    (1, "Assist"),
    (2, "Only while pressing assist"),
];

pub static CANONCUSTOM_FUNCTIONS1D_LENSAFSTOPBUTTON_VALUES: &[(i64, &str)] = &[
    (0, "AF stop"),
    (1, "AF start"),
    (2, "AE lock while metering"),
    (3, "AF point: M -> Auto / Auto -> Ctr."),
    (4, "AF mode: ONE SHOT <-> AI SERVO"),
    (5, "IS start"),
];

pub static CANONCUSTOM_FUNCTIONS1D_SHUTTERRELEASENOCFCARD_VALUES: &[(i64, &str)] = &[
    (0, "Yes"),
    (1, "No"),
];

pub static CANONCUSTOM_FUNCTIONS1D_AISERVOTRACKINGSENSITIVITY_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (1, "Slow"),
    (2, "Moderately slow"),
    (3, "Moderately fast"),
    (4, "Fast"),
];

pub static CANONCUSTOM_FUNCTIONS1D_AISERVOCONTINUOUSSHOOTING_VALUES: &[(i64, &str)] = &[
    (0, "Shooting not possible without focus"),
    (1, "Shooting possible without focus"),
];

pub static CANONCUSTOM_FUNCTIONS1D_ISOSPEEDEXPANSION_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];

pub static CANONCUSTOM_FUNCTIONS1D_SHUTTERAELBUTTON_VALUES: &[(i64, &str)] = &[
    (0, "AF/AE lock stop"),
    (1, "AE lock/AF"),
    (2, "AF/AF lock, No AE lock"),
    (3, "AE/AF, No AE lock"),
];

pub static CANONCUSTOM_FUNCTIONS1D_MANUALTV_VALUES: &[(i64, &str)] = &[
    (0, "Tv=Main/Av=Control"),
    (1, "Tv=Control/Av=Main"),
    (2, "Tv=Main/Av=Main w/o lens"),
    (3, "Tv=Control/Av=Main w/o lens"),
];

pub static CANONCUSTOM_FUNCTIONS1D_EXPOSURELEVELINCREMENTS_VALUES: &[(i64, &str)] = &[
    (0, "1/3-stop set, 1/3-stop comp."),
    (1, "1-stop set, 1/3-stop comp."),
    (2, "1/2-stop set, 1/2-stop comp."),
];

pub static CANONCUSTOM_FUNCTIONS1D_USMLENSELECTRONICMF_VALUES: &[(i64, &str)] = &[
    (0, "Turns on after one-shot AF"),
    (1, "Turns off after one-shot AF"),
    (2, "Always turned off"),
];

pub static CANONCUSTOM_FUNCTIONS1D_LCDPANELS_VALUES: &[(i64, &str)] = &[
    (0, "Remain. shots/File no."),
    (1, "ISO/Remain. shots"),
    (2, "ISO/File no."),
    (3, "Shots in folder/Remain. shots"),
];

pub static CANONCUSTOM_FUNCTIONS1D_AEBSEQUENCEAUTOCANCEL_VALUES: &[(i64, &str)] = &[
    (0, "0,-,+/Enabled"),
    (1, "0,-,+/Disabled"),
    (2, "-,0,+/Enabled"),
    (3, "-,0,+/Disabled"),
];

/// CanonCustom::Functions2 tags
pub static CANONCUSTOM_FUNCTIONS2: phf::Map<u16, TagDef> = phf::phf_map! {
    1031u16 => TagDef { name: "ViewInfoDuringExposure", values: Some(CANONCUSTOM_FUNCTIONS2_VIEWINFODURINGEXPOSURE_VALUES) },
    1032u16 => TagDef { name: "LCDIlluminationDuringBulb", values: Some(CANONCUSTOM_FUNCTIONS2_LCDILLUMINATIONDURINGBULB_VALUES) },
    1033u16 => TagDef { name: "InfoButtonWhenShooting", values: Some(CANONCUSTOM_FUNCTIONS2_INFOBUTTONWHENSHOOTING_VALUES) },
    1034u16 => TagDef { name: "ViewfinderWarnings", values: None },
    1035u16 => TagDef { name: "LVShootingAreaDisplay", values: Some(CANONCUSTOM_FUNCTIONS2_LVSHOOTINGAREADISPLAY_VALUES) },
    1036u16 => TagDef { name: "LVShootingAreaDisplay", values: Some(CANONCUSTOM_FUNCTIONS2_LVSHOOTINGAREADISPLAY_VALUES) },
    1281u16 => TagDef { name: "USMLensElectronicMF", values: Some(CANONCUSTOM_FUNCTIONS2_USMLENSELECTRONICMF_VALUES) },
    1282u16 => TagDef { name: "AIServoTrackingSensitivity", values: Some(CANONCUSTOM_FUNCTIONS2_AISERVOTRACKINGSENSITIVITY_VALUES) },
    1283u16 => TagDef { name: "AIServoImagePriority", values: Some(CANONCUSTOM_FUNCTIONS2_AISERVOIMAGEPRIORITY_VALUES) },
    1284u16 => TagDef { name: "AIServoTrackingMethod", values: Some(CANONCUSTOM_FUNCTIONS2_AISERVOTRACKINGMETHOD_VALUES) },
    1285u16 => TagDef { name: "LensDriveNoAF", values: Some(CANONCUSTOM_FUNCTIONS2_LENSDRIVENOAF_VALUES) },
    1286u16 => TagDef { name: "LensAFStopButton", values: Some(CANONCUSTOM_FUNCTIONS2_LENSAFSTOPBUTTON_VALUES) },
    1287u16 => TagDef { name: "AFMicroadjustment", values: None },
    1288u16 => TagDef { name: "AFPointAreaExpansion", values: Some(CANONCUSTOM_FUNCTIONS2_AFPOINTAREAEXPANSION_VALUES) },
    1289u16 => TagDef { name: "SelectableAFPoint", values: Some(CANONCUSTOM_FUNCTIONS2_SELECTABLEAFPOINT_VALUES) },
    1290u16 => TagDef { name: "SwitchToRegisteredAFPoint", values: Some(CANONCUSTOM_FUNCTIONS2_SWITCHTOREGISTEREDAFPOINT_VALUES) },
    1291u16 => TagDef { name: "AFPointAutoSelection", values: Some(CANONCUSTOM_FUNCTIONS2_AFPOINTAUTOSELECTION_VALUES) },
    1292u16 => TagDef { name: "AFPointDisplayDuringFocus", values: Some(CANONCUSTOM_FUNCTIONS2_AFPOINTDISPLAYDURINGFOCUS_VALUES) },
    1293u16 => TagDef { name: "AFPointBrightness", values: Some(CANONCUSTOM_FUNCTIONS2_AFPOINTBRIGHTNESS_VALUES) },
    1294u16 => TagDef { name: "AFAssistBeam", values: Some(CANONCUSTOM_FUNCTIONS2_AFASSISTBEAM_VALUES) },
    1295u16 => TagDef { name: "AFPointSelectionMethod", values: Some(CANONCUSTOM_FUNCTIONS2_AFPOINTSELECTIONMETHOD_VALUES) },
    1296u16 => TagDef { name: "VFDisplayIllumination", values: Some(CANONCUSTOM_FUNCTIONS2_VFDISPLAYILLUMINATION_VALUES) },
    1297u16 => TagDef { name: "AFDuringLiveView", values: Some(CANONCUSTOM_FUNCTIONS2_AFDURINGLIVEVIEW_VALUES) },
    1298u16 => TagDef { name: "SelectAFAreaSelectMode", values: None },
    1299u16 => TagDef { name: "ManualAFPointSelectPattern", values: Some(CANONCUSTOM_FUNCTIONS2_MANUALAFPOINTSELECTPATTERN_VALUES) },
    1300u16 => TagDef { name: "DisplayAllAFPoints", values: Some(CANONCUSTOM_FUNCTIONS2_DISPLAYALLAFPOINTS_VALUES) },
    1301u16 => TagDef { name: "FocusDisplayAIServoAndMF", values: Some(CANONCUSTOM_FUNCTIONS2_FOCUSDISPLAYAISERVOANDMF_VALUES) },
    1302u16 => TagDef { name: "OrientationLinkedAFPoint", values: Some(CANONCUSTOM_FUNCTIONS2_ORIENTATIONLINKEDAFPOINT_VALUES) },
    1303u16 => TagDef { name: "MultiControllerWhileMetering", values: Some(CANONCUSTOM_FUNCTIONS2_MULTICONTROLLERWHILEMETERING_VALUES) },
    1304u16 => TagDef { name: "AccelerationTracking", values: None },
    1305u16 => TagDef { name: "AIServoFirstImagePriority", values: Some(CANONCUSTOM_FUNCTIONS2_AISERVOFIRSTIMAGEPRIORITY_VALUES) },
    1306u16 => TagDef { name: "AIServoSecondImagePriority", values: Some(CANONCUSTOM_FUNCTIONS2_AISERVOSECONDIMAGEPRIORITY_VALUES) },
    1307u16 => TagDef { name: "AFAreaSelectMethod", values: Some(CANONCUSTOM_FUNCTIONS2_AFAREASELECTMETHOD_VALUES) },
    1308u16 => TagDef { name: "AutoAFPointColorTracking", values: Some(CANONCUSTOM_FUNCTIONS2_AUTOAFPOINTCOLORTRACKING_VALUES) },
    1309u16 => TagDef { name: "VFDisplayIllumination", values: None },
    1310u16 => TagDef { name: "InitialAFPointAIServoAF", values: Some(CANONCUSTOM_FUNCTIONS2_INITIALAFPOINTAISERVOAF_VALUES) },
    1551u16 => TagDef { name: "MirrorLockup", values: Some(CANONCUSTOM_FUNCTIONS2_MIRRORLOCKUP_VALUES) },
    1552u16 => TagDef { name: "ContinuousShootingSpeed", values: None },
    1553u16 => TagDef { name: "ContinuousShotLimit", values: None },
    1554u16 => TagDef { name: "RestrictDriveModes", values: None },
    1793u16 => TagDef { name: "Shutter-AELock", values: Some(CANONCUSTOM_FUNCTIONS2_SHUTTER_AELOCK_VALUES) },
    1794u16 => TagDef { name: "AFOnAELockButtonSwitch", values: Some(CANONCUSTOM_FUNCTIONS2_AFONAELOCKBUTTONSWITCH_VALUES) },
    1795u16 => TagDef { name: "QuickControlDialInMeter", values: Some(CANONCUSTOM_FUNCTIONS2_QUICKCONTROLDIALINMETER_VALUES) },
    1796u16 => TagDef { name: "SetButtonWhenShooting", values: Some(CANONCUSTOM_FUNCTIONS2_SETBUTTONWHENSHOOTING_VALUES) },
    1797u16 => TagDef { name: "ManualTv", values: Some(CANONCUSTOM_FUNCTIONS2_MANUALTV_VALUES) },
    1798u16 => TagDef { name: "DialDirectionTvAv", values: Some(CANONCUSTOM_FUNCTIONS2_DIALDIRECTIONTVAV_VALUES) },
    1799u16 => TagDef { name: "AvSettingWithoutLens", values: Some(CANONCUSTOM_FUNCTIONS2_AVSETTINGWITHOUTLENS_VALUES) },
    1800u16 => TagDef { name: "WBMediaImageSizeSetting", values: Some(CANONCUSTOM_FUNCTIONS2_WBMEDIAIMAGESIZESETTING_VALUES) },
    1801u16 => TagDef { name: "LockMicrophoneButton", values: None },
    1802u16 => TagDef { name: "ButtonFunctionControlOff", values: Some(CANONCUSTOM_FUNCTIONS2_BUTTONFUNCTIONCONTROLOFF_VALUES) },
    1803u16 => TagDef { name: "AssignFuncButton", values: Some(CANONCUSTOM_FUNCTIONS2_ASSIGNFUNCBUTTON_VALUES) },
    1804u16 => TagDef { name: "CustomControls", values: None },
    1805u16 => TagDef { name: "StartMovieShooting", values: Some(CANONCUSTOM_FUNCTIONS2_STARTMOVIESHOOTING_VALUES) },
    1806u16 => TagDef { name: "FlashButtonFunction", values: Some(CANONCUSTOM_FUNCTIONS2_FLASHBUTTONFUNCTION_VALUES) },
    1807u16 => TagDef { name: "MultiFunctionLock", values: None },
    1808u16 => TagDef { name: "TrashButtonFunction", values: Some(CANONCUSTOM_FUNCTIONS2_TRASHBUTTONFUNCTION_VALUES) },
    1809u16 => TagDef { name: "ShutterReleaseWithoutLens", values: Some(CANONCUSTOM_FUNCTIONS2_SHUTTERRELEASEWITHOUTLENS_VALUES) },
    1810u16 => TagDef { name: "ControlRingRotation", values: Some(CANONCUSTOM_FUNCTIONS2_CONTROLRINGROTATION_VALUES) },
    1811u16 => TagDef { name: "FocusRingRotation", values: Some(CANONCUSTOM_FUNCTIONS2_FOCUSRINGROTATION_VALUES) },
    1812u16 => TagDef { name: "RFLensMFFocusRingSensitivity", values: Some(CANONCUSTOM_FUNCTIONS2_RFLENSMFFOCUSRINGSENSITIVITY_VALUES) },
    1813u16 => TagDef { name: "CustomizeDials", values: None },
    2059u16 => TagDef { name: "FocusingScreen", values: Some(CANONCUSTOM_FUNCTIONS2_FOCUSINGSCREEN_VALUES) },
    2060u16 => TagDef { name: "TimerLength", values: None },
    2061u16 => TagDef { name: "ShortReleaseTimeLag", values: Some(CANONCUSTOM_FUNCTIONS2_SHORTRELEASETIMELAG_VALUES) },
    2062u16 => TagDef { name: "AddAspectRatioInfo", values: Some(CANONCUSTOM_FUNCTIONS2_ADDASPECTRATIOINFO_VALUES) },
    2063u16 => TagDef { name: "AddOriginalDecisionData", values: Some(CANONCUSTOM_FUNCTIONS2_ADDORIGINALDECISIONDATA_VALUES) },
    2064u16 => TagDef { name: "LiveViewExposureSimulation", values: Some(CANONCUSTOM_FUNCTIONS2_LIVEVIEWEXPOSURESIMULATION_VALUES) },
    2065u16 => TagDef { name: "LCDDisplayAtPowerOn", values: Some(CANONCUSTOM_FUNCTIONS2_LCDDISPLAYATPOWERON_VALUES) },
    2066u16 => TagDef { name: "MemoAudioQuality", values: Some(CANONCUSTOM_FUNCTIONS2_MEMOAUDIOQUALITY_VALUES) },
    2067u16 => TagDef { name: "DefaultEraseOption", values: Some(CANONCUSTOM_FUNCTIONS2_DEFAULTERASEOPTION_VALUES) },
    2068u16 => TagDef { name: "RetractLensOnPowerOff", values: Some(CANONCUSTOM_FUNCTIONS2_RETRACTLENSONPOWEROFF_VALUES) },
    2069u16 => TagDef { name: "AddIPTCInformation", values: Some(CANONCUSTOM_FUNCTIONS2_ADDIPTCINFORMATION_VALUES) },
    2070u16 => TagDef { name: "AudioCompression", values: Some(CANONCUSTOM_FUNCTIONS2_AUDIOCOMPRESSION_VALUES) },
    257u16 => TagDef { name: "ExposureLevelIncrements", values: Some(CANONCUSTOM_FUNCTIONS2_EXPOSURELEVELINCREMENTS_VALUES) },
    258u16 => TagDef { name: "ISOSpeedIncrements", values: Some(CANONCUSTOM_FUNCTIONS2_ISOSPEEDINCREMENTS_VALUES) },
    259u16 => TagDef { name: "ISOSpeedRange", values: None },
    260u16 => TagDef { name: "AEBAutoCancel", values: Some(CANONCUSTOM_FUNCTIONS2_AEBAUTOCANCEL_VALUES) },
    261u16 => TagDef { name: "AEBSequence", values: Some(CANONCUSTOM_FUNCTIONS2_AEBSEQUENCE_VALUES) },
    262u16 => TagDef { name: "AEBShotCount", values: Some(CANONCUSTOM_FUNCTIONS2_AEBSHOTCOUNT_VALUES) },
    263u16 => TagDef { name: "SpotMeterLinkToAFPoint", values: Some(CANONCUSTOM_FUNCTIONS2_SPOTMETERLINKTOAFPOINT_VALUES) },
    264u16 => TagDef { name: "SafetyShift", values: Some(CANONCUSTOM_FUNCTIONS2_SAFETYSHIFT_VALUES) },
    265u16 => TagDef { name: "UsableShootingModes", values: None },
    266u16 => TagDef { name: "UsableMeteringModes", values: None },
    267u16 => TagDef { name: "ExposureModeInManual", values: Some(CANONCUSTOM_FUNCTIONS2_EXPOSUREMODEINMANUAL_VALUES) },
    268u16 => TagDef { name: "ShutterSpeedRange", values: None },
    269u16 => TagDef { name: "ApertureRange", values: None },
    270u16 => TagDef { name: "ApplyShootingMeteringMode", values: None },
    271u16 => TagDef { name: "FlashSyncSpeedAv", values: Some(CANONCUSTOM_FUNCTIONS2_FLASHSYNCSPEEDAV_VALUES) },
    272u16 => TagDef { name: "AEMicroadjustment", values: None },
    273u16 => TagDef { name: "FEMicroadjustment", values: None },
    274u16 => TagDef { name: "SameExposureForNewAperture", values: Some(CANONCUSTOM_FUNCTIONS2_SAMEEXPOSUREFORNEWAPERTURE_VALUES) },
    275u16 => TagDef { name: "ExposureCompAutoCancel", values: Some(CANONCUSTOM_FUNCTIONS2_EXPOSURECOMPAUTOCANCEL_VALUES) },
    276u16 => TagDef { name: "AELockMeterModeAfterFocus", values: None },
    513u16 => TagDef { name: "LongExposureNoiseReduction", values: Some(CANONCUSTOM_FUNCTIONS2_LONGEXPOSURENOISEREDUCTION_VALUES) },
    514u16 => TagDef { name: "HighISONoiseReduction", values: Some(CANONCUSTOM_FUNCTIONS2_HIGHISONOISEREDUCTION_VALUES) },
    515u16 => TagDef { name: "HighlightTonePriority", values: Some(CANONCUSTOM_FUNCTIONS2_HIGHLIGHTTONEPRIORITY_VALUES) },
    516u16 => TagDef { name: "AutoLightingOptimizer", values: Some(CANONCUSTOM_FUNCTIONS2_AUTOLIGHTINGOPTIMIZER_VALUES) },
    772u16 => TagDef { name: "ETTLII", values: Some(CANONCUSTOM_FUNCTIONS2_ETTLII_VALUES) },
    773u16 => TagDef { name: "ShutterCurtainSync", values: Some(CANONCUSTOM_FUNCTIONS2_SHUTTERCURTAINSYNC_VALUES) },
    774u16 => TagDef { name: "FlashFiring", values: Some(CANONCUSTOM_FUNCTIONS2_FLASHFIRING_VALUES) },
};

pub static CANONCUSTOM_FUNCTIONS2_VIEWINFODURINGEXPOSURE_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Enable"),
];

pub static CANONCUSTOM_FUNCTIONS2_LCDILLUMINATIONDURINGBULB_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANONCUSTOM_FUNCTIONS2_INFOBUTTONWHENSHOOTING_VALUES: &[(i64, &str)] = &[
    (0, "Displays camera settings"),
    (1, "Displays shooting functions"),
];

pub static CANONCUSTOM_FUNCTIONS2_LVSHOOTINGAREADISPLAY_VALUES: &[(i64, &str)] = &[
    (0, "Masked"),
    (1, "Outlined"),
];

pub static CANONCUSTOM_FUNCTIONS2_USMLENSELECTRONICMF_VALUES: &[(i64, &str)] = &[
    (0, "Enable after one-shot AF"),
    (1, "Disable after one-shot AF"),
    (2, "Disable in AF mode"),
];

pub static CANONCUSTOM_FUNCTIONS2_AISERVOTRACKINGSENSITIVITY_VALUES: &[(i64, &str)] = &[
    (-1, "Medium Slow"),
    (-2, "Slow"),
    (0, "Standard"),
    (1, "Medium Fast"),
    (2, "Fast"),
];

pub static CANONCUSTOM_FUNCTIONS2_AISERVOIMAGEPRIORITY_VALUES: &[(i64, &str)] = &[
    (0, "1: AF, 2: Tracking"),
    (1, "1: AF, 2: Drive speed"),
    (2, "1: Release, 2: Drive speed"),
    (3, "1: Release, 2: Tracking"),
];

pub static CANONCUSTOM_FUNCTIONS2_AISERVOTRACKINGMETHOD_VALUES: &[(i64, &str)] = &[
    (0, "Main focus point priority"),
    (1, "Continuous AF track priority"),
];

pub static CANONCUSTOM_FUNCTIONS2_LENSDRIVENOAF_VALUES: &[(i64, &str)] = &[
    (0, "Focus search on"),
    (1, "Focus search off"),
];

pub static CANONCUSTOM_FUNCTIONS2_LENSAFSTOPBUTTON_VALUES: &[(i64, &str)] = &[
    (0, "AF stop"),
    (1, "AF start"),
    (2, "AE lock"),
    (3, "AF point: M->Auto/Auto->ctr"),
    (4, "One Shot <-> AI servo"),
    (5, "IS start"),
    (6, "Switch to registered AF point"),
    (7, "Spot AF"),
];

pub static CANONCUSTOM_FUNCTIONS2_AFPOINTAREAEXPANSION_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Enable"),
];

pub static CANONCUSTOM_FUNCTIONS2_SELECTABLEAFPOINT_VALUES: &[(i64, &str)] = &[
    (0, "45 points"),
    (1, "19 points"),
    (2, "11 points"),
    (3, "Inner 9 points"),
    (4, "Outer 9 points"),
];

pub static CANONCUSTOM_FUNCTIONS2_SWITCHTOREGISTEREDAFPOINT_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Switch with multi-controller"),
    (2, "Only while AEL is pressed"),
];

pub static CANONCUSTOM_FUNCTIONS2_AFPOINTAUTOSELECTION_VALUES: &[(i64, &str)] = &[
    (0, "Control-direct:disable/Main:enable"),
    (1, "Control-direct:disable/Main:disable"),
    (2, "Control-direct:enable/Main:enable"),
];

pub static CANONCUSTOM_FUNCTIONS2_AFPOINTDISPLAYDURINGFOCUS_VALUES: &[(i64, &str)] = &[
    (0, "On"),
    (1, "Off"),
    (2, "On (when focus achieved)"),
];

pub static CANONCUSTOM_FUNCTIONS2_AFPOINTBRIGHTNESS_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Brighter"),
];

pub static CANONCUSTOM_FUNCTIONS2_AFASSISTBEAM_VALUES: &[(i64, &str)] = &[
    (0, "Emits"),
    (1, "Does not emit"),
    (2, "IR AF assist beam only"),
];

pub static CANONCUSTOM_FUNCTIONS2_AFPOINTSELECTIONMETHOD_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Multi-controller direct"),
    (2, "Quick Control Dial direct"),
];

pub static CANONCUSTOM_FUNCTIONS2_VFDISPLAYILLUMINATION_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Enable"),
    (2, "Disable"),
];

pub static CANONCUSTOM_FUNCTIONS2_AFDURINGLIVEVIEW_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Enable"),
];

pub static CANONCUSTOM_FUNCTIONS2_MANUALAFPOINTSELECTPATTERN_VALUES: &[(i64, &str)] = &[
    (0, "Stops at AF area edges"),
    (1, "Continuous"),
];

pub static CANONCUSTOM_FUNCTIONS2_DISPLAYALLAFPOINTS_VALUES: &[(i64, &str)] = &[
    (0, "Enable"),
    (1, "Disable"),
];

pub static CANONCUSTOM_FUNCTIONS2_FOCUSDISPLAYAISERVOANDMF_VALUES: &[(i64, &str)] = &[
    (0, "Enable"),
    (1, "Disable"),
];

pub static CANONCUSTOM_FUNCTIONS2_ORIENTATIONLINKEDAFPOINT_VALUES: &[(i64, &str)] = &[
    (0, "Same for vertical and horizontal"),
    (1, "Select different AF points"),
];

pub static CANONCUSTOM_FUNCTIONS2_MULTICONTROLLERWHILEMETERING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "AF point selection"),
];

pub static CANONCUSTOM_FUNCTIONS2_AISERVOFIRSTIMAGEPRIORITY_VALUES: &[(i64, &str)] = &[
    (-1, "Release priority"),
    (0, "Equal priority"),
    (1, "Focus priority"),
];

pub static CANONCUSTOM_FUNCTIONS2_AISERVOSECONDIMAGEPRIORITY_VALUES: &[(i64, &str)] = &[
    (-1, "Shooting speed priority"),
    (0, "Equal priority"),
    (1, "Focus priority"),
];

pub static CANONCUSTOM_FUNCTIONS2_AFAREASELECTMETHOD_VALUES: &[(i64, &str)] = &[
    (0, "AF area selection button"),
    (1, "Main dial"),
];

pub static CANONCUSTOM_FUNCTIONS2_AUTOAFPOINTCOLORTRACKING_VALUES: &[(i64, &str)] = &[
    (0, "On-Shot AF only"),
    (1, "Disable"),
];

pub static CANONCUSTOM_FUNCTIONS2_INITIALAFPOINTAISERVOAF_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Initial AF point selected"),
    (2, "Manual AF point"),
];

pub static CANONCUSTOM_FUNCTIONS2_MIRRORLOCKUP_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Enable"),
    (2, "Enable: Down with Set"),
];

pub static CANONCUSTOM_FUNCTIONS2_SHUTTER_AELOCK_VALUES: &[(i64, &str)] = &[
    (0, "AF/AE lock"),
    (1, "AE lock/AF"),
    (2, "AF/AF lock, No AE lock"),
    (3, "AE/AF, No AE lock"),
];

pub static CANONCUSTOM_FUNCTIONS2_AFONAELOCKBUTTONSWITCH_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Enable"),
];

pub static CANONCUSTOM_FUNCTIONS2_QUICKCONTROLDIALINMETER_VALUES: &[(i64, &str)] = &[
    (0, "Exposure comp/Aperture"),
    (1, "AF point selection"),
    (2, "ISO speed"),
    (3, "AF point selection swapped with Exposure comp"),
    (4, "ISO speed swapped with Exposure comp"),
];

pub static CANONCUSTOM_FUNCTIONS2_SETBUTTONWHENSHOOTING_VALUES: &[(i64, &str)] = &[
    (0, "Normal (disabled)"),
    (1, "Image quality"),
    (2, "Picture style"),
    (3, "Menu display"),
    (4, "Image playback"),
    (5, "Quick control screen"),
    (6, "Record movie (Live View)"),
];

pub static CANONCUSTOM_FUNCTIONS2_MANUALTV_VALUES: &[(i64, &str)] = &[
    (0, "Tv=Main/Av=Control"),
    (1, "Tv=Control/Av=Main"),
];

pub static CANONCUSTOM_FUNCTIONS2_DIALDIRECTIONTVAV_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Reversed"),
];

pub static CANONCUSTOM_FUNCTIONS2_AVSETTINGWITHOUTLENS_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Enable"),
];

pub static CANONCUSTOM_FUNCTIONS2_WBMEDIAIMAGESIZESETTING_VALUES: &[(i64, &str)] = &[
    (0, "Rear LCD panel"),
    (1, "LCD monitor"),
    (2, "Off (disable button)"),
];

pub static CANONCUSTOM_FUNCTIONS2_BUTTONFUNCTIONCONTROLOFF_VALUES: &[(i64, &str)] = &[
    (0, "Normal (enable)"),
    (1, "Disable main, Control, Multi-control"),
];

pub static CANONCUSTOM_FUNCTIONS2_ASSIGNFUNCBUTTON_VALUES: &[(i64, &str)] = &[
    (0, "LCD brightness"),
    (1, "Image quality"),
    (2, "Exposure comp./AEB setting"),
    (3, "Image jump with main dial"),
    (4, "Live view function settings"),
];

pub static CANONCUSTOM_FUNCTIONS2_STARTMOVIESHOOTING_VALUES: &[(i64, &str)] = &[
    (0, "Default (from LV)"),
    (1, "Quick start (FEL button)"),
];

pub static CANONCUSTOM_FUNCTIONS2_FLASHBUTTONFUNCTION_VALUES: &[(i64, &str)] = &[
    (0, "Raise built-in flash"),
    (1, "ISO speed"),
];

pub static CANONCUSTOM_FUNCTIONS2_TRASHBUTTONFUNCTION_VALUES: &[(i64, &str)] = &[
    (0, "Normal (set center AF point)"),
    (1, "Depth-of-field preview"),
];

pub static CANONCUSTOM_FUNCTIONS2_SHUTTERRELEASEWITHOUTLENS_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Enable"),
];

pub static CANONCUSTOM_FUNCTIONS2_CONTROLRINGROTATION_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Reversed"),
];

pub static CANONCUSTOM_FUNCTIONS2_FOCUSRINGROTATION_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Reversed"),
];

pub static CANONCUSTOM_FUNCTIONS2_RFLENSMFFOCUSRINGSENSITIVITY_VALUES: &[(i64, &str)] = &[
    (0, "Varies With Rotation Speed"),
    (1, "Linked To Rotation Angle"),
];

pub static CANONCUSTOM_FUNCTIONS2_FOCUSINGSCREEN_VALUES: &[(i64, &str)] = &[
    (0, "Ef-A"),
    (1, "Ef-D"),
    (2, "Ef-S"),
];

pub static CANONCUSTOM_FUNCTIONS2_SHORTRELEASETIMELAG_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Enable"),
];

pub static CANONCUSTOM_FUNCTIONS2_ADDASPECTRATIOINFO_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "6:6"),
    (2, "3:4"),
    (3, "4:5"),
    (4, "6:7"),
    (5, "10:12"),
    (6, "5:7"),
];

pub static CANONCUSTOM_FUNCTIONS2_ADDORIGINALDECISIONDATA_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static CANONCUSTOM_FUNCTIONS2_LIVEVIEWEXPOSURESIMULATION_VALUES: &[(i64, &str)] = &[
    (0, "Disable (LCD auto adjust)"),
    (1, "Enable (simulates exposure)"),
];

pub static CANONCUSTOM_FUNCTIONS2_LCDDISPLAYATPOWERON_VALUES: &[(i64, &str)] = &[
    (0, "Display"),
    (1, "Retain power off status"),
];

pub static CANONCUSTOM_FUNCTIONS2_MEMOAUDIOQUALITY_VALUES: &[(i64, &str)] = &[
    (0, "High (48 kHz)"),
    (1, "Low (8 kHz)"),
];

pub static CANONCUSTOM_FUNCTIONS2_DEFAULTERASEOPTION_VALUES: &[(i64, &str)] = &[
    (0, "Cancel selected"),
    (1, "Erase selected"),
    (2, "Erase RAW selected"),
    (3, "Erase non-RAW selected"),
];

pub static CANONCUSTOM_FUNCTIONS2_RETRACTLENSONPOWEROFF_VALUES: &[(i64, &str)] = &[
    (0, "Enable"),
    (1, "Disable"),
];

pub static CANONCUSTOM_FUNCTIONS2_ADDIPTCINFORMATION_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Enable"),
];

pub static CANONCUSTOM_FUNCTIONS2_AUDIOCOMPRESSION_VALUES: &[(i64, &str)] = &[
    (0, "Enable"),
    (1, "Disable"),
];

pub static CANONCUSTOM_FUNCTIONS2_EXPOSURELEVELINCREMENTS_VALUES: &[(i64, &str)] = &[
    (0, "1/3-stop set, 1/3-stop comp."),
    (1, "1-stop set, 1/3-stop comp."),
    (2, "1/2-stop set, 1/2-stop comp."),
];

pub static CANONCUSTOM_FUNCTIONS2_ISOSPEEDINCREMENTS_VALUES: &[(i64, &str)] = &[
    (0, "1/3 Stop"),
    (1, "1 Stop"),
];

pub static CANONCUSTOM_FUNCTIONS2_AEBAUTOCANCEL_VALUES: &[(i64, &str)] = &[
    (0, "On"),
    (1, "Off"),
];

pub static CANONCUSTOM_FUNCTIONS2_AEBSEQUENCE_VALUES: &[(i64, &str)] = &[
    (0, "0,-,+"),
    (1, "-,0,+"),
    (2, "+,0,-"),
];

pub static CANONCUSTOM_FUNCTIONS2_AEBSHOTCOUNT_VALUES: &[(i64, &str)] = &[
    (2, "2 shots"),
    (3, "3 shots"),
    (5, "5 shots"),
    (7, "7 shots"),
];

pub static CANONCUSTOM_FUNCTIONS2_SPOTMETERLINKTOAFPOINT_VALUES: &[(i64, &str)] = &[
    (0, "Disable (use center AF point)"),
    (1, "Enable (use active AF point)"),
];

pub static CANONCUSTOM_FUNCTIONS2_SAFETYSHIFT_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Enable (Tv/Av)"),
    (2, "Enable (ISO speed)"),
];

pub static CANONCUSTOM_FUNCTIONS2_EXPOSUREMODEINMANUAL_VALUES: &[(i64, &str)] = &[
    (0, "Specified metering mode"),
    (1, "Evaluative metering"),
    (2, "Partial metering"),
    (3, "Spot metering"),
    (4, "Center-weighted average"),
];

pub static CANONCUSTOM_FUNCTIONS2_FLASHSYNCSPEEDAV_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "1/250 Fixed"),
];

pub static CANONCUSTOM_FUNCTIONS2_SAMEEXPOSUREFORNEWAPERTURE_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "ISO Speed"),
    (2, "Shutter Speed"),
];

pub static CANONCUSTOM_FUNCTIONS2_EXPOSURECOMPAUTOCANCEL_VALUES: &[(i64, &str)] = &[
    (0, "Enable"),
    (1, "Disable"),
];

pub static CANONCUSTOM_FUNCTIONS2_LONGEXPOSURENOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Auto"),
    (2, "On"),
];

pub static CANONCUSTOM_FUNCTIONS2_HIGHISONOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (1, "Low"),
    (2, "Strong"),
    (3, "Off"),
];

pub static CANONCUSTOM_FUNCTIONS2_HIGHLIGHTTONEPRIORITY_VALUES: &[(i64, &str)] = &[
    (0, "Disable"),
    (1, "Enable"),
];

pub static CANONCUSTOM_FUNCTIONS2_AUTOLIGHTINGOPTIMIZER_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (1, "Low"),
    (2, "Strong"),
    (3, "Disable"),
];

pub static CANONCUSTOM_FUNCTIONS2_ETTLII_VALUES: &[(i64, &str)] = &[
    (0, "Evaluative"),
    (1, "Average"),
];

pub static CANONCUSTOM_FUNCTIONS2_SHUTTERCURTAINSYNC_VALUES: &[(i64, &str)] = &[
    (0, "1st-curtain sync"),
    (1, "2nd-curtain sync"),
];

pub static CANONCUSTOM_FUNCTIONS2_FLASHFIRING_VALUES: &[(i64, &str)] = &[
    (0, "Fires"),
    (1, "Does not fire"),
];

/// CanonCustom::PersonalFuncValues tags
pub static CANONCUSTOM_PERSONALFUNCVALUES: phf::Map<u16, TagDef> = phf::phf_map! {
    4u16 => TagDef { name: "PF4ExposureTimeMin", values: None },
    5u16 => TagDef { name: "PF4ExposureTimeMax", values: None },
    6u16 => TagDef { name: "PF5ApertureMin", values: None },
    7u16 => TagDef { name: "PF5ApertureMax", values: None },
};

/// CanonCustom::PersonalFuncs tags
pub static CANONCUSTOM_PERSONALFUNCS: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "PF0CustomFuncRegistration", values: None },
    10u16 => TagDef { name: "PF9ChangeBracketSequence", values: None },
    11u16 => TagDef { name: "PF10RetainProgramShift", values: None },
    14u16 => TagDef { name: "PF13DrivePriority", values: None },
    15u16 => TagDef { name: "PF14DisableFocusSearch", values: None },
    16u16 => TagDef { name: "PF15DisableAFAssistBeam", values: None },
    17u16 => TagDef { name: "PF16AutoFocusPointShoot", values: None },
    18u16 => TagDef { name: "PF17DisableAFPointSel", values: None },
    19u16 => TagDef { name: "PF18EnableAutoAFPointSel", values: None },
    2u16 => TagDef { name: "PF1DisableShootingModes", values: None },
    20u16 => TagDef { name: "PF19ContinuousShootSpeed", values: None },
    21u16 => TagDef { name: "PF20LimitContinousShots", values: None },
    22u16 => TagDef { name: "PF21EnableQuietOperation", values: None },
    24u16 => TagDef { name: "PF23SetTimerLengths", values: None },
    25u16 => TagDef { name: "PF24LightLCDDuringBulb", values: None },
    26u16 => TagDef { name: "PF25DefaultClearSettings", values: None },
    27u16 => TagDef { name: "PF26ShortenReleaseLag", values: None },
    28u16 => TagDef { name: "PF27ReverseDialRotation", values: None },
    29u16 => TagDef { name: "PF28NoQuickDialExpComp", values: None },
    3u16 => TagDef { name: "PF2DisableMeteringModes", values: None },
    30u16 => TagDef { name: "PF29QuickDialSwitchOff", values: None },
    31u16 => TagDef { name: "PF30EnlargementMode", values: None },
    32u16 => TagDef { name: "PF31OriginalDecisionData", values: None },
    4u16 => TagDef { name: "PF3ManualExposureMetering", values: None },
    5u16 => TagDef { name: "PF4ExposureTimeLimits", values: None },
    6u16 => TagDef { name: "PF5ApertureLimits", values: None },
    7u16 => TagDef { name: "PF6PresetShootingModes", values: None },
    8u16 => TagDef { name: "PF7BracketContinuousShoot", values: None },
    9u16 => TagDef { name: "PF8SetBracketShots", values: None },
};


/// Look up a tag by ID in the main table.
pub fn lookup(_tag_id: u16) -> Option<&'static TagDef> {
    // Default to main table - override in specific modules
    None
}

//! Panasonic MakerNotes tag definitions.
//! Auto-generated from ExifTool - DO NOT EDIT

#![allow(dead_code)]

/// Tag definition with name and optional value mappings.
#[derive(Debug, Clone)]
pub struct TagDef {
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}

/// Panasonic::FaceDetInfo tags
pub static PANASONIC_FACEDETINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "NumFacePositions", values: None },
    1u16 => TagDef { name: "Face1Position", values: None },
    13u16 => TagDef { name: "Face4Position", values: None },
    17u16 => TagDef { name: "Face5Position", values: None },
    5u16 => TagDef { name: "Face2Position", values: None },
    9u16 => TagDef { name: "Face3Position", values: None },
};

/// Panasonic::FaceRecInfo tags
pub static PANASONIC_FACERECINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "FacesRecognized", values: None },
    100u16 => TagDef { name: "RecognizedFace3Name", values: None },
    120u16 => TagDef { name: "RecognizedFace3Position", values: None },
    128u16 => TagDef { name: "RecognizedFace3Age", values: None },
    24u16 => TagDef { name: "RecognizedFace1Position", values: None },
    32u16 => TagDef { name: "RecognizedFace1Age", values: None },
    4u16 => TagDef { name: "RecognizedFace1Name", values: None },
    52u16 => TagDef { name: "RecognizedFace2Name", values: None },
    72u16 => TagDef { name: "RecognizedFace2Position", values: None },
    80u16 => TagDef { name: "RecognizedFace2Age", values: None },
};

/// Panasonic::Main tags
pub static PANASONIC_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "ImageQuality", values: Some(PANASONIC_MAIN_IMAGEQUALITY_VALUES) },
    101u16 => TagDef { name: "Title", values: None },
    102u16 => TagDef { name: "BabyName", values: None },
    103u16 => TagDef { name: "Location", values: None },
    105u16 => TagDef { name: "Country", values: None },
    107u16 => TagDef { name: "State", values: None },
    109u16 => TagDef { name: "City", values: None },
    111u16 => TagDef { name: "Landmark", values: None },
    112u16 => TagDef { name: "IntelligentResolution", values: Some(PANASONIC_MAIN_INTELLIGENTRESOLUTION_VALUES) },
    118u16 => TagDef { name: "MergedImages", values: None },
    119u16 => TagDef { name: "BurstSpeed", values: None },
    121u16 => TagDef { name: "IntelligentD-Range", values: Some(PANASONIC_MAIN_INTELLIGENTD_RANGE_VALUES) },
    124u16 => TagDef { name: "ClearRetouch", values: Some(PANASONIC_MAIN_CLEARRETOUCH_VALUES) },
    128u16 => TagDef { name: "City2", values: None },
    134u16 => TagDef { name: "ManometerPressure", values: None },
    137u16 => TagDef { name: "PhotoStyle", values: Some(PANASONIC_MAIN_PHOTOSTYLE_VALUES) },
    138u16 => TagDef { name: "ShadingCompensation", values: Some(PANASONIC_MAIN_SHADINGCOMPENSATION_VALUES) },
    139u16 => TagDef { name: "WBShiftIntelligentAuto", values: None },
    140u16 => TagDef { name: "AccelerometerZ", values: None },
    141u16 => TagDef { name: "AccelerometerX", values: None },
    142u16 => TagDef { name: "AccelerometerY", values: None },
    143u16 => TagDef { name: "CameraOrientation", values: Some(PANASONIC_MAIN_CAMERAORIENTATION_VALUES) },
    144u16 => TagDef { name: "RollAngle", values: None },
    145u16 => TagDef { name: "PitchAngle", values: None },
    146u16 => TagDef { name: "WBShiftCreativeControl", values: None },
    147u16 => TagDef { name: "SweepPanoramaDirection", values: Some(PANASONIC_MAIN_SWEEPPANORAMADIRECTION_VALUES) },
    148u16 => TagDef { name: "SweepPanoramaFieldOfView", values: None },
    15u16 => TagDef { name: "AFAreaMode", values: None },
    150u16 => TagDef { name: "TimerRecording", values: Some(PANASONIC_MAIN_TIMERRECORDING_VALUES) },
    157u16 => TagDef { name: "InternalNDFilter", values: None },
    158u16 => TagDef { name: "HDR", values: Some(PANASONIC_MAIN_HDR_VALUES) },
    159u16 => TagDef { name: "ShutterType", values: Some(PANASONIC_MAIN_SHUTTERTYPE_VALUES) },
    161u16 => TagDef { name: "FilterEffect", values: None },
    163u16 => TagDef { name: "ClearRetouchValue", values: None },
    167u16 => TagDef { name: "OutputLUT", values: None },
    171u16 => TagDef { name: "TouchAE", values: Some(PANASONIC_MAIN_TOUCHAE_VALUES) },
    172u16 => TagDef { name: "MonochromeFilterEffect", values: Some(PANASONIC_MAIN_MONOCHROMEFILTEREFFECT_VALUES) },
    173u16 => TagDef { name: "HighlightShadow", values: None },
    175u16 => TagDef { name: "TimeStamp", values: None },
    179u16 => TagDef { name: "VideoBurstResolution", values: Some(PANASONIC_MAIN_VIDEOBURSTRESOLUTION_VALUES) },
    180u16 => TagDef { name: "MultiExposure", values: Some(PANASONIC_MAIN_MULTIEXPOSURE_VALUES) },
    185u16 => TagDef { name: "RedEyeRemoval", values: Some(PANASONIC_MAIN_REDEYEREMOVAL_VALUES) },
    187u16 => TagDef { name: "VideoBurstMode", values: Some(PANASONIC_MAIN_VIDEOBURSTMODE_VALUES) },
    188u16 => TagDef { name: "DiffractionCorrection", values: Some(PANASONIC_MAIN_DIFFRACTIONCORRECTION_VALUES) },
    189u16 => TagDef { name: "FocusBracket", values: None },
    190u16 => TagDef { name: "LongExposureNRUsed", values: Some(PANASONIC_MAIN_LONGEXPOSURENRUSED_VALUES) },
    191u16 => TagDef { name: "PostFocusMerging", values: None },
    193u16 => TagDef { name: "VideoPreburst", values: Some(PANASONIC_MAIN_VIDEOPREBURST_VALUES) },
    196u16 => TagDef { name: "LensTypeMake", values: None },
    197u16 => TagDef { name: "LensTypeModel", values: None },
    2u16 => TagDef { name: "FirmwareVersion", values: None },
    202u16 => TagDef { name: "SensorType", values: Some(PANASONIC_MAIN_SENSORTYPE_VALUES) },
    209u16 => TagDef { name: "ISO", values: None },
    210u16 => TagDef { name: "MonochromeGrainEffect", values: Some(PANASONIC_MAIN_MONOCHROMEGRAINEFFECT_VALUES) },
    214u16 => TagDef { name: "NoiseReductionStrength", values: None },
    222u16 => TagDef { name: "AFAreaSize", values: None },
    228u16 => TagDef { name: "LensTypeModel", values: None },
    232u16 => TagDef { name: "MinimumISO", values: None },
    233u16 => TagDef { name: "AFSubjectDetection", values: Some(PANASONIC_MAIN_AFSUBJECTDETECTION_VALUES) },
    238u16 => TagDef { name: "DynamicRangeBoost", values: Some(PANASONIC_MAIN_DYNAMICRANGEBOOST_VALUES) },
    241u16 => TagDef { name: "LUT1Name", values: None },
    243u16 => TagDef { name: "LUT1Opacity", values: None },
    244u16 => TagDef { name: "LUT2Name", values: None },
    245u16 => TagDef { name: "LUT2Opacity", values: None },
    26u16 => TagDef { name: "ImageStabilization", values: Some(PANASONIC_MAIN_IMAGESTABILIZATION_VALUES) },
    28u16 => TagDef { name: "MacroMode", values: Some(PANASONIC_MAIN_MACROMODE_VALUES) },
    3u16 => TagDef { name: "WhiteBalance", values: Some(PANASONIC_MAIN_WHITEBALANCE_VALUES) },
    31u16 => TagDef { name: "ShootingMode", values: Some(PANASONIC_MAIN_SHOOTINGMODE_VALUES) },
    32u16 => TagDef { name: "Audio", values: Some(PANASONIC_MAIN_AUDIO_VALUES) },
    32768u16 => TagDef { name: "MakerNoteVersion", values: None },
    32769u16 => TagDef { name: "SceneMode", values: Some(PANASONIC_MAIN_SCENEMODE_VALUES) },
    32770u16 => TagDef { name: "HighlightWarning", values: Some(PANASONIC_MAIN_HIGHLIGHTWARNING_VALUES) },
    32771u16 => TagDef { name: "DarkFocusEnvironment", values: Some(PANASONIC_MAIN_DARKFOCUSENVIRONMENT_VALUES) },
    32772u16 => TagDef { name: "WBRedLevel", values: None },
    32773u16 => TagDef { name: "WBGreenLevel", values: None },
    32774u16 => TagDef { name: "WBBlueLevel", values: None },
    32776u16 => TagDef { name: "TextStamp", values: Some(PANASONIC_MAIN_TEXTSTAMP_VALUES) },
    32777u16 => TagDef { name: "TextStamp", values: Some(PANASONIC_MAIN_TEXTSTAMP_VALUES) },
    32784u16 => TagDef { name: "BabyAge", values: None },
    32786u16 => TagDef { name: "Transform", values: None },
    33u16 => TagDef { name: "DataDump", values: None },
    35u16 => TagDef { name: "WhiteBalanceBias", values: None },
    3584u16 => TagDef { name: "PrintIM", values: None },
    36u16 => TagDef { name: "FlashBias", values: None },
    37u16 => TagDef { name: "InternalSerialNumber", values: None },
    38u16 => TagDef { name: "PanasonicExifVersion", values: None },
    39u16 => TagDef { name: "VideoFrameRate", values: Some(PANASONIC_MAIN_VIDEOFRAMERATE_VALUES) },
    40u16 => TagDef { name: "ColorEffect", values: Some(PANASONIC_MAIN_COLOREFFECT_VALUES) },
    41u16 => TagDef { name: "TimeSincePowerOn", values: None },
    42u16 => TagDef { name: "BurstMode", values: Some(PANASONIC_MAIN_BURSTMODE_VALUES) },
    43u16 => TagDef { name: "SequenceNumber", values: None },
    44u16 => TagDef { name: "ContrastMode", values: Some(PANASONIC_MAIN_CONTRASTMODE_VALUES) },
    45u16 => TagDef { name: "NoiseReduction", values: Some(PANASONIC_MAIN_NOISEREDUCTION_VALUES) },
    46u16 => TagDef { name: "SelfTimer", values: Some(PANASONIC_MAIN_SELFTIMER_VALUES) },
    48u16 => TagDef { name: "Rotation", values: Some(PANASONIC_MAIN_ROTATION_VALUES) },
    49u16 => TagDef { name: "AFAssistLamp", values: Some(PANASONIC_MAIN_AFASSISTLAMP_VALUES) },
    50u16 => TagDef { name: "ColorMode", values: Some(PANASONIC_MAIN_COLORMODE_VALUES) },
    51u16 => TagDef { name: "BabyAge", values: None },
    52u16 => TagDef { name: "OpticalZoomMode", values: Some(PANASONIC_MAIN_OPTICALZOOMMODE_VALUES) },
    53u16 => TagDef { name: "ConversionLens", values: Some(PANASONIC_MAIN_CONVERSIONLENS_VALUES) },
    54u16 => TagDef { name: "TravelDay", values: None },
    56u16 => TagDef { name: "BatteryLevel", values: Some(PANASONIC_MAIN_BATTERYLEVEL_VALUES) },
    57u16 => TagDef { name: "Contrast", values: Some(PANASONIC_MAIN_CONTRAST_VALUES) },
    58u16 => TagDef { name: "WorldTimeLocation", values: Some(PANASONIC_MAIN_WORLDTIMELOCATION_VALUES) },
    59u16 => TagDef { name: "TextStamp", values: Some(PANASONIC_MAIN_TEXTSTAMP_VALUES) },
    60u16 => TagDef { name: "ProgramISO", values: Some(PANASONIC_MAIN_PROGRAMISO_VALUES) },
    61u16 => TagDef { name: "AdvancedSceneType", values: None },
    62u16 => TagDef { name: "TextStamp", values: Some(PANASONIC_MAIN_TEXTSTAMP_VALUES) },
    63u16 => TagDef { name: "FacesDetected", values: None },
    64u16 => TagDef { name: "Saturation", values: Some(PANASONIC_MAIN_SATURATION_VALUES) },
    65u16 => TagDef { name: "Sharpness", values: Some(PANASONIC_MAIN_SHARPNESS_VALUES) },
    66u16 => TagDef { name: "FilmMode", values: Some(PANASONIC_MAIN_FILMMODE_VALUES) },
    67u16 => TagDef { name: "JPEGQuality", values: Some(PANASONIC_MAIN_JPEGQUALITY_VALUES) },
    68u16 => TagDef { name: "ColorTempKelvin", values: None },
    69u16 => TagDef { name: "BracketSettings", values: Some(PANASONIC_MAIN_BRACKETSETTINGS_VALUES) },
    7u16 => TagDef { name: "FocusMode", values: Some(PANASONIC_MAIN_FOCUSMODE_VALUES) },
    70u16 => TagDef { name: "WBShiftAB", values: None },
    71u16 => TagDef { name: "WBShiftGM", values: None },
    72u16 => TagDef { name: "FlashCurtain", values: Some(PANASONIC_MAIN_FLASHCURTAIN_VALUES) },
    73u16 => TagDef { name: "LongExposureNoiseReduction", values: Some(PANASONIC_MAIN_LONGEXPOSURENOISEREDUCTION_VALUES) },
    75u16 => TagDef { name: "PanasonicImageWidth", values: None },
    76u16 => TagDef { name: "PanasonicImageHeight", values: None },
    77u16 => TagDef { name: "AFPointPosition", values: None },
    78u16 => TagDef { name: "FaceDetInfo", values: None },
    81u16 => TagDef { name: "LensType", values: None },
    8195u16 => TagDef { name: "TimeInfo", values: None },
    82u16 => TagDef { name: "LensSerialNumber", values: None },
    83u16 => TagDef { name: "AccessoryType", values: None },
    84u16 => TagDef { name: "AccessorySerialNumber", values: None },
    89u16 => TagDef { name: "Transform", values: None },
    93u16 => TagDef { name: "IntelligentExposure", values: Some(PANASONIC_MAIN_INTELLIGENTEXPOSURE_VALUES) },
    96u16 => TagDef { name: "LensFirmwareVersion", values: None },
    97u16 => TagDef { name: "FaceRecInfo", values: None },
    98u16 => TagDef { name: "FlashWarning", values: Some(PANASONIC_MAIN_FLASHWARNING_VALUES) },
    99u16 => TagDef { name: "RecognizedFaceFlags", values: None },
};

pub static PANASONIC_MAIN_IMAGEQUALITY_VALUES: &[(i64, &str)] = &[
    (1, "TIFF"),
    (11, "Full HD Movie"),
    (12, "4k Movie"),
    (2, "High"),
    (3, "Normal"),
    (6, "Very High"),
    (7, "RAW"),
    (9, "Motion Picture"),
];

pub static PANASONIC_MAIN_INTELLIGENTRESOLUTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Low"),
    (2, "Standard"),
    (3, "High"),
    (4, "Extended"),
];

pub static PANASONIC_MAIN_INTELLIGENTD_RANGE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Low"),
    (2, "Standard"),
    (3, "High"),
];

pub static PANASONIC_MAIN_CLEARRETOUCH_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static PANASONIC_MAIN_PHOTOSTYLE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Standard or Custom"),
    (11, "L. Monochrome"),
    (12, "Like709"),
    (15, "L. Monochrome D"),
    (17, "V-Log"),
    (18, "Cinelike D2"),
    (2, "Vivid"),
    (3, "Natural"),
    (4, "Monochrome"),
    (5, "Scenery"),
    (6, "Portrait"),
    (8, "Cinelike D"),
    (9, "Cinelike V"),
];

pub static PANASONIC_MAIN_SHADINGCOMPENSATION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static PANASONIC_MAIN_CAMERAORIENTATION_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Rotate CW"),
    (2, "Rotate 180"),
    (3, "Rotate CCW"),
    (4, "Tilt Upwards"),
    (5, "Tilt Downwards"),
];

pub static PANASONIC_MAIN_SWEEPPANORAMADIRECTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Left to Right"),
    (2, "Right to Left"),
    (3, "Top to Bottom"),
    (4, "Bottom to Top"),
];

pub static PANASONIC_MAIN_TIMERRECORDING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Time Lapse"),
    (2, "Stop-motion Animation"),
    (3, "Focus Bracketing"),
];

pub static PANASONIC_MAIN_HDR_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (100, "1 EV"),
    (200, "2 EV"),
    (300, "3 EV"),
    (32868, "1 EV (Auto)"),
    (32968, "2 EV (Auto)"),
    (33068, "3 EV (Auto)"),
];

pub static PANASONIC_MAIN_SHUTTERTYPE_VALUES: &[(i64, &str)] = &[
    (0, "Mechanical"),
    (1, "Electronic"),
    (2, "Hybrid"),
];

pub static PANASONIC_MAIN_TOUCHAE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static PANASONIC_MAIN_MONOCHROMEFILTEREFFECT_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Yellow"),
    (2, "Orange"),
    (3, "Red"),
    (4, "Green"),
];

pub static PANASONIC_MAIN_VIDEOBURSTRESOLUTION_VALUES: &[(i64, &str)] = &[
    (1, "Off or 4K"),
    (4, "6K"),
];

pub static PANASONIC_MAIN_MULTIEXPOSURE_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "Off"),
    (2, "On"),
];

pub static PANASONIC_MAIN_REDEYEREMOVAL_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static PANASONIC_MAIN_VIDEOBURSTMODE_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (1032, "Focus Stacking"),
    (2064, "6K Burst"),
    (2080, "6K Burst (Start/Stop)"),
    (24, "4K Burst"),
    (264, "Loop Recording"),
    (4, "Post Focus"),
    (40, "4K Burst (Start/Stop)"),
    (4097, "High Resolution Mode"),
    (72, "4K Pre-burst"),
];

pub static PANASONIC_MAIN_DIFFRACTIONCORRECTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Auto"),
];

pub static PANASONIC_MAIN_LONGEXPOSURENRUSED_VALUES: &[(i64, &str)] = &[
    (1, "No"),
    (2, "Yes"),
];

pub static PANASONIC_MAIN_VIDEOPREBURST_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "4K or 6K"),
];

pub static PANASONIC_MAIN_SENSORTYPE_VALUES: &[(i64, &str)] = &[
    (0, "Multi-aspect"),
    (1, "Standard"),
];

pub static PANASONIC_MAIN_MONOCHROMEGRAINEFFECT_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Low"),
    (2, "Standard"),
    (3, "High"),
];

pub static PANASONIC_MAIN_AFSUBJECTDETECTION_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "Human Eye/Face/Body"),
    (10, "Train"),
    (11, "Train (main part priority)"),
    (12, "Airplane"),
    (13, "Airplane (nose priority)"),
    (2, "Animal"),
    (3, "Human Eye/Face"),
    (4, "Animal Body"),
    (5, "Animal Eye/Body"),
    (6, "Car"),
    (7, "Motorcycle"),
    (8, "Car (main part priority)"),
    (9, "Motorcycle (helmet priority)"),
];

pub static PANASONIC_MAIN_DYNAMICRANGEBOOST_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static PANASONIC_MAIN_IMAGESTABILIZATION_VALUES: &[(i64, &str)] = &[
    (10, "Dual IS Panning"),
    (11, "Dual2 IS"),
    (12, "Dual2 IS Panning"),
    (2, "On, Optical"),
    (3, "Off"),
    (4, "On, Mode 2"),
    (5, "On, Optical Panning"),
    (6, "On, Body-only"),
    (7, "On, Body-only Panning"),
    (9, "Dual IS"),
];

pub static PANASONIC_MAIN_MACROMODE_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
    (257, "Tele-Macro"),
    (513, "Macro Zoom"),
];

pub static PANASONIC_MAIN_WHITEBALANCE_VALUES: &[(i64, &str)] = &[
    (1, "Auto"),
    (10, "Black & White"),
    (11, "Manual 2"),
    (12, "Shade"),
    (13, "Kelvin"),
    (14, "Manual 3"),
    (15, "Manual 4"),
    (19, "Auto (cool)"),
    (2, "Daylight"),
    (3, "Cloudy"),
    (4, "Incandescent"),
    (5, "Manual"),
    (8, "Flash"),
];

pub static PANASONIC_MAIN_SHOOTINGMODE_VALUES: &[(i64, &str)] = &[
    (1, "Normal"),
    (10, "Spot"),
    (11, "Manual"),
    (12, "Movie Preview"),
    (13, "Panning"),
    (14, "Simple"),
    (15, "Color Effects"),
    (16, "Self Portrait"),
    (17, "Economy"),
    (18, "Fireworks"),
    (19, "Party"),
    (2, "Portrait"),
    (20, "Snow"),
    (21, "Night Scenery"),
    (22, "Food"),
    (23, "Baby"),
    (24, "Soft Skin"),
    (25, "Candlelight"),
    (26, "Starry Night"),
    (27, "High Sensitivity"),
    (28, "Panorama Assist"),
    (29, "Underwater"),
    (3, "Scenery"),
    (30, "Beach"),
    (31, "Aerial Photo"),
    (32, "Sunset"),
    (33, "Pet"),
    (34, "Intelligent ISO"),
    (35, "Clipboard"),
    (36, "High Speed Continuous Shooting"),
    (37, "Intelligent Auto"),
    (39, "Multi-aspect"),
    (4, "Sports"),
    (41, "Transform"),
    (42, "Flash Burst"),
    (43, "Pin Hole"),
    (44, "Film Grain"),
    (45, "My Color"),
    (46, "Photo Frame"),
    (48, "Movie"),
    (5, "Night Portrait"),
    (51, "HDR"),
    (52, "Peripheral Defocus"),
    (55, "Handheld Night Shot"),
    (57, "3D"),
    (59, "Creative Control"),
    (6, "Program"),
    (60, "Intelligent Auto Plus"),
    (62, "Panorama"),
    (63, "Glass Through"),
    (64, "HDR"),
    (66, "Digital Filter"),
    (67, "Clear Portrait"),
    (68, "Silky Skin"),
    (69, "Backlit Softness"),
    (7, "Aperture Priority"),
    (70, "Clear in Backlight"),
    (71, "Relaxing Tone"),
    (72, "Sweet Child's Face"),
    (73, "Distinct Scenery"),
    (74, "Bright Blue Sky"),
    (75, "Romantic Sunset Glow"),
    (76, "Vivid Sunset Glow"),
    (77, "Glistening Water"),
    (78, "Clear Nightscape"),
    (79, "Cool Night Sky"),
    (8, "Shutter Priority"),
    (80, "Warm Glowing Nightscape"),
    (81, "Artistic Nightscape"),
    (82, "Glittering Illuminations"),
    (83, "Clear Night Portrait"),
    (84, "Soft Image of a Flower"),
    (85, "Appetizing Food"),
    (86, "Cute Dessert"),
    (87, "Freeze Animal Motion"),
    (88, "Clear Sports Shot"),
    (89, "Monochrome"),
    (9, "Macro"),
    (90, "Creative Control"),
    (92, "Handheld Night Shot"),
];

pub static PANASONIC_MAIN_AUDIO_VALUES: &[(i64, &str)] = &[
    (1, "Yes"),
    (2, "No"),
    (3, "Stereo"),
];

pub static PANASONIC_MAIN_SCENEMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Normal"),
    (10, "Spot"),
    (11, "Manual"),
    (12, "Movie Preview"),
    (13, "Panning"),
    (14, "Simple"),
    (15, "Color Effects"),
    (16, "Self Portrait"),
    (17, "Economy"),
    (18, "Fireworks"),
    (19, "Party"),
    (2, "Portrait"),
    (20, "Snow"),
    (21, "Night Scenery"),
    (22, "Food"),
    (23, "Baby"),
    (24, "Soft Skin"),
    (25, "Candlelight"),
    (26, "Starry Night"),
    (27, "High Sensitivity"),
    (28, "Panorama Assist"),
    (29, "Underwater"),
    (3, "Scenery"),
    (30, "Beach"),
    (31, "Aerial Photo"),
    (32, "Sunset"),
    (33, "Pet"),
    (34, "Intelligent ISO"),
    (35, "Clipboard"),
    (36, "High Speed Continuous Shooting"),
    (37, "Intelligent Auto"),
    (39, "Multi-aspect"),
    (4, "Sports"),
    (41, "Transform"),
    (42, "Flash Burst"),
    (43, "Pin Hole"),
    (44, "Film Grain"),
    (45, "My Color"),
    (46, "Photo Frame"),
    (48, "Movie"),
    (5, "Night Portrait"),
    (51, "HDR"),
    (52, "Peripheral Defocus"),
    (55, "Handheld Night Shot"),
    (57, "3D"),
    (59, "Creative Control"),
    (6, "Program"),
    (60, "Intelligent Auto Plus"),
    (62, "Panorama"),
    (63, "Glass Through"),
    (64, "HDR"),
    (66, "Digital Filter"),
    (67, "Clear Portrait"),
    (68, "Silky Skin"),
    (69, "Backlit Softness"),
    (7, "Aperture Priority"),
    (70, "Clear in Backlight"),
    (71, "Relaxing Tone"),
    (72, "Sweet Child's Face"),
    (73, "Distinct Scenery"),
    (74, "Bright Blue Sky"),
    (75, "Romantic Sunset Glow"),
    (76, "Vivid Sunset Glow"),
    (77, "Glistening Water"),
    (78, "Clear Nightscape"),
    (79, "Cool Night Sky"),
    (8, "Shutter Priority"),
    (80, "Warm Glowing Nightscape"),
    (81, "Artistic Nightscape"),
    (82, "Glittering Illuminations"),
    (83, "Clear Night Portrait"),
    (84, "Soft Image of a Flower"),
    (85, "Appetizing Food"),
    (86, "Cute Dessert"),
    (87, "Freeze Animal Motion"),
    (88, "Clear Sports Shot"),
    (89, "Monochrome"),
    (9, "Macro"),
    (90, "Creative Control"),
    (92, "Handheld Night Shot"),
];

pub static PANASONIC_MAIN_HIGHLIGHTWARNING_VALUES: &[(i64, &str)] = &[
    (0, "Disabled"),
    (1, "No"),
    (2, "Yes"),
];

pub static PANASONIC_MAIN_DARKFOCUSENVIRONMENT_VALUES: &[(i64, &str)] = &[
    (1, "No"),
    (2, "Yes"),
];

pub static PANASONIC_MAIN_TEXTSTAMP_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "On"),
];

pub static PANASONIC_MAIN_VIDEOFRAMERATE_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
];

pub static PANASONIC_MAIN_COLOREFFECT_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "Warm"),
    (3, "Cool"),
    (4, "Black & White"),
    (5, "Sepia"),
    (6, "Happy"),
    (8, "Vivid"),
];

pub static PANASONIC_MAIN_BURSTMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
    (17, "On (with flash)"),
    (18, "Aperture Bracketing"),
    (2, "Auto Exposure Bracketing (AEB)"),
    (3, "Focus Bracketing"),
    (4, "Unlimited"),
    (8, "White Balance Bracketing"),
];

pub static PANASONIC_MAIN_CONTRASTMODE_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Low"),
    (13, "High Dynamic"),
    (2, "High"),
    (24, "Dynamic Range (film-like)"),
    (256, "Low"),
    (272, "Normal"),
    (288, "High"),
    (46, "Match Filter Effects Toy"),
    (5, "Normal 2"),
    (55, "Match Photo Style L. Monochrome"),
    (6, "Medium Low"),
    (7, "Medium High"),
];

pub static PANASONIC_MAIN_NOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (1, "Low (-1)"),
    (2, "High (+1)"),
    (3, "Lowest (-2)"),
    (4, "Highest (+2)"),
    (5, "+5"),
    (6, "+6"),
    (65531, "-5"),
    (65532, "-4"),
    (65533, "-3"),
    (65534, "-2"),
    (65535, "-1"),
];

pub static PANASONIC_MAIN_SELFTIMER_VALUES: &[(i64, &str)] = &[
    (0, "Off (0)"),
    (1, "Off"),
    (2, "10 s"),
    (258, "2 s after shutter pressed"),
    (266, "10 s after shutter pressed"),
    (3, "2 s"),
    (4, "10 s / 3 pictures"),
    (778, "3 photos after 10 s"),
];

pub static PANASONIC_MAIN_ROTATION_VALUES: &[(i64, &str)] = &[
    (1, "Horizontal (normal)"),
    (3, "Rotate 180"),
    (6, "Rotate 90 CW"),
    (8, "Rotate 270 CW"),
];

pub static PANASONIC_MAIN_AFASSISTLAMP_VALUES: &[(i64, &str)] = &[
    (1, "Fired"),
    (2, "Enabled but Not Used"),
    (3, "Disabled but Required"),
    (4, "Disabled and Not Required"),
];

pub static PANASONIC_MAIN_COLORMODE_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Natural"),
    (2, "Vivid"),
];

pub static PANASONIC_MAIN_OPTICALZOOMMODE_VALUES: &[(i64, &str)] = &[
    (1, "Standard"),
    (2, "Extended"),
];

pub static PANASONIC_MAIN_CONVERSIONLENS_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "Wide"),
    (3, "Telephoto"),
    (4, "Macro"),
];

pub static PANASONIC_MAIN_BATTERYLEVEL_VALUES: &[(i64, &str)] = &[
    (1, "Full"),
    (2, "Medium"),
    (256, "n/a"),
    (3, "Low"),
    (4, "Near Empty"),
    (7, "Near Full"),
    (8, "Medium Low"),
];

pub static PANASONIC_MAIN_CONTRAST_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
];

pub static PANASONIC_MAIN_WORLDTIMELOCATION_VALUES: &[(i64, &str)] = &[
    (1, "Home"),
    (2, "Destination"),
];

pub static PANASONIC_MAIN_PROGRAMISO_VALUES: &[(i64, &str)] = &[
    (-1, "n/a"),
    (65534, "Intelligent ISO"),
    (65535, "n/a"),
];

pub static PANASONIC_MAIN_SATURATION_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
];

pub static PANASONIC_MAIN_SHARPNESS_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
];

pub static PANASONIC_MAIN_FILMMODE_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "Standard (color)"),
    (10, "Nostalgic"),
    (11, "Vibrant"),
    (2, "Dynamic (color)"),
    (3, "Nature (color)"),
    (4, "Smooth (color)"),
    (5, "Standard (B&W)"),
    (6, "Dynamic (B&W)"),
    (7, "Smooth (B&W)"),
];

pub static PANASONIC_MAIN_JPEGQUALITY_VALUES: &[(i64, &str)] = &[
    (0, "n/a (Movie)"),
    (2, "High"),
    (255, "n/a (RAW only)"),
    (3, "Standard"),
    (6, "Very High"),
];

pub static PANASONIC_MAIN_BRACKETSETTINGS_VALUES: &[(i64, &str)] = &[
    (0, "No Bracket"),
    (1, "3 Images, Sequence 0/-/+"),
    (2, "3 Images, Sequence -/0/+"),
    (3, "5 Images, Sequence 0/-/+"),
    (4, "5 Images, Sequence -/0/+"),
    (5, "7 Images, Sequence 0/-/+"),
    (6, "7 Images, Sequence -/0/+"),
];

pub static PANASONIC_MAIN_FOCUSMODE_VALUES: &[(i64, &str)] = &[
    (1, "Auto"),
    (2, "Manual"),
    (4, "Auto, Focus button"),
    (5, "Auto, Continuous"),
    (6, "AF-S"),
    (7, "AF-C"),
    (8, "AF-F"),
];

pub static PANASONIC_MAIN_FLASHCURTAIN_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "1st"),
    (2, "2nd"),
];

pub static PANASONIC_MAIN_LONGEXPOSURENOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "On"),
];

pub static PANASONIC_MAIN_INTELLIGENTEXPOSURE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Low"),
    (2, "Standard"),
    (3, "High"),
];

pub static PANASONIC_MAIN_FLASHWARNING_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes (flash required but disabled)"),
];

/// Panasonic::TimeInfo tags
pub static PANASONIC_TIMEINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "PanasonicDateTime", values: None },
    16u16 => TagDef { name: "TimeLapseShotNumber", values: None },
};


/// Look up a tag by ID in the main table.
pub fn lookup(_tag_id: u16) -> Option<&'static TagDef> {
    // Default to main table - override in specific modules
    None
}

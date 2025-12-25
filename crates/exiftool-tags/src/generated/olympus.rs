//! Olympus MakerNotes tag definitions.
//! Auto-generated from ExifTool - DO NOT EDIT

#![allow(dead_code)]

/// Tag definition with name and optional value mappings.
#[derive(Debug, Clone)]
pub struct TagDef {
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}

/// Minolta::CameraSettings tags
pub static MINOLTA_CAMERASETTINGS: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "ExposureMode", values: Some(MINOLTA_CAMERASETTINGS_EXPOSUREMODE_VALUES) },
    10u16 => TagDef { name: "FNumber", values: None },
    11u16 => TagDef { name: "MacroMode", values: Some(MINOLTA_CAMERASETTINGS_MACROMODE_VALUES) },
    12u16 => TagDef { name: "DigitalZoom", values: Some(MINOLTA_CAMERASETTINGS_DIGITALZOOM_VALUES) },
    13u16 => TagDef { name: "ExposureCompensation", values: None },
    14u16 => TagDef { name: "BracketStep", values: Some(MINOLTA_CAMERASETTINGS_BRACKETSTEP_VALUES) },
    18u16 => TagDef { name: "FocalLength", values: None },
    19u16 => TagDef { name: "FocusDistance", values: None },
    2u16 => TagDef { name: "FlashMode", values: Some(MINOLTA_CAMERASETTINGS_FLASHMODE_VALUES) },
    20u16 => TagDef { name: "FlashFired", values: Some(MINOLTA_CAMERASETTINGS_FLASHFIRED_VALUES) },
    21u16 => TagDef { name: "MinoltaDate", values: None },
    22u16 => TagDef { name: "MinoltaTime", values: None },
    23u16 => TagDef { name: "MaxAperture", values: None },
    26u16 => TagDef { name: "FileNumberMemory", values: Some(MINOLTA_CAMERASETTINGS_FILENUMBERMEMORY_VALUES) },
    28u16 => TagDef { name: "ColorBalanceRed", values: None },
    29u16 => TagDef { name: "ColorBalanceGreen", values: None },
    3u16 => TagDef { name: "WhiteBalance", values: None },
    30u16 => TagDef { name: "ColorBalanceBlue", values: None },
    31u16 => TagDef { name: "Saturation", values: Some(MINOLTA_CAMERASETTINGS_SATURATION_VALUES) },
    32u16 => TagDef { name: "Contrast", values: Some(MINOLTA_CAMERASETTINGS_CONTRAST_VALUES) },
    33u16 => TagDef { name: "Sharpness", values: Some(MINOLTA_CAMERASETTINGS_SHARPNESS_VALUES) },
    34u16 => TagDef { name: "SubjectProgram", values: Some(MINOLTA_CAMERASETTINGS_SUBJECTPROGRAM_VALUES) },
    35u16 => TagDef { name: "FlashExposureComp", values: None },
    36u16 => TagDef { name: "ISOSetting", values: Some(MINOLTA_CAMERASETTINGS_ISOSETTING_VALUES) },
    37u16 => TagDef { name: "MinoltaModelID", values: Some(MINOLTA_CAMERASETTINGS_MINOLTAMODELID_VALUES) },
    38u16 => TagDef { name: "IntervalMode", values: Some(MINOLTA_CAMERASETTINGS_INTERVALMODE_VALUES) },
    39u16 => TagDef { name: "FolderName", values: Some(MINOLTA_CAMERASETTINGS_FOLDERNAME_VALUES) },
    4u16 => TagDef { name: "MinoltaImageSize", values: Some(MINOLTA_CAMERASETTINGS_MINOLTAIMAGESIZE_VALUES) },
    40u16 => TagDef { name: "ColorMode", values: Some(MINOLTA_CAMERASETTINGS_COLORMODE_VALUES) },
    41u16 => TagDef { name: "ColorFilter", values: None },
    43u16 => TagDef { name: "InternalFlash", values: Some(MINOLTA_CAMERASETTINGS_INTERNALFLASH_VALUES) },
    44u16 => TagDef { name: "Brightness", values: None },
    47u16 => TagDef { name: "WideFocusZone", values: Some(MINOLTA_CAMERASETTINGS_WIDEFOCUSZONE_VALUES) },
    48u16 => TagDef { name: "FocusMode", values: Some(MINOLTA_CAMERASETTINGS_FOCUSMODE_VALUES) },
    49u16 => TagDef { name: "FocusArea", values: Some(MINOLTA_CAMERASETTINGS_FOCUSAREA_VALUES) },
    5u16 => TagDef { name: "MinoltaQuality", values: Some(MINOLTA_CAMERASETTINGS_MINOLTAQUALITY_VALUES) },
    50u16 => TagDef { name: "DECPosition", values: Some(MINOLTA_CAMERASETTINGS_DECPOSITION_VALUES) },
    51u16 => TagDef { name: "ColorProfile", values: Some(MINOLTA_CAMERASETTINGS_COLORPROFILE_VALUES) },
    52u16 => TagDef { name: "DataImprint", values: Some(MINOLTA_CAMERASETTINGS_DATAIMPRINT_VALUES) },
    6u16 => TagDef { name: "DriveMode", values: Some(MINOLTA_CAMERASETTINGS_DRIVEMODE_VALUES) },
    63u16 => TagDef { name: "FlashMetering", values: Some(MINOLTA_CAMERASETTINGS_FLASHMETERING_VALUES) },
    7u16 => TagDef { name: "MeteringMode", values: Some(MINOLTA_CAMERASETTINGS_METERINGMODE_VALUES) },
    8u16 => TagDef { name: "ISO", values: None },
    9u16 => TagDef { name: "ExposureTime", values: None },
};

pub static MINOLTA_CAMERASETTINGS_EXPOSUREMODE_VALUES: &[(i64, &str)] = &[
    (0, "Program"),
    (1, "Aperture Priority"),
    (2, "Shutter Priority"),
    (3, "Manual"),
];

pub static MINOLTA_CAMERASETTINGS_MACROMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static MINOLTA_CAMERASETTINGS_DIGITALZOOM_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Electronic magnification"),
    (2, "2x"),
];

pub static MINOLTA_CAMERASETTINGS_BRACKETSTEP_VALUES: &[(i64, &str)] = &[
    (0, "1/3 EV"),
    (1, "2/3 EV"),
    (2, "1 EV"),
];

pub static MINOLTA_CAMERASETTINGS_FLASHMODE_VALUES: &[(i64, &str)] = &[
    (0, "Fill flash"),
    (1, "Red-eye reduction"),
    (2, "Rear flash sync"),
    (3, "Wireless"),
    (4, "Off?"),
];

pub static MINOLTA_CAMERASETTINGS_FLASHFIRED_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];

pub static MINOLTA_CAMERASETTINGS_FILENUMBERMEMORY_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static MINOLTA_CAMERASETTINGS_SATURATION_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
];

pub static MINOLTA_CAMERASETTINGS_CONTRAST_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
];

pub static MINOLTA_CAMERASETTINGS_SHARPNESS_VALUES: &[(i64, &str)] = &[
    (0, "Hard"),
    (1, "Normal"),
    (2, "Soft"),
];

pub static MINOLTA_CAMERASETTINGS_SUBJECTPROGRAM_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "Portrait"),
    (2, "Text"),
    (3, "Night portrait"),
    (4, "Sunset"),
    (5, "Sports action"),
];

pub static MINOLTA_CAMERASETTINGS_ISOSETTING_VALUES: &[(i64, &str)] = &[
    (0, "100"),
    (1, "200"),
    (2, "400"),
    (3, "800"),
    (4, "Auto"),
    (5, "64"),
];

pub static MINOLTA_CAMERASETTINGS_MINOLTAMODELID_VALUES: &[(i64, &str)] = &[
    (0, "DiMAGE 7, X1, X21 or X31"),
    (1, "DiMAGE 5"),
    (2, "DiMAGE S304"),
    (3, "DiMAGE S404"),
    (4, "DiMAGE 7i"),
    (5, "DiMAGE 7Hi"),
    (6, "DiMAGE A1"),
    (7, "DiMAGE A2 or S414"),
];

pub static MINOLTA_CAMERASETTINGS_INTERVALMODE_VALUES: &[(i64, &str)] = &[
    (0, "Still Image"),
    (1, "Time-lapse Movie"),
];

pub static MINOLTA_CAMERASETTINGS_FOLDERNAME_VALUES: &[(i64, &str)] = &[
    (0, "Standard Form"),
    (1, "Data Form"),
];

pub static MINOLTA_CAMERASETTINGS_MINOLTAIMAGESIZE_VALUES: &[(i64, &str)] = &[
    (0, "Full"),
    (1, "1600x1200"),
    (2, "1280x960"),
    (3, "640x480"),
    (6, "2080x1560"),
    (7, "2560x1920"),
    (8, "3264x2176"),
];

pub static MINOLTA_CAMERASETTINGS_COLORMODE_VALUES: &[(i64, &str)] = &[
    (0, "Natural color"),
    (1, "Black & White"),
    (2, "Vivid color"),
    (3, "Solarization"),
    (4, "Adobe RGB"),
];

pub static MINOLTA_CAMERASETTINGS_INTERNALFLASH_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Fired"),
];

pub static MINOLTA_CAMERASETTINGS_WIDEFOCUSZONE_VALUES: &[(i64, &str)] = &[
    (0, "No zone"),
    (1, "Center zone (horizontal orientation)"),
    (2, "Center zone (vertical orientation)"),
    (3, "Left zone"),
    (4, "Right zone"),
];

pub static MINOLTA_CAMERASETTINGS_FOCUSMODE_VALUES: &[(i64, &str)] = &[
    (0, "AF"),
    (1, "MF"),
];

pub static MINOLTA_CAMERASETTINGS_FOCUSAREA_VALUES: &[(i64, &str)] = &[
    (0, "Wide Focus (normal)"),
    (1, "Spot Focus"),
];

pub static MINOLTA_CAMERASETTINGS_MINOLTAQUALITY_VALUES: &[(i64, &str)] = &[
    (0, "Raw"),
    (1, "Super Fine"),
    (2, "Fine"),
    (3, "Standard"),
    (4, "Economy"),
    (5, "Extra Fine"),
];

pub static MINOLTA_CAMERASETTINGS_DECPOSITION_VALUES: &[(i64, &str)] = &[
    (0, "Exposure"),
    (1, "Contrast"),
    (2, "Saturation"),
    (3, "Filter"),
];

pub static MINOLTA_CAMERASETTINGS_COLORPROFILE_VALUES: &[(i64, &str)] = &[
    (0, "Not Embedded"),
    (1, "Embedded"),
];

pub static MINOLTA_CAMERASETTINGS_DATAIMPRINT_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "YYYY/MM/DD"),
    (2, "MM/DD/HH:MM"),
    (3, "Text"),
    (4, "Text + ID#"),
];

pub static MINOLTA_CAMERASETTINGS_DRIVEMODE_VALUES: &[(i64, &str)] = &[
    (0, "Single"),
    (1, "Continuous"),
    (2, "Self-timer"),
    (4, "Bracketing"),
    (5, "Interval"),
    (6, "UHS continuous"),
    (7, "HS continuous"),
];

pub static MINOLTA_CAMERASETTINGS_FLASHMETERING_VALUES: &[(i64, &str)] = &[
    (0, "ADI (Advanced Distance Integration)"),
    (1, "Pre-flash TTL"),
    (2, "Manual flash control"),
];

pub static MINOLTA_CAMERASETTINGS_METERINGMODE_VALUES: &[(i64, &str)] = &[
    (0, "Multi-segment"),
    (1, "Center-weighted average"),
    (2, "Spot"),
];

/// Olympus::AFInfo tags
pub static OLYMPUS_AFINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    1580u16 => TagDef { name: "CAFSensitivity", values: None },
};

/// Olympus::AFTargetInfo tags
pub static OLYMPUS_AFTARGETINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "AFFrameSize", values: None },
    2u16 => TagDef { name: "AFFocusArea", values: None },
    6u16 => TagDef { name: "AFSelectedArea", values: None },
};

/// Olympus::CameraSettings tags
pub static OLYMPUS_CAMERASETTINGS: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "CameraSettingsVersion", values: None },
    1024u16 => TagDef { name: "FlashMode", values: Some(OLYMPUS_CAMERASETTINGS_FLASHMODE_VALUES) },
    1025u16 => TagDef { name: "FlashExposureComp", values: None },
    1027u16 => TagDef { name: "FlashRemoteControl", values: Some(OLYMPUS_CAMERASETTINGS_FLASHREMOTECONTROL_VALUES) },
    1028u16 => TagDef { name: "FlashControlMode", values: None },
    1029u16 => TagDef { name: "FlashIntensity", values: None },
    1030u16 => TagDef { name: "ManualFlashStrength", values: None },
    1280u16 => TagDef { name: "WhiteBalance2", values: Some(OLYMPUS_CAMERASETTINGS_WHITEBALANCE2_VALUES) },
    1281u16 => TagDef { name: "WhiteBalanceTemperature", values: None },
    1282u16 => TagDef { name: "WhiteBalanceBracket", values: None },
    1283u16 => TagDef { name: "CustomSaturation", values: None },
    1284u16 => TagDef { name: "ModifiedSaturation", values: Some(OLYMPUS_CAMERASETTINGS_MODIFIEDSATURATION_VALUES) },
    1285u16 => TagDef { name: "ContrastSetting", values: None },
    1286u16 => TagDef { name: "SharpnessSetting", values: None },
    1287u16 => TagDef { name: "ColorSpace", values: Some(OLYMPUS_CAMERASETTINGS_COLORSPACE_VALUES) },
    1289u16 => TagDef { name: "SceneMode", values: Some(OLYMPUS_CAMERASETTINGS_SCENEMODE_VALUES) },
    1290u16 => TagDef { name: "NoiseReduction", values: Some(OLYMPUS_CAMERASETTINGS_NOISEREDUCTION_VALUES) },
    1291u16 => TagDef { name: "DistortionCorrection", values: Some(OLYMPUS_CAMERASETTINGS_DISTORTIONCORRECTION_VALUES) },
    1292u16 => TagDef { name: "ShadingCompensation", values: Some(OLYMPUS_CAMERASETTINGS_SHADINGCOMPENSATION_VALUES) },
    1293u16 => TagDef { name: "CompressionFactor", values: None },
    1295u16 => TagDef { name: "Gradation", values: None },
    1312u16 => TagDef { name: "PictureMode", values: None },
    1313u16 => TagDef { name: "PictureModeSaturation", values: None },
    1314u16 => TagDef { name: "PictureModeHue", values: None },
    1315u16 => TagDef { name: "PictureModeContrast", values: None },
    1316u16 => TagDef { name: "PictureModeSharpness", values: None },
    1317u16 => TagDef { name: "PictureModeBWFilter", values: Some(OLYMPUS_CAMERASETTINGS_PICTUREMODEBWFILTER_VALUES) },
    1318u16 => TagDef { name: "PictureModeTone", values: Some(OLYMPUS_CAMERASETTINGS_PICTUREMODETONE_VALUES) },
    1319u16 => TagDef { name: "NoiseFilter", values: None },
    1321u16 => TagDef { name: "ArtFilter", values: None },
    1324u16 => TagDef { name: "MagicFilter", values: None },
    1325u16 => TagDef { name: "PictureModeEffect", values: None },
    1326u16 => TagDef { name: "ToneLevel", values: None },
    1327u16 => TagDef { name: "ArtFilterEffect", values: None },
    1330u16 => TagDef { name: "ColorCreatorEffect", values: None },
    1335u16 => TagDef { name: "MonochromeProfileSettings", values: None },
    1336u16 => TagDef { name: "FilmGrainEffect", values: Some(OLYMPUS_CAMERASETTINGS_FILMGRAINEFFECT_VALUES) },
    1337u16 => TagDef { name: "ColorProfileSettings", values: None },
    1338u16 => TagDef { name: "MonochromeVignetting", values: None },
    1339u16 => TagDef { name: "MonochromeColor", values: Some(OLYMPUS_CAMERASETTINGS_MONOCHROMECOLOR_VALUES) },
    1536u16 => TagDef { name: "DriveMode", values: None },
    1537u16 => TagDef { name: "PanoramaMode", values: None },
    1539u16 => TagDef { name: "ImageQuality2", values: Some(OLYMPUS_CAMERASETTINGS_IMAGEQUALITY2_VALUES) },
    1540u16 => TagDef { name: "ImageStabilization", values: Some(OLYMPUS_CAMERASETTINGS_IMAGESTABILIZATION_VALUES) },
    2052u16 => TagDef { name: "StackedImage", values: None },
    2081u16 => TagDef { name: "ISOAutoSettings", values: None },
    2304u16 => TagDef { name: "ManometerPressure", values: None },
    2305u16 => TagDef { name: "ManometerReading", values: None },
    2306u16 => TagDef { name: "ExtendedWBDetect", values: Some(OLYMPUS_CAMERASETTINGS_EXTENDEDWBDETECT_VALUES) },
    2307u16 => TagDef { name: "RollAngle", values: None },
    2308u16 => TagDef { name: "PitchAngle", values: None },
    2312u16 => TagDef { name: "DateTimeUTC", values: None },
    256u16 => TagDef { name: "PreviewImageValid", values: Some(OLYMPUS_CAMERASETTINGS_PREVIEWIMAGEVALID_VALUES) },
    257u16 => TagDef { name: "PreviewImageStart", values: None },
    258u16 => TagDef { name: "PreviewImageLength", values: None },
    512u16 => TagDef { name: "ExposureMode", values: Some(OLYMPUS_CAMERASETTINGS_EXPOSUREMODE_VALUES) },
    513u16 => TagDef { name: "AELock", values: Some(OLYMPUS_CAMERASETTINGS_AELOCK_VALUES) },
    514u16 => TagDef { name: "MeteringMode", values: Some(OLYMPUS_CAMERASETTINGS_METERINGMODE_VALUES) },
    515u16 => TagDef { name: "ExposureShift", values: None },
    516u16 => TagDef { name: "NDFilter", values: Some(OLYMPUS_CAMERASETTINGS_NDFILTER_VALUES) },
    768u16 => TagDef { name: "MacroMode", values: Some(OLYMPUS_CAMERASETTINGS_MACROMODE_VALUES) },
    769u16 => TagDef { name: "FocusMode", values: None },
    770u16 => TagDef { name: "FocusProcess", values: None },
    771u16 => TagDef { name: "AFSearch", values: Some(OLYMPUS_CAMERASETTINGS_AFSEARCH_VALUES) },
    772u16 => TagDef { name: "AFAreas", values: None },
    773u16 => TagDef { name: "AFPointSelected", values: None },
    774u16 => TagDef { name: "AFFineTune", values: Some(OLYMPUS_CAMERASETTINGS_AFFINETUNE_VALUES) },
    775u16 => TagDef { name: "AFFineTuneAdj", values: None },
    776u16 => TagDef { name: "FocusBracketStepSize", values: None },
    777u16 => TagDef { name: "AISubjectTrackingMode", values: Some(OLYMPUS_CAMERASETTINGS_AISUBJECTTRACKINGMODE_VALUES) },
    778u16 => TagDef { name: "AFTargetInfo", values: None },
    779u16 => TagDef { name: "SubjectDetectInfo", values: None },
};

pub static OLYMPUS_CAMERASETTINGS_FLASHMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
];

pub static OLYMPUS_CAMERASETTINGS_FLASHREMOTECONTROL_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Channel 1, Low"),
    (10, "Channel 2, Mid"),
    (11, "Channel 3, Mid"),
    (12, "Channel 4, Mid"),
    (17, "Channel 1, High"),
    (18, "Channel 2, High"),
    (19, "Channel 3, High"),
    (2, "Channel 2, Low"),
    (20, "Channel 4, High"),
    (3, "Channel 3, Low"),
    (4, "Channel 4, Low"),
    (9, "Channel 1, Mid"),
];

pub static OLYMPUS_CAMERASETTINGS_WHITEBALANCE2_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Auto (Keep Warm Color Off)"),
    (16, "7500K (Fine Weather with Shade)"),
    (17, "6000K (Cloudy)"),
    (18, "5300K (Fine Weather)"),
    (20, "3000K (Tungsten light)"),
    (21, "3600K (Tungsten light-like)"),
    (22, "Auto Setup"),
    (23, "5500K (Flash)"),
    (256, "One Touch WB 1"),
    (257, "One Touch WB 2"),
    (258, "One Touch WB 3"),
    (259, "One Touch WB 4"),
    (33, "6600K (Daylight fluorescent)"),
    (34, "4500K (Neutral white fluorescent)"),
    (35, "4000K (Cool white fluorescent)"),
    (36, "White Fluorescent"),
    (48, "3600K (Tungsten light-like)"),
    (512, "Custom WB 1"),
    (513, "Custom WB 2"),
    (514, "Custom WB 3"),
    (515, "Custom WB 4"),
    (67, "Underwater"),
];

pub static OLYMPUS_CAMERASETTINGS_MODIFIEDSATURATION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "CM1 (Red Enhance)"),
    (2, "CM2 (Green Enhance)"),
    (3, "CM3 (Blue Enhance)"),
    (4, "CM4 (Skin Tones)"),
];

pub static OLYMPUS_CAMERASETTINGS_COLORSPACE_VALUES: &[(i64, &str)] = &[
    (0, "sRGB"),
    (1, "Adobe RGB"),
    (2, "Pro Photo RGB"),
];

pub static OLYMPUS_CAMERASETTINGS_SCENEMODE_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (10, "Landscape"),
    (11, "Night Scene"),
    (12, "Self Portrait"),
    (13, "Panorama"),
    (14, "2 in 1"),
    (142, "Hand-held Starlight"),
    (15, "Movie"),
    (154, "HDR"),
    (16, "Landscape+Portrait"),
    (17, "Night+Portrait"),
    (18, "Indoor"),
    (19, "Fireworks"),
    (197, "Panning"),
    (20, "Sunset"),
    (203, "Light Trails"),
    (204, "Backlight HDR"),
    (205, "Silent"),
    (206, "Multi Focus Shot"),
    (21, "Beauty Skin"),
    (22, "Macro"),
    (23, "Super Macro"),
    (24, "Food"),
    (25, "Documents"),
    (26, "Museum"),
    (27, "Shoot & Select"),
    (28, "Beach & Snow"),
    (29, "Self Protrait+Timer"),
    (30, "Candle"),
    (31, "Available Light"),
    (32, "Behind Glass"),
    (33, "My Mode"),
    (34, "Pet"),
    (35, "Underwater Wide1"),
    (36, "Underwater Macro"),
    (37, "Shoot & Select1"),
    (38, "Shoot & Select2"),
    (39, "High Key"),
    (40, "Digital Image Stabilization"),
    (41, "Auction"),
    (42, "Beach"),
    (43, "Snow"),
    (44, "Underwater Wide2"),
    (45, "Low Key"),
    (46, "Children"),
    (47, "Vivid"),
    (48, "Nature Macro"),
    (49, "Underwater Snapshot"),
    (50, "Shooting Guide"),
    (54, "Face Portrait"),
    (57, "Bulb"),
    (59, "Smile Shot"),
    (6, "Auto"),
    (60, "Quick Shutter"),
    (63, "Slow Shutter"),
    (64, "Bird Watching"),
    (65, "Multiple Exposure"),
    (66, "e-Portrait"),
    (67, "Soft Background Shot"),
    (7, "Sport"),
    (8, "Portrait"),
    (9, "Landscape+Portrait"),
];

pub static OLYMPUS_CAMERASETTINGS_NOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "(none)"),
];

pub static OLYMPUS_CAMERASETTINGS_DISTORTIONCORRECTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static OLYMPUS_CAMERASETTINGS_SHADINGCOMPENSATION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static OLYMPUS_CAMERASETTINGS_PICTUREMODEBWFILTER_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "Neutral"),
    (2, "Yellow"),
    (3, "Orange"),
    (4, "Red"),
    (5, "Green"),
];

pub static OLYMPUS_CAMERASETTINGS_PICTUREMODETONE_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "Neutral"),
    (2, "Sepia"),
    (3, "Blue"),
    (4, "Purple"),
    (5, "Green"),
];

pub static OLYMPUS_CAMERASETTINGS_FILMGRAINEFFECT_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Low"),
    (2, "Medium"),
    (3, "High"),
];

pub static OLYMPUS_CAMERASETTINGS_MONOCHROMECOLOR_VALUES: &[(i64, &str)] = &[
    (0, "(none)"),
    (1, "Normal"),
    (2, "Sepia"),
    (3, "Blue"),
    (4, "Purple"),
    (5, "Green"),
];

pub static OLYMPUS_CAMERASETTINGS_IMAGEQUALITY2_VALUES: &[(i64, &str)] = &[
    (1, "SQ"),
    (2, "HQ"),
    (3, "SHQ"),
    (4, "RAW"),
    (5, "SQ (5)"),
];

pub static OLYMPUS_CAMERASETTINGS_IMAGESTABILIZATION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On, S-IS1 (All Direction Shake IS)"),
    (2, "On, S-IS2 (Vertical Shake IS)"),
    (3, "On, S-IS3 (Horizontal Shake IS)"),
    (4, "On, S-IS Auto"),
];

pub static OLYMPUS_CAMERASETTINGS_EXTENDEDWBDETECT_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static OLYMPUS_CAMERASETTINGS_PREVIEWIMAGEVALID_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];

pub static OLYMPUS_CAMERASETTINGS_EXPOSUREMODE_VALUES: &[(i64, &str)] = &[
    (1, "Manual"),
    (2, "Program"),
    (3, "Aperture-priority AE"),
    (4, "Shutter speed priority AE"),
    (5, "Program-shift"),
];

pub static OLYMPUS_CAMERASETTINGS_AELOCK_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static OLYMPUS_CAMERASETTINGS_METERINGMODE_VALUES: &[(i64, &str)] = &[
    (1027, "Spot+Shadow control"),
    (2, "Center-weighted average"),
    (261, "Pattern+AF"),
    (3, "Spot"),
    (5, "ESP"),
    (515, "Spot+Highlight control"),
];

pub static OLYMPUS_CAMERASETTINGS_NDFILTER_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static OLYMPUS_CAMERASETTINGS_MACROMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
    (2, "Super Macro"),
];

pub static OLYMPUS_CAMERASETTINGS_AFSEARCH_VALUES: &[(i64, &str)] = &[
    (0, "Not Ready"),
    (1, "Ready"),
];

pub static OLYMPUS_CAMERASETTINGS_AFFINETUNE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static OLYMPUS_CAMERASETTINGS_AISUBJECTTRACKINGMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1024, "Birds; Object Not Found"),
    (1025, "Birds; Object Found"),
    (1280, "Dogs & Cats; Object Not Found"),
    (1281, "Dogs & Cats; Object Found"),
    (1536, "Human; Object Not Found"),
    (1537, "Human; Object Found"),
    (256, "Motorsports; Object Not Found"),
    (257, "Motorsports; Racing Car Found"),
    (258, "Motorsports; Car Found"),
    (259, "Motorsports; Motorcyle Found"),
    (512, "Airplanes; Object Not Found"),
    (513, "Airplanes; Passenger/Transport Plane Found"),
    (514, "Airplanes; Small Plane/Fighter Jet Found"),
    (515, "Airplanes; Helicopter Found"),
    (768, "Trains; Object Not Found"),
    (769, "Trains; Object Found"),
];

/// Olympus::Equipment tags
pub static OLYMPUS_EQUIPMENT: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "EquipmentVersion", values: None },
    1027u16 => TagDef { name: "ConversionLens", values: None },
    256u16 => TagDef { name: "CameraType2", values: None },
    257u16 => TagDef { name: "SerialNumber", values: None },
    258u16 => TagDef { name: "InternalSerialNumber", values: None },
    259u16 => TagDef { name: "FocalPlaneDiagonal", values: None },
    260u16 => TagDef { name: "BodyFirmwareVersion", values: None },
    4096u16 => TagDef { name: "FlashType", values: Some(OLYMPUS_EQUIPMENT_FLASHTYPE_VALUES) },
    4097u16 => TagDef { name: "FlashModel", values: Some(OLYMPUS_EQUIPMENT_FLASHMODEL_VALUES) },
    4098u16 => TagDef { name: "FlashFirmwareVersion", values: None },
    4099u16 => TagDef { name: "FlashSerialNumber", values: None },
    513u16 => TagDef { name: "LensType", values: None },
    514u16 => TagDef { name: "LensSerialNumber", values: None },
    515u16 => TagDef { name: "LensModel", values: None },
    516u16 => TagDef { name: "LensFirmwareVersion", values: None },
    517u16 => TagDef { name: "MaxApertureAtMinFocal", values: None },
    518u16 => TagDef { name: "MaxApertureAtMaxFocal", values: None },
    519u16 => TagDef { name: "MinFocalLength", values: None },
    520u16 => TagDef { name: "MaxFocalLength", values: None },
    522u16 => TagDef { name: "MaxAperture", values: None },
    523u16 => TagDef { name: "LensProperties", values: None },
    769u16 => TagDef { name: "Extender", values: None },
    770u16 => TagDef { name: "ExtenderSerialNumber", values: None },
    771u16 => TagDef { name: "ExtenderModel", values: None },
    772u16 => TagDef { name: "ExtenderFirmwareVersion", values: None },
};

pub static OLYMPUS_EQUIPMENT_FLASHTYPE_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (2, "Simple E-System"),
    (3, "E-System"),
    (4, "E-System (body powered)"),
];

pub static OLYMPUS_EQUIPMENT_FLASHMODEL_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "FL-20"),
    (11, "FL-600R"),
    (13, "FL-LM3"),
    (15, "FL-900R"),
    (2, "FL-50"),
    (3, "RF-11"),
    (4, "TF-22"),
    (5, "FL-36"),
    (6, "FL-50R"),
    (7, "FL-36R"),
    (9, "FL-14"),
];

/// Olympus::FETags tags
pub static OLYMPUS_FETAGS: phf::Map<u16, TagDef> = phf::phf_map! {
    256u16 => TagDef { name: "BodyFirmwareVersion", values: None },
};

/// Olympus::FocusInfo tags
pub static OLYMPUS_FOCUSINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "FocusInfoVersion", values: None },
    4609u16 => TagDef { name: "ExternalFlash", values: None },
    4611u16 => TagDef { name: "ExternalFlashGuideNumber", values: None },
    4612u16 => TagDef { name: "ExternalFlashBounce", values: Some(OLYMPUS_FOCUSINFO_EXTERNALFLASHBOUNCE_VALUES) },
    4613u16 => TagDef { name: "ExternalFlashZoom", values: None },
    4616u16 => TagDef { name: "InternalFlash", values: Some(OLYMPUS_FOCUSINFO_INTERNALFLASH_VALUES) },
    4617u16 => TagDef { name: "ManualFlash", values: None },
    4618u16 => TagDef { name: "MacroLED", values: Some(OLYMPUS_FOCUSINFO_MACROLED_VALUES) },
    521u16 => TagDef { name: "AutoFocus", values: Some(OLYMPUS_FOCUSINFO_AUTOFOCUS_VALUES) },
    528u16 => TagDef { name: "SceneDetect", values: None },
    529u16 => TagDef { name: "SceneArea", values: None },
    530u16 => TagDef { name: "SceneDetectData", values: None },
    5376u16 => TagDef { name: "SensorTemperature", values: None },
    5632u16 => TagDef { name: "ImageStabilization", values: None },
    768u16 => TagDef { name: "ZoomStepCount", values: None },
    769u16 => TagDef { name: "FocusStepCount", values: None },
    771u16 => TagDef { name: "FocusStepInfinity", values: None },
    772u16 => TagDef { name: "FocusStepNear", values: None },
    773u16 => TagDef { name: "FocusDistance", values: None },
    776u16 => TagDef { name: "AFPoint", values: None },
    795u16 => TagDef { name: "AFPointDetails", values: None },
    808u16 => TagDef { name: "AFInfo", values: None },
};

pub static OLYMPUS_FOCUSINFO_EXTERNALFLASHBOUNCE_VALUES: &[(i64, &str)] = &[
    (0, "Bounce or Off"),
    (1, "Direct"),
];

pub static OLYMPUS_FOCUSINFO_INTERNALFLASH_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static OLYMPUS_FOCUSINFO_MACROLED_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static OLYMPUS_FOCUSINFO_AUTOFOCUS_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

/// Olympus::ImageProcessing tags
pub static OLYMPUS_IMAGEPROCESSING: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "ImageProcessingVersion", values: None },
    1536u16 => TagDef { name: "BlackLevel2", values: None },
    1552u16 => TagDef { name: "GainBase", values: None },
    1553u16 => TagDef { name: "ValidBits", values: None },
    1554u16 => TagDef { name: "CropLeft", values: None },
    1555u16 => TagDef { name: "CropTop", values: None },
    1556u16 => TagDef { name: "CropWidth", values: None },
    1557u16 => TagDef { name: "CropHeight", values: None },
    1589u16 => TagDef { name: "UnknownBlock1", values: None },
    1590u16 => TagDef { name: "UnknownBlock2", values: None },
    2053u16 => TagDef { name: "SensorCalibration", values: None },
    256u16 => TagDef { name: "WB_RBLevels", values: None },
    258u16 => TagDef { name: "WB_RBLevels3000K", values: None },
    259u16 => TagDef { name: "WB_RBLevels3300K", values: None },
    260u16 => TagDef { name: "WB_RBLevels3600K", values: None },
    261u16 => TagDef { name: "WB_RBLevels3900K", values: None },
    262u16 => TagDef { name: "WB_RBLevels4000K", values: None },
    263u16 => TagDef { name: "WB_RBLevels4300K", values: None },
    264u16 => TagDef { name: "WB_RBLevels4500K", values: None },
    265u16 => TagDef { name: "WB_RBLevels4800K", values: None },
    266u16 => TagDef { name: "WB_RBLevels5300K", values: None },
    267u16 => TagDef { name: "WB_RBLevels6000K", values: None },
    268u16 => TagDef { name: "WB_RBLevels6600K", values: None },
    269u16 => TagDef { name: "WB_RBLevels7500K", values: None },
    270u16 => TagDef { name: "WB_RBLevelsCWB1", values: None },
    271u16 => TagDef { name: "WB_RBLevelsCWB2", values: None },
    272u16 => TagDef { name: "WB_RBLevelsCWB3", values: None },
    273u16 => TagDef { name: "WB_RBLevelsCWB4", values: None },
    275u16 => TagDef { name: "WB_GLevel3000K", values: None },
    276u16 => TagDef { name: "WB_GLevel3300K", values: None },
    277u16 => TagDef { name: "WB_GLevel3600K", values: None },
    278u16 => TagDef { name: "WB_GLevel3900K", values: None },
    279u16 => TagDef { name: "WB_GLevel4000K", values: None },
    280u16 => TagDef { name: "WB_GLevel4300K", values: None },
    281u16 => TagDef { name: "WB_GLevel4500K", values: None },
    282u16 => TagDef { name: "WB_GLevel4800K", values: None },
    283u16 => TagDef { name: "WB_GLevel5300K", values: None },
    284u16 => TagDef { name: "WB_GLevel6000K", values: None },
    285u16 => TagDef { name: "WB_GLevel6600K", values: None },
    286u16 => TagDef { name: "WB_GLevel7500K", values: None },
    287u16 => TagDef { name: "WB_GLevel", values: None },
    4112u16 => TagDef { name: "NoiseReduction2", values: Some(OLYMPUS_IMAGEPROCESSING_NOISEREDUCTION2_VALUES) },
    4113u16 => TagDef { name: "DistortionCorrection2", values: Some(OLYMPUS_IMAGEPROCESSING_DISTORTIONCORRECTION2_VALUES) },
    4114u16 => TagDef { name: "ShadingCompensation2", values: Some(OLYMPUS_IMAGEPROCESSING_SHADINGCOMPENSATION2_VALUES) },
    4124u16 => TagDef { name: "MultipleExposureMode", values: None },
    4355u16 => TagDef { name: "UnknownBlock3", values: None },
    4356u16 => TagDef { name: "UnknownBlock4", values: None },
    4370u16 => TagDef { name: "AspectRatio", values: None },
    4371u16 => TagDef { name: "AspectFrame", values: None },
    4608u16 => TagDef { name: "FacesDetected", values: None },
    4609u16 => TagDef { name: "FaceDetectArea", values: None },
    4610u16 => TagDef { name: "MaxFaces", values: None },
    4611u16 => TagDef { name: "FaceDetectFrameSize", values: None },
    4615u16 => TagDef { name: "FaceDetectFrameCrop", values: None },
    4870u16 => TagDef { name: "CameraTemperature", values: None },
    512u16 => TagDef { name: "ColorMatrix", values: None },
    6400u16 => TagDef { name: "KeystoneCompensation", values: None },
    6401u16 => TagDef { name: "KeystoneDirection", values: Some(OLYMPUS_IMAGEPROCESSING_KEYSTONEDIRECTION_VALUES) },
    6406u16 => TagDef { name: "KeystoneValue", values: None },
    768u16 => TagDef { name: "Enhancer", values: None },
    769u16 => TagDef { name: "EnhancerValues", values: None },
    784u16 => TagDef { name: "CoringFilter", values: None },
    785u16 => TagDef { name: "CoringValues", values: None },
    8464u16 => TagDef { name: "GNDFilterType", values: Some(OLYMPUS_IMAGEPROCESSING_GNDFILTERTYPE_VALUES) },
};

pub static OLYMPUS_IMAGEPROCESSING_NOISEREDUCTION2_VALUES: &[(i64, &str)] = &[
    (0, "(none)"),
];

pub static OLYMPUS_IMAGEPROCESSING_DISTORTIONCORRECTION2_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static OLYMPUS_IMAGEPROCESSING_SHADINGCOMPENSATION2_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static OLYMPUS_IMAGEPROCESSING_KEYSTONEDIRECTION_VALUES: &[(i64, &str)] = &[
    (0, "Vertical"),
    (1, "Horizontal"),
];

pub static OLYMPUS_IMAGEPROCESSING_GNDFILTERTYPE_VALUES: &[(i64, &str)] = &[
    (0, "High"),
    (1, "Medium"),
    (2, "Soft"),
];

/// Olympus::Main tags
pub static OLYMPUS_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "MakerNoteVersion", values: None },
    1u16 => TagDef { name: "MinoltaCameraSettingsOld", values: None },
    1024u16 => TagDef { name: "SensorArea", values: None },
    10240u16 => TagDef { name: "Olympus2800", values: None },
    1025u16 => TagDef { name: "BlackLevel", values: None },
    1027u16 => TagDef { name: "SceneMode", values: Some(OLYMPUS_MAIN_SCENEMODE_VALUES) },
    1028u16 => TagDef { name: "SerialNumber", values: None },
    1029u16 => TagDef { name: "Firmware", values: None },
    10496u16 => TagDef { name: "Olympus2900", values: None },
    12288u16 => TagDef { name: "RawInfo", values: None },
    129u16 => TagDef { name: "PreviewImageData", values: None },
    136u16 => TagDef { name: "PreviewImageStart", values: None },
    137u16 => TagDef { name: "PreviewImageLength", values: None },
    16384u16 => TagDef { name: "MainInfo", values: None },
    20480u16 => TagDef { name: "UnknownInfo", values: None },
    256u16 => TagDef { name: "ThumbnailImage", values: None },
    260u16 => TagDef { name: "BodyFirmwareVersion", values: None },
    3u16 => TagDef { name: "MinoltaCameraSettings", values: None },
    3584u16 => TagDef { name: "PrintIM", values: None },
    3840u16 => TagDef { name: "DataDump", values: None },
    3841u16 => TagDef { name: "DataDump2", values: None },
    3844u16 => TagDef { name: "ZoomedPreviewStart", values: None },
    3845u16 => TagDef { name: "ZoomedPreviewLength", values: None },
    3846u16 => TagDef { name: "ZoomedPreviewSize", values: None },
    4096u16 => TagDef { name: "ShutterSpeedValue", values: None },
    4097u16 => TagDef { name: "ISOValue", values: None },
    4098u16 => TagDef { name: "ApertureValue", values: None },
    4099u16 => TagDef { name: "BrightnessValue", values: None },
    4100u16 => TagDef { name: "FlashMode", values: Some(OLYMPUS_MAIN_FLASHMODE_VALUES) },
    4101u16 => TagDef { name: "FlashDevice", values: Some(OLYMPUS_MAIN_FLASHDEVICE_VALUES) },
    4102u16 => TagDef { name: "ExposureCompensation", values: None },
    4103u16 => TagDef { name: "SensorTemperature", values: None },
    4104u16 => TagDef { name: "LensTemperature", values: None },
    4105u16 => TagDef { name: "LightCondition", values: None },
    4106u16 => TagDef { name: "FocusRange", values: Some(OLYMPUS_MAIN_FOCUSRANGE_VALUES) },
    4107u16 => TagDef { name: "FocusMode", values: Some(OLYMPUS_MAIN_FOCUSMODE_VALUES) },
    4108u16 => TagDef { name: "ManualFocusDistance", values: None },
    4109u16 => TagDef { name: "ZoomStepCount", values: None },
    4110u16 => TagDef { name: "FocusStepCount", values: None },
    4111u16 => TagDef { name: "Sharpness", values: Some(OLYMPUS_MAIN_SHARPNESS_VALUES) },
    4112u16 => TagDef { name: "FlashChargeLevel", values: None },
    4113u16 => TagDef { name: "ColorMatrix", values: None },
    4114u16 => TagDef { name: "BlackLevel", values: None },
    4115u16 => TagDef { name: "ColorTemperatureBG", values: None },
    4116u16 => TagDef { name: "ColorTemperatureRG", values: None },
    4117u16 => TagDef { name: "WBMode", values: Some(OLYMPUS_MAIN_WBMODE_VALUES) },
    4119u16 => TagDef { name: "RedBalance", values: None },
    4120u16 => TagDef { name: "BlueBalance", values: None },
    4121u16 => TagDef { name: "ColorMatrixNumber", values: None },
    4122u16 => TagDef { name: "SerialNumber", values: None },
    4123u16 => TagDef { name: "ExternalFlashAE1_0", values: None },
    4124u16 => TagDef { name: "ExternalFlashAE2_0", values: None },
    4125u16 => TagDef { name: "InternalFlashAE1_0", values: None },
    4126u16 => TagDef { name: "InternalFlashAE2_0", values: None },
    4127u16 => TagDef { name: "ExternalFlashAE1", values: None },
    4128u16 => TagDef { name: "ExternalFlashAE2", values: None },
    4129u16 => TagDef { name: "InternalFlashAE1", values: None },
    4130u16 => TagDef { name: "InternalFlashAE2", values: None },
    4131u16 => TagDef { name: "FlashExposureComp", values: None },
    4132u16 => TagDef { name: "InternalFlashTable", values: None },
    4133u16 => TagDef { name: "ExternalFlashGValue", values: None },
    4134u16 => TagDef { name: "ExternalFlashBounce", values: Some(OLYMPUS_MAIN_EXTERNALFLASHBOUNCE_VALUES) },
    4135u16 => TagDef { name: "ExternalFlashZoom", values: None },
    4136u16 => TagDef { name: "ExternalFlashMode", values: None },
    4137u16 => TagDef { name: "Contrast", values: Some(OLYMPUS_MAIN_CONTRAST_VALUES) },
    4138u16 => TagDef { name: "SharpnessFactor", values: None },
    4139u16 => TagDef { name: "ColorControl", values: None },
    4140u16 => TagDef { name: "ValidBits", values: None },
    4141u16 => TagDef { name: "CoringFilter", values: None },
    4142u16 => TagDef { name: "OlympusImageWidth", values: None },
    4143u16 => TagDef { name: "OlympusImageHeight", values: None },
    4144u16 => TagDef { name: "SceneDetect", values: None },
    4145u16 => TagDef { name: "SceneArea", values: None },
    4147u16 => TagDef { name: "SceneDetectData", values: None },
    4148u16 => TagDef { name: "CompressionRatio", values: None },
    4149u16 => TagDef { name: "PreviewImageValid", values: Some(OLYMPUS_MAIN_PREVIEWIMAGEVALID_VALUES) },
    4150u16 => TagDef { name: "PreviewImageStart", values: None },
    4151u16 => TagDef { name: "PreviewImageLength", values: None },
    4152u16 => TagDef { name: "AFResult", values: None },
    4153u16 => TagDef { name: "CCDScanMode", values: Some(OLYMPUS_MAIN_CCDSCANMODE_VALUES) },
    4154u16 => TagDef { name: "NoiseReduction", values: Some(OLYMPUS_MAIN_NOISEREDUCTION_VALUES) },
    4155u16 => TagDef { name: "FocusStepInfinity", values: None },
    4156u16 => TagDef { name: "FocusStepNear", values: None },
    4157u16 => TagDef { name: "LightValueCenter", values: None },
    4158u16 => TagDef { name: "LightValuePeriphery", values: None },
    4159u16 => TagDef { name: "FieldCount", values: None },
    512u16 => TagDef { name: "SpecialMode", values: None },
    513u16 => TagDef { name: "Quality", values: None },
    514u16 => TagDef { name: "Macro", values: Some(OLYMPUS_MAIN_MACRO_VALUES) },
    515u16 => TagDef { name: "BWMode", values: Some(OLYMPUS_MAIN_BWMODE_VALUES) },
    516u16 => TagDef { name: "DigitalZoom", values: None },
    517u16 => TagDef { name: "FocalPlaneDiagonal", values: None },
    518u16 => TagDef { name: "LensDistortionParams", values: None },
    519u16 => TagDef { name: "CameraType", values: None },
    520u16 => TagDef { name: "TextInfo", values: None },
    521u16 => TagDef { name: "CameraID", values: None },
    523u16 => TagDef { name: "EpsonImageWidth", values: None },
    524u16 => TagDef { name: "EpsonImageHeight", values: None },
    525u16 => TagDef { name: "EpsonSoftware", values: None },
    64u16 => TagDef { name: "CompressedImageSize", values: None },
    640u16 => TagDef { name: "PreviewImage", values: None },
    768u16 => TagDef { name: "PreCaptureFrames", values: None },
    769u16 => TagDef { name: "WhiteBoard", values: None },
    770u16 => TagDef { name: "OneTouchWB", values: Some(OLYMPUS_MAIN_ONETOUCHWB_VALUES) },
    771u16 => TagDef { name: "WhiteBalanceBracket", values: None },
    772u16 => TagDef { name: "WhiteBalanceBias", values: None },
    8208u16 => TagDef { name: "Equipment", values: None },
    8224u16 => TagDef { name: "CameraSettings", values: None },
    8240u16 => TagDef { name: "RawDevelopment", values: None },
    8241u16 => TagDef { name: "RawDev2", values: None },
    8256u16 => TagDef { name: "ImageProcessing", values: None },
    8272u16 => TagDef { name: "FocusInfo", values: None },
    8448u16 => TagDef { name: "Olympus2100", values: None },
    8704u16 => TagDef { name: "Olympus2200", values: None },
    8960u16 => TagDef { name: "Olympus2300", values: None },
    9216u16 => TagDef { name: "Olympus2400", values: None },
    9472u16 => TagDef { name: "Olympus2500", values: None },
    9728u16 => TagDef { name: "Olympus2600", values: None },
    9984u16 => TagDef { name: "Olympus2700", values: None },
};

pub static OLYMPUS_MAIN_SCENEMODE_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Standard"),
    (10, "Self Portrait"),
    (100, "Panorama"),
    (101, "Magic Filter"),
    (103, "HDR"),
    (11, "Indoor"),
    (12, "Beach & Snow"),
    (13, "Beach"),
    (14, "Snow"),
    (15, "Self Portrait+Self Timer"),
    (16, "Sunset"),
    (17, "Cuisine"),
    (18, "Documents"),
    (19, "Candle"),
    (2, "Auto"),
    (20, "Fireworks"),
    (21, "Available Light"),
    (22, "Vivid"),
    (23, "Underwater Wide1"),
    (24, "Underwater Macro"),
    (25, "Museum"),
    (26, "Behind Glass"),
    (27, "Auction"),
    (28, "Shoot & Select1"),
    (29, "Shoot & Select2"),
    (3, "Intelligent Auto"),
    (30, "Underwater Wide2"),
    (31, "Digital Image Stabilization"),
    (32, "Face Portrait"),
    (33, "Pet"),
    (34, "Smile Shot"),
    (35, "Quick Shutter"),
    (4, "Portrait"),
    (43, "Hand-held Starlight"),
    (5, "Landscape+Portrait"),
    (6, "Landscape"),
    (7, "Night Scene"),
    (8, "Night+Portrait"),
    (9, "Sport"),
];

pub static OLYMPUS_MAIN_FLASHMODE_VALUES: &[(i64, &str)] = &[
    (2, "On"),
    (3, "Off"),
];

pub static OLYMPUS_MAIN_FLASHDEVICE_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "Internal"),
    (4, "External"),
    (5, "Internal + External"),
];

pub static OLYMPUS_MAIN_FOCUSRANGE_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Macro"),
];

pub static OLYMPUS_MAIN_FOCUSMODE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Manual"),
];

pub static OLYMPUS_MAIN_SHARPNESS_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Hard"),
    (2, "Soft"),
];

pub static OLYMPUS_MAIN_WBMODE_VALUES: &[(i64, &str)] = &[
    (1, "Auto"),
];

pub static OLYMPUS_MAIN_EXTERNALFLASHBOUNCE_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];

pub static OLYMPUS_MAIN_CONTRAST_VALUES: &[(i64, &str)] = &[
    (0, "High"),
    (1, "Normal"),
    (2, "Low"),
];

pub static OLYMPUS_MAIN_PREVIEWIMAGEVALID_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];

pub static OLYMPUS_MAIN_CCDSCANMODE_VALUES: &[(i64, &str)] = &[
    (0, "Interlaced"),
    (1, "Progressive"),
];

pub static OLYMPUS_MAIN_NOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static OLYMPUS_MAIN_MACRO_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
    (2, "Super Macro"),
];

pub static OLYMPUS_MAIN_BWMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
    (6, "(none)"),
];

pub static OLYMPUS_MAIN_ONETOUCHWB_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
    (2, "On (Preset)"),
];

/// Olympus::RawDevelopment tags
pub static OLYMPUS_RAWDEVELOPMENT: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "RawDevVersion", values: None },
    256u16 => TagDef { name: "RawDevExposureBiasValue", values: None },
    257u16 => TagDef { name: "RawDevWhiteBalanceValue", values: None },
    258u16 => TagDef { name: "RawDevWBFineAdjustment", values: None },
    259u16 => TagDef { name: "RawDevGrayPoint", values: None },
    260u16 => TagDef { name: "RawDevSaturationEmphasis", values: None },
    261u16 => TagDef { name: "RawDevMemoryColorEmphasis", values: None },
    262u16 => TagDef { name: "RawDevContrastValue", values: None },
    263u16 => TagDef { name: "RawDevSharpnessValue", values: None },
    264u16 => TagDef { name: "RawDevColorSpace", values: Some(OLYMPUS_RAWDEVELOPMENT_RAWDEVCOLORSPACE_VALUES) },
    265u16 => TagDef { name: "RawDevEngine", values: Some(OLYMPUS_RAWDEVELOPMENT_RAWDEVENGINE_VALUES) },
    266u16 => TagDef { name: "RawDevNoiseReduction", values: Some(OLYMPUS_RAWDEVELOPMENT_RAWDEVNOISEREDUCTION_VALUES) },
    267u16 => TagDef { name: "RawDevEditStatus", values: Some(OLYMPUS_RAWDEVELOPMENT_RAWDEVEDITSTATUS_VALUES) },
    268u16 => TagDef { name: "RawDevSettings", values: Some(OLYMPUS_RAWDEVELOPMENT_RAWDEVSETTINGS_VALUES) },
};

pub static OLYMPUS_RAWDEVELOPMENT_RAWDEVCOLORSPACE_VALUES: &[(i64, &str)] = &[
    (0, "sRGB"),
    (1, "Adobe RGB"),
    (2, "Pro Photo RGB"),
];

pub static OLYMPUS_RAWDEVELOPMENT_RAWDEVENGINE_VALUES: &[(i64, &str)] = &[
    (0, "High Speed"),
    (1, "High Function"),
    (2, "Advanced High Speed"),
    (3, "Advanced High Function"),
];

pub static OLYMPUS_RAWDEVELOPMENT_RAWDEVNOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "(none)"),
];

pub static OLYMPUS_RAWDEVELOPMENT_RAWDEVEDITSTATUS_VALUES: &[(i64, &str)] = &[
    (0, "Original"),
    (1, "Edited (Landscape)"),
    (6, "Edited (Portrait)"),
    (8, "Edited (Portrait)"),
];

pub static OLYMPUS_RAWDEVELOPMENT_RAWDEVSETTINGS_VALUES: &[(i64, &str)] = &[
    (0, "(none)"),
];

/// Olympus::RawDevelopment2 tags
pub static OLYMPUS_RAWDEVELOPMENT2: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "RawDevVersion", values: None },
    256u16 => TagDef { name: "RawDevExposureBiasValue", values: None },
    257u16 => TagDef { name: "RawDevWhiteBalance", values: Some(OLYMPUS_RAWDEVELOPMENT2_RAWDEVWHITEBALANCE_VALUES) },
    258u16 => TagDef { name: "RawDevWhiteBalanceValue", values: None },
    259u16 => TagDef { name: "RawDevWBFineAdjustment", values: None },
    260u16 => TagDef { name: "RawDevGrayPoint", values: None },
    261u16 => TagDef { name: "RawDevContrastValue", values: None },
    262u16 => TagDef { name: "RawDevSharpnessValue", values: None },
    263u16 => TagDef { name: "RawDevSaturationEmphasis", values: None },
    264u16 => TagDef { name: "RawDevMemoryColorEmphasis", values: None },
    265u16 => TagDef { name: "RawDevColorSpace", values: Some(OLYMPUS_RAWDEVELOPMENT2_RAWDEVCOLORSPACE_VALUES) },
    266u16 => TagDef { name: "RawDevNoiseReduction", values: Some(OLYMPUS_RAWDEVELOPMENT2_RAWDEVNOISEREDUCTION_VALUES) },
    267u16 => TagDef { name: "RawDevEngine", values: Some(OLYMPUS_RAWDEVELOPMENT2_RAWDEVENGINE_VALUES) },
    268u16 => TagDef { name: "RawDevPictureMode", values: Some(OLYMPUS_RAWDEVELOPMENT2_RAWDEVPICTUREMODE_VALUES) },
    269u16 => TagDef { name: "RawDevPMSaturation", values: None },
    270u16 => TagDef { name: "RawDevPMContrast", values: None },
    271u16 => TagDef { name: "RawDevPMSharpness", values: None },
    272u16 => TagDef { name: "RawDevPM_BWFilter", values: Some(OLYMPUS_RAWDEVELOPMENT2_RAWDEVPM_BWFILTER_VALUES) },
    273u16 => TagDef { name: "RawDevPMPictureTone", values: Some(OLYMPUS_RAWDEVELOPMENT2_RAWDEVPMPICTURETONE_VALUES) },
    274u16 => TagDef { name: "RawDevGradation", values: None },
    275u16 => TagDef { name: "RawDevSaturation3", values: None },
    281u16 => TagDef { name: "RawDevAutoGradation", values: Some(OLYMPUS_RAWDEVELOPMENT2_RAWDEVAUTOGRADATION_VALUES) },
    288u16 => TagDef { name: "RawDevPMNoiseFilter", values: None },
    289u16 => TagDef { name: "RawDevArtFilter", values: None },
    32768u16 => TagDef { name: "RawDevSubIFD", values: None },
};

pub static OLYMPUS_RAWDEVELOPMENT2_RAWDEVWHITEBALANCE_VALUES: &[(i64, &str)] = &[
    (1, "Color Temperature"),
    (2, "Gray Point"),
];

pub static OLYMPUS_RAWDEVELOPMENT2_RAWDEVCOLORSPACE_VALUES: &[(i64, &str)] = &[
    (0, "sRGB"),
    (1, "Adobe RGB"),
    (2, "Pro Photo RGB"),
];

pub static OLYMPUS_RAWDEVELOPMENT2_RAWDEVNOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "(none)"),
];

pub static OLYMPUS_RAWDEVELOPMENT2_RAWDEVENGINE_VALUES: &[(i64, &str)] = &[
    (0, "High Speed"),
    (1, "High Function"),
];

pub static OLYMPUS_RAWDEVELOPMENT2_RAWDEVPICTUREMODE_VALUES: &[(i64, &str)] = &[
    (1, "Vivid"),
    (2, "Natural"),
    (256, "Monotone"),
    (3, "Muted"),
    (512, "Sepia"),
];

pub static OLYMPUS_RAWDEVELOPMENT2_RAWDEVPM_BWFILTER_VALUES: &[(i64, &str)] = &[
    (1, "Neutral"),
    (2, "Yellow"),
    (3, "Orange"),
    (4, "Red"),
    (5, "Green"),
];

pub static OLYMPUS_RAWDEVELOPMENT2_RAWDEVPMPICTURETONE_VALUES: &[(i64, &str)] = &[
    (1, "Neutral"),
    (2, "Sepia"),
    (3, "Blue"),
    (4, "Purple"),
    (5, "Green"),
];

pub static OLYMPUS_RAWDEVELOPMENT2_RAWDEVAUTOGRADATION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

/// Olympus::RawInfo tags
pub static OLYMPUS_RAWINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "RawInfoVersion", values: None },
    1536u16 => TagDef { name: "BlackLevel2", values: None },
    1537u16 => TagDef { name: "YCbCrCoefficients", values: None },
    1553u16 => TagDef { name: "ValidPixelDepth", values: None },
    1554u16 => TagDef { name: "CropLeft", values: None },
    1555u16 => TagDef { name: "CropTop", values: None },
    1556u16 => TagDef { name: "CropWidth", values: None },
    1557u16 => TagDef { name: "CropHeight", values: None },
    256u16 => TagDef { name: "WB_RBLevelsUsed", values: None },
    272u16 => TagDef { name: "WB_RBLevelsAuto", values: None },
    288u16 => TagDef { name: "WB_RBLevelsShade", values: None },
    289u16 => TagDef { name: "WB_RBLevelsCloudy", values: None },
    290u16 => TagDef { name: "WB_RBLevelsFineWeather", values: None },
    291u16 => TagDef { name: "WB_RBLevelsTungsten", values: None },
    292u16 => TagDef { name: "WB_RBLevelsEveningSunlight", values: None },
    304u16 => TagDef { name: "WB_RBLevelsDaylightFluor", values: None },
    305u16 => TagDef { name: "WB_RBLevelsDayWhiteFluor", values: None },
    306u16 => TagDef { name: "WB_RBLevelsCoolWhiteFluor", values: None },
    307u16 => TagDef { name: "WB_RBLevelsWhiteFluorescent", values: None },
    4096u16 => TagDef { name: "LightSource", values: Some(OLYMPUS_RAWINFO_LIGHTSOURCE_VALUES) },
    4097u16 => TagDef { name: "WhiteBalanceComp", values: None },
    4112u16 => TagDef { name: "SaturationSetting", values: None },
    4113u16 => TagDef { name: "HueSetting", values: None },
    4114u16 => TagDef { name: "ContrastSetting", values: None },
    4115u16 => TagDef { name: "SharpnessSetting", values: None },
    512u16 => TagDef { name: "ColorMatrix2", values: None },
    784u16 => TagDef { name: "CoringFilter", values: None },
    785u16 => TagDef { name: "CoringValues", values: None },
    8192u16 => TagDef { name: "CMExposureCompensation", values: None },
    8193u16 => TagDef { name: "CMWhiteBalance", values: None },
    8194u16 => TagDef { name: "CMWhiteBalanceComp", values: None },
    8208u16 => TagDef { name: "CMWhiteBalanceGrayPoint", values: None },
    8224u16 => TagDef { name: "CMSaturation", values: None },
    8225u16 => TagDef { name: "CMHue", values: None },
    8226u16 => TagDef { name: "CMContrast", values: None },
    8227u16 => TagDef { name: "CMSharpness", values: None },
};

pub static OLYMPUS_RAWINFO_LIGHTSOURCE_VALUES: &[(i64, &str)] = &[
    (0, "Unknown"),
    (16, "Shade"),
    (17, "Cloudy"),
    (18, "Fine Weather"),
    (20, "Tungsten (Incandescent)"),
    (22, "Evening Sunlight"),
    (256, "One Touch White Balance"),
    (33, "Daylight Fluorescent"),
    (34, "Day White Fluorescent"),
    (35, "Cool White Fluorescent"),
    (36, "White Fluorescent"),
    (512, "Custom 1-4"),
];

/// Olympus::SubjectDetectInfo tags
pub static OLYMPUS_SUBJECTDETECTINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "SubjectDetectFrameSize", values: None },
    10u16 => TagDef { name: "SubjectDetectStatus", values: Some(OLYMPUS_SUBJECTDETECTINFO_SUBJECTDETECTSTATUS_VALUES) },
    2u16 => TagDef { name: "SubjectDetectArea", values: None },
    6u16 => TagDef { name: "SubjectDetectDetail", values: None },
};

pub static OLYMPUS_SUBJECTDETECTINFO_SUBJECTDETECTSTATUS_VALUES: &[(i64, &str)] = &[
    (0, "No Data"),
    (257, "Subject and L1 Detail Detected"),
    (258, "Subject and L2 Detail Detected"),
    (260, "Subject Detected, No Details"),
    (515, "Face and Eye Detected"),
    (516, "Face Detected"),
    (771, "Subject Detail or Eye Detected"),
    (772, "No Subject or Face Detected"),
];


/// Look up a tag by ID in the main table.
pub fn lookup(_tag_id: u16) -> Option<&'static TagDef> {
    // Default to main table - override in specific modules
    None
}

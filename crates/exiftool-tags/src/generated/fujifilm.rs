//! FujiFilm MakerNotes tag definitions.
//! Auto-generated from ExifTool - DO NOT EDIT

#![allow(dead_code)]

/// Tag definition with name and optional value mappings.
#[derive(Debug, Clone)]
pub struct TagDef {
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}

/// FujiFilm::AFCSettings tags
pub static FUJIFILM_AFCSETTINGS: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "AF-CSetting", values: Some(FUJIFILM_AFCSETTINGS_AF_CSETTING_VALUES) },
};

pub static FUJIFILM_AFCSETTINGS_AF_CSETTING_VALUES: &[(i64, &str)] = &[
    (16, "Set 4 (suddenly appearing subject)"),
    (258, "Set 1 (multi-purpose)"),
    (290, "Set 3 (accelerating subject)"),
    (291, "Set 5 (erratic motion)"),
    (515, "Set 2 (ignore obstacles)"),
];

/// FujiFilm::Main tags
pub static FUJIFILM_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "Version", values: None },
    14339u16 => TagDef { name: "VideoRecordingMode", values: Some(FUJIFILM_MAIN_VIDEORECORDINGMODE_VALUES) },
    14340u16 => TagDef { name: "PeripheralLighting", values: Some(FUJIFILM_MAIN_PERIPHERALLIGHTING_VALUES) },
    14342u16 => TagDef { name: "VideoCompression", values: Some(FUJIFILM_MAIN_VIDEOCOMPRESSION_VALUES) },
    14368u16 => TagDef { name: "FrameRate", values: None },
    14369u16 => TagDef { name: "FrameWidth", values: None },
    14370u16 => TagDef { name: "FrameHeight", values: None },
    14372u16 => TagDef { name: "FullHDHighSpeedRec", values: Some(FUJIFILM_MAIN_FULLHDHIGHSPEEDREC_VALUES) },
    16u16 => TagDef { name: "InternalSerialNumber", values: None },
    16389u16 => TagDef { name: "FaceElementSelected", values: None },
    16640u16 => TagDef { name: "FacesDetected", values: None },
    16643u16 => TagDef { name: "FacePositions", values: None },
    16896u16 => TagDef { name: "NumFaceElements", values: None },
    16897u16 => TagDef { name: "FaceElementTypes", values: None },
    16899u16 => TagDef { name: "FaceElementPositions", values: None },
    17026u16 => TagDef { name: "FaceRecInfo", values: None },
    32768u16 => TagDef { name: "FileSource", values: None },
    32770u16 => TagDef { name: "OrderNumber", values: None },
    32771u16 => TagDef { name: "FrameNumber", values: None },
    4096u16 => TagDef { name: "Quality", values: None },
    4097u16 => TagDef { name: "Sharpness", values: Some(FUJIFILM_MAIN_SHARPNESS_VALUES) },
    4098u16 => TagDef { name: "WhiteBalance", values: Some(FUJIFILM_MAIN_WHITEBALANCE_VALUES) },
    4099u16 => TagDef { name: "Saturation", values: Some(FUJIFILM_MAIN_SATURATION_VALUES) },
    4100u16 => TagDef { name: "Contrast", values: Some(FUJIFILM_MAIN_CONTRAST_VALUES) },
    4101u16 => TagDef { name: "ColorTemperature", values: None },
    4102u16 => TagDef { name: "Contrast", values: Some(FUJIFILM_MAIN_CONTRAST_VALUES) },
    4106u16 => TagDef { name: "WhiteBalanceFineTune", values: None },
    4107u16 => TagDef { name: "NoiseReduction", values: Some(FUJIFILM_MAIN_NOISEREDUCTION_VALUES) },
    4110u16 => TagDef { name: "NoiseReduction", values: Some(FUJIFILM_MAIN_NOISEREDUCTION_VALUES) },
    4111u16 => TagDef { name: "Clarity", values: Some(FUJIFILM_MAIN_CLARITY_VALUES) },
    4112u16 => TagDef { name: "FujiFlashMode", values: Some(FUJIFILM_MAIN_FUJIFLASHMODE_VALUES) },
    4113u16 => TagDef { name: "FlashExposureComp", values: None },
    4128u16 => TagDef { name: "Macro", values: Some(FUJIFILM_MAIN_MACRO_VALUES) },
    4129u16 => TagDef { name: "FocusMode", values: Some(FUJIFILM_MAIN_FOCUSMODE_VALUES) },
    4130u16 => TagDef { name: "AFMode", values: Some(FUJIFILM_MAIN_AFMODE_VALUES) },
    4131u16 => TagDef { name: "FocusPixel", values: None },
    4139u16 => TagDef { name: "PrioritySettings", values: None },
    4141u16 => TagDef { name: "FocusSettings", values: None },
    4142u16 => TagDef { name: "AFCSettings", values: None },
    4144u16 => TagDef { name: "SlowSync", values: Some(FUJIFILM_MAIN_SLOWSYNC_VALUES) },
    4145u16 => TagDef { name: "PictureMode", values: Some(FUJIFILM_MAIN_PICTUREMODE_VALUES) },
    4146u16 => TagDef { name: "ExposureCount", values: None },
    4147u16 => TagDef { name: "EXRAuto", values: Some(FUJIFILM_MAIN_EXRAUTO_VALUES) },
    4148u16 => TagDef { name: "EXRMode", values: Some(FUJIFILM_MAIN_EXRMODE_VALUES) },
    4151u16 => TagDef { name: "MultipleExposure", values: Some(FUJIFILM_MAIN_MULTIPLEEXPOSURE_VALUES) },
    4160u16 => TagDef { name: "ShadowTone", values: Some(FUJIFILM_MAIN_SHADOWTONE_VALUES) },
    4161u16 => TagDef { name: "HighlightTone", values: Some(FUJIFILM_MAIN_HIGHLIGHTTONE_VALUES) },
    4164u16 => TagDef { name: "DigitalZoom", values: None },
    4165u16 => TagDef { name: "LensModulationOptimizer", values: Some(FUJIFILM_MAIN_LENSMODULATIONOPTIMIZER_VALUES) },
    4167u16 => TagDef { name: "GrainEffectRoughness", values: Some(FUJIFILM_MAIN_GRAINEFFECTROUGHNESS_VALUES) },
    4168u16 => TagDef { name: "ColorChromeEffect", values: Some(FUJIFILM_MAIN_COLORCHROMEEFFECT_VALUES) },
    4169u16 => TagDef { name: "BWAdjustment", values: None },
    4171u16 => TagDef { name: "BWMagentaGreen", values: None },
    4172u16 => TagDef { name: "GrainEffectSize", values: Some(FUJIFILM_MAIN_GRAINEFFECTSIZE_VALUES) },
    4173u16 => TagDef { name: "CropMode", values: Some(FUJIFILM_MAIN_CROPMODE_VALUES) },
    4174u16 => TagDef { name: "ColorChromeFXBlue", values: Some(FUJIFILM_MAIN_COLORCHROMEFXBLUE_VALUES) },
    4176u16 => TagDef { name: "ShutterType", values: Some(FUJIFILM_MAIN_SHUTTERTYPE_VALUES) },
    4177u16 => TagDef { name: "CropFlag", values: None },
    4178u16 => TagDef { name: "CropTopLeft", values: None },
    4179u16 => TagDef { name: "CropSize", values: None },
    4352u16 => TagDef { name: "AutoBracketing", values: Some(FUJIFILM_MAIN_AUTOBRACKETING_VALUES) },
    4353u16 => TagDef { name: "SequenceNumber", values: None },
    4354u16 => TagDef { name: "WhiteBalanceBracketing", values: Some(FUJIFILM_MAIN_WHITEBALANCEBRACKETING_VALUES) },
    4355u16 => TagDef { name: "DriveSettings", values: None },
    4357u16 => TagDef { name: "PixelShiftShots", values: None },
    4358u16 => TagDef { name: "PixelShiftOffset", values: None },
    4432u16 => TagDef { name: "CompositeImageMode", values: Some(FUJIFILM_MAIN_COMPOSITEIMAGEMODE_VALUES) },
    4433u16 => TagDef { name: "CompositeImageCount1", values: None },
    4434u16 => TagDef { name: "CompositeImageCount2", values: None },
    4435u16 => TagDef { name: "PanoramaAngle", values: None },
    4436u16 => TagDef { name: "PanoramaDirection", values: Some(FUJIFILM_MAIN_PANORAMADIRECTION_VALUES) },
    45585u16 => TagDef { name: "Parallax", values: None },
    4609u16 => TagDef { name: "AdvancedFilter", values: Some(FUJIFILM_MAIN_ADVANCEDFILTER_VALUES) },
    4624u16 => TagDef { name: "ColorMode", values: Some(FUJIFILM_MAIN_COLORMODE_VALUES) },
    4864u16 => TagDef { name: "BlurWarning", values: Some(FUJIFILM_MAIN_BLURWARNING_VALUES) },
    4865u16 => TagDef { name: "FocusWarning", values: Some(FUJIFILM_MAIN_FOCUSWARNING_VALUES) },
    4866u16 => TagDef { name: "ExposureWarning", values: Some(FUJIFILM_MAIN_EXPOSUREWARNING_VALUES) },
    4868u16 => TagDef { name: "GEImageSize", values: None },
    5120u16 => TagDef { name: "DynamicRange", values: Some(FUJIFILM_MAIN_DYNAMICRANGE_VALUES) },
    5121u16 => TagDef { name: "FilmMode", values: Some(FUJIFILM_MAIN_FILMMODE_VALUES) },
    5122u16 => TagDef { name: "DynamicRangeSetting", values: Some(FUJIFILM_MAIN_DYNAMICRANGESETTING_VALUES) },
    5123u16 => TagDef { name: "DevelopmentDynamicRange", values: None },
    5124u16 => TagDef { name: "MinFocalLength", values: None },
    5125u16 => TagDef { name: "MaxFocalLength", values: None },
    5126u16 => TagDef { name: "MaxApertureAtMinFocal", values: None },
    5127u16 => TagDef { name: "MaxApertureAtMaxFocal", values: None },
    5131u16 => TagDef { name: "AutoDynamicRange", values: None },
    5154u16 => TagDef { name: "ImageStabilization", values: None },
    5157u16 => TagDef { name: "SceneRecognition", values: Some(FUJIFILM_MAIN_SCENERECOGNITION_VALUES) },
    5169u16 => TagDef { name: "Rating", values: None },
    5174u16 => TagDef { name: "ImageGeneration", values: Some(FUJIFILM_MAIN_IMAGEGENERATION_VALUES) },
    5176u16 => TagDef { name: "ImageCount", values: None },
    5187u16 => TagDef { name: "DRangePriority", values: Some(FUJIFILM_MAIN_DRANGEPRIORITY_VALUES) },
    5188u16 => TagDef { name: "DRangePriorityAuto", values: Some(FUJIFILM_MAIN_DRANGEPRIORITYAUTO_VALUES) },
    5189u16 => TagDef { name: "DRangePriorityFixed", values: Some(FUJIFILM_MAIN_DRANGEPRIORITYFIXED_VALUES) },
    5190u16 => TagDef { name: "FlickerReduction", values: None },
    5191u16 => TagDef { name: "FujiModel", values: None },
    5192u16 => TagDef { name: "FujiModel2", values: None },
    5194u16 => TagDef { name: "WBRed", values: None },
    5195u16 => TagDef { name: "WBGreen", values: None },
    5196u16 => TagDef { name: "WBBlue", values: None },
    5197u16 => TagDef { name: "RollAngle", values: None },
};

pub static FUJIFILM_MAIN_VIDEORECORDINGMODE_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (16, "F-log"),
    (32, "HLG"),
    (48, "F-log2"),
];

pub static FUJIFILM_MAIN_PERIPHERALLIGHTING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static FUJIFILM_MAIN_VIDEOCOMPRESSION_VALUES: &[(i64, &str)] = &[
    (1, "Log GOP"),
    (2, "All Intra"),
];

pub static FUJIFILM_MAIN_FULLHDHIGHSPEEDREC_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "On"),
];

pub static FUJIFILM_MAIN_SHARPNESS_VALUES: &[(i64, &str)] = &[
    (0, "-4 (softest)"),
    (1, "-3 (very soft)"),
    (130, "-1 (medium soft)"),
    (132, "+1 (medium hard)"),
    (2, "-2 (soft)"),
    (3, "0 (normal)"),
    (32768, "Film Simulation"),
    (4, "+2 (hard)"),
    (5, "+3 (very hard)"),
    (6, "+4 (hardest)"),
    (65535, "n/a"),
];

pub static FUJIFILM_MAIN_WHITEBALANCE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Auto (white priority)"),
    (1024, "Incandescent"),
    (1280, "Flash"),
    (1536, "Underwater"),
    (2, "Auto (ambiance priority)"),
    (256, "Daylight"),
    (3840, "Custom"),
    (3841, "Custom2"),
    (3842, "Custom3"),
    (3843, "Custom4"),
    (3844, "Custom5"),
    (4080, "Kelvin"),
    (512, "Cloudy"),
    (768, "Daylight Fluorescent"),
    (769, "Day White Fluorescent"),
    (770, "White Fluorescent"),
    (771, "Warm White Fluorescent"),
    (772, "Living Room Warm White Fluorescent"),
];

pub static FUJIFILM_MAIN_SATURATION_VALUES: &[(i64, &str)] = &[
    (0, "0 (normal)"),
    (1024, "-2 (low)"),
    (1216, "-3 (very low)"),
    (1248, "-4 (lowest)"),
    (128, "+1 (medium high)"),
    (1280, "Acros"),
    (1281, "Acros Red Filter"),
    (1282, "Acros Yellow Filter"),
    (1283, "Acros Green Filter"),
    (192, "+3 (very high)"),
    (224, "+4 (highest)"),
    (256, "+2 (high)"),
    (32768, "Film Simulation"),
    (384, "-1 (medium low)"),
    (512, "Low"),
    (768, "None (B&W)"),
    (769, "B&W Red Filter"),
    (770, "B&W Yellow Filter"),
    (771, "B&W Green Filter"),
    (784, "B&W Sepia"),
];

pub static FUJIFILM_MAIN_CONTRAST_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (128, "Medium High"),
    (256, "High"),
    (32768, "Film Simulation"),
    (384, "Medium Low"),
    (512, "Low"),
];

pub static FUJIFILM_MAIN_NOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (128, "Normal"),
    (256, "n/a"),
    (64, "Low"),
];

pub static FUJIFILM_MAIN_CLARITY_VALUES: &[(i64, &str)] = &[
    (-1000, "-1"),
    (-2000, "-2"),
    (-3000, "-3"),
    (-4000, "-4"),
    (-5000, "-5"),
    (0, "0"),
    (1000, "1"),
    (2000, "2"),
    (3000, "3"),
    (4000, "4"),
    (5000, "5"),
];

pub static FUJIFILM_MAIN_FUJIFLASHMODE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "On"),
    (16, "Commander"),
    (2, "Off"),
    (3, "Red-eye reduction"),
    (32768, "Not Attached"),
    (33056, "TTL"),
    (33568, "TTL Auto - Did not fire"),
    (38976, "Manual"),
    (39008, "Flash Commander"),
    (39040, "Multi-flash"),
    (4, "External"),
    (43296, "1st Curtain (front)"),
    (43552, "TTL Slow - 1st Curtain (front)"),
    (43808, "TTL Auto - 1st Curtain (front)"),
    (44320, "TTL - Red-eye Flash - 1st Curtain (front)"),
    (44576, "TTL Slow - Red-eye Flash - 1st Curtain (front)"),
    (44832, "TTL Auto - Red-eye Flash - 1st Curtain (front)"),
    (51488, "2nd Curtain (rear)"),
    (51744, "TTL Slow - 2nd Curtain (rear)"),
    (52000, "TTL Auto - 2nd Curtain (rear)"),
    (52512, "TTL - Red-eye Flash - 2nd Curtain (rear)"),
    (52768, "TTL Slow - Red-eye Flash - 2nd Curtain (rear)"),
    (53024, "TTL Auto - Red-eye Flash - 2nd Curtain (rear)"),
    (59680, "High Speed Sync (HSS)"),
];

pub static FUJIFILM_MAIN_MACRO_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static FUJIFILM_MAIN_FOCUSMODE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Manual"),
    (65535, "Movie"),
];

pub static FUJIFILM_MAIN_AFMODE_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Single Point"),
    (256, "Zone"),
    (512, "Wide/Tracking"),
];

pub static FUJIFILM_MAIN_SLOWSYNC_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static FUJIFILM_MAIN_PICTUREMODE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Portrait"),
    (10, "Sunset"),
    (11, "Museum"),
    (12, "Party"),
    (13, "Flower"),
    (14, "Text"),
    (15, "Natural Light & Flash"),
    (16, "Beach"),
    (17, "Snow"),
    (18, "Fireworks"),
    (19, "Underwater"),
    (2, "Landscape"),
    (20, "Portrait with Skin Correction"),
    (22, "Panorama"),
    (23, "Night (tripod)"),
    (24, "Pro Low-light"),
    (25, "Pro Focus"),
    (256, "Aperture-priority AE"),
    (26, "Portrait 2"),
    (27, "Dog Face Detection"),
    (28, "Cat Face Detection"),
    (3, "Macro"),
    (4, "Sports"),
    (48, "HDR"),
    (5, "Night Scene"),
    (512, "Shutter speed priority AE"),
    (6, "Program AE"),
    (64, "Advanced Filter"),
    (7, "Natural Light"),
    (768, "Manual"),
    (8, "Anti-blur"),
    (9, "Beach & Snow"),
];

pub static FUJIFILM_MAIN_EXRAUTO_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Manual"),
];

pub static FUJIFILM_MAIN_EXRMODE_VALUES: &[(i64, &str)] = &[
    (256, "HR (High Resolution)"),
    (512, "SN (Signal to Noise priority)"),
    (768, "DR (Dynamic Range priority)"),
];

pub static FUJIFILM_MAIN_MULTIPLEEXPOSURE_VALUES: &[(i64, &str)] = &[
    (1, "Additive"),
    (2, "Average"),
    (3, "Light"),
    (4, "Dark"),
];

pub static FUJIFILM_MAIN_SHADOWTONE_VALUES: &[(i64, &str)] = &[
    (-16, "+1 (medium hard)"),
    (-32, "+2 (hard)"),
    (-48, "+3 (very hard)"),
    (-64, "+4 (hardest)"),
    (0, "0 (normal)"),
    (16, "-1 (medium soft)"),
    (32, "-2 (soft)"),
];

pub static FUJIFILM_MAIN_HIGHLIGHTTONE_VALUES: &[(i64, &str)] = &[
    (-16, "+1 (medium hard)"),
    (-32, "+2 (hard)"),
    (-48, "+3 (very hard)"),
    (-64, "+4 (hardest)"),
    (0, "0 (normal)"),
    (16, "-1 (medium soft)"),
    (32, "-2 (soft)"),
];

pub static FUJIFILM_MAIN_LENSMODULATIONOPTIMIZER_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static FUJIFILM_MAIN_GRAINEFFECTROUGHNESS_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (32, "Weak"),
    (64, "Strong"),
];

pub static FUJIFILM_MAIN_COLORCHROMEEFFECT_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (32, "Weak"),
    (64, "Strong"),
];

pub static FUJIFILM_MAIN_GRAINEFFECTSIZE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (16, "Small"),
    (32, "Large"),
];

pub static FUJIFILM_MAIN_CROPMODE_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "Full-frame on GFX"),
    (2, "Sports Finder Mode"),
    (4, "Electronic Shutter 1.25x Crop"),
    (8, "Digital Tele-Conv"),
];

pub static FUJIFILM_MAIN_COLORCHROMEFXBLUE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (32, "Weak"),
    (64, "Strong"),
];

pub static FUJIFILM_MAIN_SHUTTERTYPE_VALUES: &[(i64, &str)] = &[
    (0, "Mechanical"),
    (1, "Electronic"),
    (2, "Electronic (long shutter speed)"),
    (3, "Electronic Front Curtain"),
];

pub static FUJIFILM_MAIN_AUTOBRACKETING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
    (2, "Pre-shot"),
];

pub static FUJIFILM_MAIN_WHITEBALANCEBRACKETING_VALUES: &[(i64, &str)] = &[
    (1023, "+/- 3"),
    (511, "+/- 1"),
    (767, "+/- 2"),
];

pub static FUJIFILM_MAIN_COMPOSITEIMAGEMODE_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "Pro Low-light"),
    (1024, "Multi-exposure"),
    (128, "HDR"),
    (2, "Pro Focus"),
    (32, "Panorama"),
];

pub static FUJIFILM_MAIN_PANORAMADIRECTION_VALUES: &[(i64, &str)] = &[
    (1, "Right"),
    (2, "Left"),
    (3, "Up"),
    (4, "Down"),
];

pub static FUJIFILM_MAIN_ADVANCEDFILTER_VALUES: &[(i64, &str)] = &[
    (1048576, "Light Leak"),
    (1245184, "Expired Film Green"),
    (1245185, "Expired Film Red"),
    (1245186, "Expired Film Neutral"),
    (131072, "Hi Key"),
    (196608, "Toy Camera"),
    (262144, "Miniature"),
    (327680, "Dynamic Tone"),
    (393217, "Partial Color Red"),
    (393218, "Partial Color Yellow"),
    (393219, "Partial Color Green"),
    (393220, "Partial Color Blue"),
    (393221, "Partial Color Orange"),
    (393222, "Partial Color Purple"),
    (458752, "Soft Focus"),
    (589824, "Low Key"),
    (65536, "Pop Color"),
];

pub static FUJIFILM_MAIN_COLORMODE_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (16, "Chrome"),
    (48, "B & W"),
];

pub static FUJIFILM_MAIN_BLURWARNING_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "Blur Warning"),
];

pub static FUJIFILM_MAIN_FOCUSWARNING_VALUES: &[(i64, &str)] = &[
    (0, "Good"),
    (1, "Out of focus"),
];

pub static FUJIFILM_MAIN_EXPOSUREWARNING_VALUES: &[(i64, &str)] = &[
    (0, "Good"),
    (1, "Bad exposure"),
];

pub static FUJIFILM_MAIN_DYNAMICRANGE_VALUES: &[(i64, &str)] = &[
    (1, "Standard"),
    (3, "Wide"),
];

pub static FUJIFILM_MAIN_FILMMODE_VALUES: &[(i64, &str)] = &[
    (0, "F0/Standard (Provia)"),
    (1024, "F4/Velvia"),
    (1280, "Pro Neg. Std"),
    (1281, "Pro Neg. Hi"),
    (1536, "Classic Chrome"),
    (1792, "Eterna"),
    (2048, "Classic Negative"),
    (2304, "Bleach Bypass"),
    (256, "F1/Studio Portrait"),
    (2560, "Nostalgic Neg"),
    (272, "F1a/Studio Portrait Enhanced Saturation"),
    (2816, "Reala ACE"),
    (288, "F1b/Studio Portrait Smooth Skin Tone (Astia)"),
    (304, "F1c/Studio Portrait Increased Sharpness"),
    (512, "F2/Fujichrome (Velvia)"),
    (768, "F3/Studio Portrait Ex"),
];

pub static FUJIFILM_MAIN_DYNAMICRANGESETTING_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Manual"),
    (256, "Standard (100%)"),
    (32768, "Film Simulation"),
    (512, "Wide1 (230%)"),
    (513, "Wide2 (400%)"),
];

pub static FUJIFILM_MAIN_SCENERECOGNITION_VALUES: &[(i64, &str)] = &[
    (0, "Unrecognized"),
    (1024, "Macro"),
    (256, "Portrait Image"),
    (259, "Night Portrait"),
    (261, "Backlit Portrait"),
    (512, "Landscape Image"),
    (768, "Night Scene"),
];

pub static FUJIFILM_MAIN_IMAGEGENERATION_VALUES: &[(i64, &str)] = &[
    (0, "Original Image"),
    (1, "Re-developed from RAW"),
];

pub static FUJIFILM_MAIN_DRANGEPRIORITY_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Fixed"),
];

pub static FUJIFILM_MAIN_DRANGEPRIORITYAUTO_VALUES: &[(i64, &str)] = &[
    (1, "Weak"),
    (2, "Strong"),
    (3, "Plus"),
];

pub static FUJIFILM_MAIN_DRANGEPRIORITYFIXED_VALUES: &[(i64, &str)] = &[
    (1, "Weak"),
    (2, "Strong"),
];


/// Look up a tag by ID in the main table.
pub fn lookup(_tag_id: u16) -> Option<&'static TagDef> {
    // Default to main table - override in specific modules
    None
}

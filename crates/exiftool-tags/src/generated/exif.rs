//! Exif MakerNotes tag definitions.
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

/// DNG::ImageSeq tags
pub static DNG_IMAGESEQ: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "SeqID", values: None },
    1u16 => TagDef { name: "SeqType", values: None },
    11u16 => TagDef { name: "SeqFinal", values: Some(DNG_IMAGESEQ_SEQFINAL_VALUES) },
    2u16 => TagDef { name: "SeqFrameInfo", values: None },
    3u16 => TagDef { name: "SeqIndex", values: None },
    7u16 => TagDef { name: "SeqCount", values: None },
};

pub static DNG_IMAGESEQ_SEQFINAL_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];

/// DNG::OriginalRaw tags
pub static DNG_ORIGINALRAW: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "OriginalRawImage", values: None },
    1u16 => TagDef { name: "OriginalRawResource", values: None },
    4u16 => TagDef { name: "OriginalTHMImage", values: None },
    5u16 => TagDef { name: "OriginalTHMResource", values: None },
};

/// DNG::ProfileDynamicRange tags
pub static DNG_PROFILEDYNAMICRANGE: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "PDRVersion", values: None },
    2u16 => TagDef { name: "DynamicRange", values: Some(DNG_PROFILEDYNAMICRANGE_DYNAMICRANGE_VALUES) },
    4u16 => TagDef { name: "HintMaxOutputValue", values: None },
};

pub static DNG_PROFILEDYNAMICRANGE_DYNAMICRANGE_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (1, "High"),
];

/// Exif::Main tags
pub static EXIF_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "InteropIndex", values: None },
    11u16 => TagDef { name: "ProcessingSoftware", values: None },
    18246u16 => TagDef { name: "Rating", values: None },
    18247u16 => TagDef { name: "XP_DIP_XML", values: None },
    18248u16 => TagDef { name: "StitchInfo", values: None },
    18249u16 => TagDef { name: "RatingPercent", values: None },
    2u16 => TagDef { name: "InteropVersion", values: None },
    20481u16 => TagDef { name: "ResolutionXUnit", values: None },
    20497u16 => TagDef { name: "GridSize", values: None },
    20506u16 => TagDef { name: "ColorTransferFunction", values: None },
    20507u16 => TagDef { name: "ThumbnailData", values: None },
    20531u16 => TagDef { name: "ThumbnailDateTime", values: None },
    254u16 => TagDef { name: "SubfileType", values: Some(EXIF_MAIN_SUBFILETYPE_VALUES) },
    255u16 => TagDef { name: "OldSubfileType", values: Some(EXIF_MAIN_OLDSUBFILETYPE_VALUES) },
    256u16 => TagDef { name: "ImageWidth", values: None },
    257u16 => TagDef { name: "ImageHeight", values: None },
    258u16 => TagDef { name: "BitsPerSample", values: None },
    259u16 => TagDef { name: "Compression", values: Some(EXIF_MAIN_COMPRESSION_VALUES) },
    262u16 => TagDef { name: "PhotometricInterpretation", values: Some(EXIF_MAIN_PHOTOMETRICINTERPRETATION_VALUES) },
    263u16 => TagDef { name: "Thresholding", values: Some(EXIF_MAIN_THRESHOLDING_VALUES) },
    264u16 => TagDef { name: "CellWidth", values: None },
    265u16 => TagDef { name: "CellLength", values: None },
    266u16 => TagDef { name: "FillOrder", values: Some(EXIF_MAIN_FILLORDER_VALUES) },
    269u16 => TagDef { name: "DocumentName", values: None },
    270u16 => TagDef { name: "ImageDescription", values: None },
    271u16 => TagDef { name: "Make", values: None },
    272u16 => TagDef { name: "Model", values: None },
    273u16 => TagDef { name: "StripOffsets", values: None },
    274u16 => TagDef { name: "Orientation", values: Some(EXIF_MAIN_ORIENTATION_VALUES) },
    277u16 => TagDef { name: "SamplesPerPixel", values: None },
    278u16 => TagDef { name: "RowsPerStrip", values: None },
    279u16 => TagDef { name: "StripByteCounts", values: None },
    280u16 => TagDef { name: "MinSampleValue", values: None },
    281u16 => TagDef { name: "MaxSampleValue", values: None },
    282u16 => TagDef { name: "XResolution", values: None },
    283u16 => TagDef { name: "YResolution", values: None },
    284u16 => TagDef { name: "PlanarConfiguration", values: Some(EXIF_MAIN_PLANARCONFIGURATION_VALUES) },
    285u16 => TagDef { name: "PageName", values: None },
    286u16 => TagDef { name: "XPosition", values: None },
    28672u16 => TagDef { name: "SonyRawFileType", values: Some(EXIF_MAIN_SONYRAWFILETYPE_VALUES) },
    28688u16 => TagDef { name: "SonyToneCurve", values: None },
    287u16 => TagDef { name: "YPosition", values: None },
    28721u16 => TagDef { name: "VignettingCorrection", values: Some(EXIF_MAIN_VIGNETTINGCORRECTION_VALUES) },
    28722u16 => TagDef { name: "VignettingCorrParams", values: None },
    28724u16 => TagDef { name: "ChromaticAberrationCorrection", values: Some(EXIF_MAIN_CHROMATICABERRATIONCORRECTION_VALUES) },
    28725u16 => TagDef { name: "ChromaticAberrationCorrParams", values: None },
    28726u16 => TagDef { name: "DistortionCorrection", values: Some(EXIF_MAIN_DISTORTIONCORRECTION_VALUES) },
    28727u16 => TagDef { name: "DistortionCorrParams", values: None },
    28728u16 => TagDef { name: "SonyRawImageSize", values: None },
    288u16 => TagDef { name: "FreeOffsets", values: None },
    289u16 => TagDef { name: "FreeByteCounts", values: None },
    290u16 => TagDef { name: "GrayResponseUnit", values: Some(EXIF_MAIN_GRAYRESPONSEUNIT_VALUES) },
    291u16 => TagDef { name: "GrayResponseCurve", values: None },
    292u16 => TagDef { name: "T4Options", values: None },
    293u16 => TagDef { name: "T6Options", values: None },
    29456u16 => TagDef { name: "BlackLevel", values: None },
    29459u16 => TagDef { name: "WB_RGGBLevels", values: None },
    296u16 => TagDef { name: "ResolutionUnit", values: Some(EXIF_MAIN_RESOLUTIONUNIT_VALUES) },
    297u16 => TagDef { name: "PageNumber", values: None },
    29895u16 => TagDef { name: "SonyCropTopLeft", values: None },
    29896u16 => TagDef { name: "SonyCropSize", values: None },
    301u16 => TagDef { name: "TransferFunction", values: None },
    305u16 => TagDef { name: "Software", values: None },
    306u16 => TagDef { name: "ModifyDate", values: None },
    315u16 => TagDef { name: "Artist", values: None },
    316u16 => TagDef { name: "HostComputer", values: None },
    317u16 => TagDef { name: "Predictor", values: Some(EXIF_MAIN_PREDICTOR_VALUES) },
    318u16 => TagDef { name: "WhitePoint", values: None },
    319u16 => TagDef { name: "PrimaryChromaticities", values: None },
    320u16 => TagDef { name: "ColorMap", values: None },
    321u16 => TagDef { name: "HalftoneHints", values: None },
    322u16 => TagDef { name: "TileWidth", values: None },
    323u16 => TagDef { name: "TileLength", values: None },
    324u16 => TagDef { name: "TileOffsets", values: None },
    325u16 => TagDef { name: "TileByteCounts", values: None },
    327u16 => TagDef { name: "CleanFaxData", values: Some(EXIF_MAIN_CLEANFAXDATA_VALUES) },
    32931u16 => TagDef { name: "WangTag1", values: None },
    32932u16 => TagDef { name: "WangAnnotation", values: None },
    32933u16 => TagDef { name: "WangTag3", values: None },
    32934u16 => TagDef { name: "WangTag4", values: None },
    330u16 => TagDef { name: "SubIFD", values: None },
    332u16 => TagDef { name: "InkSet", values: Some(EXIF_MAIN_INKSET_VALUES) },
    33421u16 => TagDef { name: "CFARepeatPatternDim", values: None },
    33422u16 => TagDef { name: "CFAPattern2", values: None },
    33423u16 => TagDef { name: "BatteryLevel", values: None },
    33424u16 => TagDef { name: "KodakIFD", values: None },
    33432u16 => TagDef { name: "Copyright", values: None },
    33434u16 => TagDef { name: "ExposureTime", values: None },
    33437u16 => TagDef { name: "FNumber", values: None },
    33445u16 => TagDef { name: "MDFileTag", values: None },
    33550u16 => TagDef { name: "PixelScale", values: None },
    337u16 => TagDef { name: "TargetPrinter", values: None },
    33723u16 => TagDef { name: "IPTC-NAA", values: None },
    338u16 => TagDef { name: "ExtraSamples", values: Some(EXIF_MAIN_EXTRASAMPLES_VALUES) },
    339u16 => TagDef { name: "SampleFormat", values: None },
    33920u16 => TagDef { name: "IntergraphMatrix", values: None },
    33922u16 => TagDef { name: "ModelTiePoint", values: None },
    34019u16 => TagDef { name: "RasterPadding", values: Some(EXIF_MAIN_RASTERPADDING_VALUES) },
    34023u16 => TagDef { name: "ImageColorIndicator", values: Some(EXIF_MAIN_IMAGECOLORINDICATOR_VALUES) },
    34024u16 => TagDef { name: "BackgroundColorIndicator", values: Some(EXIF_MAIN_BACKGROUNDCOLORINDICATOR_VALUES) },
    34030u16 => TagDef { name: "HCUsage", values: Some(EXIF_MAIN_HCUSAGE_VALUES) },
    34118u16 => TagDef { name: "SEMInfo", values: None },
    34152u16 => TagDef { name: "AFCP_IPTC", values: None },
    34264u16 => TagDef { name: "ModelTransform", values: None },
    34306u16 => TagDef { name: "WB_GRGBLevels", values: None },
    34310u16 => TagDef { name: "LeafData", values: None },
    34377u16 => TagDef { name: "PhotoshopSettings", values: None },
    346u16 => TagDef { name: "Indexed", values: Some(EXIF_MAIN_INDEXED_VALUES) },
    34665u16 => TagDef { name: "ExifOffset", values: None },
    34675u16 => TagDef { name: "ICC_Profile", values: None },
    34687u16 => TagDef { name: "TIFF_FXExtensions", values: None },
    34688u16 => TagDef { name: "MultiProfiles", values: None },
    34689u16 => TagDef { name: "SharedData", values: None },
    347u16 => TagDef { name: "JPEGTables", values: None },
    34735u16 => TagDef { name: "GeoTiffDirectory", values: None },
    34736u16 => TagDef { name: "GeoTiffDoubleParams", values: None },
    34737u16 => TagDef { name: "GeoTiffAsciiParams", values: None },
    34850u16 => TagDef { name: "ExposureProgram", values: Some(EXIF_MAIN_EXPOSUREPROGRAM_VALUES) },
    34852u16 => TagDef { name: "SpectralSensitivity", values: None },
    34853u16 => TagDef { name: "GPSInfo", values: None },
    34855u16 => TagDef { name: "ISO", values: None },
    34856u16 => TagDef { name: "Opto-ElectricConvFactor", values: None },
    34858u16 => TagDef { name: "TimeZoneOffset", values: None },
    34859u16 => TagDef { name: "SelfTimerMode", values: None },
    34864u16 => TagDef { name: "SensitivityType", values: Some(EXIF_MAIN_SENSITIVITYTYPE_VALUES) },
    34865u16 => TagDef { name: "StandardOutputSensitivity", values: None },
    34866u16 => TagDef { name: "RecommendedExposureIndex", values: None },
    34867u16 => TagDef { name: "ISOSpeed", values: None },
    34868u16 => TagDef { name: "ISOSpeedLatitudeyyy", values: None },
    34869u16 => TagDef { name: "ISOSpeedLatitudezzz", values: None },
    34954u16 => TagDef { name: "LeafSubIFD", values: None },
    351u16 => TagDef { name: "OPIProxy", values: Some(EXIF_MAIN_OPIPROXY_VALUES) },
    36864u16 => TagDef { name: "ExifVersion", values: None },
    36867u16 => TagDef { name: "DateTimeOriginal", values: None },
    36868u16 => TagDef { name: "CreateDate", values: None },
    36873u16 => TagDef { name: "GooglePlusUploadCode", values: None },
    36880u16 => TagDef { name: "OffsetTime", values: None },
    36881u16 => TagDef { name: "OffsetTimeOriginal", values: None },
    36882u16 => TagDef { name: "OffsetTimeDigitized", values: None },
    37121u16 => TagDef { name: "ComponentsConfiguration", values: Some(EXIF_MAIN_COMPONENTSCONFIGURATION_VALUES) },
    37122u16 => TagDef { name: "CompressedBitsPerPixel", values: None },
    37377u16 => TagDef { name: "ShutterSpeedValue", values: None },
    37378u16 => TagDef { name: "ApertureValue", values: None },
    37379u16 => TagDef { name: "BrightnessValue", values: None },
    37380u16 => TagDef { name: "ExposureCompensation", values: None },
    37381u16 => TagDef { name: "MaxApertureValue", values: None },
    37382u16 => TagDef { name: "SubjectDistance", values: None },
    37383u16 => TagDef { name: "MeteringMode", values: Some(EXIF_MAIN_METERINGMODE_VALUES) },
    37384u16 => TagDef { name: "LightSource", values: Some(EXIF_MAIN_LIGHTSOURCE_VALUES) },
    37385u16 => TagDef { name: "Flash", values: Some(EXIF_MAIN_FLASH_VALUES) },
    37386u16 => TagDef { name: "FocalLength", values: None },
    37387u16 => TagDef { name: "FlashEnergy", values: None },
    37392u16 => TagDef { name: "FocalPlaneResolutionUnit", values: Some(EXIF_MAIN_FOCALPLANERESOLUTIONUNIT_VALUES) },
    37393u16 => TagDef { name: "ImageNumber", values: None },
    37394u16 => TagDef { name: "SecurityClassification", values: None },
    37395u16 => TagDef { name: "ImageHistory", values: None },
    37396u16 => TagDef { name: "SubjectArea", values: None },
    37398u16 => TagDef { name: "TIFF-EPStandardID", values: None },
    37399u16 => TagDef { name: "SensingMethod", values: Some(EXIF_MAIN_SENSINGMETHOD_VALUES) },
    37500u16 => TagDef { name: "MakerNoteApple", values: None },
    37510u16 => TagDef { name: "UserComment", values: None },
    37520u16 => TagDef { name: "SubSecTime", values: None },
    37521u16 => TagDef { name: "SubSecTimeOriginal", values: None },
    37522u16 => TagDef { name: "SubSecTimeDigitized", values: None },
    37680u16 => TagDef { name: "MSPropertySetStorage", values: None },
    37681u16 => TagDef { name: "MSDocumentTextPosition", values: None },
    37724u16 => TagDef { name: "ImageSourceData", values: None },
    37888u16 => TagDef { name: "AmbientTemperature", values: None },
    37889u16 => TagDef { name: "Humidity", values: None },
    37890u16 => TagDef { name: "Pressure", values: None },
    37891u16 => TagDef { name: "WaterDepth", values: None },
    37892u16 => TagDef { name: "Acceleration", values: None },
    37893u16 => TagDef { name: "CameraElevationAngle", values: None },
    39321u16 => TagDef { name: "XiaomiSettings", values: None },
    39424u16 => TagDef { name: "XiaomiModel", values: None },
    400u16 => TagDef { name: "GlobalParametersIFD", values: None },
    40091u16 => TagDef { name: "XPTitle", values: None },
    40092u16 => TagDef { name: "XPComment", values: None },
    40093u16 => TagDef { name: "XPAuthor", values: None },
    40094u16 => TagDef { name: "XPKeywords", values: None },
    40095u16 => TagDef { name: "XPSubject", values: None },
    401u16 => TagDef { name: "ProfileType", values: Some(EXIF_MAIN_PROFILETYPE_VALUES) },
    402u16 => TagDef { name: "FaxProfile", values: Some(EXIF_MAIN_FAXPROFILE_VALUES) },
    403u16 => TagDef { name: "CodingMethods", values: None },
    4096u16 => TagDef { name: "RelatedImageFileFormat", values: None },
    40960u16 => TagDef { name: "FlashpixVersion", values: None },
    40961u16 => TagDef { name: "ColorSpace", values: Some(EXIF_MAIN_COLORSPACE_VALUES) },
    40962u16 => TagDef { name: "ExifImageWidth", values: None },
    40963u16 => TagDef { name: "ExifImageHeight", values: None },
    40964u16 => TagDef { name: "RelatedSoundFile", values: None },
    40965u16 => TagDef { name: "InteropOffset", values: None },
    4097u16 => TagDef { name: "RelatedImageWidth", values: None },
    40976u16 => TagDef { name: "SamsungRawPointersOffset", values: None },
    40977u16 => TagDef { name: "SamsungRawPointersLength", values: None },
    4098u16 => TagDef { name: "RelatedImageHeight", values: None },
    41217u16 => TagDef { name: "SamsungRawByteOrder", values: None },
    41218u16 => TagDef { name: "SamsungRawUnknown", values: None },
    41483u16 => TagDef { name: "FlashEnergy", values: None },
    41484u16 => TagDef { name: "SpatialFrequencyResponse", values: None },
    41486u16 => TagDef { name: "FocalPlaneXResolution", values: None },
    41487u16 => TagDef { name: "FocalPlaneYResolution", values: None },
    41488u16 => TagDef { name: "FocalPlaneResolutionUnit", values: Some(EXIF_MAIN_FOCALPLANERESOLUTIONUNIT_VALUES) },
    41492u16 => TagDef { name: "SubjectLocation", values: None },
    41493u16 => TagDef { name: "ExposureIndex", values: None },
    41494u16 => TagDef { name: "TIFF-EPStandardID", values: None },
    41495u16 => TagDef { name: "SensingMethod", values: Some(EXIF_MAIN_SENSINGMETHOD_VALUES) },
    41728u16 => TagDef { name: "FileSource", values: Some(EXIF_MAIN_FILESOURCE_VALUES) },
    41729u16 => TagDef { name: "SceneType", values: Some(EXIF_MAIN_SCENETYPE_VALUES) },
    41730u16 => TagDef { name: "CFAPattern", values: None },
    41985u16 => TagDef { name: "CustomRendered", values: Some(EXIF_MAIN_CUSTOMRENDERED_VALUES) },
    41986u16 => TagDef { name: "ExposureMode", values: Some(EXIF_MAIN_EXPOSUREMODE_VALUES) },
    41987u16 => TagDef { name: "WhiteBalance", values: Some(EXIF_MAIN_WHITEBALANCE_VALUES) },
    41988u16 => TagDef { name: "DigitalZoomRatio", values: None },
    41989u16 => TagDef { name: "FocalLengthIn35mmFormat", values: None },
    41990u16 => TagDef { name: "SceneCaptureType", values: Some(EXIF_MAIN_SCENECAPTURETYPE_VALUES) },
    41991u16 => TagDef { name: "GainControl", values: Some(EXIF_MAIN_GAINCONTROL_VALUES) },
    41992u16 => TagDef { name: "Contrast", values: Some(EXIF_MAIN_CONTRAST_VALUES) },
    41993u16 => TagDef { name: "Saturation", values: Some(EXIF_MAIN_SATURATION_VALUES) },
    41994u16 => TagDef { name: "Sharpness", values: Some(EXIF_MAIN_SHARPNESS_VALUES) },
    41995u16 => TagDef { name: "DeviceSettingDescription", values: None },
    41996u16 => TagDef { name: "SubjectDistanceRange", values: Some(EXIF_MAIN_SUBJECTDISTANCERANGE_VALUES) },
    42016u16 => TagDef { name: "ImageUniqueID", values: None },
    42032u16 => TagDef { name: "OwnerName", values: None },
    42033u16 => TagDef { name: "SerialNumber", values: None },
    42034u16 => TagDef { name: "LensInfo", values: None },
    42035u16 => TagDef { name: "LensMake", values: None },
    42036u16 => TagDef { name: "LensModel", values: None },
    42037u16 => TagDef { name: "LensSerialNumber", values: None },
    42038u16 => TagDef { name: "ImageTitle", values: None },
    42039u16 => TagDef { name: "Photographer", values: None },
    42040u16 => TagDef { name: "ImageEditor", values: None },
    42041u16 => TagDef { name: "CameraFirmware", values: None },
    42042u16 => TagDef { name: "RAWDevelopingSoftware", values: None },
    42043u16 => TagDef { name: "ImageEditingSoftware", values: None },
    42044u16 => TagDef { name: "MetadataEditingSoftware", values: None },
    42080u16 => TagDef { name: "CompositeImage", values: Some(EXIF_MAIN_COMPOSITEIMAGE_VALUES) },
    42081u16 => TagDef { name: "CompositeImageCount", values: None },
    42082u16 => TagDef { name: "CompositeImageExposureTimes", values: None },
    42112u16 => TagDef { name: "GDALMetadata", values: None },
    42113u16 => TagDef { name: "GDALNoData", values: None },
    42240u16 => TagDef { name: "Gamma", values: None },
    437u16 => TagDef { name: "JPEGTables", values: None },
    46275u16 => TagDef { name: "HasselbladRawImage", values: None },
    48129u16 => TagDef { name: "PixelFormat", values: Some(EXIF_MAIN_PIXELFORMAT_VALUES) },
    48130u16 => TagDef { name: "Transformation", values: Some(EXIF_MAIN_TRANSFORMATION_VALUES) },
    48131u16 => TagDef { name: "Uncompressed", values: Some(EXIF_MAIN_UNCOMPRESSED_VALUES) },
    48132u16 => TagDef { name: "ImageType", values: None },
    48320u16 => TagDef { name: "ImageOffset", values: None },
    48321u16 => TagDef { name: "ImageByteCount", values: None },
    48322u16 => TagDef { name: "AlphaOffset", values: None },
    48323u16 => TagDef { name: "AlphaByteCount", values: None },
    48324u16 => TagDef { name: "ImageDataDiscard", values: Some(EXIF_MAIN_IMAGEDATADISCARD_VALUES) },
    48325u16 => TagDef { name: "AlphaDataDiscard", values: Some(EXIF_MAIN_ALPHADATADISCARD_VALUES) },
    50255u16 => TagDef { name: "Annotations", values: None },
    50341u16 => TagDef { name: "PrintIM", values: None },
    50457u16 => TagDef { name: "HasselbladXML", values: None },
    50459u16 => TagDef { name: "HasselbladExif", values: None },
    50547u16 => TagDef { name: "OriginalFileName", values: None },
    50560u16 => TagDef { name: "USPTOOriginalContentType", values: Some(EXIF_MAIN_USPTOORIGINALCONTENTTYPE_VALUES) },
    50656u16 => TagDef { name: "CR2CFAPattern", values: None },
    50706u16 => TagDef { name: "DNGVersion", values: None },
    50707u16 => TagDef { name: "DNGBackwardVersion", values: None },
    50708u16 => TagDef { name: "UniqueCameraModel", values: None },
    50709u16 => TagDef { name: "LocalizedCameraModel", values: None },
    50710u16 => TagDef { name: "CFAPlaneColor", values: None },
    50711u16 => TagDef { name: "CFALayout", values: Some(EXIF_MAIN_CFALAYOUT_VALUES) },
    50712u16 => TagDef { name: "LinearizationTable", values: None },
    50713u16 => TagDef { name: "BlackLevelRepeatDim", values: None },
    50714u16 => TagDef { name: "BlackLevel", values: None },
    50715u16 => TagDef { name: "BlackLevelDeltaH", values: None },
    50716u16 => TagDef { name: "BlackLevelDeltaV", values: None },
    50717u16 => TagDef { name: "WhiteLevel", values: None },
    50718u16 => TagDef { name: "DefaultScale", values: None },
    50719u16 => TagDef { name: "DefaultCropOrigin", values: None },
    50720u16 => TagDef { name: "DefaultCropSize", values: None },
    50721u16 => TagDef { name: "ColorMatrix1", values: None },
    50722u16 => TagDef { name: "ColorMatrix2", values: None },
    50723u16 => TagDef { name: "CameraCalibration1", values: None },
    50724u16 => TagDef { name: "CameraCalibration2", values: None },
    50725u16 => TagDef { name: "ReductionMatrix1", values: None },
    50726u16 => TagDef { name: "ReductionMatrix2", values: None },
    50727u16 => TagDef { name: "AnalogBalance", values: None },
    50728u16 => TagDef { name: "AsShotNeutral", values: None },
    50729u16 => TagDef { name: "AsShotWhiteXY", values: None },
    50730u16 => TagDef { name: "BaselineExposure", values: None },
    50731u16 => TagDef { name: "BaselineNoise", values: None },
    50732u16 => TagDef { name: "BaselineSharpness", values: None },
    50733u16 => TagDef { name: "BayerGreenSplit", values: None },
    50734u16 => TagDef { name: "LinearResponseLimit", values: None },
    50735u16 => TagDef { name: "CameraSerialNumber", values: None },
    50736u16 => TagDef { name: "DNGLensInfo", values: None },
    50737u16 => TagDef { name: "ChromaBlurRadius", values: None },
    50738u16 => TagDef { name: "AntiAliasStrength", values: None },
    50739u16 => TagDef { name: "ShadowScale", values: None },
    50740u16 => TagDef { name: "SR2Private", values: None },
    50741u16 => TagDef { name: "MakerNoteSafety", values: Some(EXIF_MAIN_MAKERNOTESAFETY_VALUES) },
    50752u16 => TagDef { name: "RawImageSegmentation", values: None },
    50778u16 => TagDef { name: "CalibrationIlluminant1", values: Some(EXIF_MAIN_CALIBRATIONILLUMINANT1_VALUES) },
    50779u16 => TagDef { name: "CalibrationIlluminant2", values: Some(EXIF_MAIN_CALIBRATIONILLUMINANT2_VALUES) },
    50780u16 => TagDef { name: "BestQualityScale", values: None },
    50781u16 => TagDef { name: "RawDataUniqueID", values: None },
    50784u16 => TagDef { name: "AliasLayerMetadata", values: None },
    50827u16 => TagDef { name: "OriginalRawFileName", values: None },
    50828u16 => TagDef { name: "OriginalRawFileData", values: None },
    50829u16 => TagDef { name: "ActiveArea", values: None },
    50830u16 => TagDef { name: "MaskedAreas", values: None },
    50831u16 => TagDef { name: "AsShotICCProfile", values: None },
    50832u16 => TagDef { name: "AsShotPreProfileMatrix", values: None },
    50833u16 => TagDef { name: "CurrentICCProfile", values: None },
    50834u16 => TagDef { name: "CurrentPreProfileMatrix", values: None },
    50879u16 => TagDef { name: "ColorimetricReference", values: Some(EXIF_MAIN_COLORIMETRICREFERENCE_VALUES) },
    50885u16 => TagDef { name: "SRawType", values: None },
    50898u16 => TagDef { name: "PanasonicTitle", values: None },
    50899u16 => TagDef { name: "PanasonicTitle2", values: None },
    50931u16 => TagDef { name: "CameraCalibrationSig", values: None },
    50932u16 => TagDef { name: "ProfileCalibrationSig", values: None },
    50933u16 => TagDef { name: "ProfileIFD", values: None },
    50934u16 => TagDef { name: "AsShotProfileName", values: None },
    50935u16 => TagDef { name: "NoiseReductionApplied", values: None },
    50936u16 => TagDef { name: "ProfileName", values: None },
    50937u16 => TagDef { name: "ProfileHueSatMapDims", values: None },
    50938u16 => TagDef { name: "ProfileHueSatMapData1", values: None },
    50939u16 => TagDef { name: "ProfileHueSatMapData2", values: None },
    50940u16 => TagDef { name: "ProfileToneCurve", values: None },
    50941u16 => TagDef { name: "ProfileEmbedPolicy", values: Some(EXIF_MAIN_PROFILEEMBEDPOLICY_VALUES) },
    50942u16 => TagDef { name: "ProfileCopyright", values: None },
    50964u16 => TagDef { name: "ForwardMatrix1", values: None },
    50965u16 => TagDef { name: "ForwardMatrix2", values: None },
    50966u16 => TagDef { name: "PreviewApplicationName", values: None },
    50967u16 => TagDef { name: "PreviewApplicationVersion", values: None },
    50968u16 => TagDef { name: "PreviewSettingsName", values: None },
    50969u16 => TagDef { name: "PreviewSettingsDigest", values: None },
    50970u16 => TagDef { name: "PreviewColorSpace", values: Some(EXIF_MAIN_PREVIEWCOLORSPACE_VALUES) },
    50971u16 => TagDef { name: "PreviewDateTime", values: None },
    50972u16 => TagDef { name: "RawImageDigest", values: None },
    50973u16 => TagDef { name: "OriginalRawFileDigest", values: None },
    50981u16 => TagDef { name: "ProfileLookTableDims", values: None },
    50982u16 => TagDef { name: "ProfileLookTableData", values: None },
    51008u16 => TagDef { name: "OpcodeList1", values: Some(EXIF_MAIN_OPCODELIST1_VALUES) },
    51009u16 => TagDef { name: "OpcodeList2", values: Some(EXIF_MAIN_OPCODELIST2_VALUES) },
    51022u16 => TagDef { name: "OpcodeList3", values: Some(EXIF_MAIN_OPCODELIST3_VALUES) },
    51041u16 => TagDef { name: "NoiseProfile", values: None },
    51043u16 => TagDef { name: "TimeCodes", values: None },
    51044u16 => TagDef { name: "FrameRate", values: None },
    51058u16 => TagDef { name: "TStop", values: None },
    51081u16 => TagDef { name: "ReelName", values: None },
    51089u16 => TagDef { name: "OriginalDefaultFinalSize", values: None },
    51090u16 => TagDef { name: "OriginalBestQualitySize", values: None },
    51091u16 => TagDef { name: "OriginalDefaultCropSize", values: None },
    51105u16 => TagDef { name: "CameraLabel", values: None },
    51107u16 => TagDef { name: "ProfileHueSatMapEncoding", values: Some(EXIF_MAIN_PROFILEHUESATMAPENCODING_VALUES) },
    51108u16 => TagDef { name: "ProfileLookTableEncoding", values: Some(EXIF_MAIN_PROFILELOOKTABLEENCODING_VALUES) },
    51109u16 => TagDef { name: "BaselineExposureOffset", values: None },
    51110u16 => TagDef { name: "DefaultBlackRender", values: Some(EXIF_MAIN_DEFAULTBLACKRENDER_VALUES) },
    51111u16 => TagDef { name: "NewRawImageDigest", values: None },
    51112u16 => TagDef { name: "RawToPreviewGain", values: None },
    51114u16 => TagDef { name: "CacheVersion", values: None },
    51125u16 => TagDef { name: "DefaultUserCrop", values: None },
    51157u16 => TagDef { name: "NikonNEFInfo", values: None },
    51159u16 => TagDef { name: "ZIFMetadata", values: None },
    51160u16 => TagDef { name: "ZIFAnnotations", values: None },
    51177u16 => TagDef { name: "DepthFormat", values: Some(EXIF_MAIN_DEPTHFORMAT_VALUES) },
    51178u16 => TagDef { name: "DepthNear", values: None },
    51179u16 => TagDef { name: "DepthFar", values: None },
    51180u16 => TagDef { name: "DepthUnits", values: Some(EXIF_MAIN_DEPTHUNITS_VALUES) },
    51181u16 => TagDef { name: "DepthMeasureType", values: Some(EXIF_MAIN_DEPTHMEASURETYPE_VALUES) },
    51182u16 => TagDef { name: "EnhanceParams", values: None },
    512u16 => TagDef { name: "JPEGProc", values: Some(EXIF_MAIN_JPEGPROC_VALUES) },
    513u16 => TagDef { name: "ThumbnailOffset", values: None },
    514u16 => TagDef { name: "ThumbnailLength", values: None },
    519u16 => TagDef { name: "JPEGQTables", values: None },
    520u16 => TagDef { name: "JPEGDCTables", values: None },
    521u16 => TagDef { name: "JPEGACTables", values: None },
    52525u16 => TagDef { name: "ProfileGainTableMap", values: None },
    52526u16 => TagDef { name: "SemanticName", values: None },
    52528u16 => TagDef { name: "SemanticInstanceID", values: None },
    52529u16 => TagDef { name: "CalibrationIlluminant3", values: Some(EXIF_MAIN_CALIBRATIONILLUMINANT3_VALUES) },
    52530u16 => TagDef { name: "CameraCalibration3", values: None },
    52531u16 => TagDef { name: "ColorMatrix3", values: None },
    52532u16 => TagDef { name: "ForwardMatrix3", values: None },
    52533u16 => TagDef { name: "IlluminantData1", values: None },
    52534u16 => TagDef { name: "IlluminantData2", values: None },
    52535u16 => TagDef { name: "IlluminantData3", values: None },
    52536u16 => TagDef { name: "MaskSubArea", values: None },
    52537u16 => TagDef { name: "ProfileHueSatMapData3", values: None },
    52538u16 => TagDef { name: "ReductionMatrix3", values: None },
    52543u16 => TagDef { name: "RGBTables", values: None },
    52544u16 => TagDef { name: "ProfileGainTableMap2", values: None },
    52545u16 => TagDef { name: "JUMBF", values: None },
    52547u16 => TagDef { name: "ColumnInterleaveFactor", values: None },
    52548u16 => TagDef { name: "ImageSequenceInfo", values: None },
    52550u16 => TagDef { name: "ImageStats", values: None },
    52551u16 => TagDef { name: "ProfileDynamicRange", values: None },
    52552u16 => TagDef { name: "ProfileGroupName", values: None },
    52553u16 => TagDef { name: "JXLDistance", values: None },
    52554u16 => TagDef { name: "JXLEffort", values: None },
    52555u16 => TagDef { name: "JXLDecodeSpeed", values: None },
    52897u16 => TagDef { name: "SEAL", values: None },
    529u16 => TagDef { name: "YCbCrCoefficients", values: None },
    530u16 => TagDef { name: "YCbCrSubSampling", values: None },
    531u16 => TagDef { name: "YCbCrPositioning", values: Some(EXIF_MAIN_YCBCRPOSITIONING_VALUES) },
    532u16 => TagDef { name: "ReferenceBlackWhite", values: None },
    59932u16 => TagDef { name: "Padding", values: None },
    59933u16 => TagDef { name: "OffsetSchema", values: None },
    65000u16 => TagDef { name: "OwnerName", values: None },
    65001u16 => TagDef { name: "SerialNumber", values: None },
    65002u16 => TagDef { name: "Lens", values: None },
    65024u16 => TagDef { name: "KDC_IFD", values: None },
    65100u16 => TagDef { name: "RawFile", values: None },
    65101u16 => TagDef { name: "Converter", values: None },
    65102u16 => TagDef { name: "WhiteBalance", values: None },
    65105u16 => TagDef { name: "Exposure", values: None },
    65106u16 => TagDef { name: "Shadows", values: None },
    65107u16 => TagDef { name: "Brightness", values: None },
    65108u16 => TagDef { name: "Contrast", values: None },
    65109u16 => TagDef { name: "Saturation", values: None },
    65110u16 => TagDef { name: "Sharpness", values: None },
    65111u16 => TagDef { name: "Smoothness", values: None },
    65112u16 => TagDef { name: "MoireFilter", values: None },
    700u16 => TagDef { name: "ApplicationNotes", values: None },
    771u16 => TagDef { name: "RenderingIntent", values: Some(EXIF_MAIN_RENDERINGINTENT_VALUES) },
};

pub static EXIF_MAIN_SUBFILETYPE_VALUES: &[(i64, &str)] = &[
    (0, "Full-resolution image"),
    (1, "Reduced-resolution image"),
    (16, "Enhanced image data"),
    (2, "Single page of multi-page image"),
    (3, "Single page of multi-page reduced-resolution image"),
    (4, "Transparency mask"),
    (4294967295, "invalid"),
    (5, "Transparency mask of reduced-resolution image"),
    (6, "Transparency mask of multi-page image"),
    (65537, "Alternate reduced-resolution image"),
    (65540, "Semantic Mask"),
    (7, "Transparency mask of reduced-resolution multi-page image"),
    (8, "Depth map"),
    (9, "Depth map of reduced-resolution image"),
];

pub static EXIF_MAIN_OLDSUBFILETYPE_VALUES: &[(i64, &str)] = &[
    (1, "Full-resolution image"),
    (2, "Reduced-resolution image"),
    (3, "Single page of multi-page image"),
];

pub static EXIF_MAIN_COMPRESSION_VALUES: &[(i64, &str)] = &[
    (1, "Uncompressed"),
    (10, "JBIG Color"),
    (2, "CCITT 1D"),
    (262, "Kodak 262"),
    (3, "T4/Group 3 Fax"),
    (32766, "Next"),
    (32767, "Sony ARW Compressed"),
    (32769, "Packed RAW"),
    (32770, "Samsung SRW Compressed"),
    (32771, "CCIRLEW"),
    (32772, "Samsung SRW Compressed 2"),
    (32773, "PackBits"),
    (32809, "Thunderscan"),
    (32867, "Kodak KDC Compressed"),
    (32895, "IT8CTPAD"),
    (32896, "IT8LW"),
    (32897, "IT8MP"),
    (32898, "IT8BL"),
    (32908, "PixarFilm"),
    (32909, "PixarLog"),
    (32946, "Deflate"),
    (32947, "DCS"),
    (33003, "Aperio JPEG 2000 YCbCr"),
    (33005, "Aperio JPEG 2000 RGB"),
    (34661, "JBIG"),
    (34676, "SGILog"),
    (34677, "SGILog24"),
    (34712, "JPEG 2000"),
    (34713, "Nikon NEF Compressed"),
    (34715, "JBIG2 TIFF FX"),
    (34718, "Microsoft Document Imaging (MDI) Binary Level Codec"),
    (34719, "Microsoft Document Imaging (MDI) Progressive Transform Codec"),
    (34720, "Microsoft Document Imaging (MDI) Vector"),
    (34887, "ESRI Lerc"),
    (34892, "Lossy JPEG"),
    (34925, "LZMA2"),
    (34926, "Zstd (old)"),
    (34927, "WebP (old)"),
    (34933, "PNG"),
    (34934, "JPEG XR"),
    (4, "T6/Group 4 Fax"),
    (5, "LZW"),
    (50000, "Zstd"),
    (50001, "WebP"),
    (50002, "JPEG XL (old)"),
    (52546, "JPEG XL"),
    (6, "JPEG (old-style)"),
    (65000, "Kodak DCR Compressed"),
    (65535, "Pentax PEF Compressed"),
    (7, "JPEG"),
    (8, "Adobe Deflate"),
    (9, "JBIG B&W"),
    (99, "JPEG"),
];

pub static EXIF_MAIN_PHOTOMETRICINTERPRETATION_VALUES: &[(i64, &str)] = &[
    (0, "WhiteIsZero"),
    (1, "BlackIsZero"),
    (10, "ITULab"),
    (2, "RGB"),
    (3, "RGB Palette"),
    (32803, "Color Filter Array"),
    (32844, "Pixar LogL"),
    (32845, "Pixar LogLuv"),
    (32892, "Sequential Color Filter"),
    (34892, "Linear Raw"),
    (4, "Transparency Mask"),
    (5, "CMYK"),
    (51177, "Depth Map"),
    (52527, "Semantic Mask"),
    (6, "YCbCr"),
    (8, "CIELab"),
    (9, "ICCLab"),
];

pub static EXIF_MAIN_THRESHOLDING_VALUES: &[(i64, &str)] = &[
    (1, "No dithering or halftoning"),
    (2, "Ordered dither or halftone"),
    (3, "Randomized dither"),
];

pub static EXIF_MAIN_FILLORDER_VALUES: &[(i64, &str)] = &[
    (1, "Normal"),
    (2, "Reversed"),
];

pub static EXIF_MAIN_ORIENTATION_VALUES: &[(i64, &str)] = &[
    (1, "Horizontal (normal)"),
    (2, "Mirror horizontal"),
    (3, "Rotate 180"),
    (4, "Mirror vertical"),
    (5, "Mirror horizontal and rotate 270 CW"),
    (6, "Rotate 90 CW"),
    (7, "Mirror horizontal and rotate 90 CW"),
    (8, "Rotate 270 CW"),
];

pub static EXIF_MAIN_PLANARCONFIGURATION_VALUES: &[(i64, &str)] = &[
    (1, "Chunky"),
    (2, "Planar"),
];

pub static EXIF_MAIN_SONYRAWFILETYPE_VALUES: &[(i64, &str)] = &[
    (0, "Sony Uncompressed 14-bit RAW"),
    (1, "Sony Uncompressed 12-bit RAW"),
    (2, "Sony Compressed RAW"),
    (3, "Sony Lossless Compressed RAW"),
    (4, "Sony Lossless Compressed RAW 2"),
    (6, "Sony Compressed RAW HQ"),
];

pub static EXIF_MAIN_VIGNETTINGCORRECTION_VALUES: &[(i64, &str)] = &[
    (256, "Off"),
    (257, "Auto"),
    (272, "Auto (ILCE-1)"),
    (511, "No correction params available"),
];

pub static EXIF_MAIN_CHROMATICABERRATIONCORRECTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Auto"),
    (255, "No correction params available"),
];

pub static EXIF_MAIN_DISTORTIONCORRECTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Auto"),
    (17, "Auto fixed by lens"),
    (255, "No correction params available"),
];

pub static EXIF_MAIN_GRAYRESPONSEUNIT_VALUES: &[(i64, &str)] = &[
    (1, "0.1"),
    (2, "0.001"),
    (3, "0.0001"),
    (4, "1e-05"),
    (5, "1e-06"),
];

pub static EXIF_MAIN_RESOLUTIONUNIT_VALUES: &[(i64, &str)] = &[
    (1, "None"),
    (2, "inches"),
    (3, "cm"),
];

pub static EXIF_MAIN_PREDICTOR_VALUES: &[(i64, &str)] = &[
    (1, "None"),
    (2, "Horizontal differencing"),
    (3, "Floating point"),
    (34892, "Horizontal difference X2"),
    (34893, "Horizontal difference X4"),
    (34894, "Floating point X2"),
    (34895, "Floating point X4"),
];

pub static EXIF_MAIN_CLEANFAXDATA_VALUES: &[(i64, &str)] = &[
    (0, "Clean"),
    (1, "Regenerated"),
    (2, "Unclean"),
];

pub static EXIF_MAIN_INKSET_VALUES: &[(i64, &str)] = &[
    (1, "CMYK"),
    (2, "Not CMYK"),
];

pub static EXIF_MAIN_EXTRASAMPLES_VALUES: &[(i64, &str)] = &[
    (0, "Unspecified"),
    (1, "Associated Alpha"),
    (2, "Unassociated Alpha"),
];

pub static EXIF_MAIN_RASTERPADDING_VALUES: &[(i64, &str)] = &[
    (0, "Byte"),
    (1, "Word"),
    (10, "Long Sector"),
    (2, "Long Word"),
    (9, "Sector"),
];

pub static EXIF_MAIN_IMAGECOLORINDICATOR_VALUES: &[(i64, &str)] = &[
    (0, "Unspecified Image Color"),
    (1, "Specified Image Color"),
];

pub static EXIF_MAIN_BACKGROUNDCOLORINDICATOR_VALUES: &[(i64, &str)] = &[
    (0, "Unspecified Background Color"),
    (1, "Specified Background Color"),
];

pub static EXIF_MAIN_HCUSAGE_VALUES: &[(i64, &str)] = &[
    (0, "CT"),
    (1, "Line Art"),
    (2, "Trap"),
];

pub static EXIF_MAIN_INDEXED_VALUES: &[(i64, &str)] = &[
    (0, "Not indexed"),
    (1, "Indexed"),
];

pub static EXIF_MAIN_EXPOSUREPROGRAM_VALUES: &[(i64, &str)] = &[
    (0, "Not Defined"),
    (1, "Manual"),
    (2, "Program AE"),
    (3, "Aperture-priority AE"),
    (4, "Shutter speed priority AE"),
    (5, "Creative (Slow speed)"),
    (6, "Action (High speed)"),
    (7, "Portrait"),
    (8, "Landscape"),
    (9, "Bulb"),
];

pub static EXIF_MAIN_SENSITIVITYTYPE_VALUES: &[(i64, &str)] = &[
    (0, "Unknown"),
    (1, "Standard Output Sensitivity"),
    (2, "Recommended Exposure Index"),
    (3, "ISO Speed"),
    (4, "Standard Output Sensitivity and Recommended Exposure Index"),
    (5, "Standard Output Sensitivity and ISO Speed"),
    (6, "Recommended Exposure Index and ISO Speed"),
    (7, "Standard Output Sensitivity, Recommended Exposure Index and ISO Speed"),
];

pub static EXIF_MAIN_OPIPROXY_VALUES: &[(i64, &str)] = &[
    (0, "Higher resolution image does not exist"),
    (1, "Higher resolution image exists"),
];

pub static EXIF_MAIN_COMPONENTSCONFIGURATION_VALUES: &[(i64, &str)] = &[
    (0, "-"),
    (1, "Y"),
    (2, "Cb"),
    (3, "Cr"),
    (4, "R"),
    (5, "G"),
    (6, "B"),
];

pub static EXIF_MAIN_METERINGMODE_VALUES: &[(i64, &str)] = &[
    (0, "Unknown"),
    (1, "Average"),
    (2, "Center-weighted average"),
    (255, "Other"),
    (3, "Spot"),
    (4, "Multi-spot"),
    (5, "Multi-segment"),
    (6, "Partial"),
];

pub static EXIF_MAIN_LIGHTSOURCE_VALUES: &[(i64, &str)] = &[
    (0, "Unknown"),
    (1, "Daylight"),
    (10, "Cloudy"),
    (11, "Shade"),
    (12, "Daylight Fluorescent"),
    (13, "Day White Fluorescent"),
    (14, "Cool White Fluorescent"),
    (15, "White Fluorescent"),
    (16, "Warm White Fluorescent"),
    (17, "Standard Light A"),
    (18, "Standard Light B"),
    (19, "Standard Light C"),
    (2, "Fluorescent"),
    (20, "D55"),
    (21, "D65"),
    (22, "D75"),
    (23, "D50"),
    (24, "ISO Studio Tungsten"),
    (255, "Other"),
    (3, "Tungsten (Incandescent)"),
    (4, "Flash"),
    (9, "Fine Weather"),
];

pub static EXIF_MAIN_FLASH_VALUES: &[(i64, &str)] = &[
    (0, "No Flash"),
    (1, "Fired"),
    (13, "On, Return not detected"),
    (15, "On, Return detected"),
    (16, "Off, Did not fire"),
    (20, "Off, Did not fire, Return not detected"),
    (24, "Auto, Did not fire"),
    (25, "Auto, Fired"),
    (29, "Auto, Fired, Return not detected"),
    (31, "Auto, Fired, Return detected"),
    (32, "No flash function"),
    (48, "Off, No flash function"),
    (5, "Fired, Return not detected"),
    (65, "Fired, Red-eye reduction"),
    (69, "Fired, Red-eye reduction, Return not detected"),
    (7, "Fired, Return detected"),
    (71, "Fired, Red-eye reduction, Return detected"),
    (73, "On, Red-eye reduction"),
    (77, "On, Red-eye reduction, Return not detected"),
    (79, "On, Red-eye reduction, Return detected"),
    (8, "On, Did not fire"),
    (80, "Off, Red-eye reduction"),
    (88, "Auto, Did not fire, Red-eye reduction"),
    (89, "Auto, Fired, Red-eye reduction"),
    (9, "On, Fired"),
    (93, "Auto, Fired, Red-eye reduction, Return not detected"),
    (95, "Auto, Fired, Red-eye reduction, Return detected"),
];

pub static EXIF_MAIN_FOCALPLANERESOLUTIONUNIT_VALUES: &[(i64, &str)] = &[
    (1, "None"),
    (2, "inches"),
    (3, "cm"),
    (4, "mm"),
    (5, "um"),
];

pub static EXIF_MAIN_SENSINGMETHOD_VALUES: &[(i64, &str)] = &[
    (1, "Monochrome area"),
    (2, "One-chip color area"),
    (3, "Two-chip color area"),
    (4, "Three-chip color area"),
    (5, "Color sequential area"),
    (6, "Monochrome linear"),
    (7, "Trilinear"),
    (8, "Color sequential linear"),
];

pub static EXIF_MAIN_PROFILETYPE_VALUES: &[(i64, &str)] = &[
    (0, "Unspecified"),
    (1, "Group 3 FAX"),
];

pub static EXIF_MAIN_FAXPROFILE_VALUES: &[(i64, &str)] = &[
    (0, "Unknown"),
    (1, "Minimal B&W lossless, S"),
    (2, "Extended B&W lossless, F"),
    (255, "Multi Profiles"),
    (3, "Lossless JBIG B&W, J"),
    (4, "Lossy color and grayscale, C"),
    (5, "Lossless color and grayscale, L"),
    (6, "Mixed raster content, M"),
    (7, "Profile T"),
];

pub static EXIF_MAIN_COLORSPACE_VALUES: &[(i64, &str)] = &[
    (1, "sRGB"),
    (2, "Adobe RGB"),
    (65533, "Wide Gamut RGB"),
    (65534, "ICC Profile"),
    (65535, "Uncalibrated"),
];

pub static EXIF_MAIN_FILESOURCE_VALUES: &[(i64, &str)] = &[
    (1, "Film Scanner"),
    (2, "Reflection Print Scanner"),
    (3, "Digital Camera"),
];

pub static EXIF_MAIN_SCENETYPE_VALUES: &[(i64, &str)] = &[
    (1, "Directly photographed"),
];

pub static EXIF_MAIN_CUSTOMRENDERED_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Custom"),
    (2, "HDR (no original saved)"),
    (3, "HDR (original saved)"),
    (4, "Original (for HDR)"),
    (6, "Panorama"),
    (7, "Portrait HDR"),
    (8, "Portrait"),
];

pub static EXIF_MAIN_EXPOSUREMODE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Manual"),
    (2, "Auto bracket"),
];

pub static EXIF_MAIN_WHITEBALANCE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Manual"),
];

pub static EXIF_MAIN_SCENECAPTURETYPE_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (1, "Landscape"),
    (2, "Portrait"),
    (3, "Night"),
    (4, "Other"),
];

pub static EXIF_MAIN_GAINCONTROL_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "Low gain up"),
    (2, "High gain up"),
    (3, "Low gain down"),
    (4, "High gain down"),
];

pub static EXIF_MAIN_CONTRAST_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Low"),
    (2, "High"),
];

pub static EXIF_MAIN_SATURATION_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Low"),
    (2, "High"),
];

pub static EXIF_MAIN_SHARPNESS_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Soft"),
    (2, "Hard"),
];

pub static EXIF_MAIN_SUBJECTDISTANCERANGE_VALUES: &[(i64, &str)] = &[
    (0, "Unknown"),
    (1, "Macro"),
    (2, "Close"),
    (3, "Distant"),
];

pub static EXIF_MAIN_COMPOSITEIMAGE_VALUES: &[(i64, &str)] = &[
    (0, "Unknown"),
    (1, "Not a Composite Image"),
    (2, "General Composite Image"),
    (3, "Composite Image Captured While Shooting"),
];

pub static EXIF_MAIN_PIXELFORMAT_VALUES: &[(i64, &str)] = &[
    (10, "16-bit BGR565"),
    (11, "16-bit Gray"),
    (12, "24-bit BGR"),
    (13, "24-bit RGB"),
    (14, "32-bit BGR"),
    (15, "32-bit BGRA"),
    (16, "32-bit PBGRA"),
    (17, "32-bit Gray Float"),
    (18, "48-bit RGB Fixed Point"),
    (19, "32-bit BGR101010"),
    (21, "48-bit RGB"),
    (22, "64-bit RGBA"),
    (23, "64-bit PRGBA"),
    (24, "96-bit RGB Fixed Point"),
    (25, "128-bit RGBA Float"),
    (26, "128-bit PRGBA Float"),
    (27, "128-bit RGB Float"),
    (28, "32-bit CMYK"),
    (29, "64-bit RGBA Fixed Point"),
    (30, "128-bit RGBA Fixed Point"),
    (31, "64-bit CMYK"),
    (32, "24-bit 3 Channels"),
    (33, "32-bit 4 Channels"),
    (34, "40-bit 5 Channels"),
    (35, "48-bit 6 Channels"),
    (36, "56-bit 7 Channels"),
    (37, "64-bit 8 Channels"),
    (38, "48-bit 3 Channels"),
    (39, "64-bit 4 Channels"),
    (40, "80-bit 5 Channels"),
    (41, "96-bit 6 Channels"),
    (42, "112-bit 7 Channels"),
    (43, "128-bit 8 Channels"),
    (44, "40-bit CMYK Alpha"),
    (45, "80-bit CMYK Alpha"),
    (46, "32-bit 3 Channels Alpha"),
    (47, "40-bit 4 Channels Alpha"),
    (48, "48-bit 5 Channels Alpha"),
    (49, "56-bit 6 Channels Alpha"),
    (5, "Black & White"),
    (50, "64-bit 7 Channels Alpha"),
    (51, "72-bit 8 Channels Alpha"),
    (52, "64-bit 3 Channels Alpha"),
    (53, "80-bit 4 Channels Alpha"),
    (54, "96-bit 5 Channels Alpha"),
    (55, "112-bit 6 Channels Alpha"),
    (56, "128-bit 7 Channels Alpha"),
    (57, "144-bit 8 Channels Alpha"),
    (58, "64-bit RGBA Half"),
    (59, "48-bit RGB Half"),
    (61, "32-bit RGBE"),
    (62, "16-bit Gray Half"),
    (63, "32-bit Gray Fixed Point"),
    (8, "8-bit Gray"),
    (9, "16-bit BGR555"),
];

pub static EXIF_MAIN_TRANSFORMATION_VALUES: &[(i64, &str)] = &[
    (0, "Horizontal (normal)"),
    (1, "Mirror vertical"),
    (2, "Mirror horizontal"),
    (3, "Rotate 180"),
    (4, "Rotate 90 CW"),
    (5, "Mirror horizontal and rotate 90 CW"),
    (6, "Mirror horizontal and rotate 270 CW"),
    (7, "Rotate 270 CW"),
];

pub static EXIF_MAIN_UNCOMPRESSED_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];

pub static EXIF_MAIN_IMAGEDATADISCARD_VALUES: &[(i64, &str)] = &[
    (0, "Full Resolution"),
    (1, "Flexbits Discarded"),
    (2, "HighPass Frequency Data Discarded"),
    (3, "Highpass and LowPass Frequency Data Discarded"),
];

pub static EXIF_MAIN_ALPHADATADISCARD_VALUES: &[(i64, &str)] = &[
    (0, "Full Resolution"),
    (1, "Flexbits Discarded"),
    (2, "HighPass Frequency Data Discarded"),
    (3, "Highpass and LowPass Frequency Data Discarded"),
];

pub static EXIF_MAIN_USPTOORIGINALCONTENTTYPE_VALUES: &[(i64, &str)] = &[
    (0, "Text or Drawing"),
    (1, "Grayscale"),
    (2, "Color"),
];

pub static EXIF_MAIN_CFALAYOUT_VALUES: &[(i64, &str)] = &[
    (1, "Rectangular"),
    (2, "Even columns offset down 1/2 row"),
    (3, "Even columns offset up 1/2 row"),
    (4, "Even rows offset right 1/2 column"),
    (5, "Even rows offset left 1/2 column"),
    (6, "Even rows offset up by 1/2 row, even columns offset left by 1/2 column"),
    (7, "Even rows offset up by 1/2 row, even columns offset right by 1/2 column"),
    (8, "Even rows offset down by 1/2 row, even columns offset left by 1/2 column"),
    (9, "Even rows offset down by 1/2 row, even columns offset right by 1/2 column"),
];

pub static EXIF_MAIN_MAKERNOTESAFETY_VALUES: &[(i64, &str)] = &[
    (0, "Unsafe"),
    (1, "Safe"),
];

pub static EXIF_MAIN_CALIBRATIONILLUMINANT1_VALUES: &[(i64, &str)] = &[
    (0, "Unknown"),
    (1, "Daylight"),
    (10, "Cloudy"),
    (11, "Shade"),
    (12, "Daylight Fluorescent"),
    (13, "Day White Fluorescent"),
    (14, "Cool White Fluorescent"),
    (15, "White Fluorescent"),
    (16, "Warm White Fluorescent"),
    (17, "Standard Light A"),
    (18, "Standard Light B"),
    (19, "Standard Light C"),
    (2, "Fluorescent"),
    (20, "D55"),
    (21, "D65"),
    (22, "D75"),
    (23, "D50"),
    (24, "ISO Studio Tungsten"),
    (255, "Other"),
    (3, "Tungsten (Incandescent)"),
    (4, "Flash"),
    (9, "Fine Weather"),
];

pub static EXIF_MAIN_CALIBRATIONILLUMINANT2_VALUES: &[(i64, &str)] = &[
    (0, "Unknown"),
    (1, "Daylight"),
    (10, "Cloudy"),
    (11, "Shade"),
    (12, "Daylight Fluorescent"),
    (13, "Day White Fluorescent"),
    (14, "Cool White Fluorescent"),
    (15, "White Fluorescent"),
    (16, "Warm White Fluorescent"),
    (17, "Standard Light A"),
    (18, "Standard Light B"),
    (19, "Standard Light C"),
    (2, "Fluorescent"),
    (20, "D55"),
    (21, "D65"),
    (22, "D75"),
    (23, "D50"),
    (24, "ISO Studio Tungsten"),
    (255, "Other"),
    (3, "Tungsten (Incandescent)"),
    (4, "Flash"),
    (9, "Fine Weather"),
];

pub static EXIF_MAIN_COLORIMETRICREFERENCE_VALUES: &[(i64, &str)] = &[
    (0, "Scene-referred"),
    (1, "Output-referred (ICC Profile Dynamic Range)"),
    (2, "Output-referred (High Dyanmic Range)"),
];

pub static EXIF_MAIN_PROFILEEMBEDPOLICY_VALUES: &[(i64, &str)] = &[
    (0, "Allow Copying"),
    (1, "Embed if Used"),
    (2, "Never Embed"),
    (3, "No Restrictions"),
];

pub static EXIF_MAIN_PREVIEWCOLORSPACE_VALUES: &[(i64, &str)] = &[
    (0, "Unknown"),
    (1, "Gray Gamma 2.2"),
    (2, "sRGB"),
    (3, "Adobe RGB"),
    (4, "ProPhoto RGB"),
];

pub static EXIF_MAIN_OPCODELIST1_VALUES: &[(i64, &str)] = &[
    (1, "WarpRectilinear"),
    (10, "DeltaPerRow"),
    (11, "DeltaPerColumn"),
    (12, "ScalePerRow"),
    (13, "ScalePerColumn"),
    (14, "WarpRectilinear2"),
    (2, "WarpFisheye"),
    (3, "FixVignetteRadial"),
    (4, "FixBadPixelsConstant"),
    (5, "FixBadPixelsList"),
    (6, "TrimBounds"),
    (7, "MapTable"),
    (8, "MapPolynomial"),
    (9, "GainMap"),
];

pub static EXIF_MAIN_OPCODELIST2_VALUES: &[(i64, &str)] = &[
    (1, "WarpRectilinear"),
    (10, "DeltaPerRow"),
    (11, "DeltaPerColumn"),
    (12, "ScalePerRow"),
    (13, "ScalePerColumn"),
    (14, "WarpRectilinear2"),
    (2, "WarpFisheye"),
    (3, "FixVignetteRadial"),
    (4, "FixBadPixelsConstant"),
    (5, "FixBadPixelsList"),
    (6, "TrimBounds"),
    (7, "MapTable"),
    (8, "MapPolynomial"),
    (9, "GainMap"),
];

pub static EXIF_MAIN_OPCODELIST3_VALUES: &[(i64, &str)] = &[
    (1, "WarpRectilinear"),
    (10, "DeltaPerRow"),
    (11, "DeltaPerColumn"),
    (12, "ScalePerRow"),
    (13, "ScalePerColumn"),
    (14, "WarpRectilinear2"),
    (2, "WarpFisheye"),
    (3, "FixVignetteRadial"),
    (4, "FixBadPixelsConstant"),
    (5, "FixBadPixelsList"),
    (6, "TrimBounds"),
    (7, "MapTable"),
    (8, "MapPolynomial"),
    (9, "GainMap"),
];

pub static EXIF_MAIN_PROFILEHUESATMAPENCODING_VALUES: &[(i64, &str)] = &[
    (0, "Linear"),
    (1, "sRGB"),
];

pub static EXIF_MAIN_PROFILELOOKTABLEENCODING_VALUES: &[(i64, &str)] = &[
    (0, "Linear"),
    (1, "sRGB"),
];

pub static EXIF_MAIN_DEFAULTBLACKRENDER_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "None"),
];

pub static EXIF_MAIN_DEPTHFORMAT_VALUES: &[(i64, &str)] = &[
    (0, "Unknown"),
    (1, "Linear"),
    (2, "Inverse"),
];

pub static EXIF_MAIN_DEPTHUNITS_VALUES: &[(i64, &str)] = &[
    (0, "Unknown"),
    (1, "Meters"),
];

pub static EXIF_MAIN_DEPTHMEASURETYPE_VALUES: &[(i64, &str)] = &[
    (0, "Unknown"),
    (1, "Optical Axis"),
    (2, "Optical Ray"),
];

pub static EXIF_MAIN_JPEGPROC_VALUES: &[(i64, &str)] = &[
    (1, "Baseline"),
    (14, "Lossless"),
];

pub static EXIF_MAIN_CALIBRATIONILLUMINANT3_VALUES: &[(i64, &str)] = &[
    (0, "Unknown"),
    (1, "Daylight"),
    (10, "Cloudy"),
    (11, "Shade"),
    (12, "Daylight Fluorescent"),
    (13, "Day White Fluorescent"),
    (14, "Cool White Fluorescent"),
    (15, "White Fluorescent"),
    (16, "Warm White Fluorescent"),
    (17, "Standard Light A"),
    (18, "Standard Light B"),
    (19, "Standard Light C"),
    (2, "Fluorescent"),
    (20, "D55"),
    (21, "D65"),
    (22, "D75"),
    (23, "D50"),
    (24, "ISO Studio Tungsten"),
    (255, "Other"),
    (3, "Tungsten (Incandescent)"),
    (4, "Flash"),
    (9, "Fine Weather"),
];

pub static EXIF_MAIN_YCBCRPOSITIONING_VALUES: &[(i64, &str)] = &[
    (1, "Centered"),
    (2, "Co-sited"),
];

pub static EXIF_MAIN_RENDERINGINTENT_VALUES: &[(i64, &str)] = &[
    (0, "Perceptual"),
    (1, "Relative Colorimetric"),
    (2, "Saturation"),
    (3, "Absolute colorimetric"),
];

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

/// IPTC::ApplicationRecord tags
pub static IPTC_APPLICATIONRECORD: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "ApplicationRecordVersion", values: None },
    10u16 => TagDef { name: "Urgency", values: Some(IPTC_APPLICATIONRECORD_URGENCY_VALUES) },
    100u16 => TagDef { name: "Country-PrimaryLocationCode", values: None },
    101u16 => TagDef { name: "Country-PrimaryLocationName", values: None },
    103u16 => TagDef { name: "OriginalTransmissionReference", values: None },
    105u16 => TagDef { name: "Headline", values: None },
    110u16 => TagDef { name: "Credit", values: None },
    115u16 => TagDef { name: "Source", values: None },
    116u16 => TagDef { name: "CopyrightNotice", values: None },
    118u16 => TagDef { name: "Contact", values: None },
    12u16 => TagDef { name: "SubjectReference", values: None },
    120u16 => TagDef { name: "Caption-Abstract", values: None },
    121u16 => TagDef { name: "LocalCaption", values: None },
    122u16 => TagDef { name: "Writer-Editor", values: None },
    125u16 => TagDef { name: "RasterizedCaption", values: None },
    130u16 => TagDef { name: "ImageType", values: None },
    131u16 => TagDef { name: "ImageOrientation", values: None },
    135u16 => TagDef { name: "LanguageIdentifier", values: None },
    15u16 => TagDef { name: "Category", values: None },
    150u16 => TagDef { name: "AudioType", values: None },
    151u16 => TagDef { name: "AudioSamplingRate", values: None },
    152u16 => TagDef { name: "AudioSamplingResolution", values: None },
    153u16 => TagDef { name: "AudioDuration", values: None },
    154u16 => TagDef { name: "AudioOutcue", values: None },
    184u16 => TagDef { name: "JobID", values: None },
    185u16 => TagDef { name: "MasterDocumentID", values: None },
    186u16 => TagDef { name: "ShortDocumentID", values: None },
    187u16 => TagDef { name: "UniqueDocumentID", values: None },
    188u16 => TagDef { name: "OwnerID", values: None },
    20u16 => TagDef { name: "SupplementalCategories", values: None },
    200u16 => TagDef { name: "ObjectPreviewFileFormat", values: Some(IPTC_APPLICATIONRECORD_OBJECTPREVIEWFILEFORMAT_VALUES) },
    201u16 => TagDef { name: "ObjectPreviewFileVersion", values: None },
    202u16 => TagDef { name: "ObjectPreviewData", values: None },
    22u16 => TagDef { name: "FixtureIdentifier", values: None },
    221u16 => TagDef { name: "Prefs", values: None },
    225u16 => TagDef { name: "ClassifyState", values: None },
    228u16 => TagDef { name: "SimilarityIndex", values: None },
    230u16 => TagDef { name: "DocumentNotes", values: None },
    231u16 => TagDef { name: "DocumentHistory", values: None },
    232u16 => TagDef { name: "ExifCameraInfo", values: None },
    25u16 => TagDef { name: "Keywords", values: None },
    255u16 => TagDef { name: "CatalogSets", values: None },
    26u16 => TagDef { name: "ContentLocationCode", values: None },
    27u16 => TagDef { name: "ContentLocationName", values: None },
    3u16 => TagDef { name: "ObjectTypeReference", values: None },
    30u16 => TagDef { name: "ReleaseDate", values: None },
    35u16 => TagDef { name: "ReleaseTime", values: None },
    37u16 => TagDef { name: "ExpirationDate", values: None },
    38u16 => TagDef { name: "ExpirationTime", values: None },
    4u16 => TagDef { name: "ObjectAttributeReference", values: None },
    40u16 => TagDef { name: "SpecialInstructions", values: None },
    42u16 => TagDef { name: "ActionAdvised", values: Some(IPTC_APPLICATIONRECORD_ACTIONADVISED_VALUES) },
    45u16 => TagDef { name: "ReferenceService", values: None },
    47u16 => TagDef { name: "ReferenceDate", values: None },
    5u16 => TagDef { name: "ObjectName", values: None },
    50u16 => TagDef { name: "ReferenceNumber", values: None },
    55u16 => TagDef { name: "DateCreated", values: None },
    60u16 => TagDef { name: "TimeCreated", values: None },
    62u16 => TagDef { name: "DigitalCreationDate", values: None },
    63u16 => TagDef { name: "DigitalCreationTime", values: None },
    65u16 => TagDef { name: "OriginatingProgram", values: None },
    7u16 => TagDef { name: "EditStatus", values: None },
    70u16 => TagDef { name: "ProgramVersion", values: None },
    75u16 => TagDef { name: "ObjectCycle", values: None },
    8u16 => TagDef { name: "EditorialUpdate", values: Some(IPTC_APPLICATIONRECORD_EDITORIALUPDATE_VALUES) },
    80u16 => TagDef { name: "By-line", values: None },
    85u16 => TagDef { name: "By-lineTitle", values: None },
    90u16 => TagDef { name: "City", values: None },
    92u16 => TagDef { name: "Sub-location", values: None },
    95u16 => TagDef { name: "Province-State", values: None },
};

pub static IPTC_APPLICATIONRECORD_URGENCY_VALUES: &[(i64, &str)] = &[
    (0, "0 (reserved)"),
    (1, "1 (most urgent)"),
    (2, "2"),
    (3, "3"),
    (4, "4"),
    (5, "5 (normal urgency)"),
    (6, "6"),
    (7, "7"),
    (8, "8 (least urgent)"),
    (9, "9 (user-defined priority)"),
];

pub static IPTC_APPLICATIONRECORD_OBJECTPREVIEWFILEFORMAT_VALUES: &[(i64, &str)] = &[
    (0, "No ObjectData"),
    (1, "IPTC-NAA Digital Newsphoto Parameter Record"),
    (10, "United Press International Down-Load Message"),
    (11, "JPEG File Interchange (JFIF)"),
    (12, "Photo-CD Image-Pac (Eastman Kodak)"),
    (13, "Bit Mapped Graphics File [.BMP] (Microsoft)"),
    (14, "Digital Audio File [.WAV] (Microsoft & Creative Labs)"),
    (15, "Audio plus Moving Video [.AVI] (Microsoft)"),
    (16, "PC DOS/Windows Executable Files [.COM][.EXE]"),
    (17, "Compressed Binary File [.ZIP] (PKWare Inc)"),
    (18, "Audio Interchange File Format AIFF (Apple Computer Inc)"),
    (19, "RIFF Wave (Microsoft Corporation)"),
    (2, "IPTC7901 Recommended Message Format"),
    (20, "Freehand (Macromedia/Aldus)"),
    (21, "Hypertext Markup Language [.HTML] (The Internet Society)"),
    (22, "MPEG 2 Audio Layer 2 (Musicom), ISO/IEC"),
    (23, "MPEG 2 Audio Layer 3, ISO/IEC"),
    (24, "Portable Document File [.PDF] Adobe"),
    (25, "News Industry Text Format (NITF)"),
    (26, "Tape Archive [.TAR]"),
    (27, "Tidningarnas Telegrambyra NITF version (TTNITF DTD)"),
    (28, "Ritzaus Bureau NITF version (RBNITF DTD)"),
    (29, "Corel Draw [.CDR]"),
    (3, "Tagged Image File Format (Adobe/Aldus Image data)"),
    (4, "Illustrator (Adobe Graphics data)"),
    (5, "AppleSingle (Apple Computer Inc)"),
    (6, "NAA 89-3 (ANPA 1312)"),
    (7, "MacBinary II"),
    (8, "IPTC Unstructured Character Oriented File Format (UCOFF)"),
    (9, "United Press International ANPA 1312 variant"),
];

pub static IPTC_APPLICATIONRECORD_ACTIONADVISED_VALUES: &[(i64, &str)] = &[
    (1, "Object Kill"),
    (2, "Object Replace"),
    (3, "Object Append"),
    (4, "Object Reference"),
];

pub static IPTC_APPLICATIONRECORD_EDITORIALUPDATE_VALUES: &[(i64, &str)] = &[
    (1, "Additional language"),
];

/// IPTC::EnvelopeRecord tags
pub static IPTC_ENVELOPERECORD: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "EnvelopeRecordVersion", values: None },
    100u16 => TagDef { name: "UniqueObjectName", values: None },
    120u16 => TagDef { name: "ARMIdentifier", values: None },
    122u16 => TagDef { name: "ARMVersion", values: None },
    20u16 => TagDef { name: "FileFormat", values: Some(IPTC_ENVELOPERECORD_FILEFORMAT_VALUES) },
    22u16 => TagDef { name: "FileVersion", values: None },
    30u16 => TagDef { name: "ServiceIdentifier", values: None },
    40u16 => TagDef { name: "EnvelopeNumber", values: None },
    5u16 => TagDef { name: "Destination", values: None },
    50u16 => TagDef { name: "ProductID", values: None },
    60u16 => TagDef { name: "EnvelopePriority", values: Some(IPTC_ENVELOPERECORD_ENVELOPEPRIORITY_VALUES) },
    70u16 => TagDef { name: "DateSent", values: None },
    80u16 => TagDef { name: "TimeSent", values: None },
    90u16 => TagDef { name: "CodedCharacterSet", values: None },
};

pub static IPTC_ENVELOPERECORD_FILEFORMAT_VALUES: &[(i64, &str)] = &[
    (0, "No ObjectData"),
    (1, "IPTC-NAA Digital Newsphoto Parameter Record"),
    (10, "United Press International Down-Load Message"),
    (11, "JPEG File Interchange (JFIF)"),
    (12, "Photo-CD Image-Pac (Eastman Kodak)"),
    (13, "Bit Mapped Graphics File [.BMP] (Microsoft)"),
    (14, "Digital Audio File [.WAV] (Microsoft & Creative Labs)"),
    (15, "Audio plus Moving Video [.AVI] (Microsoft)"),
    (16, "PC DOS/Windows Executable Files [.COM][.EXE]"),
    (17, "Compressed Binary File [.ZIP] (PKWare Inc)"),
    (18, "Audio Interchange File Format AIFF (Apple Computer Inc)"),
    (19, "RIFF Wave (Microsoft Corporation)"),
    (2, "IPTC7901 Recommended Message Format"),
    (20, "Freehand (Macromedia/Aldus)"),
    (21, "Hypertext Markup Language [.HTML] (The Internet Society)"),
    (22, "MPEG 2 Audio Layer 2 (Musicom), ISO/IEC"),
    (23, "MPEG 2 Audio Layer 3, ISO/IEC"),
    (24, "Portable Document File [.PDF] Adobe"),
    (25, "News Industry Text Format (NITF)"),
    (26, "Tape Archive [.TAR]"),
    (27, "Tidningarnas Telegrambyra NITF version (TTNITF DTD)"),
    (28, "Ritzaus Bureau NITF version (RBNITF DTD)"),
    (29, "Corel Draw [.CDR]"),
    (3, "Tagged Image File Format (Adobe/Aldus Image data)"),
    (4, "Illustrator (Adobe Graphics data)"),
    (5, "AppleSingle (Apple Computer Inc)"),
    (6, "NAA 89-3 (ANPA 1312)"),
    (7, "MacBinary II"),
    (8, "IPTC Unstructured Character Oriented File Format (UCOFF)"),
    (9, "United Press International ANPA 1312 variant"),
];

pub static IPTC_ENVELOPERECORD_ENVELOPEPRIORITY_VALUES: &[(i64, &str)] = &[
    (0, "0 (reserved)"),
    (1, "1 (most urgent)"),
    (2, "2"),
    (3, "3"),
    (4, "4"),
    (5, "5 (normal urgency)"),
    (6, "6"),
    (7, "7"),
    (8, "8 (least urgent)"),
    (9, "9 (user-defined priority)"),
];

/// IPTC::Main tags
pub static IPTC_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "IPTCEnvelope", values: None },
    2u16 => TagDef { name: "IPTCApplication", values: None },
    240u16 => TagDef { name: "IPTCFotoStation", values: None },
    3u16 => TagDef { name: "IPTCNewsPhoto", values: None },
    7u16 => TagDef { name: "IPTCPreObjectData", values: None },
    8u16 => TagDef { name: "IPTCObjectData", values: None },
    9u16 => TagDef { name: "IPTCPostObjectData", values: None },
};

/// IPTC::NewsPhoto tags
pub static IPTC_NEWSPHOTO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "NewsPhotoVersion", values: None },
    10u16 => TagDef { name: "IPTCPictureNumber", values: None },
    100u16 => TagDef { name: "ScanningDirection", values: Some(IPTC_NEWSPHOTO_SCANNINGDIRECTION_VALUES) },
    102u16 => TagDef { name: "IPTCImageRotation", values: Some(IPTC_NEWSPHOTO_IPTCIMAGEROTATION_VALUES) },
    110u16 => TagDef { name: "DataCompressionMethod", values: None },
    120u16 => TagDef { name: "QuantizationMethod", values: Some(IPTC_NEWSPHOTO_QUANTIZATIONMETHOD_VALUES) },
    125u16 => TagDef { name: "EndPoints", values: None },
    130u16 => TagDef { name: "ExcursionTolerance", values: Some(IPTC_NEWSPHOTO_EXCURSIONTOLERANCE_VALUES) },
    135u16 => TagDef { name: "BitsPerComponent", values: None },
    140u16 => TagDef { name: "MaximumDensityRange", values: None },
    145u16 => TagDef { name: "GammaCompensatedValue", values: None },
    20u16 => TagDef { name: "IPTCImageWidth", values: None },
    30u16 => TagDef { name: "IPTCImageHeight", values: None },
    40u16 => TagDef { name: "IPTCPixelWidth", values: None },
    50u16 => TagDef { name: "IPTCPixelHeight", values: None },
    55u16 => TagDef { name: "SupplementalType", values: Some(IPTC_NEWSPHOTO_SUPPLEMENTALTYPE_VALUES) },
    60u16 => TagDef { name: "ColorRepresentation", values: Some(IPTC_NEWSPHOTO_COLORREPRESENTATION_VALUES) },
    64u16 => TagDef { name: "InterchangeColorSpace", values: Some(IPTC_NEWSPHOTO_INTERCHANGECOLORSPACE_VALUES) },
    65u16 => TagDef { name: "ColorSequence", values: None },
    66u16 => TagDef { name: "ICC_Profile", values: None },
    70u16 => TagDef { name: "ColorCalibrationMatrix", values: None },
    80u16 => TagDef { name: "LookupTable", values: None },
    84u16 => TagDef { name: "NumIndexEntries", values: None },
    85u16 => TagDef { name: "ColorPalette", values: None },
    86u16 => TagDef { name: "IPTCBitsPerSample", values: None },
    90u16 => TagDef { name: "SampleStructure", values: Some(IPTC_NEWSPHOTO_SAMPLESTRUCTURE_VALUES) },
};

pub static IPTC_NEWSPHOTO_SCANNINGDIRECTION_VALUES: &[(i64, &str)] = &[
    (0, "L-R, Top-Bottom"),
    (1, "R-L, Top-Bottom"),
    (2, "L-R, Bottom-Top"),
    (3, "R-L, Bottom-Top"),
    (4, "Top-Bottom, L-R"),
    (5, "Bottom-Top, L-R"),
    (6, "Top-Bottom, R-L"),
    (7, "Bottom-Top, R-L"),
];

pub static IPTC_NEWSPHOTO_IPTCIMAGEROTATION_VALUES: &[(i64, &str)] = &[
    (0, "0"),
    (1, "90"),
    (2, "180"),
    (3, "270"),
];

pub static IPTC_NEWSPHOTO_QUANTIZATIONMETHOD_VALUES: &[(i64, &str)] = &[
    (0, "Linear Reflectance/Transmittance"),
    (1, "Linear Density"),
    (2, "IPTC Ref B"),
    (3, "Linear Dot Percent"),
    (4, "AP Domestic Analogue"),
    (5, "Compression Method Specific"),
    (6, "Color Space Specific"),
    (7, "Gamma Compensated"),
];

pub static IPTC_NEWSPHOTO_EXCURSIONTOLERANCE_VALUES: &[(i64, &str)] = &[
    (0, "Not Allowed"),
    (1, "Allowed"),
];

pub static IPTC_NEWSPHOTO_SUPPLEMENTALTYPE_VALUES: &[(i64, &str)] = &[
    (0, "Main Image"),
    (1, "Reduced Resolution Image"),
    (2, "Logo"),
    (3, "Rasterized Caption"),
];

pub static IPTC_NEWSPHOTO_COLORREPRESENTATION_VALUES: &[(i64, &str)] = &[
    (0, "No Image, Single Frame"),
    (1024, "4 Components, Single Frame"),
    (1025, "4 Components, Frame Sequential in Multiple Objects"),
    (1026, "4 Components, Frame Sequential in One Object"),
    (1027, "4 Components, Line Sequential"),
    (1028, "4 Components, Pixel Sequential"),
    (1029, "4 Components, Special Interleaving"),
    (256, "Monochrome, Single Frame"),
    (768, "3 Components, Single Frame"),
    (769, "3 Components, Frame Sequential in Multiple Objects"),
    (770, "3 Components, Frame Sequential in One Object"),
    (771, "3 Components, Line Sequential"),
    (772, "3 Components, Pixel Sequential"),
    (773, "3 Components, Special Interleaving"),
];

pub static IPTC_NEWSPHOTO_INTERCHANGECOLORSPACE_VALUES: &[(i64, &str)] = &[
    (1, "X,Y,Z CIE"),
    (2, "RGB SMPTE"),
    (3, "Y,U,V (K) (D65)"),
    (4, "RGB Device Dependent"),
    (5, "CMY (K) Device Dependent"),
    (6, "Lab (K) CIE"),
    (7, "YCbCr"),
    (8, "sRGB"),
];

pub static IPTC_NEWSPHOTO_SAMPLESTRUCTURE_VALUES: &[(i64, &str)] = &[
    (0, "OrthogonalConstangSampling"),
    (1, "Orthogonal4-2-2Sampling"),
    (2, "CompressionDependent"),
];

/// IPTC::ObjectData tags
pub static IPTC_OBJECTDATA: phf::Map<u16, TagDef> = phf::phf_map! {
    10u16 => TagDef { name: "SubFile", values: None },
};

/// IPTC::PostObjectData tags
pub static IPTC_POSTOBJECTDATA: phf::Map<u16, TagDef> = phf::phf_map! {
    10u16 => TagDef { name: "ConfirmedObjectSize", values: None },
};

/// IPTC::PreObjectData tags
pub static IPTC_PREOBJECTDATA: phf::Map<u16, TagDef> = phf::phf_map! {
    10u16 => TagDef { name: "SizeMode", values: Some(IPTC_PREOBJECTDATA_SIZEMODE_VALUES) },
    20u16 => TagDef { name: "MaxSubfileSize", values: None },
    90u16 => TagDef { name: "ObjectSizeAnnounced", values: None },
    95u16 => TagDef { name: "MaximumObjectSize", values: None },
};

pub static IPTC_PREOBJECTDATA_SIZEMODE_VALUES: &[(i64, &str)] = &[
    (0, "Size Not Known"),
    (1, "Size Known"),
];

/// Kodak::IFD tags
pub static KODAK_IFD: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "KodakVersion", values: None },
    1u16 => TagDef { name: "UnknownEV", values: None },
    1001u16 => TagDef { name: "OriginalFileName", values: None },
    1002u16 => TagDef { name: "KodakTag", values: None },
    1003u16 => TagDef { name: "SensorLeftBorder", values: None },
    1004u16 => TagDef { name: "SensorTopBorder", values: None },
    1005u16 => TagDef { name: "SensorImageWidth", values: None },
    1006u16 => TagDef { name: "SensorImageHeight", values: None },
    1007u16 => TagDef { name: "BlackLevelTop", values: None },
    1008u16 => TagDef { name: "BlackLevelBottom", values: None },
    1009u16 => TagDef { name: "TextualInfo", values: None },
    1010u16 => TagDef { name: "FlashMode", values: None },
    1011u16 => TagDef { name: "FlashCompensation", values: None },
    1012u16 => TagDef { name: "WindMode", values: None },
    1013u16 => TagDef { name: "FocusMode", values: None },
    1016u16 => TagDef { name: "MinAperture", values: None },
    1017u16 => TagDef { name: "MaxAperture", values: None },
    1018u16 => TagDef { name: "WhiteBalanceMode", values: None },
    1019u16 => TagDef { name: "WhiteBalanceDetected", values: None },
    1020u16 => TagDef { name: "WhiteBalance", values: None },
    1021u16 => TagDef { name: "Processing", values: None },
    1022u16 => TagDef { name: "ImageAbsoluteX", values: None },
    1023u16 => TagDef { name: "ImageAbsoluteY", values: None },
    1024u16 => TagDef { name: "ApplicationKeyString", values: None },
    1025u16 => TagDef { name: "Time", values: None },
    1026u16 => TagDef { name: "GPSString", values: None },
    1027u16 => TagDef { name: "EventLogCapture", values: None },
    1028u16 => TagDef { name: "ComponentTable", values: None },
    1029u16 => TagDef { name: "CustomIlluminant", values: None },
    1030u16 => TagDef { name: "CameraTemperature", values: None },
    1031u16 => TagDef { name: "AdapterVoltage", values: None },
    1032u16 => TagDef { name: "BatteryVoltage", values: None },
    1033u16 => TagDef { name: "DacVoltages", values: None },
    1034u16 => TagDef { name: "IlluminantDetectorData", values: None },
    1035u16 => TagDef { name: "PixelClockFrequency", values: None },
    1036u16 => TagDef { name: "CenterPixel", values: None },
    1037u16 => TagDef { name: "BurstCount", values: None },
    1038u16 => TagDef { name: "BlackLevelRough", values: None },
    1039u16 => TagDef { name: "OffsetMapHorizontal", values: None },
    1040u16 => TagDef { name: "OffsetMapVertical", values: None },
    1041u16 => TagDef { name: "Histogram", values: None },
    1042u16 => TagDef { name: "VerticalClockOverlaps", values: None },
    1044u16 => TagDef { name: "XilinxVersion", values: None },
    1045u16 => TagDef { name: "FirmwareVersion", values: None },
    1046u16 => TagDef { name: "BlackLevelRoughAfter", values: None },
    1055u16 => TagDef { name: "ImageCropX", values: None },
    1056u16 => TagDef { name: "ImageCropY", values: None },
    1059u16 => TagDef { name: "IntegrationTime", values: None },
    1080u16 => TagDef { name: "MainBoardVersion", values: None },
    1081u16 => TagDef { name: "ImagerBoardVersion", values: None },
    1533u16 => TagDef { name: "ImagerPowerOnDelayMsec", values: None },
    1536u16 => TagDef { name: "ImagerBiasSettlingDelayMsec", values: None },
    2000u16 => TagDef { name: "StandardMatrixDaylight", values: None },
    2001u16 => TagDef { name: "StandardMatrixTungsten", values: None },
    2002u16 => TagDef { name: "StandardMatrixFluorescent", values: None },
    2003u16 => TagDef { name: "StandardMatrixFlash", values: None },
    2004u16 => TagDef { name: "StandardMatrixCustom", values: None },
    2010u16 => TagDef { name: "DeviantMatrixDaylight", values: None },
    2011u16 => TagDef { name: "DeviantMatrixTungsten", values: None },
    2012u16 => TagDef { name: "DeviantMatrixFluorescent", values: None },
    2013u16 => TagDef { name: "DeviantMatrixFlash", values: None },
    2014u16 => TagDef { name: "DeviantMatrixCustom", values: None },
    2020u16 => TagDef { name: "UniqueMatrixDaylight", values: None },
    2021u16 => TagDef { name: "UniqueMatrixTungsten", values: None },
    2022u16 => TagDef { name: "UniqueMatrixFluorescent", values: None },
    2023u16 => TagDef { name: "UniqueMatrixFlash", values: None },
    2024u16 => TagDef { name: "UniqueMatrixCustom", values: None },
    2025u16 => TagDef { name: "UniqueMatrixAuto", values: None },
    2100u16 => TagDef { name: "StandardWhiteDaylight", values: None },
    2101u16 => TagDef { name: "StandardWhiteTungsten", values: None },
    2102u16 => TagDef { name: "StandardWhiteFluorescent", values: None },
    2103u16 => TagDef { name: "StandardWhiteFlash", values: None },
    2104u16 => TagDef { name: "StandardWhiteCustom", values: None },
    2110u16 => TagDef { name: "DeviantWhiteDaylight", values: None },
    2111u16 => TagDef { name: "DeviantWhiteTungsten", values: None },
    2112u16 => TagDef { name: "DeviantWhiteFluorescent", values: None },
    2113u16 => TagDef { name: "DeviantWhiteFlash", values: None },
    2114u16 => TagDef { name: "DeviantWhiteCustom", values: None },
    2118u16 => TagDef { name: "ColorTemperature", values: None },
    2130u16 => TagDef { name: "WB_RGBMulDaylight", values: None },
    2131u16 => TagDef { name: "WB_RGBMulTungsten", values: None },
    2132u16 => TagDef { name: "WB_RGBMulFluorescent", values: None },
    2133u16 => TagDef { name: "WB_RGBMulFlash", values: None },
    2140u16 => TagDef { name: "WB_RGBCoeffsDaylight", values: None },
    2141u16 => TagDef { name: "WB_RGBCoeffsTungsten", values: None },
    2142u16 => TagDef { name: "WB_RGBCoeffsFluorescent", values: None },
    2143u16 => TagDef { name: "WB_RGBCoeffsFlash", values: None },
    2200u16 => TagDef { name: "ExposureGainDaylight", values: None },
    2201u16 => TagDef { name: "ExposureGainTungsten", values: None },
    2202u16 => TagDef { name: "ExposureGainFluorescent", values: None },
    2203u16 => TagDef { name: "ExposureGainFlash", values: None },
    2204u16 => TagDef { name: "ExposureGainCustom", values: None },
    2205u16 => TagDef { name: "AnalogISOTable", values: None },
    2206u16 => TagDef { name: "AnalogCaptureISO", values: None },
    2207u16 => TagDef { name: "ISOCalibrationGain", values: None },
    2300u16 => TagDef { name: "MonitorMatrix", values: None },
    2302u16 => TagDef { name: "Gamma", values: None },
    2305u16 => TagDef { name: "GammaTable", values: None },
    2306u16 => TagDef { name: "LogScale", values: None },
    2307u16 => TagDef { name: "BaseISO", values: None },
    2308u16 => TagDef { name: "LinLogCoring", values: None },
    2309u16 => TagDef { name: "PatternGainConversionTable", values: None },
    2311u16 => TagDef { name: "DefectList", values: None },
    2312u16 => TagDef { name: "DefectListPacked", values: None },
    2313u16 => TagDef { name: "ImageSpace", values: None },
    2314u16 => TagDef { name: "ThumbnailCompressionTable", values: None },
    2315u16 => TagDef { name: "ThumbnailExpansionTable", values: None },
    2316u16 => TagDef { name: "ImageCompressionTable", values: None },
    2317u16 => TagDef { name: "ImageExpansionTable", values: None },
    2319u16 => TagDef { name: "DefectIsoCode", values: None },
    2320u16 => TagDef { name: "BaseISODaylight", values: None },
    2321u16 => TagDef { name: "BaseISOTungsten", values: None },
    2322u16 => TagDef { name: "BaseISOFluorescent", values: None },
    2323u16 => TagDef { name: "BaseISOFlash", values: None },
    2330u16 => TagDef { name: "MatrixSelectThreshold", values: None },
    2331u16 => TagDef { name: "MatrixSelectK", values: None },
    2332u16 => TagDef { name: "IlluminantDetectTable", values: None },
    2334u16 => TagDef { name: "MatrixSelectThreshold1", values: None },
    2335u16 => TagDef { name: "MatrixSelectThreshold2", values: None },
    2350u16 => TagDef { name: "EnableSharpening", values: None },
    2351u16 => TagDef { name: "SharpeningKernel", values: None },
    2352u16 => TagDef { name: "EdgeMapSlope", values: None },
    2353u16 => TagDef { name: "EdgeMapX1", values: None },
    2354u16 => TagDef { name: "EdgeMapX2", values: None },
    2355u16 => TagDef { name: "KernelDenominators", values: None },
    2356u16 => TagDef { name: "EdgeMapX3", values: None },
    2357u16 => TagDef { name: "EdgeMapX4", values: None },
    2371u16 => TagDef { name: "AtCaptureUserCrop", values: None },
    2372u16 => TagDef { name: "ImageResolution", values: None },
    2373u16 => TagDef { name: "ImageResolutionJpg", values: None },
    2384u16 => TagDef { name: "EdgeSplineLow", values: None },
    2385u16 => TagDef { name: "EdgeSplineMed", values: None },
    2386u16 => TagDef { name: "EdgeSplineHigh", values: None },
    2400u16 => TagDef { name: "PatternImagerWidth", values: None },
    2401u16 => TagDef { name: "PatternImagerHeight", values: None },
    2402u16 => TagDef { name: "PatternAreaWidth", values: None },
    2403u16 => TagDef { name: "PatternAreaHeight", values: None },
    2404u16 => TagDef { name: "PatternCorrectionGains", values: None },
    2406u16 => TagDef { name: "PatternX", values: None },
    2407u16 => TagDef { name: "PatternY", values: None },
    2408u16 => TagDef { name: "PatternCorrectionFactors", values: None },
    2409u16 => TagDef { name: "PatternCorrectionFactorScale", values: None },
    2410u16 => TagDef { name: "PatternCropRows1", values: None },
    2411u16 => TagDef { name: "PatternCropRows2", values: None },
    2412u16 => TagDef { name: "PatternCropCols1", values: None },
    2413u16 => TagDef { name: "PatternCropCols2", values: None },
    2417u16 => TagDef { name: "PixelCorrectionScale", values: None },
    2418u16 => TagDef { name: "PixelCorrectionOffset", values: None },
    2500u16 => TagDef { name: "ImagerFileProductionLevel", values: None },
    2501u16 => TagDef { name: "ImagerFileDateCreated", values: None },
    2502u16 => TagDef { name: "CalibrationVersion", values: None },
    2503u16 => TagDef { name: "ImagerFileTagsVersionStandard", values: None },
    2504u16 => TagDef { name: "IFCameraModel", values: None },
    2505u16 => TagDef { name: "CalibrationHistory", values: None },
    2506u16 => TagDef { name: "CalibrationLog", values: None },
    2510u16 => TagDef { name: "SensorSerialNumber", values: None },
    2570u16 => TagDef { name: "DefectConcealThresTable", values: None },
    2653u16 => TagDef { name: "OmenInitialIPFStrength", values: None },
    2654u16 => TagDef { name: "OmenEarlyStrength", values: None },
    2655u16 => TagDef { name: "OmenAutoStrength", values: None },
    2656u16 => TagDef { name: "OmenAtCaptureStrength", values: None },
    2658u16 => TagDef { name: "OmenFocalLengthLimit", values: None },
    2660u16 => TagDef { name: "OmenSurfaceIndex", values: None },
    3u16 => TagDef { name: "ExposureValue", values: None },
    3109u16 => TagDef { name: "SBABlack", values: None },
    3110u16 => TagDef { name: "SBAGray", values: None },
    3111u16 => TagDef { name: "SBAWhite", values: None },
    3112u16 => TagDef { name: "GaussianWeights", values: None },
    3113u16 => TagDef { name: "SfsBoundary", values: None },
    3122u16 => TagDef { name: "SBANeutralBAL", values: None },
    3123u16 => TagDef { name: "SBAGreenMagentaBAL", values: None },
    3124u16 => TagDef { name: "SBAIlluminantBAL", values: None },
    3125u16 => TagDef { name: "SBAAnalysisComplete", values: None },
    3133u16 => TagDef { name: "QTableLarge50Pct", values: None },
    3134u16 => TagDef { name: "QTableLarge67Pct", values: None },
    3135u16 => TagDef { name: "QTableLarge100Pct", values: None },
    3136u16 => TagDef { name: "QTableMedium50Pct", values: None },
    3137u16 => TagDef { name: "QTableMedium67Pct", values: None },
    3138u16 => TagDef { name: "QTableMedium100Pct", values: None },
    3139u16 => TagDef { name: "QTableSmall50Pct", values: None },
    3140u16 => TagDef { name: "QTableSmall67Pct", values: None },
    3141u16 => TagDef { name: "QTableSmall100Pct", values: None },
    3142u16 => TagDef { name: "SBAHighGray", values: None },
    3143u16 => TagDef { name: "SBALowGray", values: None },
    3144u16 => TagDef { name: "CaptureLook", values: None },
    3145u16 => TagDef { name: "SBAIllOffset", values: None },
    3146u16 => TagDef { name: "SBAGmOffset", values: None },
    3169u16 => TagDef { name: "ProcessBorderColsLeft", values: None },
    3170u16 => TagDef { name: "ProcessBorderColsRight", values: None },
    3171u16 => TagDef { name: "ProcessBorderRowsTop", values: None },
    3172u16 => TagDef { name: "ProcessBorderRowsBottom", values: None },
    3183u16 => TagDef { name: "CFAOffsetRows", values: None },
    3184u16 => TagDef { name: "ShiftCols", values: None },
    3185u16 => TagDef { name: "CFAOffsetCols", values: None },
    3194u16 => TagDef { name: "DMDitherMatrix", values: None },
    3195u16 => TagDef { name: "DMDitherMatrixWidth", values: None },
    3196u16 => TagDef { name: "DMDitherMatrixHeight", values: None },
    3197u16 => TagDef { name: "MaxPixelValueThreshold", values: None },
    3198u16 => TagDef { name: "HoleFillDeltaThreshold", values: None },
    3199u16 => TagDef { name: "DarkPedestal", values: None },
    3200u16 => TagDef { name: "ImageProcessingFileTagsVersionNumber", values: None },
    3201u16 => TagDef { name: "ImageProcessingFileDateCreated", values: None },
    3202u16 => TagDef { name: "DoublingMicrovolts", values: None },
    3203u16 => TagDef { name: "DarkFrameShortExposure", values: None },
    3204u16 => TagDef { name: "DarkFrameLongExposure", values: None },
    3205u16 => TagDef { name: "DarkFrameCountFactor", values: None },
    3208u16 => TagDef { name: "HoleFillDarkDeltaThreshold", values: None },
    3210u16 => TagDef { name: "ColumnResetOffsets", values: None },
    3211u16 => TagDef { name: "ColumnGainFactors", values: None },
    3301u16 => TagDef { name: "FirmwareVersion", values: None },
    3303u16 => TagDef { name: "HostSoftwareRendering", values: Some(KODAK_IFD_HOSTSOFTWARERENDERING_VALUES) },
    3502u16 => TagDef { name: "IPAVersion", values: None },
    3512u16 => TagDef { name: "FinishFileType", values: Some(KODAK_IFD_FINISHFILETYPE_VALUES) },
    3513u16 => TagDef { name: "FinishResolution", values: Some(KODAK_IFD_FINISHRESOLUTION_VALUES) },
    3514u16 => TagDef { name: "FinishNoise", values: Some(KODAK_IFD_FINISHNOISE_VALUES) },
    3515u16 => TagDef { name: "FinishSharpening", values: Some(KODAK_IFD_FINISHSHARPENING_VALUES) },
    3516u16 => TagDef { name: "FinishLook", values: Some(KODAK_IFD_FINISHLOOK_VALUES) },
    3517u16 => TagDef { name: "FinishExposure", values: Some(KODAK_IFD_FINISHEXPOSURE_VALUES) },
    3595u16 => TagDef { name: "SigmaScalingFactorLowRes", values: None },
    3596u16 => TagDef { name: "SigmaScalingFactorCamera", values: None },
    3597u16 => TagDef { name: "SigmaImpulseParameters", values: None },
    3598u16 => TagDef { name: "SigmaNoiseThreshTableV2", values: None },
    3599u16 => TagDef { name: "SigmaSizeTable", values: None },
    3601u16 => TagDef { name: "SigmaNoiseFilterCalTableV1", values: None },
    3602u16 => TagDef { name: "SigmaNoiseFilterTableV1", values: None },
    3607u16 => TagDef { name: "NifNonlinearity12To16", values: None },
    3608u16 => TagDef { name: "SBALog12Transform", values: None },
    3609u16 => TagDef { name: "InverseSBALog12Transform", values: None },
    3610u16 => TagDef { name: "ToneScale0", values: None },
    3611u16 => TagDef { name: "ToneScale1", values: None },
    3612u16 => TagDef { name: "ToneScale2", values: None },
    3613u16 => TagDef { name: "ToneScale3", values: None },
    3614u16 => TagDef { name: "ToneScale4", values: None },
    3615u16 => TagDef { name: "ToneScale5", values: None },
    3616u16 => TagDef { name: "ToneScale6", values: None },
    3617u16 => TagDef { name: "ToneScale7", values: None },
    3618u16 => TagDef { name: "ToneScale8", values: None },
    3619u16 => TagDef { name: "ToneScale9", values: None },
    3660u16 => TagDef { name: "KodakLook", values: None },
    3661u16 => TagDef { name: "IPFCameraModel", values: None },
    3662u16 => TagDef { name: "AH2GreenInterpolationThreshold", values: None },
    3663u16 => TagDef { name: "ResamplingKernelDenominators067", values: None },
    3664u16 => TagDef { name: "ResamplingKernelDenominators050", values: None },
    3665u16 => TagDef { name: "ResamplingKernelDenominators100", values: None },
    3680u16 => TagDef { name: "CFAInterpolationAlgorithm", values: Some(KODAK_IFD_CFAINTERPOLATIONALGORITHM_VALUES) },
    3681u16 => TagDef { name: "CFAInterpolationMetric", values: Some(KODAK_IFD_CFAINTERPOLATIONMETRIC_VALUES) },
    3682u16 => TagDef { name: "CFAZipperFixThreshold", values: None },
    3683u16 => TagDef { name: "NoiseReductionParametersKhufuRGB", values: None },
    3684u16 => TagDef { name: "NoiseReductionParametersKhufu6MP", values: None },
    3685u16 => TagDef { name: "NoiseReductionParametersKhufu3MP", values: None },
    3690u16 => TagDef { name: "ChromaNoiseHighFThresh", values: None },
    3691u16 => TagDef { name: "ChromaNoiseLowFThresh", values: None },
    3692u16 => TagDef { name: "ChromaNoiseEdgeMapThresh", values: None },
    3693u16 => TagDef { name: "ChromaNoiseColorSpace", values: None },
    3694u16 => TagDef { name: "EnableChromaNoiseReduction", values: None },
    3695u16 => TagDef { name: "NoiseReductionParametersHostRGB", values: None },
    3696u16 => TagDef { name: "NoiseReductionParametersHost6MP", values: None },
    3697u16 => TagDef { name: "NoiseReductionParametersHost3MP", values: None },
    3698u16 => TagDef { name: "NoiseReductionParametersCamera", values: None },
    3699u16 => TagDef { name: "NoiseReductionParametersAtCapture", values: None },
    3700u16 => TagDef { name: "LCDMatrix", values: None },
    3701u16 => TagDef { name: "LCDMatrixChickFix", values: None },
    3702u16 => TagDef { name: "LCDMatrixMarvin", values: None },
    3708u16 => TagDef { name: "LCDGammaTableChickFix", values: None },
    3709u16 => TagDef { name: "LCDGammaTableMarvin", values: None },
    3710u16 => TagDef { name: "LCDGammaTable", values: None },
    3730u16 => TagDef { name: "Fac18Per", values: None },
    3731u16 => TagDef { name: "Fac170Per", values: None },
    3732u16 => TagDef { name: "Fac100Per", values: None },
    3740u16 => TagDef { name: "RGBtoeV0", values: None },
    3741u16 => TagDef { name: "RGBtoeV1", values: None },
    3742u16 => TagDef { name: "RGBtoeV2", values: None },
    3743u16 => TagDef { name: "RGBtoeV3", values: None },
    3744u16 => TagDef { name: "RGBtoeV4", values: None },
    3745u16 => TagDef { name: "RGBtoeV5", values: None },
    3746u16 => TagDef { name: "RGBtoeV6", values: None },
    3747u16 => TagDef { name: "RGBtoeV7", values: None },
    3748u16 => TagDef { name: "RGBtoeV8", values: None },
    3749u16 => TagDef { name: "RGBtoeV9", values: None },
    3750u16 => TagDef { name: "LCDHistLUT0", values: None },
    3751u16 => TagDef { name: "LCDHistLUT1", values: None },
    3752u16 => TagDef { name: "LCDHistLUT2", values: None },
    3753u16 => TagDef { name: "LCDHistLUT3", values: None },
    3754u16 => TagDef { name: "LCDHistLUT4", values: None },
    3755u16 => TagDef { name: "LCDHistLUT5", values: None },
    3756u16 => TagDef { name: "LCDHistLUT6", values: None },
    3757u16 => TagDef { name: "LCDHistLUT7", values: None },
    3758u16 => TagDef { name: "LCDHistLUT8", values: None },
    3759u16 => TagDef { name: "LCDHistLUT9", values: None },
    3800u16 => TagDef { name: "InterpolationCoefficients", values: None },
    3840u16 => TagDef { name: "NoiseReductionParametersHostNormal", values: None },
    3841u16 => TagDef { name: "NoiseReductionParametersHostStrong", values: None },
    3842u16 => TagDef { name: "NoiseReductionParametersHostLow", values: None },
    3850u16 => TagDef { name: "MariahTextureThreshold", values: None },
    3851u16 => TagDef { name: "MariahMapLoThreshold", values: None },
    3852u16 => TagDef { name: "MariahMapHiThreshold", values: None },
    3853u16 => TagDef { name: "MariahChromaBlurSize", values: None },
    3854u16 => TagDef { name: "MariahSigmaThreshold", values: None },
    3855u16 => TagDef { name: "MariahThresholds", values: None },
    3856u16 => TagDef { name: "MariahThresholdsNormal", values: None },
    3857u16 => TagDef { name: "MariahThresholdsStrong", values: None },
    3858u16 => TagDef { name: "MariahThresholdsLow", values: None },
    3890u16 => TagDef { name: "KhufuI0Thresholds", values: None },
    3891u16 => TagDef { name: "KhufuI1Thresholds", values: None },
    3892u16 => TagDef { name: "KhufuI2Thresholds", values: None },
    3893u16 => TagDef { name: "KhufuI3Thresholds", values: None },
    3894u16 => TagDef { name: "KhufuI4Thresholds", values: None },
    3895u16 => TagDef { name: "KhufuI5Thresholds", values: None },
    3900u16 => TagDef { name: "CondadoDayBVThresh", values: None },
    3901u16 => TagDef { name: "CondadoNeuRange", values: None },
    3902u16 => TagDef { name: "CondadoBVFactor", values: None },
    3903u16 => TagDef { name: "CondadoIllFactor", values: None },
    3904u16 => TagDef { name: "CondadoTunThresh", values: None },
    3905u16 => TagDef { name: "CondadoFluThresh", values: None },
    3906u16 => TagDef { name: "CondadoDayOffsets", values: None },
    3907u16 => TagDef { name: "CondadoTunOffsets", values: None },
    3908u16 => TagDef { name: "CondadoFluOffsets", values: None },
    3954u16 => TagDef { name: "NifNonlinearityExt", values: None },
    3955u16 => TagDef { name: "InvNifNonLinearity", values: None },
    5001u16 => TagDef { name: "InputProfile", values: None },
    5002u16 => TagDef { name: "KodakLookProfile", values: None },
    5003u16 => TagDef { name: "OutputProfile", values: None },
    5008u16 => TagDef { name: "SourceProfilePrefix", values: None },
    5009u16 => TagDef { name: "ToneCurveProfileName", values: None },
    5010u16 => TagDef { name: "InputProfile", values: None },
    5011u16 => TagDef { name: "ProcessParametersV2", values: None },
    6000u16 => TagDef { name: "ScriptVersion", values: None },
    6020u16 => TagDef { name: "ISO", values: None },
    6100u16 => TagDef { name: "ImagerCols", values: None },
    6110u16 => TagDef { name: "ImagerRows", values: None },
    6120u16 => TagDef { name: "PartialActiveCols1", values: None },
    6130u16 => TagDef { name: "PartialActiveCols2", values: None },
    6140u16 => TagDef { name: "PartialActiveRows1", values: None },
    6150u16 => TagDef { name: "PartialActiveRows2", values: None },
    6160u16 => TagDef { name: "ElectricalBlackColumns", values: None },
    6170u16 => TagDef { name: "ResetBlackSegRows", values: None },
    6200u16 => TagDef { name: "CaptureWidthNormal", values: None },
    6201u16 => TagDef { name: "CaptureHeightNormal", values: None },
    6210u16 => TagDef { name: "CaptureWidthTest", values: None },
    6220u16 => TagDef { name: "ImageSegmentStartLine", values: None },
    6221u16 => TagDef { name: "ImageSegmentLines", values: None },
    6222u16 => TagDef { name: "SkipLineTime", values: None },
    6240u16 => TagDef { name: "FastResetLineTime", values: None },
    6250u16 => TagDef { name: "NormalLineTime", values: None },
    6260u16 => TagDef { name: "MinIntegrationRows", values: None },
    6270u16 => TagDef { name: "PreReadFastResetCount", values: None },
    6280u16 => TagDef { name: "TransferTimeNormal", values: None },
    6281u16 => TagDef { name: "TransferTimeTest", values: None },
    6282u16 => TagDef { name: "QuietTime", values: None },
    6300u16 => TagDef { name: "OverClockCols", values: None },
    6310u16 => TagDef { name: "H2ResetBlackPixels", values: None },
    6320u16 => TagDef { name: "H3ResetBlackPixels", values: None },
    6330u16 => TagDef { name: "BlackAcquireRows", values: None },
    6340u16 => TagDef { name: "OverClockRows", values: None },
    6350u16 => TagDef { name: "H3ResetBlackColumns", values: None },
    6360u16 => TagDef { name: "DarkBlackSegRows", values: None },
    6401u16 => TagDef { name: "FifoenOnePixelDelay", values: None },
    6402u16 => TagDef { name: "ReadoutTypeRequested", values: None },
    6403u16 => TagDef { name: "ReadoutTypeActual", values: None },
    6410u16 => TagDef { name: "OffsetDacValue", values: None },
    6420u16 => TagDef { name: "TempAmpGainX100", values: None },
    6430u16 => TagDef { name: "VarrayDacNominalValues", values: None },
    6500u16 => TagDef { name: "C14Configuration", values: None },
    65000u16 => TagDef { name: "ComLenBlkSize", values: None },
    6510u16 => TagDef { name: "TDA1Offset", values: None },
    6511u16 => TagDef { name: "TDA1Bandwidth", values: None },
    6512u16 => TagDef { name: "TDA1Gain", values: None },
    6513u16 => TagDef { name: "TDA1EdgePolarity", values: None },
    6520u16 => TagDef { name: "TDA2Offset", values: None },
    6521u16 => TagDef { name: "TDA2Bandwidth", values: None },
    6522u16 => TagDef { name: "TDA2Gain", values: None },
    6523u16 => TagDef { name: "TDA2EdgePolarity", values: None },
    6530u16 => TagDef { name: "TDA3Offset", values: None },
    6531u16 => TagDef { name: "TDA3Bandwidth", values: None },
    6532u16 => TagDef { name: "TDA3Gain", values: None },
    6533u16 => TagDef { name: "TDA3EdgePolarity", values: None },
    6540u16 => TagDef { name: "TDA4Offset", values: None },
    6541u16 => TagDef { name: "TDA4Bandwidth", values: None },
    6542u16 => TagDef { name: "TDA4Gain", values: None },
    6543u16 => TagDef { name: "TDA4EdgePolarity", values: None },
};

pub static KODAK_IFD_HOSTSOFTWARERENDERING_VALUES: &[(i64, &str)] = &[
    (0, "Normal (sRGB)"),
    (1, "Linear (camera RGB)"),
    (2, "Pro Photo RGB"),
    (3, "Unknown"),
    (4, "Other Profile"),
];

pub static KODAK_IFD_FINISHFILETYPE_VALUES: &[(i64, &str)] = &[
    (0, "JPEG Best"),
    (1, "JPEG Better"),
    (2, "JPEG Good"),
    (3, "TIFF RGB"),
];

pub static KODAK_IFD_FINISHRESOLUTION_VALUES: &[(i64, &str)] = &[
    (0, "100%"),
    (1, "67%"),
    (2, "50%"),
    (3, "25%"),
];

pub static KODAK_IFD_FINISHNOISE_VALUES: &[(i64, &str)] = &[
    (0, "Normal"),
    (1, "Strong"),
    (2, "Low"),
];

pub static KODAK_IFD_FINISHSHARPENING_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "High"),
    (2, "Medium"),
    (3, "Low"),
];

pub static KODAK_IFD_FINISHLOOK_VALUES: &[(i64, &str)] = &[
    (0, "Product"),
    (1, "Portrait"),
    (10, "Product Hi Color Hold"),
    (11, "Portrait Hi Color Hold"),
    (13, "DCS BW Normal"),
    (14, "DCS BW Wratten 8"),
    (15, "DCS BW Wratten 25"),
    (16, "DCS Sepia 1"),
    (17, "DCS Sepia 2"),
    (2, "Product Reduced"),
    (3, "Portrait Reduced"),
    (4, "Monochrome Product"),
    (5, "Monochrome Portrait"),
    (6, "Wedding"),
    (7, "Event"),
    (8, "Product Hi Color"),
    (9, "Portrait Hi Color"),
];

pub static KODAK_IFD_FINISHEXPOSURE_VALUES: &[(i64, &str)] = &[
    (0, "Yes"),
    (1, "No"),
];

pub static KODAK_IFD_CFAINTERPOLATIONALGORITHM_VALUES: &[(i64, &str)] = &[
    (0, "AH2"),
    (1, "Karnak"),
];

pub static KODAK_IFD_CFAINTERPOLATIONMETRIC_VALUES: &[(i64, &str)] = &[
    (0, "Linear12"),
    (1, "KLUT12"),
];

/// Kodak::KDC_IFD tags
pub static KODAK_KDC_IFD: phf::Map<u16, TagDef> = phf::phf_map! {
    64000u16 => TagDef { name: "SerialNumber", values: None },
    64013u16 => TagDef { name: "WhiteBalance", values: Some(KODAK_KDC_IFD_WHITEBALANCE_VALUES) },
};

pub static KODAK_KDC_IFD_WHITEBALANCE_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "Fluorescent"),
    (2, "Tungsten"),
    (3, "Daylight"),
    (6, "Shade"),
];

/// Kodak::Processing tags
pub static KODAK_PROCESSING: phf::Map<u16, TagDef> = phf::phf_map! {
    20u16 => TagDef { name: "WB_RGBLevels", values: None },
};

/// Microsoft::Stitch tags
pub static MICROSOFT_STITCH: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "PanoramicStitchVersion", values: None },
    1u16 => TagDef { name: "PanoramicStitchCameraMotion", values: Some(MICROSOFT_STITCH_PANORAMICSTITCHCAMERAMOTION_VALUES) },
    2u16 => TagDef { name: "PanoramicStitchMapType", values: Some(MICROSOFT_STITCH_PANORAMICSTITCHMAPTYPE_VALUES) },
};

pub static MICROSOFT_STITCH_PANORAMICSTITCHCAMERAMOTION_VALUES: &[(i64, &str)] = &[
    (2, "Rigid Scale"),
    (3, "Affine"),
    (4, "3D Rotation"),
    (5, "Homography"),
];

pub static MICROSOFT_STITCH_PANORAMICSTITCHMAPTYPE_VALUES: &[(i64, &str)] = &[
    (0, "Perspective"),
    (1, "Horizontal Cylindrical"),
    (2, "Horizontal Spherical"),
    (257, "Vertical Cylindrical"),
    (258, "Vertical Spherical"),
];

/// Nikon::DistortionInfo tags
pub static NIKON_DISTORTIONINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "DistortionCorrectionVersion", values: None },
    20u16 => TagDef { name: "RadialDistortionCoefficient1", values: None },
    28u16 => TagDef { name: "RadialDistortionCoefficient2", values: None },
    36u16 => TagDef { name: "RadialDistortionCoefficient3", values: None },
    4u16 => TagDef { name: "DistortionCorrection", values: Some(NIKON_DISTORTIONINFO_DISTORTIONCORRECTION_VALUES) },
};

pub static NIKON_DISTORTIONINFO_DISTORTIONCORRECTION_VALUES: &[(i64, &str)] = &[
    (0, "No Lens Attached"),
    (1, "On (Optional)"),
    (2, "Off"),
    (3, "On (Required)"),
];

/// Nikon::NEFInfo tags
pub static NIKON_NEFINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    5u16 => TagDef { name: "DistortionInfo", values: None },
    6u16 => TagDef { name: "VignetteInfo", values: None },
};

/// Nikon::VignetteInfo tags
pub static NIKON_VIGNETTEINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "VignetteCorrectionVersion", values: None },
    36u16 => TagDef { name: "VignetteCoefficient1", values: None },
    52u16 => TagDef { name: "VignetteCoefficient2", values: None },
    68u16 => TagDef { name: "VignetteCoefficient3", values: None },
};

/// Photoshop::ChannelOptions tags
pub static PHOTOSHOP_CHANNELOPTIONS: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "ChannelColorSpace", values: Some(PHOTOSHOP_CHANNELOPTIONS_CHANNELCOLORSPACE_VALUES) },
    11u16 => TagDef { name: "ChannelOpacity", values: None },
    12u16 => TagDef { name: "ChannelColorIndicates", values: Some(PHOTOSHOP_CHANNELOPTIONS_CHANNELCOLORINDICATES_VALUES) },
    2u16 => TagDef { name: "ChannelColorData", values: None },
};

pub static PHOTOSHOP_CHANNELOPTIONS_CHANNELCOLORSPACE_VALUES: &[(i64, &str)] = &[
    (0, "RGB"),
    (1, "HSB"),
    (2, "CMYK"),
    (7, "Lab"),
    (8, "Grayscale"),
];

pub static PHOTOSHOP_CHANNELOPTIONS_CHANNELCOLORINDICATES_VALUES: &[(i64, &str)] = &[
    (0, "Selected Areas"),
    (1, "Masked Areas"),
    (2, "Spot Color"),
];

/// Photoshop::JPEG_Quality tags
pub static PHOTOSHOP_JPEG_QUALITY: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "PhotoshopQuality", values: None },
    1u16 => TagDef { name: "PhotoshopFormat", values: Some(PHOTOSHOP_JPEG_QUALITY_PHOTOSHOPFORMAT_VALUES) },
    2u16 => TagDef { name: "ProgressiveScans", values: Some(PHOTOSHOP_JPEG_QUALITY_PROGRESSIVESCANS_VALUES) },
};

pub static PHOTOSHOP_JPEG_QUALITY_PHOTOSHOPFORMAT_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (1, "Optimized"),
    (257, "Progressive"),
];

pub static PHOTOSHOP_JPEG_QUALITY_PROGRESSIVESCANS_VALUES: &[(i64, &str)] = &[
    (1, "3 Scans"),
    (2, "4 Scans"),
    (3, "5 Scans"),
];

/// Photoshop::Main tags
pub static PHOTOSHOP_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    1000u16 => TagDef { name: "Photoshop2Info", values: None },
    10000u16 => TagDef { name: "PrintFlagsInfo", values: None },
    1001u16 => TagDef { name: "MacintoshPrintInfo", values: None },
    1002u16 => TagDef { name: "XMLData", values: None },
    1003u16 => TagDef { name: "Photoshop2ColorTable", values: None },
    1005u16 => TagDef { name: "ResolutionInfo", values: None },
    1006u16 => TagDef { name: "AlphaChannelsNames", values: None },
    1007u16 => TagDef { name: "DisplayInfo", values: None },
    1008u16 => TagDef { name: "PStringCaption", values: None },
    1009u16 => TagDef { name: "BorderInformation", values: None },
    1010u16 => TagDef { name: "BackgroundColor", values: None },
    1011u16 => TagDef { name: "PrintFlags", values: None },
    1012u16 => TagDef { name: "BW_HalftoningInfo", values: None },
    1013u16 => TagDef { name: "ColorHalftoningInfo", values: None },
    1014u16 => TagDef { name: "DuotoneHalftoningInfo", values: None },
    1015u16 => TagDef { name: "BW_TransferFunc", values: None },
    1016u16 => TagDef { name: "ColorTransferFuncs", values: None },
    1017u16 => TagDef { name: "DuotoneTransferFuncs", values: None },
    1018u16 => TagDef { name: "DuotoneImageInfo", values: None },
    1019u16 => TagDef { name: "EffectiveBW", values: None },
    1020u16 => TagDef { name: "ObsoletePhotoshopTag1", values: None },
    1021u16 => TagDef { name: "EPSOptions", values: None },
    1022u16 => TagDef { name: "QuickMaskInfo", values: None },
    1023u16 => TagDef { name: "ObsoletePhotoshopTag2", values: None },
    1024u16 => TagDef { name: "TargetLayerID", values: None },
    1025u16 => TagDef { name: "WorkingPath", values: None },
    1026u16 => TagDef { name: "LayersGroupInfo", values: None },
    1027u16 => TagDef { name: "ObsoletePhotoshopTag3", values: None },
    1028u16 => TagDef { name: "IPTCData", values: None },
    1029u16 => TagDef { name: "RawImageMode", values: None },
    1030u16 => TagDef { name: "JPEG_Quality", values: None },
    1032u16 => TagDef { name: "GridGuidesInfo", values: None },
    1033u16 => TagDef { name: "PhotoshopBGRThumbnail", values: None },
    1034u16 => TagDef { name: "CopyrightFlag", values: Some(PHOTOSHOP_MAIN_COPYRIGHTFLAG_VALUES) },
    1035u16 => TagDef { name: "URL", values: None },
    1036u16 => TagDef { name: "PhotoshopThumbnail", values: None },
    1037u16 => TagDef { name: "GlobalAngle", values: None },
    1038u16 => TagDef { name: "ColorSamplersResource", values: None },
    1039u16 => TagDef { name: "ICC_Profile", values: None },
    1040u16 => TagDef { name: "Watermark", values: None },
    1041u16 => TagDef { name: "ICC_Untagged", values: None },
    1042u16 => TagDef { name: "EffectsVisible", values: None },
    1043u16 => TagDef { name: "SpotHalftone", values: None },
    1044u16 => TagDef { name: "IDsBaseValue", values: None },
    1045u16 => TagDef { name: "UnicodeAlphaNames", values: None },
    1046u16 => TagDef { name: "IndexedColorTableCount", values: None },
    1047u16 => TagDef { name: "TransparentIndex", values: None },
    1049u16 => TagDef { name: "GlobalAltitude", values: None },
    1050u16 => TagDef { name: "SliceInfo", values: None },
    1051u16 => TagDef { name: "WorkflowURL", values: None },
    1052u16 => TagDef { name: "JumpToXPEP", values: None },
    1053u16 => TagDef { name: "AlphaIdentifiers", values: None },
    1054u16 => TagDef { name: "URL_List", values: None },
    1057u16 => TagDef { name: "VersionInfo", values: None },
    1058u16 => TagDef { name: "EXIFInfo", values: None },
    1059u16 => TagDef { name: "ExifInfo2", values: None },
    1060u16 => TagDef { name: "XMP", values: None },
    1061u16 => TagDef { name: "IPTCDigest", values: None },
    1062u16 => TagDef { name: "PrintScaleInfo", values: None },
    1064u16 => TagDef { name: "PixelInfo", values: None },
    1065u16 => TagDef { name: "LayerComps", values: None },
    1066u16 => TagDef { name: "AlternateDuotoneColors", values: None },
    1067u16 => TagDef { name: "AlternateSpotColors", values: None },
    1069u16 => TagDef { name: "LayerSelectionIDs", values: None },
    1070u16 => TagDef { name: "HDRToningInfo", values: None },
    1071u16 => TagDef { name: "PrintInfo", values: None },
    1072u16 => TagDef { name: "LayerGroupsEnabledID", values: None },
    1073u16 => TagDef { name: "ColorSamplersResource2", values: None },
    1074u16 => TagDef { name: "MeasurementScale", values: None },
    1075u16 => TagDef { name: "TimelineInfo", values: None },
    1076u16 => TagDef { name: "SheetDisclosure", values: None },
    1077u16 => TagDef { name: "ChannelOptions", values: None },
    1078u16 => TagDef { name: "OnionSkins", values: None },
    1080u16 => TagDef { name: "CountInfo", values: None },
    1082u16 => TagDef { name: "PrintInfo2", values: None },
    1083u16 => TagDef { name: "PrintStyle", values: None },
    1084u16 => TagDef { name: "MacintoshNSPrintInfo", values: None },
    1085u16 => TagDef { name: "WindowsDEVMODE", values: None },
    1086u16 => TagDef { name: "AutoSaveFilePath", values: None },
    1087u16 => TagDef { name: "AutoSaveFormat", values: None },
    1088u16 => TagDef { name: "PathSelectionState", values: None },
    2999u16 => TagDef { name: "ClippingPathName", values: None },
    3000u16 => TagDef { name: "OriginPathInfo", values: None },
    7000u16 => TagDef { name: "ImageReadyVariables", values: None },
    7001u16 => TagDef { name: "ImageReadyDataSets", values: None },
    8000u16 => TagDef { name: "LightroomWorkflow", values: None },
};

pub static PHOTOSHOP_MAIN_COPYRIGHTFLAG_VALUES: &[(i64, &str)] = &[
    (0, "False"),
    (1, "True"),
];

/// Photoshop::PixelInfo tags
pub static PHOTOSHOP_PIXELINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    4u16 => TagDef { name: "PixelAspectRatio", values: None },
};

/// Photoshop::PrintScaleInfo tags
pub static PHOTOSHOP_PRINTSCALEINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "PrintStyle", values: Some(PHOTOSHOP_PRINTSCALEINFO_PRINTSTYLE_VALUES) },
    10u16 => TagDef { name: "PrintScale", values: None },
    2u16 => TagDef { name: "PrintPosition", values: None },
};

pub static PHOTOSHOP_PRINTSCALEINFO_PRINTSTYLE_VALUES: &[(i64, &str)] = &[
    (0, "Centered"),
    (1, "Size to Fit"),
    (2, "User Defined"),
];

/// Photoshop::Resolution tags
pub static PHOTOSHOP_RESOLUTION: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "XResolution", values: None },
    2u16 => TagDef { name: "DisplayedUnitsX", values: Some(PHOTOSHOP_RESOLUTION_DISPLAYEDUNITSX_VALUES) },
    4u16 => TagDef { name: "YResolution", values: None },
    6u16 => TagDef { name: "DisplayedUnitsY", values: Some(PHOTOSHOP_RESOLUTION_DISPLAYEDUNITSY_VALUES) },
};

pub static PHOTOSHOP_RESOLUTION_DISPLAYEDUNITSX_VALUES: &[(i64, &str)] = &[
    (1, "inches"),
    (2, "cm"),
];

pub static PHOTOSHOP_RESOLUTION_DISPLAYEDUNITSY_VALUES: &[(i64, &str)] = &[
    (1, "inches"),
    (2, "cm"),
];

/// Photoshop::SliceInfo tags
pub static PHOTOSHOP_SLICEINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    20u16 => TagDef { name: "SlicesGroupName", values: None },
    24u16 => TagDef { name: "NumSlices", values: None },
};

/// Photoshop::VersionInfo tags
pub static PHOTOSHOP_VERSIONINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    4u16 => TagDef { name: "HasRealMergedData", values: Some(PHOTOSHOP_VERSIONINFO_HASREALMERGEDDATA_VALUES) },
    5u16 => TagDef { name: "WriterName", values: None },
    9u16 => TagDef { name: "ReaderName", values: None },
};

pub static PHOTOSHOP_VERSIONINFO_HASREALMERGEDDATA_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];

/// Sony::SR2Private tags
pub static SONY_SR2PRIVATE: phf::Map<u16, TagDef> = phf::phf_map! {
    29184u16 => TagDef { name: "SR2SubIFDOffset", values: None },
    29185u16 => TagDef { name: "SR2SubIFDLength", values: None },
    29217u16 => TagDef { name: "SR2SubIFDKey", values: None },
    29248u16 => TagDef { name: "IDC_IFD", values: None },
    29249u16 => TagDef { name: "IDC2_IFD", values: None },
    29264u16 => TagDef { name: "MRWInfo", values: None },
};

/// SonyIDC::Main tags
pub static SONYIDC_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    32768u16 => TagDef { name: "IDCCreativeStyle", values: Some(SONYIDC_MAIN_IDCCREATIVESTYLE_VALUES) },
    32769u16 => TagDef { name: "CreativeStyleWasChanged", values: Some(SONYIDC_MAIN_CREATIVESTYLEWASCHANGED_VALUES) },
    32770u16 => TagDef { name: "PresetWhiteBalance", values: Some(SONYIDC_MAIN_PRESETWHITEBALANCE_VALUES) },
    32787u16 => TagDef { name: "ColorTemperatureAdj", values: None },
    32788u16 => TagDef { name: "PresetWhiteBalanceAdj", values: None },
    32789u16 => TagDef { name: "ColorCorrection", values: None },
    32790u16 => TagDef { name: "SaturationAdj", values: None },
    32791u16 => TagDef { name: "ContrastAdj", values: None },
    32792u16 => TagDef { name: "BrightnessAdj", values: None },
    32793u16 => TagDef { name: "HueAdj", values: None },
    32794u16 => TagDef { name: "SharpnessAdj", values: None },
    32795u16 => TagDef { name: "SharpnessOvershoot", values: None },
    32796u16 => TagDef { name: "SharpnessUndershoot", values: None },
    32797u16 => TagDef { name: "SharpnessThreshold", values: None },
    32798u16 => TagDef { name: "NoiseReductionMode", values: Some(SONYIDC_MAIN_NOISEREDUCTIONMODE_VALUES) },
    32801u16 => TagDef { name: "GrayPoint", values: None },
    32802u16 => TagDef { name: "D-RangeOptimizerMode", values: Some(SONYIDC_MAIN_D_RANGEOPTIMIZERMODE_VALUES) },
    32803u16 => TagDef { name: "D-RangeOptimizerValue", values: None },
    32804u16 => TagDef { name: "D-RangeOptimizerHighlight", values: None },
    32806u16 => TagDef { name: "HighlightColorDistortReduct", values: Some(SONYIDC_MAIN_HIGHLIGHTCOLORDISTORTREDUCT_VALUES) },
    32807u16 => TagDef { name: "NoiseReductionValue", values: None },
    32808u16 => TagDef { name: "EdgeNoiseReduction", values: None },
    32809u16 => TagDef { name: "ColorNoiseReduction", values: None },
    32813u16 => TagDef { name: "D-RangeOptimizerShadow", values: None },
    32816u16 => TagDef { name: "PeripheralIllumCentralRadius", values: None },
    32817u16 => TagDef { name: "PeripheralIllumCentralValue", values: None },
    32818u16 => TagDef { name: "PeripheralIllumPeriphValue", values: None },
    32832u16 => TagDef { name: "DistortionCompensation", values: Some(SONYIDC_MAIN_DISTORTIONCOMPENSATION_VALUES) },
    36864u16 => TagDef { name: "ToneCurveBrightnessX", values: None },
    36865u16 => TagDef { name: "ToneCurveRedX", values: None },
    36866u16 => TagDef { name: "ToneCurveGreenX", values: None },
    36867u16 => TagDef { name: "ToneCurveBlueX", values: None },
    36868u16 => TagDef { name: "ToneCurveBrightnessY", values: None },
    36869u16 => TagDef { name: "ToneCurveRedY", values: None },
    36870u16 => TagDef { name: "ToneCurveGreenY", values: None },
    36871u16 => TagDef { name: "ToneCurveBlueY", values: None },
    36877u16 => TagDef { name: "ChromaticAberrationCorrection", values: Some(SONYIDC_MAIN_CHROMATICABERRATIONCORRECTION_VALUES) },
    36878u16 => TagDef { name: "InclinationCorrection", values: Some(SONYIDC_MAIN_INCLINATIONCORRECTION_VALUES) },
    36879u16 => TagDef { name: "InclinationAngle", values: None },
    36880u16 => TagDef { name: "Cropping", values: Some(SONYIDC_MAIN_CROPPING_VALUES) },
    36881u16 => TagDef { name: "CropArea", values: None },
    36882u16 => TagDef { name: "PreviewImageSize", values: None },
    36883u16 => TagDef { name: "PxShiftPeriphEdgeNR", values: Some(SONYIDC_MAIN_PXSHIFTPERIPHEDGENR_VALUES) },
    36884u16 => TagDef { name: "PxShiftPeriphEdgeNRValue", values: None },
    36887u16 => TagDef { name: "WhitesAdj", values: None },
    36888u16 => TagDef { name: "BlacksAdj", values: None },
    36889u16 => TagDef { name: "HighlightsAdj", values: None },
    36890u16 => TagDef { name: "ShadowsAdj", values: None },
    513u16 => TagDef { name: "IDCPreviewStart", values: None },
    514u16 => TagDef { name: "IDCPreviewLength", values: None },
    53248u16 => TagDef { name: "CurrentVersion", values: None },
    53249u16 => TagDef { name: "VersionIFD", values: None },
    53504u16 => TagDef { name: "VersionCreateDate", values: None },
    53505u16 => TagDef { name: "VersionModifyDate", values: None },
};

pub static SONYIDC_MAIN_IDCCREATIVESTYLE_VALUES: &[(i64, &str)] = &[
    (1, "Camera Setting"),
    (10, "Clear"),
    (11, "Deep"),
    (12, "Light"),
    (13, "Sunset"),
    (14, "Night View"),
    (15, "Autumn Leaves"),
    (16, "B&W"),
    (17, "Sepia"),
    (2, "Standard"),
    (3, "Real"),
    (4, "Vivid"),
    (5, "Adobe RGB"),
    (6, "A100 Standard"),
    (7, "Neutral"),
    (8, "Portrait"),
    (9, "Landscape"),
];

pub static SONYIDC_MAIN_CREATIVESTYLEWASCHANGED_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];

pub static SONYIDC_MAIN_PRESETWHITEBALANCE_VALUES: &[(i64, &str)] = &[
    (1, "Camera Setting"),
    (10, "Warm White Fluorescent"),
    (11, "Tungsten"),
    (12, "Flash"),
    (13, "Auto"),
    (2, "Color Temperature"),
    (3, "Specify Gray Point"),
    (4, "Daylight"),
    (5, "Cloudy"),
    (6, "Shade"),
    (7, "Cool White Fluorescent"),
    (8, "Day Light Fluorescent"),
    (9, "Day White Fluorescent"),
];

pub static SONYIDC_MAIN_NOISEREDUCTIONMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static SONYIDC_MAIN_D_RANGEOPTIMIZERMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Auto"),
    (2, "Manual"),
];

pub static SONYIDC_MAIN_HIGHLIGHTCOLORDISTORTREDUCT_VALUES: &[(i64, &str)] = &[
    (0, "Standard"),
    (1, "Advanced"),
];

pub static SONYIDC_MAIN_DISTORTIONCOMPENSATION_VALUES: &[(i64, &str)] = &[
    (-1, "n/a"),
    (1, "On"),
    (2, "Off"),
];

pub static SONYIDC_MAIN_CHROMATICABERRATIONCORRECTION_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static SONYIDC_MAIN_INCLINATIONCORRECTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static SONYIDC_MAIN_CROPPING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static SONYIDC_MAIN_PXSHIFTPERIPHEDGENR_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];


/// Look up a tag by ID in the main table.
pub fn lookup(_tag_id: u16) -> Option<&'static TagDef> {
    // Default to main table - override in specific modules
    None
}

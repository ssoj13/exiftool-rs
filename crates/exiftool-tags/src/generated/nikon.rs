//! Nikon MakerNotes tag definitions.
//! Auto-generated from ExifTool - DO NOT EDIT

#![allow(dead_code)]

/// Tag definition with name and optional value mappings.
#[derive(Debug, Clone)]
pub struct TagDef {
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}

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

/// Nikon::AFInfo tags
pub static NIKON_AFINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "AFAreaMode", values: Some(NIKON_AFINFO_AFAREAMODE_VALUES) },
    1u16 => TagDef { name: "AFPoint", values: Some(NIKON_AFINFO_AFPOINT_VALUES) },
    2u16 => TagDef { name: "AFPointsInFocus", values: Some(NIKON_AFINFO_AFPOINTSINFOCUS_VALUES) },
};

pub static NIKON_AFINFO_AFAREAMODE_VALUES: &[(i64, &str)] = &[
    (0, "Single Area"),
    (1, "Dynamic Area"),
    (2, "Dynamic Area (closest subject)"),
    (3, "Group Dynamic"),
    (4, "Single Area (wide)"),
    (5, "Dynamic Area (wide)"),
];

pub static NIKON_AFINFO_AFPOINT_VALUES: &[(i64, &str)] = &[
    (0, "Center"),
    (1, "Top"),
    (10, "Far Right"),
    (2, "Bottom"),
    (3, "Mid-left"),
    (4, "Mid-right"),
    (5, "Upper-left"),
    (6, "Upper-right"),
    (7, "Lower-left"),
    (8, "Lower-right"),
    (9, "Far Left"),
];

pub static NIKON_AFINFO_AFPOINTSINFOCUS_VALUES: &[(i64, &str)] = &[
    (0, "(none)"),
    (2047, "All 11 Points"),
];

/// Nikon::AFInfo2V0100 tags
pub static NIKON_AFINFO2V0100: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "AFInfo2Version", values: None },
    16u16 => TagDef { name: "AFImageWidth", values: None },
    18u16 => TagDef { name: "AFImageHeight", values: None },
    20u16 => TagDef { name: "AFAreaXPosition", values: None },
    22u16 => TagDef { name: "AFAreaYPosition", values: None },
    24u16 => TagDef { name: "AFAreaWidth", values: None },
    26u16 => TagDef { name: "AFAreaHeight", values: None },
    28u16 => TagDef { name: "ContrastDetectAFInFocus", values: Some(NIKON_AFINFO2V0100_CONTRASTDETECTAFINFOCUS_VALUES) },
    4u16 => TagDef { name: "AFDetectionMethod", values: Some(NIKON_AFINFO2V0100_AFDETECTIONMETHOD_VALUES) },
    5u16 => TagDef { name: "AFAreaMode", values: Some(NIKON_AFINFO2V0100_AFAREAMODE_VALUES) },
    6u16 => TagDef { name: "FocusPointSchema", values: Some(NIKON_AFINFO2V0100_FOCUSPOINTSCHEMA_VALUES) },
    7u16 => TagDef { name: "PrimaryAFPoint", values: Some(NIKON_AFINFO2V0100_PRIMARYAFPOINT_VALUES) },
    8u16 => TagDef { name: "AFPointsUsed", values: None },
};

pub static NIKON_AFINFO2V0100_CONTRASTDETECTAFINFOCUS_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];

pub static NIKON_AFINFO2V0100_AFDETECTIONMETHOD_VALUES: &[(i64, &str)] = &[
    (0, "Phase Detect"),
    (1, "Contrast Detect"),
    (2, "Hybrid"),
];

pub static NIKON_AFINFO2V0100_AFAREAMODE_VALUES: &[(i64, &str)] = &[
    (0, "Single Area"),
    (1, "Dynamic Area"),
    (10, "Single Area (wide)"),
    (11, "Dynamic Area (wide)"),
    (12, "Dynamic Area (wide, 3D-tracking)"),
    (128, "Single"),
    (129, "Auto (41 points)"),
    (13, "Group Area"),
    (130, "Subject Tracking (41 points)"),
    (131, "Face Priority (41 points)"),
    (14, "Dynamic Area (25 points)"),
    (15, "Dynamic Area (72 points)"),
    (16, "Group Area (HL)"),
    (17, "Group Area (VL)"),
    (18, "Dynamic Area (49 points)"),
    (192, "Pinpoint"),
    (193, "Single"),
    (194, "Dynamic"),
    (195, "Wide (S)"),
    (196, "Wide (L)"),
    (197, "Auto"),
    (199, "Auto"),
    (2, "Dynamic Area (closest subject)"),
    (3, "Group Dynamic"),
    (4, "Dynamic Area (9 points)"),
    (5, "Dynamic Area (21 points)"),
    (6, "Dynamic Area (51 points)"),
    (7, "Dynamic Area (51 points, 3D-tracking)"),
    (8, "Auto-area"),
    (9, "Dynamic Area (3D-tracking)"),
];

pub static NIKON_AFINFO2V0100_FOCUSPOINTSCHEMA_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "51-point"),
    (2, "11-point"),
    (3, "39-point"),
];

pub static NIKON_AFINFO2V0100_PRIMARYAFPOINT_VALUES: &[(i64, &str)] = &[
    (0, "(none)"),
    (1, "C6 (Center)"),
    (10, "E6"),
    (11, "C5"),
    (12, "B5"),
    (13, "A4"),
    (14, "D5"),
    (15, "E4"),
    (16, "C8"),
    (17, "B8"),
    (18, "A7"),
    (19, "D8"),
    (2, "B6"),
    (20, "E7"),
    (21, "C9"),
    (22, "B9"),
    (23, "A8"),
    (24, "D9"),
    (25, "E8"),
    (26, "C10"),
    (27, "B10"),
    (28, "A9"),
    (29, "D10"),
    (3, "A5"),
    (30, "E9"),
    (31, "C11"),
    (32, "B11"),
    (33, "D11"),
    (34, "C4"),
    (35, "B4"),
    (36, "A3"),
    (37, "D4"),
    (38, "E3"),
    (39, "C3"),
    (4, "D6"),
    (40, "B3"),
    (41, "A2"),
    (42, "D3"),
    (43, "E2"),
    (44, "C2"),
    (45, "B2"),
    (46, "A1"),
    (47, "D2"),
    (48, "E1"),
    (49, "C1"),
    (5, "E5"),
    (50, "B1"),
    (51, "D1"),
    (6, "C7"),
    (7, "B7"),
    (8, "A6"),
    (9, "D7"),
];

/// Nikon::AFTune tags
pub static NIKON_AFTUNE: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "AFFineTune", values: Some(NIKON_AFTUNE_AFFINETUNE_VALUES) },
    1u16 => TagDef { name: "AFFineTuneIndex", values: None },
    2u16 => TagDef { name: "AFFineTuneAdj", values: None },
    3u16 => TagDef { name: "AFFineTuneAdjTele", values: None },
};

pub static NIKON_AFTUNE_AFFINETUNE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On (1)"),
    (2, "On (2)"),
    (3, "On (Zoom)"),
];

/// Nikon::BarometerInfo tags
pub static NIKON_BAROMETERINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "BarometerInfoVersion", values: None },
    6u16 => TagDef { name: "Altitude", values: None },
};

/// Nikon::ColorBalance1 tags
pub static NIKON_COLORBALANCE1: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "WB_RBGGLevels", values: None },
};

/// Nikon::ColorBalanceA tags
pub static NIKON_COLORBALANCEA: phf::Map<u16, TagDef> = phf::phf_map! {
    624u16 => TagDef { name: "WB_RBLevels", values: None },
    626u16 => TagDef { name: "WB_RBLevelsAuto", values: None },
    628u16 => TagDef { name: "WB_RBLevelsDaylight", values: None },
    642u16 => TagDef { name: "WB_RBLevelsIncandescent", values: None },
    656u16 => TagDef { name: "WB_RBLevelsFluorescent", values: None },
    662u16 => TagDef { name: "WB_RBLevelsCloudy", values: None },
    676u16 => TagDef { name: "WB_RBLevelsFlash", values: None },
    690u16 => TagDef { name: "WB_RBLevelsShade", values: None },
};

/// Nikon::DistortInfo tags
pub static NIKON_DISTORTINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "DistortionVersion", values: None },
    4u16 => TagDef { name: "AutoDistortionControl", values: Some(NIKON_DISTORTINFO_AUTODISTORTIONCONTROL_VALUES) },
};

pub static NIKON_DISTORTINFO_AUTODISTORTIONCONTROL_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
    (2, "On (underwater)"),
];

/// Nikon::FaceDetect tags
pub static NIKON_FACEDETECT: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "FaceDetectFrameSize", values: None },
    12u16 => TagDef { name: "Face3Position", values: None },
    16u16 => TagDef { name: "Face4Position", values: None },
    20u16 => TagDef { name: "Face5Position", values: None },
    24u16 => TagDef { name: "Face6Position", values: None },
    28u16 => TagDef { name: "Face7Position", values: None },
    3u16 => TagDef { name: "FacesDetected", values: None },
    32u16 => TagDef { name: "Face8Position", values: None },
    36u16 => TagDef { name: "Face9Position", values: None },
    4u16 => TagDef { name: "Face1Position", values: None },
    40u16 => TagDef { name: "Face10Position", values: None },
    44u16 => TagDef { name: "Face11Position", values: None },
    48u16 => TagDef { name: "Face12Position", values: None },
    8u16 => TagDef { name: "Face2Position", values: None },
};

/// Nikon::FileInfo tags
pub static NIKON_FILEINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "FileInfoVersion", values: None },
    3u16 => TagDef { name: "DirectoryNumber", values: None },
    4u16 => TagDef { name: "FileNumber", values: None },
};

/// Nikon::FlashInfo0100 tags
pub static NIKON_FLASHINFO0100: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "FlashInfoVersion", values: None },
    10u16 => TagDef { name: "FlashOutput", values: None },
    11u16 => TagDef { name: "FlashFocalLength", values: None },
    12u16 => TagDef { name: "RepeatingFlashRate", values: None },
    13u16 => TagDef { name: "RepeatingFlashCount", values: None },
    14u16 => TagDef { name: "FlashGNDistance", values: Some(NIKON_FLASHINFO0100_FLASHGNDISTANCE_VALUES) },
    15u16 => TagDef { name: "FlashGroupAControlMode", values: Some(NIKON_FLASHINFO0100_FLASHGROUPACONTROLMODE_VALUES) },
    16u16 => TagDef { name: "FlashGroupBControlMode", values: Some(NIKON_FLASHINFO0100_FLASHGROUPBCONTROLMODE_VALUES) },
    17u16 => TagDef { name: "FlashGroupAOutput", values: None },
    18u16 => TagDef { name: "FlashGroupBOutput", values: None },
    4u16 => TagDef { name: "FlashSource", values: Some(NIKON_FLASHINFO0100_FLASHSOURCE_VALUES) },
    6u16 => TagDef { name: "ExternalFlashFirmware", values: None },
    8u16 => TagDef { name: "ExternalFlashFlags", values: Some(NIKON_FLASHINFO0100_EXTERNALFLASHFLAGS_VALUES) },
};

pub static NIKON_FLASHINFO0100_FLASHGNDISTANCE_VALUES: &[(i64, &str)] = &[
    (0, "0"),
    (1, "0.1 m"),
    (10, "1.0 m"),
    (11, "1.1 m"),
    (12, "1.3 m"),
    (13, "1.4 m"),
    (14, "1.6 m"),
    (15, "1.8 m"),
    (16, "2.0 m"),
    (17, "2.2 m"),
    (18, "2.5 m"),
    (19, "2.8 m"),
    (2, "0.2 m"),
    (20, "3.2 m"),
    (21, "3.6 m"),
    (22, "4.0 m"),
    (23, "4.5 m"),
    (24, "5.0 m"),
    (25, "5.6 m"),
    (255, "n/a"),
    (26, "6.3 m"),
    (27, "7.1 m"),
    (28, "8.0 m"),
    (29, "9.0 m"),
    (3, "0.3 m"),
    (30, "10.0 m"),
    (31, "11.0 m"),
    (32, "13.0 m"),
    (33, "14.0 m"),
    (34, "16.0 m"),
    (35, "18.0 m"),
    (36, "20.0 m"),
    (4, "0.4 m"),
    (5, "0.5 m"),
    (6, "0.6 m"),
    (7, "0.7 m"),
    (8, "0.8 m"),
    (9, "0.9 m"),
];

pub static NIKON_FLASHINFO0100_FLASHGROUPACONTROLMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "iTTL-BL"),
    (2, "iTTL"),
    (3, "Auto Aperture"),
    (4, "Automatic"),
    (5, "GN (distance priority)"),
    (6, "Manual"),
    (7, "Repeating Flash"),
];

pub static NIKON_FLASHINFO0100_FLASHGROUPBCONTROLMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "iTTL-BL"),
    (2, "iTTL"),
    (3, "Auto Aperture"),
    (4, "Automatic"),
    (5, "GN (distance priority)"),
    (6, "Manual"),
    (7, "Repeating Flash"),
];

pub static NIKON_FLASHINFO0100_FLASHSOURCE_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "External"),
    (2, "Internal"),
];

pub static NIKON_FLASHINFO0100_EXTERNALFLASHFLAGS_VALUES: &[(i64, &str)] = &[
    (0, "(none)"),
];

/// Nikon::GEM tags
pub static NIKON_GEM: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "DigitalGEM", values: None },
};

/// Nikon::HDRInfo tags
pub static NIKON_HDRINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "HDRInfoVersion", values: None },
    4u16 => TagDef { name: "HDR", values: Some(NIKON_HDRINFO_HDR_VALUES) },
    5u16 => TagDef { name: "HDRLevel", values: Some(NIKON_HDRINFO_HDRLEVEL_VALUES) },
    6u16 => TagDef { name: "HDRSmoothing", values: Some(NIKON_HDRINFO_HDRSMOOTHING_VALUES) },
    7u16 => TagDef { name: "HDRLevel2", values: Some(NIKON_HDRINFO_HDRLEVEL2_VALUES) },
};

pub static NIKON_HDRINFO_HDR_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On (normal)"),
    (48, "Auto"),
];

pub static NIKON_HDRINFO_HDRLEVEL_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "1 EV"),
    (2, "2 EV"),
    (255, "n/a"),
    (3, "3 EV"),
];

pub static NIKON_HDRINFO_HDRSMOOTHING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Normal"),
    (2, "Low"),
    (255, "n/a"),
    (3, "High"),
    (48, "Auto"),
];

pub static NIKON_HDRINFO_HDRLEVEL2_VALUES: &[(i64, &str)] = &[
    (0, "Auto"),
    (1, "1 EV"),
    (2, "2 EV"),
    (255, "n/a"),
    (3, "3 EV"),
];

/// Nikon::ISOInfo tags
pub static NIKON_ISOINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "ISO", values: None },
    10u16 => TagDef { name: "ISOExpansion2", values: Some(NIKON_ISOINFO_ISOEXPANSION2_VALUES) },
    4u16 => TagDef { name: "ISOExpansion", values: Some(NIKON_ISOINFO_ISOEXPANSION_VALUES) },
    6u16 => TagDef { name: "ISO2", values: None },
};

pub static NIKON_ISOINFO_ISOEXPANSION2_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (257, "Hi 0.3"),
    (258, "Hi 0.5"),
    (259, "Hi 0.7"),
    (260, "Hi 1.0"),
    (261, "Hi 1.3"),
    (262, "Hi 1.5"),
    (263, "Hi 1.7"),
    (264, "Hi 2.0"),
    (513, "Lo 0.3"),
    (514, "Lo 0.5"),
    (515, "Lo 0.7"),
    (516, "Lo 1.0"),
];

pub static NIKON_ISOINFO_ISOEXPANSION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (257, "Hi 0.3"),
    (258, "Hi 0.5"),
    (259, "Hi 0.7"),
    (260, "Hi 1.0"),
    (261, "Hi 1.3"),
    (262, "Hi 1.5"),
    (263, "Hi 1.7"),
    (264, "Hi 2.0"),
    (265, "Hi 2.3"),
    (266, "Hi 2.5"),
    (267, "Hi 2.7"),
    (268, "Hi 3.0"),
    (269, "Hi 3.3"),
    (270, "Hi 3.5"),
    (271, "Hi 3.7"),
    (272, "Hi 4.0"),
    (273, "Hi 4.3"),
    (274, "Hi 4.5"),
    (275, "Hi 4.7"),
    (276, "Hi 5.0"),
    (513, "Lo 0.3"),
    (514, "Lo 0.5"),
    (515, "Lo 0.7"),
    (516, "Lo 1.0"),
];

/// Nikon::LensData00 tags
pub static NIKON_LENSDATA00: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "LensDataVersion", values: None },
    10u16 => TagDef { name: "MaxApertureAtMinFocal", values: None },
    11u16 => TagDef { name: "MaxApertureAtMaxFocal", values: None },
    6u16 => TagDef { name: "LensIDNumber", values: None },
    7u16 => TagDef { name: "LensFStops", values: None },
    8u16 => TagDef { name: "MinFocalLength", values: None },
    9u16 => TagDef { name: "MaxFocalLength", values: None },
};

/// Nikon::LocationInfo tags
pub static NIKON_LOCATIONINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "LocationInfoVersion", values: None },
    4u16 => TagDef { name: "TextEncoding", values: Some(NIKON_LOCATIONINFO_TEXTENCODING_VALUES) },
    5u16 => TagDef { name: "CountryCode", values: None },
    9u16 => TagDef { name: "Location", values: None },
};

pub static NIKON_LOCATIONINFO_TEXTENCODING_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "UTF8"),
    (2, "UTF16"),
];

/// Nikon::Main tags
pub static NIKON_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "MakerNoteVersion", values: None },
    11u16 => TagDef { name: "WhiteBalanceFineTune", values: None },
    12u16 => TagDef { name: "WB_RBLevels", values: None },
    128u16 => TagDef { name: "ImageAdjustment", values: None },
    129u16 => TagDef { name: "ToneComp", values: None },
    13u16 => TagDef { name: "ProgramShift", values: None },
    130u16 => TagDef { name: "AuxiliaryLens", values: None },
    131u16 => TagDef { name: "LensType", values: None },
    132u16 => TagDef { name: "Lens", values: None },
    133u16 => TagDef { name: "ManualFocusDistance", values: None },
    134u16 => TagDef { name: "DigitalZoom", values: None },
    135u16 => TagDef { name: "FlashMode", values: Some(NIKON_MAIN_FLASHMODE_VALUES) },
    136u16 => TagDef { name: "AFInfo", values: None },
    137u16 => TagDef { name: "ShootingMode", values: None },
    139u16 => TagDef { name: "LensFStops", values: None },
    14u16 => TagDef { name: "ExposureDifference", values: None },
    140u16 => TagDef { name: "ContrastCurve", values: None },
    141u16 => TagDef { name: "ColorHue", values: None },
    143u16 => TagDef { name: "SceneMode", values: None },
    144u16 => TagDef { name: "LightSource", values: None },
    145u16 => TagDef { name: "ShotInfoD40", values: None },
    146u16 => TagDef { name: "HueAdjustment", values: None },
    147u16 => TagDef { name: "NEFCompression", values: Some(NIKON_MAIN_NEFCOMPRESSION_VALUES) },
    148u16 => TagDef { name: "SaturationAdj", values: None },
    149u16 => TagDef { name: "NoiseReduction", values: None },
    15u16 => TagDef { name: "ISOSelection", values: None },
    150u16 => TagDef { name: "NEFLinearizationTable", values: None },
    151u16 => TagDef { name: "ColorBalance0100", values: None },
    152u16 => TagDef { name: "LensData0100", values: None },
    153u16 => TagDef { name: "RawImageCenter", values: None },
    154u16 => TagDef { name: "SensorPixelSize", values: None },
    156u16 => TagDef { name: "SceneAssist", values: None },
    157u16 => TagDef { name: "DateStampMode", values: Some(NIKON_MAIN_DATESTAMPMODE_VALUES) },
    158u16 => TagDef { name: "RetouchHistory", values: None },
    16u16 => TagDef { name: "DataDump", values: None },
    160u16 => TagDef { name: "SerialNumber", values: None },
    162u16 => TagDef { name: "ImageDataSize", values: None },
    165u16 => TagDef { name: "ImageCount", values: None },
    166u16 => TagDef { name: "DeletedImageCount", values: None },
    167u16 => TagDef { name: "ShutterCount", values: None },
    168u16 => TagDef { name: "FlashInfo0100", values: None },
    169u16 => TagDef { name: "ImageOptimization", values: None },
    17u16 => TagDef { name: "PreviewIFD", values: None },
    170u16 => TagDef { name: "Saturation", values: None },
    171u16 => TagDef { name: "VariProgram", values: None },
    172u16 => TagDef { name: "ImageStabilization", values: None },
    173u16 => TagDef { name: "AFResponse", values: None },
    176u16 => TagDef { name: "MultiExposure", values: None },
    177u16 => TagDef { name: "HighISONoiseReduction", values: Some(NIKON_MAIN_HIGHISONOISEREDUCTION_VALUES) },
    179u16 => TagDef { name: "ToningEffect", values: None },
    18u16 => TagDef { name: "FlashExposureComp", values: None },
    182u16 => TagDef { name: "PowerUpTime", values: None },
    183u16 => TagDef { name: "AFInfo2", values: None },
    184u16 => TagDef { name: "FileInfo", values: None },
    185u16 => TagDef { name: "AFTune", values: None },
    187u16 => TagDef { name: "RetouchInfo", values: None },
    189u16 => TagDef { name: "PictureControlData", values: None },
    19u16 => TagDef { name: "ISOSetting", values: None },
    191u16 => TagDef { name: "SilentPhotography", values: Some(NIKON_MAIN_SILENTPHOTOGRAPHY_VALUES) },
    195u16 => TagDef { name: "BarometerInfo", values: None },
    2u16 => TagDef { name: "ISO", values: None },
    20u16 => TagDef { name: "ColorBalanceA", values: None },
    22u16 => TagDef { name: "ImageBoundary", values: None },
    23u16 => TagDef { name: "ExternalFlashExposureComp", values: None },
    24u16 => TagDef { name: "FlashExposureBracketValue", values: None },
    25u16 => TagDef { name: "ExposureBracketValue", values: None },
    26u16 => TagDef { name: "ImageProcessing", values: None },
    27u16 => TagDef { name: "CropHiSpeed", values: Some(NIKON_MAIN_CROPHISPEED_VALUES) },
    28u16 => TagDef { name: "ExposureTuning", values: None },
    29u16 => TagDef { name: "SerialNumber", values: None },
    3u16 => TagDef { name: "ColorMode", values: None },
    30u16 => TagDef { name: "ColorSpace", values: Some(NIKON_MAIN_COLORSPACE_VALUES) },
    31u16 => TagDef { name: "VRInfo", values: None },
    32u16 => TagDef { name: "ImageAuthentication", values: Some(NIKON_MAIN_IMAGEAUTHENTICATION_VALUES) },
    33u16 => TagDef { name: "FaceDetect", values: None },
    34u16 => TagDef { name: "ActiveD-Lighting", values: Some(NIKON_MAIN_ACTIVED_LIGHTING_VALUES) },
    35u16 => TagDef { name: "PictureControlData", values: None },
    3584u16 => TagDef { name: "PrintIM", values: None },
    3585u16 => TagDef { name: "NikonCaptureData", values: None },
    3593u16 => TagDef { name: "NikonCaptureVersion", values: None },
    3598u16 => TagDef { name: "NikonCaptureOffsets", values: None },
    36u16 => TagDef { name: "WorldTime", values: None },
    3600u16 => TagDef { name: "NikonScanIFD", values: None },
    3603u16 => TagDef { name: "NikonCaptureEditVersions", values: None },
    3613u16 => TagDef { name: "NikonICCProfile", values: None },
    3614u16 => TagDef { name: "NikonCaptureOutput", values: None },
    3618u16 => TagDef { name: "NEFBitDepth", values: None },
    37u16 => TagDef { name: "ISOInfo", values: None },
    4u16 => TagDef { name: "Quality", values: None },
    42u16 => TagDef { name: "VignetteControl", values: Some(NIKON_MAIN_VIGNETTECONTROL_VALUES) },
    43u16 => TagDef { name: "DistortInfo", values: None },
    44u16 => TagDef { name: "UnknownInfo", values: None },
    5u16 => TagDef { name: "WhiteBalance", values: None },
    50u16 => TagDef { name: "UnknownInfo2", values: None },
    52u16 => TagDef { name: "ShutterMode", values: Some(NIKON_MAIN_SHUTTERMODE_VALUES) },
    53u16 => TagDef { name: "HDRInfo", values: None },
    55u16 => TagDef { name: "MechanicalShutterCount", values: None },
    57u16 => TagDef { name: "LocationInfo", values: None },
    6u16 => TagDef { name: "Sharpness", values: None },
    61u16 => TagDef { name: "BlackLevel", values: None },
    62u16 => TagDef { name: "ImageSizeRAW", values: Some(NIKON_MAIN_IMAGESIZERAW_VALUES) },
    63u16 => TagDef { name: "WhiteBalanceFineTune", values: None },
    68u16 => TagDef { name: "JPGCompression", values: Some(NIKON_MAIN_JPGCOMPRESSION_VALUES) },
    69u16 => TagDef { name: "CropArea", values: None },
    7u16 => TagDef { name: "FocusMode", values: None },
    78u16 => TagDef { name: "NikonSettings", values: None },
    79u16 => TagDef { name: "ColorTemperatureAuto", values: None },
    8u16 => TagDef { name: "FlashSetting", values: None },
    81u16 => TagDef { name: "MakerNotes0x51", values: None },
    86u16 => TagDef { name: "MakerNotes0x56", values: None },
    9u16 => TagDef { name: "FlashType", values: None },
};

pub static NIKON_MAIN_FLASHMODE_VALUES: &[(i64, &str)] = &[
    (0, "Did Not Fire"),
    (1, "Fired, Manual"),
    (18, "LED Light"),
    (3, "Not Ready"),
    (7, "Fired, External"),
    (8, "Fired, Commander Mode"),
    (9, "Fired, TTL Mode"),
];

pub static NIKON_MAIN_NEFCOMPRESSION_VALUES: &[(i64, &str)] = &[
    (1, "Lossy (type 1)"),
    (10, "Packed 14 bits"),
    (13, "High Efficiency"),
    (14, "High Efficiency*"),
    (2, "Uncompressed"),
    (3, "Lossless"),
    (4, "Lossy (type 2)"),
    (5, "Striped packed 12 bits"),
    (6, "Uncompressed (reduced to 12 bit)"),
    (7, "Unpacked 12 bits"),
    (8, "Small"),
    (9, "Packed 12 bits"),
];

pub static NIKON_MAIN_DATESTAMPMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Date & Time"),
    (2, "Date"),
    (3, "Date Counter"),
];

pub static NIKON_MAIN_HIGHISONOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Minimal"),
    (2, "Low"),
    (3, "Medium Low"),
    (4, "Normal"),
    (5, "Medium High"),
    (6, "High"),
];

pub static NIKON_MAIN_SILENTPHOTOGRAPHY_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKON_MAIN_CROPHISPEED_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "1.3x Crop"),
    (10, "1.3x Movie Crop"),
    (11, "FX Uncropped"),
    (12, "DX Uncropped"),
    (13, "2.8x Movie Crop"),
    (14, "1.4x Movie Crop"),
    (15, "1.5x Movie Crop"),
    (17, "FX 1:1 Crop"),
    (18, "DX 1:1 Crop"),
    (2, "DX Crop"),
    (3, "5:4 Crop"),
    (4, "3:2 Crop"),
    (6, "16:9 Crop"),
    (8, "2.7x Crop"),
    (9, "DX Movie 16:9 Crop"),
];

pub static NIKON_MAIN_COLORSPACE_VALUES: &[(i64, &str)] = &[
    (1, "sRGB"),
    (2, "Adobe RGB"),
    (4, "BT.2100"),
];

pub static NIKON_MAIN_IMAGEAUTHENTICATION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKON_MAIN_ACTIVED_LIGHTING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Low"),
    (10, "Extra High 3"),
    (11, "Extra High 4"),
    (3, "Normal"),
    (5, "High"),
    (65535, "Auto"),
    (7, "Extra High"),
    (8, "Extra High 1"),
    (9, "Extra High 2"),
];

pub static NIKON_MAIN_VIGNETTECONTROL_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Low"),
    (3, "Normal"),
    (5, "High"),
];

pub static NIKON_MAIN_SHUTTERMODE_VALUES: &[(i64, &str)] = &[
    (0, "Mechanical"),
    (16, "Electronic"),
    (48, "Electronic Front Curtain"),
    (64, "Electronic (Movie)"),
    (80, "Auto (Mechanical)"),
    (81, "Auto (Electronic Front Curtain)"),
    (96, "Electronic (High Speed)"),
];

pub static NIKON_MAIN_IMAGESIZERAW_VALUES: &[(i64, &str)] = &[
    (1, "Large"),
    (2, "Medium"),
    (3, "Small"),
];

pub static NIKON_MAIN_JPGCOMPRESSION_VALUES: &[(i64, &str)] = &[
    (1, "Size Priority"),
    (3, "Optimal Quality"),
];

/// Nikon::MakerNotes0x51 tags
pub static NIKON_MAKERNOTES0X51: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "FirmwareVersion51", values: None },
    10u16 => TagDef { name: "NEFCompression", values: Some(NIKON_MAKERNOTES0X51_NEFCOMPRESSION_VALUES) },
};

pub static NIKON_MAKERNOTES0X51_NEFCOMPRESSION_VALUES: &[(i64, &str)] = &[
    (1, "Lossy (type 1)"),
    (10, "Packed 14 bits"),
    (13, "High Efficiency"),
    (14, "High Efficiency*"),
    (2, "Uncompressed"),
    (3, "Lossless"),
    (4, "Lossy (type 2)"),
    (5, "Striped packed 12 bits"),
    (6, "Uncompressed (reduced to 12 bit)"),
    (7, "Unpacked 12 bits"),
    (8, "Small"),
    (9, "Packed 12 bits"),
];

/// Nikon::MakerNotes0x56 tags
pub static NIKON_MAKERNOTES0X56: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "FirmwareVersion56", values: None },
    12u16 => TagDef { name: "PixelShiftID", values: None },
    4u16 => TagDef { name: "BurstGroupID", values: None },
};

/// Nikon::MultiExposure tags
pub static NIKON_MULTIEXPOSURE: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "MultiExposureVersion", values: None },
    1u16 => TagDef { name: "MultiExposureMode", values: Some(NIKON_MULTIEXPOSURE_MULTIEXPOSUREMODE_VALUES) },
    3u16 => TagDef { name: "MultiExposureAutoGain", values: Some(NIKON_MULTIEXPOSURE_MULTIEXPOSUREAUTOGAIN_VALUES) },
};

pub static NIKON_MULTIEXPOSURE_MULTIEXPOSUREMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Multiple Exposure"),
    (2, "Image Overlay"),
    (3, "HDR"),
];

pub static NIKON_MULTIEXPOSURE_MULTIEXPOSUREAUTOGAIN_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

/// Nikon::PictureControl tags
pub static NIKON_PICTURECONTROL: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "PictureControlVersion", values: None },
    24u16 => TagDef { name: "PictureControlBase", values: None },
    4u16 => TagDef { name: "PictureControlName", values: None },
    48u16 => TagDef { name: "PictureControlAdjust", values: Some(NIKON_PICTURECONTROL_PICTURECONTROLADJUST_VALUES) },
    49u16 => TagDef { name: "PictureControlQuickAdjust", values: None },
    50u16 => TagDef { name: "Sharpness", values: None },
    51u16 => TagDef { name: "Contrast", values: None },
    52u16 => TagDef { name: "Brightness", values: None },
    53u16 => TagDef { name: "Saturation", values: None },
    54u16 => TagDef { name: "HueAdjustment", values: None },
    55u16 => TagDef { name: "FilterEffect", values: Some(NIKON_PICTURECONTROL_FILTEREFFECT_VALUES) },
    56u16 => TagDef { name: "ToningEffect", values: Some(NIKON_PICTURECONTROL_TONINGEFFECT_VALUES) },
    57u16 => TagDef { name: "ToningSaturation", values: None },
};

pub static NIKON_PICTURECONTROL_PICTURECONTROLADJUST_VALUES: &[(i64, &str)] = &[
    (0, "Default Settings"),
    (1, "Quick Adjust"),
    (2, "Full Control"),
];

pub static NIKON_PICTURECONTROL_FILTEREFFECT_VALUES: &[(i64, &str)] = &[
    (128, "Off"),
    (129, "Yellow"),
    (130, "Orange"),
    (131, "Red"),
    (132, "Green"),
    (255, "n/a"),
];

pub static NIKON_PICTURECONTROL_TONINGEFFECT_VALUES: &[(i64, &str)] = &[
    (128, "B&W"),
    (129, "Sepia"),
    (130, "Cyanotype"),
    (131, "Red"),
    (132, "Yellow"),
    (133, "Green"),
    (134, "Blue-green"),
    (135, "Blue"),
    (136, "Purple-blue"),
    (137, "Red-purple"),
    (255, "n/a"),
];

/// Nikon::PreviewIFD tags
pub static NIKON_PREVIEWIFD: phf::Map<u16, TagDef> = phf::phf_map! {
    254u16 => TagDef { name: "SubfileType", values: Some(NIKON_PREVIEWIFD_SUBFILETYPE_VALUES) },
    259u16 => TagDef { name: "Compression", values: Some(NIKON_PREVIEWIFD_COMPRESSION_VALUES) },
    296u16 => TagDef { name: "ResolutionUnit", values: Some(NIKON_PREVIEWIFD_RESOLUTIONUNIT_VALUES) },
    513u16 => TagDef { name: "PreviewImageStart", values: None },
    514u16 => TagDef { name: "PreviewImageLength", values: None },
    531u16 => TagDef { name: "YCbCrPositioning", values: Some(NIKON_PREVIEWIFD_YCBCRPOSITIONING_VALUES) },
};

pub static NIKON_PREVIEWIFD_SUBFILETYPE_VALUES: &[(i64, &str)] = &[
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

pub static NIKON_PREVIEWIFD_COMPRESSION_VALUES: &[(i64, &str)] = &[
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

pub static NIKON_PREVIEWIFD_RESOLUTIONUNIT_VALUES: &[(i64, &str)] = &[
    (1, "None"),
    (2, "inches"),
    (3, "cm"),
];

pub static NIKON_PREVIEWIFD_YCBCRPOSITIONING_VALUES: &[(i64, &str)] = &[
    (1, "Centered"),
    (2, "Co-sited"),
];

/// Nikon::ROC tags
pub static NIKON_ROC: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "DigitalROC", values: None },
};

/// Nikon::RetouchInfo tags
pub static NIKON_RETOUCHINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "RetouchInfoVersion", values: None },
    5u16 => TagDef { name: "RetouchNEFProcessing", values: Some(NIKON_RETOUCHINFO_RETOUCHNEFPROCESSING_VALUES) },
};

pub static NIKON_RETOUCHINFO_RETOUCHNEFPROCESSING_VALUES: &[(i64, &str)] = &[
    (-1, "Off"),
    (1, "On"),
];

/// Nikon::Scan tags
pub static NIKON_SCAN: phf::Map<u16, TagDef> = phf::phf_map! {
    2u16 => TagDef { name: "FilmType", values: None },
    256u16 => TagDef { name: "DigitalICE", values: None },
    272u16 => TagDef { name: "ROCInfo", values: None },
    288u16 => TagDef { name: "GEMInfo", values: None },
    512u16 => TagDef { name: "DigitalDEEShadowAdj", values: None },
    513u16 => TagDef { name: "DigitalDEEThreshold", values: None },
    514u16 => TagDef { name: "DigitalDEEHighlightAdj", values: None },
    64u16 => TagDef { name: "MultiSample", values: None },
    65u16 => TagDef { name: "BitDepth", values: None },
    80u16 => TagDef { name: "MasterGain", values: None },
    81u16 => TagDef { name: "ColorGain", values: None },
    96u16 => TagDef { name: "ScanImageEnhancer", values: Some(NIKON_SCAN_SCANIMAGEENHANCER_VALUES) },
};

pub static NIKON_SCAN_SCANIMAGEENHANCER_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

/// Nikon::ShotInfoD40 tags
pub static NIKON_SHOTINFOD40: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "ShotInfoVersion", values: None },
    582u16 => TagDef { name: "ShutterCount", values: None },
    729u16 => TagDef { name: "CustomSettingsD40", values: None },
};

/// Nikon::UnknownInfo tags
pub static NIKON_UNKNOWNINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "UnknownInfoVersion", values: None },
};

/// Nikon::UnknownInfo2 tags
pub static NIKON_UNKNOWNINFO2: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "UnknownInfo2Version", values: None },
};

/// Nikon::VRInfo tags
pub static NIKON_VRINFO: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "VRInfoVersion", values: None },
    4u16 => TagDef { name: "VibrationReduction", values: Some(NIKON_VRINFO_VIBRATIONREDUCTION_VALUES) },
    6u16 => TagDef { name: "VRMode", values: Some(NIKON_VRINFO_VRMODE_VALUES) },
    8u16 => TagDef { name: "VRType", values: Some(NIKON_VRINFO_VRTYPE_VALUES) },
};

pub static NIKON_VRINFO_VIBRATIONREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "n/a"),
    (1, "On"),
    (2, "Off"),
];

pub static NIKON_VRINFO_VRMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Normal"),
    (3, "Sport"),
];

pub static NIKON_VRINFO_VRTYPE_VALUES: &[(i64, &str)] = &[
    (2, "In-body"),
    (3, "In-body + Lens"),
];

/// Nikon::WorldTime tags
pub static NIKON_WORLDTIME: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "TimeZone", values: None },
    2u16 => TagDef { name: "DaylightSavings", values: Some(NIKON_WORLDTIME_DAYLIGHTSAVINGS_VALUES) },
    3u16 => TagDef { name: "DateDisplayFormat", values: Some(NIKON_WORLDTIME_DATEDISPLAYFORMAT_VALUES) },
};

pub static NIKON_WORLDTIME_DAYLIGHTSAVINGS_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];

pub static NIKON_WORLDTIME_DATEDISPLAYFORMAT_VALUES: &[(i64, &str)] = &[
    (0, "Y/M/D"),
    (1, "M/D/Y"),
    (2, "D/M/Y"),
];

/// NikonCapture::Brightness tags
pub static NIKONCAPTURE_BRIGHTNESS: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "BrightnessAdj", values: None },
    8u16 => TagDef { name: "EnhanceDarkTones", values: Some(NIKONCAPTURE_BRIGHTNESS_ENHANCEDARKTONES_VALUES) },
};

pub static NIKONCAPTURE_BRIGHTNESS_ENHANCEDARKTONES_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

/// NikonCapture::ColorBoost tags
pub static NIKONCAPTURE_COLORBOOST: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "ColorBoostType", values: Some(NIKONCAPTURE_COLORBOOST_COLORBOOSTTYPE_VALUES) },
    1u16 => TagDef { name: "ColorBoostLevel", values: None },
};

pub static NIKONCAPTURE_COLORBOOST_COLORBOOSTTYPE_VALUES: &[(i64, &str)] = &[
    (0, "Nature"),
    (1, "People"),
];

/// NikonCapture::CropData tags
pub static NIKONCAPTURE_CROPDATA: phf::Map<u16, TagDef> = phf::phf_map! {
    142u16 => TagDef { name: "CropOutputWidthInches", values: None },
    150u16 => TagDef { name: "CropOutputHeightInches", values: None },
    158u16 => TagDef { name: "CropScaledResolution", values: None },
    174u16 => TagDef { name: "CropSourceResolution", values: None },
    182u16 => TagDef { name: "CropOutputResolution", values: None },
    190u16 => TagDef { name: "CropOutputScale", values: None },
    198u16 => TagDef { name: "CropOutputWidth", values: None },
    206u16 => TagDef { name: "CropOutputHeight", values: None },
    214u16 => TagDef { name: "CropOutputPixels", values: None },
    30u16 => TagDef { name: "CropLeft", values: None },
    38u16 => TagDef { name: "CropTop", values: None },
    46u16 => TagDef { name: "CropRight", values: None },
    54u16 => TagDef { name: "CropBottom", values: None },
};

/// NikonCapture::Exposure tags
pub static NIKONCAPTURE_EXPOSURE: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "ExposureAdj", values: None },
    18u16 => TagDef { name: "ExposureAdj2", values: None },
    36u16 => TagDef { name: "ActiveD-Lighting", values: Some(NIKONCAPTURE_EXPOSURE_ACTIVED_LIGHTING_VALUES) },
    37u16 => TagDef { name: "ActiveD-LightingMode", values: Some(NIKONCAPTURE_EXPOSURE_ACTIVED_LIGHTINGMODE_VALUES) },
};

pub static NIKONCAPTURE_EXPOSURE_ACTIVED_LIGHTING_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_EXPOSURE_ACTIVED_LIGHTINGMODE_VALUES: &[(i64, &str)] = &[
    (0, "Unchanged"),
    (1, "Off"),
    (2, "Low"),
    (3, "Normal"),
    (4, "High"),
    (6, "Extra High"),
    (7, "Extra High 1"),
    (8, "Extra High 2"),
];

/// NikonCapture::Main tags
pub static NIKONCAPTURE_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
};

pub static NIKONCAPTURE_MAIN_QUICKFIX_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_COLORBOOSTER_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_D_LIGHTINGHQSELECTED_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];

pub static NIKONCAPTURE_MAIN_NOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_UNSHARPMASK_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_CURVES_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_COLORBALANCEADJ_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_ADVANCEDRAW_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_WHITEBALANCEADJ_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_VIGNETTECONTROL_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_FLIPHORIZONTAL_VALUES: &[(i64, &str)] = &[
    (0, "No"),
    (1, "Yes"),
];

pub static NIKONCAPTURE_MAIN_COLORABERRATIONCONTROL_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_PHOTOEFFECTS_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_D_LIGHTINGHS_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_PICTURECONTROL_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_AUTOREDEYE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_IMAGEDUSTOFF_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_D_LIGHTINGHQ_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_MAIN_LCHEDITOR_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

/// NikonCapture::NoiseReduction tags
pub static NIKONCAPTURE_NOISEREDUCTION: phf::Map<u16, TagDef> = phf::phf_map! {
    13u16 => TagDef { name: "NoiseReductionSharpness", values: None },
    17u16 => TagDef { name: "NoiseReductionMethod", values: Some(NIKONCAPTURE_NOISEREDUCTION_NOISEREDUCTIONMETHOD_VALUES) },
    21u16 => TagDef { name: "ColorMoireReduction", values: Some(NIKONCAPTURE_NOISEREDUCTION_COLORMOIREREDUCTION_VALUES) },
    23u16 => TagDef { name: "NoiseReduction", values: Some(NIKONCAPTURE_NOISEREDUCTION_NOISEREDUCTION_VALUES) },
    24u16 => TagDef { name: "ColorNoiseReductionIntensity", values: None },
    28u16 => TagDef { name: "ColorNoiseReductionSharpness", values: None },
    4u16 => TagDef { name: "EdgeNoiseReduction", values: Some(NIKONCAPTURE_NOISEREDUCTION_EDGENOISEREDUCTION_VALUES) },
    5u16 => TagDef { name: "ColorMoireReductionMode", values: Some(NIKONCAPTURE_NOISEREDUCTION_COLORMOIREREDUCTIONMODE_VALUES) },
    9u16 => TagDef { name: "NoiseReductionIntensity", values: None },
};

pub static NIKONCAPTURE_NOISEREDUCTION_NOISEREDUCTIONMETHOD_VALUES: &[(i64, &str)] = &[
    (0, "Faster"),
    (1, "Better Quality"),
    (2, "Better Quality 2013"),
];

pub static NIKONCAPTURE_NOISEREDUCTION_COLORMOIREREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_NOISEREDUCTION_NOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_NOISEREDUCTION_EDGENOISEREDUCTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

pub static NIKONCAPTURE_NOISEREDUCTION_COLORMOIREREDUCTIONMODE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Low"),
    (2, "Medium"),
    (3, "High"),
];

/// NikonCapture::PhotoEffects tags
pub static NIKONCAPTURE_PHOTOEFFECTS: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "PhotoEffectsType", values: Some(NIKONCAPTURE_PHOTOEFFECTS_PHOTOEFFECTSTYPE_VALUES) },
    4u16 => TagDef { name: "PhotoEffectsRed", values: None },
    6u16 => TagDef { name: "PhotoEffectsGreen", values: None },
    8u16 => TagDef { name: "PhotoEffectsBlue", values: None },
};

pub static NIKONCAPTURE_PHOTOEFFECTS_PHOTOEFFECTSTYPE_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1, "B&W"),
    (2, "Sepia"),
    (3, "Tinted"),
];

/// NikonCapture::PictureCtrl tags
pub static NIKONCAPTURE_PICTURECTRL: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "PictureControlActive", values: Some(NIKONCAPTURE_PICTURECTRL_PICTURECONTROLACTIVE_VALUES) },
    19u16 => TagDef { name: "PictureControlMode", values: None },
    42u16 => TagDef { name: "QuickAdjust", values: None },
    43u16 => TagDef { name: "SharpeningAdj", values: None },
    44u16 => TagDef { name: "ContrastAdj", values: None },
    45u16 => TagDef { name: "BrightnessAdj", values: None },
    46u16 => TagDef { name: "SaturationAdj", values: None },
    47u16 => TagDef { name: "HueAdj", values: None },
};

pub static NIKONCAPTURE_PICTURECTRL_PICTURECONTROLACTIVE_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "On"),
];

/// NikonCapture::RedEyeData tags
pub static NIKONCAPTURE_REDEYEDATA: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "RedEyeCorrection", values: Some(NIKONCAPTURE_REDEYEDATA_REDEYECORRECTION_VALUES) },
};

pub static NIKONCAPTURE_REDEYEDATA_REDEYECORRECTION_VALUES: &[(i64, &str)] = &[
    (0, "Off"),
    (1, "Automatic"),
    (2, "Click on Eyes"),
];

/// NikonCapture::UnsharpData tags
pub static NIKONCAPTURE_UNSHARPDATA: phf::Map<u16, TagDef> = phf::phf_map! {
    100u16 => TagDef { name: "Unsharp4Color", values: Some(NIKONCAPTURE_UNSHARPDATA_UNSHARP4COLOR_VALUES) },
    104u16 => TagDef { name: "Unsharp4Intensity", values: None },
    106u16 => TagDef { name: "Unsharp4HaloWidth", values: None },
    19u16 => TagDef { name: "Unsharp1Color", values: Some(NIKONCAPTURE_UNSHARPDATA_UNSHARP1COLOR_VALUES) },
    23u16 => TagDef { name: "Unsharp1Intensity", values: None },
    25u16 => TagDef { name: "Unsharp1HaloWidth", values: None },
    46u16 => TagDef { name: "Unsharp2Color", values: Some(NIKONCAPTURE_UNSHARPDATA_UNSHARP2COLOR_VALUES) },
    50u16 => TagDef { name: "Unsharp2Intensity", values: None },
    52u16 => TagDef { name: "Unsharp2HaloWidth", values: None },
    73u16 => TagDef { name: "Unsharp3Color", values: Some(NIKONCAPTURE_UNSHARPDATA_UNSHARP3COLOR_VALUES) },
    77u16 => TagDef { name: "Unsharp3Intensity", values: None },
    79u16 => TagDef { name: "Unsharp3HaloWidth", values: None },
};

pub static NIKONCAPTURE_UNSHARPDATA_UNSHARP4COLOR_VALUES: &[(i64, &str)] = &[
    (0, "RGB"),
    (1, "Red"),
    (2, "Green"),
    (3, "Blue"),
    (4, "Yellow"),
    (5, "Magenta"),
    (6, "Cyan"),
];

pub static NIKONCAPTURE_UNSHARPDATA_UNSHARP1COLOR_VALUES: &[(i64, &str)] = &[
    (0, "RGB"),
    (1, "Red"),
    (2, "Green"),
    (3, "Blue"),
    (4, "Yellow"),
    (5, "Magenta"),
    (6, "Cyan"),
];

pub static NIKONCAPTURE_UNSHARPDATA_UNSHARP2COLOR_VALUES: &[(i64, &str)] = &[
    (0, "RGB"),
    (1, "Red"),
    (2, "Green"),
    (3, "Blue"),
    (4, "Yellow"),
    (5, "Magenta"),
    (6, "Cyan"),
];

pub static NIKONCAPTURE_UNSHARPDATA_UNSHARP3COLOR_VALUES: &[(i64, &str)] = &[
    (0, "RGB"),
    (1, "Red"),
    (2, "Green"),
    (3, "Blue"),
    (4, "Yellow"),
    (5, "Magenta"),
    (6, "Cyan"),
];

/// NikonCapture::WBAdjData tags
pub static NIKONCAPTURE_WBADJDATA: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "WBAdjRedBalance", values: None },
    16u16 => TagDef { name: "WBAdjMode", values: Some(NIKONCAPTURE_WBADJDATA_WBADJMODE_VALUES) },
    20u16 => TagDef { name: "WBAdjLighting", values: Some(NIKONCAPTURE_WBADJDATA_WBADJLIGHTING_VALUES) },
    24u16 => TagDef { name: "WBAdjTemperature", values: None },
    37u16 => TagDef { name: "WBAdjTint", values: None },
    8u16 => TagDef { name: "WBAdjBlueBalance", values: None },
};

pub static NIKONCAPTURE_WBADJDATA_WBADJMODE_VALUES: &[(i64, &str)] = &[
    (1, "Use Gray Point"),
    (2, "Recorded Value"),
    (3, "Use Temperature"),
    (4, "Calculate Automatically"),
    (5, "Auto2"),
    (6, "Underwater"),
    (7, "Auto1"),
];

pub static NIKONCAPTURE_WBADJDATA_WBADJLIGHTING_VALUES: &[(i64, &str)] = &[
    (0, "None"),
    (1024, "High Color Rendering Fluorescent (warm white)"),
    (1025, "High Color Rendering Fluorescent (3700K)"),
    (1026, "High Color Rendering Fluorescent (cool white)"),
    (1027, "High Color Rendering Fluorescent (5000K)"),
    (1028, "High Color Rendering Fluorescent (daylight)"),
    (1280, "Flash"),
    (1281, "Flash (FL-G1 filter)"),
    (1282, "Flash (FL-G2 filter)"),
    (1283, "Flash (TN-A1 filter)"),
    (1284, "Flash (TN-A2 filter)"),
    (1536, "Sodium Vapor Lamps"),
    (256, "Incandescent"),
    (512, "Daylight (direct sunlight)"),
    (513, "Daylight (shade)"),
    (514, "Daylight (cloudy)"),
    (768, "Standard Fluorescent (warm white)"),
    (769, "Standard Fluorescent (3700K)"),
    (770, "Standard Fluorescent (cool white)"),
    (771, "Standard Fluorescent (5000K)"),
    (772, "Standard Fluorescent (daylight)"),
    (773, "Standard Fluorescent (high temperature mercury vapor)"),
];

/// NikonCustom::SettingsD40 tags
pub static NIKONCUSTOM_SETTINGSD40: phf::Map<u16, TagDef> = phf::phf_map! {
    9u16 => TagDef { name: "FlashLevel", values: None },
};

/// NikonSettings::Main tags
pub static NIKONSETTINGS_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    1u16 => TagDef { name: "ISOAutoHiLimit", values: Some(NIKONSETTINGS_MAIN_ISOAUTOHILIMIT_VALUES) },
    108u16 => TagDef { name: "ShootingInfoDisplay", values: Some(NIKONSETTINGS_MAIN_SHOOTINGINFODISPLAY_VALUES) },
    11u16 => TagDef { name: "FlickerReductionShooting", values: Some(NIKONSETTINGS_MAIN_FLICKERREDUCTIONSHOOTING_VALUES) },
    116u16 => TagDef { name: "FlickAdvanceDirection", values: Some(NIKONSETTINGS_MAIN_FLICKADVANCEDIRECTION_VALUES) },
    117u16 => TagDef { name: "HDMIOutputResolution", values: Some(NIKONSETTINGS_MAIN_HDMIOUTPUTRESOLUTION_VALUES) },
    119u16 => TagDef { name: "HDMIOutputRange", values: Some(NIKONSETTINGS_MAIN_HDMIOUTPUTRANGE_VALUES) },
    12u16 => TagDef { name: "FlickerReductionIndicator", values: Some(NIKONSETTINGS_MAIN_FLICKERREDUCTIONINDICATOR_VALUES) },
    128u16 => TagDef { name: "RemoteFuncButton", values: Some(NIKONSETTINGS_MAIN_REMOTEFUNCBUTTON_VALUES) },
    13u16 => TagDef { name: "MovieISOAutoHiLimit", values: Some(NIKONSETTINGS_MAIN_MOVIEISOAUTOHILIMIT_VALUES) },
    139u16 => TagDef { name: "CmdDialsReverseRotation", values: Some(NIKONSETTINGS_MAIN_CMDDIALSREVERSEROTATION_VALUES) },
    14u16 => TagDef { name: "MovieISOAutoControlManualMode", values: Some(NIKONSETTINGS_MAIN_MOVIEISOAUTOCONTROLMANUALMODE_VALUES) },
    141u16 => TagDef { name: "FocusPeakingHighlightColor", values: Some(NIKONSETTINGS_MAIN_FOCUSPEAKINGHIGHLIGHTCOLOR_VALUES) },
    142u16 => TagDef { name: "ContinuousModeDisplay", values: Some(NIKONSETTINGS_MAIN_CONTINUOUSMODEDISPLAY_VALUES) },
    143u16 => TagDef { name: "ShutterSpeedLock", values: Some(NIKONSETTINGS_MAIN_SHUTTERSPEEDLOCK_VALUES) },
    144u16 => TagDef { name: "ApertureLock", values: Some(NIKONSETTINGS_MAIN_APERTURELOCK_VALUES) },
    145u16 => TagDef { name: "MovieHighlightDisplayThreshold", values: Some(NIKONSETTINGS_MAIN_MOVIEHIGHLIGHTDISPLAYTHRESHOLD_VALUES) },
    146u16 => TagDef { name: "HDMIExternalRecorder", values: Some(NIKONSETTINGS_MAIN_HDMIEXTERNALRECORDER_VALUES) },
    147u16 => TagDef { name: "BlockShotAFResponse", values: Some(NIKONSETTINGS_MAIN_BLOCKSHOTAFRESPONSE_VALUES) },
    148u16 => TagDef { name: "SubjectMotion", values: Some(NIKONSETTINGS_MAIN_SUBJECTMOTION_VALUES) },
    149u16 => TagDef { name: "Three-DTrackingFaceDetection", values: Some(NIKONSETTINGS_MAIN_THREE_DTRACKINGFACEDETECTION_VALUES) },
    15u16 => TagDef { name: "MovieWhiteBalanceSameAsPhoto", values: Some(NIKONSETTINGS_MAIN_MOVIEWHITEBALANCESAMEASPHOTO_VALUES) },
    151u16 => TagDef { name: "StoreByOrientation", values: Some(NIKONSETTINGS_MAIN_STOREBYORIENTATION_VALUES) },
    153u16 => TagDef { name: "DynamicAreaAFAssist", values: Some(NIKONSETTINGS_MAIN_DYNAMICAREAAFASSIST_VALUES) },
    154u16 => TagDef { name: "ExposureCompStepSize", values: Some(NIKONSETTINGS_MAIN_EXPOSURECOMPSTEPSIZE_VALUES) },
    155u16 => TagDef { name: "SyncReleaseMode", values: Some(NIKONSETTINGS_MAIN_SYNCRELEASEMODE_VALUES) },
    156u16 => TagDef { name: "ModelingFlash", values: Some(NIKONSETTINGS_MAIN_MODELINGFLASH_VALUES) },
    157u16 => TagDef { name: "AutoBracketModeM", values: Some(NIKONSETTINGS_MAIN_AUTOBRACKETMODEM_VALUES) },
    158u16 => TagDef { name: "PreviewButton", values: Some(NIKONSETTINGS_MAIN_PREVIEWBUTTON_VALUES) },
    160u16 => TagDef { name: "Func1Button", values: Some(NIKONSETTINGS_MAIN_FUNC1BUTTON_VALUES) },
    162u16 => TagDef { name: "Func2Button", values: Some(NIKONSETTINGS_MAIN_FUNC2BUTTON_VALUES) },
    163u16 => TagDef { name: "AF-OnButton", values: Some(NIKONSETTINGS_MAIN_AF_ONBUTTON_VALUES) },
    164u16 => TagDef { name: "SubSelector", values: Some(NIKONSETTINGS_MAIN_SUBSELECTOR_VALUES) },
    165u16 => TagDef { name: "SubSelectorCenter", values: Some(NIKONSETTINGS_MAIN_SUBSELECTORCENTER_VALUES) },
    167u16 => TagDef { name: "LensFunc1Button", values: Some(NIKONSETTINGS_MAIN_LENSFUNC1BUTTON_VALUES) },
    168u16 => TagDef { name: "CmdDialsApertureSetting", values: Some(NIKONSETTINGS_MAIN_CMDDIALSAPERTURESETTING_VALUES) },
    169u16 => TagDef { name: "MultiSelector", values: Some(NIKONSETTINGS_MAIN_MULTISELECTOR_VALUES) },
    170u16 => TagDef { name: "LiveViewButtonOptions", values: Some(NIKONSETTINGS_MAIN_LIVEVIEWBUTTONOPTIONS_VALUES) },
    171u16 => TagDef { name: "LightSwitch", values: Some(NIKONSETTINGS_MAIN_LIGHTSWITCH_VALUES) },
    177u16 => TagDef { name: "MoviePreviewButton", values: Some(NIKONSETTINGS_MAIN_MOVIEPREVIEWBUTTON_VALUES) },
    179u16 => TagDef { name: "MovieFunc1Button", values: Some(NIKONSETTINGS_MAIN_MOVIEFUNC1BUTTON_VALUES) },
    181u16 => TagDef { name: "MovieFunc2Button", values: Some(NIKONSETTINGS_MAIN_MOVIEFUNC2BUTTON_VALUES) },
    182u16 => TagDef { name: "AssignMovieSubselector", values: Some(NIKONSETTINGS_MAIN_ASSIGNMOVIESUBSELECTOR_VALUES) },
    184u16 => TagDef { name: "LimitAFAreaModeSelD9", values: Some(NIKONSETTINGS_MAIN_LIMITAFAREAMODESELD9_VALUES) },
    185u16 => TagDef { name: "LimitAFAreaModeSelD25", values: Some(NIKONSETTINGS_MAIN_LIMITAFAREAMODESELD25_VALUES) },
    188u16 => TagDef { name: "LimitAFAreaModeSel3D", values: Some(NIKONSETTINGS_MAIN_LIMITAFAREAMODESEL3D_VALUES) },
    189u16 => TagDef { name: "LimitAFAreaModeSelGroup", values: Some(NIKONSETTINGS_MAIN_LIMITAFAREAMODESELGROUP_VALUES) },
    190u16 => TagDef { name: "LimitAFAreaModeSelAuto", values: Some(NIKONSETTINGS_MAIN_LIMITAFAREAMODESELAUTO_VALUES) },
    193u16 => TagDef { name: "LimitSelectableImageArea5To4", values: Some(NIKONSETTINGS_MAIN_LIMITSELECTABLEIMAGEAREA5TO4_VALUES) },
    194u16 => TagDef { name: "LimitSelectableImageArea1To1", values: Some(NIKONSETTINGS_MAIN_LIMITSELECTABLEIMAGEAREA1TO1_VALUES) },
    2u16 => TagDef { name: "ISOAutoFlashLimit", values: Some(NIKONSETTINGS_MAIN_ISOAUTOFLASHLIMIT_VALUES) },
    212u16 => TagDef { name: "PhotoShootingMenuBank", values: Some(NIKONSETTINGS_MAIN_PHOTOSHOOTINGMENUBANK_VALUES) },
    213u16 => TagDef { name: "CustomSettingsBank", values: Some(NIKONSETTINGS_MAIN_CUSTOMSETTINGSBANK_VALUES) },
    214u16 => TagDef { name: "LimitAF-AreaModeSelPinpoint", values: Some(NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELPINPOINT_VALUES) },
    215u16 => TagDef { name: "LimitAF-AreaModeSelDynamic", values: Some(NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELDYNAMIC_VALUES) },
    216u16 => TagDef { name: "LimitAF-AreaModeSelWideAF_S", values: Some(NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELWIDEAF_S_VALUES) },
    217u16 => TagDef { name: "LimitAF-AreaModeSelWideAF_L", values: Some(NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELWIDEAF_L_VALUES) },
    218u16 => TagDef { name: "LowLightAF", values: Some(NIKONSETTINGS_MAIN_LOWLIGHTAF_VALUES) },
    219u16 => TagDef { name: "LimitSelectableImageAreaDX", values: Some(NIKONSETTINGS_MAIN_LIMITSELECTABLEIMAGEAREADX_VALUES) },
    220u16 => TagDef { name: "LimitSelectableImageArea5To4", values: Some(NIKONSETTINGS_MAIN_LIMITSELECTABLEIMAGEAREA5TO4_VALUES) },
    221u16 => TagDef { name: "LimitSelectableImageArea1To1", values: Some(NIKONSETTINGS_MAIN_LIMITSELECTABLEIMAGEAREA1TO1_VALUES) },
    222u16 => TagDef { name: "LimitSelectableImageArea16To9", values: Some(NIKONSETTINGS_MAIN_LIMITSELECTABLEIMAGEAREA16TO9_VALUES) },
    223u16 => TagDef { name: "ApplySettingsToLiveView", values: Some(NIKONSETTINGS_MAIN_APPLYSETTINGSTOLIVEVIEW_VALUES) },
    224u16 => TagDef { name: "FocusPeakingLevel", values: Some(NIKONSETTINGS_MAIN_FOCUSPEAKINGLEVEL_VALUES) },
    234u16 => TagDef { name: "LensControlRing", values: Some(NIKONSETTINGS_MAIN_LENSCONTROLRING_VALUES) },
    237u16 => TagDef { name: "MovieMultiSelector", values: Some(NIKONSETTINGS_MAIN_MOVIEMULTISELECTOR_VALUES) },
    238u16 => TagDef { name: "MovieAFSpeed", values: None },
    239u16 => TagDef { name: "MovieAFSpeedApply", values: Some(NIKONSETTINGS_MAIN_MOVIEAFSPEEDAPPLY_VALUES) },
    240u16 => TagDef { name: "MovieAFTrackingSensitivity", values: Some(NIKONSETTINGS_MAIN_MOVIEAFTRACKINGSENSITIVITY_VALUES) },
    241u16 => TagDef { name: "MovieHighlightDisplayPattern", values: Some(NIKONSETTINGS_MAIN_MOVIEHIGHLIGHTDISPLAYPATTERN_VALUES) },
    242u16 => TagDef { name: "SubDialFrameAdvanceRating5", values: Some(NIKONSETTINGS_MAIN_SUBDIALFRAMEADVANCERATING5_VALUES) },
    243u16 => TagDef { name: "SubDialFrameAdvanceRating4", values: Some(NIKONSETTINGS_MAIN_SUBDIALFRAMEADVANCERATING4_VALUES) },
    244u16 => TagDef { name: "SubDialFrameAdvanceRating3", values: Some(NIKONSETTINGS_MAIN_SUBDIALFRAMEADVANCERATING3_VALUES) },
    245u16 => TagDef { name: "SubDialFrameAdvanceRating2", values: Some(NIKONSETTINGS_MAIN_SUBDIALFRAMEADVANCERATING2_VALUES) },
    246u16 => TagDef { name: "SubDialFrameAdvanceRating1", values: Some(NIKONSETTINGS_MAIN_SUBDIALFRAMEADVANCERATING1_VALUES) },
    247u16 => TagDef { name: "SubDialFrameAdvanceRating0", values: Some(NIKONSETTINGS_MAIN_SUBDIALFRAMEADVANCERATING0_VALUES) },
    249u16 => TagDef { name: "MovieAF-OnButton", values: Some(NIKONSETTINGS_MAIN_MOVIEAF_ONBUTTON_VALUES) },
    251u16 => TagDef { name: "SecondarySlotFunction", values: Some(NIKONSETTINGS_MAIN_SECONDARYSLOTFUNCTION_VALUES) },
    252u16 => TagDef { name: "SilentPhotography", values: Some(NIKONSETTINGS_MAIN_SILENTPHOTOGRAPHY_VALUES) },
    253u16 => TagDef { name: "ExtendedShutterSpeeds", values: Some(NIKONSETTINGS_MAIN_EXTENDEDSHUTTERSPEEDS_VALUES) },
    258u16 => TagDef { name: "HDMIBitDepth", values: Some(NIKONSETTINGS_MAIN_HDMIBITDEPTH_VALUES) },
    259u16 => TagDef { name: "HDMIOutputHDR", values: Some(NIKONSETTINGS_MAIN_HDMIOUTPUTHDR_VALUES) },
    260u16 => TagDef { name: "HDMIViewAssist", values: Some(NIKONSETTINGS_MAIN_HDMIVIEWASSIST_VALUES) },
    265u16 => TagDef { name: "BracketSet", values: Some(NIKONSETTINGS_MAIN_BRACKETSET_VALUES) },
    266u16 => TagDef { name: "BracketProgram", values: Some(NIKONSETTINGS_MAIN_BRACKETPROGRAM_VALUES) },
    267u16 => TagDef { name: "BracketIncrement", values: Some(NIKONSETTINGS_MAIN_BRACKETINCREMENT_VALUES) },
    268u16 => TagDef { name: "BracketIncrement", values: Some(NIKONSETTINGS_MAIN_BRACKETINCREMENT_VALUES) },
    270u16 => TagDef { name: "MonitorBrightness", values: None },
    278u16 => TagDef { name: "GroupAreaC1", values: Some(NIKONSETTINGS_MAIN_GROUPAREAC1_VALUES) },
    279u16 => TagDef { name: "AutoAreaAFStartingPoint", values: Some(NIKONSETTINGS_MAIN_AUTOAREAAFSTARTINGPOINT_VALUES) },
    280u16 => TagDef { name: "FocusPointPersistence", values: Some(NIKONSETTINGS_MAIN_FOCUSPOINTPERSISTENCE_VALUES) },
    281u16 => TagDef { name: "LimitAFAreaModeSelD49", values: Some(NIKONSETTINGS_MAIN_LIMITAFAREAMODESELD49_VALUES) },
    282u16 => TagDef { name: "LimitAFAreaModeSelD105", values: Some(NIKONSETTINGS_MAIN_LIMITAFAREAMODESELD105_VALUES) },
    283u16 => TagDef { name: "LimitAFAreaModeSelGroupC1", values: Some(NIKONSETTINGS_MAIN_LIMITAFAREAMODESELGROUPC1_VALUES) },
    284u16 => TagDef { name: "LimitAFAreaModeSelGroupC2", values: Some(NIKONSETTINGS_MAIN_LIMITAFAREAMODESELGROUPC2_VALUES) },
    285u16 => TagDef { name: "AutoFocusModeRestrictions", values: Some(NIKONSETTINGS_MAIN_AUTOFOCUSMODERESTRICTIONS_VALUES) },
    286u16 => TagDef { name: "FocusPointBrightness", values: Some(NIKONSETTINGS_MAIN_FOCUSPOINTBRIGHTNESS_VALUES) },
    287u16 => TagDef { name: "CHModeShootingSpeed", values: None },
    288u16 => TagDef { name: "CLModeShootingSpeed", values: None },
    289u16 => TagDef { name: "QuietShutterShootingSpeed", values: Some(NIKONSETTINGS_MAIN_QUIETSHUTTERSHOOTINGSPEED_VALUES) },
    29u16 => TagDef { name: "AF-CPrioritySel", values: Some(NIKONSETTINGS_MAIN_AF_CPRIORITYSEL_VALUES) },
    290u16 => TagDef { name: "LimitReleaseModeSelCL", values: Some(NIKONSETTINGS_MAIN_LIMITRELEASEMODESELCL_VALUES) },
    291u16 => TagDef { name: "LimitReleaseModeSelCH", values: Some(NIKONSETTINGS_MAIN_LIMITRELEASEMODESELCH_VALUES) },
    292u16 => TagDef { name: "LimitReleaseModeSelQ", values: Some(NIKONSETTINGS_MAIN_LIMITRELEASEMODESELQ_VALUES) },
    293u16 => TagDef { name: "LimitReleaseModeSelTimer", values: Some(NIKONSETTINGS_MAIN_LIMITRELEASEMODESELTIMER_VALUES) },
    294u16 => TagDef { name: "LimitReleaseModeSelMirror-Up", values: Some(NIKONSETTINGS_MAIN_LIMITRELEASEMODESELMIRROR_UP_VALUES) },
    295u16 => TagDef { name: "LimitSelectableImageArea16To9", values: Some(NIKONSETTINGS_MAIN_LIMITSELECTABLEIMAGEAREA16TO9_VALUES) },
    296u16 => TagDef { name: "RearControPanelDisplay", values: Some(NIKONSETTINGS_MAIN_REARCONTROPANELDISPLAY_VALUES) },
    297u16 => TagDef { name: "FlashBurstPriority", values: Some(NIKONSETTINGS_MAIN_FLASHBURSTPRIORITY_VALUES) },
    298u16 => TagDef { name: "RecallShootFuncExposureMode", values: Some(NIKONSETTINGS_MAIN_RECALLSHOOTFUNCEXPOSUREMODE_VALUES) },
    299u16 => TagDef { name: "RecallShootFuncShutterSpeed", values: Some(NIKONSETTINGS_MAIN_RECALLSHOOTFUNCSHUTTERSPEED_VALUES) },
    3u16 => TagDef { name: "ISOAutoShutterTime", values: Some(NIKONSETTINGS_MAIN_ISOAUTOSHUTTERTIME_VALUES) },
    30u16 => TagDef { name: "AF-SPrioritySel", values: Some(NIKONSETTINGS_MAIN_AF_SPRIORITYSEL_VALUES) },
    300u16 => TagDef { name: "RecallShootFuncAperture", values: Some(NIKONSETTINGS_MAIN_RECALLSHOOTFUNCAPERTURE_VALUES) },
    301u16 => TagDef { name: "RecallShootFuncExposureComp", values: Some(NIKONSETTINGS_MAIN_RECALLSHOOTFUNCEXPOSURECOMP_VALUES) },
    302u16 => TagDef { name: "RecallShootFuncISO", values: Some(NIKONSETTINGS_MAIN_RECALLSHOOTFUNCISO_VALUES) },
    303u16 => TagDef { name: "RecallShootFuncMeteringMode", values: Some(NIKONSETTINGS_MAIN_RECALLSHOOTFUNCMETERINGMODE_VALUES) },
    304u16 => TagDef { name: "RecallShootFuncWhiteBalance", values: Some(NIKONSETTINGS_MAIN_RECALLSHOOTFUNCWHITEBALANCE_VALUES) },
    305u16 => TagDef { name: "RecallShootFuncAFAreaMode", values: Some(NIKONSETTINGS_MAIN_RECALLSHOOTFUNCAFAREAMODE_VALUES) },
    306u16 => TagDef { name: "RecallShootFuncFocusTracking", values: Some(NIKONSETTINGS_MAIN_RECALLSHOOTFUNCFOCUSTRACKING_VALUES) },
    307u16 => TagDef { name: "RecallShootFuncAF-On", values: Some(NIKONSETTINGS_MAIN_RECALLSHOOTFUNCAF_ON_VALUES) },
    308u16 => TagDef { name: "VerticalFuncButton", values: Some(NIKONSETTINGS_MAIN_VERTICALFUNCBUTTON_VALUES) },
    309u16 => TagDef { name: "Func3Button", values: Some(NIKONSETTINGS_MAIN_FUNC3BUTTON_VALUES) },
    310u16 => TagDef { name: "VerticalAF-OnButton", values: Some(NIKONSETTINGS_MAIN_VERTICALAF_ONBUTTON_VALUES) },
    311u16 => TagDef { name: "VerticalMultiSelector", values: Some(NIKONSETTINGS_MAIN_VERTICALMULTISELECTOR_VALUES) },
    312u16 => TagDef { name: "MeteringButton", values: Some(NIKONSETTINGS_MAIN_METERINGBUTTON_VALUES) },
    313u16 => TagDef { name: "PlaybackFlickUp", values: Some(NIKONSETTINGS_MAIN_PLAYBACKFLICKUP_VALUES) },
    314u16 => TagDef { name: "PlaybackFlickUpRating", values: Some(NIKONSETTINGS_MAIN_PLAYBACKFLICKUPRATING_VALUES) },
    315u16 => TagDef { name: "PlaybackFlickDown", values: Some(NIKONSETTINGS_MAIN_PLAYBACKFLICKDOWN_VALUES) },
    316u16 => TagDef { name: "PlaybackFlickDownRating", values: Some(NIKONSETTINGS_MAIN_PLAYBACKFLICKDOWNRATING_VALUES) },
    317u16 => TagDef { name: "MovieFunc3Button", values: Some(NIKONSETTINGS_MAIN_MOVIEFUNC3BUTTON_VALUES) },
    32u16 => TagDef { name: "AFPointSel", values: Some(NIKONSETTINGS_MAIN_AFPOINTSEL_VALUES) },
    336u16 => TagDef { name: "ShutterType", values: Some(NIKONSETTINGS_MAIN_SHUTTERTYPE_VALUES) },
    337u16 => TagDef { name: "LensFunc2Button", values: Some(NIKONSETTINGS_MAIN_LENSFUNC2BUTTON_VALUES) },
    34u16 => TagDef { name: "AFActivation", values: Some(NIKONSETTINGS_MAIN_AFACTIVATION_VALUES) },
    344u16 => TagDef { name: "USBPowerDelivery", values: Some(NIKONSETTINGS_MAIN_USBPOWERDELIVERY_VALUES) },
    345u16 => TagDef { name: "EnergySavingMode", values: Some(NIKONSETTINGS_MAIN_ENERGYSAVINGMODE_VALUES) },
    348u16 => TagDef { name: "BracketingBurstOptions", values: Some(NIKONSETTINGS_MAIN_BRACKETINGBURSTOPTIONS_VALUES) },
    35u16 => TagDef { name: "FocusPointWrap", values: Some(NIKONSETTINGS_MAIN_FOCUSPOINTWRAP_VALUES) },
    350u16 => TagDef { name: "PrimarySlot", values: Some(NIKONSETTINGS_MAIN_PRIMARYSLOT_VALUES) },
    351u16 => TagDef { name: "ReverseFocusRing", values: Some(NIKONSETTINGS_MAIN_REVERSEFOCUSRING_VALUES) },
    352u16 => TagDef { name: "VerticalFuncButton", values: Some(NIKONSETTINGS_MAIN_VERTICALFUNCBUTTON_VALUES) },
    353u16 => TagDef { name: "VerticalAFOnButton", values: Some(NIKONSETTINGS_MAIN_VERTICALAFONBUTTON_VALUES) },
    354u16 => TagDef { name: "VerticalMultiSelector", values: Some(NIKONSETTINGS_MAIN_VERTICALMULTISELECTOR_VALUES) },
    356u16 => TagDef { name: "VerticalMovieFuncButton", values: Some(NIKONSETTINGS_MAIN_VERTICALMOVIEFUNCBUTTON_VALUES) },
    357u16 => TagDef { name: "VerticalMovieAFOnButton", values: Some(NIKONSETTINGS_MAIN_VERTICALMOVIEAFONBUTTON_VALUES) },
    361u16 => TagDef { name: "LimitAF-AreaModeSelAutoPeople", values: Some(NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELAUTOPEOPLE_VALUES) },
    362u16 => TagDef { name: "LimitAF-AreaModeSelAutoAnimals", values: Some(NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELAUTOANIMALS_VALUES) },
    363u16 => TagDef { name: "LimitAF-AreaModeSelWideLPeople", values: Some(NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELWIDELPEOPLE_VALUES) },
    364u16 => TagDef { name: "LimitAF-AreaModeSelWideLAnimals", values: Some(NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELWIDELANIMALS_VALUES) },
    365u16 => TagDef { name: "SaveFocus", values: Some(NIKONSETTINGS_MAIN_SAVEFOCUS_VALUES) },
    366u16 => TagDef { name: "AFAreaMode", values: Some(NIKONSETTINGS_MAIN_AFAREAMODE_VALUES) },
    367u16 => TagDef { name: "MovieAFAreaMode", values: Some(NIKONSETTINGS_MAIN_MOVIEAFAREAMODE_VALUES) },
    368u16 => TagDef { name: "PreferSubSelectorCenter", values: Some(NIKONSETTINGS_MAIN_PREFERSUBSELECTORCENTER_VALUES) },
    369u16 => TagDef { name: "KeepExposureWithTeleconverter", values: Some(NIKONSETTINGS_MAIN_KEEPEXPOSUREWITHTELECONVERTER_VALUES) },
    37u16 => TagDef { name: "ManualFocusPointIllumination", values: Some(NIKONSETTINGS_MAIN_MANUALFOCUSPOINTILLUMINATION_VALUES) },
    372u16 => TagDef { name: "FocusPointSelectionSpeed", values: Some(NIKONSETTINGS_MAIN_FOCUSPOINTSELECTIONSPEED_VALUES) },
    38u16 => TagDef { name: "AF-AssistIlluminator", values: Some(NIKONSETTINGS_MAIN_AF_ASSISTILLUMINATOR_VALUES) },
    39u16 => TagDef { name: "ManualFocusRingInAFMode", values: Some(NIKONSETTINGS_MAIN_MANUALFOCUSRINGINAFMODE_VALUES) },
    41u16 => TagDef { name: "ISOStepSize", values: Some(NIKONSETTINGS_MAIN_ISOSTEPSIZE_VALUES) },
    42u16 => TagDef { name: "ExposureControlStepSize", values: Some(NIKONSETTINGS_MAIN_EXPOSURECONTROLSTEPSIZE_VALUES) },
    43u16 => TagDef { name: "EasyExposureCompensation", values: Some(NIKONSETTINGS_MAIN_EASYEXPOSURECOMPENSATION_VALUES) },
    44u16 => TagDef { name: "MatrixMetering", values: Some(NIKONSETTINGS_MAIN_MATRIXMETERING_VALUES) },
    45u16 => TagDef { name: "CenterWeightedAreaSize", values: Some(NIKONSETTINGS_MAIN_CENTERWEIGHTEDAREASIZE_VALUES) },
    47u16 => TagDef { name: "FineTuneOptMatrixMetering", values: None },
    48u16 => TagDef { name: "FineTuneOptCenterWeighted", values: None },
    49u16 => TagDef { name: "FineTuneOptSpotMetering", values: None },
    50u16 => TagDef { name: "FineTuneOptHighlightWeighted", values: None },
    51u16 => TagDef { name: "ShutterReleaseButtonAE-L", values: Some(NIKONSETTINGS_MAIN_SHUTTERRELEASEBUTTONAE_L_VALUES) },
    52u16 => TagDef { name: "StandbyMonitorOffTime", values: Some(NIKONSETTINGS_MAIN_STANDBYMONITOROFFTIME_VALUES) },
    53u16 => TagDef { name: "SelfTimerTime", values: Some(NIKONSETTINGS_MAIN_SELFTIMERTIME_VALUES) },
    54u16 => TagDef { name: "SelfTimerShotCount", values: None },
    55u16 => TagDef { name: "SelfTimerShotInterval", values: Some(NIKONSETTINGS_MAIN_SELFTIMERSHOTINTERVAL_VALUES) },
    56u16 => TagDef { name: "PlaybackMonitorOffTime", values: Some(NIKONSETTINGS_MAIN_PLAYBACKMONITOROFFTIME_VALUES) },
    57u16 => TagDef { name: "MenuMonitorOffTime", values: Some(NIKONSETTINGS_MAIN_MENUMONITOROFFTIME_VALUES) },
    58u16 => TagDef { name: "ShootingInfoMonitorOffTime", values: Some(NIKONSETTINGS_MAIN_SHOOTINGINFOMONITOROFFTIME_VALUES) },
    59u16 => TagDef { name: "ImageReviewMonitorOffTime", values: Some(NIKONSETTINGS_MAIN_IMAGEREVIEWMONITOROFFTIME_VALUES) },
    60u16 => TagDef { name: "LiveViewMonitorOffTime", values: Some(NIKONSETTINGS_MAIN_LIVEVIEWMONITOROFFTIME_VALUES) },
    62u16 => TagDef { name: "CLModeShootingSpeed", values: None },
    63u16 => TagDef { name: "MaxContinuousRelease", values: None },
    64u16 => TagDef { name: "ExposureDelayMode", values: Some(NIKONSETTINGS_MAIN_EXPOSUREDELAYMODE_VALUES) },
    65u16 => TagDef { name: "ElectronicFront-CurtainShutter", values: Some(NIKONSETTINGS_MAIN_ELECTRONICFRONT_CURTAINSHUTTER_VALUES) },
    66u16 => TagDef { name: "FileNumberSequence", values: Some(NIKONSETTINGS_MAIN_FILENUMBERSEQUENCE_VALUES) },
    67u16 => TagDef { name: "FramingGridDisplay", values: Some(NIKONSETTINGS_MAIN_FRAMINGGRIDDISPLAY_VALUES) },
    69u16 => TagDef { name: "LCDIllumination", values: Some(NIKONSETTINGS_MAIN_LCDILLUMINATION_VALUES) },
    70u16 => TagDef { name: "OpticalVR", values: Some(NIKONSETTINGS_MAIN_OPTICALVR_VALUES) },
    71u16 => TagDef { name: "FlashSyncSpeed", values: Some(NIKONSETTINGS_MAIN_FLASHSYNCSPEED_VALUES) },
    72u16 => TagDef { name: "FlashShutterSpeed", values: Some(NIKONSETTINGS_MAIN_FLASHSHUTTERSPEED_VALUES) },
    73u16 => TagDef { name: "FlashExposureCompArea", values: Some(NIKONSETTINGS_MAIN_FLASHEXPOSURECOMPAREA_VALUES) },
    74u16 => TagDef { name: "AutoFlashISOSensitivity", values: Some(NIKONSETTINGS_MAIN_AUTOFLASHISOSENSITIVITY_VALUES) },
    81u16 => TagDef { name: "AssignBktButton", values: Some(NIKONSETTINGS_MAIN_ASSIGNBKTBUTTON_VALUES) },
    82u16 => TagDef { name: "AssignMovieRecordButton", values: Some(NIKONSETTINGS_MAIN_ASSIGNMOVIERECORDBUTTON_VALUES) },
    83u16 => TagDef { name: "MultiSelectorShootMode", values: Some(NIKONSETTINGS_MAIN_MULTISELECTORSHOOTMODE_VALUES) },
    84u16 => TagDef { name: "MultiSelectorPlaybackMode", values: Some(NIKONSETTINGS_MAIN_MULTISELECTORPLAYBACKMODE_VALUES) },
    86u16 => TagDef { name: "MultiSelectorLiveView", values: Some(NIKONSETTINGS_MAIN_MULTISELECTORLIVEVIEW_VALUES) },
    88u16 => TagDef { name: "CmdDialsReverseRotExposureComp", values: None },
    89u16 => TagDef { name: "CmdDialsChangeMainSubExposure", values: None },
    90u16 => TagDef { name: "CmdDialsChangeMainSub", values: Some(NIKONSETTINGS_MAIN_CMDDIALSCHANGEMAINSUB_VALUES) },
    91u16 => TagDef { name: "CmdDialsMenuAndPlayback", values: Some(NIKONSETTINGS_MAIN_CMDDIALSMENUANDPLAYBACK_VALUES) },
    92u16 => TagDef { name: "SubDialFrameAdvance", values: Some(NIKONSETTINGS_MAIN_SUBDIALFRAMEADVANCE_VALUES) },
    93u16 => TagDef { name: "ReleaseButtonToUseDial", values: Some(NIKONSETTINGS_MAIN_RELEASEBUTTONTOUSEDIAL_VALUES) },
    94u16 => TagDef { name: "ReverseIndicators", values: Some(NIKONSETTINGS_MAIN_REVERSEINDICATORS_VALUES) },
    98u16 => TagDef { name: "MovieShutterButton", values: Some(NIKONSETTINGS_MAIN_MOVIESHUTTERBUTTON_VALUES) },
    99u16 => TagDef { name: "Language", values: Some(NIKONSETTINGS_MAIN_LANGUAGE_VALUES) },
};

pub static NIKONSETTINGS_MAIN_ISOAUTOHILIMIT_VALUES: &[(i64, &str)] = &[
    (1, "ISO 200"),
    (10, "ISO 1000"),
    (11, "ISO 1100"),
    (12, "ISO 1250"),
    (13, "ISO 1600"),
    (14, "ISO 2000"),
    (15, "ISO 2200"),
    (16, "ISO 2500"),
    (17, "ISO 3200"),
    (18, "ISO 4000"),
    (19, "ISO 4500"),
    (2, "ISO 250"),
    (20, "ISO 5000"),
    (21, "ISO 6400"),
    (22, "ISO 8000"),
    (23, "ISO 9000"),
    (24, "ISO 10000"),
    (25, "ISO 12800"),
    (26, "ISO 16000"),
    (27, "ISO 18000"),
    (28, "ISO 20000"),
    (29, "ISO 25600"),
    (3, "ISO 280"),
    (30, "ISO 32000"),
    (31, "ISO 36000"),
    (32, "ISO 40000"),
    (33, "ISO 51200"),
    (34, "ISO 64000"),
    (35, "ISO 72000"),
    (36, "ISO 81200"),
    (37, "ISO 102400"),
    (38, "ISO Hi 0.3"),
    (39, "ISO Hi 0.5"),
    (4, "ISO 320"),
    (40, "ISO Hi 0.7"),
    (41, "ISO Hi 1.0"),
    (42, "ISO Hi 2.0"),
    (43, "ISO Hi 3.0"),
    (44, "ISO Hi 4.0"),
    (45, "ISO Hi 5.0"),
    (5, "ISO 400"),
    (6, "ISO 500"),
    (7, "ISO 560"),
    (8, "ISO 640"),
    (9, "ISO 800"),
];

pub static NIKONSETTINGS_MAIN_SHOOTINGINFODISPLAY_VALUES: &[(i64, &str)] = &[
    (1, "Auto"),
    (2, "Manual (dark on light)"),
    (3, "Manual (light on dark)"),
];

pub static NIKONSETTINGS_MAIN_FLICKERREDUCTIONSHOOTING_VALUES: &[(i64, &str)] = &[
    (1, "Enable"),
    (2, "Disable"),
];

pub static NIKONSETTINGS_MAIN_FLICKADVANCEDIRECTION_VALUES: &[(i64, &str)] = &[
    (1, "Right to Left"),
    (2, "Left to Right"),
];

pub static NIKONSETTINGS_MAIN_HDMIOUTPUTRESOLUTION_VALUES: &[(i64, &str)] = &[
    (1, "Auto"),
    (2, "2160p"),
    (3, "1080p"),
    (4, "1080i"),
    (5, "720p"),
    (6, "576p"),
    (7, "480p"),
];

pub static NIKONSETTINGS_MAIN_HDMIOUTPUTRANGE_VALUES: &[(i64, &str)] = &[
    (1, "Auto"),
    (2, "Limit"),
    (3, "Full"),
];

pub static NIKONSETTINGS_MAIN_FLICKERREDUCTIONINDICATOR_VALUES: &[(i64, &str)] = &[
    (1, "Enable"),
    (2, "Disable"),
];

pub static NIKONSETTINGS_MAIN_REMOTEFUNCBUTTON_VALUES: &[(i64, &str)] = &[
    (1, "AF-On"),
    (10, "LiveView Info Display On/Off"),
    (11, "Recall Shooting Functions"),
    (12, "None"),
    (2, "AF Lock Only"),
    (3, "AE Lock (reset on release)"),
    (4, "AE Lock Only"),
    (5, "AE/AF Lock"),
    (6, "FV Lock"),
    (7, "Flash Disable/Enable"),
    (8, "Preview"),
    (9, "+NEF(RAW)"),
];

pub static NIKONSETTINGS_MAIN_MOVIEISOAUTOHILIMIT_VALUES: &[(i64, &str)] = &[
    (1, "ISO 200"),
    (10, "ISO 1000"),
    (11, "ISO 1100"),
    (12, "ISO 1250"),
    (13, "ISO 1600"),
    (14, "ISO 2000"),
    (15, "ISO 2200"),
    (16, "ISO 2500"),
    (17, "ISO 3200"),
    (18, "ISO 4000"),
    (19, "ISO 4500"),
    (2, "ISO 250"),
    (20, "ISO 5000"),
    (21, "ISO 6400"),
    (22, "ISO 8000"),
    (23, "ISO 9000"),
    (24, "ISO 10000"),
    (25, "ISO 12800"),
    (26, "ISO 16000"),
    (27, "ISO 18000"),
    (28, "ISO 20000"),
    (29, "ISO 25600"),
    (3, "ISO 280"),
    (30, "ISO 32000"),
    (31, "ISO 36000"),
    (32, "ISO 40000"),
    (33, "ISO 51200"),
    (34, "ISO 64000"),
    (35, "ISO 72000"),
    (36, "ISO 81200"),
    (37, "ISO 102400"),
    (38, "ISO Hi 0.3"),
    (39, "ISO Hi 0.5"),
    (4, "ISO 320"),
    (40, "ISO Hi 0.7"),
    (41, "ISO Hi 1.0"),
    (42, "ISO Hi 2.0"),
    (43, "ISO Hi 3.0"),
    (44, "ISO Hi 4.0"),
    (45, "ISO Hi 5.0"),
    (5, "ISO 400"),
    (6, "ISO 500"),
    (7, "ISO 560"),
    (8, "ISO 640"),
    (9, "ISO 800"),
];

pub static NIKONSETTINGS_MAIN_CMDDIALSREVERSEROTATION_VALUES: &[(i64, &str)] = &[
    (1, "No"),
    (2, "Shutter Speed & Aperture"),
];

pub static NIKONSETTINGS_MAIN_MOVIEISOAUTOCONTROLMANUALMODE_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_FOCUSPEAKINGHIGHLIGHTCOLOR_VALUES: &[(i64, &str)] = &[
    (1, "Red"),
    (2, "Yellow"),
    (3, "Blue"),
    (4, "White"),
];

pub static NIKONSETTINGS_MAIN_CONTINUOUSMODEDISPLAY_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_SHUTTERSPEEDLOCK_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_APERTURELOCK_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_MOVIEHIGHLIGHTDISPLAYTHRESHOLD_VALUES: &[(i64, &str)] = &[
    (1, "255"),
    (2, "248"),
    (3, "235"),
    (4, "224"),
    (5, "213"),
    (6, "202"),
    (7, "191"),
    (8, "180"),
];

pub static NIKONSETTINGS_MAIN_HDMIEXTERNALRECORDER_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_BLOCKSHOTAFRESPONSE_VALUES: &[(i64, &str)] = &[
    (1, "1 (Quick)"),
    (2, "2"),
    (3, "3 (Normal)"),
    (4, "4"),
    (5, "5 (Delay)"),
];

pub static NIKONSETTINGS_MAIN_SUBJECTMOTION_VALUES: &[(i64, &str)] = &[
    (1, "Erratic"),
    (2, "Steady"),
];

pub static NIKONSETTINGS_MAIN_THREE_DTRACKINGFACEDETECTION_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_MOVIEWHITEBALANCESAMEASPHOTO_VALUES: &[(i64, &str)] = &[
    (1, "Yes"),
    (2, "No"),
];

pub static NIKONSETTINGS_MAIN_STOREBYORIENTATION_VALUES: &[(i64, &str)] = &[
    (1, "Focus Point"),
    (2, "Focus Point and AF-area mode"),
    (3, "Off"),
];

pub static NIKONSETTINGS_MAIN_DYNAMICAREAAFASSIST_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_EXPOSURECOMPSTEPSIZE_VALUES: &[(i64, &str)] = &[
    (1, "1/3 EV"),
    (2, "1/2 EV"),
    (3, "1 EV"),
];

pub static NIKONSETTINGS_MAIN_SYNCRELEASEMODE_VALUES: &[(i64, &str)] = &[
    (1, "Sync"),
    (2, "No Sync"),
];

pub static NIKONSETTINGS_MAIN_MODELINGFLASH_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_AUTOBRACKETMODEM_VALUES: &[(i64, &str)] = &[
    (1, "Flash/Speed"),
    (2, "Flash/Speed/Aperture"),
    (3, "Flash/Aperture"),
    (4, "Flash Only"),
];

pub static NIKONSETTINGS_MAIN_PREVIEWBUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Preset Focus Point - Press To Recall"),
    (10, "AF-AreaMode Group C1"),
    (11, "AF-AreaMode Group C2"),
    (12, "AF-AreaMode Auto Area"),
    (13, "AF-AreaMode + AF-On S"),
    (14, "AF-AreaMode + AF-On D9"),
    (15, "AF-AreaMode + AF-On D25"),
    (16, "AF-AreaMode + AF-On D49"),
    (17, "AF-AreaMode + AF-On D105"),
    (18, "AF-AreaMode + AF-On 3D"),
    (19, "AF-AreaMode + AF-On Group"),
    (2, "Preset Focus Point - Hold To Recall"),
    (20, "AF-AreaMode + AF-On Group C1"),
    (21, "AF-AreaMode + AF-On Group C2"),
    (22, "AF-AreaMode + AF-On Auto Area"),
    (23, "AF-On"),
    (24, "AF Lock Only"),
    (25, "AE Lock (hold)"),
    (26, "AE/WB Lock (hold)"),
    (27, "AE Lock (reset on release)"),
    (28, "AE Lock Only"),
    (29, "AE/AF Lock"),
    (3, "AF-AreaMode S"),
    (30, "FV Lock"),
    (31, "Flash Disable/Enable"),
    (32, "Preview"),
    (33, "Recall Shooting Functions"),
    (34, "Bracketing Burst"),
    (35, "Synchronized Release (Master)"),
    (36, "Synchronized Release (Remote)"),
    (39, "+NEF(RAW)"),
    (4, "AF-AreaMode D9"),
    (40, "Grid Display"),
    (41, "Virtual Horizon"),
    (42, "Voice Memo"),
    (43, "Wired LAN"),
    (44, "My Menu"),
    (45, "My Menu Top Item"),
    (46, "Playback"),
    (47, "Filtered Playback"),
    (48, "Photo Shooting Bank"),
    (49, "AF Mode/AF Area Mode"),
    (5, "AF-AreaMode D25"),
    (50, "Image Area"),
    (51, "Active-D Lighting"),
    (52, "Exposure Delay Mode"),
    (53, "Shutter/Aperture Lock"),
    (54, "1 Stop Speed/Aperture"),
    (55, "Non-CPU Lens"),
    (56, "None"),
    (6, "AF-AreaMode D49"),
    (7, "AF-AreaMode D105"),
    (8, "AF-AreaMode 3D"),
    (9, "AF-AreaMode Group"),
];

pub static NIKONSETTINGS_MAIN_FUNC1BUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Preset Focus Point - Press To Recall"),
    (10, "AF-AreaMode Group C1"),
    (11, "AF-AreaMode Group C2"),
    (12, "AF-AreaMode Auto Area"),
    (13, "AF-AreaMode + AF-On S"),
    (14, "AF-AreaMode + AF-On D9"),
    (15, "AF-AreaMode + AF-On D25"),
    (16, "AF-AreaMode + AF-On D49"),
    (17, "AF-AreaMode + AF-On D105"),
    (18, "AF-AreaMode + AF-On 3D"),
    (19, "AF-AreaMode + AF-On Group"),
    (2, "Preset Focus Point - Hold To Recall"),
    (20, "AF-AreaMode + AF-On Group C1"),
    (21, "AF-AreaMode + AF-On Group C2"),
    (22, "AF-AreaMode + AF-On Auto Area"),
    (23, "AF-On"),
    (24, "AF Lock Only"),
    (25, "AE Lock (hold)"),
    (26, "AE/WB Lock (hold)"),
    (27, "AE Lock (reset on release)"),
    (28, "AE Lock Only"),
    (29, "AE/AF Lock"),
    (3, "AF-AreaMode S"),
    (30, "FV Lock"),
    (31, "Flash Disable/Enable"),
    (32, "Preview"),
    (33, "Recall Shooting Functions"),
    (34, "Bracketing Burst"),
    (35, "Synchronized Release (Master)"),
    (36, "Synchronized Release (Remote)"),
    (39, "+NEF(RAW)"),
    (4, "AF-AreaMode D9"),
    (40, "Grid Display"),
    (41, "Virtual Horizon"),
    (42, "Voice Memo"),
    (43, "Wired LAN"),
    (44, "My Menu"),
    (45, "My Menu Top Item"),
    (46, "Playback"),
    (47, "Filtered Playback"),
    (48, "Photo Shooting Bank"),
    (49, "AF Mode/AF Area Mode"),
    (5, "AF-AreaMode D25"),
    (50, "Image Area"),
    (51, "Active-D Lighting"),
    (52, "Exposure Delay Mode"),
    (53, "Shutter/Aperture Lock"),
    (54, "1 Stop Speed/Aperture"),
    (55, "Non-CPU Lens"),
    (56, "None"),
    (6, "AF-AreaMode D49"),
    (7, "AF-AreaMode D105"),
    (8, "AF-AreaMode 3D"),
    (9, "AF-AreaMode Group"),
];

pub static NIKONSETTINGS_MAIN_FUNC2BUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Preset Focus Point - Press To Recall"),
    (10, "AF-AreaMode Group C1"),
    (11, "AF-AreaMode Group C2"),
    (12, "AF-AreaMode Auto Area"),
    (13, "AF-AreaMode + AF-On S"),
    (14, "AF-AreaMode + AF-On D9"),
    (15, "AF-AreaMode + AF-On D25"),
    (16, "AF-AreaMode + AF-On D49"),
    (17, "AF-AreaMode + AF-On D105"),
    (18, "AF-AreaMode + AF-On 3D"),
    (19, "AF-AreaMode + AF-On Group"),
    (2, "Preset Focus Point - Hold To Recall"),
    (20, "AF-AreaMode + AF-On Group C1"),
    (21, "AF-AreaMode + AF-On Group C2"),
    (22, "AF-AreaMode + AF-On Auto Area"),
    (23, "AF-On"),
    (24, "AF Lock Only"),
    (25, "AE Lock (hold)"),
    (26, "AE/WB Lock (hold)"),
    (27, "AE Lock (reset on release)"),
    (28, "AE Lock Only"),
    (29, "AE/AF Lock"),
    (3, "AF-AreaMode S"),
    (30, "FV Lock"),
    (31, "Flash Disable/Enable"),
    (32, "Preview"),
    (33, "Recall Shooting Functions"),
    (34, "Bracketing Burst"),
    (35, "Synchronized Release (Master)"),
    (36, "Synchronized Release (Remote)"),
    (39, "+NEF(RAW)"),
    (4, "AF-AreaMode D9"),
    (40, "Grid Display"),
    (41, "Virtual Horizon"),
    (42, "Voice Memo"),
    (43, "Wired LAN"),
    (44, "My Menu"),
    (45, "My Menu Top Item"),
    (46, "Playback"),
    (47, "Filtered Playback"),
    (48, "Photo Shooting Bank"),
    (49, "AF Mode/AF Area Mode"),
    (5, "AF-AreaMode D25"),
    (50, "Image Area"),
    (51, "Active-D Lighting"),
    (52, "Exposure Delay Mode"),
    (53, "Shutter/Aperture Lock"),
    (54, "1 Stop Speed/Aperture"),
    (55, "Non-CPU Lens"),
    (56, "None"),
    (6, "AF-AreaMode D49"),
    (7, "AF-AreaMode D105"),
    (8, "AF-AreaMode 3D"),
    (9, "AF-AreaMode Group"),
];

pub static NIKONSETTINGS_MAIN_AF_ONBUTTON_VALUES: &[(i64, &str)] = &[
    (1, "AF-AreaMode S"),
    (10, "AF-AreaMode Auto Area"),
    (11, "AF-AreaMode + AF-On S"),
    (12, "AF-AreaMode + AF-On D9"),
    (13, "AF-AreaMode + AF-On D25"),
    (14, "AF-AreaMode + AF-On D49"),
    (15, "AF-AreaMode + AF-On D105"),
    (16, "AF-AreaMode + AF-On 3D"),
    (17, "AF-AreaMode + AF-On Group"),
    (18, "AF-AreaMode + AF-On Group C1"),
    (19, "AF-AreaMode + AF-On Group C2"),
    (2, "AF-AreaMode D9"),
    (20, "AF-AreaMode + AF-On Auto Area"),
    (21, "AF-On"),
    (22, "AF Lock Only"),
    (23, "AE Lock (hold)"),
    (24, "AE/WB Lock (hold)"),
    (25, "AE Lock (reset on release)"),
    (26, "AE Lock Only"),
    (27, "AE/AF Lock"),
    (28, "Recall Shooting Functions"),
    (29, "None"),
    (3, "AF-AreaMode D25"),
    (4, "AF-AreaMode D49"),
    (5, "AF-AreaMode D105"),
    (6, "AF-AreaMode 3D"),
    (7, "AF-AreaMode Group"),
    (8, "AF-AreaMode Group C1"),
    (9, "AF-AreaMode Group C2"),
];

pub static NIKONSETTINGS_MAIN_SUBSELECTOR_VALUES: &[(i64, &str)] = &[
    (1, "Same as MultiSelector"),
    (2, "Focus Point Selection"),
];

pub static NIKONSETTINGS_MAIN_SUBSELECTORCENTER_VALUES: &[(i64, &str)] = &[
    (1, "Preset Focus Point - Press To Recall"),
    (10, "AF-AreaMode Group"),
    (11, "AF-AreaMode Group C1"),
    (12, "AF-AreaMode Group C2"),
    (13, "AF-AreaMode Auto Area"),
    (14, "AF-AreaMode + AF-On S"),
    (15, "AF-AreaMode + AF-On D9"),
    (16, "AF-AreaMode + AF-On D25"),
    (17, "AF-AreaMode + AF-On D49"),
    (18, "AF-AreaMode + AF-On D105"),
    (19, "AF-AreaMode + AF-On 3D"),
    (2, "Preset Focus Point - Hold To Recall"),
    (20, "AF-AreaMode + AF-On Group"),
    (21, "AF-AreaMode + AF-On Group C1"),
    (22, "AF-AreaMode + AF-On Group C2"),
    (23, "AF-AreaMode + AF-On Auto Area"),
    (24, "AF-On"),
    (25, "AF Lock Only"),
    (26, "AE Lock (hold)"),
    (27, "AE/WB Lock (hold)"),
    (28, "AE Lock (reset on release)"),
    (29, "AE Lock Only"),
    (3, "Center Focus Point"),
    (30, "AE/AF Lock"),
    (31, "FV Lock"),
    (32, "Flash Disable/Enable"),
    (33, "Preview"),
    (34, "Recall Shooting Functions"),
    (35, "Bracketing Burst"),
    (36, "Synchronized Release (Master)"),
    (37, "Synchronized Release (Remote)"),
    (38, "None"),
    (4, "AF-AreaMode S"),
    (5, "AF-AreaMode D9"),
    (6, "AF-AreaMode D25"),
    (7, "AF-AreaMode D49"),
    (8, "AF-AreaMode D105"),
    (9, "AF-AreaMode 3D"),
];

pub static NIKONSETTINGS_MAIN_LENSFUNC1BUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Preset Focus Point - Press To Recall"),
    (10, "AF-AreaMode Group C1"),
    (11, "AF-AreaMode Group C2"),
    (12, "AF-AreaMode Auto Area"),
    (13, "AF-AreaMode + AF-On S"),
    (14, "AF-AreaMode + AF-On D9"),
    (15, "AF-AreaMode + AF-On D25"),
    (16, "AF-AreaMode + AF-On D49"),
    (17, "AF-AreaMode + AF-On D105"),
    (18, "AF-AreaMode + AF-On 3D"),
    (19, "AF-AreaMode + AF-On Group"),
    (2, "Preset Focus Point - Hold To Recall"),
    (20, "AF-AreaMode + AF-On Group C1"),
    (21, "AF-AreaMode + AF-On Group C2"),
    (22, "AF-AreaMode + AF-On Auto Area"),
    (23, "AF-On"),
    (24, "AF Lock Only"),
    (25, "AE Lock Only"),
    (26, "AE/AF Lock"),
    (27, "Flash Disable/Enable"),
    (28, "Recall Shooting Functions"),
    (29, "Synchronized Release (Master)"),
    (3, "AF-AreaMode S"),
    (30, "Synchronized Release (Remote)"),
    (4, "AF-AreaMode D9"),
    (5, "AF-AreaMode D25"),
    (6, "AF-AreaMode D49"),
    (7, "AF-AreaMode D105"),
    (8, "AF-AreaMode 3D"),
    (9, "AF-AreaMode Group"),
];

pub static NIKONSETTINGS_MAIN_CMDDIALSAPERTURESETTING_VALUES: &[(i64, &str)] = &[
    (1, "Sub-command Dial"),
    (2, "Aperture Ring"),
];

pub static NIKONSETTINGS_MAIN_MULTISELECTOR_VALUES: &[(i64, &str)] = &[
    (1, "Restart Standby Timer"),
    (2, "Do Nothing"),
];

pub static NIKONSETTINGS_MAIN_LIVEVIEWBUTTONOPTIONS_VALUES: &[(i64, &str)] = &[
    (1, "Enable"),
    (2, "Enable (Standby Timer Active)"),
    (3, "Disable"),
];

pub static NIKONSETTINGS_MAIN_LIGHTSWITCH_VALUES: &[(i64, &str)] = &[
    (1, "LCD Backlight"),
    (2, "LCD Backlight and Shooting Information"),
];

pub static NIKONSETTINGS_MAIN_MOVIEPREVIEWBUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Power Aperture (Open)"),
    (2, "Exposure Compensation"),
    (3, "Grid Display"),
    (4, "Zoom (Low)"),
    (5, "Zoom (1:1)"),
    (6, "Zoom (High)"),
    (7, "Image Area"),
    (8, "Microphone Sensitivity"),
    (9, "None"),
];

pub static NIKONSETTINGS_MAIN_MOVIEFUNC1BUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Power Aperture (Close)"),
    (2, "Exposure Compensation"),
    (3, "Grid Display"),
    (4, "Zoom (Low)"),
    (5, "Zoom (1:1)"),
    (6, "Zoom (High)"),
    (7, "Image Area"),
    (8, "Microphone Sensitivity"),
    (9, "None"),
];

pub static NIKONSETTINGS_MAIN_MOVIEFUNC2BUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Grid Display"),
    (2, "Zoom (Low)"),
    (3, "Zoom (1:1)"),
    (4, "Zoom (High)"),
    (5, "Image Area"),
    (6, "Microphone Sensitivity"),
    (7, "None"),
];

pub static NIKONSETTINGS_MAIN_ASSIGNMOVIESUBSELECTOR_VALUES: &[(i64, &str)] = &[
    (1, "Center Focus Point"),
    (10, "Record Movie"),
    (11, "None"),
    (2, "AF Lock Only"),
    (3, "AE Lock (hold)"),
    (4, "AE/WB Lock (hold)"),
    (5, "AE Lock Only"),
    (6, "AE/AF Lock"),
    (7, "Zoom (Low)"),
    (8, "Zoom (1:1)"),
    (9, "Zoom (High)"),
];

pub static NIKONSETTINGS_MAIN_LIMITAFAREAMODESELD9_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITAFAREAMODESELD25_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITAFAREAMODESEL3D_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITAFAREAMODESELGROUP_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITAFAREAMODESELAUTO_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITSELECTABLEIMAGEAREA5TO4_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITSELECTABLEIMAGEAREA1TO1_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_ISOAUTOFLASHLIMIT_VALUES: &[(i64, &str)] = &[
    (1, "Same As Without Flash"),
    (10, "ISO 800"),
    (11, "ISO 1000"),
    (13, "ISO 1250"),
    (14, "ISO 1600"),
    (15, "ISO 2000"),
    (17, "ISO 2500"),
    (18, "ISO 3200"),
    (19, "ISO 4000"),
    (2, "ISO 200"),
    (21, "ISO 5000"),
    (22, "ISO 6400"),
    (23, "ISO 8000"),
    (25, "ISO 10000"),
    (26, "ISO 12800"),
    (27, "ISO 16000"),
    (29, "ISO 20000"),
    (3, "ISO 250"),
    (30, "ISO 25600"),
    (31, "ISO 32000"),
    (33, "ISO 40000"),
    (34, "ISO 51200"),
    (35, "ISO 64000"),
    (36, "ISO 72000"),
    (37, "ISO 81200"),
    (38, "ISO 102400"),
    (39, "ISO Hi 0.3"),
    (40, "ISO Hi 0.5"),
    (41, "ISO Hi 0.7"),
    (42, "ISO Hi 1.0"),
    (43, "ISO Hi 2.0"),
    (44, "ISO Hi 3.0"),
    (45, "ISO Hi 4.0"),
    (46, "ISO Hi 5.0"),
    (5, "ISO 320"),
    (6, "ISO 400"),
    (7, "ISO 500"),
    (9, "ISO 640"),
];

pub static NIKONSETTINGS_MAIN_PHOTOSHOOTINGMENUBANK_VALUES: &[(i64, &str)] = &[
    (1, "A"),
    (2, "B"),
    (3, "C"),
    (4, "D"),
];

pub static NIKONSETTINGS_MAIN_CUSTOMSETTINGSBANK_VALUES: &[(i64, &str)] = &[
    (1, "A"),
    (2, "B"),
    (3, "C"),
    (4, "D"),
];

pub static NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELPINPOINT_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELDYNAMIC_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELWIDEAF_S_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELWIDEAF_L_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LOWLIGHTAF_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_LIMITSELECTABLEIMAGEAREADX_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITSELECTABLEIMAGEAREA16TO9_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_APPLYSETTINGSTOLIVEVIEW_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_FOCUSPEAKINGLEVEL_VALUES: &[(i64, &str)] = &[
    (1, "High Sensitivity"),
    (2, "Standard Sensitivity"),
    (3, "Low Sensitivity"),
    (4, "Off"),
];

pub static NIKONSETTINGS_MAIN_LENSCONTROLRING_VALUES: &[(i64, &str)] = &[
    (1, "Aperture"),
    (2, "Exposure Compensation"),
    (3, "ISO Sensitivity"),
    (4, "None (Disabled)"),
];

pub static NIKONSETTINGS_MAIN_MOVIEMULTISELECTOR_VALUES: &[(i64, &str)] = &[
    (1, "Center Focus Point"),
    (2, "Zoom (Low)"),
    (3, "Zoom (1:1)"),
    (4, "Zoom (High)"),
    (5, "Record Movie"),
    (6, "None"),
];

pub static NIKONSETTINGS_MAIN_MOVIEAFSPEEDAPPLY_VALUES: &[(i64, &str)] = &[
    (1, "Always"),
    (2, "Only During Recording"),
];

pub static NIKONSETTINGS_MAIN_MOVIEAFTRACKINGSENSITIVITY_VALUES: &[(i64, &str)] = &[
    (1, "1 (High)"),
    (2, "2"),
    (3, "3"),
    (4, "4 (Normal)"),
    (5, "5"),
    (6, "6"),
    (7, "7 (Low)"),
];

pub static NIKONSETTINGS_MAIN_MOVIEHIGHLIGHTDISPLAYPATTERN_VALUES: &[(i64, &str)] = &[
    (1, "Pattern 1"),
    (2, "Pattern 2"),
    (3, "Off"),
];

pub static NIKONSETTINGS_MAIN_SUBDIALFRAMEADVANCERATING5_VALUES: &[(i64, &str)] = &[
    (1, "No"),
    (2, "Yes"),
];

pub static NIKONSETTINGS_MAIN_SUBDIALFRAMEADVANCERATING4_VALUES: &[(i64, &str)] = &[
    (1, "No"),
    (2, "Yes"),
];

pub static NIKONSETTINGS_MAIN_SUBDIALFRAMEADVANCERATING3_VALUES: &[(i64, &str)] = &[
    (1, "No"),
    (2, "Yes"),
];

pub static NIKONSETTINGS_MAIN_SUBDIALFRAMEADVANCERATING2_VALUES: &[(i64, &str)] = &[
    (1, "No"),
    (2, "Yes"),
];

pub static NIKONSETTINGS_MAIN_SUBDIALFRAMEADVANCERATING1_VALUES: &[(i64, &str)] = &[
    (1, "No"),
    (2, "Yes"),
];

pub static NIKONSETTINGS_MAIN_SUBDIALFRAMEADVANCERATING0_VALUES: &[(i64, &str)] = &[
    (1, "No"),
    (2, "Yes"),
];

pub static NIKONSETTINGS_MAIN_MOVIEAF_ONBUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Center Focus Point"),
    (10, "Zoom (High)"),
    (11, "Record Movie"),
    (12, "None"),
    (2, "AF-On"),
    (3, "AF Lock Only"),
    (4, "AE Lock (hold)"),
    (5, "AE Lock Only"),
    (6, "AE/AF Lock"),
    (7, "LiveView Info Display On/Off"),
    (8, "Zoom (Low)"),
    (9, "Zoom (1:1)"),
];

pub static NIKONSETTINGS_MAIN_SECONDARYSLOTFUNCTION_VALUES: &[(i64, &str)] = &[
    (1, "Overflow"),
    (2, "Backup"),
    (3, "NEF Primary + JPG Secondary"),
    (4, "JPG Primary + JPG Secondary"),
];

pub static NIKONSETTINGS_MAIN_SILENTPHOTOGRAPHY_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_EXTENDEDSHUTTERSPEEDS_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_HDMIBITDEPTH_VALUES: &[(i64, &str)] = &[
    (1, "8 Bit"),
    (2, "10 Bit"),
];

pub static NIKONSETTINGS_MAIN_HDMIOUTPUTHDR_VALUES: &[(i64, &str)] = &[
    (2, "On"),
    (3, "Off"),
];

pub static NIKONSETTINGS_MAIN_HDMIVIEWASSIST_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_BRACKETSET_VALUES: &[(i64, &str)] = &[
    (1, "AE/Flash"),
    (2, "AE"),
    (3, "Flash"),
    (4, "White Balance"),
    (5, "Active-D Lighting"),
];

pub static NIKONSETTINGS_MAIN_BRACKETPROGRAM_VALUES: &[(i64, &str)] = &[
    (15, "+3F"),
    (16, "-3F"),
    (17, "+2F"),
    (18, "-2F"),
    (19, "Disabled"),
    (20, "3F"),
    (21, "5F"),
    (22, "7F"),
    (23, "9F"),
];

pub static NIKONSETTINGS_MAIN_BRACKETINCREMENT_VALUES: &[(i64, &str)] = &[
    (1, "0.3"),
    (3, "0.5"),
    (4, "1.0"),
    (5, "2.0"),
    (6, "3.0"),
];

pub static NIKONSETTINGS_MAIN_GROUPAREAC1_VALUES: &[(i64, &str)] = &[
    (1, "1x7"),
    (10, "7x7"),
    (11, "7x5"),
    (12, "7x3"),
    (13, "7x1"),
    (14, "11x3"),
    (15, "11x1"),
    (16, "15x3"),
    (17, "15x1"),
    (2, "1x5"),
    (3, "3x7"),
    (4, "3x5"),
    (5, "3x3"),
    (6, "5x7"),
    (7, "5x5"),
    (8, "5x3"),
    (9, "5x1"),
];

pub static NIKONSETTINGS_MAIN_AUTOAREAAFSTARTINGPOINT_VALUES: &[(i64, &str)] = &[
    (1, "Enable"),
    (2, "Disable"),
];

pub static NIKONSETTINGS_MAIN_FOCUSPOINTPERSISTENCE_VALUES: &[(i64, &str)] = &[
    (1, "Auto"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_LIMITAFAREAMODESELD49_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITAFAREAMODESELD105_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITAFAREAMODESELGROUPC1_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITAFAREAMODESELGROUPC2_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_AUTOFOCUSMODERESTRICTIONS_VALUES: &[(i64, &str)] = &[
    (1, "AF-S"),
    (2, "AF-C"),
    (3, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_FOCUSPOINTBRIGHTNESS_VALUES: &[(i64, &str)] = &[
    (1, "Extra High"),
    (2, "High"),
    (3, "Normal"),
    (4, "Low"),
];

pub static NIKONSETTINGS_MAIN_QUIETSHUTTERSHOOTINGSPEED_VALUES: &[(i64, &str)] = &[
    (1, "Single"),
    (2, "5 fps"),
    (3, "4 fps"),
    (4, "3 fps"),
    (5, "2 fps"),
    (6, "1 fps"),
];

pub static NIKONSETTINGS_MAIN_AF_CPRIORITYSEL_VALUES: &[(i64, &str)] = &[
    (1, "Release"),
    (2, "Release + Focus"),
    (3, "Focus + Release"),
    (4, "Focus"),
];

pub static NIKONSETTINGS_MAIN_LIMITRELEASEMODESELCL_VALUES: &[(i64, &str)] = &[
    (0, "No Limit"),
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITRELEASEMODESELCH_VALUES: &[(i64, &str)] = &[
    (0, "No Limit"),
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITRELEASEMODESELQ_VALUES: &[(i64, &str)] = &[
    (0, "No Limit"),
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITRELEASEMODESELTIMER_VALUES: &[(i64, &str)] = &[
    (0, "No Limit"),
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITRELEASEMODESELMIRROR_UP_VALUES: &[(i64, &str)] = &[
    (0, "No Limit"),
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_REARCONTROPANELDISPLAY_VALUES: &[(i64, &str)] = &[
    (1, "Release Mode"),
    (2, "Frame Count"),
];

pub static NIKONSETTINGS_MAIN_FLASHBURSTPRIORITY_VALUES: &[(i64, &str)] = &[
    (1, "Frame Rate"),
    (2, "Exposure"),
];

pub static NIKONSETTINGS_MAIN_RECALLSHOOTFUNCEXPOSUREMODE_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "On"),
];

pub static NIKONSETTINGS_MAIN_RECALLSHOOTFUNCSHUTTERSPEED_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "On"),
];

pub static NIKONSETTINGS_MAIN_ISOAUTOSHUTTERTIME_VALUES: &[(i64, &str)] = &[
    (1, "Auto (Slowest)"),
    (10, "1/1600 s"),
    (11, "1/1250 s"),
    (12, "1/1000 s"),
    (13, "1/800 s"),
    (14, "1/640 s"),
    (15, "1/500 s"),
    (16, "1/400 s"),
    (17, "1/320 s"),
    (18, "1/250 s"),
    (19, "1/200 s"),
    (2, "Auto (Slower)"),
    (20, "1/160 s"),
    (21, "1/125 s"),
    (22, "1/100 s"),
    (23, "1/80 s"),
    (24, "1/60 s"),
    (25, "1/50 s"),
    (26, "1/40 s"),
    (27, "1/30 s"),
    (28, "1/25 s"),
    (29, "1/20 s"),
    (3, "Auto"),
    (30, "1/15 s"),
    (31, "1/13 s"),
    (32, "1/10 s"),
    (33, "1/8 s"),
    (34, "1/6 s"),
    (35, "1/5 s"),
    (36, "1/4 s"),
    (37, "1/3 s"),
    (38, "1/2.5 s"),
    (39, "1/2 s"),
    (4, "Auto (Faster)"),
    (40, "1/1.6 s"),
    (41, "1/1.3 s"),
    (42, "1 s"),
    (43, "1.3 s"),
    (44, "1.6 s"),
    (45, "2 s"),
    (46, "2.5 s"),
    (47, "3 s"),
    (48, "4 s"),
    (49, "5 s"),
    (5, "Auto (Fastest)"),
    (50, "6 s"),
    (51, "8 s"),
    (52, "10 s"),
    (53, "13 s"),
    (54, "15 s"),
    (55, "20 s"),
    (56, "25 s"),
    (57, "30 s"),
    (6, "1/4000 s"),
    (7, "1/3200 s"),
    (8, "1/2500 s"),
    (9, "1/2000 s"),
];

pub static NIKONSETTINGS_MAIN_AF_SPRIORITYSEL_VALUES: &[(i64, &str)] = &[
    (1, "Release"),
    (2, "Focus"),
];

pub static NIKONSETTINGS_MAIN_RECALLSHOOTFUNCAPERTURE_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "On"),
];

pub static NIKONSETTINGS_MAIN_RECALLSHOOTFUNCEXPOSURECOMP_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "On"),
];

pub static NIKONSETTINGS_MAIN_RECALLSHOOTFUNCISO_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "On"),
];

pub static NIKONSETTINGS_MAIN_RECALLSHOOTFUNCMETERINGMODE_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "On"),
];

pub static NIKONSETTINGS_MAIN_RECALLSHOOTFUNCWHITEBALANCE_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "On"),
];

pub static NIKONSETTINGS_MAIN_RECALLSHOOTFUNCAFAREAMODE_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "On"),
];

pub static NIKONSETTINGS_MAIN_RECALLSHOOTFUNCFOCUSTRACKING_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "On"),
];

pub static NIKONSETTINGS_MAIN_RECALLSHOOTFUNCAF_ON_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "On"),
];

pub static NIKONSETTINGS_MAIN_VERTICALFUNCBUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Preset Focus Point"),
    (10, "Voice Memo"),
    (11, "Playback"),
    (12, "Filtered Playback"),
    (13, "Photo Shooting Bank"),
    (14, "Exposure Mode"),
    (15, "Exposure Comp"),
    (16, "AF Mode/AF Area Mode"),
    (17, "Image Area"),
    (18, "ISO"),
    (19, "Active-D Lighting"),
    (2, "AE Lock (hold)"),
    (20, "Metering"),
    (21, "Exposure Delay Mode"),
    (22, "Shutter/Aperture Lock"),
    (23, "1 Stop Speed/Aperture"),
    (24, "Rating 0"),
    (25, "Rating 5"),
    (26, "Rating 4"),
    (27, "Rating 3"),
    (28, "Rating 2"),
    (29, "Rating 1"),
    (3, "AE/WB Lock (hold)"),
    (30, "Candidate For Deletion"),
    (31, "Non-CPU Lens"),
    (32, "None"),
    (4, "AE Lock (reset on release)"),
    (5, "FV Lock"),
    (6, "Preview"),
    (7, "+NEF(RAW)"),
    (8, "Grid Display"),
    (9, "Virtual Horizon"),
];

pub static NIKONSETTINGS_MAIN_FUNC3BUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Voice Memo"),
    (10, "Rating 3"),
    (11, "Rating 2"),
    (12, "Rating 1"),
    (13, "Candidate For Deletion"),
    (14, "None"),
    (2, "Select To Send"),
    (3, "Wired LAN"),
    (4, "My Menu"),
    (5, "My Menu Top Item"),
    (6, "Filtered Playback"),
    (7, "Rating 0"),
    (8, "Rating 5"),
    (9, "Rating 4"),
];

pub static NIKONSETTINGS_MAIN_VERTICALAF_ONBUTTON_VALUES: &[(i64, &str)] = &[
    (1, "AF-AreaMode S"),
    (10, "AF-AreaMode Auto Area"),
    (11, "AF-AreaMode + AF-On S"),
    (12, "AF-AreaMode + AF-On D9"),
    (13, "AF-AreaMode + AF-On D25"),
    (14, "AF-AreaMode + AF-On D49"),
    (15, "AF-AreaMode + AF-On D105"),
    (16, "AF-AreaMode + AF-On 3D"),
    (17, "AF-AreaMode + AF-On Group"),
    (18, "AF-AreaMode + AF-On Group C1"),
    (19, "AF-AreaMode + AF-On Group C2"),
    (2, "AF-AreaMode D9"),
    (20, "AF-AreaMode + AF-On Auto Area"),
    (21, "Same as AF-On"),
    (22, "AF-On"),
    (23, "AF Lock Only"),
    (24, "AE Lock (hold)"),
    (25, "AE/WB Lock (hold)"),
    (26, "AE Lock (reset on release)"),
    (27, "AE Lock Only"),
    (28, "AE/AF Lock"),
    (29, "Recall Shooting Functions"),
    (3, "AF-AreaMode D25"),
    (30, "None"),
    (4, "AF-AreaMode D49"),
    (5, "AF-AreaMode D105"),
    (6, "AF-AreaMode 3D"),
    (7, "AF-AreaMode Group"),
    (8, "AF-AreaMode Group C1"),
    (9, "AF-AreaMode Group C2"),
];

pub static NIKONSETTINGS_MAIN_VERTICALMULTISELECTOR_VALUES: &[(i64, &str)] = &[
    (1, "Same as MultiSelector"),
    (2, "Focus Point Selection"),
];

pub static NIKONSETTINGS_MAIN_METERINGBUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Photo Shooting Bank"),
    (2, "Image Area"),
    (3, "Active-D Lighting"),
    (4, "Metering"),
    (5, "Exposure Delay Mode"),
    (6, "Shutter/Aperture Lock"),
    (7, "1 Stop Speed/Aperture"),
    (8, "Non-CPU Lens"),
    (9, "None"),
];

pub static NIKONSETTINGS_MAIN_PLAYBACKFLICKUP_VALUES: &[(i64, &str)] = &[
    (1, "Rating"),
    (2, "Select To Send"),
    (3, "Protect"),
    (4, "Voice Memo"),
    (5, "None"),
];

pub static NIKONSETTINGS_MAIN_PLAYBACKFLICKUPRATING_VALUES: &[(i64, &str)] = &[
    (1, "Rating 5"),
    (2, "Rating 4"),
    (3, "Rating 3"),
    (4, "Rating 2"),
    (5, "Rating 1"),
    (6, "Candidate for Deletion"),
];

pub static NIKONSETTINGS_MAIN_PLAYBACKFLICKDOWN_VALUES: &[(i64, &str)] = &[
    (1, "Rating"),
    (2, "Select To Send"),
    (3, "Protect"),
    (4, "Voice Memo"),
    (5, "None"),
];

pub static NIKONSETTINGS_MAIN_PLAYBACKFLICKDOWNRATING_VALUES: &[(i64, &str)] = &[
    (1, "Rating 5"),
    (2, "Rating 4"),
    (3, "Rating 3"),
    (4, "Rating 2"),
    (5, "Rating 1"),
    (6, "Candidate for Deletion"),
];

pub static NIKONSETTINGS_MAIN_MOVIEFUNC3BUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Record Movie"),
    (2, "My Menu"),
    (3, "My Menu Top Item"),
    (4, "None"),
];

pub static NIKONSETTINGS_MAIN_AFPOINTSEL_VALUES: &[(i64, &str)] = &[
    (1, "105 Points"),
    (2, "27 Points"),
    (3, "15 Points"),
];

pub static NIKONSETTINGS_MAIN_SHUTTERTYPE_VALUES: &[(i64, &str)] = &[
    (1, "Auto"),
    (2, "Mechanical"),
    (3, "Electronic"),
];

pub static NIKONSETTINGS_MAIN_LENSFUNC2BUTTON_VALUES: &[(i64, &str)] = &[
    (1, "AF-On"),
    (10, "Matrix Metering"),
    (11, "Center-weighted Metering"),
    (12, "Spot Metering"),
    (13, "Highlight-weighted Metering"),
    (14, "Bracketing Burst"),
    (15, "Synchronized Release (Master)"),
    (16, "Synchronized Release (Remote)"),
    (19, "+NEF(RAW)"),
    (2, "AF Lock Only"),
    (20, "Subject Tracking"),
    (21, "Grid Display"),
    (22, "Zoom (Low)"),
    (23, "Zoom (1:1)"),
    (24, "Zoom (High)"),
    (25, "My Menu"),
    (26, "My Menu Top Item"),
    (27, "Playback"),
    (28, "None"),
    (3, "AE Lock (hold)"),
    (4, "AE Lock (reset on release)"),
    (5, "AE Lock Only"),
    (6, "AE/AF Lock"),
    (7, "FV Lock"),
    (8, "Flash Disable/Enable"),
    (9, "Preview"),
];

pub static NIKONSETTINGS_MAIN_AFACTIVATION_VALUES: &[(i64, &str)] = &[
    (1, "Shutter/AF-On"),
    (2, "AF-On Only"),
];

pub static NIKONSETTINGS_MAIN_USBPOWERDELIVERY_VALUES: &[(i64, &str)] = &[
    (1, "Enable"),
    (2, "Disable"),
];

pub static NIKONSETTINGS_MAIN_ENERGYSAVINGMODE_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_BRACKETINGBURSTOPTIONS_VALUES: &[(i64, &str)] = &[
    (1, "Enable"),
    (2, "Disable"),
];

pub static NIKONSETTINGS_MAIN_FOCUSPOINTWRAP_VALUES: &[(i64, &str)] = &[
    (1, "Wrap"),
    (2, "No Wrap"),
];

pub static NIKONSETTINGS_MAIN_PRIMARYSLOT_VALUES: &[(i64, &str)] = &[
    (1, "CFexpress/XQD Card"),
    (2, "SD Card"),
];

pub static NIKONSETTINGS_MAIN_REVERSEFOCUSRING_VALUES: &[(i64, &str)] = &[
    (1, "Not Reversed"),
    (2, "Reversed"),
];

pub static NIKONSETTINGS_MAIN_VERTICALAFONBUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Same as AF-On Button"),
    (10, "Zoom (Low)"),
    (11, "Zoom (1:1)"),
    (12, "Zoom (High)"),
    (13, "None"),
    (2, "Select Center Focus Point"),
    (3, "AF-On"),
    (4, "AF Lock Only"),
    (5, "AE Lock (hold)"),
    (6, "AE Lock (reset on release)"),
    (7, "AE Lock Only"),
    (8, "AE/AF Lock"),
    (9, "LiveView Info Display On/Off"),
];

pub static NIKONSETTINGS_MAIN_VERTICALMOVIEFUNCBUTTON_VALUES: &[(i64, &str)] = &[
    (1, "LiveView Info Display On/Off"),
    (2, "Record Movie"),
    (3, "Exposure Compensation"),
    (4, "ISO"),
    (5, "None"),
];

pub static NIKONSETTINGS_MAIN_VERTICALMOVIEAFONBUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Same as AF-On"),
    (10, "Zoom (1:1)"),
    (11, "Zoom (High)"),
    (12, "Record Movie"),
    (13, "None"),
    (2, "Center Focus Point"),
    (3, "AF-On"),
    (4, "AF Lock Only"),
    (5, "AE Lock (hold)"),
    (6, "AE Lock Only"),
    (7, "AE/AF Lock"),
    (8, "LiveView Info Display On/Off"),
    (9, "Zoom (Low)"),
];

pub static NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELAUTOPEOPLE_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELAUTOANIMALS_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELWIDELPEOPLE_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_LIMITAF_AREAMODESELWIDELANIMALS_VALUES: &[(i64, &str)] = &[
    (1, "Limit"),
    (2, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_SAVEFOCUS_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_AFAREAMODE_VALUES: &[(i64, &str)] = &[
    (10, "Auto (Animals)"),
    (2, "Single-point"),
    (3, "Dynamic-area"),
    (4, "Wide (S)"),
    (5, "Wide (L)"),
    (6, "Wide (L-people)"),
    (7, "Wide (L-animals)"),
    (8, "Auto"),
    (9, "Auto (People)"),
];

pub static NIKONSETTINGS_MAIN_MOVIEAFAREAMODE_VALUES: &[(i64, &str)] = &[
    (1, "Single-point"),
    (2, "Wide (S)"),
    (3, "Wide (L)"),
    (4, "Wide (L-people)"),
    (5, "Wide (L-animals)"),
    (6, "Auto"),
    (7, "Auto (People)"),
    (8, "Auto (Animals)"),
];

pub static NIKONSETTINGS_MAIN_PREFERSUBSELECTORCENTER_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "On"),
];

pub static NIKONSETTINGS_MAIN_KEEPEXPOSUREWITHTELECONVERTER_VALUES: &[(i64, &str)] = &[
    (1, "Off"),
    (2, "Shutter Speed"),
    (3, "ISO"),
];

pub static NIKONSETTINGS_MAIN_MANUALFOCUSPOINTILLUMINATION_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "On During Focus Point Selection Only"),
];

pub static NIKONSETTINGS_MAIN_FOCUSPOINTSELECTIONSPEED_VALUES: &[(i64, &str)] = &[
    (1, "Normal"),
    (2, "High"),
    (3, "Very High"),
];

pub static NIKONSETTINGS_MAIN_AF_ASSISTILLUMINATOR_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_MANUALFOCUSRINGINAFMODE_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_ISOSTEPSIZE_VALUES: &[(i64, &str)] = &[
    (1, "1/3 EV"),
    (2, "1/2 EV"),
    (3, "1 EV"),
];

pub static NIKONSETTINGS_MAIN_EXPOSURECONTROLSTEPSIZE_VALUES: &[(i64, &str)] = &[
    (1, "1/3 EV"),
    (2, "1/2 EV"),
    (3, "1 EV"),
];

pub static NIKONSETTINGS_MAIN_EASYEXPOSURECOMPENSATION_VALUES: &[(i64, &str)] = &[
    (1, "On (auto reset)"),
    (2, "On"),
    (3, "Off"),
];

pub static NIKONSETTINGS_MAIN_MATRIXMETERING_VALUES: &[(i64, &str)] = &[
    (1, "Face Detection On"),
    (2, "Face Detection Off"),
];

pub static NIKONSETTINGS_MAIN_CENTERWEIGHTEDAREASIZE_VALUES: &[(i64, &str)] = &[
    (1, "8 mm"),
    (2, "12 mm"),
    (3, "15 mm"),
    (4, "20 mm"),
    (5, "Average"),
];

pub static NIKONSETTINGS_MAIN_SHUTTERRELEASEBUTTONAE_L_VALUES: &[(i64, &str)] = &[
    (1, "On (Half Press)"),
    (2, "On (Burst Mode)"),
    (3, "Off"),
];

pub static NIKONSETTINGS_MAIN_STANDBYMONITOROFFTIME_VALUES: &[(i64, &str)] = &[
    (1, "4 s"),
    (2, "6 s"),
    (3, "10 s"),
    (4, "30 s"),
    (5, "1 min"),
    (6, "5 min"),
    (7, "10 min"),
    (8, "30 min"),
    (9, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_SELFTIMERTIME_VALUES: &[(i64, &str)] = &[
    (1, "2 s"),
    (2, "5 s"),
    (3, "10 s"),
    (4, "20 s"),
];

pub static NIKONSETTINGS_MAIN_SELFTIMERSHOTINTERVAL_VALUES: &[(i64, &str)] = &[
    (1, "0.5 s"),
    (2, "1 s"),
    (3, "2 s"),
    (4, "3 s"),
];

pub static NIKONSETTINGS_MAIN_PLAYBACKMONITOROFFTIME_VALUES: &[(i64, &str)] = &[
    (1, "4 s"),
    (2, "10 s"),
    (3, "20 s"),
    (4, "1 min"),
    (5, "5 min"),
    (6, "10 min"),
];

pub static NIKONSETTINGS_MAIN_MENUMONITOROFFTIME_VALUES: &[(i64, &str)] = &[
    (1, "4 s"),
    (2, "10 s"),
    (3, "20 s"),
    (4, "1 min"),
    (5, "5 min"),
    (6, "10 min"),
];

pub static NIKONSETTINGS_MAIN_SHOOTINGINFOMONITOROFFTIME_VALUES: &[(i64, &str)] = &[
    (1, "4 s"),
    (2, "10 s"),
    (3, "20 s"),
    (4, "1 min"),
    (5, "5 min"),
    (6, "10 min"),
];

pub static NIKONSETTINGS_MAIN_IMAGEREVIEWMONITOROFFTIME_VALUES: &[(i64, &str)] = &[
    (1, "2 s"),
    (2, "4 s"),
    (3, "10 s"),
    (4, "20 s"),
    (5, "1 min"),
    (6, "5 min"),
    (7, "10 min"),
];

pub static NIKONSETTINGS_MAIN_LIVEVIEWMONITOROFFTIME_VALUES: &[(i64, &str)] = &[
    (1, "5 min"),
    (2, "10 min"),
    (3, "15 min"),
    (4, "20 min"),
    (5, "30 min"),
    (6, "No Limit"),
];

pub static NIKONSETTINGS_MAIN_EXPOSUREDELAYMODE_VALUES: &[(i64, &str)] = &[
    (1, "3 s"),
    (2, "2 s"),
    (3, "1 s"),
    (4, "0.5 s"),
    (5, "0.2 s"),
    (6, "Off"),
];

pub static NIKONSETTINGS_MAIN_ELECTRONICFRONT_CURTAINSHUTTER_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_FILENUMBERSEQUENCE_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_FRAMINGGRIDDISPLAY_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_LCDILLUMINATION_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_OPTICALVR_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "Off"),
];

pub static NIKONSETTINGS_MAIN_FLASHSYNCSPEED_VALUES: &[(i64, &str)] = &[
    (1, "1/250 s (auto FP)"),
    (2, "1/250 s"),
    (3, "1/200 s"),
    (4, "1/160 s"),
    (5, "1/125 s"),
    (6, "1/100 s"),
    (7, "1/80 s"),
    (8, "1/60 s"),
];

pub static NIKONSETTINGS_MAIN_FLASHSHUTTERSPEED_VALUES: &[(i64, &str)] = &[
    (1, "1/60 s"),
    (2, "1/30 s"),
    (3, "1/15 s"),
    (4, "1/8 s"),
    (5, "1/4 s"),
    (6, "1/2 s"),
    (7, "1 s"),
    (8, "2 s"),
];

pub static NIKONSETTINGS_MAIN_FLASHEXPOSURECOMPAREA_VALUES: &[(i64, &str)] = &[
    (1, "Entire Frame"),
    (2, "Background Only"),
];

pub static NIKONSETTINGS_MAIN_AUTOFLASHISOSENSITIVITY_VALUES: &[(i64, &str)] = &[
    (1, "Subject and Background"),
    (2, "Subject Only"),
];

pub static NIKONSETTINGS_MAIN_ASSIGNBKTBUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Auto Bracketing"),
    (2, "Multiple Exposure"),
    (3, "HDR (high dynamic range)"),
    (4, "None"),
];

pub static NIKONSETTINGS_MAIN_ASSIGNMOVIERECORDBUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Voice Memo"),
    (2, "Photo Shooting Bank"),
    (3, "Exposure Mode"),
    (4, "AF Mode/AF Area Mode"),
    (5, "Image Area"),
    (6, "Shutter/Aperture Lock"),
    (7, "None"),
];

pub static NIKONSETTINGS_MAIN_MULTISELECTORSHOOTMODE_VALUES: &[(i64, &str)] = &[
    (1, "Select Center Focus Point"),
    (2, "Preset Focus Point - Press To Recall"),
    (3, "Preset Focus Point - Hold To Recall"),
    (4, "None"),
];

pub static NIKONSETTINGS_MAIN_MULTISELECTORPLAYBACKMODE_VALUES: &[(i64, &str)] = &[
    (1, "Filtered Playback"),
    (2, "View Histograms"),
    (3, "Zoom (Low)"),
    (4, "Zoom (1:1)"),
    (5, "Zoom (High)"),
    (6, "Choose Folder"),
];

pub static NIKONSETTINGS_MAIN_MULTISELECTORLIVEVIEW_VALUES: &[(i64, &str)] = &[
    (1, "Select Center Focus Point"),
    (2, "Zoom (Low)"),
    (3, "Zoom (1:1)"),
    (4, "Zoom (High)"),
    (5, "None"),
];

pub static NIKONSETTINGS_MAIN_CMDDIALSCHANGEMAINSUB_VALUES: &[(i64, &str)] = &[
    (1, "Autofocus On, Exposure On"),
    (2, "Autofocus Off, Exposure On"),
];

pub static NIKONSETTINGS_MAIN_CMDDIALSMENUANDPLAYBACK_VALUES: &[(i64, &str)] = &[
    (1, "On"),
    (2, "On (Image Review Excluded)"),
    (3, "Off"),
];

pub static NIKONSETTINGS_MAIN_SUBDIALFRAMEADVANCE_VALUES: &[(i64, &str)] = &[
    (1, "10 Frames"),
    (2, "50 Frames"),
    (3, "Rating"),
    (4, "Protect"),
    (5, "Stills Only"),
    (6, "Movies Only"),
    (7, "Folder"),
];

pub static NIKONSETTINGS_MAIN_RELEASEBUTTONTOUSEDIAL_VALUES: &[(i64, &str)] = &[
    (1, "Yes"),
    (2, "No"),
];

pub static NIKONSETTINGS_MAIN_REVERSEINDICATORS_VALUES: &[(i64, &str)] = &[
    (1, "+ 0 -"),
    (2, "- 0 +"),
];

pub static NIKONSETTINGS_MAIN_MOVIESHUTTERBUTTON_VALUES: &[(i64, &str)] = &[
    (1, "Take Photo"),
    (2, "Record Movie"),
];

pub static NIKONSETTINGS_MAIN_LANGUAGE_VALUES: &[(i64, &str)] = &[
    (15, "Portuguese (Br)"),
    (5, "English"),
    (6, "Spanish"),
    (8, "French"),
];


/// Look up a tag by ID in the main table.
pub fn lookup(_tag_id: u16) -> Option<&'static TagDef> {
    // Default to main table - override in specific modules
    None
}

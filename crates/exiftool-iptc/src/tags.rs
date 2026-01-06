//! IPTC tag definitions.
//!
//! Based on IPTC-IIM specification and ExifTool IPTC.pm

use phf::phf_map;

/// Tag data format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagFormat {
    String,   // Variable-length string
    Text,     // Long text (2000 chars)
    Digits,   // Numeric digits as string
    Int16u,   // 16-bit unsigned int
    Binary,   // Raw binary data
}

/// Tag information.
#[derive(Debug, Clone, Copy)]
pub struct TagInfo {
    pub name: &'static str,
    pub format: TagFormat,
    pub is_list: bool,      // Can have multiple values
    pub max_len: Option<u16>,
}

impl TagInfo {
    const fn new(name: &'static str, format: TagFormat) -> Self {
        Self { name, format, is_list: false, max_len: None }
    }

    const fn list(name: &'static str, format: TagFormat) -> Self {
        Self { name, format, is_list: true, max_len: None }
    }

    const fn with_len(name: &'static str, format: TagFormat, max_len: u16) -> Self {
        Self { name, format, is_list: false, max_len: Some(max_len) }
    }
}

// ============================================================================
// Record 1: Envelope Record
// ============================================================================

static ENVELOPE_TAGS: phf::Map<u8, TagInfo> = phf_map! {
    0u8 => TagInfo::new("EnvelopeRecordVersion", TagFormat::Int16u),
    5u8 => TagInfo::list("Destination", TagFormat::String),
    20u8 => TagInfo::new("FileFormat", TagFormat::Int16u),
    22u8 => TagInfo::new("FileVersion", TagFormat::Int16u),
    30u8 => TagInfo::with_len("ServiceIdentifier", TagFormat::String, 10),
    40u8 => TagInfo::with_len("EnvelopeNumber", TagFormat::Digits, 8),
    50u8 => TagInfo::list("ProductID", TagFormat::String),
    60u8 => TagInfo::with_len("EnvelopePriority", TagFormat::Digits, 1),
    70u8 => TagInfo::with_len("DateSent", TagFormat::Digits, 8),
    80u8 => TagInfo::with_len("TimeSent", TagFormat::String, 11),
    90u8 => TagInfo::with_len("CodedCharacterSet", TagFormat::String, 32),
    100u8 => TagInfo::with_len("UniqueObjectName", TagFormat::String, 80),
    120u8 => TagInfo::new("ARMIdentifier", TagFormat::Int16u),
    122u8 => TagInfo::new("ARMVersion", TagFormat::Int16u),
};

static ENVELOPE_TAGS_BY_NAME: phf::Map<&'static str, u8> = phf_map! {
    "EnvelopeRecordVersion" => 0u8,
    "Destination" => 5u8,
    "FileFormat" => 20u8,
    "FileVersion" => 22u8,
    "ServiceIdentifier" => 30u8,
    "EnvelopeNumber" => 40u8,
    "ProductID" => 50u8,
    "EnvelopePriority" => 60u8,
    "DateSent" => 70u8,
    "TimeSent" => 80u8,
    "CodedCharacterSet" => 90u8,
    "UniqueObjectName" => 100u8,
    "ARMIdentifier" => 120u8,
    "ARMVersion" => 122u8,
};

/// Get envelope tag info by ID.
pub fn envelope_tag(id: u8) -> Option<&'static TagInfo> {
    ENVELOPE_TAGS.get(&id)
}

/// Get envelope tag by name.
pub fn envelope_tag_by_name(name: &str) -> Option<(u8, &'static TagInfo)> {
    ENVELOPE_TAGS_BY_NAME.get(name).and_then(|&id| {
        ENVELOPE_TAGS.get(&id).map(|info| (id, info))
    })
}

// ============================================================================
// Record 2: Application Record (most commonly used)
// ============================================================================

static APPLICATION_TAGS: phf::Map<u8, TagInfo> = phf_map! {
    0u8 => TagInfo::new("ApplicationRecordVersion", TagFormat::Int16u),
    3u8 => TagInfo::with_len("ObjectTypeReference", TagFormat::String, 67),
    4u8 => TagInfo::list("ObjectAttributeReference", TagFormat::String),
    5u8 => TagInfo::with_len("ObjectName", TagFormat::String, 64),
    7u8 => TagInfo::with_len("EditStatus", TagFormat::String, 64),
    8u8 => TagInfo::with_len("EditorialUpdate", TagFormat::Digits, 2),
    10u8 => TagInfo::with_len("Urgency", TagFormat::Digits, 1),
    12u8 => TagInfo::list("SubjectReference", TagFormat::String),
    15u8 => TagInfo::with_len("Category", TagFormat::String, 3),
    20u8 => TagInfo::list("SupplementalCategories", TagFormat::String),
    22u8 => TagInfo::with_len("FixtureIdentifier", TagFormat::String, 32),
    25u8 => TagInfo::list("Keywords", TagFormat::String),
    26u8 => TagInfo::list("ContentLocationCode", TagFormat::String),
    27u8 => TagInfo::list("ContentLocationName", TagFormat::String),
    30u8 => TagInfo::with_len("ReleaseDate", TagFormat::Digits, 8),
    35u8 => TagInfo::with_len("ReleaseTime", TagFormat::String, 11),
    37u8 => TagInfo::with_len("ExpirationDate", TagFormat::Digits, 8),
    38u8 => TagInfo::with_len("ExpirationTime", TagFormat::String, 11),
    40u8 => TagInfo::with_len("SpecialInstructions", TagFormat::String, 256),
    42u8 => TagInfo::with_len("ActionAdvised", TagFormat::Digits, 2),
    45u8 => TagInfo::list("ReferenceService", TagFormat::String),
    47u8 => TagInfo::list("ReferenceDate", TagFormat::Digits),
    50u8 => TagInfo::list("ReferenceNumber", TagFormat::Digits),
    55u8 => TagInfo::with_len("DateCreated", TagFormat::Digits, 8),
    60u8 => TagInfo::with_len("TimeCreated", TagFormat::String, 11),
    62u8 => TagInfo::with_len("DigitalCreationDate", TagFormat::Digits, 8),
    63u8 => TagInfo::with_len("DigitalCreationTime", TagFormat::String, 11),
    65u8 => TagInfo::with_len("OriginatingProgram", TagFormat::String, 32),
    70u8 => TagInfo::with_len("ProgramVersion", TagFormat::String, 10),
    75u8 => TagInfo::with_len("ObjectCycle", TagFormat::String, 1),
    80u8 => TagInfo::list("Byline", TagFormat::String),
    85u8 => TagInfo::list("BylineTitle", TagFormat::String),
    90u8 => TagInfo::with_len("City", TagFormat::String, 32),
    92u8 => TagInfo::with_len("Sublocation", TagFormat::String, 32),
    95u8 => TagInfo::with_len("Province-State", TagFormat::String, 32),
    100u8 => TagInfo::with_len("Country-PrimaryLocationCode", TagFormat::String, 3),
    101u8 => TagInfo::with_len("Country-PrimaryLocationName", TagFormat::String, 64),
    103u8 => TagInfo::with_len("OriginalTransmissionReference", TagFormat::String, 32),
    105u8 => TagInfo::with_len("Headline", TagFormat::String, 256),
    110u8 => TagInfo::with_len("Credit", TagFormat::String, 32),
    115u8 => TagInfo::with_len("Source", TagFormat::String, 32),
    116u8 => TagInfo::with_len("CopyrightNotice", TagFormat::String, 128),
    118u8 => TagInfo::list("Contact", TagFormat::String),
    120u8 => TagInfo::with_len("Caption-Abstract", TagFormat::Text, 2000),
    121u8 => TagInfo::with_len("LocalCaption", TagFormat::Text, 256),
    122u8 => TagInfo::list("Writer-Editor", TagFormat::String),
    125u8 => TagInfo::new("RasterizedCaption", TagFormat::Binary),
    130u8 => TagInfo::with_len("ImageType", TagFormat::String, 2),
    131u8 => TagInfo::with_len("ImageOrientation", TagFormat::String, 1),
    135u8 => TagInfo::with_len("LanguageIdentifier", TagFormat::String, 3),
    150u8 => TagInfo::with_len("AudioType", TagFormat::String, 2),
    151u8 => TagInfo::with_len("AudioSamplingRate", TagFormat::Digits, 6),
    152u8 => TagInfo::with_len("AudioSamplingResolution", TagFormat::Digits, 2),
    153u8 => TagInfo::with_len("AudioDuration", TagFormat::Digits, 6),
    154u8 => TagInfo::with_len("AudioOutcue", TagFormat::String, 64),
    184u8 => TagInfo::with_len("JobID", TagFormat::String, 64),
    185u8 => TagInfo::with_len("MasterDocumentID", TagFormat::String, 256),
    186u8 => TagInfo::with_len("ShortDocumentID", TagFormat::String, 64),
    187u8 => TagInfo::with_len("UniqueDocumentID", TagFormat::String, 128),
    188u8 => TagInfo::with_len("OwnerID", TagFormat::String, 128),
    200u8 => TagInfo::new("ObjectPreviewFileFormat", TagFormat::Int16u),
    201u8 => TagInfo::new("ObjectPreviewFileVersion", TagFormat::Int16u),
    202u8 => TagInfo::new("ObjectPreviewData", TagFormat::Binary),
    221u8 => TagInfo::new("Prefs", TagFormat::String),
    225u8 => TagInfo::new("ClassifyState", TagFormat::String),
    228u8 => TagInfo::new("SimilarityIndex", TagFormat::String),
    230u8 => TagInfo::new("DocumentNotes", TagFormat::Text),
    231u8 => TagInfo::new("DocumentHistory", TagFormat::Text),
    232u8 => TagInfo::new("ExifCameraInfo", TagFormat::String),
    255u8 => TagInfo::new("CatalogSets", TagFormat::String),
};

static APPLICATION_TAGS_BY_NAME: phf::Map<&'static str, u8> = phf_map! {
    "ApplicationRecordVersion" => 0u8,
    "ObjectTypeReference" => 3u8,
    "ObjectAttributeReference" => 4u8,
    "ObjectName" => 5u8,
    "Title" => 5u8,  // Alias
    "EditStatus" => 7u8,
    "EditorialUpdate" => 8u8,
    "Urgency" => 10u8,
    "SubjectReference" => 12u8,
    "Category" => 15u8,
    "SupplementalCategories" => 20u8,
    "FixtureIdentifier" => 22u8,
    "Keywords" => 25u8,
    "ContentLocationCode" => 26u8,
    "ContentLocationName" => 27u8,
    "ReleaseDate" => 30u8,
    "ReleaseTime" => 35u8,
    "ExpirationDate" => 37u8,
    "ExpirationTime" => 38u8,
    "SpecialInstructions" => 40u8,
    "Instructions" => 40u8,  // Alias
    "ActionAdvised" => 42u8,
    "ReferenceService" => 45u8,
    "ReferenceDate" => 47u8,
    "ReferenceNumber" => 50u8,
    "DateCreated" => 55u8,
    "TimeCreated" => 60u8,
    "DigitalCreationDate" => 62u8,
    "DigitalCreationTime" => 63u8,
    "OriginatingProgram" => 65u8,
    "ProgramVersion" => 70u8,
    "ObjectCycle" => 75u8,
    "Byline" => 80u8,
    "Author" => 80u8,  // Alias
    "Creator" => 80u8,  // Alias
    "BylineTitle" => 85u8,
    "AuthorTitle" => 85u8,  // Alias
    "City" => 90u8,
    "Sublocation" => 92u8,
    "Location" => 92u8,  // Alias
    "Province-State" => 95u8,
    "State" => 95u8,  // Alias
    "Country-PrimaryLocationCode" => 100u8,
    "CountryCode" => 100u8,  // Alias
    "Country-PrimaryLocationName" => 101u8,
    "Country" => 101u8,  // Alias
    "OriginalTransmissionReference" => 103u8,
    "TransmissionReference" => 103u8,  // Alias
    "Headline" => 105u8,
    "Credit" => 110u8,
    "Source" => 115u8,
    "CopyrightNotice" => 116u8,
    "Copyright" => 116u8,  // Alias
    "Contact" => 118u8,
    "Caption-Abstract" => 120u8,
    "Caption" => 120u8,  // Alias
    "Description" => 120u8,  // Alias
    "LocalCaption" => 121u8,
    "Writer-Editor" => 122u8,
    "CaptionWriter" => 122u8,  // Alias
    "RasterizedCaption" => 125u8,
    "ImageType" => 130u8,
    "ImageOrientation" => 131u8,
    "LanguageIdentifier" => 135u8,
    "AudioType" => 150u8,
    "AudioSamplingRate" => 151u8,
    "AudioSamplingResolution" => 152u8,
    "AudioDuration" => 153u8,
    "AudioOutcue" => 154u8,
    "JobID" => 184u8,
    "MasterDocumentID" => 185u8,
    "ShortDocumentID" => 186u8,
    "UniqueDocumentID" => 187u8,
    "OwnerID" => 188u8,
    "ObjectPreviewFileFormat" => 200u8,
    "ObjectPreviewFileVersion" => 201u8,
    "ObjectPreviewData" => 202u8,
    "Prefs" => 221u8,
    "ClassifyState" => 225u8,
    "SimilarityIndex" => 228u8,
    "DocumentNotes" => 230u8,
    "DocumentHistory" => 231u8,
    "ExifCameraInfo" => 232u8,
    "CatalogSets" => 255u8,
};

/// Get application tag info by ID.
pub fn application_tag(id: u8) -> Option<&'static TagInfo> {
    APPLICATION_TAGS.get(&id)
}

/// Get application tag by name (supports aliases).
pub fn application_tag_by_name(name: &str) -> Option<(u8, &'static TagInfo)> {
    APPLICATION_TAGS_BY_NAME.get(name).and_then(|&id| {
        APPLICATION_TAGS.get(&id).map(|info| (id, info))
    })
}

// ============================================================================
// Record 3: NewsPhoto Record
// ============================================================================

static NEWSPHOTO_TAGS: phf::Map<u8, TagInfo> = phf_map! {
    0u8 => TagInfo::new("NewsPhotoVersion", TagFormat::Int16u),
    10u8 => TagInfo::new("IPTCPictureNumber", TagFormat::String),
    20u8 => TagInfo::new("IPTCImageWidth", TagFormat::Int16u),
    30u8 => TagInfo::new("IPTCImageHeight", TagFormat::Int16u),
    40u8 => TagInfo::new("IPTCPixelWidth", TagFormat::Int16u),
    50u8 => TagInfo::new("IPTCPixelHeight", TagFormat::Int16u),
    55u8 => TagInfo::new("SupplementalType", TagFormat::Int16u),
    60u8 => TagInfo::new("ColorRepresentation", TagFormat::Int16u),
    64u8 => TagInfo::new("InterchangeColorSpace", TagFormat::Int16u),
    65u8 => TagInfo::new("ColorSequence", TagFormat::Int16u),
    66u8 => TagInfo::new("ICC_Profile", TagFormat::Binary),
    70u8 => TagInfo::new("ColorCalibrationMatrix", TagFormat::Binary),
    80u8 => TagInfo::new("LookupTable", TagFormat::Binary),
    84u8 => TagInfo::new("NumIndexEntries", TagFormat::Int16u),
    85u8 => TagInfo::new("ColorPalette", TagFormat::Binary),
    86u8 => TagInfo::new("IPTCBitsPerSample", TagFormat::Int16u),
    90u8 => TagInfo::new("SampleStructure", TagFormat::Int16u),
    100u8 => TagInfo::new("ScanningDirection", TagFormat::Int16u),
    102u8 => TagInfo::new("IPTCImageRotation", TagFormat::Int16u),
    110u8 => TagInfo::new("DataCompressionMethod", TagFormat::Int16u),
    120u8 => TagInfo::new("QuantizationMethod", TagFormat::Int16u),
    125u8 => TagInfo::new("EndPoints", TagFormat::Binary),
    130u8 => TagInfo::new("ExcursionTolerance", TagFormat::Int16u),
    135u8 => TagInfo::new("BitsPerComponent", TagFormat::Int16u),
    140u8 => TagInfo::new("MaximumDensityRange", TagFormat::Int16u),
    145u8 => TagInfo::new("GammaCompensatedValue", TagFormat::Int16u),
};

static NEWSPHOTO_TAGS_BY_NAME: phf::Map<&'static str, u8> = phf_map! {
    "NewsPhotoVersion" => 0u8,
    "IPTCPictureNumber" => 10u8,
    "IPTCImageWidth" => 20u8,
    "IPTCImageHeight" => 30u8,
    "IPTCPixelWidth" => 40u8,
    "IPTCPixelHeight" => 50u8,
    "SupplementalType" => 55u8,
    "ColorRepresentation" => 60u8,
    "InterchangeColorSpace" => 64u8,
    "ColorSequence" => 65u8,
    "ICC_Profile" => 66u8,
    "ColorCalibrationMatrix" => 70u8,
    "LookupTable" => 80u8,
    "NumIndexEntries" => 84u8,
    "ColorPalette" => 85u8,
    "IPTCBitsPerSample" => 86u8,
    "SampleStructure" => 90u8,
    "ScanningDirection" => 100u8,
    "IPTCImageRotation" => 102u8,
    "DataCompressionMethod" => 110u8,
    "QuantizationMethod" => 120u8,
    "EndPoints" => 125u8,
    "ExcursionTolerance" => 130u8,
    "BitsPerComponent" => 135u8,
    "MaximumDensityRange" => 140u8,
    "GammaCompensatedValue" => 145u8,
};

/// Get newsphoto tag info by ID.
pub fn newsphoto_tag(id: u8) -> Option<&'static TagInfo> {
    NEWSPHOTO_TAGS.get(&id)
}

/// Get newsphoto tag by name.
pub fn newsphoto_tag_by_name(name: &str) -> Option<(u8, &'static TagInfo)> {
    NEWSPHOTO_TAGS_BY_NAME.get(name).and_then(|&id| {
        NEWSPHOTO_TAGS.get(&id).map(|info| (id, info))
    })
}

// ============================================================================
// Utility functions
// ============================================================================

/// Get all application tag names (for CLI completion, etc.).
pub fn all_application_tags() -> impl Iterator<Item = &'static str> {
    APPLICATION_TAGS.values().map(|t| t.name)
}

/// Check if a tag supports list values.
pub fn is_list_tag(record: u8, tag: u8) -> bool {
    match record {
        1 => ENVELOPE_TAGS.get(&tag).map_or(false, |t| t.is_list),
        2 => APPLICATION_TAGS.get(&tag).map_or(false, |t| t.is_list),
        3 => NEWSPHOTO_TAGS.get(&tag).map_or(false, |t| t.is_list),
        _ => false,
    }
}

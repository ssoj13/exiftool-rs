//! IPTC-IIM (International Press Telecommunications Council - Information Interchange Model) parser.
//!
//! IPTC is found in JPEG APP13 segments with "Photoshop 3.0" header,
//! wrapped in Photoshop IRB (Image Resource Block) format.
//!
//! # Structure
//!
//! APP13 segment:
//! - "Photoshop 3.0\0" header (14 bytes)
//! - IRB resources (multiple)
//!
//! IRB resource:
//! - "8BIM" signature (4 bytes)
//! - Resource ID (2 bytes BE) - 0x0404 for IPTC
//! - Pascal string name (1-byte length + string, padded to even)
//! - Size (4 bytes BE)
//! - Data (padded to even)
//!
//! IPTC dataset:
//! - 0x1C marker (1 byte)
//! - Record number (1 byte)
//! - Dataset number (1 byte)
//! - Size (2 bytes BE, or 4 bytes if high bit set)
//! - Data

use exiftool_attrs::{AttrValue, Attrs};

/// Photoshop 3.0 APP13 header
const PHOTOSHOP_HEADER: &[u8] = b"Photoshop 3.0\0";

/// 8BIM signature for IRB
const IRB_SIGNATURE: &[u8] = b"8BIM";

/// IPTC resource ID in IRB
const IPTC_RESOURCE_ID: u16 = 0x0404;

/// IPTC tag marker
const IPTC_MARKER: u8 = 0x1C;

/// IPTC parser for APP13 segments.
pub struct IptcParser;

impl IptcParser {
    /// Parse raw IPTC data (datasets without Photoshop/IRB wrapper).
    ///
    /// Use this when you already extracted the IPTC data from APP13.
    pub fn parse(data: &[u8]) -> Result<Attrs, ()> {
        Self::parse_iptc_data(data).ok_or(())
    }

    /// Parse IPTC from APP13 segment data (with Photoshop header).
    pub fn parse_app13(data: &[u8]) -> Option<Attrs> {
        // Check Photoshop header
        if !data.starts_with(PHOTOSHOP_HEADER) {
            return None;
        }

        let mut pos = PHOTOSHOP_HEADER.len();

        // Parse IRB resources
        while pos + 12 <= data.len() {
            // Check 8BIM signature
            if &data[pos..pos + 4] != IRB_SIGNATURE {
                break;
            }
            pos += 4;

            // Resource ID
            let resource_id = u16::from_be_bytes([data[pos], data[pos + 1]]);
            pos += 2;

            // Pascal string (name) - 1 byte length + string, padded to even
            let name_len = data[pos] as usize;
            pos += 1;
            pos += name_len;
            // Pad to even
            if !(1 + name_len).is_multiple_of(2) {
                pos += 1;
            }

            if pos + 4 > data.len() {
                break;
            }

            // Resource size
            let size = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
            pos += 4;

            if pos + size > data.len() {
                break;
            }

            // Check if this is IPTC resource
            if resource_id == IPTC_RESOURCE_ID {
                let iptc_data = &data[pos..pos + size];
                return Self::parse_iptc_data(iptc_data);
            }

            // Skip resource data (padded to even)
            pos += size;
            if !size.is_multiple_of(2) {
                pos += 1;
            }
        }

        None
    }

    /// Parse raw IPTC data (datasets).
    fn parse_iptc_data(data: &[u8]) -> Option<Attrs> {
        let mut attrs = Attrs::new();
        let mut pos = 0;

        while pos + 5 <= data.len() {
            // Tag marker
            if data[pos] != IPTC_MARKER {
                break;
            }
            pos += 1;

            // Record and dataset numbers
            let record = data[pos];
            let dataset = data[pos + 1];
            pos += 2;

            // Size (2 bytes BE, extended if high bit set)
            let size_high = data[pos];
            let size_low = data[pos + 1];
            pos += 2;

            let size = if size_high & 0x80 != 0 {
                // Extended size (not common)
                let ext_size = ((size_high & 0x7F) as usize) << 8 | size_low as usize;
                if pos + ext_size > data.len() {
                    break;
                }
                // Read actual size from extended bytes
                let mut actual_size = 0usize;
                for i in 0..ext_size {
                    actual_size = actual_size << 8 | data[pos + i] as usize;
                }
                pos += ext_size;
                actual_size
            } else {
                ((size_high as usize) << 8) | size_low as usize
            };

            if pos + size > data.len() {
                break;
            }

            let value_data = &data[pos..pos + size];
            pos += size;

            // Parse based on record type
            if record == 2 {
                // Application Record (most common)
                Self::parse_application_record(&mut attrs, dataset, value_data);
            } else if record == 1 {
                // Envelope Record
                Self::parse_envelope_record(&mut attrs, dataset, value_data);
            }
        }

        if attrs.is_empty() {
            None
        } else {
            Some(attrs)
        }
    }

    /// Parse Application Record (record 2) dataset.
    fn parse_application_record(attrs: &mut Attrs, dataset: u8, data: &[u8]) {
        let (name, is_list) = match dataset {
            0 => ("RecordVersion", false),
            3 => ("ObjectTypeReference", false),
            4 => ("ObjectAttributeReference", true),
            5 => ("ObjectName", false),  // Title
            7 => ("EditStatus", false),
            8 => ("EditorialUpdate", false),
            10 => ("Urgency", false),
            12 => ("SubjectReference", true),
            15 => ("Category", false),
            20 => ("SupplementalCategories", true),
            22 => ("FixtureIdentifier", false),
            25 => ("Keywords", true),
            26 => ("ContentLocationCode", true),
            27 => ("ContentLocationName", true),
            30 => ("ReleaseDate", false),
            35 => ("ReleaseTime", false),
            37 => ("ExpirationDate", false),
            38 => ("ExpirationTime", false),
            40 => ("SpecialInstructions", false),
            42 => ("ActionAdvised", false),
            45 => ("ReferenceService", true),
            47 => ("ReferenceDate", true),
            50 => ("ReferenceNumber", true),
            55 => ("DateCreated", false),
            60 => ("TimeCreated", false),
            62 => ("DigitalCreationDate", false),
            63 => ("DigitalCreationTime", false),
            65 => ("OriginatingProgram", false),
            70 => ("ProgramVersion", false),
            75 => ("ObjectCycle", false),
            80 => ("By-line", true),  // Creator
            85 => ("By-lineTitle", true),
            90 => ("City", false),
            92 => ("Sub-location", false),
            95 => ("Province-State", false),
            100 => ("Country-PrimaryLocationCode", false),
            101 => ("Country-PrimaryLocationName", false),
            103 => ("OriginalTransmissionReference", false),
            105 => ("Headline", false),
            110 => ("Credit", false),
            115 => ("Source", false),
            116 => ("CopyrightNotice", false),
            118 => ("Contact", true),
            120 => ("Caption-Abstract", false),  // Description
            121 => ("LocalCaption", false),
            122 => ("Writer-Editor", true),
            125 => ("RasterizedCaption", false),
            130 => ("ImageType", false),
            131 => ("ImageOrientation", false),
            135 => ("LanguageIdentifier", false),
            150 => ("AudioType", false),
            151 => ("AudioSamplingRate", false),
            152 => ("AudioSamplingResolution", false),
            153 => ("AudioDuration", false),
            154 => ("AudioOutcue", false),
            184 => ("JobID", false),
            185 => ("MasterDocumentID", false),
            186 => ("ShortDocumentID", false),
            187 => ("UniqueDocumentID", false),
            188 => ("OwnerID", false),
            200 => ("ObjectPreviewFileFormat", false),
            201 => ("ObjectPreviewFileVersion", false),
            202 => ("ObjectPreviewData", false),
            221 => ("Prefs", false),
            225 => ("ClassifyState", false),
            228 => ("SimilarityIndex", false),
            230 => ("DocumentNotes", false),
            231 => ("DocumentHistory", false),
            232 => ("ExifCameraInfo", false),
            255 => ("CatalogSets", true),
            _ => return,
        };

        let key = format!("IPTC:{}", name);

        // Convert data to string
        let value = String::from_utf8_lossy(data).trim().to_string();
        if value.is_empty() {
            return;
        }

        if is_list {
            // Append to list
            if let Some(AttrValue::List(list)) = attrs.get_mut(&key) {
                list.push(AttrValue::Str(value));
            } else {
                attrs.set(&key, AttrValue::List(vec![AttrValue::Str(value)]));
            }
        } else {
            attrs.set(&key, AttrValue::Str(value));
        }
    }

    /// Parse Envelope Record (record 1) dataset.
    fn parse_envelope_record(attrs: &mut Attrs, dataset: u8, data: &[u8]) {
        let name = match dataset {
            0 => "EnvelopeRecordVersion",
            5 => "Destination",
            20 => "FileFormat",
            22 => "FileVersion",
            30 => "ServiceIdentifier",
            40 => "EnvelopeNumber",
            50 => "ProductID",
            60 => "EnvelopePriority",
            70 => "DateSent",
            80 => "TimeSent",
            90 => "CodedCharacterSet",
            100 => "UniqueObjectName",
            120 => "ARMIdentifier",
            122 => "ARMVersion",
            _ => return,
        };

        let key = format!("IPTC:{}", name);
        let value = String::from_utf8_lossy(data).trim().to_string();
        if !value.is_empty() {
            attrs.set(&key, AttrValue::Str(value));
        }
    }

    /// Extract IPTC from JPEG APP13 segment data.
    ///
    /// Returns the raw IPTC data if found, for modification/rewriting.
    pub fn extract_iptc_data(data: &[u8]) -> Option<Vec<u8>> {
        if !data.starts_with(PHOTOSHOP_HEADER) {
            return None;
        }

        let mut pos = PHOTOSHOP_HEADER.len();

        while pos + 12 <= data.len() {
            if &data[pos..pos + 4] != IRB_SIGNATURE {
                break;
            }
            pos += 4;

            let resource_id = u16::from_be_bytes([data[pos], data[pos + 1]]);
            pos += 2;

            let name_len = data[pos] as usize;
            pos += 1;
            pos += name_len;
            if !(1 + name_len).is_multiple_of(2) {
                pos += 1;
            }

            if pos + 4 > data.len() {
                break;
            }

            let size = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
            pos += 4;

            if pos + size > data.len() {
                break;
            }

            if resource_id == IPTC_RESOURCE_ID {
                return Some(data[pos..pos + size].to_vec());
            }

            pos += size;
            if !size.is_multiple_of(2) {
                pos += 1;
            }
        }

        None
    }
}

/// IPTC writer - builds IPTC data from attributes.
pub struct IptcWriter;

impl IptcWriter {
    /// Build IPTC data from attributes.
    pub fn build(attrs: &Attrs) -> Vec<u8> {
        let mut data = Vec::new();

        // Write Application Record (2) datasets
        for (key, value) in attrs.iter() {
            if !key.starts_with("IPTC:") {
                continue;
            }

            let name = &key[5..];
            let dataset = Self::name_to_dataset(name);
            if dataset == 0 {
                continue;
            }

            match value {
                AttrValue::Str(s) => {
                    Self::write_dataset(&mut data, 2, dataset, s.as_bytes());
                }
                AttrValue::List(list) => {
                    for item in list {
                        if let AttrValue::Str(s) = item {
                            Self::write_dataset(&mut data, 2, dataset, s.as_bytes());
                        }
                    }
                }
                _ => {}
            }
        }

        data
    }

    /// Build APP13 segment data with IPTC.
    pub fn build_app13(attrs: &Attrs) -> Vec<u8> {
        let iptc_data = Self::build(attrs);
        if iptc_data.is_empty() {
            return Vec::new();
        }

        let mut data = Vec::new();

        // Photoshop header
        data.extend_from_slice(PHOTOSHOP_HEADER);

        // 8BIM resource for IPTC
        data.extend_from_slice(IRB_SIGNATURE);

        // Resource ID
        data.extend_from_slice(&IPTC_RESOURCE_ID.to_be_bytes());

        // Pascal string name (empty)
        data.push(0); // Length 0
        data.push(0); // Pad to even

        // Size
        data.extend_from_slice(&(iptc_data.len() as u32).to_be_bytes());

        // IPTC data
        data.extend_from_slice(&iptc_data);

        // Pad to even
        if !iptc_data.len().is_multiple_of(2) {
            data.push(0);
        }

        data
    }

    /// Write a single IPTC dataset.
    fn write_dataset(data: &mut Vec<u8>, record: u8, dataset: u8, value: &[u8]) {
        data.push(IPTC_MARKER);
        data.push(record);
        data.push(dataset);

        let len = value.len();
        if len < 32768 {
            data.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
            // Extended size (rare)
            data.push(0x80 | 4); // 4 bytes follow
            data.push(0);
            data.extend_from_slice(&(len as u32).to_be_bytes());
        }

        data.extend_from_slice(value);
    }

    /// Convert tag name to dataset number.
    fn name_to_dataset(name: &str) -> u8 {
        match name {
            "RecordVersion" => 0,
            "ObjectTypeReference" => 3,
            "ObjectAttributeReference" => 4,
            "ObjectName" | "Title" => 5,
            "EditStatus" => 7,
            "EditorialUpdate" => 8,
            "Urgency" => 10,
            "SubjectReference" => 12,
            "Category" => 15,
            "SupplementalCategories" => 20,
            "FixtureIdentifier" => 22,
            "Keywords" => 25,
            "ContentLocationCode" => 26,
            "ContentLocationName" => 27,
            "ReleaseDate" => 30,
            "ReleaseTime" => 35,
            "ExpirationDate" => 37,
            "ExpirationTime" => 38,
            "SpecialInstructions" => 40,
            "ActionAdvised" => 42,
            "ReferenceService" => 45,
            "ReferenceDate" => 47,
            "ReferenceNumber" => 50,
            "DateCreated" => 55,
            "TimeCreated" => 60,
            "DigitalCreationDate" => 62,
            "DigitalCreationTime" => 63,
            "OriginatingProgram" => 65,
            "ProgramVersion" => 70,
            "ObjectCycle" => 75,
            "By-line" | "Creator" => 80,
            "By-lineTitle" => 85,
            "City" => 90,
            "Sub-location" => 92,
            "Province-State" => 95,
            "Country-PrimaryLocationCode" => 100,
            "Country-PrimaryLocationName" => 101,
            "OriginalTransmissionReference" => 103,
            "Headline" => 105,
            "Credit" => 110,
            "Source" => 115,
            "CopyrightNotice" | "Copyright" => 116,
            "Contact" => 118,
            "Caption-Abstract" | "Description" => 120,
            "LocalCaption" => 121,
            "Writer-Editor" => 122,
            "ImageType" => 130,
            "ImageOrientation" => 131,
            "LanguageIdentifier" => 135,
            "JobID" => 184,
            "MasterDocumentID" => 185,
            "ShortDocumentID" => 186,
            "UniqueDocumentID" => 187,
            "OwnerID" => 188,
            "ObjectPreviewFileFormat" => 200,
            "ObjectPreviewFileVersion" => 201,
            "ObjectPreviewData" => 202,
            "CatalogSets" => 255,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_iptc_dataset() {
        // Build test IPTC data: Title + Keywords
        let mut iptc_data = Vec::new();

        // ObjectName (Title) - dataset 5
        let title = b"Test Title";
        iptc_data.push(IPTC_MARKER);
        iptc_data.push(2); // record
        iptc_data.push(5); // dataset
        iptc_data.extend_from_slice(&(title.len() as u16).to_be_bytes());
        iptc_data.extend_from_slice(title);

        // Keyword 1 - dataset 25
        let keyword1 = b"keyword1";
        iptc_data.push(IPTC_MARKER);
        iptc_data.push(2);
        iptc_data.push(25);
        iptc_data.extend_from_slice(&(keyword1.len() as u16).to_be_bytes());
        iptc_data.extend_from_slice(keyword1);

        // Keyword 2
        let keyword2 = b"keyword2";
        iptc_data.push(IPTC_MARKER);
        iptc_data.push(2);
        iptc_data.push(25);
        iptc_data.extend_from_slice(&(keyword2.len() as u16).to_be_bytes());
        iptc_data.extend_from_slice(keyword2);

        let attrs = IptcParser::parse_iptc_data(&iptc_data).unwrap();

        assert_eq!(attrs.get_str("IPTC:ObjectName"), Some("Test Title"));

        if let Some(AttrValue::List(keywords)) = attrs.get("IPTC:Keywords") {
            assert_eq!(keywords.len(), 2);
            assert_eq!(keywords[0], AttrValue::Str("keyword1".into()));
            assert_eq!(keywords[1], AttrValue::Str("keyword2".into()));
        } else {
            panic!("Keywords should be a list");
        }
    }

    #[test]
    fn build_iptc_data() {
        let mut attrs = Attrs::new();
        attrs.set("IPTC:ObjectName", AttrValue::Str("My Title".into()));
        attrs.set("IPTC:Keywords", AttrValue::List(vec!["tag1".into(), "tag2".into()]));

        let data = IptcWriter::build(&attrs);

        // Parse it back
        let parsed = IptcParser::parse_iptc_data(&data).unwrap();

        assert_eq!(parsed.get_str("IPTC:ObjectName"), Some("My Title"));

        if let Some(AttrValue::List(keywords)) = parsed.get("IPTC:Keywords") {
            assert_eq!(keywords.len(), 2);
        } else {
            panic!("Keywords should be a list");
        }
    }

    #[test]
    fn build_app13_segment() {
        let mut attrs = Attrs::new();
        attrs.set("IPTC:Headline", AttrValue::Str("Test Headline".into()));

        let data = IptcWriter::build_app13(&attrs);

        // Should start with Photoshop header
        assert!(data.starts_with(PHOTOSHOP_HEADER));

        // Should have 8BIM signature
        let pos = PHOTOSHOP_HEADER.len();
        assert_eq!(&data[pos..pos + 4], IRB_SIGNATURE);

        // Parse the full APP13 segment
        let parsed = IptcParser::parse_app13(&data).unwrap();
        assert_eq!(parsed.get_str("IPTC:Headline"), Some("Test Headline"));
    }
}

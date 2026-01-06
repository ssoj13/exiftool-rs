//! Ricoh MakerNotes parser.
//!
//! Note: Ricoh cameras acquired from Pentax use Pentax MakerNotes format.
//! This parser handles native Ricoh MakerNotes format.
//!
//! Ricoh MakerNotes structure:
//! - Header: "Rv" or "RICOH" (varies by model)
//! - Standard TIFF IFD after header
//!
//! Known tags:
//! - 0x0001: MakerNoteType
//! - 0x0002: FirmwareVersion
//! - 0x0005: SerialNumber
//! - 0x000e: ImageInfo
//! - 0x1001: ManometerPressure (theta models)
//! - 0x1002: ManometerReading
//! - 0x1003: AccelerometerX
//! - 0x1004: AccelerometerY
//! - 0x1005: AccelerometerZ
//! - 0x1006: CompassHeading
//! - 0x1007: ManualWhiteBalance
//! - 0x1009: DigitalZoom
//! - 0x1100: FaceInfo
//! - 0x2001: RicohSubdir (contains additional tags)

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::Attrs;
use exiftool_core::{ByteOrder, IfdReader};

/// Ricoh MakerNotes parser.
pub struct RicohParser;

/// Known Ricoh MakerNote tags.
static RICOH_TAGS: &[(u16, &str)] = &[
    (0x0001, "MakerNoteType"),
    (0x0002, "FirmwareVersion"),
    (0x0005, "SerialNumber"),
    (0x000e, "ImageInfo"),
    (0x1001, "ManometerPressure"),
    (0x1002, "ManometerReading"),
    (0x1003, "AccelerometerX"),
    (0x1004, "AccelerometerY"),
    (0x1005, "AccelerometerZ"),
    (0x1006, "CompassHeading"),
    (0x1007, "ManualWhiteBalance"),
    (0x1009, "DigitalZoom"),
    (0x1100, "FaceInfo"),
    (0x2001, "RicohSubdir"),
];

impl VendorParser for RicohParser {
    fn vendor(&self) -> Vendor {
        Vendor::Ricoh
    }

    fn parse(&self, data: &[u8], parent_byte_order: ByteOrder) -> Option<Attrs> {
        if data.len() < 8 {
            return None;
        }

        // Check for Ricoh headers
        let (ifd_data, byte_order) = if data.starts_with(b"Rv") {
            // "Rv" header (2 bytes)
            (&data[2..], ByteOrder::BigEndian)
        } else if data.starts_with(b"RICOH\0") {
            // "RICOH\0" header (6 bytes)
            (&data[6..], ByteOrder::BigEndian)
        } else if data.starts_with(b"RICOH") {
            // "RICOH" header (5 bytes) + possible padding
            (&data[8..], ByteOrder::BigEndian)
        } else {
            // No header, try as direct IFD
            (data, parent_byte_order)
        };

        if ifd_data.len() < 6 {
            return None;
        }

        let reader = IfdReader::new(ifd_data, byte_order, 0);
        let (entries, _) = reader.read_ifd(0).ok()?;

        let mut attrs = Attrs::new();

        for entry in entries {
            let tag_name = RICOH_TAGS
                .iter()
                .find(|(t, _)| *t == entry.tag)
                .map(|(_, n)| *n);

            if let Some(name) = tag_name {
                attrs.set(name, entry_to_attr(&entry));
            } else {
                let name = format!("Unknown_0x{:04X}", entry.tag);
                attrs.set(&name, entry_to_attr(&entry));
            }
        }

        Some(attrs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_vendor() {
        assert_eq!(RicohParser.vendor(), Vendor::Ricoh);
    }
}

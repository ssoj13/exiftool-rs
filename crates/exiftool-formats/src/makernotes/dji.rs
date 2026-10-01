//! DJI MakerNotes parser.
//!
//! DJI MakerNotes structure (Phantom drones):
//! - Standard IFD format
//! - Uses parent byte order
//!
//! Known tags:
//! - 0x0001: Make (string)
//! - 0x0003: SpeedX (float) - drone speed X axis
//! - 0x0004: SpeedY (float) - drone speed Y axis
//! - 0x0005: SpeedZ (float) - drone speed Z axis
//! - 0x0006: Pitch (float) - drone pitch angle
//! - 0x0007: Yaw (float) - drone yaw angle
//! - 0x0008: Roll (float) - drone roll angle
//! - 0x0009: CameraPitch (float) - gimbal pitch
//! - 0x000a: CameraYaw (float) - gimbal yaw
//! - 0x000b: CameraRoll (float) - gimbal roll

use super::{Vendor, VendorParser};
use crate::utils::entry_to_attr;
use exiftool_attrs::{AttrValue, Attrs};
use exiftool_core::ByteOrder;
use exiftool_tags::generated::dji;

/// DJI MakerNotes parser.
pub struct DjiParser;

impl VendorParser for DjiParser {
    fn vendor(&self) -> Vendor {
        Vendor::Dji
    }

    fn parse(&self, data: &[u8], byte_order: ByteOrder) -> Option<Attrs> {
        let entries = super::parse_ifd_entries(data, byte_order, 0)?;

        let mut attrs = Attrs::new();

        for entry in entries {
            if let Some(tag_def) = dji::DJI_MAIN.get(&entry.tag) {
                let value = format_value(&entry, tag_def.values);
                attrs.set(tag_def.name, value);
            }
        }

        Some(attrs)
    }
}

/// Format IFD entry value with PrintConv lookup.
fn format_value(
    entry: &exiftool_core::IfdEntry,
    values_map: Option<&'static [(i64, &'static str)]>,
) -> AttrValue {
    if let Some(map) = values_map {
        if let Some(int_val) = entry.value.as_u32().map(|v| v as i64) {
            for &(key, label) in map {
                if key == int_val {
                    return AttrValue::Str(label.to_string());
                }
            }
        }
    }
    entry_to_attr(entry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_vendor() {
        assert_eq!(DjiParser.vendor(), Vendor::Dji);
    }
}

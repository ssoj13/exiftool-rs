//! DJI MakerNotes tag definitions.
//! Auto-generated from ExifTool - DO NOT EDIT

#![allow(dead_code)]

/// Tag definition with name and optional value mappings.
#[derive(Debug, Clone)]
pub struct TagDef {
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}

/// DJI::Main tags (Phantom drones)
pub static DJI_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    0x0001u16 => TagDef { name: "Make", values: None },
    0x0003u16 => TagDef { name: "SpeedX", values: None },
    0x0004u16 => TagDef { name: "SpeedY", values: None },
    0x0005u16 => TagDef { name: "SpeedZ", values: None },
    0x0006u16 => TagDef { name: "Pitch", values: None },
    0x0007u16 => TagDef { name: "Yaw", values: None },
    0x0008u16 => TagDef { name: "Roll", values: None },
    0x0009u16 => TagDef { name: "CameraPitch", values: None },
    0x000au16 => TagDef { name: "CameraYaw", values: None },
    0x000bu16 => TagDef { name: "CameraRoll", values: None },
};

/// Look up a tag by ID in the main table.
pub fn lookup(tag_id: u16) -> Option<&'static TagDef> {
    DJI_MAIN.get(&tag_id)
}

//! Samsung MakerNotes tag definitions.
//! Auto-generated from ExifTool - DO NOT EDIT

#![allow(dead_code)]

/// Tag definition with name and optional value mappings.
#[derive(Debug, Clone)]
pub struct TagDef {
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}

/// Samsung::Main tags
pub static SAMSUNG_MAIN: phf::Map<u16, TagDef> = phf::phf_map! {
    0u16 => TagDef { name: "MakerNoteVersion", values: None },
    11u16 => TagDef { name: "SamsungIFD", values: None },
    2u16 => TagDef { name: "PreviewImageStart", values: None },
    3u16 => TagDef { name: "PreviewImageLength", values: None },
};


/// Look up a tag by ID in the main table.
pub fn lookup(_tag_id: u16) -> Option<&'static TagDef> {
    // Default to main table - override in specific modules
    None
}

//! Attribute schema definitions.
//!
//! Schemas define the expected attributes for an entity type,
//! including type, flags, and UI hints.

/// Attribute flags (bitfield).
pub type AttrFlags = u8;

/// Attribute affects render - changes invalidate cache.
pub const FLAG_DAG: AttrFlags = 1 << 0;
/// Attribute shown in UI.
pub const FLAG_DISPLAY: AttrFlags = 1 << 1;
/// Attribute can be animated/keyframed.
pub const FLAG_KEYABLE: AttrFlags = 1 << 2;
/// Attribute is read-only (computed).
pub const FLAG_READONLY: AttrFlags = 1 << 3;
/// Internal attribute, not shown to user.
pub const FLAG_INTERNAL: AttrFlags = 1 << 4;

/// Expected type of attribute value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrType {
    Bool,
    Int,
    UInt,
    Float,
    Double,
    String,
    Bytes,
    Rational,
    URational,
    DateTime,
    Uuid,
    List,
    Map,
    Json,
}

/// Single attribute definition.
#[derive(Debug, Clone)]
pub struct AttrDef {
    pub name: &'static str,
    pub attr_type: AttrType,
    pub flags: AttrFlags,
    /// UI hints: options or range [min, max, step].
    pub ui_options: &'static [&'static str],
    /// Display order (lower = higher in list).
    pub order: f32,
}

impl AttrDef {
    /// Create new attribute definition.
    pub const fn new(name: &'static str, attr_type: AttrType, flags: AttrFlags) -> Self {
        Self {
            name,
            attr_type,
            flags,
            ui_options: &[],
            order: 99.0,
        }
    }

    /// Create attribute with UI options.
    pub const fn with_ui(
        name: &'static str,
        attr_type: AttrType,
        flags: AttrFlags,
        ui_options: &'static [&'static str],
    ) -> Self {
        Self {
            name,
            attr_type,
            flags,
            ui_options,
            order: 99.0,
        }
    }

    /// Create attribute with display order.
    pub const fn with_order(
        name: &'static str,
        attr_type: AttrType,
        flags: AttrFlags,
        order: f32,
    ) -> Self {
        Self {
            name,
            attr_type,
            flags,
            ui_options: &[],
            order,
        }
    }

    /// Check if attribute affects DAG.
    pub const fn is_dag(&self) -> bool {
        self.flags & FLAG_DAG != 0
    }

    /// Check if attribute is shown in UI.
    pub const fn is_display(&self) -> bool {
        self.flags & FLAG_DISPLAY != 0
    }

    /// Check if attribute can be keyframed.
    pub const fn is_keyable(&self) -> bool {
        self.flags & FLAG_KEYABLE != 0
    }

    /// Check if attribute is read-only.
    pub const fn is_readonly(&self) -> bool {
        self.flags & FLAG_READONLY != 0
    }

    /// Check if attribute is internal.
    pub const fn is_internal(&self) -> bool {
        self.flags & FLAG_INTERNAL != 0
    }
}

/// Schema: collection of attribute definitions for an entity type.
#[derive(Debug, Clone)]
pub struct AttrSchema {
    pub name: &'static str,
    defs: Box<[AttrDef]>,
}

impl AttrSchema {
    /// Create schema from static slice.
    pub fn new(name: &'static str, defs: &[AttrDef]) -> Self {
        Self {
            name,
            defs: defs.to_vec().into_boxed_slice(),
        }
    }

    /// Create schema by composing multiple slices.
    pub fn from_slices(name: &'static str, slices: &[&[AttrDef]]) -> Self {
        let defs: Vec<AttrDef> = slices.iter().flat_map(|s| s.iter().cloned()).collect();
        Self {
            name,
            defs: defs.into_boxed_slice(),
        }
    }

    /// Find attribute definition by name.
    pub fn get(&self, name: &str) -> Option<&AttrDef> {
        self.defs.iter().find(|d| d.name == name)
    }

    /// Check if attribute affects DAG.
    pub fn is_dag(&self, name: &str) -> bool {
        self.get(name).is_some_and(|d| d.is_dag())
    }

    /// Check if attribute is display.
    pub fn is_display(&self, name: &str) -> bool {
        self.get(name).is_some_and(|d| d.is_display())
    }

    /// Get all display attributes.
    pub fn display_attrs(&self) -> impl Iterator<Item = &AttrDef> {
        self.defs.iter().filter(|d| d.is_display())
    }

    /// Iterate all definitions.
    pub fn iter(&self) -> impl Iterator<Item = &AttrDef> {
        self.defs.iter()
    }
}

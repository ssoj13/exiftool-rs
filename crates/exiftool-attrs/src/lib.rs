//! Attribute storage system for EXIF metadata.
//!
//! Provides typed storage for metadata values with:
//! - Schema-based validation
//! - Dirty tracking for write operations
//! - Serialization support
//!
//! # Example
//!
//! ```
//! use exiftool_attrs::{Attrs, AttrValue};
//!
//! let mut attrs = Attrs::new();
//! attrs.set("Make", AttrValue::Str("Canon".to_string()));
//! attrs.set("ISO", AttrValue::UInt(400));
//!
//! assert_eq!(attrs.get_str("Make"), Some("Canon"));
//! assert_eq!(attrs.get_u32("ISO"), Some(400));
//! ```

mod error;
mod schema;
mod value;

pub use error::{Error, Result};
pub use schema::{AttrDef, AttrFlags, AttrSchema, AttrType};
pub use schema::{FLAG_DAG, FLAG_DISPLAY, FLAG_INTERNAL, FLAG_KEYABLE, FLAG_READONLY};
pub use value::AttrValue;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, Ordering};

/// Attribute container: string key -> typed value.
///
/// Includes dirty tracking for write operations.
/// Optional schema for validation.
#[derive(Debug, Serialize, Deserialize)]
pub struct Attrs {
    #[serde(default)]
    map: HashMap<String, AttrValue>,

    /// Dirty flag: set when attributes are modified.
    #[serde(skip)]
    #[serde(default = "Attrs::default_dirty")]
    dirty: AtomicBool,

    /// Optional schema reference for validation.
    #[serde(skip)]
    schema: Option<&'static AttrSchema>,
}

impl Default for Attrs {
    fn default() -> Self {
        Self::new()
    }
}

impl Attrs {
    fn default_dirty() -> AtomicBool {
        AtomicBool::new(false)
    }

    /// Create new empty attribute container.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            dirty: AtomicBool::new(false),
            schema: None,
        }
    }

    /// Create with schema for validation.
    pub fn with_schema(schema: &'static AttrSchema) -> Self {
        Self {
            map: HashMap::new(),
            dirty: AtomicBool::new(false),
            schema: Some(schema),
        }
    }

    /// Attach schema (for deserialized entities).
    pub fn attach_schema(&mut self, schema: &'static AttrSchema) {
        self.schema = Some(schema);
    }

    /// Get current schema reference.
    pub fn schema(&self) -> Option<&'static AttrSchema> {
        self.schema
    }

    /// Set attribute value, marking dirty if DAG attribute.
    pub fn set(&mut self, key: impl Into<String>, value: AttrValue) {
        let key = key.into();

        let changed = match self.map.get(&key) {
            Some(existing) => existing != &value,
            None => true,
        };

        self.map.insert(key.clone(), value);

        if changed {
            let is_dag = match &self.schema {
                Some(schema) => schema.is_dag(&key),
                None => true,
            };

            if is_dag {
                self.dirty.store(true, Ordering::Relaxed);
            }
        }
    }

    /// Get attribute value by key.
    pub fn get(&self, key: &str) -> Option<&AttrValue> {
        self.map.get(key)
    }

    /// Get mutable reference to attribute value.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut AttrValue> {
        self.map.get_mut(key)
    }

    /// Remove attribute by key.
    pub fn remove(&mut self, key: &str) -> Option<AttrValue> {
        self.map.remove(key)
    }

    /// Check if attribute exists.
    pub fn contains(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    /// Get number of attributes.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Iterate over all attributes.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &AttrValue)> {
        self.map.iter()
    }

    /// Iterate mutably over all attributes.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut AttrValue)> {
        self.map.iter_mut()
    }

    // === Type-specific getters ===

    pub fn get_str(&self, key: &str) -> Option<&str> {
        match self.map.get(key) {
            Some(AttrValue::Str(s)) => Some(s),
            _ => None,
        }
    }

    pub fn get_i32(&self, key: &str) -> Option<i32> {
        match self.map.get(key) {
            Some(AttrValue::Int(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn get_u32(&self, key: &str) -> Option<u32> {
        match self.map.get(key) {
            Some(AttrValue::UInt(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn get_f32(&self, key: &str) -> Option<f32> {
        match self.map.get(key) {
            Some(AttrValue::Float(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        match self.map.get(key) {
            Some(AttrValue::Double(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.map.get(key) {
            Some(AttrValue::Bool(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn get_bytes(&self, key: &str) -> Option<&[u8]> {
        match self.map.get(key) {
            Some(AttrValue::Bytes(v)) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn get_rational(&self, key: &str) -> Option<(i32, i32)> {
        match self.map.get(key) {
            Some(AttrValue::Rational(n, d)) => Some((*n, *d)),
            _ => None,
        }
    }

    pub fn get_urational(&self, key: &str) -> Option<(u32, u32)> {
        match self.map.get(key) {
            Some(AttrValue::URational(n, d)) => Some((*n, *d)),
            _ => None,
        }
    }

    pub fn get_uuid(&self, key: &str) -> Option<uuid::Uuid> {
        match self.map.get(key) {
            Some(AttrValue::Uuid(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn get_list(&self, key: &str) -> Option<&Vec<AttrValue>> {
        match self.map.get(key) {
            Some(AttrValue::List(v)) => Some(v),
            _ => None,
        }
    }

    pub fn get_map(&self, key: &str) -> Option<&HashMap<String, AttrValue>> {
        match self.map.get(key) {
            Some(AttrValue::Map(v)) => Some(v),
            _ => None,
        }
    }

    // === Dirty tracking ===

    /// Check if attributes have been modified.
    pub fn is_dirty(&self) -> bool {
        self.dirty.load(Ordering::Relaxed)
    }

    /// Clear dirty flag.
    pub fn clear_dirty(&self) {
        self.dirty.store(false, Ordering::Relaxed);
    }

    /// Mark as dirty manually.
    pub fn mark_dirty(&self) {
        self.dirty.store(true, Ordering::Relaxed);
    }

    // === Hashing ===

    /// Hash all attributes in sorted key order.
    pub fn hash_all(&self) -> u64 {
        let mut keys: Vec<&String> = self.map.keys().collect();
        keys.sort_unstable();

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for key in keys {
            key.hash(&mut hasher);
            if let Some(val) = self.map.get(key) {
                val.hash(&mut hasher);
            }
        }
        hasher.finish()
    }

    // === JSON helpers ===

    /// Get JSON value and deserialize.
    pub fn get_json<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        match self.map.get(key) {
            Some(AttrValue::Json(s)) => serde_json::from_str(s).ok(),
            _ => None,
        }
    }

    /// Serialize value to JSON and store.
    pub fn set_json<T: serde::Serialize>(&mut self, key: impl Into<String>, value: &T) {
        if let Ok(json) = serde_json::to_string(value) {
            self.set(key, AttrValue::Json(json));
        }
    }
}

impl Clone for Attrs {
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            dirty: AtomicBool::new(self.dirty.load(Ordering::Relaxed)),
            schema: self.schema,
        }
    }
}

// === Nested Group Support ===

impl Attrs {
    /// Get or create a nested group by key.
    /// Creates the group if it doesn't exist.
    pub fn group_mut(&mut self, key: &str) -> &mut Attrs {
        // Ensure the key exists with a Group value
        if !matches!(self.map.get(key), Some(AttrValue::Group(_))) {
            self.map.insert(key.to_string(), AttrValue::Group(Box::new(Attrs::new())));
        }
        
        match self.map.get_mut(key) {
            Some(AttrValue::Group(attrs)) => attrs.as_mut(),
            _ => unreachable!(),
        }
    }

    /// Get a nested group by key (read-only).
    pub fn group(&self, key: &str) -> Option<&Attrs> {
        match self.map.get(key) {
            Some(AttrValue::Group(attrs)) => Some(attrs.as_ref()),
            _ => None,
        }
    }

    /// Set a value by path (e.g., "Canon:AFInfo:Mode").
    /// Creates intermediate groups as needed.
    pub fn set_path(&mut self, path: &str, value: impl Into<AttrValue>) {
        let parts: Vec<&str> = path.split(':').collect();
        if parts.is_empty() {
            return;
        }

        if parts.len() == 1 {
            self.set(parts[0], value.into());
            return;
        }

        // Navigate/create intermediate groups
        let mut current = self.group_mut(parts[0]);
        for part in &parts[1..parts.len() - 1] {
            current = current.group_mut(part);
        }
        current.set(parts[parts.len() - 1], value.into());
    }

    /// Get a value by path (e.g., "Canon:AFInfo:Mode").
    pub fn get_path(&self, path: &str) -> Option<&AttrValue> {
        let parts: Vec<&str> = path.split(':').collect();
        if parts.is_empty() {
            return None;
        }

        if parts.len() == 1 {
            return self.get(parts[0]);
        }

        // Navigate through groups
        let mut current = self.group(parts[0])?;
        for part in &parts[1..parts.len() - 1] {
            current = current.group(part)?;
        }
        current.get(parts[parts.len() - 1])
    }

    /// Iterate over all values recursively, yielding (path, value) pairs.
    /// Paths are colon-separated (e.g., "Canon:AFInfo:Mode").
    pub fn iter_flat(&self) -> FlatIter<'_> {
        FlatIter::new(self)
    }

    /// Count all values recursively (including nested groups).
    pub fn count_recursive(&self) -> usize {
        let mut count = 0;
        for (_, value) in self.map.iter() {
            match value {
                AttrValue::Group(nested) => count += nested.count_recursive(),
                _ => count += 1,
            }
        }
        count
    }

    /// Pretty-print the attrs tree with indentation.
    pub fn display_tree(&self) -> AttrsTreeDisplay<'_> {
        AttrsTreeDisplay { attrs: self, indent: 0 }
    }
}

/// Iterator that flattens nested Attrs into (path, value) pairs.
pub struct FlatIter<'a> {
    stack: Vec<(String, std::collections::hash_map::Iter<'a, String, AttrValue>)>,
}

impl<'a> FlatIter<'a> {
    fn new(attrs: &'a Attrs) -> Self {
        Self {
            stack: vec![(String::new(), attrs.map.iter())],
        }
    }
}

impl<'a> Iterator for FlatIter<'a> {
    type Item = (String, &'a AttrValue);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (prefix, iter) = self.stack.last_mut()?;
            
            if let Some((key, value)) = iter.next() {
                let path = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}:{}", prefix, key)
                };

                match value {
                    AttrValue::Group(nested) => {
                        // Push nested group onto stack
                        self.stack.push((path, nested.map.iter()));
                        continue;
                    }
                    _ => return Some((path, value)),
                }
            } else {
                // Current iterator exhausted, pop stack
                self.stack.pop();
            }
        }
    }
}

/// Tree-style display for Attrs.
pub struct AttrsTreeDisplay<'a> {
    attrs: &'a Attrs,
    indent: usize,
}

impl<'a> std::fmt::Display for AttrsTreeDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent_str = "  ".repeat(self.indent);
        
        // Sort keys for consistent output
        let mut keys: Vec<&String> = self.attrs.map.keys().collect();
        keys.sort();

        for key in keys {
            if let Some(value) = self.attrs.map.get(key) {
                match value {
                    AttrValue::Group(nested) => {
                        writeln!(f, "{}{}:", indent_str, key)?;
                        let nested_display = AttrsTreeDisplay { 
                            attrs: nested, 
                            indent: self.indent + 1 
                        };
                        write!(f, "{}", nested_display)?;
                    }
                    _ => writeln!(f, "{}{}: {}", indent_str, key, value)?,
                }
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for Attrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_tree())
    }
}

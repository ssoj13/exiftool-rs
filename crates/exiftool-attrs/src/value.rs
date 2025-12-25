//! Attribute value types.
//!
//! AttrValue represents any metadata value with support for
//! EXIF-specific types like rationals and byte arrays.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use uuid::Uuid;

/// Generic attribute value supporting EXIF types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use]
pub enum AttrValue {
    // Basic types
    Bool(bool),
    Str(String),
    Int8(i8),
    Int(i32),
    UInt(u32),
    Int64(i64),
    UInt64(u64),
    Float(f32),
    Double(f64),

    // EXIF-specific types
    /// Signed rational (numerator, denominator).
    Rational(i32, i32),
    /// Unsigned rational (numerator, denominator).
    URational(u32, u32),
    /// Binary data (undefined bytes).
    Bytes(Vec<u8>),
    /// Date/time value.
    DateTime(NaiveDateTime),

    // Vector types
    Vec3([f32; 3]),
    Vec4([f32; 4]),

    // Collection types
    Uuid(Uuid),
    List(Vec<AttrValue>),
    Map(HashMap<String, AttrValue>),
    Set(HashSet<AttrValue>),

    /// JSON-encoded nested data.
    Json(String),

    /// Nested attribute group (for MakerNotes sub-IFDs).
    Group(Box<crate::Attrs>),
}

impl AttrValue {
    /// Get type name for error messages.
    pub fn type_name(&self) -> &'static str {
        match self {
            AttrValue::Bool(_) => "bool",
            AttrValue::Str(_) => "string",
            AttrValue::Int8(_) => "int8",
            AttrValue::Int(_) => "int32",
            AttrValue::UInt(_) => "uint32",
            AttrValue::Int64(_) => "int64",
            AttrValue::UInt64(_) => "uint64",
            AttrValue::Float(_) => "float",
            AttrValue::Double(_) => "double",
            AttrValue::Rational(_, _) => "rational",
            AttrValue::URational(_, _) => "urational",
            AttrValue::Bytes(_) => "bytes",
            AttrValue::DateTime(_) => "datetime",
            AttrValue::Vec3(_) => "vec3",
            AttrValue::Vec4(_) => "vec4",
            AttrValue::Uuid(_) => "uuid",
            AttrValue::List(_) => "list",
            AttrValue::Map(_) => "map",
            AttrValue::Set(_) => "set",
            AttrValue::Json(_) => "json",
            AttrValue::Group(_) => "group",
        }
    }

    /// Try to get as string reference.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            AttrValue::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get as i32.
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            AttrValue::Int(v) => Some(*v),
            AttrValue::Int8(v) => Some(*v as i32),
            _ => None,
        }
    }

    /// Try to get as u32.
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            AttrValue::UInt(v) => Some(*v),
            _ => None,
        }
    }

    /// Try to get as f64 (converts from any numeric).
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            AttrValue::Float(v) => Some(*v as f64),
            AttrValue::Double(v) => Some(*v),
            AttrValue::Int(v) => Some(*v as f64),
            AttrValue::UInt(v) => Some(*v as f64),
            AttrValue::Rational(n, d) if *d != 0 => Some(*n as f64 / *d as f64),
            AttrValue::URational(n, d) if *d != 0 => Some(*n as f64 / *d as f64),
            _ => None,
        }
    }

    /// Try to get as bytes.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            AttrValue::Bytes(v) => Some(v),
            _ => None,
        }
    }
}

impl Hash for AttrValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use std::collections::hash_map::DefaultHasher;
        std::mem::discriminant(self).hash(state);
        match self {
            AttrValue::Bool(v) => v.hash(state),
            AttrValue::Str(v) => v.hash(state),
            AttrValue::Int8(v) => v.hash(state),
            AttrValue::Int(v) => v.hash(state),
            AttrValue::UInt(v) => v.hash(state),
            AttrValue::Int64(v) => v.hash(state),
            AttrValue::UInt64(v) => v.hash(state),
            AttrValue::Float(v) => v.to_bits().hash(state),
            AttrValue::Double(v) => v.to_bits().hash(state),
            AttrValue::Rational(n, d) => {
                n.hash(state);
                d.hash(state);
            }
            AttrValue::URational(n, d) => {
                n.hash(state);
                d.hash(state);
            }
            AttrValue::Bytes(v) => v.hash(state),
            AttrValue::DateTime(v) => v.hash(state),
            AttrValue::Vec3(arr) => arr.iter().for_each(|f| f.to_bits().hash(state)),
            AttrValue::Vec4(arr) => arr.iter().for_each(|f| f.to_bits().hash(state)),
            AttrValue::Uuid(v) => v.hash(state),
            AttrValue::List(v) => v.hash(state),
            AttrValue::Map(v) => {
                let mut acc: u64 = 0;
                for (k, val) in v {
                    let mut h = DefaultHasher::new();
                    k.hash(&mut h);
                    val.hash(&mut h);
                    acc ^= h.finish();
                }
                acc.hash(state);
            }
            AttrValue::Set(v) => {
                let mut acc: u64 = 0;
                for val in v {
                    let mut h = DefaultHasher::new();
                    val.hash(&mut h);
                    acc ^= h.finish();
                }
                acc.hash(state);
            }
            AttrValue::Json(v) => v.hash(state),
            AttrValue::Group(v) => v.hash_all().hash(state),
        }
    }
}

fn f32_bits_eq(a: f32, b: f32) -> bool {
    a.to_bits() == b.to_bits()
}

fn f32_slice_eq(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| f32_bits_eq(*x, *y))
}

impl PartialEq for AttrValue {
    fn eq(&self, other: &Self) -> bool {
        use AttrValue::*;
        match (self, other) {
            (Bool(a), Bool(b)) => a == b,
            (Str(a), Str(b)) => a == b,
            (Int8(a), Int8(b)) => a == b,
            (Int(a), Int(b)) => a == b,
            (UInt(a), UInt(b)) => a == b,
            (Int64(a), Int64(b)) => a == b,
            (UInt64(a), UInt64(b)) => a == b,
            (Float(a), Float(b)) => f32_bits_eq(*a, *b),
            (Double(a), Double(b)) => a.to_bits() == b.to_bits(),
            (Rational(n1, d1), Rational(n2, d2)) => n1 == n2 && d1 == d2,
            (URational(n1, d1), URational(n2, d2)) => n1 == n2 && d1 == d2,
            (Bytes(a), Bytes(b)) => a == b,
            (DateTime(a), DateTime(b)) => a == b,
            (Vec3(a), Vec3(b)) => f32_slice_eq(a, b),
            (Vec4(a), Vec4(b)) => f32_slice_eq(a, b),
            (Uuid(a), Uuid(b)) => a == b,
            (List(a), List(b)) => a == b,
            (Map(a), Map(b)) => {
                a.len() == b.len() && a.iter().all(|(k, v)| b.get(k).is_some_and(|ov| ov == v))
            }
            (Set(a), Set(b)) => a == b,
            (Json(a), Json(b)) => a == b,
            (Group(a), Group(b)) => a.hash_all() == b.hash_all(),
            _ => false,
        }
    }
}

impl Eq for AttrValue {}

impl std::fmt::Display for AttrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttrValue::Bool(v) => write!(f, "{}", v),
            AttrValue::Str(v) => write!(f, "{}", v),
            AttrValue::Int8(v) => write!(f, "{}", v),
            AttrValue::Int(v) => write!(f, "{}", v),
            AttrValue::UInt(v) => write!(f, "{}", v),
            AttrValue::Int64(v) => write!(f, "{}", v),
            AttrValue::UInt64(v) => write!(f, "{}", v),
            AttrValue::Float(v) => write!(f, "{}", v),
            AttrValue::Double(v) => write!(f, "{}", v),
            AttrValue::Rational(n, d) => write!(f, "{}/{}", n, d),
            AttrValue::URational(n, d) => write!(f, "{}/{}", n, d),
            AttrValue::Bytes(v) => write!(f, "<{} bytes>", v.len()),
            AttrValue::DateTime(v) => write!(f, "{}", v.format("%Y:%m:%d %H:%M:%S")),
            AttrValue::Vec3(v) => write!(f, "[{}, {}, {}]", v[0], v[1], v[2]),
            AttrValue::Vec4(v) => write!(f, "[{}, {}, {}, {}]", v[0], v[1], v[2], v[3]),
            AttrValue::Uuid(v) => write!(f, "{}", v),
            AttrValue::List(v) => write!(f, "[{} items]", v.len()),
            AttrValue::Map(v) => write!(f, "{{{} entries}}", v.len()),
            AttrValue::Set(v) => write!(f, "{{{} items}}", v.len()),
            AttrValue::Json(v) => write!(f, "{}", v),
            AttrValue::Group(v) => write!(f, "<group: {} attrs>", v.len()),
        }
    }
}

// Conversion traits for convenience

impl From<bool> for AttrValue {
    fn from(v: bool) -> Self {
        AttrValue::Bool(v)
    }
}

impl From<i32> for AttrValue {
    fn from(v: i32) -> Self {
        AttrValue::Int(v)
    }
}

impl From<u32> for AttrValue {
    fn from(v: u32) -> Self {
        AttrValue::UInt(v)
    }
}

impl From<f32> for AttrValue {
    fn from(v: f32) -> Self {
        AttrValue::Float(v)
    }
}

impl From<f64> for AttrValue {
    fn from(v: f64) -> Self {
        AttrValue::Double(v)
    }
}

impl From<String> for AttrValue {
    fn from(v: String) -> Self {
        AttrValue::Str(v)
    }
}

impl From<&str> for AttrValue {
    fn from(v: &str) -> Self {
        AttrValue::Str(v.to_string())
    }
}

impl From<Vec<u8>> for AttrValue {
    fn from(v: Vec<u8>) -> Self {
        AttrValue::Bytes(v)
    }
}

impl From<Uuid> for AttrValue {
    fn from(v: Uuid) -> Self {
        AttrValue::Uuid(v)
    }
}

impl From<NaiveDateTime> for AttrValue {
    fn from(v: NaiveDateTime) -> Self {
        AttrValue::DateTime(v)
    }
}

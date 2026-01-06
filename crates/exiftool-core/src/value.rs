//! Raw EXIF values before conversion.
//!
//! RawValue represents the parsed binary data from EXIF tags
//! before any value conversion (ValueConv) or print conversion (PrintConv).

use crate::ExifFormat;

/// Unsigned rational number (numerator/denominator).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct URational {
    pub num: u32,
    pub den: u32,
}

impl URational {
    pub const fn new(num: u32, den: u32) -> Self {
        Self { num, den }
    }

    /// Convert to f64, returning 0.0 if denominator is zero.
    pub fn to_f64(self) -> f64 {
        if self.den == 0 {
            0.0
        } else {
            self.num as f64 / self.den as f64
        }
    }
}

impl std::fmt::Display for URational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

/// Signed rational number (numerator/denominator).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct SRational {
    pub num: i32,
    pub den: i32,
}

impl SRational {
    pub const fn new(num: i32, den: i32) -> Self {
        Self { num, den }
    }

    /// Convert to f64, returning 0.0 if denominator is zero.
    pub fn to_f64(self) -> f64 {
        if self.den == 0 {
            0.0
        } else {
            self.num as f64 / self.den as f64
        }
    }
}

impl std::fmt::Display for SRational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

/// Raw value parsed from EXIF data.
///
/// This represents the binary data interpreted according to the EXIF format type.
/// Single values and arrays are unified - single value is just array of length 1.
#[derive(Debug, Clone, PartialEq)]
#[must_use]
pub enum RawValue {
    /// Unsigned 8-bit integers (format 1).
    UInt8(Vec<u8>),
    /// ASCII string, null-terminated (format 2).
    String(String),
    /// Unsigned 16-bit integers (format 3).
    UInt16(Vec<u16>),
    /// Unsigned 32-bit integers (format 4).
    UInt32(Vec<u32>),
    /// Unsigned rationals (format 5).
    URational(Vec<URational>),
    /// Signed 8-bit integers (format 6).
    Int8(Vec<i8>),
    /// Undefined/binary data (format 7).
    Undefined(Vec<u8>),
    /// Signed 16-bit integers (format 8).
    Int16(Vec<i16>),
    /// Signed 32-bit integers (format 9).
    Int32(Vec<i32>),
    /// Signed rationals (format 10).
    SRational(Vec<SRational>),
    /// 32-bit floats (format 11).
    Float(Vec<f32>),
    /// 64-bit doubles (format 12).
    Double(Vec<f64>),
    /// Unsigned 64-bit integers (format 16, BigTIFF).
    UInt64(Vec<u64>),
    /// Signed 64-bit integers (format 17, BigTIFF).
    Int64(Vec<i64>),
}

impl RawValue {
    /// Get the EXIF format type of this value.
    pub fn format(&self) -> ExifFormat {
        match self {
            RawValue::UInt8(_) => ExifFormat::UInt8,
            RawValue::String(_) => ExifFormat::String,
            RawValue::UInt16(_) => ExifFormat::UInt16,
            RawValue::UInt32(_) => ExifFormat::UInt32,
            RawValue::URational(_) => ExifFormat::URational,
            RawValue::Int8(_) => ExifFormat::Int8,
            RawValue::Undefined(_) => ExifFormat::Undefined,
            RawValue::Int16(_) => ExifFormat::Int16,
            RawValue::Int32(_) => ExifFormat::Int32,
            RawValue::SRational(_) => ExifFormat::SRational,
            RawValue::Float(_) => ExifFormat::Float,
            RawValue::Double(_) => ExifFormat::Double,
            RawValue::UInt64(_) => ExifFormat::UInt64,
            RawValue::Int64(_) => ExifFormat::Int64,
        }
    }

    /// Number of elements in this value.
    pub fn count(&self) -> usize {
        match self {
            RawValue::UInt8(v) => v.len(),
            RawValue::String(s) => s.len(),
            RawValue::UInt16(v) => v.len(),
            RawValue::UInt32(v) => v.len(),
            RawValue::URational(v) => v.len(),
            RawValue::Int8(v) => v.len(),
            RawValue::Undefined(v) => v.len(),
            RawValue::Int16(v) => v.len(),
            RawValue::Int32(v) => v.len(),
            RawValue::SRational(v) => v.len(),
            RawValue::Float(v) => v.len(),
            RawValue::Double(v) => v.len(),
            RawValue::UInt64(v) => v.len(),
            RawValue::Int64(v) => v.len(),
        }
    }

    /// Try to get as a single u32 value.
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            RawValue::UInt8(v) if v.len() == 1 => Some(v[0] as u32),
            RawValue::UInt16(v) if v.len() == 1 => Some(v[0] as u32),
            RawValue::UInt32(v) if v.len() == 1 => Some(v[0]),
            _ => None,
        }
    }

    /// Try to get as Vec<u32> (for StripOffsets/StripByteCounts).
    pub fn as_u32_vec(&self) -> Option<Vec<u32>> {
        match self {
            RawValue::UInt8(v) => Some(v.iter().map(|&x| x as u32).collect()),
            RawValue::UInt16(v) => Some(v.iter().map(|&x| x as u32).collect()),
            RawValue::UInt32(v) => Some(v.clone()),
            RawValue::UInt64(v) => Some(v.iter().map(|&x| x as u32).collect()),
            _ => None,
        }
    }

    /// Try to get as a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            RawValue::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Try to get as a single unsigned rational.
    pub fn as_urational(&self) -> Option<URational> {
        match self {
            RawValue::URational(v) if !v.is_empty() => Some(v[0]),
            _ => None,
        }
    }

    /// Try to get as a single signed rational.
    pub fn as_srational(&self) -> Option<SRational> {
        match self {
            RawValue::SRational(v) if !v.is_empty() => Some(v[0]),
            _ => None,
        }
    }

    /// Get raw bytes reference for undefined/binary data.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            RawValue::Undefined(v) => Some(v.as_slice()),
            RawValue::UInt8(v) => Some(v.as_slice()),
            _ => None,
        }
    }
}

impl std::fmt::Display for RawValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawValue::String(s) => write!(f, "{}", s),
            RawValue::UInt8(v) if v.len() == 1 => write!(f, "{}", v[0]),
            RawValue::UInt16(v) if v.len() == 1 => write!(f, "{}", v[0]),
            RawValue::UInt32(v) if v.len() == 1 => write!(f, "{}", v[0]),
            RawValue::URational(v) if v.len() == 1 => write!(f, "{}", v[0]),
            RawValue::SRational(v) if v.len() == 1 => write!(f, "{}", v[0]),
            RawValue::Float(v) if v.len() == 1 => write!(f, "{}", v[0]),
            RawValue::Double(v) if v.len() == 1 => write!(f, "{}", v[0]),
            RawValue::Undefined(v) => write!(f, "<{} bytes>", v.len()),
            _ => write!(f, "<{} x {}>", self.count(), self.format().name()),
        }
    }
}

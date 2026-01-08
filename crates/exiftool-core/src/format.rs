//! EXIF data format types.
//!
//! EXIF specification defines 18 data types (formats) for tag values.
//! Each format has a specific size in bytes.
//!
//! Reference: EXIF 2.32 specification, Section 4.6.2

use crate::{Error, Result};

/// EXIF data format types per TIFF/EXIF specification.
///
/// Format IDs 1-12 are from TIFF 6.0.
/// Format IDs 13-18 are EXIF extensions (13 = IFD pointer, 16-18 = BigTIFF).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ExifFormat {
    /// Unsigned 8-bit integer (BYTE).
    UInt8 = 1,
    /// ASCII string, null-terminated.
    String = 2,
    /// Unsigned 16-bit integer (SHORT).
    UInt16 = 3,
    /// Unsigned 32-bit integer (LONG).
    UInt32 = 4,
    /// Unsigned rational: two LONG values (numerator/denominator).
    URational = 5,
    /// Signed 8-bit integer (SBYTE).
    Int8 = 6,
    /// Undefined byte sequence (UNDEFINED).
    Undefined = 7,
    /// Signed 16-bit integer (SSHORT).
    Int16 = 8,
    /// Signed 32-bit integer (SLONG).
    Int32 = 9,
    /// Signed rational: two SLONG values (numerator/denominator).
    SRational = 10,
    /// 32-bit IEEE float (FLOAT).
    Float = 11,
    /// 64-bit IEEE double (DOUBLE).
    Double = 12,
    /// IFD pointer (same as LONG, used for sub-IFDs).
    Ifd = 13,
    /// Unicode string (UTF-16), rarely used.
    Unicode = 14,
    /// Complex number (two floats), rarely used.
    Complex = 15,
    /// Unsigned 64-bit integer (BigTIFF LONG8).
    UInt64 = 16,
    /// Signed 64-bit integer (BigTIFF SLONG8).
    Int64 = 17,
    /// 64-bit IFD pointer (BigTIFF IFD8).
    Ifd64 = 18,
    /// UTF-8 string (EXIF 3.0, type 129).
    Utf8 = 129,
}

impl ExifFormat {
    /// Parse format from u16 value.
    pub fn from_u16(value: u16) -> Result<Self> {
        match value {
            1 => Ok(ExifFormat::UInt8),
            2 => Ok(ExifFormat::String),
            3 => Ok(ExifFormat::UInt16),
            4 => Ok(ExifFormat::UInt32),
            5 => Ok(ExifFormat::URational),
            6 => Ok(ExifFormat::Int8),
            7 => Ok(ExifFormat::Undefined),
            8 => Ok(ExifFormat::Int16),
            9 => Ok(ExifFormat::Int32),
            10 => Ok(ExifFormat::SRational),
            11 => Ok(ExifFormat::Float),
            12 => Ok(ExifFormat::Double),
            13 => Ok(ExifFormat::Ifd),
            14 => Ok(ExifFormat::Unicode),
            15 => Ok(ExifFormat::Complex),
            16 => Ok(ExifFormat::UInt64),
            17 => Ok(ExifFormat::Int64),
            18 => Ok(ExifFormat::Ifd64),
            129 => Ok(ExifFormat::Utf8),
            _ => Err(Error::InvalidFormat(value)),
        }
    }

    /// Size of one element in bytes.
    ///
    /// For rationals, this is size of the complete value (8 bytes = 2 x 4).
    #[inline]
    pub const fn size(self) -> usize {
        match self {
            ExifFormat::UInt8 => 1,
            ExifFormat::String => 1,
            ExifFormat::UInt16 => 2,
            ExifFormat::UInt32 => 4,
            ExifFormat::URational => 8, // 2 x u32
            ExifFormat::Int8 => 1,
            ExifFormat::Undefined => 1,
            ExifFormat::Int16 => 2,
            ExifFormat::Int32 => 4,
            ExifFormat::SRational => 8, // 2 x i32
            ExifFormat::Float => 4,
            ExifFormat::Double => 8,
            ExifFormat::Ifd => 4,
            ExifFormat::Unicode => 2,
            ExifFormat::Complex => 8, // 2 x f32
            ExifFormat::UInt64 => 8,
            ExifFormat::Int64 => 8,
            ExifFormat::Ifd64 => 8,
            ExifFormat::Utf8 => 1,
        }
    }

    /// Human-readable name matching ExifTool conventions.
    pub const fn name(self) -> &'static str {
        match self {
            ExifFormat::UInt8 => "int8u",
            ExifFormat::String => "string",
            ExifFormat::UInt16 => "int16u",
            ExifFormat::UInt32 => "int32u",
            ExifFormat::URational => "rational64u",
            ExifFormat::Int8 => "int8s",
            ExifFormat::Undefined => "undef",
            ExifFormat::Int16 => "int16s",
            ExifFormat::Int32 => "int32s",
            ExifFormat::SRational => "rational64s",
            ExifFormat::Float => "float",
            ExifFormat::Double => "double",
            ExifFormat::Ifd => "ifd",
            ExifFormat::Unicode => "unicode",
            ExifFormat::Complex => "complex",
            ExifFormat::UInt64 => "int64u",
            ExifFormat::Int64 => "int64s",
            ExifFormat::Ifd64 => "ifd64",
            ExifFormat::Utf8 => "utf8",
        }
    }

    /// Check if this format is numeric (integer or float).
    #[inline]
    pub const fn is_numeric(self) -> bool {
        matches!(
            self,
            ExifFormat::UInt8
                | ExifFormat::UInt16
                | ExifFormat::UInt32
                | ExifFormat::UInt64
                | ExifFormat::Int8
                | ExifFormat::Int16
                | ExifFormat::Int32
                | ExifFormat::Int64
                | ExifFormat::Float
                | ExifFormat::Double
                | ExifFormat::URational
                | ExifFormat::SRational
        )
    }

    /// Check if this format is a rational number.
    #[inline]
    pub const fn is_rational(self) -> bool {
        matches!(self, ExifFormat::URational | ExifFormat::SRational)
    }

    /// Check if this format is an IFD pointer.
    #[inline]
    pub const fn is_ifd_pointer(self) -> bool {
        matches!(self, ExifFormat::Ifd | ExifFormat::Ifd64)
    }
}

impl std::fmt::Display for ExifFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_sizes() {
        assert_eq!(ExifFormat::UInt8.size(), 1);
        assert_eq!(ExifFormat::UInt16.size(), 2);
        assert_eq!(ExifFormat::UInt32.size(), 4);
        assert_eq!(ExifFormat::URational.size(), 8);
        assert_eq!(ExifFormat::Double.size(), 8);
    }

    #[test]
    fn format_parsing() {
        assert_eq!(ExifFormat::from_u16(1).unwrap(), ExifFormat::UInt8);
        assert_eq!(ExifFormat::from_u16(5).unwrap(), ExifFormat::URational);
        assert_eq!(ExifFormat::from_u16(129).unwrap(), ExifFormat::Utf8);
        assert!(ExifFormat::from_u16(0).is_err());
        assert!(ExifFormat::from_u16(99).is_err());
    }

    #[test]
    fn utf8_format() {
        let utf8 = ExifFormat::Utf8;
        assert_eq!(utf8.size(), 1);
        assert_eq!(utf8.name(), "utf8");
        assert!(!utf8.is_numeric());
        assert!(!utf8.is_rational());
        assert!(!utf8.is_ifd_pointer());
    }
}

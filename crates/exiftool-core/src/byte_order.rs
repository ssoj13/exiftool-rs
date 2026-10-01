//! Byte order (endianness) handling for TIFF/EXIF parsing.
//!
//! TIFF files can be either big-endian (Motorola, "MM") or
//! little-endian (Intel, "II"). The byte order marker appears
//! at the start of the TIFF header.

use crate::{Error, Result};

/// Byte order for multi-byte values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub enum ByteOrder {
    /// Little-endian (Intel, "II") - least significant byte first.
    LittleEndian,
    /// Big-endian (Motorola, "MM") - most significant byte first.
    BigEndian,
}

impl ByteOrder {
    /// Parse byte order from TIFF header marker.
    ///
    /// - `II` (0x4949) = Little-endian (Intel)
    /// - `MM` (0x4D4D) = Big-endian (Motorola)
    pub fn from_marker(marker: [u8; 2]) -> Result<Self> {
        match &marker {
            b"II" => Ok(ByteOrder::LittleEndian),
            b"MM" => Ok(ByteOrder::BigEndian),
            _ => Err(Error::InvalidByteOrder(marker)),
        }
    }

    /// Read u16 from bytes with this byte order.
    #[inline]
    pub fn read_u16(self, bytes: [u8; 2]) -> u16 {
        match self {
            ByteOrder::LittleEndian => u16::from_le_bytes(bytes),
            ByteOrder::BigEndian => u16::from_be_bytes(bytes),
        }
    }

    /// Read u32 from bytes with this byte order.
    #[inline]
    pub fn read_u32(self, bytes: [u8; 4]) -> u32 {
        match self {
            ByteOrder::LittleEndian => u32::from_le_bytes(bytes),
            ByteOrder::BigEndian => u32::from_be_bytes(bytes),
        }
    }

    /// Read i16 from bytes with this byte order.
    #[inline]
    pub fn read_i16(self, bytes: [u8; 2]) -> i16 {
        match self {
            ByteOrder::LittleEndian => i16::from_le_bytes(bytes),
            ByteOrder::BigEndian => i16::from_be_bytes(bytes),
        }
    }

    /// Read i32 from bytes with this byte order.
    #[inline]
    pub fn read_i32(self, bytes: [u8; 4]) -> i32 {
        match self {
            ByteOrder::LittleEndian => i32::from_le_bytes(bytes),
            ByteOrder::BigEndian => i32::from_be_bytes(bytes),
        }
    }

    /// Read u64 from bytes with this byte order (for BigTIFF).
    #[inline]
    pub fn read_u64(self, bytes: [u8; 8]) -> u64 {
        match self {
            ByteOrder::LittleEndian => u64::from_le_bytes(bytes),
            ByteOrder::BigEndian => u64::from_be_bytes(bytes),
        }
    }

    /// Read f32 from bytes with this byte order.
    #[inline]
    pub fn read_f32(self, bytes: [u8; 4]) -> f32 {
        f32::from_bits(self.read_u32(bytes))
    }

    /// Read f64 from bytes with this byte order.
    #[inline]
    pub fn read_f64(self, bytes: [u8; 8]) -> f64 {
        f64::from_bits(self.read_u64(bytes))
    }
}

impl std::fmt::Display for ByteOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByteOrder::LittleEndian => write!(f, "little-endian (II)"),
            ByteOrder::BigEndian => write!(f, "big-endian (MM)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_byte_order() {
        assert_eq!(ByteOrder::from_marker(*b"II").unwrap(), ByteOrder::LittleEndian);
        assert_eq!(ByteOrder::from_marker(*b"MM").unwrap(), ByteOrder::BigEndian);
        assert!(ByteOrder::from_marker(*b"XX").is_err());
    }

    #[test]
    fn read_values() {
        let le = ByteOrder::LittleEndian;
        let be = ByteOrder::BigEndian;

        // 0x0102 in different byte orders
        assert_eq!(le.read_u16([0x02, 0x01]), 0x0102);
        assert_eq!(be.read_u16([0x01, 0x02]), 0x0102);

        // 0x01020304 in different byte orders
        assert_eq!(le.read_u32([0x04, 0x03, 0x02, 0x01]), 0x01020304);
        assert_eq!(be.read_u32([0x01, 0x02, 0x03, 0x04]), 0x01020304);
    }
}

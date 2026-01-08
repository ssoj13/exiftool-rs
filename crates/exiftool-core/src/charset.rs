//! Character encoding support for metadata strings.
//!
//! Many cameras (especially Japanese brands) store strings in non-UTF8 encodings.
//! This module provides decoding for common character sets found in EXIF data.
//!
//! # Supported Encodings
//!
//! - UTF-8 (default)
//! - ShiftJIS (Japanese cameras: Canon, Nikon JP models)
//! - EUC-JP (older Japanese systems)
//! - Latin1 / ISO-8859-1 (Western European)
//! - Windows-1252 (Windows Western)
//! - Big5 (Traditional Chinese)
//! - GB2312/GBK (Simplified Chinese)
//! - EUC-KR (Korean)
//!
//! # Usage
//!
//! ```
//! use exiftool_core::charset::{Charset, decode};
//!
//! // Decode ShiftJIS bytes (キ=0x834C, ヤ=0x8384, ノ=0x836D, ン=0x8393)
//! let bytes = [0x83, 0x4C, 0x83, 0x84, 0x83, 0x6D, 0x83, 0x93]; // "キヤノン" in ShiftJIS
//! let text = decode(&bytes, Charset::ShiftJIS);
//! assert_eq!(text, "キヤノン");
//!
//! // Auto-detect encoding
//! let detected = Charset::detect(&bytes);
//! ```

use encoding_rs::{
    Encoding, BIG5, EUC_JP, EUC_KR, GBK, SHIFT_JIS, UTF_8, WINDOWS_1252,
};

/// Supported character encodings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Charset {
    /// UTF-8 (default).
    #[default]
    Utf8,
    /// Shift_JIS (Japanese).
    ShiftJIS,
    /// EUC-JP (Japanese Unix).
    EucJP,
    /// ISO-8859-1 / Latin1 (Western European).
    Latin1,
    /// Windows-1252 (Windows Western).
    Windows1252,
    /// Big5 (Traditional Chinese).
    Big5,
    /// GBK / GB2312 (Simplified Chinese).
    Gbk,
    /// EUC-KR (Korean).
    EucKR,
}

impl Charset {
    /// Get encoding_rs Encoding reference (returns None for Latin1 which needs special handling).
    fn encoding(&self) -> Option<&'static Encoding> {
        match self {
            Charset::Utf8 => Some(UTF_8),
            Charset::ShiftJIS => Some(SHIFT_JIS),
            Charset::EucJP => Some(EUC_JP),
            Charset::Latin1 => None, // Handle manually - direct byte to char
            Charset::Windows1252 => Some(WINDOWS_1252),
            Charset::Big5 => Some(BIG5),
            Charset::Gbk => Some(GBK),
            Charset::EucKR => Some(EUC_KR),
        }
    }

    /// Get charset name as string.
    pub fn name(&self) -> &'static str {
        match self {
            Charset::Utf8 => "UTF-8",
            Charset::ShiftJIS => "Shift_JIS",
            Charset::EucJP => "EUC-JP",
            Charset::Latin1 => "ISO-8859-1",
            Charset::Windows1252 => "Windows-1252",
            Charset::Big5 => "Big5",
            Charset::Gbk => "GBK",
            Charset::EucKR => "EUC-KR",
        }
    }

    /// Try to detect encoding from byte patterns.
    ///
    /// Heuristic detection based on common patterns:
    /// - Valid UTF-8 with multi-byte sequences → UTF-8
    /// - ShiftJIS lead bytes (0x81-0x9F, 0xE0-0xFC) → ShiftJIS
    /// - High-bit-set single bytes (0x80-0xFF) → Latin1/Windows-1252
    ///
    /// Returns None if data is ASCII-only or ambiguous.
    pub fn detect(data: &[u8]) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        // Check for valid UTF-8 first
        if std::str::from_utf8(data).is_ok() {
            // Check if it has any multi-byte UTF-8 sequences
            if data.iter().any(|&b| b >= 0x80) {
                return Some(Charset::Utf8);
            }
            // Pure ASCII - no encoding needed
            return None;
        }

        // Check for ShiftJIS patterns
        if Self::looks_like_shiftjis(data) {
            return Some(Charset::ShiftJIS);
        }

        // Check for EUC-JP patterns
        if Self::looks_like_eucjp(data) {
            return Some(Charset::EucJP);
        }

        // Default to Windows-1252 for Western European
        if data.iter().any(|&b| b >= 0x80) {
            return Some(Charset::Windows1252);
        }

        None
    }

    /// Check if data looks like ShiftJIS.
    fn looks_like_shiftjis(data: &[u8]) -> bool {
        let mut i = 0;
        let mut sjis_pairs = 0;
        let mut invalid = 0;

        while i < data.len() {
            let b = data[i];

            // ShiftJIS lead byte ranges
            if (0x81..=0x9F).contains(&b) || (0xE0..=0xFC).contains(&b) {
                if i + 1 < data.len() {
                    let trail = data[i + 1];
                    // Valid trail byte
                    if (0x40..=0x7E).contains(&trail) || (0x80..=0xFC).contains(&trail) {
                        sjis_pairs += 1;
                        i += 2;
                        continue;
                    }
                }
                invalid += 1;
            } else if (0xA1..=0xDF).contains(&b) {
                // Half-width katakana
                sjis_pairs += 1;
            }
            i += 1;
        }

        // ShiftJIS if we found valid pairs and few invalid sequences
        sjis_pairs > 0 && invalid < sjis_pairs / 2
    }

    /// Check if data looks like EUC-JP.
    fn looks_like_eucjp(data: &[u8]) -> bool {
        let mut i = 0;
        let mut eucjp_pairs = 0;

        while i < data.len() {
            let b = data[i];

            // EUC-JP lead byte (0xA1-0xFE)
            if (0xA1..=0xFE).contains(&b) {
                if i + 1 < data.len() {
                    let trail = data[i + 1];
                    // Valid trail byte
                    if (0xA1..=0xFE).contains(&trail) {
                        eucjp_pairs += 1;
                        i += 2;
                        continue;
                    }
                }
            }
            // SS2 (half-width katakana)
            if b == 0x8E && i + 1 < data.len() {
                let trail = data[i + 1];
                if (0xA1..=0xDF).contains(&trail) {
                    eucjp_pairs += 1;
                    i += 2;
                    continue;
                }
            }
            i += 1;
        }

        eucjp_pairs > 0
    }

    /// Parse charset name from string (case-insensitive).
    pub fn from_name(name: &str) -> Option<Self> {
        let name_lower = name.to_lowercase();
        match name_lower.as_str() {
            "utf-8" | "utf8" => Some(Charset::Utf8),
            "shift_jis" | "shift-jis" | "shiftjis" | "sjis" | "ms932" | "cp932" => {
                Some(Charset::ShiftJIS)
            }
            "euc-jp" | "eucjp" | "euc_jp" => Some(Charset::EucJP),
            "iso-8859-1" | "latin1" | "latin-1" | "iso8859-1" => Some(Charset::Latin1),
            "windows-1252" | "cp1252" | "win1252" => Some(Charset::Windows1252),
            "big5" | "big-5" => Some(Charset::Big5),
            "gbk" | "gb2312" | "gb18030" | "cp936" => Some(Charset::Gbk),
            "euc-kr" | "euckr" | "ksc5601" | "cp949" => Some(Charset::EucKR),
            _ => None,
        }
    }
}

/// Decode bytes using specified charset.
///
/// If decoding fails or produces replacement characters, returns a lossy conversion.
pub fn decode(data: &[u8], charset: Charset) -> String {
    // Latin1/ISO-8859-1 is a direct 1:1 byte to Unicode mapping
    if charset == Charset::Latin1 {
        return data.iter().map(|&b| b as char).collect();
    }
    
    if let Some(encoding) = charset.encoding() {
        let (result, _, _) = encoding.decode(data);
        result.into_owned()
    } else {
        // Fallback to UTF-8 lossy
        String::from_utf8_lossy(data).into_owned()
    }
}

/// Decode bytes with auto-detection of charset.
///
/// Tries to detect encoding, falls back to UTF-8 lossy if detection fails.
pub fn decode_auto(data: &[u8]) -> String {
    if let Some(charset) = Charset::detect(data) {
        decode(data, charset)
    } else {
        // Try UTF-8, fall back to lossy
        String::from_utf8(data.to_vec()).unwrap_or_else(|_| String::from_utf8_lossy(data).into_owned())
    }
}

/// Encode string to bytes using specified charset.
///
/// Returns bytes, replacing unencodable characters with '?'.
pub fn encode(text: &str, charset: Charset) -> Vec<u8> {
    // Latin1/ISO-8859-1 - only chars 0-255 can be represented
    if charset == Charset::Latin1 {
        return text.chars().map(|c| {
            if (c as u32) <= 255 { c as u8 } else { b'?' }
        }).collect();
    }
    
    if let Some(encoding) = charset.encoding() {
        let (result, _, _) = encoding.encode(text);
        result.into_owned()
    } else {
        text.as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_decode() {
        let data = "Hello 世界".as_bytes();
        assert_eq!(decode(data, Charset::Utf8), "Hello 世界");
    }

    #[test]
    fn test_shiftjis_decode() {
        // "キヤノン" (Canon) in ShiftJIS
        // キ=0x834C, ヤ=0x8384, ノ=0x836D, ン=0x8393
        let data = [0x83, 0x4C, 0x83, 0x84, 0x83, 0x6D, 0x83, 0x93];
        assert_eq!(decode(&data, Charset::ShiftJIS), "キヤノン");
    }

    #[test]
    fn test_shiftjis_decode_nikon() {
        // "ニコン" (Nikon) in ShiftJIS
        let data = [0x83, 0x6A, 0x83, 0x52, 0x83, 0x93];
        assert_eq!(decode(&data, Charset::ShiftJIS), "ニコン");
    }

    #[test]
    fn test_latin1_decode() {
        // "Ångström" in Latin1
        let data = [0xC5, 0x6E, 0x67, 0x73, 0x74, 0x72, 0xF6, 0x6D];
        assert_eq!(decode(&data, Charset::Latin1), "Ångström");
    }

    #[test]
    fn test_detect_shiftjis() {
        // ShiftJIS bytes (キヤノン)
        let data = [0x83, 0x4C, 0x83, 0x84, 0x83, 0x6D, 0x83, 0x93];
        assert_eq!(Charset::detect(&data), Some(Charset::ShiftJIS));
    }

    #[test]
    fn test_detect_utf8() {
        // Valid UTF-8 with non-ASCII
        let data = "日本語".as_bytes();
        assert_eq!(Charset::detect(data), Some(Charset::Utf8));
    }

    #[test]
    fn test_detect_ascii() {
        // Pure ASCII - no special encoding
        let data = b"Hello World";
        assert_eq!(Charset::detect(data), None);
    }

    #[test]
    fn test_charset_from_name() {
        assert_eq!(Charset::from_name("utf-8"), Some(Charset::Utf8));
        assert_eq!(Charset::from_name("UTF8"), Some(Charset::Utf8));
        assert_eq!(Charset::from_name("shift_jis"), Some(Charset::ShiftJIS));
        assert_eq!(Charset::from_name("SJIS"), Some(Charset::ShiftJIS));
        assert_eq!(Charset::from_name("latin1"), Some(Charset::Latin1));
        assert_eq!(Charset::from_name("unknown"), None);
    }

    #[test]
    fn test_decode_auto() {
        // ShiftJIS (キヤノン)
        let sjis = [0x83, 0x4C, 0x83, 0x84, 0x83, 0x6D, 0x83, 0x93];
        assert_eq!(decode_auto(&sjis), "キヤノン");

        // UTF-8
        let utf8 = "日本語".as_bytes();
        assert_eq!(decode_auto(utf8), "日本語");

        // ASCII
        let ascii = b"Hello";
        assert_eq!(decode_auto(ascii), "Hello");
    }

    #[test]
    fn test_encode() {
        let text = "キヤノン";
        let encoded = encode(text, Charset::ShiftJIS);
        assert_eq!(encoded, vec![0x83, 0x4C, 0x83, 0x84, 0x83, 0x6D, 0x83, 0x93]);
    }
}

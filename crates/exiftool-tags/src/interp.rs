//! Value interpretation system (PrintConv equivalent).
//!
//! Converts raw numeric values to human-readable strings.
//!
//! # Example
//!
//! ```
//! use exiftool_tags::interp::interpret_value;
//!
//! // Orientation tag value 6 = "Rotate 90 CW"
//! let result = interpret_value("Orientation", 6);
//! assert_eq!(result, Some("Rotate 90 CW".to_string()));
//! ```

use phf::phf_map;

/// Interpret a tag value as human-readable string.
pub fn interpret_value(tag_name: &str, value: i64) -> Option<String> {
    match tag_name {
        // IFD0 tags
        "Orientation" => ORIENTATION.get(&(value as u8)).map(|s| s.to_string()),
        "ResolutionUnit" => RESOLUTION_UNIT.get(&(value as u8)).map(|s| s.to_string()),
        "YCbCrPositioning" => YCBCR_POSITIONING.get(&(value as u8)).map(|s| s.to_string()),

        // EXIF tags
        "ExposureProgram" => EXPOSURE_PROGRAM.get(&(value as u8)).map(|s| s.to_string()),
        "MeteringMode" => METERING_MODE.get(&(value as u8)).map(|s| s.to_string()),
        "LightSource" => LIGHT_SOURCE.get(&(value as u8)).map(|s| s.to_string()),
        "Flash" => interpret_flash(value as u16),
        "SensingMethod" => SENSING_METHOD.get(&(value as u8)).map(|s| s.to_string()),
        "FileSource" => FILE_SOURCE.get(&(value as u8)).map(|s| s.to_string()),
        "SceneType" => SCENE_TYPE.get(&(value as u8)).map(|s| s.to_string()),
        "CustomRendered" => CUSTOM_RENDERED.get(&(value as u8)).map(|s| s.to_string()),
        "ExposureMode" => EXPOSURE_MODE.get(&(value as u8)).map(|s| s.to_string()),
        "WhiteBalance" => WHITE_BALANCE.get(&(value as u8)).map(|s| s.to_string()),
        "SceneCaptureType" => SCENE_CAPTURE_TYPE.get(&(value as u8)).map(|s| s.to_string()),
        "GainControl" => GAIN_CONTROL.get(&(value as u8)).map(|s| s.to_string()),
        "Contrast" | "Saturation" | "Sharpness" => CONTRAST_SAT_SHARP.get(&(value as u8)).map(|s| s.to_string()),
        "SubjectDistanceRange" => SUBJECT_DISTANCE_RANGE.get(&(value as u8)).map(|s| s.to_string()),
        "ColorSpace" => COLOR_SPACE.get(&(value as u16)).map(|s| s.to_string()),
        "Compression" => COMPRESSION.get(&(value as u16)).map(|s| s.to_string()),
        "SensitivityType" => SENSITIVITY_TYPE.get(&(value as u8)).map(|s| s.to_string()),

        // GPS tags
        "GPSAltitudeRef" => GPS_ALTITUDE_REF.get(&(value as u8)).map(|s| s.to_string()),
        "GPSStatus" => GPS_STATUS.get(&(value as u8)).map(|s| s.to_string()),
        "GPSMeasureMode" => GPS_MEASURE_MODE.get(&(value as u8)).map(|s| s.to_string()),
        "GPSDifferential" => GPS_DIFFERENTIAL.get(&(value as u8)).map(|s| s.to_string()),

        _ => None,
    }
}

/// Format exposure time as fraction.
pub fn format_exposure_time(seconds: f64) -> String {
    if seconds >= 1.0 {
        format!("{:.1} sec", seconds)
    } else if seconds > 0.0 {
        let denom = (1.0 / seconds).round() as i32;
        format!("1/{} sec", denom)
    } else {
        "0 sec".to_string()
    }
}

/// Format F-number.
pub fn format_fnumber(f: f64) -> String {
    if f == f.floor() {
        format!("f/{:.0}", f)
    } else {
        format!("f/{:.1}", f)
    }
}

/// Format focal length.
pub fn format_focal_length(mm: f64) -> String {
    if mm == mm.floor() {
        format!("{:.0} mm", mm)
    } else {
        format!("{:.1} mm", mm)
    }
}

/// Format GPS coordinate.
pub fn format_gps_coord(degrees: f64, is_latitude: bool) -> String {
    let abs_deg = degrees.abs();
    let d = abs_deg.floor() as i32;
    let m_float = (abs_deg - d as f64) * 60.0;
    let m = m_float.floor() as i32;
    let s = (m_float - m as f64) * 60.0;

    let dir = if is_latitude {
        if degrees >= 0.0 { "N" } else { "S" }
    } else {
        if degrees >= 0.0 { "E" } else { "W" }
    };

    format!("{}° {}' {:.2}\" {}", d, m, s, dir)
}

/// Interpret Flash tag (bitmask).
fn interpret_flash(value: u16) -> Option<String> {
    let fired = (value & 0x01) != 0;
    let mode = (value >> 3) & 0x03;
    let red_eye = (value & 0x40) != 0;

    let mut parts = Vec::new();

    if fired {
        parts.push("Fired");
    } else {
        parts.push("No Flash");
    }

    match mode {
        1 => parts.push("On"),
        2 => parts.push("Off"),
        3 => parts.push("Auto"),
        _ => {}
    }

    if red_eye {
        parts.push("Red-eye reduction");
    }

    Some(parts.join(", "))
}

// === Value tables ===

static ORIENTATION: phf::Map<u8, &'static str> = phf_map! {
    1u8 => "Horizontal (normal)",
    2u8 => "Mirror horizontal",
    3u8 => "Rotate 180",
    4u8 => "Mirror vertical",
    5u8 => "Mirror horizontal and rotate 270 CW",
    6u8 => "Rotate 90 CW",
    7u8 => "Mirror horizontal and rotate 90 CW",
    8u8 => "Rotate 270 CW",
};

static RESOLUTION_UNIT: phf::Map<u8, &'static str> = phf_map! {
    1u8 => "None",
    2u8 => "inches",
    3u8 => "cm",
};

static YCBCR_POSITIONING: phf::Map<u8, &'static str> = phf_map! {
    1u8 => "Centered",
    2u8 => "Co-sited",
};

static EXPOSURE_PROGRAM: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "Not Defined",
    1u8 => "Manual",
    2u8 => "Program AE",
    3u8 => "Aperture-priority AE",
    4u8 => "Shutter speed priority AE",
    5u8 => "Creative (Slow speed)",
    6u8 => "Action (High speed)",
    7u8 => "Portrait",
    8u8 => "Landscape",
    9u8 => "Bulb",
};

static METERING_MODE: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "Unknown",
    1u8 => "Average",
    2u8 => "Center-weighted average",
    3u8 => "Spot",
    4u8 => "Multi-spot",
    5u8 => "Multi-segment",
    6u8 => "Partial",
    255u8 => "Other",
};

static LIGHT_SOURCE: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "Unknown",
    1u8 => "Daylight",
    2u8 => "Fluorescent",
    3u8 => "Tungsten (Incandescent)",
    4u8 => "Flash",
    9u8 => "Fine Weather",
    10u8 => "Cloudy",
    11u8 => "Shade",
    12u8 => "Daylight Fluorescent",
    13u8 => "Day White Fluorescent",
    14u8 => "Cool White Fluorescent",
    15u8 => "White Fluorescent",
    16u8 => "Warm White Fluorescent",
    17u8 => "Standard Light A",
    18u8 => "Standard Light B",
    19u8 => "Standard Light C",
    20u8 => "D55",
    21u8 => "D65",
    22u8 => "D75",
    23u8 => "D50",
    24u8 => "ISO Studio Tungsten",
    255u8 => "Other",
};

static SENSING_METHOD: phf::Map<u8, &'static str> = phf_map! {
    1u8 => "Not defined",
    2u8 => "One-chip color area",
    3u8 => "Two-chip color area",
    4u8 => "Three-chip color area",
    5u8 => "Color sequential area",
    7u8 => "Trilinear",
    8u8 => "Color sequential linear",
};

static FILE_SOURCE: phf::Map<u8, &'static str> = phf_map! {
    1u8 => "Film Scanner",
    2u8 => "Reflection Print Scanner",
    3u8 => "Digital Camera",
};

static SCENE_TYPE: phf::Map<u8, &'static str> = phf_map! {
    1u8 => "Directly photographed",
};

static CUSTOM_RENDERED: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "Normal",
    1u8 => "Custom",
    2u8 => "HDR (no original saved)",
    3u8 => "HDR (original saved)",
    4u8 => "Original (for HDR)",
    6u8 => "Panorama",
    7u8 => "Portrait HDR",
    8u8 => "Portrait",
};

static EXPOSURE_MODE: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "Auto",
    1u8 => "Manual",
    2u8 => "Auto bracket",
};

static WHITE_BALANCE: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "Auto",
    1u8 => "Manual",
};

static SCENE_CAPTURE_TYPE: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "Standard",
    1u8 => "Landscape",
    2u8 => "Portrait",
    3u8 => "Night",
    4u8 => "Other",
};

static GAIN_CONTROL: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "None",
    1u8 => "Low gain up",
    2u8 => "High gain up",
    3u8 => "Low gain down",
    4u8 => "High gain down",
};

static CONTRAST_SAT_SHARP: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "Normal",
    1u8 => "Low",
    2u8 => "High",
};

static SUBJECT_DISTANCE_RANGE: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "Unknown",
    1u8 => "Macro",
    2u8 => "Close",
    3u8 => "Distant",
};

static COLOR_SPACE: phf::Map<u16, &'static str> = phf_map! {
    1u16 => "sRGB",
    2u16 => "Adobe RGB",
    65535u16 => "Uncalibrated",
};

static COMPRESSION: phf::Map<u16, &'static str> = phf_map! {
    1u16 => "Uncompressed",
    2u16 => "CCITT 1D",
    3u16 => "T4/Group 3 Fax",
    4u16 => "T6/Group 4 Fax",
    5u16 => "LZW",
    6u16 => "JPEG (old-style)",
    7u16 => "JPEG",
    8u16 => "Adobe Deflate",
    9u16 => "JBIG B&W",
    10u16 => "JBIG Color",
    99u16 => "JPEG",
    262u16 => "Kodak 262",
    32766u16 => "Next",
    32767u16 => "Sony ARW Compressed",
    32769u16 => "Packed RAW",
    32770u16 => "Samsung SRW Compressed",
    32771u16 => "CCIRLEW",
    32772u16 => "Samsung SRW Compressed 2",
    32773u16 => "PackBits",
    32809u16 => "Thunderscan",
    32867u16 => "Kodak KDC Compressed",
    32895u16 => "IT8CTPAD",
    32896u16 => "IT8LW",
    32897u16 => "IT8MP",
    32898u16 => "IT8BL",
    32908u16 => "PixarFilm",
    32909u16 => "PixarLog",
    32946u16 => "Deflate",
    32947u16 => "DCS",
    33003u16 => "Aperio JPEG 2000 YCbCr",
    33005u16 => "Aperio JPEG 2000 RGB",
    34661u16 => "JBIG",
    34676u16 => "SGILog",
    34677u16 => "SGILog24",
    34712u16 => "JPEG 2000",
    34713u16 => "Nikon NEF Compressed",
    34715u16 => "JBIG2 TIFF FX",
    34718u16 => "Microsoft Document Imaging (MDI) Binary Level Codec",
    34719u16 => "Microsoft Document Imaging (MDI) Progressive Transform Codec",
    34720u16 => "Microsoft Document Imaging (MDI) Vector",
    34887u16 => "ESRI Lerc",
    34892u16 => "Lossy JPEG",
    34925u16 => "LZMA2",
    34926u16 => "Zstd",
    34927u16 => "WebP",
    34933u16 => "PNG",
    34934u16 => "JPEG XR",
    50000u16 => "Kodak DCR Compressed",
    50001u16 => "Pentax PEF Compressed",
    65000u16 => "Kodak DCR Compressed",
    65535u16 => "Pentax PEF Compressed",
};

static SENSITIVITY_TYPE: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "Unknown",
    1u8 => "Standard Output Sensitivity",
    2u8 => "Recommended Exposure Index",
    3u8 => "ISO Speed",
    4u8 => "Standard Output Sensitivity and Recommended Exposure Index",
    5u8 => "Standard Output Sensitivity and ISO Speed",
    6u8 => "Recommended Exposure Index and ISO Speed",
    7u8 => "Standard Output Sensitivity, Recommended Exposure Index and ISO Speed",
};

static GPS_ALTITUDE_REF: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "Above Sea Level",
    1u8 => "Below Sea Level",
};

static GPS_STATUS: phf::Map<u8, &'static str> = phf_map! {
    65u8 => "Measurement Active", // 'A'
    86u8 => "Measurement Void",   // 'V'
};

static GPS_MEASURE_MODE: phf::Map<u8, &'static str> = phf_map! {
    50u8 => "2-Dimensional Measurement", // '2'
    51u8 => "3-Dimensional Measurement", // '3'
};

static GPS_DIFFERENTIAL: phf::Map<u8, &'static str> = phf_map! {
    0u8 => "No Correction",
    1u8 => "Differential Corrected",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orientation() {
        assert_eq!(interpret_value("Orientation", 1), Some("Horizontal (normal)".to_string()));
        assert_eq!(interpret_value("Orientation", 6), Some("Rotate 90 CW".to_string()));
        assert_eq!(interpret_value("Orientation", 99), None);
    }

    #[test]
    fn test_exposure_program() {
        assert_eq!(interpret_value("ExposureProgram", 2), Some("Program AE".to_string()));
        assert_eq!(interpret_value("ExposureProgram", 3), Some("Aperture-priority AE".to_string()));
    }

    #[test]
    fn test_flash() {
        // No flash
        assert_eq!(interpret_value("Flash", 0), Some("No Flash".to_string()));
        // Flash fired
        assert_eq!(interpret_value("Flash", 1), Some("Fired".to_string()));
        // Flash fired, auto mode
        assert_eq!(interpret_value("Flash", 25), Some("Fired, Auto".to_string()));
    }

    #[test]
    fn test_format_exposure() {
        assert_eq!(format_exposure_time(1.0 / 125.0), "1/125 sec");
        assert_eq!(format_exposure_time(2.0), "2.0 sec");
        assert_eq!(format_exposure_time(1.0 / 4000.0), "1/4000 sec");
    }

    #[test]
    fn test_format_fnumber() {
        assert_eq!(format_fnumber(2.8), "f/2.8");
        assert_eq!(format_fnumber(8.0), "f/8");
    }

    #[test]
    fn test_format_gps() {
        let result = format_gps_coord(40.7128, true);
        assert!(result.contains("40°"));
        assert!(result.ends_with(" N"));

        let result = format_gps_coord(-74.0060, false);
        assert!(result.contains("74°"));
        assert!(result.ends_with(" W"));
    }

    #[test]
    fn test_compression() {
        assert_eq!(interpret_value("Compression", 1), Some("Uncompressed".to_string()));
        assert_eq!(interpret_value("Compression", 7), Some("JPEG".to_string()));
    }
}

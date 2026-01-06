//! GoPro MakerNotes/GPMF tag definitions.
//! Auto-generated from ExifTool - DO NOT EDIT

#![allow(dead_code)]

/// Tag definition with name and optional value mappings.
#[derive(Debug, Clone)]
pub struct TagDef {
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}

/// GoPro GPMF FourCC tags (from APP6 "GoPro" segment or MP4 GPMF box)
/// Note: GoPro uses 4-char codes, not u16 IDs
pub static GOPRO_GPMF: phf::Map<&'static str, TagDef> = phf::phf_map! {
    "ACCL" => TagDef { name: "Accelerometer", values: None },
    "GYRO" => TagDef { name: "Gyroscope", values: None },
    "GPS5" => TagDef { name: "GPSData", values: None },
    "GPSU" => TagDef { name: "GPSTimestamp", values: None },
    "GPSF" => TagDef { name: "GPSMeasureMode", values: None },
    "GPSP" => TagDef { name: "GPSHPositioningError", values: None },
    "CORI" => TagDef { name: "CameraOrientation", values: None },
    "IORI" => TagDef { name: "ImageOrientation", values: None },
    "GRAV" => TagDef { name: "GravityVector", values: None },
    "WNDM" => TagDef { name: "WindProcessing", values: None },
    "MWET" => TagDef { name: "MicrophoneWet", values: None },
    "AALP" => TagDef { name: "AudioLevel", values: None },
    "WRGB" => TagDef { name: "WhiteBalanceRGB", values: None },
    "ISOE" => TagDef { name: "ISOSensor", values: None },
    "SHUT" => TagDef { name: "ShutterSpeed", values: None },
    "UNIF" => TagDef { name: "Uniformity", values: None },
    "FACE" => TagDef { name: "FaceDetected", values: None },
    "FCNM" => TagDef { name: "FaceCoordinates", values: None },
    "SCEN" => TagDef { name: "SceneClassification", values: None },
    "HUES" => TagDef { name: "HueScore", values: None },
    "SROT" => TagDef { name: "SensorReadoutTime", values: None },
    "MSKP" => TagDef { name: "MaskPresent", values: None },
    "LRVO" => TagDef { name: "LRVOffset", values: None },
    "LRVS" => TagDef { name: "LRVSize", values: None },
    "DVID" => TagDef { name: "DeviceID", values: None },
    "DVNM" => TagDef { name: "DeviceName", values: None },
    "SIUN" => TagDef { name: "SIUnits", values: None },
    "UNIT" => TagDef { name: "Units", values: None },
    "SCAL" => TagDef { name: "Scale", values: None },
    "TMPC" => TagDef { name: "Temperature", values: None },
    "STMP" => TagDef { name: "Timestamp", values: None },
    "TSMP" => TagDef { name: "TotalSamples", values: None },
    "ORIN" => TagDef { name: "OriginalInputFormat", values: None },
    "ORIO" => TagDef { name: "OriginalOutputFormat", values: None },
    "MTRX" => TagDef { name: "TransformMatrix", values: None },
    "CASN" => TagDef { name: "CameraSerialNumber", values: None },
    "MINF" => TagDef { name: "ModelInfo", values: None },
    "FIRM" => TagDef { name: "FirmwareVersion", values: None },
    "LENS" => TagDef { name: "LensSerialNumber", values: None },
    "CAME" => TagDef { name: "CameraName", values: None },
    "SETT" => TagDef { name: "CameraSettings", values: None },
    "OREN" => TagDef { name: "AutoOrientation", values: None },
    "WBAL" => TagDef { name: "WhiteBalance", values: None },
    "MUID" => TagDef { name: "MediaUID", values: None },
    "HMMT" => TagDef { name: "HighlightMoments", values: None },
    "EXPO" => TagDef { name: "ExposureTimes", values: None },
    "MAGN" => TagDef { name: "Magnetometer", values: None },
    "ALLD" => TagDef { name: "AutoLowLightDuration", values: None },
    "DISP" => TagDef { name: "Disparity", values: None },
};

/// Lookup by FourCC string
pub fn lookup(fourcc: &str) -> Option<&'static TagDef> {
    GOPRO_GPMF.get(fourcc)
}

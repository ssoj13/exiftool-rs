//! ICC Profile tag definitions.

use phf::phf_map;

/// Tag signature to human-readable name mapping.
static TAG_NAMES: phf::Map<&'static str, &'static str> = phf_map! {
    // Standard tags
    "A2B0" => "AToB0",
    "A2B1" => "AToB1",
    "A2B2" => "AToB2",
    "A2B3" => "AToB3",
    "B2A0" => "BToA0",
    "B2A1" => "BToA1",
    "B2A2" => "BToA2",
    "B2A3" => "BToA3",
    "B2D0" => "BToD0",
    "B2D1" => "BToD1",
    "B2D2" => "BToD2",
    "B2D3" => "BToD3",
    "D2B0" => "DToB0",
    "D2B1" => "DToB1",
    "D2B2" => "DToB2",
    "D2B3" => "DToB3",
    "bXYZ" => "BlueMatrixColumn",
    "bTRC" => "BlueTRC",
    "calt" => "CalibrationDateTime",
    "targ" => "CharTarget",
    "chad" => "ChromaticAdaptation",
    "chrm" => "Chromaticity",
    "cicp" => "CodingIndependentCodePoints",
    "ciis" => "ColorimetricIntentImageState",
    "clro" => "ColorantOrder",
    "clrt" => "ColorantTable",
    "clot" => "ColorantTableOut",
    "cprt" => "ProfileCopyright",
    "crdi" => "CRDInfo",
    "desc" => "ProfileDescription",
    "dmnd" => "DeviceMfgDesc",
    "dmdd" => "DeviceModelDesc",
    "devs" => "DeviceSettings",
    "gamt" => "Gamut",
    "kTRC" => "GrayTRC",
    "gXYZ" => "GreenMatrixColumn",
    "gTRC" => "GreenTRC",
    "lumi" => "Luminance",
    "meas" => "Measurement",
    "bkpt" => "MediaBlackPoint",
    "wtpt" => "MediaWhitePoint",
    "ncol" => "NamedColor",
    "ncl2" => "NamedColor2",
    "resp" => "OutputResponse",
    "pre0" => "Preview0",
    "pre1" => "Preview1",
    "pre2" => "Preview2",
    "pseq" => "ProfileSequenceDesc",
    "psid" => "ProfileSequenceIdentifier",
    "psd0" => "PostScript2CRD0",
    "psd1" => "PostScript2CRD1",
    "psd2" => "PostScript2CRD2",
    "psd3" => "PostScript2CRD3",
    "ps2s" => "PostScript2CSA",
    "ps2i" => "PS2RenderingIntent",
    "rXYZ" => "RedMatrixColumn",
    "rTRC" => "RedTRC",
    "rig0" => "PerceptualRenderingIntentGamut",
    "rig2" => "SaturationRenderingIntentGamut",
    "scrd" => "ScreeningDesc",
    "scrn" => "Screening",
    "tech" => "Technology",
    "bfd " => "UCRBG",
    "vued" => "ViewingCondDesc",
    "view" => "ViewingConditions",
    "meta" => "Metadata",

    // ColorSync custom tags
    "psvm" => "PS2CRDVMSize",
    "vcgt" => "VideoCardGamma",
    "mmod" => "MakeAndModel",
    "dscm" => "ProfileDescriptionML",
    "ndin" => "NativeDisplayInfo",

    // Microsoft custom
    "MS00" => "WCSProfiles",

    // ICC v5 additions
    "A2M0" => "AToM0",
    "bcp0" => "BRDFColorimetricParam0",
    "bcp1" => "BRDFColorimetricParam1",
    "bcp2" => "BRDFColorimetricParam2",
    "bcp3" => "BRDFColorimetricParam3",
    "bsp0" => "BRDFSpectralParam0",
    "bsp1" => "BRDFSpectralParam1",
    "bsp2" => "BRDFSpectralParam2",
    "bsp3" => "BRDFSpectralParam3",
    "cept" => "ColorEncodingParams",
    "csnm" => "ColorSpaceName",
    "cloo" => "ColorantOrderOut",
    "clio" => "ColorantInfoOut",
    "c2sp" => "CustomToStandardPcc",
    "CxF " => "CXF",
    "gdb0" => "GamutBoundaryDescription0",
    "gdb1" => "GamutBoundaryDescription1",
    "gdb2" => "GamutBoundaryDescription2",
    "gdb3" => "GamutBoundaryDescription3",
    "mdv " => "MultiplexDefaultValues",
    "mcta" => "MultiplexTypeArray",
    "minf" => "MeasurementInfo",
    "miin" => "MeasurementInputInfo",
    "M2A0" => "MToA0",
    "M2B0" => "MToB0",
    "M2B1" => "MToB1",
    "M2B2" => "MToB2",
    "M2B3" => "MToB3",
    "M2S0" => "MToS0",
    "M2S1" => "MToS1",
    "M2S2" => "MToS2",
    "M2S3" => "MToS3",
    "psin" => "ProfileSequenceInfo",
    "rfnm" => "ReferenceName",
    "scoe" => "SceneColorimetryEstimates",
    "sape" => "SceneAppearanceEstimates",
    "fpce" => "FocalPlaneColorimetryEstimates",
    "rhoc" => "ReflectionHardcopyOrigColorimetry",
    "rpoc" => "ReflectionPrintOutputColorimetry",
    "svcn" => "SpectralViewingConditions",
    "swpt" => "SpectralWhitePoint",
    "s2cp" => "StandardToCustomPcc",
    "smap" => "SurfaceMap",
    "hdgm" => "HDGainMapInfo",
};

/// Get human-readable tag name from signature.
pub fn tag_name(sig: &str) -> &str {
    TAG_NAMES.get(sig).copied().unwrap_or(sig)
}

/// Technology type values.
#[allow(dead_code)]
pub fn technology_name(sig: &str) -> &'static str {
    match sig {
        "fscn" => "Film Scanner",
        "dcam" => "Digital Camera",
        "rscn" => "Reflective Scanner",
        "ijet" => "Ink Jet Printer",
        "twax" => "Thermal Wax Printer",
        "epho" => "Electrophotographic Printer",
        "esta" => "Electrostatic Printer",
        "dsub" => "Dye Sublimation Printer",
        "rpho" => "Photographic Paper Printer",
        "fprn" => "Film Writer",
        "vidm" => "Video Monitor",
        "vidc" => "Video Camera",
        "pjtv" => "Projection Television",
        "CRT " => "Cathode Ray Tube Display",
        "PMD " => "Passive Matrix Display",
        "AMD " => "Active Matrix Display",
        "KPCD" => "Photo CD",
        "imgs" => "Photo Image Setter",
        "grav" => "Gravure",
        "offs" => "Offset Lithography",
        "silk" => "Silkscreen",
        "flex" => "Flexography",
        "mpfs" => "Motion Picture Film Scanner",
        "mpfr" => "Motion Picture Film Recorder",
        "dmpc" => "Digital Motion Picture Camera",
        "dcpj" => "Digital Cinema Projector",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_names() {
        assert_eq!(tag_name("desc"), "ProfileDescription");
        assert_eq!(tag_name("cprt"), "ProfileCopyright");
        assert_eq!(tag_name("rTRC"), "RedTRC");
        assert_eq!(tag_name("unknown"), "unknown");
    }

    #[test]
    fn test_technology() {
        assert_eq!(technology_name("dcam"), "Digital Camera");
        assert_eq!(technology_name("vidm"), "Video Monitor");
    }
}

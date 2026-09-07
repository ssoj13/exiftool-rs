//! TIFF-family file type classification (ExifTool `%fileTypeLookup` + DNG override).
//!
//! ExifTool maps many extensions to base type TIFF, then sets `FileType` from the
//! extension. `DNGVersion` (0xC612) overrides to DNG (`ExifTool.pm` ~8763).
//!
//! Unique-magic TIFF variants (CR2, ORF, RW2) keep dedicated parsers and never
//! reach this classifier.

/// FileType for a TIFF-family extension (lower-case, no dot).
/// `None` for generic `tif`/`tiff`/`btf` — those stay TIFF/BigTIFF unless DNG/Make says otherwise.
#[must_use]
pub fn format_from_extension(ext: &str) -> Option<&'static str> {
    Some(match ext.to_ascii_lowercase().as_str() {
        "3fr" => "3FR",
        "arw" | "arq" => "ARW",
        "dcp" => "DCP",
        "dcr" => "DCR",
        "dng" => "DNG",
        "erf" => "ERF",
        "fff" => "FFF",
        "gpr" => "GPR",
        "hdp" | "wdp" | "jxr" => "HDP",
        "iiq" => "IIQ",
        "k25" => "K25",
        "kdc" => "KDC",
        "mef" => "MEF",
        "mos" => "MOS",
        "nef" => "NEF",
        "nrw" => "NRW",
        "pef" => "PEF",
        "rwl" => "RWL",
        "sr2" => "SR2",
        "srf" => "SRF",
        "srw" => "SRW",
        "tif" | "tiff" | "btf" => return None,
        _ => return None,
    })
}

#[must_use]
fn is_generic_tiff_extension(ext: &str) -> bool {
    matches!(ext.to_ascii_lowercase().as_str(), "tif" | "tiff" | "btf")
}

/// Best-effort FileType from IFD0 Make when no extension hint is available.
/// Better than ExifTool on anonymous streams; unused when hint is generic TIFF.
#[must_use]
pub fn format_from_make(make: &str) -> Option<&'static str> {
    let m = make.to_ascii_lowercase();
    if m.contains("nikon") {
        Some("NEF")
    } else if m.contains("sony") {
        Some("ARW")
    } else if m.contains("pentax") {
        Some("PEF")
    } else if m.contains("epson") {
        Some("ERF")
    } else if m.contains("samsung") {
        Some("SRW")
    } else if m.contains("mamiya") {
        Some("MEF")
    } else if m.contains("kodak") {
        Some("DCR")
    } else if m.contains("hasselblad") {
        Some("3FR")
    } else if m.contains("phase one") || m.contains("phaseone") {
        Some("IIQ")
    } else if m.contains("leaf") {
        Some("MOS")
    } else if m.contains("leica") {
        Some("RWL")
    } else {
        None
    }
}

/// Classify TIFF-family `Metadata.format` after IFD0 is parsed.
///
/// Order (ExifTool, then better):
/// 1. `DNGVersion` already set `format` to DNG — keep it.
/// 2. Extension hint if it names a specific RAW/DNG type.
/// 3. Make string (streams / missing extension).
/// 4. Leave TIFF / BigTIFF.
pub fn classify(format: &mut &'static str, ext_hint: Option<&str>, make: Option<&str>) {
    if format.eq_ignore_ascii_case("DNG") {
        return;
    }
    if let Some(ext) = ext_hint {
        if let Some(named) = format_from_extension(ext) {
            *format = named;
            return;
        }
        // ExifTool: .tif/.tiff stay TIFF even if Make is Nikon/Sony/…
        if is_generic_tiff_extension(ext) {
            return;
        }
    }
    if let Some(make) = make {
        if let Some(named) = format_from_make(make) {
            *format = named;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dng_version_wins_over_nef_hint() {
        let mut fmt: &'static str = "DNG";
        classify(&mut fmt, Some("nef"), Some("NIKON CORPORATION"));
        assert_eq!(fmt, "DNG");
    }

    #[test]
    fn extension_nef() {
        let mut fmt: &'static str = "TIFF";
        classify(&mut fmt, Some("NEF"), Some("Canon"));
        assert_eq!(fmt, "NEF");
    }

    #[test]
    fn generic_tif_plus_nikon_stays_tiff() {
        let mut fmt: &'static str = "TIFF";
        classify(&mut fmt, Some("tif"), Some("NIKON CORPORATION"));
        assert_eq!(fmt, "TIFF");
    }

    #[test]
    fn no_hint_nikon_is_nef() {
        let mut fmt: &'static str = "TIFF";
        classify(&mut fmt, None, Some("NIKON CORPORATION"));
        assert_eq!(fmt, "NEF");
    }

    #[test]
    fn extension_map_matches_exiftool_tiff_bases() {
        assert_eq!(format_from_extension("arw"), Some("ARW"));
        assert_eq!(format_from_extension("dng"), Some("DNG"));
        assert_eq!(format_from_extension("pef"), Some("PEF"));
        assert_eq!(format_from_extension("tiff"), None);
    }
}

//! Write metadata to images (-t, --shift, --geotag, --icc, --tagsFromFile).

use std::fs::File;
use std::io::BufReader;
use anyhow::{Context, Result};
use exiftool_attrs::AttrValue;
use exiftool_formats::{
    build_xmp_string, ArwWriter, Cr2Writer, DcrWriter, ErfWriter, ExrWriter, FffWriter, FlacWriter,
    FormatRegistry, GifWriter, HdrWriter, HeicWriter, Id3Writer, IiqWriter, JpegWriter, JxlWriter,
    MefWriter, Mp4Writer, MosWriter, NefWriter, OrfWriter, PefWriter, PngWriter, PnmWriter,
    RafWriter, Rw2Writer, RwlWriter, SrwWriter, TiffWriter, WavWriter, WebpWriter,
};

use crate::args::Args;
use crate::datetime;

/// Write metadata to image files.
pub fn write_image(args: &Args, registry: &FormatRegistry) -> Result<()> {
    if args.files.is_empty() {
        anyhow::bail!("No input file specified for write operation.\n\nUsage: exif -t Tag=Value <FILE>");
    }

    for path in &args.files {
        if args.verbose >= 1 {
            eprintln!("Processing: {}", path.display());
        }
        let file = File::open(path)
            .with_context(|| format!("Cannot open: {}", path.display()))?;
        let mut reader = BufReader::new(file);
        let mut metadata = registry
            .parse(&mut reader)
            .with_context(|| format!("Cannot parse: {}", path.display()))?;

        if let Some(offset) = args.shift {
            datetime::apply_time_shift(&mut metadata, offset);
        }

        if let Some(ref gpx_path) = args.geotag {
            match crate::geotag::GpxTrack::from_file(gpx_path) {
                Ok(track) => {
                    let timestamp = metadata
                        .exif
                        .get("DateTimeOriginal")
                        .or_else(|| metadata.exif.get("CreateDate"))
                        .and_then(|v| v.as_str())
                        .and_then(|s| crate::geotag::parse_exif_datetime(s));
                    if let Some(ts) = timestamp {
                        if let Some((lat, lon, ele)) = track.find_position(ts) {
                            let lat_ref = if lat >= 0.0 { "N" } else { "S" };
                            let lon_ref = if lon >= 0.0 { "E" } else { "W" };
                            metadata.exif.set("GPSLatitude", AttrValue::Double(lat.abs()));
                            metadata.exif.set("GPSLatitudeRef", AttrValue::Str(lat_ref.to_string()));
                            metadata.exif.set("GPSLongitude", AttrValue::Double(lon.abs()));
                            metadata.exif.set("GPSLongitudeRef", AttrValue::Str(lon_ref.to_string()));
                            if let Some(altitude) = ele {
                                let alt_ref = if altitude >= 0.0 { 0u32 } else { 1u32 };
                                metadata.exif.set("GPSAltitude", AttrValue::Double(altitude.abs()));
                                metadata.exif.set("GPSAltitudeRef", AttrValue::UInt(alt_ref));
                            }
                            eprintln!("  Geotagged: {:.6}, {:.6}", lat, lon);
                        } else {
                            eprintln!("  Warning: No GPS position found for photo timestamp");
                        }
                    } else {
                        eprintln!("  Warning: Photo has no DateTimeOriginal for geotagging");
                    }
                }
                Err(e) => eprintln!("Warning: Cannot parse GPX: {}", e),
            }
        }

        if let Some(ref icc_path) = args.icc_profile {
            match std::fs::read(icc_path) {
                Ok(icc_data) => {
                    metadata.icc = Some(icc_data);
                    eprintln!(
                        "  ICC profile: {} ({} bytes)",
                        icc_path.display(),
                        metadata.icc.as_ref().map(|d| d.len()).unwrap_or(0)
                    );
                }
                Err(e) => eprintln!("Warning: Cannot read ICC profile: {}", e),
            }
        }

        if let Some(ref src_path) = args.tags_from_file {
            let src_file = File::open(src_path)
                .with_context(|| format!("Cannot open source: {}", src_path.display()))?;
            let mut src_reader = BufReader::new(src_file);
            match registry.parse(&mut src_reader) {
                Ok(src_meta) => {
                    let mut copied = 0;
                    for (tag, value) in src_meta.exif.iter() {
                        if tag.starts_with('_') || tag == "ThumbnailImage" || tag == "PreviewImage" {
                            continue;
                        }
                        if !args.copy_tags.is_empty()
                            && !args.copy_tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
                        {
                            continue;
                        }
                        metadata.exif.set(tag, value.clone());
                        copied += 1;
                    }
                    if src_meta.xmp.is_some()
                        && (args.copy_tags.is_empty()
                            || args.copy_tags.iter().any(|t| t.eq_ignore_ascii_case("XMP")))
                    {
                        metadata.xmp = src_meta.xmp.clone();
                    }
                    if src_meta.icc.is_some()
                        && (args.copy_tags.is_empty()
                            || args.copy_tags.iter().any(|t| t.eq_ignore_ascii_case("ICC")))
                    {
                        metadata.icc = src_meta.icc.clone();
                    }
                    eprintln!("  Copied {} tags from {}", copied, src_path.display());
                }
                Err(e) => eprintln!("Warning: Cannot parse source file: {}", e),
            }
        }

        for (tag, value) in &args.tags {
            metadata.exif.set(tag, AttrValue::Str(value.clone()));
        }

        let output_path = if let Some(ref out) = args.write_file {
            out.clone()
        } else if args.inplace {
            path.clone()
        } else {
            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
            let ext = path.extension().unwrap_or_default().to_string_lossy();
            path.with_file_name(format!("{}_modified.{}", stem, ext))
        };

        if !metadata.is_writable() {
            let reason = if metadata.is_camera_raw() {
                let make = metadata.exif.get_str("Make").unwrap_or(metadata.format);
                format!("Camera RAW file ({}) is read-only", make.trim())
            } else {
                format!("Format {} does not support writing", metadata.format)
            };
            anyhow::bail!(
                "{}.\n\nWritable formats: JPEG, PNG, TIFF, DNG, WebP, HEIC, EXR, HDR, MP4, MOV, M4V, MP3, FLAC, PNM, GIF, WAV, JXL, CR2, ARW, ORF, NEF, RAF, RW2, PEF, SRW, RWL, 3FR, FFF, ERF, MEF, DCR, KDC, K25, MOS, IIQ",
                reason
            );
        }

        // Generate XMP from attrs when metadata.xmp is unset but we have XMP-prefixed tags
        if metadata.xmp.is_none() {
            if let Ok(Some(xmp)) = build_xmp_string(&metadata) {
                metadata.xmp = Some(xmp);
            }
        }

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let output_data = match metadata.format {
            "JPEG" => {
                let mut out = Vec::new();
                JpegWriter::write_metadata(&mut reader, &mut out, &metadata)?;
                out
            }
            "PNG" => {
                let mut out = Vec::new();
                PngWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "TIFF" | "DNG" => {
                let mut out = Vec::new();
                TiffWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "WebP" => {
                let mut out = Vec::new();
                WebpWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "HEIC" | "HEIF" | "AVIF" => {
                let mut out = Vec::new();
                HeicWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "CR2" => {
                let mut out = Vec::new();
                Cr2Writer::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "ARW" | "SRF" | "SR2" => {
                let mut out = Vec::new();
                ArwWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "ORF" => {
                let mut out = Vec::new();
                OrfWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "NEF" | "NRW" => {
                let mut out = Vec::new();
                NefWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "RW2" => {
                let mut out = Vec::new();
                Rw2Writer::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "PEF" => {
                let mut out = Vec::new();
                PefWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "SRW" => {
                let mut out = Vec::new();
                SrwWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "RWL" => {
                let mut out = Vec::new();
                RwlWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "3FR" | "FFF" => {
                let mut out = Vec::new();
                FffWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "ERF" => {
                let mut out = Vec::new();
                ErfWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "MEF" => {
                let mut out = Vec::new();
                MefWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "DCR" | "KDC" | "K25" => {
                let mut out = Vec::new();
                DcrWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "MOS" => {
                let mut out = Vec::new();
                MosWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "IIQ" => {
                let mut out = Vec::new();
                IiqWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "RAF" => {
                let mut out = std::io::Cursor::new(Vec::new());
                RafWriter::write(&mut reader, &mut out, &metadata)?;
                out.into_inner()
            }
            "HDR" => {
                let mut out = Vec::new();
                HdrWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "EXR" => {
                let mut out = Vec::new();
                ExrWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "MP3" => {
                let mut out = Vec::new();
                Id3Writer::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "FLAC" => {
                let mut out = Vec::new();
                FlacWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "PBM" | "PGM" | "PPM" | "PAM" => {
                let mut out = Vec::new();
                PnmWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "GIF" => {
                let mut out = Vec::new();
                GifWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "WAV" => {
                let mut out = Vec::new();
                WavWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "JXL" => {
                let mut out = Vec::new();
                JxlWriter::write(&mut reader, &mut out, &metadata)?;
                out
            }
            "MP4" | "MOV" | "M4V" | "M4A" | "M4B" | "M4P" | "3GP" | "3G2" | "F4V" => {
                let mut out = Vec::new();
                Mp4Writer::write(&mut reader, &mut out, &metadata)?;
                out
            }
            fmt => anyhow::bail!("Write not supported for: {}", fmt),
        };

        if args.inplace && output_path == *path {
            let tmp = output_path.with_extension("tmp");
            std::fs::write(&tmp, &output_data)?;
            if args.overwrite_original {
                std::fs::rename(&tmp, &output_path)?;
            } else {
                let ext = path
                    .extension()
                    .map(|e| e.to_string_lossy().to_string())
                    .unwrap_or_default();
                let stem = path.file_stem().unwrap_or_default().to_string_lossy();
                let backup = path.with_file_name(format!(
                    "{}_original.{}",
                    stem,
                    if ext.is_empty() { "bak" } else { &ext }
                ));
                std::fs::copy(path, &backup)?;
                std::fs::rename(&tmp, &output_path)?;
                eprintln!("Original backed up to: {}", backup.display());
            }
        } else {
            std::fs::write(&output_path, &output_data)?;
        }

        eprintln!("Wrote: {} ({} bytes)", output_path.display(), output_data.len());
    }

    Ok(())
}

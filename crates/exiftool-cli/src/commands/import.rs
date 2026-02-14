//! Import tags from JSON/CSV (--json=, --csv=).

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use exiftool_attrs::AttrValue;
use exiftool_formats::{
    build_xmp_string, ArwWriter, Cr2Writer, DcrWriter, ErfWriter, ExrWriter, FffWriter, FlacWriter,
    FormatRegistry, GifWriter, HdrWriter, HeicWriter, Id3Writer, IiqWriter, JpegWriter, JxlWriter,
    MefWriter, Mp4Writer, MosWriter, NefWriter, OrfWriter, PefWriter, PngWriter, PnmWriter,
    RafWriter, Rw2Writer, RwlWriter, SrwWriter, TiffWriter, WavWriter, WebpWriter,
};

use crate::args::Args;

/// Import tags from JSON file.
pub fn import_from_json(args: &Args, registry: &FormatRegistry) -> Result<()> {
    let json_path = args.json_import.as_ref().unwrap();
    let json_str = std::fs::read_to_string(json_path)
        .with_context(|| format!("Cannot read: {}", json_path.display()))?;

    let json: serde_json::Value =
        serde_json::from_str(&json_str)
            .with_context(|| format!("Invalid JSON in: {}", json_path.display()))?;

    let obj = json
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("JSON must be an object"))?;

    let is_file_keyed = obj
        .values()
        .next()
        .map(|v| v.is_object())
        .unwrap_or(false);

    if is_file_keyed {
        for (file_path, tags_val) in obj {
            let path = PathBuf::from(file_path);
            if !path.exists() {
                eprintln!("Warning: File not found: {}", file_path);
                continue;
            }
            let tags = tags_val
                .as_object()
                .ok_or_else(|| anyhow::anyhow!("Tags for {} must be an object", file_path))?;
            write_tags_to_file(&path, tags, args, registry)?;
        }
    } else {
        if args.files.is_empty() {
            anyhow::bail!("No target files specified. Use: exif --json=tags.json -p photo.jpg");
        }
        for path in &args.files {
            write_tags_to_file(path, obj, args, registry)?;
        }
    }
    Ok(())
}

/// Import tags from CSV file.
pub fn import_from_csv(args: &Args, registry: &FormatRegistry) -> Result<()> {
    let csv_path = args.csv_import.as_ref().unwrap();
    let csv_str = std::fs::read_to_string(csv_path)
        .with_context(|| format!("Cannot read: {}", csv_path.display()))?;

    let mut lines = csv_str.lines();
    let header_line = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("CSV file is empty"))?;
    let headers: Vec<&str> = parse_csv_line(header_line);

    let source_col = headers
        .iter()
        .position(|h| h.eq_ignore_ascii_case("SourceFile"))
        .ok_or_else(|| anyhow::anyhow!("CSV must have SourceFile column"))?;

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let values: Vec<&str> = parse_csv_line(line);
        if values.len() <= source_col {
            continue;
        }
        let file_path = values[source_col];
        let path = PathBuf::from(file_path);
        if !path.exists() {
            eprintln!("Warning: File not found: {}", file_path);
            continue;
        }
        let mut tags = serde_json::Map::new();
        for (i, header) in headers.iter().enumerate() {
            if i == source_col || *header == "Format" {
                continue;
            }
            if let Some(value) = values.get(i) {
                if !value.is_empty() {
                    tags.insert(
                        header.to_string(),
                        serde_json::Value::String(value.to_string()),
                    );
                }
            }
        }
        if !tags.is_empty() {
            write_tags_to_file(&path, &tags, args, registry)?;
        }
    }
    Ok(())
}

fn parse_csv_line(line: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;
    let chars: Vec<char> = line.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == ',' && !in_quotes {
            let field = &line[start..i];
            result.push(field.trim().trim_matches('"'));
            start = i + 1;
        }
    }
    if start <= line.len() {
        result.push(line[start..].trim().trim_matches('"'));
    }
    result
}

fn write_tags_to_file(
    path: &Path,
    tags: &serde_json::Map<String, serde_json::Value>,
    args: &Args,
    registry: &FormatRegistry,
) -> Result<()> {
    let file = File::open(path).with_context(|| format!("Cannot open: {}", path.display()))?;
    let mut reader = BufReader::new(file);
    let mut metadata = registry
        .parse(&mut reader)
        .with_context(|| format!("Cannot parse: {}", path.display()))?;

    if !metadata.is_writable() {
        eprintln!("Warning: {} is not writable, skipping", path.display());
        return Ok(());
    }

    for (tag, value) in tags {
        let str_val = match value {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            _ => value.to_string(),
        };
        metadata.exif.set(tag, AttrValue::Str(str_val));
    }

    if metadata.xmp.is_none() {
        if let Ok(Some(xmp)) = build_xmp_string(&metadata) {
            metadata.xmp = Some(xmp);
        }
    }

    let output_path = if args.inplace {
        path.to_path_buf()
    } else if let Some(ref out) = args.write_file {
        out.clone()
    } else {
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let ext = path.extension().unwrap_or_default().to_string_lossy();
        path.with_file_name(format!("{}_modified.{}", stem, ext))
    };

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
        fmt => {
            eprintln!("Warning: Write not supported for {}: {}", fmt, path.display());
            return Ok(());
        }
    };

    if args.inplace && output_path == *path {
        let tmp = output_path.with_extension("tmp");
        std::fs::write(&tmp, &output_data)?;
        std::fs::rename(&tmp, &output_path)?;
    } else {
        std::fs::write(&output_path, &output_data)?;
    }

    eprintln!("Wrote: {} ({} tags)", output_path.display(), tags.len());
    Ok(())
}

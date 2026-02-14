//! Delete metadata command (--delete).

use std::fs::File;
use std::io::BufReader;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use anyhow::{Context, Result};
use exiftool_formats::{
    ArwWriter, Cr2Writer, DcrWriter, ErfWriter, ExrWriter, FffWriter, FlacWriter, FormatRegistry,
    GifWriter, HdrWriter, HeicWriter, Id3Writer, IiqWriter, JpegWriter, JxlWriter, MefWriter,
    MosWriter, Mp4Writer, NefWriter, OrfWriter, PefWriter, PngWriter, PnmWriter, RafWriter,
    Rw2Writer, RwlWriter, SrwWriter, TiffWriter, WavWriter, WebpWriter,
};

use crate::args::Args;
use crate::paths;

/// Remove all metadata from files.
pub fn delete_metadata(args: &Args, registry: &FormatRegistry) -> Result<()> {
    if !args.inplace && args.write_file.is_none() {
        anyhow::bail!("--delete requires -p (in-place) or -w <file> (output file)");
    }

    let files = paths::expand_paths(
        &args.files,
        args.recursive,
        &args.extensions,
        &args.exclude,
        args.newer,
        args.older,
        args.minsize,
        args.maxsize,
    );

    if files.is_empty() {
        anyhow::bail!("No files to process");
    }

    let mut processed = 0;
    let mut errors = 0;

    for path in &files {
        match delete_metadata_single(path, args, registry) {
            Ok(()) => {
                println!("Stripped: {}", path.display());
                processed += 1;
            }
            Err(e) => {
                eprintln!("Error {}: {}", path.display(), e);
                errors += 1;
            }
        }
    }

    eprintln!("Processed {} files, {} errors", processed, errors);
    if errors > 0 {
        std::process::exit(1);
    }
    Ok(())
}

/// Delete metadata from a single file.
fn delete_metadata_single(path: &Path, args: &Args, registry: &FormatRegistry) -> Result<()> {
    let file = File::open(path)
        .with_context(|| format!("Cannot open: {}", path.display()))?;
    let mut reader = BufReader::new(file);
    let mut metadata = registry
        .parse(&mut reader)
        .with_context(|| format!("Cannot parse: {}", path.display()))?;

    if !metadata.is_writable() {
        anyhow::bail!("Format {} does not support writing", metadata.format);
    }

    metadata.exif.clear();
    metadata.xmp = None;
    metadata.icc = None;
    metadata.thumbnail = None;
    metadata.preview = None;

    reader.seek(SeekFrom::Start(0))?;

    let output_data = match metadata.format {
        "RAF" => {
            let mut out = std::io::Cursor::new(Vec::new());
            RafWriter::write(&mut reader, &mut out, &metadata)?;
            out.into_inner()
        }
        _ => {
            let mut out = Vec::new();
            match metadata.format {
                "JPEG" => JpegWriter::write(&mut reader, &mut out, None, None, None)?,
                "PNG" => PngWriter::write(&mut reader, &mut out, &metadata)?,
                "TIFF" | "DNG" => TiffWriter::write(&mut reader, &mut out, &metadata)?,
                "WebP" => WebpWriter::write(&mut reader, &mut out, &metadata)?,
                "HEIC" | "HEIF" | "AVIF" => HeicWriter::write(&mut reader, &mut out, &metadata)?,
                "CR2" => Cr2Writer::write(&mut reader, &mut out, &metadata)?,
                "ARW" | "SRF" | "SR2" => ArwWriter::write(&mut reader, &mut out, &metadata)?,
                "ORF" => OrfWriter::write(&mut reader, &mut out, &metadata)?,
                "NEF" | "NRW" => NefWriter::write(&mut reader, &mut out, &metadata)?,
                "RW2" => Rw2Writer::write(&mut reader, &mut out, &metadata)?,
                "PEF" => PefWriter::write(&mut reader, &mut out, &metadata)?,
                "SRW" => SrwWriter::write(&mut reader, &mut out, &metadata)?,
                "RWL" => RwlWriter::write(&mut reader, &mut out, &metadata)?,
                "3FR" | "FFF" => FffWriter::write(&mut reader, &mut out, &metadata)?,
                "ERF" => ErfWriter::write(&mut reader, &mut out, &metadata)?,
                "MEF" => MefWriter::write(&mut reader, &mut out, &metadata)?,
                "DCR" | "KDC" | "K25" => DcrWriter::write(&mut reader, &mut out, &metadata)?,
                "MOS" => MosWriter::write(&mut reader, &mut out, &metadata)?,
                "IIQ" => IiqWriter::write(&mut reader, &mut out, &metadata)?,
                "EXR" => ExrWriter::write(&mut reader, &mut out, &metadata)?,
                "HDR" => HdrWriter::write(&mut reader, &mut out, &metadata)?,
                "MP3" => Id3Writer::write(&mut reader, &mut out, &metadata)?,
                "FLAC" => FlacWriter::write(&mut reader, &mut out, &metadata)?,
                "PBM" | "PGM" | "PPM" | "PAM" => PnmWriter::write(&mut reader, &mut out, &metadata)?,
                "GIF" => GifWriter::write(&mut reader, &mut out, &metadata)?,
                "WAV" => WavWriter::write(&mut reader, &mut out, &metadata)?,
                "JXL" => JxlWriter::write(&mut reader, &mut out, &metadata)?,
                "MP4" | "MOV" | "M4V" | "M4A" | "M4B" | "M4P" | "3GP" | "3G2" | "F4V" => {
                    Mp4Writer::write(&mut reader, &mut out, &metadata)?
                }
                fmt => anyhow::bail!("Cannot strip metadata from {}", fmt),
            }
            out
        }
    };

    let output_path = args.write_file.as_deref().unwrap_or(path);
    let tmp_path = output_path.with_extension("tmp");
    std::fs::write(&tmp_path, &output_data)?;
    std::fs::rename(&tmp_path, output_path)?;

    Ok(())
}

//! HTML dump showing file structure (-htmlDump).

use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use exiftool_formats::FormatRegistry;

use crate::args::Args;
use crate::paths;

/// Generate HTML dump showing file structure.
pub fn html_dump(args: &Args, registry: &FormatRegistry) -> Result<()> {
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
        anyhow::bail!("No files specified for -htmlDump");
    }

    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<title>File Structure Dump</title>\n");
    html.push_str("<style>\n");
    html.push_str("body { font-family: 'SF Mono', Monaco, Consolas, monospace; margin: 20px; background: #1e1e1e; color: #d4d4d4; }\n");
    html.push_str("h1 { color: #569cd6; }\n");
    html.push_str("h2 { color: #4ec9b0; border-bottom: 1px solid #444; padding-bottom: 5px; }\n");
    html.push_str(".file-info { background: #252526; padding: 15px; border-radius: 5px; margin: 10px 0; }\n");
    html.push_str(".hex-dump { background: #1e1e1e; border: 1px solid #444; padding: 10px; overflow-x: auto; }\n");
    html.push_str(".hex-row { display: flex; }\n");
    html.push_str(".hex-offset { color: #608b4e; width: 80px; }\n");
    html.push_str(".hex-bytes { color: #ce9178; flex: 1; }\n");
    html.push_str(".hex-ascii { color: #9cdcfe; width: 180px; }\n");
    html.push_str(".marker { background: #264f78; padding: 2px 6px; border-radius: 3px; margin: 2px; display: inline-block; }\n");
    html.push_str(".marker-exif { background: #4e7a25; }\n");
    html.push_str(".marker-xmp { background: #7a4e25; }\n");
    html.push_str(".marker-icc { background: #4e257a; }\n");
    html.push_str(".meta-table { width: 100%; border-collapse: collapse; margin: 10px 0; }\n");
    html.push_str(".meta-table th, .meta-table td { padding: 8px; text-align: left; border-bottom: 1px solid #444; }\n");
    html.push_str(".meta-table th { background: #333; color: #569cd6; }\n");
    html.push_str(".section { margin: 20px 0; }\n");
    html.push_str("</style>\n</head>\n<body>\n");
    html.push_str("<h1>File Structure Analysis</h1>\n");

    for path in &files {
        html_dump_single(path, registry, &mut html)?;
    }

    html.push_str("</body>\n</html>\n");

    if let Some(ref output_path) = args.output {
        std::fs::write(output_path, &html)
            .with_context(|| format!("Cannot write: {}", output_path.display()))?;
        eprintln!("Wrote: {}", output_path.display());
    } else {
        print!("{}", html);
    }

    Ok(())
}

fn html_dump_single(path: &Path, registry: &FormatRegistry, html: &mut String) -> Result<()> {
    let file_data = std::fs::read(path)
        .with_context(|| format!("Cannot read: {}", path.display()))?;

    let file_size = file_data.len();
    let _ = writeln!(html, "<div class=\"file-info\">");
    let _ = writeln!(html, "<h2>{}</h2>", escape_html(&path.display().to_string()));
    let _ = writeln!(
        html,
        "<p><strong>Size:</strong> {} bytes ({:.2} KB)</p>",
        file_size,
        file_size as f64 / 1024.0
    );

    let format = detect_format(&file_data);
    let _ = writeln!(html, "<p><strong>Format:</strong> {}</p>", format);

    let _ = writeln!(html, "<div class=\"section\"><h3>Structure</h3>");
    show_structure_markers(&file_data, &format, html);
    let _ = writeln!(html, "</div>");

    let _ = writeln!(html, "<div class=\"section\"><h3>Header (first 256 bytes)</h3>");
    let _ = writeln!(html, "<div class=\"hex-dump\">");
    let preview_len = file_data.len().min(256);
    for offset in (0..preview_len).step_by(16) {
        let end = (offset + 16).min(preview_len);
        let chunk = &file_data[offset..end];

        let _ = write!(html, "<div class=\"hex-row\">");
        let _ = write!(html, "<span class=\"hex-offset\">{:08X}</span>", offset);

        let _ = write!(html, "<span class=\"hex-bytes\">");
        for (i, b) in chunk.iter().enumerate() {
            if i == 8 {
                let _ = write!(html, " ");
            }
            let _ = write!(html, "{:02X} ", b);
        }
        for i in chunk.len()..16 {
            if i == 8 {
                let _ = write!(html, " ");
            }
            let _ = write!(html, "   ");
        }
        let _ = write!(html, "</span>");

        let _ = write!(html, "<span class=\"hex-ascii\">");
        for b in chunk {
            let c = if *b >= 0x20 && *b < 0x7F {
                *b as char
            } else {
                '.'
            };
            let _ = write!(html, "{}", escape_html(&c.to_string()));
        }
        let _ = writeln!(html, "</span></div>");
    }
    let _ = writeln!(html, "</div></div>");

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    if let Ok(metadata) = registry.parse(&mut reader) {
        let _ = writeln!(
            html,
            "<div class=\"section\"><h3>Metadata Summary ({} tags)</h3>",
            metadata.exif.len()
        );
        let _ = writeln!(html, "<table class=\"meta-table\">");
        let _ = writeln!(html, "<tr><th>Tag</th><th>Value</th></tr>");
        let important = [
            "Make",
            "Model",
            "DateTimeOriginal",
            "ISO",
            "ExposureTime",
            "FNumber",
            "FocalLength",
            "ImageWidth",
            "ImageHeight",
            "Software",
        ];
        for tag in &important {
            if let Some(v) = metadata.exif.get(*tag) {
                let _ = writeln!(
                    html,
                    "<tr><td>{}</td><td>{}</td></tr>",
                    tag,
                    escape_html(&v.to_string())
                );
            }
        }
        let _ = writeln!(html, "</table></div>");
    }

    let _ = writeln!(html, "</div>");
    Ok(())
}

fn detect_format(data: &[u8]) -> &'static str {
    if data.len() < 4 {
        return "Unknown";
    }
    match &data[0..4] {
        [0xFF, 0xD8, 0xFF, _] => "JPEG",
        [0x89, 0x50, 0x4E, 0x47] => "PNG",
        [0x49, 0x49, 0x2A, 0x00] => "TIFF (Little Endian)",
        [0x4D, 0x4D, 0x00, 0x2A] => "TIFF (Big Endian)",
        [0x52, 0x49, 0x46, 0x46] => {
            if data.len() >= 12 && &data[8..12] == b"WEBP" {
                "WebP"
            } else if data.len() >= 12 && &data[8..12] == b"AVI " {
                "AVI"
            } else {
                "RIFF"
            }
        }
        _ => {
            if data.len() >= 8 && &data[4..8] == b"ftyp" {
                if data.len() >= 12 {
                    let brand = &data[8..12];
                    if brand == b"heic" || brand == b"heix" || brand == b"mif1" {
                        "HEIC/HEIF"
                    } else if brand == b"avif" {
                        "AVIF"
                    } else if brand == b"mp41" || brand == b"mp42" || brand == b"isom" {
                        "MP4"
                    } else if brand == b"qt  " {
                        "QuickTime MOV"
                    } else {
                        "ISOBMFF"
                    }
                } else {
                    "ISOBMFF"
                }
            } else if data.len() >= 4 && &data[0..4] == b"fLaC" {
                "FLAC"
            } else if data.len() >= 3 && &data[0..3] == b"ID3" {
                "MP3 (ID3)"
            } else if data.len() >= 4 && &data[0..4] == [0x76, 0x2F, 0x31, 0x01] {
                "OpenEXR"
            } else if data.len() >= 10 && &data[0..10] == b"#?RADIANCE" {
                "Radiance HDR"
            } else {
                "Unknown"
            }
        }
    }
}

fn show_structure_markers(data: &[u8], format: &str, html: &mut String) {
    match format {
        "JPEG" => {
            let _ = write!(html, "<p>Markers: ");
            let mut i = 0;
            while i < data.len() - 1 {
                if data[i] == 0xFF {
                    let marker = data[i + 1];
                    let name = jpeg_marker_name(marker);
                    let class = if name.contains("EXIF") {
                        "marker marker-exif"
                    } else if name.contains("XMP") {
                        "marker marker-xmp"
                    } else if name.contains("ICC") {
                        "marker marker-icc"
                    } else {
                        "marker"
                    };
                    let _ = write!(
                        html,
                        "<span class=\"{}\">{} ({:02X})</span> ",
                        class, name, marker
                    );
                    if (marker >= 0xE0 && marker <= 0xEF)
                        || marker == 0xFE
                        || marker == 0xDB
                        || marker == 0xC4
                    {
                        if i + 3 < data.len() {
                            let len = ((data[i + 2] as usize) << 8) | (data[i + 3] as usize);
                            i += 2 + len;
                            continue;
                        }
                    }
                    if marker == 0xD8 || marker == 0xD9 {
                        i += 2;
                        continue;
                    }
                }
                i += 1;
            }
            let _ = writeln!(html, "</p>");
        }
        "PNG" => {
            let _ = write!(html, "<p>Chunks: ");
            let mut i = 8;
            while i + 8 <= data.len() {
                let len =
                    u32::from_be_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]) as usize;
                let chunk_type = std::str::from_utf8(&data[i + 4..i + 8]).unwrap_or("????");
                let class = if chunk_type == "eXIf" || chunk_type == "tEXt" || chunk_type == "iTXt"
                {
                    "marker marker-exif"
                } else if chunk_type == "iCCP" {
                    "marker marker-icc"
                } else {
                    "marker"
                };
                let _ = write!(
                    html,
                    "<span class=\"{}\">{} ({}B)</span> ",
                    class, chunk_type, len
                );
                i += 12 + len;
            }
            let _ = writeln!(html, "</p>");
        }
        _ => {
            let _ = writeln!(
                html,
                "<p>Structure visualization not available for this format.</p>"
            );
        }
    }
}

fn jpeg_marker_name(marker: u8) -> &'static str {
    match marker {
        0xD8 => "SOI",
        0xD9 => "EOI",
        0xE0 => "APP0/JFIF",
        0xE1 => "APP1/EXIF",
        0xE2 => "APP2/ICC",
        0xED => "APP13/IPTC",
        0xEE => "APP14",
        0xDB => "DQT",
        0xC0 => "SOF0",
        0xC2 => "SOF2",
        0xC4 => "DHT",
        0xDA => "SOS",
        0xFE => "COM",
        _ => "???",
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

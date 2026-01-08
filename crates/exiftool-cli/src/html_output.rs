//! HTML output format for metadata.

use exiftool_formats::Metadata;
use std::fmt::Write;
use std::path::Path;

/// Print metadata as HTML table to stdout.
pub fn print_html(path: &Path, m: &Metadata, filter: &[String]) {
    let mut output = String::new();
    format_html(path, m, filter, &mut output);
    print!("{}", output);
}

/// Format metadata as HTML table.
pub fn format_html(path: &Path, m: &Metadata, filter: &[String], out: &mut String) {
    let _ = writeln!(out, "<!DOCTYPE html>");
    let _ = writeln!(out, "<html>");
    let _ = writeln!(out, "<head>");
    let _ = writeln!(out, "  <meta charset=\"UTF-8\">");
    let _ = writeln!(out, "  <title>Metadata: {}</title>", escape_html(&path.display().to_string()));
    let _ = writeln!(out, "  <style>");
    let _ = writeln!(out, "    body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 20px; }}");
    let _ = writeln!(out, "    h2 {{ color: #333; border-bottom: 2px solid #4a90d9; padding-bottom: 8px; }}");
    let _ = writeln!(out, "    table {{ border-collapse: collapse; width: 100%; max-width: 900px; }}");
    let _ = writeln!(out, "    th, td {{ padding: 8px 12px; text-align: left; border-bottom: 1px solid #ddd; }}");
    let _ = writeln!(out, "    th {{ background: #f5f5f5; font-weight: 600; width: 30%; }}");
    let _ = writeln!(out, "    tr:hover {{ background: #f9f9f9; }}");
    let _ = writeln!(out, "    .format {{ color: #666; font-size: 0.9em; }}");
    let _ = writeln!(out, "    .value {{ word-break: break-word; }}");
    let _ = writeln!(out, "  </style>");
    let _ = writeln!(out, "</head>");
    let _ = writeln!(out, "<body>");
    let _ = writeln!(out, "  <h2>{}</h2>", escape_html(&path.display().to_string()));
    let _ = writeln!(out, "  <p class=\"format\">Format: {}</p>", m.format);
    let _ = writeln!(out, "  <table>");
    let _ = writeln!(out, "    <tr><th>Tag</th><th>Value</th></tr>");
    
    // Collect and sort entries
    let mut entries: Vec<_> = m.exif.iter()
        .filter(|(k, _)| filter.is_empty() || filter.iter().any(|f| {
            if f.contains('*') || f.contains('?') {
                crate::glob_match(f, k)
            } else {
                f.eq_ignore_ascii_case(k)
            }
        }))
        .collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));
    
    for (k, v) in entries {
        let _ = writeln!(out, "    <tr><th>{}</th><td class=\"value\">{}</td></tr>", 
            escape_html(k), escape_html(&v.to_string()));
    }
    
    // Additional info
    if let Some(ref xmp) = m.xmp {
        let _ = writeln!(out, "    <tr><th>XMP</th><td>{} bytes</td></tr>", xmp.len());
    }
    if let Some(ref thumb) = m.thumbnail {
        let _ = writeln!(out, "    <tr><th>Thumbnail</th><td>{} bytes</td></tr>", thumb.len());
    }
    if let Some(ref preview) = m.preview {
        let _ = writeln!(out, "    <tr><th>Preview</th><td>{} bytes</td></tr>", preview.len());
    }
    
    let _ = writeln!(out, "  </table>");
    let _ = writeln!(out, "</body>");
    let _ = writeln!(out, "</html>");
}

/// Escape HTML special characters.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

//! Tag filter utilities (glob matching, wildcard expansion).
//!
//! # Why
//!
//! -g/--get accepts patterns with * and ?. We need to match tags and expand wildcards
//! against actual metadata keys.
//!
//! # Where used
//!
//! - `output.rs` / `print_*` — filter which tags to display
//! - `output_csv_unified` — expand filters for CSV columns
//! - `xml_output`, `html_output` — tag filtering

use exiftool_formats::Metadata;

/// Simple glob matching for tag names (* = any chars, ? = one char).
pub fn glob_match(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    glob_match_impl(&p, &t)
}

fn glob_match_impl(p: &[char], t: &[char]) -> bool {
    match (p.first(), t.first()) {
        (None, None) => true,
        (Some('*'), _) => {
            glob_match_impl(&p[1..], t) || (!t.is_empty() && glob_match_impl(p, &t[1..]))
        }
        (Some('?'), Some(_)) => glob_match_impl(&p[1..], &t[1..]),
        (Some(pc), Some(tc)) if pc.eq_ignore_ascii_case(tc) => glob_match_impl(&p[1..], &t[1..]),
        _ => false,
    }
}

/// Check if tag matches any of the filter patterns.
pub fn tag_matches(tag: &str, filters: &[String]) -> bool {
    if filters.is_empty() {
        return true;
    }
    filters.iter().any(|f| {
        if f.contains('*') || f.contains('?') {
            glob_match(f, tag)
        } else {
            f.eq_ignore_ascii_case(tag)
        }
    })
}

/// Check if filter is a simple tag name (no wildcards).
pub fn is_simple_filter(filters: &[String]) -> bool {
    filters.len() == 1 && !filters[0].contains('*') && !filters[0].contains('?')
}

/// Check if any filter has wildcards.
pub fn has_wildcards(filters: &[String]) -> bool {
    filters.iter().any(|f| f.contains('*') || f.contains('?'))
}

/// Expand wildcard patterns to actual tag names from metadata.
pub fn expand_filters(filters: &[String], metadata: &Metadata) -> Vec<String> {
    if filters.is_empty() {
        return vec![];
    }
    if !has_wildcards(filters) {
        return filters.to_vec();
    }

    let mut result = Vec::new();
    for (tag, _) in metadata.exif.iter() {
        if tag_matches(tag, filters) && !result.contains(tag) {
            result.push(tag.clone());
        }
    }
    result.sort();
    result
}

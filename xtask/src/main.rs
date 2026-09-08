//! xtask - code generation for exiftool-rs.
//!
//! Commands:
//!   cargo xtask codegen       - generate tag tables from ExifTool Perl
//!   cargo xtask dump          - dump ExifTool tags to JSON

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Sanitize identifier for Rust (replace invalid chars with underscore)
fn sanitize_ident(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Code generation tasks for exiftool-rs")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate Rust code from ExifTool Perl sources
    Codegen {
        /// Output directory for generated code
        #[arg(short, long, default_value = "crates/exiftool-tags/src/generated")]
        output: PathBuf,
    },

    /// Dump ExifTool tags to JSON
    Dump {
        /// Output JSON file
        #[arg(short, long, default_value = "xtask/tags.json")]
        output: PathBuf,
    },
}

// JSON structure from dump_tags.pl
#[derive(Debug, Deserialize, Serialize)]
struct TagTable {
    format: Option<String>,
    first_entry: Option<i32>,
    tags: Option<BTreeMap<String, TagInfo>>,
    #[serde(default)]
    blobs: Option<Vec<SubdirBlob>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SubdirBlob {
    index: u16,
    name: String,
    format: Option<String>,
    sub_table: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TagInfo {
    name: String,
    #[serde(default)]
    format: Option<serde_json::Value>, // Can be string or number
    #[serde(default)]
    mask: Option<u64>,
    #[serde(default)]
    values: Option<BTreeMap<String, serde_json::Value>>,
    sub_table: Option<String>,
    group: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Codegen { output } => codegen(&output)?,
        Commands::Dump { output } => dump_tags(&output)?,
    }

    Ok(())
}

fn codegen(output: &PathBuf) -> Result<()> {
    // Ensure tags.json exists
    let json_path = PathBuf::from("xtask/tags.json");
    if !json_path.exists() {
        println!("tags.json not found, running dump first...");
        dump_tags(&json_path)?;
    }

    println!("Generating code from {:?} to {:?}", json_path, output);

    // Create output directory
    fs::create_dir_all(output)?;

    // Read tags.json
    let json_content = fs::read_to_string(&json_path).context("Failed to read tags.json")?;

    let data: BTreeMap<String, BTreeMap<String, TagTable>> =
        serde_json::from_str(&json_content).context("Failed to parse tags.json")?;

    // Generate module files
    generate_mod_rs(&data, output)?;
    generate_vendor_modules(&data, output)?;

    println!("Code generation complete!");
    Ok(())
}

fn dump_tags(output: &PathBuf) -> Result<()> {
    let script = PathBuf::from("xtask/dump_tags.pl");

    if !script.exists() {
        anyhow::bail!("dump_tags.pl not found at {:?}", script);
    }

    println!("Running Perl dumper...");
    let result = Command::new("perl")
        .arg(&script)
        .output()
        .context("Failed to run perl")?;

    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        anyhow::bail!("Perl script failed: {}", stderr);
    }

    fs::write(output, &result.stdout).context("Failed to write output")?;

    println!("Dumped tags to {:?}", output);
    Ok(())
}

fn generate_mod_rs(
    data: &BTreeMap<String, BTreeMap<String, TagTable>>,
    output: &std::path::Path,
) -> Result<()> {
    let mut code = String::from(
        r#"//! Auto-generated tag definitions from ExifTool.
//! DO NOT EDIT MANUALLY - regenerate with: cargo xtask codegen

"#,
    );

    // Module declarations (dump vendors + hand-maintained extras)
    let mut mods: Vec<String> = data.keys().map(|v| v.to_lowercase()).collect();
    for extra in ["dji", "gopro"] {
        if !mods.iter().any(|m| m == extra) {
            mods.push(extra.to_string());
        }
    }
    mods.sort();
    for mod_name in &mods {
        code.push_str(&format!("pub mod {};\n", mod_name));
    }

    // No prelude - use vendor::TABLE_NAME directly to avoid collisions

    let out_file = output.join("mod.rs");
    fs::write(&out_file, code)?;
    println!("Generated {:?}", out_file);
    Ok(())
}

fn generate_vendor_modules(
    data: &BTreeMap<String, BTreeMap<String, TagTable>>,
    output: &std::path::Path,
) -> Result<()> {
    for (vendor, tables) in data {
        let mut code = format!(
            r#"//! {} MakerNotes tag definitions.
//! Auto-generated from ExifTool - DO NOT EDIT

#![allow(dead_code)]

/// Tag definition with name and optional value mappings.
#[derive(Debug, Clone)]
pub struct TagDef {{
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}}

/// Bit-sliced ProcessBinaryData tag (ExifTool `0.1` index + Mask).
#[derive(Debug, Clone)]
pub struct MaskDef {{
    pub index: u16,
    pub mask: u32,
    pub name: &'static str,
    pub values: Option<&'static [(i64, &'static str)]>,
}}

"#,
            vendor
        );

        // Track generated constants to avoid duplicates
        let mut generated_constants: HashSet<String> = HashSet::new();
        let mut tables_with_masks: HashSet<String> = HashSet::new();

        // Generate tag tables
        for (table_name, table) in tables {
            if let Some(tags) = &table.tags {
                let safe_name = sanitize_ident(&table_name.replace("::", "_")).to_uppercase();

                code.push_str(&format!("/// {} tags\n", table_name));
                code.push_str(&format!(
                    "pub static {}: phf::Map<u16, TagDef> = phf::phf_map! {{\n",
                    safe_name
                ));

                let mut mask_rows: Vec<(u16, u32, String, String)> = Vec::new();
                for (id, info) in tags {
                    let values_ref = if let Some(values) = &info.values {
                        if !values.is_empty() {
                            let values_name = format!(
                                "{}_{}_VALUES",
                                safe_name,
                                sanitize_ident(&info.name).to_uppercase()
                            );
                            format!("Some({})", values_name)
                        } else {
                            "None".to_string()
                        }
                    } else {
                        "None".to_string()
                    };

                    if id.contains('.') {
                        let index: i64 = id.split('.').next().unwrap_or("0").parse().unwrap_or(0);
                        if !(0..=0xFFFF).contains(&index) {
                            continue;
                        }
                        let mask = info.mask.unwrap_or(0) as u32;
                        mask_rows.push((index as u16, mask, info.name.clone(), values_ref));
                        continue;
                    }

                    let tag_id: i64 = id.parse().unwrap_or(0);
                    if !(0..=0xFFFF).contains(&tag_id) {
                        continue;
                    }

                    if let Some(mask) = info.mask {
                        mask_rows.push((
                            tag_id as u16,
                            mask as u32,
                            info.name.clone(),
                            values_ref.clone(),
                        ));
                    }

                    code.push_str(&format!(
                        "    {}u16 => TagDef {{ name: \"{}\", values: {} }},\n",
                        tag_id, info.name, values_ref
                    ));
                }
                code.push_str("};\n\n");

                if !mask_rows.is_empty() {
                    tables_with_masks.insert(safe_name.clone());
                    code.push_str(&format!(
                        "/// {} Mask bitfields (ExifTool 0.1-style indices)\n",
                        table_name
                    ));
                    code.push_str(&format!(
                        "pub static {}_MASKS: &[MaskDef] = &[\n",
                        safe_name
                    ));
                    for (index, mask, name, values_ref) in &mask_rows {
                        code.push_str(&format!(
                            "    MaskDef {{ index: {}, mask: {:#x}, name: \"{}\", values: {} }},\n",
                            index, mask, name, values_ref
                        ));
                    }
                    code.push_str("];\n\n");
                }

                // Generate value mappings (avoiding duplicates)
                for info in tags.values() {
                    if let Some(values) = &info.values {
                        if !values.is_empty() {
                            let safe_tag_name = sanitize_ident(&info.name).to_uppercase();
                            let values_name = format!("{}_{}_VALUES", safe_name, safe_tag_name);

                            // Skip if already generated
                            if !generated_constants.insert(values_name.clone()) {
                                continue;
                            }

                            code.push_str(&format!(
                                "pub static {}: &[(i64, &str)] = &[\n",
                                values_name
                            ));
                            for (k, v) in values {
                                let val: i64 = k.parse().unwrap_or(0);
                                // Convert value to string and escape
                                let v_str = match v {
                                    serde_json::Value::String(s) => s.clone(),
                                    serde_json::Value::Number(n) => n.to_string(),
                                    _ => continue,
                                };
                                let escaped = v_str.replace('\\', "\\\\").replace('"', "\\\"");
                                code.push_str(&format!("    ({}, \"{}\"),\n", val, escaped));
                            }
                            code.push_str("];\n\n");
                        }
                    }
                }
            }
        }

        if vendor == "Nikon" {
            emit_nikon_shotinfo_custom(tables, &tables_with_masks, &mut code);
        }

        // Add lookup function
        code.push_str(
            r#"
/// Look up a tag by ID in the main table.
pub fn lookup(_tag_id: u16) -> Option<&'static TagDef> {
    // Default to main table - override in specific modules
    None
}
"#,
        );

        let mod_name = vendor.to_lowercase();
        let out_file = output.join(format!("{}.rs", mod_name));
        fs::write(&out_file, code)?;
        println!("Generated {:?}", out_file);
    }

    Ok(())
}

fn undef_len(format: &Option<serde_json::Value>) -> Option<u16> {
    let s = match format {
        Some(serde_json::Value::String(s)) => s.as_str(),
        _ => return None,
    };
    undef_len_str(s)
}

fn undef_len_str(s: &str) -> Option<u16> {
    let inner = s.strip_prefix("undef[")?.strip_suffix("]")?;
    inner.parse().ok()
}

fn is_settings_ptr(name: &str) -> bool {
    name == "CustomSettingsOffset"
        || name.contains("MenuOffset")
        || name.contains("MenuSettingsOffset")
}

fn emit_nikon_shotinfo_custom(
    tables: &BTreeMap<String, TagTable>,
    tables_with_masks: &HashSet<String>,
    code: &mut String,
) {
    let mut rows: Vec<(String, u16, u16, String, String)> = Vec::new();
    let mut ptrs: Vec<(String, u16, String)> = Vec::new();
    for (table_name, table) in tables {
        if let Some(tags) = &table.tags {
            for (id, info) in tags {
                if is_settings_ptr(&info.name) {
                    if let Ok(index) = id.parse::<u16>() {
                        if let Some(sub) = &info.sub_table {
                            let short = sub.trim_start_matches("Image::ExifTool::");
                            ptrs.push((table_name.clone(), index, short.to_string()));
                            if short.contains("NikonCustom::Settings") {
                                let leaf = short.rsplit("::").next().unwrap_or(short);
                                let group = leaf.replacen("Settings", "CustomSettings", 1);
                                let masks_ident =
                                    sanitize_ident(&short.replace("::", "_")).to_uppercase();
                                rows.push((short.to_string(), 0, 0, group, masks_ident));
                            }
                        }
                    }
                }
                if !info.name.starts_with("CustomSettings") || info.name.contains("Offset") {
                    continue;
                }
                if id.contains('.') {
                    continue;
                }
                let Ok(offset) = id.parse::<u16>() else {
                    continue;
                };
                let Some(size) = undef_len(&info.format) else {
                    continue;
                };
                let Some(sub) = &info.sub_table else {
                    continue;
                };
                let short = sub.trim_start_matches("Image::ExifTool::");
                let masks_ident = sanitize_ident(&short.replace("::", "_")).to_uppercase();
                rows.push((
                    table_name.clone(),
                    offset,
                    size,
                    info.name.clone(),
                    masks_ident,
                ));
            }
        }
        if let Some(blobs) = &table.blobs {
            for b in blobs {
                if !b.name.starts_with("CustomSettings") || b.name.contains("Offset") {
                    continue;
                }
                let Some(size) = b.format.as_deref().and_then(undef_len_str) else {
                    continue;
                };
                let Some(sub) = &b.sub_table else {
                    continue;
                };
                let short = sub.trim_start_matches("Image::ExifTool::");
                let masks_ident = sanitize_ident(&short.replace("::", "_")).to_uppercase();
                rows.push((
                    table_name.clone(),
                    b.index,
                    size,
                    b.name.clone(),
                    masks_ident,
                ));
            }
        }
    }
    if rows.is_empty() && ptrs.is_empty() {
        return;
    }
    rows.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(a.3.cmp(&b.3)));
    let mut seen = HashSet::new();
    rows.retain(|(table, _, _, group, _)| seen.insert((table.clone(), group.clone())));
    ptrs.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    ptrs.dedup();
    code.push_str(
        r#"/// ShotInfo / MenuSettings blob slice for a NikonCustom Settings* subdirectory.
/// `size` 0 means the slice runs to the last Mask index (pointer-target Settings tables).
#[derive(Debug, Clone, Copy)]
pub struct ShotInfoCustomDir {
    pub shot_info_table: &'static str,
    pub offset: u16,
    pub size: u16,
    pub group: &'static str,
    pub masks: &'static [MaskDef],
}

/// int32u pointer (`Start => $val`) to a nested binary table inside ShotInfo.
#[derive(Debug, Clone, Copy)]
pub struct ShotInfoCustomPtr {
    pub shot_info_table: &'static str,
    pub pointer_index: u16,
    pub nested_table: &'static str,
}

/// CustomSettings subdirs found on dumped Nikon tables.
pub static NIKON_SHOTINFO_CUSTOM_DIRS: &[ShotInfoCustomDir] = &[
"#,
    );
    for (table_name, offset, size, group, masks_ident) in &rows {
        let masks_ref = if tables_with_masks.contains(masks_ident) {
            format!("{}_MASKS", masks_ident)
        } else {
            "&[]".to_string()
        };
        code.push_str(&format!(
            "    ShotInfoCustomDir {{ shot_info_table: \"{}\", offset: {}, size: {}, group: \"{}\", masks: {} }},
",
            table_name, offset, size, group, masks_ref
        ));
    }
    code.push_str(
        r#"];

pub static NIKON_SHOTINFO_CUSTOM_PTRS: &[ShotInfoCustomPtr] = &[
"#,
    );
    for (table_name, index, nested) in &ptrs {
        code.push_str(&format!(
            "    ShotInfoCustomPtr {{ shot_info_table: \"{}\", pointer_index: {}, nested_table: \"{}\" }},
",
            table_name, index, nested
        ));
    }
    code.push_str("];\n\n");
}

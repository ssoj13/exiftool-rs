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
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
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
}

#[derive(Debug, Deserialize, Serialize)]
struct TagInfo {
    name: String,
    #[serde(default)]
    format: Option<serde_json::Value>,  // Can be string or number
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
    let json_content = fs::read_to_string(&json_path)
        .context("Failed to read tags.json")?;

    let data: BTreeMap<String, BTreeMap<String, TagTable>> = serde_json::from_str(&json_content)
        .context("Failed to parse tags.json")?;

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

    fs::write(output, &result.stdout)
        .context("Failed to write output")?;

    println!("Dumped tags to {:?}", output);
    Ok(())
}

fn generate_mod_rs(data: &BTreeMap<String, BTreeMap<String, TagTable>>, output: &std::path::Path) -> Result<()> {
    let mut code = String::from(
r#"//! Auto-generated tag definitions from ExifTool.
//! DO NOT EDIT MANUALLY - regenerate with: cargo xtask codegen

"#);

    // Module declarations
    for vendor in data.keys() {
        let mod_name = vendor.to_lowercase();
        code.push_str(&format!("pub mod {};\n", mod_name));
    }

    // No prelude - use vendor::TABLE_NAME directly to avoid collisions

    let out_file = output.join("mod.rs");
    fs::write(&out_file, code)?;
    println!("Generated {:?}", out_file);
    Ok(())
}

fn generate_vendor_modules(data: &BTreeMap<String, BTreeMap<String, TagTable>>, output: &std::path::Path) -> Result<()> {
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

"#, vendor);

        // Track generated constants to avoid duplicates
        let mut generated_constants: HashSet<String> = HashSet::new();

        // Generate tag tables
        for (table_name, table) in tables {
            if let Some(tags) = &table.tags {
                let safe_name = sanitize_ident(&table_name.replace("::", "_")).to_uppercase();

                code.push_str(&format!("/// {} tags\n", table_name));
                code.push_str(&format!("pub static {}: phf::Map<u16, TagDef> = phf::phf_map! {{\n", safe_name));

                for (id, info) in tags {
                    let tag_id: i64 = id.parse().unwrap_or(0);
                    if !(0..=0xFFFF).contains(&tag_id) {
                        continue;
                    }

                    let safe_tag_name = sanitize_ident(&info.name).to_uppercase();

                    // Build values array reference if present
                    let values_ref = if let Some(values) = &info.values {
                        if !values.is_empty() {
                            let values_name = format!("{}_{}_VALUES", safe_name, safe_tag_name);
                            format!("Some({})", values_name)
                        } else {
                            "None".to_string()
                        }
                    } else {
                        "None".to_string()
                    };

                    code.push_str(&format!(
                        "    {}u16 => TagDef {{ name: \"{}\", values: {} }},\n",
                        tag_id, info.name, values_ref
                    ));
                }
                code.push_str("};\n\n");

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

                            code.push_str(&format!("pub static {}: &[(i64, &str)] = &[\n", values_name));
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

        // Add lookup function
        code.push_str(r#"
/// Look up a tag by ID in the main table.
pub fn lookup(_tag_id: u16) -> Option<&'static TagDef> {
    // Default to main table - override in specific modules
    None
}
"#);

        let mod_name = vendor.to_lowercase();
        let out_file = output.join(format!("{}.rs", mod_name));
        fs::write(&out_file, code)?;
        println!("Generated {:?}", out_file);
    }

    Ok(())
}

use anyhow::{Context, Result};
use super::args::{InspectArgs, OutputFormat};
use crate::context::{condition, codegen_ctx};
use std::fs;
use std::path::Path;
use serde_json::Value;

/// Main entry for `forge-template inspect`
pub fn run(args: InspectArgs) -> Result<()> {
    let target = args.target.to_lowercase();

    if args.verbose {
        eprintln!("🔍 forge-template inspect");
        eprintln!("  target: {}", target);
        eprintln!("  file:   {}", args.file.display());
        eprintln!("  format: {:?}", args.as_format);
        eprintln!();
    }

    match target.as_str() {
        // ------------------------------------------------------------
        // Inspect raw YAML → JSON
        // ------------------------------------------------------------
        "yaml" => {
            let val = read_yaml_as_json(&args.file)
                .with_context(|| format!("failed to read YAML file {}", args.file.display()))?;
            print_value(&val, args.as_format, args.pretty, args.compact)?;
        }

        // ------------------------------------------------------------
        // Inspect conditioned YAML (header/payload normalized)
        // ------------------------------------------------------------
        "conditioned" | "normalized" => {
            let raw = read_yaml_as_json(&args.file)?;
            let conditioned = condition::condition_yaml(&raw);
            print_value(&conditioned, args.as_format, args.pretty, args.compact)?;
        }

        // ------------------------------------------------------------
        // Inspect final Forge context (used by templates)
        // ------------------------------------------------------------
        "ctx" | "context" => {
            let raw = read_yaml_as_json(&args.file)?;
            let conditioned = condition::condition_yaml(&raw);
            let ctx = codegen_ctx::build_yaml_codegen_context(&conditioned);
            print_value(&ctx, args.as_format, args.pretty, args.compact)?;
        }

        // ------------------------------------------------------------
        // Inspect Mustache template tokens
        // ------------------------------------------------------------
        "template" => {
            let tpl_text = fs::read_to_string(&args.file)
                .with_context(|| format!("failed to read template {}", args.file.display()))?;
            let tpl = mustache::compile_str(&tpl_text)
                .with_context(|| "failed to compile mustache template")?;
            println!("{:#?}", tpl);
        }

        // ------------------------------------------------------------
        // Invalid target
        // ------------------------------------------------------------
        _ => {
            eprintln!("❌ Unknown inspect target: '{}'", target);
            eprintln!("Available targets:");
            eprintln!("  - yaml");
            eprintln!("  - conditioned");
            eprintln!("  - ctx");
            eprintln!("  - template");
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

/// Load YAML → serde_json::Value
fn read_yaml_as_json(path: &Path) -> Result<Value> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    let yaml_val: serde_yaml::Value = serde_yaml::from_str(&raw)
        .with_context(|| "failed to parse YAML")?;
    let json_val = serde_json::to_value(yaml_val)
        .with_context(|| "failed to convert YAML → JSON")?;
    Ok(json_val)
}

/// Pretty-print JSON or YAML with optional compact/pretty modes
fn print_value(
    val: &Value,
    format: OutputFormat,
    pretty: bool,
    compact: bool,
) -> Result<()> {
    match format {
        OutputFormat::Json => {
            if compact {
                println!("{}", serde_json::to_string(val)?);
            } else if pretty {
                println!("{}", serde_json::to_string_pretty(val)?);
            } else {
                println!("{}", serde_json::to_string_pretty(val)?);
            }
        }
        OutputFormat::Yaml => {
            let s = serde_yaml::to_string(val)?;
            if compact {
                // remove blank lines for compactness
                for line in s.lines().filter(|l| !l.trim().is_empty()) {
                    println!("{}", line.trim_end());
                }
            } else {
                print!("{s}");
            }
        }
    }
    Ok(())
}

use std::fs;
use std::time::Instant;
use anyhow::{Context, Result};
use crate::codegen::render_yaml_from_abs;
use crate::context::{condition, codegen_ctx};
use crate::utils;
use super::args::RenderArgs;

/// Render command entrypoint
pub fn run(args: RenderArgs) -> Result<()> {
    // ------------------------------------------------------------
    // 0. Setup logging and timing
    // ------------------------------------------------------------
    let start = Instant::now();
    if args.verbose {
        eprintln!("🔧 forge-template render starting...");
        eprintln!("  YAML: {}", args.yaml.display());
        eprintln!("  TPL : {}", args.template.display());
    }

    // ------------------------------------------------------------
    // 1. Parse YAML and build conditioned context
    // ------------------------------------------------------------
    if args.verbose {
        eprintln!("📄 Parsing YAML...");
    }

    // Step 1: load raw YAML → serde_json::Value
    let raw_yaml: serde_json::Value = helpers::read_and_parse_yaml(&args.yaml)
        .with_context(|| format!("failed to read YAML: {}", args.yaml.display()))?;

    // Step 2: condition → { header, payload }
    let conditioned = condition::condition_yaml(&raw_yaml);

    if args.verbose {
        eprintln!(
            "✅ Conditioned YAML → header.type={:?}",
            conditioned.get("header")
                .and_then(|h| h.get("type"))
                .and_then(|v| v.as_str())
                .unwrap_or("<unknown>")
        );
    }

    // Step 3: build Forge codegen context (what Mustache sees)
    let ctx = codegen_ctx::build_yaml_codegen_context(&conditioned);

    if args.json {
        // Dump JSON context and exit
        let pretty = serde_json::to_string_pretty(&ctx)?;
        println!("{pretty}");
        return Ok(());
    }

    // ------------------------------------------------------------
    // 2. Optionally perform dry-run
    // ------------------------------------------------------------
    if args.dry_run {
        let json_preview = serde_json::to_string_pretty(&ctx)?;
        println!("🧪 Dry-run: YAML + template validated successfully.\n");
        println!("--- Context Preview ---\n{}\n------------------------", json_preview);
        eprintln!(
            "⏱️ Dry-run completed in {:.3?}",
            start.elapsed()
        );
        return Ok(());
    }

    // ------------------------------------------------------------
    // 3. Full render pipeline (YAML → Mustache → text)
    // ------------------------------------------------------------
    if args.verbose {
        eprintln!("🎨 Rendering template...");
    }

    let rendered = render_yaml_from_abs(&args.yaml, &args.template)
        .context("template rendering failed")?;

    // ------------------------------------------------------------
    // 4. Output handling
    // ------------------------------------------------------------
    if let Some(out_path) = args.out {
        fs::write(&out_path, &rendered)
            .with_context(|| format!("failed to write output to {}", out_path.display()))?;
        println!("✅ Wrote rendered output to {}", out_path.display());
    } else {
        println!("{rendered}");
    }

    // ------------------------------------------------------------
    // 5. Timing summary
    // ------------------------------------------------------------
    if args.verbose {
        eprintln!("⏱️ Total elapsed: {:.3?}", start.elapsed());
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Local utility (for dry-run / verbose pipelines)
// -----------------------------------------------------------------------------
mod helpers {
    use std::fs;
    use std::path::Path;
    use anyhow::{Context, Result};
    use serde_json::Value;

    /// Read YAML file and convert it into a JSON Value
    pub fn read_and_parse_yaml(path: &Path) -> Result<Value> {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read file {}", path.display()))?;
        let yaml_val: serde_yaml::Value =
            serde_yaml::from_str(&raw).with_context(|| "failed to parse YAML")?;
        let json_val = serde_json::to_value(yaml_val)
            .with_context(|| "failed to convert YAML → JSON")?;
        Ok(json_val)
    }
}

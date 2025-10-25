use std::{fs, path::PathBuf, process::Command};
use clap::Parser;
use colored::*;
use anyhow::{Result, anyhow};
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about = "Forge IDE Template Orchestrator")]
struct Args {
    /// YAML context file (manifest or struct)
    #[arg(short, long)]
    context: PathBuf,

    /// Mustache template path
    #[arg(short, long)]
    template: PathBuf,

    /// Output destination file
    #[arg(short, long)]
    output: PathBuf,

    /// Overwrite output if exists
    #[arg(long)]
    force: bool,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    manifest: Option<serde_yaml::Value>,
    crates: Option<serde_yaml::Value>,
}

fn ensure_path(path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn render_template(ctx: &PathBuf, template: &PathBuf, output: &PathBuf, force: bool) -> Result<()> {
    if output.exists() && !force {
        println!("{}", format!("SKIP: {}", output.display()).yellow());
        return Ok(());
    }

    ensure_path(output)?;
    let output_str = Command::new("forge-template")
        .arg("render")
        .arg(ctx)
        .arg(template)
        .output()?;

    if !output_str.status.success() {
        return Err(anyhow!("forge-template render failed"));
    }

    fs::write(output, &output_str.stdout)?;
    println!("{}", format!("OK: Rendered → {}", output.display()).green());
    Ok(())
}

fn register_mod(output: &PathBuf) -> Result<()> {
    let mod_name = output.file_stem().unwrap().to_str().unwrap();
    let mod_rs = output.parent().unwrap().join("mod.rs");
    let lib_rs = output.parent().unwrap().parent().unwrap().join("lib.rs");

    let root = if mod_rs.exists() {
        mod_rs
    } else if lib_rs.exists() {
        lib_rs
    } else {
        println!("{}", "WARN: No mod.rs or lib.rs found".yellow());
        return Ok(());
    };

    let text = fs::read_to_string(&root)?;
    if !text.contains(&format!("pub mod {}", mod_name)) {
        fs::write(&root, format!("{}\npub mod {};\n", text, mod_name))?;
        println!("{}", format!("ADD: Registered module {}", mod_name).green());
    } else {
        println!("{}", format!("INFO: Module already registered {}", mod_name).cyan());
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("{}", "[INIT] Forge IDE Template Orchestrator".bold());
    render_template(&args.context, &args.template, &args.output, args.force)?;
    register_mod(&args.output)?;
    println!("{}", "[DONE] All templates processed".bold());
    Ok(())
}

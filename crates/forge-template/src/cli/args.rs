use clap::{Args, ValueEnum};
use std::path::PathBuf;

// ============================================================================
// RENDER COMMAND
// ============================================================================
#[derive(Args, Debug)]
#[command(about = "Render YAML + Mustache into deterministic output")]
pub struct RenderArgs {
    /// Path to YAML file
    pub yaml: PathBuf,

    /// Path to Mustache template
    pub template: PathBuf,

    /// Optional output file path
    #[arg(short, long)]
    pub out: Option<PathBuf>,

    /// Only parse and show context (don’t render)
    #[arg(long, help = "Run the full parse pipeline but skip rendering")]
    pub dry_run: bool,

    /// Print normalized JSON context instead of rendering
    #[arg(long, help = "Dump the Forge codegen context as JSON")]
    pub json: bool,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Display elapsed time metrics
    #[arg(long)]
    pub timing: bool,
}

// ============================================================================
// INSPECT COMMAND
// ============================================================================
#[derive(Args, Debug)]
#[command(about = "Inspect YAML, conditioned structure, context, or template")]
pub struct InspectArgs {
    /// Inspection target: yaml | conditioned | ctx | template
    pub target: String,

    /// Path to file (YAML or template)
    pub file: PathBuf,

    /// Output format (json or yaml)
    #[arg(long = "as", value_enum, default_value_t = OutputFormat::Json)]
    pub as_format: OutputFormat,

    /// Pretty-print output (default true for JSON/YAML)
    #[arg(long)]
    pub pretty: bool,

    /// Compact mode (no indentation, minimal spacing)
    #[arg(long)]
    pub compact: bool,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
}

// ============================================================================
// SERVE COMMAND
// ============================================================================
#[derive(Args, Debug)]
#[command(about = "Run forge-template as a long-running HTTP service")]
pub struct ServeArgs {
    /// Port to listen on
    #[arg(long, default_value_t = 7070)]
    pub port: u16,

    /// Address to bind (default 127.0.0.1)
    #[arg(long, default_value = "127.0.0.1")]
    pub addr: String,

    /// Enable hot reload (watch YAML + template files)
    #[arg(long)]
    pub watch: bool,

    /// Verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

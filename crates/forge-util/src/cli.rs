use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "forge-util", version, about = "Forge development utilities")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan the workspace and emit a YAML “crate -> dirs/files” map
    ScanSources(ScanSourcesArgs),
}

#[derive(clap::Args, Debug, Clone)]
pub struct ScanSourcesArgs {
    /// Workspace root; defaults to $FORGE_ROOT or CWD
    #[arg(long)]
    pub root: Option<PathBuf>,

    /// Verbose logging (print directories as they are scanned)
    #[arg(short, long)]
    pub verbose: bool,

    /// Comma-separated dirs to exclude (in addition to defaults)
    #[arg(long, default_value = "")]
    pub exclude: String,
}

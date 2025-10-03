mod cli;
mod utils;
mod scan_sources;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::ScanSources(args) => scan_sources::run(args),
    }
}

pub mod args;
pub mod render_cmd;
pub mod inspect_cmd;
pub mod serve_cmd;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ForgeCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Render YAML + Mustache into deterministic output
    Render(args::RenderArgs),

    /// Inspect YAML, context, or templates
    Inspect(args::InspectArgs),

    /// Run forge-template as a long-running service
    Serve(args::ServeArgs),
}

pub fn run() -> Result<()> {
    let cli = ForgeCli::parse();

    match cli.command {
        Commands::Render(args) => render_cmd::run(args),
        Commands::Inspect(args) => inspect_cmd::run(args),
        Commands::Serve(args) => serve_cmd::run(args),
    }
}

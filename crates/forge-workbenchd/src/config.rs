use std::net::SocketAddr;
use std::path::PathBuf;
use clap::Parser;

/// Shared application state for the daemon.
#[derive(Clone)]
pub struct AppState {
    /// Root folder to scan and serve to the web UI
    pub root: PathBuf,
    /// Optional API token (checked by middleware)
    pub token: Option<String>,
}

/// Command-line arguments / env config for the daemon.
#[derive(Parser, Debug)]
pub struct Args {
    /// Project root to scan (defaults to $FORGE_ROOT or CWD)
    #[arg(long)]
    pub root: Option<PathBuf>,

    /// Bind address for the HTTP server
    #[arg(long, default_value = "0.0.0.0:8787")]
    pub bind: SocketAddr,

    /// Optional API token (requests must send header: X-Forge-Token)
    #[arg(long)]
    pub token: Option<String>,
}

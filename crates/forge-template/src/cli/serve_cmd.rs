use anyhow::Result;
use super::args::ServeArgs;

pub fn run(args: ServeArgs) -> Result<()> {
    println!("🚀 Starting forge-template service on port {}", args.port);
    // Future: add Axum or Warp server for /render and /inspect
    Ok(())
}

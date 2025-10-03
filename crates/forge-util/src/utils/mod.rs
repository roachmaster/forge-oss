// crates/forge-util/src/utils/mod.rs
pub mod env;

use anyhow::Result;
use std::env as std_env;
use std::path::{Path, PathBuf};

/// Resolve the workspace root from (in order): CLI arg, $FORGE_ROOT, or CWD.
pub fn resolve_workspace_root(cli_root: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(r) = cli_root {
        return Ok(r);
    }
    if let Ok(r) = std_env::var("FORGE_ROOT") {
        return Ok(PathBuf::from(r));
    }
    Ok(std_env::current_dir()?)
}

/// `<workspace>/crates`
pub fn crates_dir(ws: &Path) -> PathBuf {
    ws.join("crates")
}

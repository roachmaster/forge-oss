mod model;
mod walker;
mod emit;

pub use model::{CrateMap, DirNode};

use anyhow::Result;
use crate::cli::ScanSourcesArgs;
use crate::utils::{crates_dir, resolve_workspace_root};

pub fn run(args: ScanSourcesArgs) -> Result<()> {
    let ws = resolve_workspace_root(args.root)?;
    let crates_root = crates_dir(&ws);

    println!("🔎 Scanning crates under {}", ws.display());

    // Build exclude list
    let mut exclude = vec![
        "target".to_string(),
        ".git".to_string(),
        "node_modules".to_string(),
        "tests".to_string(),
    ];
    if !args.exclude.trim().is_empty() {
        exclude.extend(args.exclude.split(',').map(|s| s.trim().to_string()));
    }

    let map = walker::scan_workspace(&crates_root, &exclude, args.verbose)?;
    emit::print_yaml(&map);
    Ok(())
}

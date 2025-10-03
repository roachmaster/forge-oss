use std::env;
use std::path::{Path, PathBuf};

/// Resolve workspace root:
/// 1) --root arg if provided
/// 2) $FORGE_ROOT if set
/// 3) current working directory
pub fn resolve_workspace_root(arg_root: Option<String>) -> PathBuf {
    if let Some(r) = arg_root {
        return PathBuf::from(r);
    }
    if let Ok(r) = env::var("FORGE_ROOT") {
        if !r.trim().is_empty() {
            return PathBuf::from(r);
        }
    }
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Return `<root>/<crates_dir>`, but don’t require it to exist.
/// Callers can decide what to do if it’s missing.
pub fn crates_dir(root: &Path, crates_dir: &str) -> PathBuf {
    root.join(crates_dir)
}

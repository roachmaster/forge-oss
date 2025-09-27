use std::path::Path;

/// Compute a stable “repo id” string for client-side namespaces.
/// We default to the canonical absolute path; if that fails, use display().
pub fn compute_repo_id(root: &Path) -> String {
    std::fs::canonicalize(root)
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| root.display().to_string())
}

// forge-proc/src/env.rs
use std::env;

/// Fetch the repo root (from FORGE_ROOT env var).
/// Fails with a clear error message if not set.
pub(crate) fn forge_root() -> Result<String, String> {
    env::var("FORGE_ROOT")
        .map_err(|_| "FORGE_ROOT must be set (e.g., /Users/you/forge-oss)".to_string())
}

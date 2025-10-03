use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::{services, AppState};
use forge_view_model::WorkbenchVM;

/// Tuning knobs (centralize so changing defaults is trivial).
const DEFAULT_TREE_DEPTH: usize = 12;   // previously 3
const MAX_TREE_DEPTH: usize = 64;       // safety cap to avoid huge payloads

#[derive(Deserialize, Debug)]
pub struct SnapshotParams {
    /// Optional override; if omitted or empty, uses AppState.root
    pub root: Option<String>,
    /// Optional depth cap; clamped to MAX_TREE_DEPTH (default: DEFAULT_TREE_DEPTH)
    pub depth: Option<usize>,
}

fn sanitize_root(param: Option<String>) -> Option<PathBuf> {
    match param.and_then(|s| {
        let t = s.trim().to_string();
        if t.is_empty() { None } else { Some(t) }
    }) {
        Some(s) => Some(PathBuf::from(s)),
        None => None,
    }
}

/// GET /v1/snapshot?root=/path&depth=3
pub async fn get_snapshot(
    State(state): State<AppState>,
    Query(p): Query<SnapshotParams>,
) -> Json<WorkbenchVM> {
    // Compute effective depth (clamped)
    let requested_depth = p.depth.unwrap_or(DEFAULT_TREE_DEPTH);
    let depth = requested_depth.min(MAX_TREE_DEPTH);

    // Resolve root:
    // - use sanitized query param if it exists and points to a directory
    // - otherwise fall back to the server default root
    let qroot = sanitize_root(p.root);
    let root = match qroot {
        Some(candidate) if Path::new(&candidate).is_dir() => candidate,
        Some(bad) => {
            eprintln!(
                "snapshot: ignoring non-directory root param: {}",
                bad.display()
            );
            state.root.clone()
        }
        None => state.root.clone(),
    };

    eprintln!(
        "snapshot: root='{}' depth_req={} depth_eff={}",
        root.display(),
        requested_depth,
        depth
    );

    // Build VM.
    let vm = services::fs_tree::build_snapshot(&root, depth);
    Json(vm)
}

use axum::{extract::{Query, State}, Json};
use serde::Deserialize;
use std::path::PathBuf;

use crate::{AppState, services};
use forge_view_model::WorkbenchVM;

/// Tuning knobs (centralize so changing defaults is trivial).
/// TODO: consider moving these into `crate::config` and/or reading from env.
const DEFAULT_TREE_DEPTH: usize = 12;   // previously 3
const MAX_TREE_DEPTH: usize = 64;       // safety cap to avoid huge payloads

#[derive(Deserialize)]
pub struct SnapshotParams {
    /// Optional override; if omitted, uses AppState.root
    pub root: Option<String>,
    /// Optional depth cap; clamped to MAX_TREE_DEPTH (default: DEFAULT_TREE_DEPTH)
    pub depth: Option<usize>,
}

/// GET /v1/snapshot?root=/path&depth=3
pub async fn get_snapshot(
    State(state): State<AppState>,
    Query(p): Query<SnapshotParams>,
) -> Json<WorkbenchVM> {
    // Root: query param wins, else server default
    let root = p.root
        .map(PathBuf::from)
        .unwrap_or_else(|| state.root.clone());

    // Depth: query param (clamped) or default
    let depth = p
        .depth
        .map(|d| d.min(MAX_TREE_DEPTH))
        .unwrap_or(DEFAULT_TREE_DEPTH);

    let vm = services::fs_tree::build_snapshot(&root, depth);
    Json(vm)
}

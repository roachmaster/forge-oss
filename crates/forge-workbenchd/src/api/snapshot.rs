use axum::{extract::{Query, State}, Json};
use serde::Deserialize;
use std::path::PathBuf;

use crate::{AppState, services};
use forge_view_model::WorkbenchVM;

#[derive(Deserialize)]
pub struct SnapshotParams {
    /// Optional override; if omitted, uses AppState.root
    pub root: Option<String>,
    /// Optional depth cap (default 3)
    pub depth: Option<usize>,
}

/// GET /v1/snapshot?root=/path&depth=3
pub async fn get_snapshot(
    State(state): State<AppState>,
    Query(p): Query<SnapshotParams>,
) -> Json<WorkbenchVM> {
    let root = p.root
        .map(PathBuf::from)
        .unwrap_or_else(|| state.root.clone());
    let depth = p.depth.unwrap_or(3);
    let vm = services::fs_tree::build_snapshot(&root, depth);
    Json(vm)
}

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;
use crate::AppState;
use axum::{extract::State, Json};
// REMOVE the debug_handler attribute (or enable axum "macros" feature)
// #[axum::debug_handler]

#[derive(Deserialize)]
pub struct SaveReq {
    pub path: String,
    pub content: String,
    #[serde(default)]
    pub client_sha: Option<String>,
}

#[derive(Serialize)]
pub struct SaveResp {
    pub path: String,
    pub sha256: String,
    pub bytes_written: usize,
}

/// POST /v1/save
pub async fn post_save(
    State(state): State<AppState>,
    Json(req): Json<SaveReq>,
) -> Json<SaveResp> {
    // Resolve path against configured root (simple, no path traversal handling here)
    let abs: PathBuf = resolve_path(&state.root, &req.path);

    // Write file (atomic-ish: direct write; consider tmp+rename for robustness)
    fs::write(&abs, req.content.as_bytes())
        .await
        .expect("write failed");

    // Compute sha256 of the saved content
    use sha2::{Digest, Sha256};
    let sha = format!("{:x}", Sha256::digest(req.content.as_bytes()));

    let resp = SaveResp {
        path: req.path,
        sha256: sha,
        bytes_written: req.content.len(),
    };
    Json(resp)
}

fn resolve_path(root: &Path, rel: &str) -> PathBuf {
    let p = Path::new(rel);
    if p.is_absolute() { p.to_path_buf() } else { root.join(p) }
}

use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;
use forge_fs::file_read::read_text_file;
use forge_fs::hash::sha256_file;

/// Client request to open a file (repo-relative).
#[derive(Deserialize)]
pub struct OpenFileReq {
    pub path: String,
    pub client_sha: Option<String>,
}

#[derive(Serialize)]
pub struct OpenFileResp {
    pub path: String,
    pub size_bytes: u64,
    pub char_count: usize,
    pub line_count: usize,
    /// Present only when content changed or client had no sha.
    pub content: Option<String>,
    pub sha256: String,
    pub unchanged: bool,
}

/// POST /v1/open  { path, client_sha? }
pub async fn post_open_file(
    State(state): State<AppState>,
    Json(req): Json<OpenFileReq>,
) -> impl IntoResponse {
    let abs = state.root.join(&req.path);

    // Compute hash first to allow cheap “unchanged” short-circuit.
    let sha: String = match sha256_file(&abs) {
        Ok(s) => s,
        Err(e) => {
            return (
                axum::http::StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": format!("open failed: {e}") })),
            )
                .into_response();
        }
    };

    // If client has same sha, return only metadata.
    if req.client_sha.as_deref() == Some(sha.as_str()) {
        match read_text_file(&abs, 0) {
            Ok(info) => {
                let resp = OpenFileResp {
                    path: req.path,
                    size_bytes: info.size_bytes,
                    char_count: info.char_count,
                    line_count: info.line_count,
                    content: None,
                    sha256: sha,
                    unchanged: true,
                };
                return Json(resp).into_response();
            }
            Err(e) => {
                return (
                    axum::http::StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "error": format!("stat failed: {e}") })),
                )
                    .into_response();
            }
        }
    }

    // Otherwise send full contents.
    match read_text_file(&abs, 0) {
        Ok(info) => {
            let resp = OpenFileResp {
                path: req.path,
                size_bytes: info.size_bytes,
                char_count: info.char_count,
                line_count: info.line_count,
                content: Some(info.content),
                sha256: sha,
                unchanged: false,
            };
            Json(resp).into_response()
        }
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("read failed: {e}") })),
        )
            .into_response(),
    }
}

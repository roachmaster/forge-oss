use axum::{
    extract::{Query, State, ws::{WebSocketUpgrade, Message, WebSocket}},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::path::PathBuf;
use crate::{AppState, services};
use forge_view_model::{WorkbenchVM, ServerToClient};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/snapshot", get(get_snapshot))
        .route("/ws", get(ws_handler))
        .with_state(state)
}

// GET /healthz
async fn healthz() -> &'static str {
    "ok"
}

#[derive(Deserialize)]
struct SnapshotParams {
    /// Optional override; if omitted, uses AppState.root
    root: Option<String>,
    /// Optional depth cap (default 3)
    depth: Option<usize>,
}

// GET /v1/snapshot?root=/path&depth=3
async fn get_snapshot(State(state): State<AppState>, Query(p): Query<SnapshotParams>) -> Json<WorkbenchVM> {
    let root = p.root
        .map(PathBuf::from)
        .unwrap_or_else(|| state.root.clone());
    let depth = p.depth.unwrap_or(3);
    let vm = services::fs_tree::build_snapshot(&root, depth);
    Json(vm)
}

// WS /ws (send one snapshot; keep the line open for future pushes)
async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws_conn(socket, state))
}

async fn ws_conn(mut socket: WebSocket, state: AppState) {
    let vm = services::fs_tree::build_snapshot(&state.root, 3);
    let msg = serde_json::to_string(&ServerToClient::Snapshot { vm }).unwrap();
    let _ = socket.send(Message::Text(msg)).await;

    // keep open, ignore client messages for now
    while let Some(Ok(_m)) = socket.recv().await {}
}

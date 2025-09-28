use axum::{
    extract::{State, ws::{Message, WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
};
use crate::{AppState, services};
use forge_view_model::{ServerToClient, ClientIntent};
use forge_fs::{file_read::read_text_file, hash::sha256_file};

/// GET /ws
pub async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws_conn(socket, state))
}

pub async fn ws_conn(mut socket: WebSocket, state: AppState) {
    // 1) Send initial snapshot
    {
        let vm = services::fs_tree::build_snapshot(&state.root, 3);
        if let Ok(text) = serde_json::to_string(&ServerToClient::Snapshot { vm }) {
            let _ = socket.send(Message::Text(text)).await;
        }
    }

    // 2) Handle client intents
    while let Some(Ok(incoming)) = socket.recv().await {
        let Message::Text(txt) = incoming else { continue };
        let Ok(intent) = serde_json::from_str::<ClientIntent>(&txt) else { continue };

        match intent {
            ClientIntent::OpenFile { path, known_sha } => {
                let abs = state.root.join(&path);

                // Compute hash first (cheap unchanged short-circuit)
                let Ok(sha) = sha256_file(&abs) else {
                    // Could send a status/terminal message; ignore for now.
                    continue;
                };

                match read_text_file(&abs, 0) {
                    Ok(info) => {
                        // If client already has the same sha, send small “unchanged” message
                        if known_sha.as_deref() == Some(&sha) {
                            let msg = ServerToClient::FileUnchanged {
                                path,
                                size_bytes: info.size_bytes,
                                char_count: info.char_count,
                                line_count: info.line_count,
                                sha256: sha,
                            };
                            send_json(&mut socket, &msg).await;
                        } else {
                            // Otherwise send full content
                            let msg = ServerToClient::FileOpened {
                                path,
                                content: info.content,
                                size_bytes: info.size_bytes,
                                char_count: info.char_count,
                                line_count: info.line_count,
                                sha256: sha,
                            };
                            send_json(&mut socket, &msg).await;
                        }
                    }
                    Err(_e) => {
                        // Optionally send a status message back (left minimal for now)
                        // e.g., ServerToClient::Status { ... }
                    }
                }
            }

            // Not yet handled on the server side—ignore for now.
            ClientIntent::ToggleDir { .. }
            | ClientIntent::SelectProject { .. }
            | ClientIntent::SaveFile { .. }
            | ClientIntent::RunCmd { .. }
            | ClientIntent::BuildCmd
            | ClientIntent::ClearLogs
            | ClientIntent::Reconnect => { /* no-op */ }
        }
    }
}

async fn send_json<T: serde::Serialize>(socket: &mut WebSocket, value: &T) {
    if let Ok(text) = serde_json::to_string(value) {
        let _ = socket.send(Message::Text(text)).await;
    }
}

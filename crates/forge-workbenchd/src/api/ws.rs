use axum::{
    extract::{State, ws::{WebSocketUpgrade, WebSocket, Message}},
    response::IntoResponse,
};
use crate::AppState;
use crate::services;
use forge_view_model::{ServerToClient, ClientIntent};
use forge_fs::{file_read::read_text_file, hash::sha256_file};
use tokio::io::AsyncWriteExt;

pub async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws_conn(socket, state))
}

pub async fn ws_conn(mut socket: WebSocket, state: AppState) {
    // Initial snapshot
    let vm = services::fs_tree::build_snapshot(&state.root, 3);
    if let Ok(text) = serde_json::to_string(&ServerToClient::Snapshot { vm }) {
        let _ = socket.send(Message::Text(text)).await;
    }

    // Receive client intents
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(txt) => {
                // Try parse as ClientIntent
                match serde_json::from_str::<ClientIntent>(&txt) {
                    Ok(intent) => {
                        match intent {
                            ClientIntent::OpenFile { path, known_sha } => {
                                println!("OpenFile intent: path={path:?}, known_sha={known_sha:?}");
                                let abs = state.root.join(&path);
                                match sha256_file(&abs) {
                                    Ok(sha) => {
                                        // If client has the same content, return metadata only
                                        if known_sha.as_deref() == Some(&sha) {
                                            match read_text_file(&abs, 0) {
                                                Ok(info) => {
                                                    println!("FileUnchanged: path={path} sha={sha}");
                                                    let resp = ServerToClient::FileUnchanged {
                                                        path,
                                                        size_bytes: info.size_bytes,
                                                        char_count: info.char_count,
                                                        line_count: info.line_count,
                                                        sha256: sha,
                                                    };
                                                    if let Ok(s) = serde_json::to_string(&resp) {
                                                        let _ = socket.send(Message::Text(s)).await;
                                                    }
                                                }
                                                Err(e) => {
                                                    eprintln!("stat failed for unchanged file {abs:?}: {e}");
                                                }
                                            }
                                        } else {
                                            // Send full contents
                                            match read_text_file(&abs, 0) {
                                                Ok(info) => {
                                                    println!("FileOpened: path={path} sha={sha} bytes={}", info.size_bytes);
                                                    let resp = ServerToClient::FileOpened {
                                                        path,
                                                        content: info.content,
                                                        size_bytes: info.size_bytes,
                                                        char_count: info.char_count,
                                                        line_count: info.line_count,
                                                        sha256: sha,
                                                    };
                                                    if let Ok(s) = serde_json::to_string(&resp) {
                                                        let _ = socket.send(Message::Text(s)).await;
                                                    }
                                                }
                                                Err(e) => {
                                                    eprintln!("read failed {abs:?}: {e}");
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("sha256 failed {abs:?}: {e}");
                                    }
                                }
                            }
                            ClientIntent::ToggleDir { path, open } => {
                                println!("ToggleDir {:?} => open={}", path, open);
                                // (No server-side mutation yet; client keeps TreeState)
                            }
                            // Ignore for now
                            _ => {}
                        }
                    }
                    Err(e) => {
                        eprintln!("WS text not a ClientIntent: {e}; payload={txt}");
                    }
                }
            }
            Message::Binary(_) => { /* ignore */ }
            Message::Ping(p)   => { let _ = socket.send(Message::Pong(p)).await; }
            Message::Pong(_)   => {}
            Message::Close(_)  => break,
        }
    }
}

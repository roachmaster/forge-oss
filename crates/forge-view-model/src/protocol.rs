use serde::{Deserialize, Serialize};

use crate::workbench::WorkbenchVM;

/// Server → Client messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ServerToClient {
    #[serde(rename = "snapshot")]
    Snapshot { vm: WorkbenchVM },

    /// Full payload when server sends (or resends) file contents.
    #[serde(rename = "file_opened")]
    FileOpened {
        path: String,
        content: String,
        size_bytes: u64,
        char_count: usize,
        line_count: usize,
        sha256: String,
    },

    /// Optimization: server indicates client already has the latest content,
    /// but still includes current file metadata so UI can update stats/sha.
    #[serde(rename = "file_unchanged")]
    FileUnchanged {
        path: String,
        size_bytes: u64,
        char_count: usize,
        line_count: usize,
        sha256: String,
    },

    // Future: Diff { ops: Vec<Patch> },
}

/// Client → Server intents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientIntent {
    SelectProject { path: String },

    /// Ask server to open a file. If `known_sha` matches the server's current
    /// hash, the server may reply with `FileUnchanged` instead of sending content.
    OpenFile { path: String, known_sha: Option<String> },

    /// Toggle a directory’s open state (explicit target for idempotency).
    ToggleDir { path: String, open: bool },

    SaveFile { path: String, content: String },
    RunCmd { cmd: String },
    BuildCmd,
    ClearLogs,
    Reconnect,
}

use serde::{Deserialize, Serialize};

use crate::workbench::WorkbenchVM;

/// Server → Client messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ServerToClient {
    #[serde(rename = "snapshot")]
    Snapshot { vm: WorkbenchVM },
    // Future: Diff { ops: Vec<Patch> },
}

/// Client → Server intents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientIntent {
    SelectProject { path: String },
    OpenFile { path: String },
    /// Toggle a directory’s open state (explicit target for idempotency).
    ToggleDir { path: String, open: bool },
    SaveFile { path: String, content: String },
    RunCmd { cmd: String },
    BuildCmd,
    ClearLogs,
    Reconnect,
}

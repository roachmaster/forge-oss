use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HeaderVM {
    pub title: String,
    pub can_build: bool,
    pub can_run: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreeNodeVM {
    pub name: String,
    pub is_dir: bool,
    pub open: bool,
    pub children: Vec<TreeNodeVM>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreeVM {
    pub roots: Vec<TreeNodeVM>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorVM {
    pub file_path: String,
    pub content: String,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub is_dirty: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalVM {
    pub lines: Vec<String>,
    pub is_busy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatusVM {
    pub msg: String,
    pub connected: bool,
}

/// End-to-end “display snapshot” the view will render.
/// The client can deserialize this and hand it to the dumb View.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkbenchVM {
    pub header: HeaderVM,
    pub tree: TreeVM,
    pub editor: EditorVM,
    pub terminal: TerminalVM,
    pub status: StatusVM,
}

/// Simple WS protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ServerToClient {
    #[serde(rename = "snapshot")]
    Snapshot { vm: WorkbenchVM },
    // Later: Diff { ops: Vec<Patch> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientIntent {
    SelectProject { path: String },
    OpenFile { path: String },
    ToggleDir { path: String },
    SaveFile { path: String, content: String },
    RunCmd { cmd: String },
    BuildCmd,
    ClearLogs,
    Reconnect,
}

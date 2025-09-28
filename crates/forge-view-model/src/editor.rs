use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorVM {
    /// Full repo-relative path of the open file.
    pub file_path: String,

    /// Entire text content of the file.
    pub content: String,

    /// Current cursor location in the client.
    pub cursor_line: usize,
    pub cursor_col: usize,

    /// True if the user has unsaved changes.
    pub is_dirty: bool,

    /// Metadata for UI and cache control.
    #[serde(default)]
    pub size_bytes: usize,
    #[serde(default)]
    pub char_count: usize,
    #[serde(default)]
    pub line_count: usize,

    /// SHA-256 of the current content.  
    /// Lets the client send `known_sha` when requesting a file so the server
    /// can skip re-sending unchanged data.
    #[serde(default)]
    pub sha256: String,
}

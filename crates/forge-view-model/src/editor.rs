use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorVM {
    pub file_path: String,
    pub content: String,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub is_dirty: bool,

    // Reserved for upcoming metadata (safe defaults keep wire format stable)
    #[serde(default)] pub size_bytes: usize,
    #[serde(default)] pub char_count: usize,
    #[serde(default)] pub line_count: usize,
}

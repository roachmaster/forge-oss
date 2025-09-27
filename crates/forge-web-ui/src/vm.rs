// crates/forge-web-ui/src/vm.rs
#[derive(Debug, Clone, Default)]
pub struct HeaderVM {
    pub title: String,
    pub can_build: bool,
    pub can_run: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TreeNodeVM {
    pub name: String,
    pub is_dir: bool,
    pub open: bool,
    pub children: Vec<TreeNodeVM>,
}
#[derive(Debug, Clone, Default)]
pub struct TreeVM {
    pub roots: Vec<TreeNodeVM>,
}

#[derive(Debug, Clone, Default)]
pub struct EditorVM {
    pub file_path: String,
    pub content: String,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub is_dirty: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TerminalVM {
    pub lines: Vec<String>,
    pub is_busy: bool,
}

#[derive(Debug, Clone, Default)]
pub struct StatusVM {
    pub msg: String,
    pub connected: bool,
}

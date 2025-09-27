use serde::{Deserialize, Serialize};

/// Single node in the file tree.
///
/// - `path`: repo-relative stable identifier (e.g., "crates/forge-web-ui/src").
/// - `is_dir`: directory vs file.
/// - `open`: initial state from server; client may override locally.
/// - `has_children`: true if dir contains entries, used to draw a chevron.
/// - `children`: populated when the node is open (files keep this empty).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreeNodeVM {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub open: bool,
    pub has_children: bool,
    pub children: Vec<TreeNodeVM>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreeVM {
    pub roots: Vec<TreeNodeVM>,
}

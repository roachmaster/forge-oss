use serde::{Deserialize, Serialize};

use crate::{HeaderVM, TreeVM, EditorVM, TerminalVM, StatusVM};

/// End-to-end display snapshot the client renders.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkbenchVM {
    pub header: HeaderVM,
    pub tree: TreeVM,
    pub editor: EditorVM,
    pub terminal: TerminalVM,
    pub status: StatusVM,
}

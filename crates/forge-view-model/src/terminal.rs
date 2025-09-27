use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalVM {
    pub lines: Vec<String>,
    pub is_busy: bool,
}

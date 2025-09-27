use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatusVM {
    pub msg: String,
    pub connected: bool,
}

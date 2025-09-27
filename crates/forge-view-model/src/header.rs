use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HeaderVM {
    /// UI title; typically the repo name.
    pub title: String,
    pub can_build: bool,
    pub can_run: bool,

    /// Optional stable project identifier (e.g., canonical root path or hash).
    /// If you’re not ready to supply this yet, leave it empty on the server.
    #[serde(default)]
    pub repo_id: String,
}

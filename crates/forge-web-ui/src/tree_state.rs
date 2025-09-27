use std::cell::RefCell;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use web_sys::window;

/// Client-owned UI state for the tree (expanded/selected).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreeState {
    /// Repo-relative paths of expanded directories.
    pub expanded: HashSet<String>,
    /// Currently selected path (file or dir), if any.
    pub selected: Option<String>,
}

impl TreeState {
    /// Toggle a directory path. Returns the resulting `open` state.
    pub fn toggle(&mut self, path: &str) -> bool {
        if self.expanded.contains(path) {
            self.expanded.remove(path);
            false
        } else {
            self.expanded.insert(path.to_string());
            true
        }
    }

    pub fn set_open(&mut self, path: &str, open: bool) {
        if open {
            self.expanded.insert(path.to_string());
        } else {
            self.expanded.remove(path);
        }
    }

    pub fn is_expanded(&self, path: &str) -> bool {
        self.expanded.contains(path)
    }

    pub fn set_selected(&mut self, path: Option<String>) {
        self.selected = path;
    }
}

/* ---------- persistence (localStorage) ---------- */

const LS_PREFIX: &str = "forge.tree_state";

fn storage() -> Option<web_sys::Storage> {
    let win = window()?;
    // local_storage() -> Result<Option<Storage>, JsValue>
    win.local_storage().ok().flatten()
}

fn ls_key_for_repo(repo_key: &str) -> String {
    format!("{}::{}", LS_PREFIX, repo_key)
}

fn compute_default_repo_key() -> String {
    // Simple default based on page host; override via `set_repo_key` at startup.
    let host = window()
        .and_then(|w| w.location().host().ok())
        .unwrap_or_else(|| "localhost".to_string());
    format!("{}::default", host)
}

fn load_tree_state_from_ls(repo_key: &str) -> TreeState {
    let key = ls_key_for_repo(repo_key);
    if let Some(store) = storage() {
        if let Ok(item) = store.get_item(&key) {
            if let Some(raw) = item {
                if let Ok(ts) = serde_json::from_str::<TreeState>(&raw) {
                    return ts;
                }
            }
        }
    }
    TreeState::default()
}

fn save_tree_state_to_ls(repo_key: &str, ts: &TreeState) {
    if let Some(store) = storage() {
        if let Ok(raw) = serde_json::to_string(ts) {
            let _ = store.set_item(&ls_key_for_repo(repo_key), &raw);
        }
    }
}

/* ---------- module-level state (scoped to wasm thread) ---------- */

thread_local! {
    static REPO_KEY: RefCell<String> = RefCell::new(compute_default_repo_key());
    pub static TREE_STATE: RefCell<TreeState> = RefCell::new(TreeState::default());
}

/// Set the repo key and load persisted TreeState for it.
pub fn set_repo_key(repo_key: impl Into<String>) {
    let key = repo_key.into();
    REPO_KEY.with(|k| k.replace(key));
    let current = REPO_KEY.with(|k| k.borrow().clone());
    let ts = load_tree_state_from_ls(&current);
    TREE_STATE.with(|s| s.replace(ts));
}

/// Get the current repo key (useful for diagnostics/tests).
pub fn get_repo_key() -> String {
    REPO_KEY.with(|k| k.borrow().clone())
}

/// Persist the current TreeState.
pub fn save_treestate() {
    let key = get_repo_key();
    TREE_STATE.with(|s| save_tree_state_to_ls(&key, &s.borrow()));
}

/// Borrow TreeState immutably.
pub fn with_treestate<F: FnOnce(&TreeState)>(f: F) {
    TREE_STATE.with(|s| f(&*s.borrow()));
}

/// Borrow TreeState mutably and auto-persist afterwards.
pub fn with_treestate_mut<F: FnOnce(&mut TreeState)>(f: F) {
    TREE_STATE.with(|s| {
        f(&mut *s.borrow_mut());
    });
    save_treestate();
}

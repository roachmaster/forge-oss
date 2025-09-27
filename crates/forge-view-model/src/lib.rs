//! Forge view model — modular façade.
//! Re-export the stable data contracts used by the web UI and daemon.

pub mod header;
pub mod tree;
pub mod editor;
pub mod terminal;
pub mod status;
pub mod workbench;
pub mod protocol;
pub mod mock;

// Public API surface (nice ergonomics for downstream crates)
pub use header::HeaderVM;
pub use tree::{TreeVM, TreeNodeVM};
pub use editor::EditorVM;
pub use terminal::TerminalVM;
pub use status::StatusVM;
pub use workbench::WorkbenchVM;
pub use protocol::{ServerToClient, ClientIntent};

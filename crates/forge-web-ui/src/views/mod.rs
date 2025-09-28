// Central views module: re-export individual views for ergonomic imports.
pub mod header;
pub mod sidebar;
pub mod editor;
pub mod terminal;
pub mod status;

pub use header::HeaderView;
pub use sidebar::SidebarView;
pub use editor::EditorView;
pub use terminal::TerminalView;
pub use status::StatusView;

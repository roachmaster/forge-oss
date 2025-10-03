// orchestrator for rendering concerns
pub mod layout;
pub mod editor_mount;
pub mod interactions;
pub mod util;      // <-- add this

// re-export the public surface we want
pub use editor_mount::mount_or_update_editor;
pub use layout::render_views;

/// top-level entry point used by the app
pub fn render_all() {
    render_views();
    mount_or_update_editor();
    interactions::wire_all();
}

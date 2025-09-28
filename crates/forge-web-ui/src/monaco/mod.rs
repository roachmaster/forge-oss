//! Minimal Rust façade around Monaco Editor.

mod loader;
mod editor;
mod value;

pub use editor::ensure_editor_for;
pub use value::{set_value, get_value};

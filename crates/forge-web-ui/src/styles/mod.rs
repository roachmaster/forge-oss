use crate::globals::with_dom;

pub mod base;
pub mod subbar;
pub mod sidebar;
pub mod editor;
pub mod terminal;
pub mod header_status;
pub mod debug_helpers;

/// Concatenate all component CSS blocks.
///
/// Note: `subbar` now exposes a `css()` fn (not a const) because
/// we want to join split strings at runtime instead of using `concat!`.
pub fn all_css() -> String {
    [
        base::CSS,
        &subbar::css(), // <-- call the fn instead of const
        sidebar::CSS,
        editor::CSS,
        terminal::CSS,
        header_status::CSS,
        debug_helpers::CSS,
    ]
    .join("\n")
}

/// Inject global CSS once at startup.
pub fn inject() {
    let css = all_css();
    with_dom(|dom| dom.inject_global_css(&css));
}

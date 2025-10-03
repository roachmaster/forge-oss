// crates/forge-web-ui/src/render/interactions/mod.rs
pub mod sidebar;
pub mod toolbar;
pub mod actions;
pub mod dom;

/// Public entrypoint to wire all UI interactions.
pub fn wire_all() {
    web_sys::console::log_1(&"🧰 wire_all() begin".into());
    sidebar::wire();
    toolbar::wire();
    web_sys::console::log_1(&"🧰 wire_all() done".into());
}

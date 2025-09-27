// crates/forge-web-ui/src/lib.rs
use wasm_bindgen::prelude::*;

mod viewcore;
mod display;
mod views;
mod dom;
mod mock_display;

use display::WorkbenchDisplay;
use views::{HeaderView, SidebarView, EditorView, TerminalView, StatusView};
use dom::DomDriver;
use viewcore::View;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // In real app, controller fills this from WS snapshot/diffs.
    let display = WorkbenchDisplay::new_mock();

    // Render views (pure)
    let header = HeaderView.render(&display);
    let sidebar = SidebarView.render(&display);
    let editor = EditorView.render(&display);
    let terminal = TerminalView.render(&display);
    let status = StatusView.render(&display);

    // Apply to DOM (imperative)
    let dom = DomDriver::new();

dom.inject_global_css(r#"
  html, body {
    margin: 0;
    padding: 0;
    height: 100%;
    font-family: system-ui, -apple-system, Segoe UI, Roboto, sans-serif;
  }
  #app-shell {
    display: grid;
    grid-template-rows: auto 1fr auto auto;
    grid-template-columns: 100%;
    height: 100%;
  }
  #header { background: #222; color: #eee; padding: 0.5rem 1rem; }
  #body { display: grid; grid-template-columns: 250px 1fr; overflow: hidden; }
  #sidebar { background: #f7f7f7; border-right: 1px solid #ddd; overflow-y: auto; padding: 0.5rem; }
  #editor { overflow: auto; padding: 1rem; }
  #terminal { background: #111; color: #0f0; font-family: monospace; padding: 0.5rem 1rem; overflow-y: auto; }
  #status { background: #eee; padding: 0.25rem 1rem; font-size: 0.9rem; }
"#);
    dom.mount_clear("header", &header).map_err(|e| JsValue::from_str(&e))?;
    dom.mount_clear("sidebar", &sidebar).map_err(|e| JsValue::from_str(&e))?;
    dom.mount_clear("editor", &editor).map_err(|e| JsValue::from_str(&e))?;
    dom.mount_clear("terminal", &terminal).map_err(|e| JsValue::from_str(&e))?;
    dom.mount_clear("status", &status).map_err(|e| JsValue::from_str(&e))?;

    Ok(())
}

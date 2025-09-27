
use wasm_bindgen::prelude::*;

#[cfg(feature = "dev")]
use console_error_panic_hook as _;
mod tree_state;
mod forge_view_model { pub use forge_view_model::*; } // re-export for local modules
mod viewcore;
mod views;
mod dom;
mod display;
mod mock_display;

mod globals;
mod style;
mod render;
mod ws;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    #[cfg(feature = "dev")]
    console_error_panic_hook::set_once();

    // Seed UI with a disconnected mock status so users see something fast.
    globals::with_display_mut(|d| {
        let mut vm = mock_display::make_mock_vm();
        vm.status.msg = "Connecting to workbenchd…".into();
        vm.status.connected = false;
        d.apply_snapshot(vm);
    });

    style::inject_global_css();
    render::render_all();

    // Connect to the daemon (WS)
    ws::connect_ws();

    Ok(())
}

// crates/forge-web-ui/src/lib.rs
use wasm_bindgen::prelude::*;

// ---- Public facades ---------------------------------------------------------
pub mod monaco;

// ---- Internal modules -------------------------------------------------------
mod viewcore;
mod views;
mod dom;
mod display;
mod mock_display;
mod globals;
mod style;
mod render;
mod ws;
mod http;
mod tree_state; // <-- add this back so `globals` can re-export it

mod forge_view_model { pub use forge_view_model::*; }

#[cfg(feature = "dev")]
use console_error_panic_hook as _;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    #[cfg(feature = "dev")]
    console_error_panic_hook::set_once();

    globals::with_display_mut(|d| {
        let mut vm = mock_display::make_mock_vm();
        vm.status.msg = "Connecting to workbenchd…".into();
        vm.status.connected = false;
        d.apply_snapshot(vm);
    });

    style::inject_global_css();
    render::render_all();
    ws::connect_ws();
    Ok(())
}

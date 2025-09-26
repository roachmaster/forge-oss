use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, Window};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Better error messages on panic (debug builds)
    console_error_panic_hook::set_once();

    let window: Window = web_sys::window().ok_or("no window")?;
    let document: Document = window.document().ok_or("no document")?;
    let body: HtmlElement = document.body().ok_or("no body")?;

    body.set_inner_html("Hello, Forge Web UI!");
    Ok(())
}

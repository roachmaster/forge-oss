use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebSocket, Event};

use super::handlers::{make_onopen, make_onmessage};
use super::status::set_status;
use super::util::make_ws_url;
use crate::globals::set_ws;
use crate::render::render_all;

/// Connect to the daemon and initialize global WS.
pub fn connect_ws() {
    let ws_url = make_ws_url();
    let ws = match WebSocket::new(&ws_url) {
        Ok(ws) => ws,
        Err(err) => {
            web_sys::console::error_1(
                &format!("WS connect failed: {err:?} — using mock data").into()
            );
            set_status("WS connect failed; using mock data", false);
            render_all();
            return;
        }
    };

    // onopen
    {
        let onopen: Closure<dyn FnMut(Event)> = make_onopen();
        ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
        onopen.forget(); // keep alive for page lifetime
    }

    // onmessage
    {
        let onmessage = make_onmessage();
        ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget(); // keep alive
    }

    set_ws(ws);
}

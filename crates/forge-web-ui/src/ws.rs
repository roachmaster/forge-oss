use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, WebSocket, Event, MessageEvent};

use crate::forge_view_model::{ServerToClient, ClientIntent, WorkbenchVM};
use crate::globals::{with_display_mut, set_ws, with_ws};
use crate::render::render_all;

/// Connect to the daemon and initialize global WS.
pub fn connect_ws() {
    let ws_url = make_ws_url();
    let ws = match WebSocket::new(&ws_url) {
        Ok(ws) => ws,
        Err(_) => {
            set_status("WS connect failed; using mock data", false);
            render_all();
            return;
        }
    };

    // onopen
    {
        let onopen = Closure::<dyn FnMut(Event)>::wrap(Box::new(move |_e: Event| {
            set_status("Connected", true);
            render_all();
        }) as Box<dyn FnMut(_)>);
        ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
        onopen.forget();
    }

// onmessage: apply snapshot and rerender
{
    let onmessage = Closure::<dyn FnMut(MessageEvent)>::wrap(Box::new(move |e: MessageEvent| {
        if let Some(txt) = e.data().as_string() {
            if let Ok(msg) = serde_json::from_str::<ServerToClient>(&txt) {
                // Only variant for now; a plain `let` avoids the irrefutable `if let` warning.
                let ServerToClient::Snapshot { vm } = msg;
                with_display_mut(|d| d.apply_snapshot(vm));
                render_all();
            }
        }
    }) as Box<dyn FnMut(_)>);
    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();
}

    set_ws(ws);
}

/// Send an intent over WS (no-op if not connected).
pub fn send_intent(intent: ClientIntent) {
    if let Ok(payload) = serde_json::to_string(&intent) {
        with_ws(|ws| { let _ = ws.send_with_str(&payload); });
    }
}

fn make_ws_url() -> String {
    let win = window().unwrap();
    let loc = win.location();
    let host = loc.host().unwrap_or_else(|_| "127.0.0.1:8080".into());
    let host_only = host.split(':').next().unwrap_or("127.0.0.1");
    format!("ws://{}:8787/ws", host_only)
}

/// Update only the status slice and keep the rest intact.
pub fn set_status(msg: &str, connected: bool) {
    with_display_mut(|d| {
        let mut vm = WorkbenchVM::default();
        vm.header = d.header.clone();
        vm.tree = d.tree.clone();
        vm.editor = d.editor.clone();
        vm.terminal = d.terminal.clone();
        vm.status.msg = msg.to_string();
        vm.status.connected = connected;
        d.apply_snapshot(vm);
    });
}

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, MessageEvent};

use crate::forge_view_model::{ServerToClient, WorkbenchVM};
use crate::globals::with_display_mut;
use crate::render::render_all;

/// Make `onopen` callback.
pub fn make_onopen() -> Closure<dyn FnMut(Event)> {
    Closure::<dyn FnMut(Event)>::wrap(Box::new(move |_e: Event| {
        web_sys::console::log_1(&"WS: connected".into());
        // We don’t call set_status() here to avoid circular import;
        // render pipeline will usually refresh status on first snapshot.
        render_all();
    }))
}

/// Make `onmessage` callback.
pub fn make_onmessage() -> Closure<dyn FnMut(MessageEvent)> {
    Closure::<dyn FnMut(MessageEvent)>::wrap(Box::new(move |e: MessageEvent| {
        if let Some(txt) = e.data().as_string() {
            match serde_json::from_str::<ServerToClient>(&txt) {
                Ok(msg) => handle_server_msg(msg),
                Err(err) => web_sys::console::error_1(
                    &format!("WS: bad JSON from server: {err:?}").into()
                ),
            }
        } else {
            web_sys::console::warn_1(&"WS: non-text frame ignored".into());
        }
    }))
}

fn handle_server_msg(msg: ServerToClient) {
    match msg {
        ServerToClient::Snapshot { mut vm } => {
            web_sys::console::log_1(&"WS: Snapshot received".into());
            // Don’t stomp the editor if snapshot carries no editor payload.
            with_display_mut(|d| {
                if vm.editor.file_path.is_empty() && vm.editor.content.is_empty() {
                    vm.editor = d.editor.clone();
                }
                d.apply_snapshot(vm);
            });
            render_all();
        }

        ServerToClient::FileOpened {
            path, content, size_bytes, char_count, line_count, sha256
        } => {
            web_sys::console::log_1(
                &format!("WS: FileOpened path={path} size={size_bytes} sha={sha256}").into()
            );

            with_display_mut(|d| {
                let mut vm = WorkbenchVM::default();
                vm.header   = d.header.clone();
                vm.tree     = d.tree.clone();
                vm.terminal = d.terminal.clone();
                vm.status   = d.status.clone();

                vm.editor = d.editor.clone();
                vm.editor.file_path  = path;
                vm.editor.content    = content;
                vm.editor.size_bytes = size_bytes as usize;
                vm.editor.char_count = char_count;
                vm.editor.line_count = line_count;
                vm.editor.sha256     = sha256;
                vm.editor.is_dirty   = false;

                d.apply_snapshot(vm);
            });
            render_all(); // triggers Monaco mount/update
        }

        ServerToClient::FileUnchanged {
            path, size_bytes, char_count, line_count, sha256
        } => {
            web_sys::console::log_1(
                &format!("WS: FileUnchanged path={path} (metadata only)").into()
            );

            with_display_mut(|d| {
                let mut vm = WorkbenchVM::default();
                vm.header   = d.header.clone();
                vm.tree     = d.tree.clone();
                vm.terminal = d.terminal.clone();
                vm.status   = d.status.clone();

                vm.editor = d.editor.clone(); // keep content
                vm.editor.file_path  = path;
                vm.editor.size_bytes = size_bytes as usize;
                vm.editor.char_count = char_count;
                vm.editor.line_count = line_count;
                vm.editor.sha256     = sha256;

                d.apply_snapshot(vm);
            });
            render_all(); // ensure editor host is present
        }
    }
}

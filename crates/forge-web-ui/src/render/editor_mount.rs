// crates/forge-web-ui/src/render/editor_mount.rs
use wasm_bindgen::prelude::*;
use crate::viewcore::Display;               // needed so d.editor() is a method
use crate::globals::with_display;
use crate::render::util::language_from_path; // <-- add this

pub fn mount_or_update_editor() {
    let (lang, content) = {
        let mut lang = String::new();
        let mut content = String::new();
        with_display(|d| {
            let e = d.editor();
            lang = language_from_path(&e.file_path);
            content = e.content.clone();
        });
        (lang, content)
    };

    wasm_bindgen_futures::spawn_local(async move {
        crate::monaco::ensure_editor_for("#editor-host", &lang, &content).await;
    });
}

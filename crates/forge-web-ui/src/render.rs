use wasm_bindgen::prelude::*;
use crate::viewcore::{Display, View};
use crate::views::{HeaderView, SidebarView, EditorView, TerminalView, StatusView};
use crate::globals::{with_display, with_dom};

pub fn render_all() {
    render_views();
    // Mount/refresh editor after nodes are in the DOM
    mount_or_update_editor();
    interactions::wire_all();
}

fn render_views() {
    with_display(|d| {
        let header   = HeaderView.render(d);
        let sidebar  = SidebarView.render(d);
        let editor   = EditorView.render(d);
        let terminal = TerminalView.render(d);
        let status   = StatusView.render(d);

        with_dom(|dom| {
            let _ = dom.mount_clear("header",   &header);
            let _ = dom.mount_clear("sidebar",  &sidebar);
            let _ = dom.mount_clear("editor",   &editor);
            let _ = dom.mount_clear("terminal", &terminal);
            let _ = dom.mount_clear("status",   &status);
        });
    });
}
fn mount_or_update_editor() {
    // Pull owned copies out of the display so the async task is 'static.
    let mut lang = String::new();
    let mut content = String::new();

    with_display(|d| {
        let e = d.editor();
        lang = language_from_path(&e.file_path);
        content = e.content.clone();
    });

    // Fire-and-forget future with owned data (no borrows → 'static)
    wasm_bindgen_futures::spawn_local(async move {
        crate::monaco::ensure_editor_for("#editor-host", &lang, &content).await;
    });
}

fn language_from_path(path: &str) -> String {
    let ext = path.rsplit('.').next().unwrap_or("");
    match ext {
        "rs"   => "rust",
        "ts"   => "typescript",
        "tsx"  => "typescript",
        "js"   => "javascript",
        "jsx"  => "javascript",
        "toml" => "toml",
        "json" => "json",
        "md"   => "markdown",
        "html" => "html",
        "css"  => "css",
        _      => "plaintext",
    }.to_string()
}

mod interactions {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{window, Element, Event};
    use crate::viewcore::Display;
    use crate::forge_view_model::ClientIntent;
    use crate::globals::{with_treestate_mut, with_display};
    use crate::ws::send_intent;

    pub fn wire_all() {
        wire_sidebar_clicks();
    }

    fn wire_sidebar_clicks() {
        let doc = match window().and_then(|w| w.document()) { Some(d) => d, None => return };
        let Some(sidebar) = doc.get_element_by_id("sidebar") else { return };

        if sidebar.get_attribute("data-bound").as_deref() == Some("1") { return; }
        let _ = sidebar.set_attribute("data-bound", "1");

        let closure = Closure::<dyn FnMut(Event)>::wrap(Box::new(move |e: Event| {
            let el: Element = match e.target().and_then(|t| t.dyn_into::<Element>().ok()) {
                Some(el) => el, None => return,
            };
            let li = if el.tag_name().eq_ignore_ascii_case("li") { el } else {
                match el.closest("li") { Ok(Some(li)) => li, _ => return }
            };

            let path = match li.get_attribute("data-path") {
                Some(p) if !p.is_empty() => p, _ => return,
            };
            let is_dir = li.get_attribute("data-dir").as_deref() == Some("true");

            if is_dir {
                let mut new_open = false;
                with_treestate_mut(|ts| { new_open = ts.toggle(&path); });
                send_intent(ClientIntent::ToggleDir { path, open: new_open });
                e.stop_propagation();
                crate::render::render_all();
                return;
            }

            // File: include currently known editor sha
            let known_sha: Option<String> = {
                let mut s = None;
                with_display(|d| {
                    let e = d.editor();
                    if !e.sha256.is_empty() { s = Some(e.sha256.clone()); }
                });
                s
            };
            send_intent(ClientIntent::OpenFile { path, known_sha });
        }) as Box<dyn FnMut(_)>);

        let _ = sidebar.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

// crates/forge-web-ui/src/render/interactions/sidebar.rs
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, console};

use crate::forge_view_model::ClientIntent;
use crate::globals::{with_display, with_treestate_mut};
use crate::viewcore::Display;
use crate::ws::send_intent;

use super::dom::{document, already_bound, mark_bound, add_click_listener};

pub fn wire() {
    console::log_1(&"🧷 wire_sidebar_clicks()".into());
    let Some(doc) = document() else {
        console::warn_1(&"⚠️ wire_sidebar_clicks: no document".into());
        return;
    };
    let Some(sidebar) = doc.get_element_by_id("sidebar") else {
        console::warn_1(&"⚠️ wire_sidebar_clicks: #sidebar not found".into());
        return;
    };
    if already_bound(&sidebar) {
        console::log_1(&"↩️ wire_sidebar_clicks: already bound, skipping".into());
        return;
    }
    mark_bound(&sidebar);
    console::log_1(&"✅ wire_sidebar_clicks: binding click delegate".into());

    add_click_listener(&sidebar, move |e: &Event| {
        console::log_1(&"🖱️ sidebar click".into());
        let Some(target) = e.target() else { return };
        let Ok(el) = target.dyn_into::<Element>() else { return };

        // Find the <li>
        let li = if el.tag_name().eq_ignore_ascii_case("li") {
            el
        } else {
            match el.closest("li") { Ok(Some(li)) => li, _ => return }
        };

        let path = match li.get_attribute("data-path") {
            Some(p) if !p.is_empty() => p,
            _ => return,
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

        // File: include known editor sha if available
        let known_sha: Option<String> = {
            let mut s = None;
            with_display(|d| {
                let e = d.editor();
                if !e.sha256.is_empty() { s = Some(e.sha256.clone()); }
            });
            s
        };
        send_intent(ClientIntent::OpenFile { path, known_sha });
    });
}

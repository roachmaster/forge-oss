// crates/forge-web-ui/src/render/interactions/toolbar.rs
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, console};

use super::dom::{document, already_bound, mark_bound, add_click_listener, find_with_attr};
use super::actions::dispatch_toolbar_action;

pub fn wire() {
    console::log_1(&"🧷 wire_subbar_actions()".into());
    let Some(doc) = document() else {
        console::warn_1(&"⚠️ wire_subbar_actions: no document".into());
        return;
    };
    let Some(subbar) = doc.get_element_by_id("subbar") else {
        console::warn_1(&"⚠️ wire_subbar_actions: #subbar not found".into());
        return;
    };
    if already_bound(&subbar) {
        console::log_1(&"↩️ wire_subbar_actions: already bound, skipping".into());
        return;
    }
    mark_bound(&subbar);
    console::log_1(&"✅ wire_subbar_actions: binding click delegate".into());

    add_click_listener(&subbar, move |e: &Event| {
        console::log_1(&"🖱️ subbar click".into());
        let Some(target) = e.target() else { return };
        let Ok(el) = target.dyn_into::<Element>() else { return };

        let Some(action_el) = find_with_attr(el, "data-action") else { return };
        let action = action_el.get_attribute("data-action").unwrap_or_default();
        if action.is_empty() { return; }

        dispatch_toolbar_action(action.as_str());
    });
}

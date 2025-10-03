// crates/forge-web-ui/src/render/interactions/dom.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, Event, console};

use crate::globals::with_display;
use crate::viewcore::Display; // needed for d.editor()

pub fn document() -> Option<Document> { window()?.document() }

pub fn already_bound(el: &Element) -> bool {
    let b = el.get_attribute("data-bound").as_deref() == Some("1");
    console::log_1(&format!("• already_bound? -> {b}").into());
    b
}

pub fn mark_bound(el: &Element) {
    let _ = el.set_attribute("data-bound", "1");
    console::log_1(&"• mark_bound(data-bound=1)".into());
}

pub fn add_click_listener<F>(el: &Element, handler: F)
where
    F: Fn(&Event) + 'static,
{
    console::log_1(&"• add_click_listener()".into());
    let cb = Closure::<dyn FnMut(Event)>::wrap(Box::new(move |e: Event| handler(&e)) as Box<dyn FnMut(_)>);
    let _ = el.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
    console::log_1(&"• add_click_listener: bound".into());
}

pub fn find_with_attr(mut el: Element, attr: &str) -> Option<Element> {
    console::log_1(&format!("• find_with_attr(start='{}', attr='{}')", el.tag_name(), attr).into());
    loop {
        if el.has_attribute(attr) {
            console::log_1(&format!("  ↳ found on <{}>", el.tag_name()).into());
            return Some(el);
        }
        match el.parent_element() {
            Some(parent) => {
                el = parent;
                console::log_1(&format!("  ↻ ascend to <{}>", el.tag_name()).into());
            }
            None => {
                console::log_1(&"  ✗ reached root without attr".into());
                return None;
            }
        }
    }
}

pub fn toggle_body_class(class_name: &str) {
    console::log_1(&format!("• toggle_body_class('{}')", class_name).into());
    let Some(doc) = document() else { console::warn_1(&"⚠️ toggle_body_class: no document".into()); return };
    let Some(body) = doc.body() else { console::warn_1(&"⚠️ toggle_body_class: no body".into()); return };

    // classList not enabled; emulate by rewriting className
    let current = body.class_name();
    console::log_1(&format!("  current='{}'", current).into());
    let mut parts: Vec<String> = current.split_whitespace().map(|s| s.to_string()).collect();

    if parts.iter().any(|c| c == class_name) {
        console::log_1(&"  -> removing".into());
        parts.retain(|c| c != class_name);
    } else {
        console::log_1(&"  -> adding".into());
        parts.push(class_name.to_string());
    }
    let newc = parts.join(" ");
    console::log_1(&format!("  new='{}'", newc).into());
    body.set_class_name(&newc);
}

pub fn snapshot_editor_path() -> String {
    let mut path = String::new();
    with_display(|d| { path = d.editor().file_path.clone(); });
    path
}

pub fn display_or_dash(s: &str) -> &str {
    if s.is_empty() { "—" } else { s }
}

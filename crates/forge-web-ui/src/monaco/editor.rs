use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, Node, console};
use js_sys::{Function, Object, Reflect};

use super::loader::wait_ready;
use super::value::update_editor_value_and_language;

// Global keys
const EDITOR_KEY: &str = "__forgeMonacoEditor";
const CONTAINER_KEY: &str = "__forgeMonacoContainer";

/// Return (editor, container) if both are set and valid.
fn get_editor_and_container() -> Option<(JsValue, HtmlElement)> {
    let win = window()?;
    let editor = Reflect::get(&win, &JsValue::from_str(EDITOR_KEY)).ok()?;
    if editor.is_undefined() || editor.is_null() {
        return None;
    }
    let container_val = Reflect::get(&win, &JsValue::from_str(CONTAINER_KEY)).ok()?;
    if container_val.is_undefined() || container_val.is_null() {
        return None;
    }
    let container: HtmlElement = container_val.dyn_into().ok()?;
    Some((editor, container))
}

/// Persist (editor, container) in `window`.
fn set_editor_and_container(editor: &JsValue, container: &HtmlElement) {
    if let Some(win) = window() {
        let _ = Reflect::set(&win, &JsValue::from_str(EDITOR_KEY), editor);
        let _ = Reflect::set(&win, &JsValue::from_str(CONTAINER_KEY), container);
    }
}

/// Dispose an existing editor instance.
fn dispose_editor(editor: &JsValue) {
    if let Ok(dispose_fn) = Reflect::get(editor, &JsValue::from_str("dispose")).and_then(|f| f.dyn_into::<Function>()) {
        let _ = dispose_fn.call0(editor);
    }
}

/// Compare two elements by identity (same underlying DOM node?)
fn same_node(a: &HtmlElement, b: &HtmlElement) -> bool {
    let an: &Node = a.as_ref();
    let bn: &Node = b.as_ref();
    an.is_same_node(Some(bn))
}

/// Ensure Monaco is mounted in `container_id`. If an editor exists and the
/// container matches, update value/language. Otherwise dispose & recreate.
pub async fn ensure_editor_for(container_id: &str, language: &str, value: &str) {
    console::log_1(
        &format!("🪵 ensure_editor_for: id={container_id} lang={language} len={}", value.len()).into()
    );

    wait_ready().await;
    console::log_1(&"🪵 Monaco loader ready".into());

    // Current live container in the DOM
    let Some(el) = window()
        .and_then(|w| w.document())
        .and_then(|d| d.query_selector(container_id).ok().flatten())
    else {
        console::error_1(&format!("⚠️ container '{}' not found", container_id).into());
        return;
    };
    let live_container: HtmlElement = match el.dyn_into() {
        Ok(c) => c,
        Err(_) => {
            console::error_1(&"⚠️ container element is not an HtmlElement".into());
            return;
        }
    };

    // If we already have an editor, check whether it's still attached to the same container.
    if let Some((existing_editor, existing_container)) = get_editor_and_container() {
        if same_node(&existing_container, &live_container) {
            // Same container: just update.
            console::log_1(&"🪵 Updating existing editor".into());
            update_editor_value_and_language(&existing_editor, language, value);
            return;
        } else {
            // Different (or replaced) container: dispose and recreate.
            console::log_1(&"🧹 Disposing editor bound to old container".into());
            dispose_editor(&existing_editor);
        }
    } else {
        console::log_1(&"🪵 Creating new Monaco editor".into());
    }

    // Create a new editor bound to the current live container.
    let win = window().unwrap();
    let monaco  = Reflect::get(&win, &JsValue::from_str("monaco")).unwrap();
    let editor  = Reflect::get(&monaco, &JsValue::from_str("editor")).unwrap();
    let create_fn: Function = Reflect::get(&editor, &JsValue::from_str("create"))
        .unwrap()
        .dyn_into()
        .unwrap();

    let opts = Object::new();
    let _ = Reflect::set(&opts, &JsValue::from_str("value"),    &JsValue::from_str(value));
    let _ = Reflect::set(&opts, &JsValue::from_str("language"), &JsValue::from_str(language));
    let _ = Reflect::set(&opts, &JsValue::from_str("automaticLayout"), &JsValue::from_bool(true));
    {
        let minimap = Object::new();
        let _ = Reflect::set(&minimap, &JsValue::from_str("enabled"), &JsValue::from_bool(false));
        let _ = Reflect::set(&opts, &JsValue::from_str("minimap"), &minimap);
    }

    let editor_instance = create_fn
        .call2(&editor, &live_container, &opts)
        .expect("monaco.editor.create failed");

    // Stash editor + its container so we can tell if the DOM element was replaced later.
    set_editor_and_container(&editor_instance, &live_container);

    // Fallback height if CSS didn't set one.
    if live_container
        .style()
        .get_property_value("height")
        .unwrap_or_default()
        .is_empty()
    {
        let _ = live_container.style().set_property("height", "100%");
    }

    console::log_1(&"✅ Monaco editor (re)mounted".into());
}

use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::JsCast;
use web_sys::{window, console};
use js_sys::{Function, Reflect};

/// Call `editor.layout()` if available.
/// Monaco sometimes needs an explicit layout after the container size settles.
fn layout_editor(editor: &JsValue) {
    if let Ok(layout_fn) = Reflect::get(editor, &JsValue::from_str("layout"))
        .and_then(|f| f.dyn_into::<Function>())
    {
        let _ = layout_fn.call0(editor);
        console::log_1(&"🪵 monaco: layout() invoked".into());
    }
}

/// Update an existing editor's language and text.
/// Falls back to `editor.setValue` if there's no model yet.
pub(super) fn update_editor_value_and_language(editor: &JsValue, language: &str, value: &str) {
    let win = window().unwrap();
    let monaco    = Reflect::get(&win, &JsValue::from_str("monaco")).unwrap();
    let editor_ns = Reflect::get(&monaco, &JsValue::from_str("editor")).unwrap();

    // model = editor.getModel()
    let get_model_fn: Function = Reflect::get(editor, &JsValue::from_str("getModel"))
        .unwrap()
        .dyn_into()
        .unwrap();
    let model = get_model_fn.call0(editor).unwrap();

    if model.is_undefined() || model.is_null() {
        // No model yet: just set the value directly on the editor
        let set_value_fn: Function = Reflect::get(editor, &JsValue::from_str("setValue"))
            .unwrap()
            .dyn_into()
            .unwrap();
        let _ = set_value_fn.call1(editor, &JsValue::from_str(value));
        layout_editor(editor);
        return;
    }

    // monaco.editor.setModelLanguage(model, language)
    let set_lang_fn: Function = Reflect::get(&editor_ns, &JsValue::from_str("setModelLanguage"))
        .unwrap()
        .dyn_into()
        .unwrap();
    let _ = set_lang_fn.call2(&editor_ns, &model, &JsValue::from_str(language));

    // model.setValue(value)
    let set_value_fn: Function = Reflect::get(&model, &JsValue::from_str("setValue"))
        .unwrap()
        .dyn_into()
        .unwrap();
    let _ = set_value_fn.call1(&model, &JsValue::from_str(value));

    layout_editor(editor);
}

/// Set editor value if mounted.
pub fn set_value(value: &str) {
    if let Some(win) = window() {
        if let Ok(editor) = Reflect::get(&win, &JsValue::from_str("__forgeMonacoEditor")) {
            if !editor.is_undefined() && !editor.is_null() {
                let set_value_fn: Function = Reflect::get(&editor, &JsValue::from_str("setValue"))
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                let _ = set_value_fn.call1(&editor, &JsValue::from_str(value));
                layout_editor(&editor);
            }
        }
    }
}

/// Get editor value if mounted.
pub fn get_value() -> Option<String> {
    let win = window()?;
    let editor = Reflect::get(&win, &JsValue::from_str("__forgeMonacoEditor")).ok()?;
    if editor.is_undefined() || editor.is_null() {
        return None;
    }
    let get_value_fn: Function = Reflect::get(&editor, &JsValue::from_str("getValue"))
        .ok()?
        .dyn_into()
        .ok()?;
    let v = get_value_fn.call0(&editor).ok()?;
    v.as_string()
}

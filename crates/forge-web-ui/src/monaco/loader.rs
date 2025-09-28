use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use web_sys::{window, Document, Window};
use js_sys::{Array, Function, Object, Promise, Reflect};

const MONACO_LOADER_URL: &str =
    "https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.49.0/min/vs/loader.min.js";
const MONACO_BASE_URL: &str =
    "https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.49.0/min";

/* ---------- small helpers ---------- */

fn doc() -> Option<Document> { window()?.document() }

fn has_global(win: &Window, key: &str) -> bool {
    Reflect::has(win, &JsValue::from_str(key)).unwrap_or(false)
}

fn set_global(win: &Window, key: &str, val: &JsValue) {
    let _ = Reflect::set(win, &JsValue::from_str(key), val);
}

fn get_global(win: &Window, key: &str) -> Option<JsValue> {
    Reflect::get(win, &JsValue::from_str(key)).ok()
}

/* Configure AMD loader path: require.config({ paths: { vs: MONACO_BASE_URL + "/vs" }}) */
fn configure_amd(win: &Window) {
    let require = get_global(win, "require").unwrap();
    let config_fn: Function = Reflect::get(&require, &JsValue::from_str("config"))
        .unwrap()
        .dyn_into()
        .unwrap();

    let paths = Object::new();
    let _ = Reflect::set(
        &paths,
        &JsValue::from_str("vs"),
        &JsValue::from_str(&(String::from(MONACO_BASE_URL) + "/vs")),
    );

    let cfg = Object::new();
    let _ = Reflect::set(&cfg, &JsValue::from_str("paths"), &paths);

    let _ = config_fn.call1(&require, &cfg);
}

/* require(['vs/editor/editor.main'], resolve) */
fn require_editor_main(win: &Window, resolve: &Function) {
    let require = get_global(win, "require").unwrap();
    let require_fn: Function = require.dyn_into().unwrap();

    let deps = Array::new();
    deps.push(&JsValue::from_str("vs/editor/editor.main"));

    // Closure that calls resolve()
    let cb = Closure::<dyn FnMut()>::new({
        let resolve = resolve.clone();
        move || {
            let _ = resolve.call0(&JsValue::NULL);
        }
    });

    let _ = require_fn.call2(&JsValue::NULL, &deps, cb.as_ref().unchecked_ref());
    cb.forget();
}

/* Inject <script src=MONACO_LOADER_URL> and run a callback on load */
fn inject_loader_and_onload<F: 'static + FnMut()>(onload_cb: F) {
    let Some(d) = doc() else { return };
    let Some(body) = d.body() else { return };

    let script = d.create_element("script").unwrap();
    let _ = script.set_attribute("src", MONACO_LOADER_URL);

    let onload = Closure::<dyn FnMut()>::new(onload_cb);
    let _ = script.add_event_listener_with_callback("load", onload.as_ref().unchecked_ref());
    onload.forget();

    let _ = body.append_child(&script);
}

/* ---------- public bootstrap API ---------- */

/// Create `window.__forgeMonacoReady` once. It resolves after Monaco is loaded.
pub fn ensure_loader_and_bootstrap() {
    let Some(win) = window() else { return };

    // Already initialized?
    if has_global(&win, "__forgeMonacoReady") {
        return;
    }

    // Promise executor signature: (resolve, reject)
    let mut executor = move |resolve: Function, _reject: Function| {
        let w = window().unwrap();

        // If AMD loader is already there, configure + require immediately.
        if has_global(&w, "require") {
            configure_amd(&w);
            require_editor_main(&w, &resolve);
            return;
        }

        // Otherwise inject loader <script>, then configure + require on load.
        inject_loader_and_onload({
            let resolve = resolve.clone();
            move || {
                let w_after = window().unwrap();
                configure_amd(&w_after);
                require_editor_main(&w_after, &resolve);
            }
        });
    };

    let promise = Promise::new(&mut executor);
    set_global(&win, "__forgeMonacoReady", &promise.into());
}

/// Await the global ready promise (created by `ensure_loader_and_bootstrap`).
pub async fn wait_ready() {
    ensure_loader_and_bootstrap();
    let win = window().unwrap();
    let p = get_global(&win, "__forgeMonacoReady").unwrap();
    let promise: Promise = p.dyn_into().unwrap();
    let _ = JsFuture::from(promise).await;
}

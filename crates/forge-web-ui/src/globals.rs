use std::cell::RefCell;

use web_sys::WebSocket;

use crate::dom::DomDriver;
use crate::display::WorkbenchDisplay;

/// DISPLAY / DOM / WS singletons (keep this file focused on these).
thread_local! {
    pub static DISPLAY: RefCell<WorkbenchDisplay> = RefCell::new(WorkbenchDisplay::new_empty());
    pub static DOM: DomDriver = DomDriver::new();
    pub static WS: RefCell<Option<WebSocket>> = RefCell::new(None);
}

/// Utility: run a closure with a mutable reference to the display.
pub fn with_display_mut<F: FnOnce(&mut WorkbenchDisplay)>(f: F) {
    DISPLAY.with(|d| f(&mut *d.borrow_mut()));
}

/// Utility: run a closure with an immutable reference to the display.
pub fn with_display<F: FnOnce(&WorkbenchDisplay)>(f: F) {
    DISPLAY.with(|d| f(&*d.borrow()));
}

/// Utility: run a closure with the DomDriver.
pub fn with_dom<F: FnOnce(&DomDriver)>(f: F) {
    DOM.with(|dom| f(dom));
}

/// Install the WS handle globally (overwrite any prior).
pub fn set_ws(ws: WebSocket) {
    WS.with(|cell| {
        cell.replace(Some(ws));
    });
}

/// Borrow the WS handle if present.
pub fn with_ws<F: FnOnce(&WebSocket)>(f: F) {
    WS.with(|cell| {
        if let Some(ws) = cell.borrow().as_ref() {
            f(ws);
        }
    });
}

/* ---------- Re-export tree state helpers ---------- */

pub use crate::tree_state::{
    TreeState,
    with_treestate,
    with_treestate_mut,
    set_repo_key,
    get_repo_key,
    save_treestate,
};

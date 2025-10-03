// crates/forge-web-ui/src/render/interactions/actions.rs
use web_sys::window;
use web_sys::console;

use crate::ws::set_status;
use crate::monaco::value::get_value;

use super::dom::{toggle_body_class, snapshot_editor_path, display_or_dash};

pub fn dispatch_toolbar_action(action: &str) {
    console::log_1(&format!("🧭 dispatch_toolbar_action('{action}')").into());
    match action {
        // Top menu buttons (left side)
        "menu-file" => {
            let file = snapshot_editor_path();
            console::log_1(&format!("• menu-file for current='{}'", file).into());
            set_status(&format!("File menu (coming soon) — current file: {}", display_or_dash(&file)), true);
            crate::render::render_all();
        }
        "menu-edit" => {
            console::log_1(&"• menu-edit".into());
            set_status("Edit menu (coming soon)…", true);
            crate::render::render_all();
        }
        "menu-view" => {
            console::log_1(&"• menu-view".into());
            set_status("View menu (coming soon)…", true);
            crate::render::render_all();
        }
        "menu-window" => {
            console::log_1(&"• menu-window".into());
            set_status("Window menu (coming soon)…", true);
            crate::render::render_all();
        }
        "menu-help" => {
            console::log_1(&"• menu-help".into());
            set_status("Help menu (coming soon)…", true);
            crate::render::render_all();
        }

        // Quick actions (right side)
        "view-toggle-sidebar"  => toggle_body_class("hide-sidebar"),
        "view-toggle-terminal" => toggle_body_class("hide-terminal"),
        "view-reload" => {
            if let Some(win) = window() {
                let _ = win.location().reload();
            }
        }

        // Demo save
        "file-save" => {
            let content = get_value().unwrap_or_default();
            let path = snapshot_editor_path();
            console::log_1(&format!("• save: path='{}' bytes={}", path, content.len()).into());
            set_status(
                &format!("(demo) Save requested: {} ({} bytes)", display_or_dash(&path), content.len()),
                true,
            );
            crate::render::render_all();
            // TODO: send actual save intent in the future
        }

        // About
        "help-about" => {
            if let Some(win) = window() {
                let _ = win.alert_with_message("Forge Workbench — early preview");
            }
        }

        // Placeholders (keep console clean)
        "view-menu" | "edit-placeholder" | "window-placeholder" => { /* no-op */ }

        // Unknown
        other => console::log_1(&format!("⚠️ Unknown toolbar action: {other}").into()),
    }
}

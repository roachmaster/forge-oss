use crate::viewcore::{Display, View, VNode};

pub struct ToolbarView;

impl<D: Display> View<D> for ToolbarView {
    fn render(&self, d: &D) -> VNode {
        let s = d.status();
        let (conn_label, conn_class) = if s.connected {
            ("Connected", "conn conn-ok")
        } else {
            ("Offline", "conn conn-bad")
        };

        VNode::el("div")
            .with_attr("class", "toolbar")
            .with_child(
                VNode::el("div")
                    .with_attr("class", "toolbar-left")
                    // File menu with dropdown
                    .with_child(menu_with_dropdown("File", "menu-file", vec![
                        ("Save", "file-save"),
                        ("Open Recent", "file-open-recent"),
                    ]))
                    .with_child(menu_with_dropdown("Edit", "menu-edit", vec![
                        ("Undo", "edit-undo"),
                        ("Redo", "edit-redo"),
                    ]))
                    .with_child(menu_with_dropdown("View", "menu-view", vec![
                        ("Toggle Sidebar", "view-toggle-sidebar"),
                        ("Toggle Terminal", "view-toggle-terminal"),
                    ]))
                    .with_child(menu_with_dropdown("Window", "menu-window", vec![
                        ("Reload", "view-reload"),
                    ]))
                    .with_child(menu_with_dropdown("Help", "menu-help", vec![
                        ("About", "help-about"),
                    ])),
            )
            .with_child(
                VNode::el("div")
                    .with_attr("class", "toolbar-right")
                    .with_child(
                        VNode::el("span")
                            .with_attr("class", conn_class)
                            .with_child(VNode::text(conn_label)),
                    ),
            )
    }
}

/// Helper: create a menu button with dropdown items
fn menu_with_dropdown(label: &str, id: &str, items: Vec<(&str, &str)>) -> VNode {
    let mut container = VNode::el("div").with_attr("class", "menu-container");
    let mut button = VNode::el("button")
        .with_attr("id", id)
        .with_attr("class", "menu-btn")
        .with_attr("data-action", id)
        .with_child(VNode::text(label));
    let mut dropdown = VNode::el("div").with_attr("class", "menu-dropdown");

    for (text, action) in items {
        dropdown = dropdown.with_child(
            VNode::el("button")
                .with_attr("data-action", action)
                .with_child(VNode::text(text)),
        );
    }

    container = container.with_child(button).with_child(dropdown);
    container
}

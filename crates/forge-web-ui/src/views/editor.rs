use web_sys::console;
use forge_view_model::EditorVM;
use crate::viewcore::{VNode, View, Display};

pub struct EditorView;

impl<D: Display> View<D> for EditorView {
    fn render(&self, d: &D) -> VNode {
        let e: &EditorVM = d.editor();
        console::log_1(&format!("🪵 EditorView.render: file='{}' len={}", e.file_path, e.content.len()).into());

        VNode::el("section")
            .with_attr("data-role", "editor")
            .with_child(
                VNode::el("div")
                    .with_attr("class", "path")
                    .with_child(VNode::text(if e.file_path.is_empty() { "— no file —" } else { &e.file_path })),
            )
            .with_child(
                VNode::el("div")
                    .with_attr("id", "editor-host")
                    .with_attr("class", "editor-host")
            )
    }
}

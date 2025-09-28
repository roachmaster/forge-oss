use crate::viewcore::{VNode, View, Display};

pub struct EditorView;

impl<D: Display> View<D> for EditorView {
    fn render(&self, d: &D) -> VNode {
        let e = d.editor();
        VNode::el("section")
            .with_attr("data-role", "editor")
            .with_child(
                VNode::el("div")
                    .with_attr("class", "path")
                    .with_child(VNode::text(&e.file_path)),
            )
            .with_child(
                VNode::el("pre")
                    .with_attr("class", "code")
                    .with_child(VNode::text(&e.content)),
            )
    }
}

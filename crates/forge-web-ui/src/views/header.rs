use forge_view_model::HeaderVM;
use crate::viewcore::{VNode, View, Display};

pub struct HeaderView;

impl<D: Display> View<D> for HeaderView {
    fn render(&self, d: &D) -> VNode {
        let h: &HeaderVM = d.header();
        VNode::el("div")
            .with_attr("data-role", "header")
            .with_child(
                VNode::el("span")
                    .with_attr("class", "title")
                    .with_child(VNode::text(&h.title)),
            )
            .with_child(
                VNode::el("div")
                    .with_attr("class", "actions")
                    .with_child(
                        VNode::el("button")
                            .with_attr("data-intent", "Build")
                            .with_attr("disabled", (!h.can_build).to_string()),
                    )
                    .with_child(
                        VNode::el("button")
                            .with_attr("data-intent", "Run")
                            .with_attr("disabled", (!h.can_run).to_string()),
                    ),
            )
    }
}

use crate::viewcore::{VNode, View, Display};

pub struct StatusView;

impl<D: Display> View<D> for StatusView {
    fn render(&self, d: &D) -> VNode {
        let s = d.status();
        VNode::el("div")
            .with_attr("data-role", "status")
            .with_attr("data-connected", s.connected.to_string())
            .with_child(VNode::text(&s.msg))
    }
}

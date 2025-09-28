use crate::viewcore::{VNode, View, Display};

pub struct TerminalView;

impl<D: Display> View<D> for TerminalView {
    fn render(&self, d: &D) -> VNode {
        let t = d.terminal();
        let mut pre = VNode::el("pre").with_attr("data-role", "terminal");
        for line in &t.lines {
            pre = pre.with_child(VNode::text(format!("{line}\n")));
        }
        pre
    }
}

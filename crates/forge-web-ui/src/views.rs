// crates/forge-web-ui/src/views.rs
use crate::viewcore::{VNode, View, Display};
use crate::vm::*;

pub struct HeaderView;
impl<D: Display> View<D> for HeaderView {
    fn render(&self, d: &D) -> VNode {
        let h = d.header();
        VNode::el("div")
            .with_attr("data-role", "header")
            .with_child(VNode::el("span").with_attr("class", "title")
                       .with_child(VNode::text(&h.title)))
            .with_child(VNode::el("div").with_attr("class", "actions")
                .with_child(VNode::el("button").with_attr("data-intent", "Build")
                    .with_attr("disabled", (!h.can_build).to_string()))
                .with_child(VNode::el("button").with_attr("data-intent", "Run")
                    .with_attr("disabled", (!h.can_run).to_string()))
            )
    }
}

pub struct SidebarView;
impl<D: Display> View<D> for SidebarView {
    fn render(&self, d: &D) -> VNode {
        fn node(n: &TreeNodeVM) -> VNode {
            let mut li = VNode::el("li")
                .with_attr("data-name", n.name.clone())
                .with_attr("data-dir", n.is_dir.to_string())
                .with_attr("data-open", n.open.to_string())
                .with_child(VNode::text(&n.name));
            if !n.children.is_empty() {
                let mut ul = VNode::el("ul");
                for c in &n.children { ul = ul.with_child(node(c)); }
                li = li.with_child(ul);
            }
            li
        }
        let mut ul = VNode::el("ul").with_attr("data-role", "tree");
        for r in &d.tree().roots { ul = ul.with_child(node(r)); }
        VNode::el("nav").with_child(ul)
    }
}

pub struct EditorView;
impl<D: Display> View<D> for EditorView {
    fn render(&self, d: &D) -> VNode {
        let e = d.editor();
        VNode::el("section").with_attr("data-role", "editor")
            .with_child(VNode::el("div").with_attr("class", "path")
                .with_child(VNode::text(&e.file_path)))
            .with_child(VNode::el("pre").with_attr("class", "code")
                .with_child(VNode::text(&e.content)))
    }
}

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

pub struct StatusView;
impl<D: Display> View<D> for StatusView {
    fn render(&self, d: &D) -> VNode {
        let s = d.status();
        VNode::el("div").with_attr("data-role", "status")
            .with_attr("data-connected", s.connected.to_string())
            .with_child(VNode::text(&s.msg))
    }
}

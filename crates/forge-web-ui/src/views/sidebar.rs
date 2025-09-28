use forge_view_model::TreeNodeVM;
use crate::viewcore::{VNode, View, Display};
use crate::globals::with_treestate;
use std::collections::HashSet;

pub struct SidebarView;

impl<D: Display> View<D> for SidebarView {
    fn render(&self, d: &D) -> VNode {
        let mut expanded: HashSet<String> = Default::default();
        with_treestate(|ts| expanded = ts.expanded.clone());

        fn node(n: &TreeNodeVM, expanded: &HashSet<String>) -> VNode {
            let effective_open = n.is_dir && (n.open || expanded.contains(&n.path));
            let class = if n.is_dir {
                if effective_open { "tree-item dir open" } else { "tree-item dir" }
            } else {
                "tree-item file"
            };

            // <li ...>
            let mut li = VNode::el("li")
                .with_attr("role", "treeitem")
                .with_attr("class", class)
                .with_attr("data-name", n.name.clone())
                .with_attr("data-path", n.path.clone())
                .with_attr("data-dir", n.is_dir.to_string())
                .with_attr("data-open", effective_open.to_string());

            if n.is_dir {
                li = li.with_attr("aria-expanded", effective_open.to_string());
            }

            // Row line: <div class="tree-row"> [chevron][icon][label] </div>
            let mut row = VNode::el("div").with_attr("class", "tree-row");
            if n.is_dir {
                let chev = if n.has_children {
                    VNode::el("span").with_attr("class", "chevron").with_attr("data-action", "toggle")
                } else {
                    VNode::el("span").with_attr("class", "chevron")
                };
                row = row.with_child(chev)
                         .with_child(VNode::el("span").with_attr("class", "icon folder"));
            } else {
                row = row
                    .with_child(VNode::el("span").with_attr("class", "spacer"))
                    .with_child(VNode::el("span").with_attr("class", "icon file"));
            }
            row = row.with_child(
                VNode::el("span").with_attr("class", "label").with_child(VNode::text(&n.name))
            );

            li = li.with_child(row);

            // Children list (below the row)
            if n.is_dir && effective_open && !n.children.is_empty() {
                let mut ul = VNode::el("ul").with_attr("role", "group");
                for c in &n.children {
                    ul = ul.with_child(node(c, expanded));
                }
                li = li.with_child(ul);
            }

            li
        }

        let mut ul = VNode::el("ul").with_attr("data-role", "tree").with_attr("role", "tree");
        for r in &d.tree().roots {
            ul = ul.with_child(node(r, &expanded));
        }
        VNode::el("nav").with_attr("id", "sidebar").with_child(ul)
    }
}

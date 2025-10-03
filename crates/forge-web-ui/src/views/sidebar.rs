// crates/forge-web-ui/src/views/sidebar.rs
use forge_view_model::TreeNodeVM;
use crate::globals::with_treestate;
use crate::viewcore::{Display, View, VNode};
use std::collections::HashSet;

// ---- debug helpers (only active with `--features dev`) ---------------------
#[cfg(feature = "dev")]
#[inline]
fn dbg_log(msg: impl AsRef<str>) {
    web_sys::console::log_1(&msg.as_ref().into());
}
#[cfg(not(feature = "dev"))]
#[inline]
fn dbg_log(_msg: impl AsRef<str>) {}

// ---------------------------------------------------------------------------

pub struct SidebarView;

impl<D: Display> View<D> for SidebarView {
    fn render(&self, d: &D) -> VNode {
        // Snapshot expanded paths from TreeState
        let mut expanded: HashSet<String> = Default::default();
        with_treestate(|ts| expanded = ts.expanded.clone());

        dbg_log(format!(
            "🧱 Sidebar.render: roots={}, expanded={}",
            d.tree().roots.len(),
            expanded.len()
        ));

        fn effective_open(n: &TreeNodeVM, expanded: &HashSet<String>) -> bool {
            n.is_dir && (n.open || expanded.contains(&n.path))
        }

        fn row_for(n: &TreeNodeVM) -> VNode {
            let mut row = VNode::el("div").with_attr("class", "tree-row");

            if n.is_dir {
                let chev = if n.has_children {
                    VNode::el("span")
                        .with_attr("class", "chevron")
                        .with_attr("data-action", "toggle")
                } else {
                    VNode::el("span").with_attr("class", "chevron")
                };
                row = row
                    .with_child(chev)
                    .with_child(VNode::el("span").with_attr("class", "icon folder"));
            } else {
                row = row
                    .with_child(VNode::el("span").with_attr("class", "chevron"))
                    .with_child(VNode::el("span").with_attr("class", "icon file"));
            }

            row = row.with_child(
                VNode::el("span")
                    .with_attr("class", "label")
                    .with_child(VNode::text(&n.name)),
            );

            // Badge: number if we have children now; “…” if lazily known to exist
            if n.is_dir && n.has_children {
                let badge_text = if !n.children.is_empty() {
                    format!("{}", n.children.len())
                } else {
                    "…".to_string()
                };
                row = row.with_child(
                    VNode::el("span")
                        .with_attr("class", "badge")
                        .with_child(VNode::text(&badge_text)),
                );
            }

            row
        }

        fn render_node(n: &TreeNodeVM, expanded: &HashSet<String>, depth: usize) -> VNode {
            let is_open = effective_open(n, expanded);
            let child_count = n.children.len();

            dbg_log(format!(
                "  • node depth={depth} name='{}' path='{}' dir={} has_children={} children_now={} open={}",
                n.name, n.path, n.is_dir, n.has_children, child_count, is_open
            ));

            let mut li = VNode::el("li")
                .with_attr("role", "treeitem")
                .with_attr(
                    "class",
                    if n.is_dir {
                        if is_open {
                            "tree-item dir open"
                        } else {
                            "tree-item dir"
                        }
                    } else {
                        "tree-item file"
                    },
                )
                .with_attr("data-name", n.name.clone())
                .with_attr("data-path", n.path.clone())
                .with_attr("data-dir", n.is_dir.to_string())
                .with_attr("data-open", is_open.to_string())
                .with_attr("data-depth", depth.to_string())
                .with_attr("data-has-children", n.has_children.to_string());

            if n.is_dir {
                li = li.with_attr("aria-expanded", is_open.to_string());
            }

            li = li.with_child(row_for(n));

            // Always render a <ul> for open dirs (even if empty now).
            if n.is_dir && is_open {
                let mut ul = VNode::el("ul").with_attr("role", "group");
                for c in &n.children {
                    ul = ul.with_child(render_node(c, expanded, depth + 1));
                }
                li = li.with_child(ul);
            }

            li
        }

        // Root UL
        let mut ul = VNode::el("ul")
            .with_attr("data-role", "tree")
            .with_attr("role", "tree");

        for r in &d.tree().roots {
            ul = ul.with_child(render_node(r, &expanded, 0));
        }

        dbg_log("✅ Sidebar.render done");

        VNode::el("nav").with_attr("id", "sidebar").with_child(ul)
    }
}

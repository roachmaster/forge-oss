//! Mock data for the WorkbenchDisplay (used in dev builds or tests).

use forge_view_model::*;
use std::cmp::Ordering;

/// Build a demo WorkbenchVM snapshot for development.
pub fn make_mock_vm() -> WorkbenchVM {
    WorkbenchVM {
        header: HeaderVM { title: "Forge IDE".into(), can_build: true, can_run: true },
        tree: mock_tree_paths(),
        editor: EditorVM {
            file_path: "crates/forge-web-ui/src/lib.rs".into(),
            content: "// hello from Forge\n".into(),
            cursor_line: 1,
            cursor_col: 1,
            is_dirty: false,
        },
        terminal: TerminalVM { lines: vec!["trunk serve…".into()], is_busy: false },
        status: StatusVM { msg: "Ready".into(), connected: true },
    }
}

fn mock_tree_paths() -> TreeVM {
    const ROOT: &str = "forge-oss";
    let paths = [
        "README.md",
        "crates/forge-web-ui/Cargo.toml",
        "crates/forge-web-ui/index.html",
        "crates/forge-web-ui/src/lib.rs",
        "crates/forge-web-ui/src/display.rs",
        "crates/forge-web-ui/src/views.rs",
        "crates/forge-web-ui/src/viewcore.rs",
        "crates/forge-web-ui/src/dom.rs",
        "crates/forge-view-model/Cargo.toml",
        "crates/forge-view-model/src/lib.rs",
        "crates/forge-workbenchd/Cargo.toml",
        "crates/forge-workbenchd/src/main.rs",
        "scripts/project_root.sh",
        "scripts/env.sh",
        "scripts/dev.sh",
        ".forge-root",
    ];

    tree_from_paths(ROOT, &paths)
}

/// ---------- Tree builder utils ----------

fn tree_from_paths(root_name: &str, paths: &[&str]) -> TreeVM {
    let mut root = dir_node(root_name);
    for p in paths { insert_path(&mut root, p); }
    sort_dirs_first(&mut root);
    TreeVM { roots: vec![root] }
}

fn insert_path(root: &mut TreeNodeVM, path: &str) {
    let mut parts = path.split('/').peekable();
    let mut cur = root;
    while let Some(part) = parts.next() {
        let is_last = parts.peek().is_none();
        if is_last {
            if part.is_empty() { break; }
            push_or_merge_file(cur, part);
        } else {
            cur = push_or_merge_dir(cur, part);
        }
    }
}

fn push_or_merge_dir<'a>(parent: &'a mut TreeNodeVM, name: &str) -> &'a mut TreeNodeVM {
    if let Some(idx) = parent.children.iter().position(|n| n.is_dir && n.name == name) {
        parent.children.get_mut(idx).unwrap()
    } else {
        parent.children.push(dir_node(name));
        parent.children.last_mut().unwrap()
    }
}
fn push_or_merge_file(parent: &mut TreeNodeVM, name: &str) {
    if !parent.children.iter().any(|n| !n.is_dir && n.name == name) {
        parent.children.push(file_node(name));
    }
}

fn dir_node(name: &str) -> TreeNodeVM {
    TreeNodeVM { name: name.into(), is_dir: true, open: true, children: vec![] }
}

fn file_node(name: &str) -> TreeNodeVM {
    TreeNodeVM { name: name.into(), is_dir: false, open: false, children: vec![] }
}

fn sort_dirs_first(node: &mut TreeNodeVM) {
    node.children.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });
    for child in &mut node.children {
        if child.is_dir { sort_dirs_first(child); }
    }
}

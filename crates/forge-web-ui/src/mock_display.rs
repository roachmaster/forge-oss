use crate::forge_view_model::{EditorVM, HeaderVM, StatusVM, TerminalVM, TreeNodeVM, TreeVM, WorkbenchVM};

/// Build a complete mock VM (used before WS snapshot arrives).
pub fn make_mock_vm() -> WorkbenchVM {
    WorkbenchVM {
        header: HeaderVM {
            title: "forge-oss".into(),
            can_build: true,
            can_run: true,
        },
        tree: mock_tree_forge_oss(),
        editor: EditorVM {
            file_path: "".into(),
            content: "// select a file from the tree\n".into(),
            cursor_line: 0,
            cursor_col: 0,
            is_dirty: false,
        },
        terminal: TerminalVM {
            lines: vec!["workbenchd: (mock) not connected".into()],
            is_busy: false,
        },
        status: StatusVM {
            msg: "Connecting to workbenchd…".into(),
            connected: false,
        },
    }
}

/* ---------------------- Mock tree ---------------------- */

fn mock_tree_forge_oss() -> TreeVM {
    // root node (path = "")
    let mut root = dir_node("forge-oss", "", true);

    // crates/
    let mut crates = dir_node("crates", "crates", true);

    // crates/forge-web-ui/
    let mut fwui = dir_node("forge-web-ui", "crates/forge-web-ui", true);

    // crates/forge-web-ui/src/
    let mut fwui_src = dir_node("src", "crates/forge-web-ui/src", true);
    fwui_src.children.push(file_node("lib.rs", "crates/forge-web-ui/src/lib.rs"));
    // (Add more if desired)
    mark_has_children(&mut fwui_src);

    fwui.children.push(fwui_src);
    fwui.children.push(file_node("Cargo.toml", "crates/forge-web-ui/Cargo.toml"));
    mark_has_children(&mut fwui);

    // crates/forge-view-model/
    let mut fvm = dir_node("forge-view-model", "crates/forge-view-model", true);
    let mut fvm_src = dir_node("src", "crates/forge-view-model/src", true);
    fvm_src.children.push(file_node("lib.rs", "crates/forge-view-model/src/lib.rs"));
    mark_has_children(&mut fvm_src);
    fvm.children.push(fvm_src);
    fvm.children.push(file_node("Cargo.toml", "crates/forge-view-model/Cargo.toml"));
    mark_has_children(&mut fvm);

    crates.children.push(fwui);
    crates.children.push(fvm);
    mark_has_children(&mut crates);

    // README at repo root (optional)
    // root.children.push(file_node("README.md", "README.md"));

    root.children.push(crates);
    mark_has_children(&mut root);

    TreeVM { roots: vec![root] }
}

/* ---------------------- Helpers ---------------------- */

fn dir_node(name: impl Into<String>, path: impl Into<String>, open: bool) -> TreeNodeVM {
    TreeNodeVM {
        path: path.into(),
        name: name.into(),
        is_dir: true,
        open,
        has_children: false, // will be computed by mark_has_children
        children: vec![],
    }
}

fn file_node(name: impl Into<String>, path: impl Into<String>) -> TreeNodeVM {
    TreeNodeVM {
        path: path.into(),
        name: name.into(),
        is_dir: false,
        open: false,
        has_children: false,
        children: vec![],
    }
}

/// After pushing children, call this so chevrons render properly for dirs.
fn mark_has_children(n: &mut TreeNodeVM) {
    if n.is_dir {
        n.has_children = !n.children.is_empty();
    }
}

use crate::*;
use std::path::Path;

/// Build a simple in-memory WorkbenchVM suitable for UI demos/tests.
pub fn mock_workbench(repo_id: impl Into<String>) -> WorkbenchVM {
    let repo_id = repo_id.into();
    let title = Path::new(&repo_id)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "project".into());

    WorkbenchVM {
        header: HeaderVM {
            title,
            can_build: true,
            can_run:  true,
            repo_id, // ensure non-empty so localStorage namespaces correctly
        },
        tree: mock_tree(),
        editor: EditorVM {
            file_path:  "README.md".into(),
            content:    "# Hello Forge\n\nThis is mock content.".into(),
            cursor_line: 0,
            cursor_col:  0,
            is_dirty:    false,
            size_bytes:  30,
            char_count:  30,
            line_count:  3,
            sha256:      "mock-sha256".into(), // NEW: mock/dummy hash
        },
        terminal: TerminalVM { lines: vec!["mock: ready".into()], is_busy: false },
        status:   StatusVM   { msg: "Connected (mock)".into(), connected: true },
    }
}

pub fn mock_tree() -> TreeVM {
    TreeVM {
        roots: vec![
            dir(".", "forge-oss", vec![
                dir("crates", "crates", vec![
                    dir("crates/forge-view-model", "forge-view-model", vec![
                        file("crates/forge-view-model/Cargo.toml", "Cargo.toml"),
                        dir("crates/forge-view-model/src", "src", vec![
                            file("crates/forge-view-model/src/lib.rs",  "lib.rs"),
                        ]),
                    ]),
                    dir("crates/forge-web-ui", "forge-web-ui", vec![
                        file("crates/forge-web-ui/Cargo.toml", "Cargo.toml"),
                        dir("crates/forge-web-ui/src", "src", vec![
                            file("crates/forge-web-ui/src/lib.rs", "lib.rs"),
                        ]),
                    ]),
                ]),
                file("README.md", "README.md"),
            ]),
        ],
    }
}

fn dir(path: impl Into<String>, name: impl Into<String>, children: Vec<TreeNodeVM>) -> TreeNodeVM {
    TreeNodeVM {
        path: path.into(),
        name: name.into(),
        is_dir: true,
        open: false,
        has_children: !children.is_empty(),
        children,
    }
}

fn file(path: impl Into<String>, name: impl Into<String>) -> TreeNodeVM {
    TreeNodeVM {
        path: path.into(),
        name: name.into(),
        is_dir: false,
        open: false,
        has_children: false,
        children: vec![],
    }
}

use forge_view_model::*;
use std::path::Path;

use forge_fs::{
    gitignore_walk::build_gitignore_walk,
    repo_id::compute_repo_id,
    tree_builder::{build_simple_tree, SimpleNode},
};

pub fn build_snapshot(root: &Path, depth: usize) -> WorkbenchVM {
    let repo_id = compute_repo_id(root);

    WorkbenchVM {
        header: HeaderVM {
            title: root
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "project".into()),
            can_build: true,
            can_run: true,
            repo_id,
        },
        tree: build_tree(root, depth),
        editor: EditorVM::default(),
        terminal: TerminalVM {
            lines: vec!["workbenchd: ready".into()],
            is_busy: false,
        },
        status: StatusVM {
            msg: "Connected".into(),
            connected: true,
        },
    }
}

pub fn build_tree(root: &Path, depth: usize) -> TreeVM {
    // Visible root label from directory name
    let root_name = root
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "root".into());

    // Walk (rel_path, is_dir) honoring .gitignore and depth
    let mut rel_paths: Vec<(String, bool)> = Vec::new();
    let walker = build_gitignore_walk(root, Some(depth.saturating_add(1)));

    for res in walker {
        let entry = match res {
            Ok(e) => e,
            Err(_) => continue,
        };

        let p = entry.path();
        if p == root {
            continue;
        }

        let rel = match p.strip_prefix(root) {
            Ok(r) => r,
            Err(_) => continue,
        };

        // Normalize to forward slashes for consistency
        let rel_str = rel.to_string_lossy().replace('\\', "/");
        let is_dir = p.is_dir();
        rel_paths.push((rel_str, is_dir));
    }

    // Build generic tree then map to VM
    let simple = build_simple_tree(&root_name, rel_paths);
    let vm_root = simple_to_vm(&simple, "."); // stable "." path at root

    TreeVM { roots: vec![vm_root] }
}


fn simple_to_vm(node: &SimpleNode, cur_path: &str) -> TreeNodeVM {
    let is_dir = node.is_dir;

    // Convert children first (and compute their paths)
    let mut vm_children = Vec::with_capacity(node.children.len());
    for child in &node.children {
        let child_path = join_rel(cur_path, &child.name);
        vm_children.push(simple_to_vm(child, &child_path));
    }

    TreeNodeVM {
        path: cur_path.to_string(),
        name: node.name.clone(),
        is_dir,
        open: false, // server default; client can override locally
        has_children: !vm_children.is_empty(),
        children: vm_children,
    }
}

fn join_rel(base: &str, name: &str) -> String {
    if base.is_empty() || base == "." {
        name.to_string()
    } else {
        format!("{}/{}", base, name)
    }
}

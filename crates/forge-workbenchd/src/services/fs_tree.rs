use forge_view_model::*;
use ignore::{DirEntry, WalkBuilder};
use std::path::{Path, PathBuf};

/// Build a full snapshot (header + tree + empty editor/terminal/status).
pub fn build_snapshot(root: &Path, depth: usize) -> WorkbenchVM {
    // Stable repo identifier for client-side persistence (e.g., localStorage namespace)
    let repo_id = std::fs::canonicalize(root)
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| root.display().to_string());

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

/// Build a TreeVM by walking the filesystem, honoring .gitignore.
/// Depth cap applies to directories; files at max depth are still included.
pub fn build_tree(root: &Path, depth: usize) -> TreeVM {
    // Root node represents the repo root; give it a stable "." path so it’s clickable.
    let root_name = root
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "root".into());
    let mut root_node = dir_node(root_name, ".".to_string());

    let root_abs = root.to_path_buf();

    let walker = WalkBuilder::new(root)
        .hidden(false) // allow dotfiles; still respect .gitignore
        .ignore(true) // respect .gitignore
        .git_ignore(true)
        .git_exclude(true)
        .git_global(true)
        .max_depth(Some(depth.saturating_add(1))) // include children at max depth
        .build();

    for res in walker {
        let entry = match res {
            Ok(e) => e,
            Err(_) => continue,
        };
        if is_root(&entry, &root_abs) {
            continue;
        }

        let rel = match entry.path().strip_prefix(root) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let path_str = rel.to_string_lossy();
        insert_path(&mut root_node, &path_str, entry.path().is_dir());
    }

    sort_dirs_first(&mut root_node);
    TreeVM { roots: vec![root_node] }
}

/* ------------ internal helpers ------------ */

fn is_root(entry: &DirEntry, root_abs: &PathBuf) -> bool {
    entry.path() == root_abs.as_path()
}

/// Insert a relative path into the in-memory tree, creating any missing nodes.
/// `rel` uses '/' separators (as produced by ignore::Walk).
fn insert_path(root: &mut TreeNodeVM, rel: &str, is_dir: bool) {
    let mut cur = root;
    let mut current_path = String::new();
    let mut iter = rel.split('/').peekable();

    while let Some(part) = iter.next() {
        let next_exists = iter.peek().is_some();

        if next_exists {
            // We are traversing into a directory on the way to a deeper node
            cur = push_or_merge_dir(cur, &current_path, part);
            // This directory definitely has children (we're going deeper)
            cur.has_children = true;

            // advance current_path
            current_path = join_rel_path(&current_path, part);
        } else {
            // last segment
            if is_dir {
                let _d = push_or_merge_dir(cur, &current_path, part);
                // children will be discovered as deeper entries arrive
            } else {
                push_or_merge_file(cur, &current_path, part);
            }
        }
    }
}

fn join_rel_path(base: &str, part: &str) -> String {
    if base.is_empty() {
        part.to_string()
    } else {
        format!("{}/{}", base, part)
    }
}

fn push_or_merge_dir<'a>(
    parent: &'a mut TreeNodeVM,
    parent_path: &str,
    name: &str,
) -> &'a mut TreeNodeVM {
    // Adding a directory child means the parent has children.
    parent.has_children = true;

    if let Some(idx) = parent
        .children
        .iter()
        .position(|n| n.is_dir && n.name == name)
    {
        parent.children.get_mut(idx).unwrap()
    } else {
        let child_path = join_rel_path(parent_path, name);
        parent.children.push(dir_node(name.to_string(), child_path));
        parent.children.last_mut().unwrap()
    }
}

fn push_or_merge_file(parent: &mut TreeNodeVM, parent_path: &str, name: &str) {
    if !parent
        .children
        .iter()
        .any(|n| !n.is_dir && n.name == name)
    {
        let path = join_rel_path(parent_path, name);
        parent.children.push(file_node(name.to_string(), path));
        // The parent has at least one child (a file)
        parent.has_children = true;
    }
}

fn dir_node(name: String, path: String) -> TreeNodeVM {
    TreeNodeVM {
        path,                // repo-relative ("." for root; otherwise "dir/subdir")
        name,
        is_dir: true,
        open: false,         // server default; client may override
        has_children: false, // flipped true when children are inserted
        children: vec![],
    }
}

fn file_node(name: String, path: String) -> TreeNodeVM {
    TreeNodeVM {
        path, // repo-relative "dir/file.ext"
        name,
        is_dir: false,
        open: false,
        has_children: false,
        children: vec![],
    }
}

fn sort_dirs_first(node: &mut TreeNodeVM) {
    node.children.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });
    for c in &mut node.children {
        if c.is_dir {
            sort_dirs_first(c);
        }
    }
}

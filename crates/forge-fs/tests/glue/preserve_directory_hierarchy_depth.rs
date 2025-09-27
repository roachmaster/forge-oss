// Scenario: preserve_directory_hierarchy_depth

use forge_fs::tree_builder::{build_simple_tree, SimpleNode};
use std::sync::{Mutex, OnceLock};

#[derive(Default)]
struct State {
    paths: Vec<(String, bool)>,
    tree: Option<SimpleNode>,
}

static STATE: OnceLock<Mutex<State>> = OnceLock::new();

fn s() -> &'static Mutex<State> {
    STATE.get_or_init(|| Mutex::new(State::default()))
}

/// set_up_and_get_paths:
/// Provide a deeply nested path and one sibling leaf at a higher level
/// to ensure that each segment becomes a directory and leaves are files.
pub fn set_up_deep_nested_paths() {
    let mut guard = s().lock().unwrap();
    guard.paths = vec![
        ("a/b/c/d/e/leaf.txt".into(), false), // deeply nested file
        ("a/b/c/other.txt".into(), false),    // sibling at shallower depth
    ];
}

/// execute:
/// Build the tree from the arranged paths.
pub fn execute_and_return_build_build_simple_tree() {
    let mut guard = s().lock().unwrap();
    let tree = build_simple_tree("repo", guard.paths.clone());
    guard.tree = Some(tree);
}

/// verify:
/// Each segment in a/b/c/d/e must be a directory.
pub fn verify_each_segment_is_a_dir() {
    let guard = s().lock().unwrap();
    let tree = guard.tree.as_ref().expect("tree not built");

    assert_eq!(tree.name, "repo");
    assert!(tree.is_dir, "root must be a directory");

    // helper: find a child node by name
    fn child<'a>(n: &'a SimpleNode, name: &str) -> &'a SimpleNode {
        n.children
            .iter()
            .find(|c| c.name == name)
            .unwrap_or_else(|| panic!("expected child '{name}' under '{}'", n.name))
    }

    // Walk: repo / a / b / c / d / e
    let a = child(tree, "a");
    assert!(a.is_dir, "'a' should be a directory");

    let b = child(a, "b");
    assert!(b.is_dir, "'b' should be a directory");

    let c = child(b, "c");
    assert!(c.is_dir, "'c' should be a directory");

    let d = child(c, "d");
    assert!(d.is_dir, "'d' should be a directory");

    let e = child(d, "e");
    assert!(e.is_dir, "'e' should be a directory");
}

/// and_verify:
/// Leaves are files: 'e/leaf.txt' and 'c/other.txt'
pub fn and_verify_leaf_nodes_are_files() {
    let guard = s().lock().unwrap();
    let tree = guard.tree.as_ref().expect("tree not built");

    fn child<'a>(n: &'a SimpleNode, name: &str) -> &'a SimpleNode {
        n.children
            .iter()
            .find(|c| c.name == name)
            .unwrap_or_else(|| panic!("expected child '{name}' under '{}'", n.name))
    }

    // repo / a / b / c / d / e / leaf.txt
    let a = child(tree, "a");
    let b = child(a, "b");
    let c = child(b, "c");
    let d = child(c, "d");
    let e = child(d, "e");
    let leaf = child(e, "leaf.txt");
    assert!(!leaf.is_dir, "'leaf.txt' should be a file");

    // repo / a / b / c / other.txt
    let other = child(c, "other.txt");
    assert!(!other.is_dir, "'other.txt' should be a file");
}

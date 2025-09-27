// Scenario: root_only_files_no_dirs

use forge_fs::tree_builder::{build_simple_tree, SimpleNode};

/// Prepare only root-level files (no directories at all)
pub fn set_up_root_only_files() -> Vec<(String, bool)> {
    vec![
        ("README.md".into(), false),
        ("Cargo.toml".into(), false),
        ("LICENSE".into(), false),
    ]
}

/// Execute: build the tree from the provided paths
pub fn execute_and_return_build_build_simple_tree(paths: Vec<(String, bool)>) -> SimpleNode {
    build_simple_tree("repo", paths)
}

/// Verify: all files exist directly under the root; no subdirectories created
pub fn verify_all_files_are_direct_children_of_root(tree: &SimpleNode) {
    assert_eq!(tree.name, "repo");
    assert!(tree.is_dir, "root should be a directory node");
    assert_eq!(
        tree.children.len(),
        3,
        "expected exactly three root children (files)"
    );

    // Names present?
    let names: Vec<&str> = tree.children.iter().map(|c| c.name.as_str()).collect();
    assert!(names.contains(&"README.md"));
    assert!(names.contains(&"Cargo.toml"));
    assert!(names.contains(&"LICENSE"));

    // All root children are files
    assert!(
        tree.children.iter().all(|c| !c.is_dir),
        "no directory nodes should be created at root"
    );
}

/// And: explicitly assert no directory nodes were created anywhere
pub fn and_verify_no_dir_nodes_created(tree: &SimpleNode) {
    // Root is a dir by definition; check all descendants are files (there should be none)
    for child in &tree.children {
        assert!(
            !child.is_dir,
            "unexpected directory node {:?} at root",
            child.name
        );
        assert!(
            child.children.is_empty(),
            "file nodes should not have children"
        );
    }
}

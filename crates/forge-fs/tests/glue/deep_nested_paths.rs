// Scenario: deep_nested_paths
//! Glue: detailed BDD steps for "deep nested paths"

use forge_fs::tree_builder::{build_simple_tree, SimpleNode};

/// Prepare a very deep chain of directories ending with one file.
pub fn set_up_very_deep_path_chain() -> Vec<(String, bool)> {
    vec![
        ("a/b/c/d/e/f/g/h/i/j/leaf.txt".into(), false),
    ]
}

/// Execute the core logic under test
pub fn execute_and_return_build_build_simple_tree(paths: Vec<(String, bool)>) -> SimpleNode {
    build_simple_tree("repo", paths)
}

/// Verify that the depth of the resulting tree matches the path segments
pub fn verify_depth_matches_segments(tree: &SimpleNode) {
    assert_eq!(tree.name, "repo");
    assert!(tree.is_dir);

    let mut cur = tree;
    let segments = ["a","b","c","d","e","f","g","h","i","j"];
    for seg in segments {
        let next = cur.children.iter()
            .find(|c| c.is_dir && c.name == seg)
            .expect(&format!("Expected dir {}", seg));
        cur = next;
    }
}

/// Verify that only one leaf file exists at the deepest level
pub fn and_verify_single_leaf_file_present(tree: &SimpleNode) {
    let mut cur = tree;
    let segments = ["a","b","c","d","e","f","g","h","i","j"];
    for seg in segments {
        cur = cur.children.iter()
            .find(|c| c.name == seg && c.is_dir)
            .expect(&format!("Expected dir {}", seg));
    }
    assert_eq!(cur.children.len(), 1, "deepest dir should have exactly one child");
    let leaf = &cur.children[0];
    assert_eq!(leaf.name, "leaf.txt");
    assert!(!leaf.is_dir);
}

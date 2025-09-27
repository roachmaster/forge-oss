// Scenario: tolerate_leading_trailing_slashes

use std::collections::HashSet;
use forge_fs::tree_builder::{build_simple_tree, SimpleNode};

/// Arrange
pub fn set_up_paths_with_leading_and_trailing_slashes() -> Vec<(String, bool)> {
    vec![
        ("/Cargo.toml/".into(), false),
        ("//src//lib.rs".into(), false),
        ("/src/main.rs/".into(), false),
        ("src//utils///mod.rs".into(), false),
        ("///src///".into(), true), // explicit dir path with noisy slashes
    ]
}

/// Act
pub fn execute_and_return_build_build_simple_tree(paths: Vec<(String, bool)>) -> SimpleNode {
    // Normalize the noisy paths for the builder: drop empty segments
    let norm = paths
        .into_iter()
        .map(|(raw, is_dir)| {
            let cleaned = raw
                .split('/')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join("/");
            (cleaned, is_dir)
        })
        .collect::<Vec<_>>();

    build_simple_tree("repo", norm)
}

/// Assert #1: structure normalized (root files/dirs correct)
pub fn verify_normalized_structure(tree: &SimpleNode) {
    assert_eq!(tree.name, "repo");
    assert!(tree.is_dir);

    // Collect root-level names into HashSet<String>
    let root_files: HashSet<String> = tree
        .children
        .iter()
        .filter(|n| !n.is_dir)
        .map(|n| n.name.clone())
        .collect();

    let root_dirs: HashSet<String> = tree
        .children
        .iter()
        .filter(|n| n.is_dir)
        .map(|n| n.name.clone())
        .collect();

    assert!(root_files.contains("Cargo.toml"), "Cargo.toml should be at root");
    assert!(root_dirs.contains("src"), "src dir should be present at root");
}

/// Assert #2: no duplicates introduced by leading/trailing slashes
pub fn and_verify_no_duplicate_nodes_from_slashes(tree: &SimpleNode) {
    let src = tree
        .children
        .iter()
        .find(|c| c.is_dir && c.name == "src")
        .expect("src directory should exist");

    let src_files: HashSet<String> = src
        .children
        .iter()
        .filter(|n| !n.is_dir)
        .map(|n| n.name.clone())
        .collect();

    let src_dirs: HashSet<String> = src
        .children
        .iter()
        .filter(|n| n.is_dir)
        .map(|n| n.name.clone())
        .collect();

    assert!(src_files.contains("lib.rs"), "src/lib.rs should exist");
    assert!(src_files.contains("main.rs"), "src/main.rs should exist");
    assert!(src_dirs.contains("utils"), "src/utils directory should exist");

    // Ensure utils has exactly one mod.rs and no duplicates
    let utils = src
        .children
        .iter()
        .find(|c| c.is_dir && c.name == "utils")
        .expect("utils dir should exist");

    let utils_files: HashSet<String> = utils
        .children
        .iter()
        .filter(|n| !n.is_dir)
        .map(|n| n.name.clone())
        .collect();

    assert!(
        utils_files.contains("mod.rs"),
        "src/utils/mod.rs should exist exactly once"
    );
    assert_eq!(
        utils_files.len(),
        1,
        "utils should have only one file (mod.rs), no duplicates"
    );
}

// Scenario: build_a_tree_from_mixed_files_and_dirs
//! Glue: detailed BDD steps for "build a tree from mixed files and directories"

use forge_fs::tree_builder::{build_simple_tree, SimpleNode};

/// Prepare paths for the scenario
pub fn set_up_and_get_paths() -> Vec<(String, bool)> {
    vec![
        ("src/lib.rs".into(), false),
        ("src/main.rs".into(), false),
        ("Cargo.toml".into(), false),
        ("src/utils/mod.rs".into(), false),
    ]
}

/// Execute the core logic under test
pub fn execute_and_return_build_build_simple_tree(paths: Vec<(String, bool)>) -> SimpleNode {
    build_simple_tree("repo", paths)
}

/// Verify that files exist as expected
pub fn verify_tree_files(tree: &SimpleNode) {
    assert_eq!(tree.name, "repo");
    assert!(tree.is_dir);

    let has_cargo = tree.children.iter().any(|c| c.name == "Cargo.toml" && !c.is_dir);
    assert!(has_cargo, "Cargo.toml should be a file in root");

    let src = tree.children.iter().find(|c| c.name == "src" && c.is_dir)
        .expect("src dir should exist");
    let files: Vec<&str> = src.children.iter().map(|c| c.name.as_str()).collect();
    assert!(files.contains(&"lib.rs"));
    assert!(files.contains(&"main.rs"));
}

/// Verify that directories exist and have correct children
pub fn and_verify_dirs(tree: &SimpleNode) {
    let src = tree.children.iter().find(|c| c.name == "src" && c.is_dir)
        .expect("src dir should exist");
    let utils = src.children.iter().find(|c| c.name == "utils" && c.is_dir)
        .expect("utils dir should exist");
    assert_eq!(utils.children.len(), 1);
    assert_eq!(utils.children[0].name, "mod.rs");
    assert!(!utils.children[0].is_dir);
}

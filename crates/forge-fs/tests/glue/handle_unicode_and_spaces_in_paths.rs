// Scenario: handle_unicode_and_spaces_in_paths
//! Glue: detailed BDD steps for "handle unicode and spaces in paths"

use forge_fs::tree_builder::{build_simple_tree, SimpleNode};

/// Prepare paths containing Unicode and spaces
pub fn set_up_paths_with_unicode_and_spaces() -> Vec<(String, bool)> {
    vec![
        ("src/üñíçødë.rs".into(), false),
        ("src/with space.rs".into(), false),
        ("tests/テスト.rs".into(), false),
        ("Cargo.toml".into(), false),
    ]
}

/// Execute the core logic under test
pub fn execute_and_return_build_build_simple_tree(paths: Vec<(String, bool)>) -> SimpleNode {
    build_simple_tree("repo", paths)
}

/// Verify that Unicode paths are represented correctly
pub fn verify_unicode_nodes_exist(tree: &SimpleNode) {
    let src = tree
        .children
        .iter()
        .find(|c| c.name == "src" && c.is_dir)
        .expect("src dir should exist");
    assert!(
        src.children.iter().any(|c| c.name == "üñíçødë.rs"),
        "src should contain üñíçødë.rs"
    );

    let tests = tree
        .children
        .iter()
        .find(|c| c.name == "tests" && c.is_dir)
        .expect("tests dir should exist");
    assert!(
        tests.children.iter().any(|c| c.name == "テスト.rs"),
        "tests should contain テスト.rs"
    );
}

/// Verify that file names with spaces are preserved
pub fn and_verify_spaces_nodes_exist(tree: &SimpleNode) {
    let src = tree
        .children
        .iter()
        .find(|c| c.name == "src" && c.is_dir)
        .expect("src dir should exist");
    assert!(
        src.children.iter().any(|c| c.name == "with space.rs"),
        "src should contain file with space in name"
    );
}

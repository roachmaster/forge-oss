// Scenario: de_duplicate_repeated_paths
//! Glue: detailed BDD steps for "de-duplicate repeated paths"

use forge_fs::tree_builder::{build_simple_tree, SimpleNode};

/// Prepare paths containing duplicates
pub fn set_up_and_get_paths_with_duplicates() -> Vec<(String, bool)> {
    vec![
        ("src".into(), true),
        ("src".into(), true),
        ("src/utils".into(), true),
        ("src/utils".into(), true),
        ("src/lib.rs".into(), false),
        ("src/lib.rs".into(), false),
        ("src/main.rs".into(), false),
        ("src/main.rs".into(), false),
        ("src/utils/mod.rs".into(), false),
        ("src/utils/mod.rs".into(), false),
        ("README.md".into(), false),
        ("README.md".into(), false),
    ]
}

/// Execute the core logic under test
pub fn execute_and_return_build_build_simple_tree(paths: Vec<(String, bool)>) -> SimpleNode {
    build_simple_tree("repo", paths)
}

/// Verify that directories appear only once
pub fn verify_no_duplicate_dirs(tree: &SimpleNode) {
    assert_eq!(tree.name, "repo");
    assert!(tree.is_dir);

    let src_dirs: Vec<_> = tree.children.iter().filter(|c| c.name == "src" && c.is_dir).collect();
    assert_eq!(src_dirs.len(), 1, "src directory should appear once");

    let src = src_dirs[0];
    let utils_dirs: Vec<_> = src.children.iter().filter(|c| c.name == "utils" && c.is_dir).collect();
    assert_eq!(utils_dirs.len(), 1, "utils directory should appear once");
}

/// Verify that files appear only once
pub fn and_verify_no_duplicate_files(tree: &SimpleNode) {
    let readme_count = tree.children.iter().filter(|c| c.name == "README.md" && !c.is_dir).count();
    assert_eq!(readme_count, 1, "README.md should appear once in root");

    let src = tree.children.iter().find(|c| c.name == "src" && c.is_dir)
        .expect("src dir should exist");
    let lib_count = src.children.iter().filter(|c| c.name == "lib.rs" && !c.is_dir).count();
    let main_count = src.children.iter().filter(|c| c.name == "main.rs" && !c.is_dir).count();
    assert_eq!(lib_count, 1, "lib.rs should appear once in src");
    assert_eq!(main_count, 1, "main.rs should appear once in src");

    let utils = src.children.iter().find(|c| c.name == "utils" && c.is_dir)
        .expect("utils dir should exist");
    let mod_count = utils.children.iter().filter(|c| c.name == "mod.rs" && !c.is_dir).count();
    assert_eq!(mod_count, 1, "mod.rs should appear once in utils");
}

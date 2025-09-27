// Scenario: sort_dirs_first_then_alpha

use forge_fs::tree_builder::{build_simple_tree, SimpleNode};

/// Prepare a mixed, deliberately unsorted set of root children (both dirs and files).
pub fn set_up_mixed_unsorted_children() -> Vec<(String, bool)> {
    vec![
        ("b.TXT".into(), false),      // file
        ("A.txt".into(), false),      // file (capitalized to check case-insensitive sort)
        ("src".into(), true),         // dir
        ("docs".into(), true),        // dir (should sort before "src")
        // a child inside src so it's definitely a directory with content (not required for ordering)
        ("src/mod.rs".into(), false),
    ]
}

/// Execute: build the tree from the provided paths.
pub fn execute_and_return_build_build_simple_tree(paths: Vec<(String, bool)>) -> SimpleNode {
    build_simple_tree("repo", paths)
}

/// Verify: in root, all directory nodes appear before any file nodes.
pub fn verify_dirs_listed_before_files(tree: &SimpleNode) {
    assert_eq!(tree.name, "repo");
    assert!(tree.is_dir);

    let children = &tree.children;
    assert!(
        !children.is_empty(),
        "expected mixed children at root (dirs and files)"
    );

    // Find first file; ensure no directory appears after that point.
    let mut seen_file = false;
    for (idx, c) in children.iter().enumerate() {
        if c.is_dir {
            assert!(
                !seen_file,
                "directory '{}' appeared after a file at index {}",
                c.name,
                idx
            );
        } else {
            seen_file = true;
        }
    }
}

/// And: within each group (dirs, then files), order is case-insensitive alphabetical.
pub fn and_verify_case_insensitive_alpha_order(tree: &SimpleNode) {
    let children = &tree.children;

    // Split into dirs and files preserving order as rendered.
    let dirs: Vec<&String> = children
        .iter()
        .filter(|c| c.is_dir)
        .map(|c| &c.name)
        .collect();

    let files: Vec<&String> = children
        .iter()
        .filter(|c| !c.is_dir)
        .map(|c| &c.name)
        .collect();

    // Basic presence sanity
    assert!(
        dirs.len() >= 2 && files.len() >= 2,
        "need at least two dirs and two files to validate sorting (got {} dirs, {} files)",
        dirs.len(),
        files.len()
    );

    // Expect specific members (scenario inputs)
    let dir_names_lower: Vec<String> = dirs.iter().map(|s| s.to_lowercase()).collect();
    assert!(dir_names_lower.contains(&"docs".to_string()));
    assert!(dir_names_lower.contains(&"src".to_string()));

    let file_names_lower: Vec<String> = files.iter().map(|s| s.to_lowercase()).collect();
    assert!(file_names_lower.contains(&"a.txt".to_string()));
    assert!(file_names_lower.contains(&"b.txt".to_string()));

    // Check case-insensitive sort inside each group
    let mut dirs_sorted = dir_names_lower.clone();
    dirs_sorted.sort();
    assert_eq!(
        dir_names_lower, dirs_sorted,
        "directories are not in case-insensitive alphabetical order: {:?}",
        dirs
    );

    let mut files_sorted = file_names_lower.clone();
    files_sorted.sort();
    assert_eq!(
        file_names_lower, files_sorted,
        "files are not in case-insensitive alphabetical order: {:?}",
        files
    );

    // Spot-check expected order from our inputs: "docs" before "src"; "A.txt" before "b.TXT"
    let pos_docs = dir_names_lower
        .iter()
        .position(|s| s == "docs")
        .expect("docs dir missing");
    let pos_src = dir_names_lower
        .iter()
        .position(|s| s == "src")
        .expect("src dir missing");
    assert!(pos_docs < pos_src, r#"expected "docs" before "src""#);

    let pos_a = file_names_lower
        .iter()
        .position(|s| s == "a.txt")
        .expect("A.txt missing");
    let pos_b = file_names_lower
        .iter()
        .position(|s| s == "b.txt")
        .expect("b.TXT missing");
    assert!(pos_a < pos_b, r#"expected "A.txt" before "b.TXT""#);
}

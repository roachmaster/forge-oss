// Scenario: idempotent_insertion_order
//! Glue: detailed BDD steps for "idempotent insertion order"

use forge_fs::tree_builder::{build_simple_tree, SimpleNode};
use std::cmp::Ordering;
use std::sync::{Mutex, OnceLock};

/// Shared state across BDD steps for this scenario
struct TestState {
    p1: Vec<(String, bool)>,
    p2: Vec<(String, bool)>,
    t1: Option<SimpleNode>,
    t2: Option<SimpleNode>,
}

static STATE: OnceLock<Mutex<TestState>> = OnceLock::new();

fn state() -> &'static Mutex<TestState> {
    STATE.get_or_init(|| {
        Mutex::new(TestState {
            p1: vec![],
            p2: vec![],
            t1: None,
            t2: None,
        })
    })
}

/// 1) set_up: prepare two permutations of the same logical paths
pub fn set_up_two_permutations_of_same_paths() {
    // The same logical set, different orders
    let paths_a = vec![
        ("src/lib.rs".to_string(), false),
        ("src/main.rs".to_string(), false),
        ("Cargo.toml".to_string(), false),
        ("src/utils/mod.rs".to_string(), false),
        ("README.md".to_string(), false),
        ("src/utils".to_string(), true),
        ("src".to_string(), true),
    ];

    let paths_b = vec![
        ("src".to_string(), true),
        ("src/utils".to_string(), true),
        ("Cargo.toml".to_string(), false),
        ("README.md".to_string(), false),
        ("src/main.rs".to_string(), false),
        ("src/lib.rs".to_string(), false),
        ("src/utils/mod.rs".to_string(), false),
    ];

    let mut s = state().lock().unwrap();
    s.p1 = paths_a;
    s.p2 = paths_b;
}

/// 2) execute: build a tree for each permutation
pub fn execute_build_tree_for_each_permutation() {
    let mut s = state().lock().unwrap();
    s.t1 = Some(build_simple_tree("repo", s.p1.clone()));
    s.t2 = Some(build_simple_tree("repo", s.p2.clone()));
}

/// 3) verify: both trees are structurally identical
pub fn verify_structural_equality_between_trees() {
    let s = state().lock().unwrap();
    let t1 = s.t1.as_ref().expect("t1 not built");
    let t2 = s.t2.as_ref().expect("t2 not built");
    assert!(eq_tree(t1, t2), "Trees built from different orders must be identical");
}

/// 4) and: each tree is sorted (dirs first, then case-insensitive alpha)
pub fn and_verify_sorted_order_in_each_tree() {
    let s = state().lock().unwrap();
    let t1 = s.t1.as_ref().expect("t1 not built");
    let t2 = s.t2.as_ref().expect("t2 not built");
    assert_sorted_invariant(t1);
    assert_sorted_invariant(t2);
}

/* ---------------- helpers ---------------- */

fn eq_tree(a: &SimpleNode, b: &SimpleNode) -> bool {
    if a.name != b.name || a.is_dir != b.is_dir || a.children.len() != b.children.len() {
        return false;
    }
    for (ca, cb) in a.children.iter().zip(b.children.iter()) {
        if !eq_tree(ca, cb) {
            return false;
        }
    }
    true
}

fn assert_sorted_invariant(node: &SimpleNode) {
    // Check dirs-first
    let mut seen_file = false;
    for c in &node.children {
        if !c.is_dir {
            seen_file = true;
        } else {
            assert!(
                !seen_file,
                "Directories must appear before files at the same level (at node '{}')",
                node.name
            );
        }
    }
    // Check case-insensitive alpha among same-kind neighbors
    for pair in node.children.windows(2) {
        let (a, b) = (&pair[0], &pair[1]);
        if a.is_dir == b.is_dir {
            let ord = a.name.to_lowercase().cmp(&b.name.to_lowercase());
            assert!(
                ord != Ordering::Greater,
                "Siblings of the same kind must be case-insensitive sorted: '{}' before '{}'",
                a.name,
                b.name
            );
        }
    }
    // Recurse
    for c in &node.children {
        if c.is_dir {
            assert_sorted_invariant(c);
        }
    }
}

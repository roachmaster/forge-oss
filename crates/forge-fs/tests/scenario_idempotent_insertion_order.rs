//! Scenario facade: idempotent_insertion_order

// Import glue steps
mod glue;

use glue::idempotent_insertion_order::{
    set_up_two_permutations_of_same_paths,
    execute_build_tree_for_each_permutation,
    verify_structural_equality_between_trees,
    and_verify_sorted_order_in_each_tree,
};

/// High-level BDD orchestration for the scenario:
/// - Prepare two permutations of the same file list
/// - Build trees from each order
/// - Verify both trees are structurally identical
/// - Verify each tree respects dirs-first + alpha sorting
#[test]
fn scenario_idempotent_insertion_order() {
    // 1. Arrange
    set_up_two_permutations_of_same_paths();

    // 2. Act
    execute_build_tree_for_each_permutation();

    // 3. Assert
    verify_structural_equality_between_trees();
    and_verify_sorted_order_in_each_tree();
}

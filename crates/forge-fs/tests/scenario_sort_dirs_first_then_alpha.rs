//! Scenario facade: sort_dirs_first_then_alpha

mod glue;

use glue::sort_dirs_first_then_alpha::{
    set_up_mixed_unsorted_children,
    execute_and_return_build_build_simple_tree,
    verify_dirs_listed_before_files,
    and_verify_case_insensitive_alpha_order,
};

#[test]
fn scenario_sort_dirs_first_then_alpha() {
    // Arrange
    let paths = set_up_mixed_unsorted_children();

    // Act
    let tree = execute_and_return_build_build_simple_tree(paths);

    // Assert
    verify_dirs_listed_before_files(&tree);
    and_verify_case_insensitive_alpha_order(&tree);
}

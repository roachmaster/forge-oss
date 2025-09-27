//! Scenario facade: tolerate_leading_trailing_slashes

mod glue;

use glue::tolerate_leading_trailing_slashes::{
    set_up_paths_with_leading_and_trailing_slashes,
    execute_and_return_build_build_simple_tree,
    verify_normalized_structure,
    and_verify_no_duplicate_nodes_from_slashes,
};

#[test]
fn scenario_tolerate_leading_trailing_slashes() {
    // Arrange
    let paths = set_up_paths_with_leading_and_trailing_slashes();

    // Act
    let tree = execute_and_return_build_build_simple_tree(paths);

    // Assert
    verify_normalized_structure(&tree);
    and_verify_no_duplicate_nodes_from_slashes(&tree);
}

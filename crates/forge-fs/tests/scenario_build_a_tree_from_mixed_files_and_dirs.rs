mod glue;
use glue::build_a_tree_from_mixed_files_and_dirs::*;

#[test]
fn scenario_build_a_tree_from_mixed_files_and_dirs() {
    // Given
    let paths = set_up_and_get_paths();

    // When
    let tree = execute_and_return_build_build_simple_tree(paths);

    // Then
    verify_tree_files(&tree);
    and_verify_dirs(&tree);
}

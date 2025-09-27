mod glue;
use glue::deep_nested_paths::*;

#[test]
fn scenario_deep_nested_paths() {
    // Given
    let paths = set_up_very_deep_path_chain();

    // When
    let tree = execute_and_return_build_build_simple_tree(paths);

    // Then
    verify_depth_matches_segments(&tree);
    and_verify_single_leaf_file_present(&tree);
}

mod glue;

use glue::de_duplicate_repeated_paths::*;

#[test]
fn scenario_de_duplicate_repeated_paths() {
    // Given
    let paths = set_up_and_get_paths_with_duplicates();

    // When
    let tree = execute_and_return_build_build_simple_tree(paths);

    // Then
    verify_no_duplicate_dirs(&tree);
    and_verify_no_duplicate_files(&tree);
}

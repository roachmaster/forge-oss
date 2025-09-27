mod glue;
use glue::handle_unicode_and_spaces_in_paths::*;

#[test]
fn scenario_handle_unicode_and_spaces_in_paths() {
    let paths = set_up_paths_with_unicode_and_spaces();
    let tree = execute_and_return_build_build_simple_tree(paths);
    verify_unicode_nodes_exist(&tree);
    and_verify_spaces_nodes_exist(&tree);
}

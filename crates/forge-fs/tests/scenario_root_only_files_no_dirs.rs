// tests/scenario_root_only_files_no_dirs.rs

mod glue;

use glue::root_only_files_no_dirs::{
    set_up_root_only_files,
    execute_and_return_build_build_simple_tree,
    verify_all_files_are_direct_children_of_root,
    and_verify_no_dir_nodes_created,
};

#[test]
fn scenario_root_only_files_no_dirs() {
    let paths = set_up_root_only_files();
    let tree = execute_and_return_build_build_simple_tree(paths);
    verify_all_files_are_direct_children_of_root(&tree);
    and_verify_no_dir_nodes_created(&tree);
}

//! Scenario facade: preserve_directory_hierarchy_depth

mod glue;

use glue::preserve_directory_hierarchy_depth::{
    set_up_deep_nested_paths,
    execute_and_return_build_build_simple_tree,
    verify_each_segment_is_a_dir,
    and_verify_leaf_nodes_are_files,
};

#[test]
fn scenario_preserve_directory_hierarchy_depth() {
    let paths = set_up_deep_nested_paths();
    let tree = execute_and_return_build_build_simple_tree();
    verify_each_segment_is_a_dir();
    and_verify_leaf_nodes_are_files();
}

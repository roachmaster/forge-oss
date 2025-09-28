// glue module for integration test scenarios
pub mod build_a_tree_from_mixed_files_and_dirs;
pub mod de_duplicate_repeated_paths;
pub mod sort_dirs_first_then_alpha;
pub mod preserve_directory_hierarchy_depth;
pub mod handle_unicode_and_spaces_in_paths;
pub mod tolerate_leading_trailing_slashes;
pub mod idempotent_insertion_order;
pub mod root_only_files_no_dirs;
pub mod deep_nested_paths;
pub mod read_simple_file_contents;
pub mod handle_nonexistent_file;
pub mod read_large_file;
pub mod read_file_with_unicode_characters;
pub mod read_empty_file;
pub mod read_file_with_leading_trailing_whitespace;
pub mod handle_permission_denied;

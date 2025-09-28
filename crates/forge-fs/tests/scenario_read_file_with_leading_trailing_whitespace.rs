//! Scenario facade: read_file_with_leading_trailing_whitespace

mod glue;

use glue::read_file_with_leading_trailing_whitespace::{
    set_up_file_with_whitespace,
    execute_read_file_contents,
    verify_whitespace_preserved,
};

#[test]
fn scenario_read_file_with_leading_trailing_whitespace() {
    let (_keep_dir_alive, path, expected) = set_up_file_with_whitespace();
    let result = execute_read_file_contents(&path);
    verify_whitespace_preserved(&result, &expected);
}

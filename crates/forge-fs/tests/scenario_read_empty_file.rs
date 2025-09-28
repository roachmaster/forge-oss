//! Scenario facade: read_empty_file

mod glue;

use glue::read_empty_file::{set_up_empty_file, execute_read_file_contents, verify_empty_result};

#[test]
fn scenario_read_empty_file() {
    let (_keep_dir_alive, path) = set_up_empty_file(); // keep until end of test
    let result = execute_read_file_contents(&path);
    verify_empty_result(&result);
}

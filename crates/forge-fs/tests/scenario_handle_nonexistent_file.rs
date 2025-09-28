//! Scenario facade: handle_nonexistent_file

mod glue;

use glue::handle_nonexistent_file::{
    set_up_nonexistent_path,
    execute_read_file_contents,
    verify_error_is_returned
};

#[test]
fn scenario_handle_nonexistent_file() {
    let path = set_up_nonexistent_path();
    let result = execute_read_file_contents(path);
    verify_error_is_returned(&result);
}

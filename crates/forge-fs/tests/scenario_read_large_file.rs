//! Scenario facade: read_large_file

mod glue;

use glue::read_large_file::{
    set_up_large_temp_file,
    execute_read_file_contents,
    verify_full_large_content_read
};

#[test]
fn scenario_read_large_file() {
    let path = set_up_large_temp_file();
    let result = execute_read_file_contents(path);
    verify_full_large_content_read(&result);
}

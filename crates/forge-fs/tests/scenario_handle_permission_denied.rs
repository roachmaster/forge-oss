//! Scenario facade: handle_permission_denied

mod glue;

use glue::handle_permission_denied::{
    set_up_file_with_denied_permissions,
    execute_read_file_contents,
    verify_permission_error_returned,
};

#[test]
fn scenario_handle_permission_denied() {
    set_up_file_with_denied_permissions();
    execute_read_file_contents();
    verify_permission_error_returned();
}

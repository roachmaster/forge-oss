//! Scenario facade: read_simple_file_contents

mod glue;

use glue::read_simple_file_contents::{
    set_up_temp_file_with_known_text,
    execute_read_file_contents,
    verify_contents_match_expected,
};

#[test]
fn scenario_read_simple_file_contents() {
    let file = set_up_temp_file_with_known_text();
    let contents = execute_read_file_contents(&file).expect("should read text file");
    verify_contents_match_expected(&contents);
}

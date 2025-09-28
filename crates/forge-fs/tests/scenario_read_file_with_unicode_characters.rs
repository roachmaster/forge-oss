//! Scenario facade: read_file_with_unicode_characters

mod glue;

use glue::read_file_with_unicode_characters::{
    set_up_file_with_unicode_content,
    execute_read_file_contents,
    verify_unicode_content_preserved,
};

#[test]
fn scenario_read_file_with_unicode_characters() {
    // 1. Prepare a file with mixed Unicode characters.
    let (path, expected) = set_up_file_with_unicode_content();

    // 2. Execute the actual file read.
    let result = execute_read_file_contents(path);

    // 3. Verify the result matches the expected Unicode text and counts.
    verify_unicode_content_preserved(&result, &expected);
}

// Scenario: read_simple_file_contents

use forge_fs::file_read::read_text_file;
use forge_fs::errors::FsError;
use assert_fs::prelude::*;
use assert_fs::NamedTempFile;
use std::path::Path;

/// Prepare a temp file with known text
pub fn set_up_temp_file_with_known_text() -> NamedTempFile {
    let file = NamedTempFile::new("sample.txt").unwrap();
    file.write_str("Hello Bitwave!\nLine2").unwrap();
    file
}

/// Execute the core file read logic
pub fn execute_read_file_contents(file: &NamedTempFile) -> Result<String, FsError> {
    let info = read_text_file(file.path(), 0)?;
    Ok(info.content)
}

/// Verify the contents match the expected string
pub fn verify_contents_match_expected(contents: &str) {
    assert!(contents.contains("Hello Bitwave!"));
    assert!(contents.contains("Line2"));
}

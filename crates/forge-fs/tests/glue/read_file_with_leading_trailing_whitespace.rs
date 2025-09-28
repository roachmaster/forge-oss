// Scenario: read_file_with_leading_trailing_whitespace

use forge_fs::errors::FsError;
use forge_fs::file_read::{read_text_file, FileInfo};

use assert_fs::prelude::*; // PathChild
use assert_fs::TempDir;

use std::path::{Path, PathBuf};

/// Step: create a temp file that has leading/trailing spaces and newlines.
pub fn set_up_file_with_whitespace() -> (TempDir, PathBuf, String) {
    let dir = TempDir::new().expect("temp dir");
    let file = dir.child("whitespace.txt");
    let content = "   \n  Hello World  \n   \n";
    file.write_str(content).expect("write whitespace file");
    (dir, file.path().to_path_buf(), content.to_string())
}

/// Step: call the core read_text_file function.
pub fn execute_read_file_contents(path: &Path) -> Result<FileInfo, FsError> {
    read_text_file(path, 0)
}

/// Step: verify that the returned content matches exactly (including spaces and newlines).
pub fn verify_whitespace_preserved(result: &Result<FileInfo, FsError>, expected: &str) {
    match result {
        Ok(info) => {
            assert_eq!(info.content, expected, "Whitespace content should be preserved exactly");
            assert_eq!(
                info.char_count,
                expected.chars().count(),
                "Char count should match"
            );
            assert_eq!(
                info.line_count,
                expected.lines().count(),
                "Line count should match"
            );
        }
        Err(e) => panic!("Expected Ok for whitespace file, got error: {:?}", e),
    }
}

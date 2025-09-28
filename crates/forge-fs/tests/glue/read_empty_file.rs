// Scenario: read_empty_file

use forge_fs::errors::FsError;
use forge_fs::file_read::{read_text_file, FileInfo};

use assert_fs::prelude::*; // PathChild + touch()
use assert_fs::TempDir;

use std::path::{Path, PathBuf};

/// Step: create an empty temporary file, returning the TempDir to keep it alive.
pub fn set_up_empty_file() -> (TempDir, PathBuf) {
    let dir = TempDir::new().expect("temp dir");
    let file = dir.child("empty.txt");
    file.touch().expect("create empty file");
    (dir, file.path().to_path_buf())
}

/// Step: read the empty file (0 = no cap).
pub fn execute_read_file_contents(path: &Path) -> Result<FileInfo, FsError> {
    read_text_file(path, 0)
}

/// Step: verify the result is an empty string with zero counts.
pub fn verify_empty_result(result: &Result<FileInfo, FsError>) {
    match result {
        Ok(info) => {
            assert!(info.content.is_empty(), "Expected empty content");
            assert_eq!(info.char_count, 0, "Char count should be 0");
            assert_eq!(info.line_count, 0, "Line count should be 0");
        }
        Err(e) => panic!("Expected Ok for empty file, got error: {:?}", e),
    }
}

// Scenario: handle_nonexistent_file
//! Glue: detailed BDD steps for "handle nonexistent file gracefully"

use forge_fs::errors::FsError;
use forge_fs::file_read::{read_text_file, FileInfo};
use std::path::PathBuf;

/// Step: prepare a path to a file that does not exist
pub fn set_up_nonexistent_path() -> PathBuf {
    // Use an obviously-absent file name in CWD.
    PathBuf::from("definitely_nonexistent_file_12345.txt")
}

/// Step: attempt to read the nonexistent file
pub fn execute_read_file_contents(path: PathBuf) -> Result<FileInfo, FsError> {
    // No cloning of Result; we just forward it to the caller.
    read_text_file(&path, 0)
}

/// Step: verify an error was returned
pub fn verify_error_is_returned(result: &Result<FileInfo, FsError>) {
    assert!(
        result.is_err(),
        "Expected an error when reading a nonexistent file"
    );
}

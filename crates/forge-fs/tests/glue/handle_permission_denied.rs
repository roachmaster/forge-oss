// Scenario: handle_permission_denied

use forge_fs::errors::FsError;
use forge_fs::file_read::{read_text_file, FileInfo};

use assert_fs::prelude::*; // PathChild
use assert_fs::TempDir;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

static mut PATH: Option<PathBuf> = None;
static mut RESULT: Option<Result<FileInfo, FsError>> = None;

/// Step: create a file but remove read permissions.
pub fn set_up_file_with_denied_permissions() {
    let dir = TempDir::new().expect("temp dir");
    let file = dir.child("no_access.txt");
    file.write_str("secret content").expect("write file");
    let path = file.path().to_path_buf();

    // Remove all permissions (simulate permission denied)
    let mut perms = fs::metadata(&path).expect("metadata").permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&path, perms).expect("set perms");

    unsafe { PATH = Some(path) };
    // Keep `dir` alive so file still exists while running test
    std::mem::forget(dir);
}

/// Step: call read_text_file and store the result.
pub fn execute_read_file_contents() {
    let path = unsafe { PATH.clone().expect("PATH must be set") };
    let res = read_text_file(&path, 0);
    unsafe { RESULT = Some(res) };
}

/// Step: assert that we got a permission error.
pub fn verify_permission_error_returned() {
    let res = unsafe { RESULT.take().expect("RESULT must be set") };
    match res {
        Ok(info) => panic!("Expected permission denied error, got Ok: {:?}", info),
        Err(err) => {
            match err {
                FsError::Io(e) => {
                    assert_eq!(
                        e.kind(),
                        std::io::ErrorKind::PermissionDenied,
                        "Expected permission denied error, got {:?}",
                        e
                    );
                }
                other => panic!("Expected FsError::Io(PermissionDenied), got {:?}", other),
            }
        }
    }
}

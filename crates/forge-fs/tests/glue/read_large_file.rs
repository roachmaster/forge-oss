// Scenario: read_large_file
//! Glue: BDD steps for "read a large file and confirm full content is read"

use forge_fs::errors::FsError;
use forge_fs::file_read::{read_text_file, FileInfo};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// Step: create a large temp file with predictable content.
/// Returns the file path so the scenario facade can pass it to execute().
pub fn set_up_large_temp_file() -> PathBuf {
    let dir = std::env::temp_dir();
    let path = dir.join("forge_fs_large_test.txt");

    // Write exactly 1000 lines; each line is 100 bytes: 99 'A's + '\n' = 100 bytes.
    let mut f = fs::File::create(&path).expect("create large temp file");
    let line = "A".repeat(99) + "\n";
    for _ in 0..1000 {
        f.write_all(line.as_bytes()).unwrap();
    }
    f.flush().unwrap();

    path
}

/// Step: execute read_text_file on the large file and return the Result.
pub fn execute_read_file_contents(path: PathBuf) -> Result<FileInfo, FsError> {
    // max_bytes = 0 means "no limit"
    read_text_file(&path, 0)
}

/// Step: verify entire content was read (not truncated) and counts make sense.
pub fn verify_full_large_content_read(result: &Result<FileInfo, FsError>) {
    let info = result.as_ref().expect("expected Ok(FileInfo)");

    // We wrote exactly 1000 * 100 = 100,000 bytes.
    // The file size from metadata should match that.
    assert!(
        info.size_bytes >= 100_000,
        "file size should be >= 100 KB (got {})",
        info.size_bytes
    );

    // Content should be the same number of bytes in UTF-8 here (ASCII only).
    assert!(
        info.content.len() >= 100_000,
        "content length should be >= 100,000 (got {})",
        info.content.len()
    );

    // Exactly 1000 lines were written.
    assert_eq!(
        info.line_count, 1000,
        "line_count should be 1000 (got {})",
        info.line_count
    );

    // Char count should be equal to byte count here (ASCII), or at least close.
    assert!(
        info.char_count >= 100_000,
        "char_count should be >= 100,000 (got {})",
        info.char_count
    );
}

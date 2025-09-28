// Scenario: read_file_with_unicode_characters
//! Glue: BDD steps for "read a file containing Unicode text (emojis, accents, non-Latin)"

use forge_fs::errors::FsError;
use forge_fs::file_read::{read_text_file, FileInfo};
use std::{fs, io::Write, path::PathBuf};

/// Step: create a UTF-8 file with diverse Unicode content and return (path, expected_text).
pub fn set_up_file_with_unicode_content() -> (PathBuf, String) {
    // Mix accents, emoji, and non-Latin scripts.
    let expected = String::from("Héllo 🌍 — 你好 мир مرحبا\n第二行 with emoji 😀 and accents: café\n");

    let path = std::env::temp_dir().join("forge_fs_unicode_test.txt");
    let mut f = fs::File::create(&path).expect("create unicode temp file");
    f.write_all(expected.as_bytes()).expect("write unicode");
    f.flush().expect("flush unicode");

    (path, expected)
}

/// Step: execute read_text_file on the provided path.
pub fn execute_read_file_contents(path: PathBuf) -> Result<FileInfo, FsError> {
    // 0 = no size cap
    read_text_file(&path, 0)
}

/// Step: verify the content is preserved exactly and counts are consistent.
pub fn verify_unicode_content_preserved(result: &Result<FileInfo, FsError>, expected: &str) {
    let info = result.as_ref().expect("expected Ok(FileInfo)");

    // Exact string equality (ensures UTF-8 preserved).
    assert_eq!(
        info.content, expected,
        "Unicode content should be preserved exactly"
    );

    // Char/line counts align with the content.
    assert_eq!(
        info.char_count,
        expected.chars().count(),
        "char_count should match number of Unicode scalar values"
    );
    assert_eq!(
        info.line_count,
        expected.lines().count(),
        "line_count should match .lines() count"
    );

    // File size should be >= content bytes (metadata reports actual file length).
    assert!(
        info.size_bytes as usize >= expected.as_bytes().len(),
        "size_bytes ({}) should be >= UTF-8 byte length ({})",
        info.size_bytes,
        expected.as_bytes().len()
    );
}

use crate::FsError;
use std::{fs, path::Path};

/// Basic text file read with size/char/line counts.
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub content: String,
    pub size_bytes: u64,
    pub char_count: usize,
    pub line_count: usize,
}

/// Reads at most `max_bytes` (0 = unlimited). Returns UTF-8 text; if it’s not valid
/// UTF-8, we lossily convert with replacement to keep UI stable.
pub fn read_text_file(path: &Path, max_bytes: usize) -> Result<FileInfo, FsError> {
    let md = fs::metadata(path)?;
    let size_bytes = md.len();

    let mut bytes = if max_bytes == 0 || size_bytes as usize <= max_bytes {
        fs::read(path)?
    } else {
        let mut f = fs::File::open(path)?;
        let mut buf = vec![0u8; max_bytes];
        use std::io::Read;
        let _n = f.read(&mut buf)?;
        buf
    };

    // Try strict UTF-8, fall back to lossy
    let content = match String::from_utf8(bytes) {
        Ok(s) => s,
        Err(e) => {
            bytes = e.into_bytes();
            String::from_utf8_lossy(&bytes).into_owned()
        }
    };

    let char_count = content.chars().count();
    let line_count = content.lines().count();

    Ok(FileInfo { content, size_bytes, char_count, line_count })
}

//! File-based template source.

use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileSource {
    pub(crate) path: PathBuf,
}

impl FileSource {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self { path: path.as_ref().to_path_buf() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
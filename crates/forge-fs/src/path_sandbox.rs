use std::path::{Component, Path, PathBuf};

use crate::FsError;

/// Join a repo-relative path safely, preventing `..` escapes.
/// Returns an absolute path inside `root`.
pub fn resolve_repo_rel(root: &Path, rel: &str) -> Result<PathBuf, FsError> {
    let root = std::fs::canonicalize(root).map_err(|_| FsError::InvalidPath)?;
    let mut acc = PathBuf::new();
    for comp in Path::new(rel).components() {
        match comp {
            Component::Prefix(_) | Component::RootDir => return Err(FsError::InvalidPath),
            Component::CurDir => {}
            Component::ParentDir => return Err(FsError::InvalidPath),
            Component::Normal(seg) => acc.push(seg),
        }
    }
    let joined = root.join(acc);
    // Ensure joined starts with root
    let joined_can = std::fs::canonicalize(&joined).unwrap_or(joined.clone());
    if !joined_can.starts_with(&root) {
        return Err(FsError::InvalidPath);
    }
    Ok(joined)
}

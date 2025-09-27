use ignore::{DirEntry, Walk, WalkBuilder};
use std::path::Path;

/// Thin wrapper that returns a configured gitignore-aware walker.
pub fn build_gitignore_walk(root: &Path, max_depth: Option<usize>) -> Walk {
    let mut b = WalkBuilder::new(root);
    b.hidden(false)
        .ignore(true)
        .git_ignore(true)
        .git_exclude(true)
        .git_global(true);
    if let Some(d) = max_depth {
        b.max_depth(Some(d));
    }
    b.build()
}

/// Convenience shim for external consumption.
pub type GitDirEntry = DirEntry;

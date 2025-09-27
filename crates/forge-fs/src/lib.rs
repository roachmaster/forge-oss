//! forge-fs: generic filesystem helpers (no VM / no web / no HTTP)

pub mod errors;
pub mod file_read;
pub mod gitignore_walk;
pub mod path_sandbox;
pub mod repo_id;
pub mod tree_builder;

pub use errors::FsError;
pub use file_read::{read_text_file, FileInfo};
pub use gitignore_walk::{build_gitignore_walk, GitDirEntry};
pub use path_sandbox::resolve_repo_rel;
pub use repo_id::compute_repo_id;

// tree_builder re-exports
pub use tree_builder::{SimpleNode, build_simple_tree, sort_dirs_first};

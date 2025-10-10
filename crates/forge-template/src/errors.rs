//! Common error types for rendering and template handling.

use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum RenderError {
    #[error("I/O error at {path:?}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("template parse error at {location}")]
    Parse { location: String },

    #[error("missing partial: {name}")]
    MissingPartial { name: String },

    #[error("missing key: {key_path} (policy=Error)")]
    MissingKey { key_path: String },

    #[error("invalid template: {msg}")]
    InvalidTemplate { msg: String },

    #[error("serde conversion failed: {0}")]
    Serde(String),
}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, RenderError>;
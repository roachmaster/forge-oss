use std::{io, string};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FsError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("invalid path")]
    InvalidPath,

    #[error("utf8 error: {0}")]
    Utf8(#[from] string::FromUtf8Error),
}

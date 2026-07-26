use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("item already exists")]
    Duplicate,
    #[error("failed to get lock")]
    Lock,
    #[error("item does not exist")]
    ItemDoesNotExist,
    #[error("failed to serialize")]
    Serialize(#[from] serde_json::Error),
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("{operation:?} failed for path {path:?}: {context:?}")]
    Path {
        operation: String,
        path: PathBuf,
        context: String,
    },
}
pub type Result<T> = std::result::Result<T, Error>;

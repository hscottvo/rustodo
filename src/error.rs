use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("item already exists")]
    Duplicate,
    #[error("failed to get lock")]
    Lock,
    #[error("item does not exist")]
    ItemDoesNotExist,
}
pub type Result<T> = std::result::Result<T, Error>;

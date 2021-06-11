use thiserror::Error;
#[derive(Error, Debug)]
pub enum CacheError {
    #[error("io error")]
    Io(#[from] std::io::Error),
}
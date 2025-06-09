use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

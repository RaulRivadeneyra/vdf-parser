use thiserror::Error;

#[derive(Debug, Error)]
pub enum VdfError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

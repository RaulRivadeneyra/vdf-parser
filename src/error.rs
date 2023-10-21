use thiserror::Error;

#[derive(Debug, Error)]
pub enum VdfError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Couldn't get value '{0}' from block '{1}'")]
    ValueNotFound(String, String),
}

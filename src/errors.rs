use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScError {
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Project not found: {0}")]
    ProjectNotFound(String),
    #[error("Git operation failed: {0}")]
    Git(#[from] git2::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

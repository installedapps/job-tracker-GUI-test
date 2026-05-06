use thiserror::Error;

#[derive(Debug, Error)]
pub enum JobTrackerError {
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("file error: {0}")]
    File(#[from] std::io::Error),
    #[error("invalid status: {0}")]
    InvalidStatus(String),
    #[error("job id is required for this operation")]
    MissingJobId,
    #[error("{0}")]
    Validation(String),
}

use std::{io, path::PathBuf, process::ExitStatus};

/// Error types for Mustang CLI operations
#[derive(Debug, thiserror::Error)]
pub enum MustangError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Mustang CLI execution failed: {status}")]
    ExecutionFailed {
        status: ExitStatus,
        stdout: String,
        stderr: String,
    },

    #[error("Mustang CLI executable file not found: {0}")]
    ExecutableNotFound(PathBuf),

    #[error("Invalid file path: {0}")]
    InvalidPath(PathBuf),

    #[error("File does not exist: {0}")]
    FileNotFound(PathBuf),

    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    #[error("Invalid parameter value: {0}")]
    InvalidParameter(String),

    #[error("Temporary file operation failed: {0}")]
    TempFile(String),
}

/// Result type alias for Mustang operations
pub type Result<T> = std::result::Result<T, MustangError>;

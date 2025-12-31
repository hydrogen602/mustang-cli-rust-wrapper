use std::{io, path::PathBuf, process::ExitStatus};

/// Error types for Mustang CLI operations
#[derive(Debug, thiserror::Error)]
pub enum MustangError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Mustang CLI execution failed: {status}\n\n{stdout}\n\n{stderr}")]
    ExecutionFailed {
        status: ExitStatus,
        stdout: String,
        stderr: String,
    },

    #[error("Mustang CLI or java file not found: {0}")]
    ExecutableOrJavaNotFound(io::Error),

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

    #[error("File already exists: {0} but overwrite is not allowed")]
    FileAlreadyExists(PathBuf),

    #[error("File is a directory: {0}")]
    FileIsDirectory(PathBuf),
}

/// Result type alias for Mustang operations
pub type Result<T> = std::result::Result<T, MustangError>;

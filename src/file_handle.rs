use crate::error::{MustangError, Result};
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::{NamedTempFile, TempPath};

/// Input file handle that can be either a direct file path or temporary file from bytes
pub enum FileInput {
    /// Direct file path
    Path(PathBuf),
    /// Temporary file created from bytes
    Temp(TempPath),
}

impl FileInput {
    /// Create from a file path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            return Err(MustangError::FileNotFound(path.clone()));
        }
        Ok(Self::Path(path))
    }

    /// Create from bytes (creates a temporary file)
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let mut temp_file = NamedTempFile::new()
            .map_err(|e| MustangError::TempFile(format!("Failed to create temp file: {}", e)))?;
        std::io::Write::write_all(&mut temp_file, data)
            .map_err(|e| MustangError::TempFile(format!("Failed to write to temp file: {}", e)))?;
        temp_file
            .flush()
            .map_err(|e| MustangError::TempFile(format!("Failed to flush temp file: {}", e)))?;
        let temp_path = temp_file.into_temp_path();
        Ok(Self::Temp(temp_path))
    }

    /// Get the path to the file (for use with CLI)
    pub fn path(&self) -> &Path {
        match self {
            Self::Path(p) => p,
            Self::Temp(t) => t.as_ref(),
        }
    }
}

impl AsRef<OsStr> for FileInput {
    fn as_ref(&self) -> &OsStr {
        self.path().as_os_str()
    }
}

/// Output file handle that can be either a direct file path or temporary file
pub enum FileOutput {
    /// Direct file path
    Path(PathBuf),
    /// Temporary file (will be read into bytes)
    Temp(TempPath),
}

impl AsRef<OsStr> for FileOutput {
    fn as_ref(&self) -> &OsStr {
        self.path().as_os_str()
    }
}

impl FileOutput {
    /// Create from a file path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Self::Path(path.as_ref().to_path_buf())
    }

    /// Create as a temporary file
    pub fn temp() -> Result<Self> {
        let temp_file = NamedTempFile::new()
            .map_err(|e| MustangError::TempFile(format!("Failed to create temp file: {}", e)))?;
        let temp_path = temp_file.into_temp_path();
        // delete the temp file now as mustang refuses to overwrite an existing file
        fs::remove_file(&temp_path)
            .map_err(|e| MustangError::TempFile(format!("Failed to delete temp file: {}", e)))?;
        Ok(Self::Temp(temp_path))
    }

    /// Get the path to the file (for use with CLI)
    pub fn path(&self) -> &Path {
        match self {
            Self::Path(p) => p,
            Self::Temp(t) => t.as_ref(),
        }
    }

    /// Read the output file into bytes
    pub fn read_bytes(&self) -> Result<Vec<u8>> {
        std::fs::read(self.path()).map_err(MustangError::from)
    }
}

#[cfg(test)]
impl From<FileOutput> for FileInput {
    fn from(output: FileOutput) -> Self {
        match output {
            FileOutput::Temp(t) => Self::Temp(t),
            FileOutput::Path(p) => Self::Path(p),
        }
    }
}

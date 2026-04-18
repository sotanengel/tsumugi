use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MediaIoError {
    #[error("ffprobe not found in PATH")]
    FfprobeNotFound,

    #[error("ffprobe failed for {path}: {message}")]
    ProbeFailed { path: PathBuf, message: String },

    #[error("failed to parse ffprobe output: {0}")]
    ParseError(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("file not found: {0}")]
    FileNotFound(PathBuf),
}

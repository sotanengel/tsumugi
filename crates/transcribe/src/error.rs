use thiserror::Error;

#[derive(Debug, Error)]
pub enum TranscribeError {
    #[error("transcription not yet implemented (Phase 2)")]
    NotImplemented,

    #[error("model not found: {0}")]
    ModelNotFound(String),

    #[error("audio file error: {0}")]
    AudioError(String),

    #[error("transcription failed: {0}")]
    TranscriptionFailed(String),
}

pub mod error;

pub use error::TranscribeError;

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Configuration for transcription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscribeConfig {
    /// Language code (e.g., "en", "ja"). None for auto-detect.
    pub language: Option<String>,
    /// Path to the whisper model file.
    pub model_path: String,
}

/// A single transcribed segment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    /// Start time in seconds.
    pub start: f64,
    /// End time in seconds.
    pub end: f64,
    /// Transcribed text.
    pub text: String,
}

/// Transcribe an audio file into text segments.
///
/// This is a stub — whisper.cpp integration will be added in Phase 2.
pub fn transcribe(
    _config: &TranscribeConfig,
    _audio_path: &Path,
) -> Result<Vec<Segment>, TranscribeError> {
    Err(TranscribeError::NotImplemented)
}

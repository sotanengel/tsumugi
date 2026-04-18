use serde::{Deserialize, Serialize};

/// Information about a media file, obtained from ffprobe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaInfo {
    pub path: String,
    pub duration_secs: f64,
    pub format_name: String,
    pub streams: Vec<StreamInfo>,
}

/// Information about a single stream within a media file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    pub index: u32,
    pub kind: StreamKind,
    pub codec_name: String,
    /// Width in pixels (video only).
    pub width: Option<u32>,
    /// Height in pixels (video only).
    pub height: Option<u32>,
    /// Frames per second (video only).
    pub fps: Option<f64>,
    /// Sample rate in Hz (audio only).
    pub sample_rate: Option<u32>,
    /// Number of audio channels (audio only).
    pub channels: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamKind {
    Video,
    Audio,
    Subtitle,
    Other,
}

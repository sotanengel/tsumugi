use std::path::Path;
use std::process::Command;

use crate::error::MediaIoError;

/// Export preset configuration.
#[derive(Debug, Clone)]
pub struct ExportConfig {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub codec: String,
    pub bitrate: String,
    pub audio_codec: String,
    pub audio_bitrate: String,
}

impl ExportConfig {
    /// SNS preset: 1080p H.264 for social media
    pub fn sns_1080p() -> Self {
        Self {
            width: 1080,
            height: 1920,
            fps: 30,
            codec: "libx264".into(),
            bitrate: "8M".into(),
            audio_codec: "aac".into(),
            audio_bitrate: "192k".into(),
        }
    }

    /// YouTube preset: 1080p H.264
    pub fn youtube_1080p() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fps: 30,
            codec: "libx264".into(),
            bitrate: "12M".into(),
            audio_codec: "aac".into(),
            audio_bitrate: "256k".into(),
        }
    }

    /// YouTube preset: 4K H.265
    pub fn youtube_4k() -> Self {
        Self {
            width: 3840,
            height: 2160,
            fps: 30,
            codec: "libx265".into(),
            bitrate: "35M".into(),
            audio_codec: "aac".into(),
            audio_bitrate: "320k".into(),
        }
    }
}

/// Export a single input file with the given config.
/// This is a Phase 0 PoC — Phase 1 will render from the timeline.
pub fn export_file(
    input: &Path,
    output: &Path,
    config: &ExportConfig,
) -> Result<(), MediaIoError> {
    if !input.exists() {
        return Err(MediaIoError::FileNotFound(input.to_path_buf()));
    }

    let result = Command::new("ffmpeg")
        .args(["-y", "-i"])
        .arg(input)
        .args([
            "-c:v",
            &config.codec,
            "-b:v",
            &config.bitrate,
            "-vf",
            &format!("scale={}:{},fps={}", config.width, config.height, config.fps),
            "-c:a",
            &config.audio_codec,
            "-b:a",
            &config.audio_bitrate,
            "-movflags",
            "+faststart",
        ])
        .arg(output)
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                MediaIoError::FfmpegNotFound
            } else {
                MediaIoError::Io(e)
            }
        })?;

    if !result.status.success() {
        return Err(MediaIoError::EncodeFailed {
            message: String::from_utf8_lossy(&result.stderr).to_string(),
        });
    }

    Ok(())
}

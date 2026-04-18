use std::path::Path;
use std::process::Command;

use crate::error::MediaIoError;
use crate::model::{MediaInfo, StreamInfo, StreamKind};

/// Probe a media file using ffprobe and return its metadata.
pub fn probe_file(path: &Path) -> Result<MediaInfo, MediaIoError> {
    if !path.exists() {
        return Err(MediaIoError::FileNotFound(path.to_path_buf()));
    }

    let output = Command::new("ffprobe")
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
        ])
        .arg(path)
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                MediaIoError::FfprobeNotFound
            } else {
                MediaIoError::Io(e)
            }
        })?;

    if !output.status.success() {
        return Err(MediaIoError::ProbeFailed {
            path: path.to_path_buf(),
            message: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| MediaIoError::ParseError(e.to_string()))?;

    parse_probe_output(&json, path)
}

fn parse_probe_output(json: &serde_json::Value, path: &Path) -> Result<MediaInfo, MediaIoError> {
    let format = json
        .get("format")
        .ok_or_else(|| MediaIoError::ParseError("missing 'format' field".into()))?;

    let duration_secs = format
        .get("duration")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);

    let format_name = format
        .get("format_name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let streams = json
        .get("streams")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(parse_stream).collect())
        .unwrap_or_default();

    Ok(MediaInfo {
        path: path.to_string_lossy().to_string(),
        duration_secs,
        format_name,
        streams,
    })
}

fn parse_stream(stream: &serde_json::Value) -> Option<StreamInfo> {
    let codec_type = stream.get("codec_type")?.as_str()?;
    let kind = match codec_type {
        "video" => StreamKind::Video,
        "audio" => StreamKind::Audio,
        "subtitle" => StreamKind::Subtitle,
        _ => StreamKind::Other,
    };

    Some(StreamInfo {
        index: stream.get("index")?.as_u64()? as u32,
        kind,
        codec_name: stream
            .get("codec_name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string(),
        width: stream.get("width").and_then(|v| v.as_u64()).map(|v| v as u32),
        height: stream.get("height").and_then(|v| v.as_u64()).map(|v| v as u32),
        fps: parse_fps(stream),
        sample_rate: stream
            .get("sample_rate")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok()),
        channels: stream.get("channels").and_then(|v| v.as_u64()).map(|v| v as u32),
    })
}

fn parse_fps(stream: &serde_json::Value) -> Option<f64> {
    let r_frame_rate = stream.get("r_frame_rate")?.as_str()?;
    let parts: Vec<&str> = r_frame_rate.split('/').collect();
    if parts.len() == 2 {
        let num: f64 = parts[0].parse().ok()?;
        let den: f64 = parts[1].parse().ok()?;
        if den > 0.0 { Some(num / den) } else { None }
    } else {
        r_frame_rate.parse().ok()
    }
}

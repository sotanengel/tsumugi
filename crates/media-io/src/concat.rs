use std::fs;
use std::path::Path;
use std::process::Command;

use crate::error::MediaIoError;

/// Concatenate multiple media files into a single output file using FFmpeg's
/// concat demuxer. This is a PoC — Phase 1 will use filter_complex for
/// more precise control.
///
/// # Arguments
/// * `inputs` - Paths to input media files (must have compatible codecs)
/// * `output` - Path for the concatenated output file
pub fn concat_files(inputs: &[&Path], output: &Path) -> Result<(), MediaIoError> {
    if inputs.is_empty() {
        return Err(MediaIoError::ProbeFailed {
            path: output.to_path_buf(),
            message: "no input files provided".into(),
        });
    }

    if inputs.len() == 1 {
        fs::copy(inputs[0], output)?;
        return Ok(());
    }

    // Create a temporary concat list file
    let list_path = output.with_extension("concat.txt");
    let list_content: String = inputs
        .iter()
        .map(|p| {
            format!(
                "file '{}'",
                p.canonicalize()
                    .unwrap_or_else(|_| p.to_path_buf())
                    .display()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(&list_path, &list_content)?;

    let result = Command::new("ffmpeg")
        .args([
            "-y",
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
        ])
        .arg(&list_path)
        .args(["-c", "copy"])
        .arg(output)
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                MediaIoError::FfmpegNotFound
            } else {
                MediaIoError::Io(e)
            }
        })?;

    // Clean up the temp list file
    let _ = fs::remove_file(&list_path);

    if !result.status.success() {
        return Err(MediaIoError::EncodeFailed {
            message: String::from_utf8_lossy(&result.stderr).to_string(),
        });
    }

    Ok(())
}

/// Encode a portion of a media file to a new file (trim + re-encode).
/// Used for extracting clips from source media.
pub fn encode_clip(
    input: &Path,
    output: &Path,
    start_secs: f64,
    duration_secs: f64,
    codec: &str,
) -> Result<(), MediaIoError> {
    if !input.exists() {
        return Err(MediaIoError::FileNotFound(input.to_path_buf()));
    }

    let result = Command::new("ffmpeg")
        .args([
            "-y",
            "-ss",
            &format!("{start_secs:.3}"),
            "-i",
        ])
        .arg(input)
        .args([
            "-t",
            &format!("{duration_secs:.3}"),
            "-c:v",
            codec,
            "-c:a",
            "aac",
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

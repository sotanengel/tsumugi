use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::State;

use timeline_engine::{
    add_clip, add_track, make_clip, remove_clip, split_clip, ClipKind, Timeline,
};

/// Application state holding the current timeline.
pub struct AppState {
    pub timeline: Mutex<Timeline>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            timeline: Mutex::new(Timeline::new(30)),
        }
    }
}

// -- Timeline commands --

#[derive(Serialize)]
pub struct TimelineDto {
    pub id: String,
    pub fps: u32,
    pub tracks: Vec<TrackDto>,
}

#[derive(Serialize)]
pub struct TrackDto {
    pub id: String,
    pub name: String,
    pub clips: Vec<ClipDto>,
    pub muted: bool,
    pub locked: bool,
}

#[derive(Serialize)]
pub struct ClipDto {
    pub id: String,
    pub kind: String,
    pub path: Option<String>,
    pub text: Option<String>,
    pub timeline_start: u64,
    pub timeline_end: u64,
    pub source_start: u64,
    pub source_end: u64,
}

fn timeline_to_dto(tl: &Timeline) -> TimelineDto {
    TimelineDto {
        id: tl.id.to_string(),
        fps: tl.fps,
        tracks: tl
            .tracks
            .iter()
            .map(|t| TrackDto {
                id: t.id.to_string(),
                name: t.name.clone(),
                muted: t.muted,
                locked: t.locked,
                clips: t
                    .clips
                    .iter()
                    .map(|c| {
                        let (kind, path, text) = match &c.kind {
                            ClipKind::Video { path } => ("video", Some(path.clone()), None),
                            ClipKind::Audio { path } => ("audio", Some(path.clone()), None),
                            ClipKind::Title { text } => ("title", None, Some(text.clone())),
                        };
                        ClipDto {
                            id: c.id.to_string(),
                            kind: kind.to_string(),
                            path,
                            text,
                            timeline_start: c.timeline_range.start,
                            timeline_end: c.timeline_range.end,
                            source_start: c.source_range.start,
                            source_end: c.source_range.end,
                        }
                    })
                    .collect(),
            })
            .collect(),
    }
}

#[tauri::command]
pub fn get_timeline(state: State<AppState>) -> TimelineDto {
    let tl = state.timeline.lock().unwrap();
    timeline_to_dto(&tl)
}

#[tauri::command]
pub fn create_new_timeline(state: State<AppState>, fps: u32) -> TimelineDto {
    let mut tl = state.timeline.lock().unwrap();
    *tl = Timeline::new(fps);
    timeline_to_dto(&tl)
}

#[tauri::command]
pub fn add_track_cmd(state: State<AppState>, name: String) -> Result<TimelineDto, String> {
    let mut tl = state.timeline.lock().unwrap();
    add_track(&mut tl, name);
    Ok(timeline_to_dto(&tl))
}

#[derive(Deserialize)]
pub struct AddClipArgs {
    pub track_id: String,
    pub kind: String,
    pub path: Option<String>,
    pub text: Option<String>,
    pub start: u64,
    pub end: u64,
}

#[tauri::command]
pub fn add_clip_cmd(state: State<AppState>, args: AddClipArgs) -> Result<TimelineDto, String> {
    let mut tl = state.timeline.lock().unwrap();
    let track_id: uuid::Uuid = args.track_id.parse().map_err(|e| format!("{e}"))?;

    let kind = match args.kind.as_str() {
        "video" => ClipKind::Video {
            path: args.path.unwrap_or_default(),
        },
        "audio" => ClipKind::Audio {
            path: args.path.unwrap_or_default(),
        },
        "title" => ClipKind::Title {
            text: args.text.unwrap_or_default(),
        },
        other => return Err(format!("unknown clip kind: {other}")),
    };

    let clip = make_clip(kind, args.start, args.end);
    {
        let track = timeline_engine::ops::find_track_mut(&mut tl, track_id)
            .map_err(|e| e.to_string())?;
        add_clip(track, clip).map_err(|e| e.to_string())?;
    }
    Ok(timeline_to_dto(&tl))
}

#[tauri::command]
pub fn remove_clip_cmd(
    state: State<AppState>,
    track_id: String,
    clip_id: String,
) -> Result<TimelineDto, String> {
    let mut tl = state.timeline.lock().unwrap();
    let tid: uuid::Uuid = track_id.parse().map_err(|e| format!("{e}"))?;
    let cid: uuid::Uuid = clip_id.parse().map_err(|e| format!("{e}"))?;
    {
        let track = timeline_engine::ops::find_track_mut(&mut tl, tid)
            .map_err(|e| e.to_string())?;
        remove_clip(track, cid).map_err(|e| e.to_string())?;
    }
    Ok(timeline_to_dto(&tl))
}

#[tauri::command]
pub fn split_clip_cmd(
    state: State<AppState>,
    track_id: String,
    clip_id: String,
    at_frame: u64,
) -> Result<TimelineDto, String> {
    let mut tl = state.timeline.lock().unwrap();
    let tid: uuid::Uuid = track_id.parse().map_err(|e| format!("{e}"))?;
    let cid: uuid::Uuid = clip_id.parse().map_err(|e| format!("{e}"))?;
    {
        let track = timeline_engine::ops::find_track_mut(&mut tl, tid)
            .map_err(|e| e.to_string())?;
        split_clip(track, cid, at_frame).map_err(|e| e.to_string())?;
    }
    Ok(timeline_to_dto(&tl))
}

// -- Media commands --

#[derive(Serialize)]
pub struct MediaInfoDto {
    pub path: String,
    pub duration_secs: f64,
    pub format_name: String,
    pub has_video: bool,
    pub has_audio: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fps: Option<f64>,
}

#[tauri::command]
pub fn probe_media(path: String) -> Result<MediaInfoDto, String> {
    let info = media_io::probe_file(&PathBuf::from(&path)).map_err(|e| e.to_string())?;
    let video_stream = info
        .streams
        .iter()
        .find(|s| s.kind == media_io::StreamKind::Video);
    let has_audio = info
        .streams
        .iter()
        .any(|s| s.kind == media_io::StreamKind::Audio);

    Ok(MediaInfoDto {
        path: info.path,
        duration_secs: info.duration_secs,
        format_name: info.format_name,
        has_video: video_stream.is_some(),
        has_audio,
        width: video_stream.and_then(|s| s.width),
        height: video_stream.and_then(|s| s.height),
        fps: video_stream.and_then(|s| s.fps),
    })
}

#[derive(Deserialize)]
pub struct ExportArgs {
    pub input: String,
    pub output: String,
    pub preset: String,
}

#[tauri::command]
pub fn export_media(args: ExportArgs) -> Result<String, String> {
    let config = match args.preset.as_str() {
        "sns_1080p" => media_io::ExportConfig::sns_1080p(),
        "youtube_1080p" => media_io::ExportConfig::youtube_1080p(),
        "youtube_4k" => media_io::ExportConfig::youtube_4k(),
        other => return Err(format!("unknown preset: {other}")),
    };

    media_io::export_file(
        &PathBuf::from(&args.input),
        &PathBuf::from(&args.output),
        &config,
    )
    .map_err(|e| e.to_string())?;

    Ok(args.output)
}

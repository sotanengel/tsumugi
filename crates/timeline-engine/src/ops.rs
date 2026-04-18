use uuid::Uuid;

use crate::error::TimelineError;
use crate::model::{Clip, ClipKind, TimeRange, Timeline, Track};

/// Add a new track to the timeline. Returns the track's ID.
pub fn add_track(timeline: &mut Timeline, name: impl Into<String>) -> Uuid {
    let track = Track::new(name);
    let id = track.id;
    timeline.tracks.push(track);
    id
}

/// Find a track by ID, returning a mutable reference.
pub fn find_track_mut(
    timeline: &mut Timeline,
    track_id: Uuid,
) -> Result<&mut Track, TimelineError> {
    timeline
        .tracks
        .iter_mut()
        .find(|t| t.id == track_id)
        .ok_or(TimelineError::TrackNotFound(track_id))
}

/// Add a clip to a track. Validates no overlap and track is not locked.
pub fn add_clip(track: &mut Track, clip: Clip) -> Result<Uuid, TimelineError> {
    if track.locked {
        return Err(TimelineError::TrackLocked);
    }

    for existing in &track.clips {
        if existing.timeline_range.overlaps(&clip.timeline_range) {
            return Err(TimelineError::ClipOverlap);
        }
    }

    let id = clip.id;
    track.clips.push(clip);
    Ok(id)
}

/// Remove a clip from a track by ID. Returns the removed clip.
pub fn remove_clip(track: &mut Track, clip_id: Uuid) -> Result<Clip, TimelineError> {
    if track.locked {
        return Err(TimelineError::TrackLocked);
    }

    let pos = track
        .clips
        .iter()
        .position(|c| c.id == clip_id)
        .ok_or(TimelineError::ClipNotFound(clip_id))?;

    Ok(track.clips.remove(pos))
}

/// Split a clip at the given frame, producing two clips.
/// The original clip is shortened, and a new clip is returned.
pub fn split_clip(
    track: &mut Track,
    clip_id: Uuid,
    at_frame: u64,
) -> Result<Uuid, TimelineError> {
    if track.locked {
        return Err(TimelineError::TrackLocked);
    }

    let clip_idx = track
        .clips
        .iter()
        .position(|c| c.id == clip_id)
        .ok_or(TimelineError::ClipNotFound(clip_id))?;

    let clip = &track.clips[clip_idx];

    if at_frame <= clip.timeline_range.start || at_frame >= clip.timeline_range.end {
        return Err(TimelineError::InvalidTimeRange {
            start: clip.timeline_range.start,
            end: clip.timeline_range.end,
        });
    }

    // Copy values before mutating to satisfy the borrow checker
    let source_offset = at_frame - clip.timeline_range.start;
    let original_source_start = clip.source_range.start;
    let original_timeline_end = clip.timeline_range.end;
    let original_source_end = clip.source_range.end;
    let clip_kind = clip.kind.clone();

    let new_clip = Clip {
        id: Uuid::new_v4(),
        kind: clip_kind,
        timeline_range: TimeRange::new(at_frame, original_timeline_end),
        source_range: TimeRange::new(original_source_start + source_offset, original_source_end),
    };
    let new_id = new_clip.id;

    // Shorten original clip
    track.clips[clip_idx].timeline_range.end = at_frame;
    track.clips[clip_idx].source_range.end = original_source_start + source_offset;

    track.clips.push(new_clip);
    Ok(new_id)
}

/// Create a clip helper for tests and common usage.
pub fn make_clip(kind: ClipKind, start: u64, end: u64) -> Clip {
    Clip {
        id: Uuid::new_v4(),
        kind,
        timeline_range: TimeRange::new(start, end),
        source_range: TimeRange::new(0, end - start),
    }
}

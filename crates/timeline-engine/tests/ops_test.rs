use timeline_engine::*;

fn video_clip(start: u64, end: u64) -> Clip {
    make_clip(ClipKind::Video { path: "test.mp4".into() }, start, end)
}

#[test]
fn add_track_to_empty_timeline() {
    let mut tl = Timeline::new(30);
    let id = add_track(&mut tl, "Video 1");
    assert_eq!(tl.tracks.len(), 1);
    assert_eq!(tl.tracks[0].id, id);
    assert_eq!(tl.tracks[0].name, "Video 1");
}

#[test]
fn add_multiple_tracks() {
    let mut tl = Timeline::new(30);
    add_track(&mut tl, "Video");
    add_track(&mut tl, "Audio");
    add_track(&mut tl, "Title");
    assert_eq!(tl.tracks.len(), 3);
}

#[test]
fn add_clip_to_track() {
    let mut tl = Timeline::new(30);
    let track_id = add_track(&mut tl, "Video");
    let track = ops::find_track_mut(&mut tl, track_id).unwrap();
    let clip = video_clip(0, 100);
    let clip_id = add_clip(track, clip).unwrap();
    assert_eq!(track.clips.len(), 1);
    assert_eq!(track.clips[0].id, clip_id);
}

#[test]
fn add_non_overlapping_clips() {
    let mut tl = Timeline::new(30);
    let track_id = add_track(&mut tl, "Video");
    let track = ops::find_track_mut(&mut tl, track_id).unwrap();
    add_clip(track, video_clip(0, 100)).unwrap();
    add_clip(track, video_clip(100, 200)).unwrap();
    assert_eq!(track.clips.len(), 2);
}

#[test]
fn add_overlapping_clips_fails() {
    let mut tl = Timeline::new(30);
    let track_id = add_track(&mut tl, "Video");
    let track = ops::find_track_mut(&mut tl, track_id).unwrap();
    add_clip(track, video_clip(0, 100)).unwrap();
    let result = add_clip(track, video_clip(50, 150));
    assert!(matches!(result, Err(TimelineError::ClipOverlap)));
}

#[test]
fn add_clip_to_locked_track_fails() {
    let mut tl = Timeline::new(30);
    let track_id = add_track(&mut tl, "Video");
    let track = ops::find_track_mut(&mut tl, track_id).unwrap();
    track.locked = true;
    let result = add_clip(track, video_clip(0, 100));
    assert!(matches!(result, Err(TimelineError::TrackLocked)));
}

#[test]
fn remove_clip_from_track() {
    let mut tl = Timeline::new(30);
    let track_id = add_track(&mut tl, "Video");
    let track = ops::find_track_mut(&mut tl, track_id).unwrap();
    let clip_id = add_clip(track, video_clip(0, 100)).unwrap();
    let removed = remove_clip(track, clip_id).unwrap();
    assert_eq!(removed.id, clip_id);
    assert!(track.clips.is_empty());
}

#[test]
fn remove_nonexistent_clip_fails() {
    let mut tl = Timeline::new(30);
    let track_id = add_track(&mut tl, "Video");
    let track = ops::find_track_mut(&mut tl, track_id).unwrap();
    let result = remove_clip(track, uuid::Uuid::new_v4());
    assert!(matches!(result, Err(TimelineError::ClipNotFound(_))));
}

#[test]
fn split_clip_at_midpoint() {
    let mut tl = Timeline::new(30);
    let track_id = add_track(&mut tl, "Video");
    let track = ops::find_track_mut(&mut tl, track_id).unwrap();
    let clip_id = add_clip(track, video_clip(0, 100)).unwrap();
    let new_id = split_clip(track, clip_id, 50).unwrap();

    assert_eq!(track.clips.len(), 2);

    let original = track.clips.iter().find(|c| c.id == clip_id).unwrap();
    assert_eq!(original.timeline_range.start, 0);
    assert_eq!(original.timeline_range.end, 50);

    let new_clip = track.clips.iter().find(|c| c.id == new_id).unwrap();
    assert_eq!(new_clip.timeline_range.start, 50);
    assert_eq!(new_clip.timeline_range.end, 100);
}

#[test]
fn split_at_boundary_fails() {
    let mut tl = Timeline::new(30);
    let track_id = add_track(&mut tl, "Video");
    let track = ops::find_track_mut(&mut tl, track_id).unwrap();
    let clip_id = add_clip(track, video_clip(0, 100)).unwrap();

    // Split at start should fail
    assert!(split_clip(track, clip_id, 0).is_err());
    // Split at end should fail
    assert!(split_clip(track, clip_id, 100).is_err());
}

#[test]
fn time_range_overlap_detection() {
    let a = TimeRange::new(0, 100);
    let b = TimeRange::new(50, 150);
    let c = TimeRange::new(100, 200);

    assert!(a.overlaps(&b));
    assert!(b.overlaps(&a));
    assert!(!a.overlaps(&c)); // adjacent, not overlapping
    assert!(!c.overlaps(&a));
}

#[test]
fn time_range_duration() {
    let r = TimeRange::new(10, 60);
    assert_eq!(r.duration(), 50);
}

use timeline_engine::*;

#[test]
fn undo_restores_previous_state() {
    let mut tl = Timeline::new(30);
    let mut history = History::new();

    // Save state, then add a track
    history.save(&tl);
    add_track(&mut tl, "Video");
    assert_eq!(tl.tracks.len(), 1);

    // Undo should restore empty timeline
    let restored = history.undo(&tl).unwrap();
    assert_eq!(restored.tracks.len(), 0);
}

#[test]
fn redo_restores_undone_state() {
    let mut tl = Timeline::new(30);
    let mut history = History::new();

    history.save(&tl);
    add_track(&mut tl, "Video");
    assert_eq!(tl.tracks.len(), 1);

    // Undo
    let prev = history.undo(&tl).unwrap();
    assert_eq!(prev.tracks.len(), 0);

    // Redo should bring back the track
    let redone = history.redo(&prev).unwrap();
    assert_eq!(redone.tracks.len(), 1);
}

#[test]
fn multiple_undo_redo() {
    let mut tl = Timeline::new(30);
    let mut history = History::new();

    // Add 3 tracks with history
    history.save(&tl);
    add_track(&mut tl, "Track 1");

    history.save(&tl);
    add_track(&mut tl, "Track 2");

    history.save(&tl);
    add_track(&mut tl, "Track 3");

    assert_eq!(tl.tracks.len(), 3);

    // Undo twice
    tl = history.undo(&tl).unwrap(); // 2 tracks
    assert_eq!(tl.tracks.len(), 2);

    tl = history.undo(&tl).unwrap(); // 1 track
    assert_eq!(tl.tracks.len(), 1);

    // Redo once
    tl = history.redo(&tl).unwrap(); // 2 tracks
    assert_eq!(tl.tracks.len(), 2);
}

#[test]
fn new_action_clears_redo_stack() {
    let mut tl = Timeline::new(30);
    let mut history = History::new();

    history.save(&tl);
    add_track(&mut tl, "Track 1");

    // Undo
    tl = history.undo(&tl).unwrap();
    assert!(history.can_redo());

    // New action should clear redo
    history.save(&tl);
    add_track(&mut tl, "Track 2");
    assert!(!history.can_redo());
}

#[test]
fn undo_on_empty_returns_none() {
    let tl = Timeline::new(30);
    let mut history = History::new();
    assert!(history.undo(&tl).is_none());
    assert!(!history.can_undo());
}

#[test]
fn redo_on_empty_returns_none() {
    let tl = Timeline::new(30);
    let mut history = History::new();
    assert!(history.redo(&tl).is_none());
    assert!(!history.can_redo());
}

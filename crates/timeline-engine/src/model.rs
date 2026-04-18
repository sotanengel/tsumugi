use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A complete timeline containing multiple tracks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub id: Uuid,
    pub tracks: Vec<Track>,
    /// Duration in frames.
    pub fps: u32,
}

impl Timeline {
    pub fn new(fps: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            tracks: Vec::new(),
            fps,
        }
    }
}

/// A single track in the timeline (video, audio, or title).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: Uuid,
    pub name: String,
    pub clips: Vec<Clip>,
    pub muted: bool,
    pub locked: bool,
}

impl Track {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            clips: Vec::new(),
            muted: false,
            locked: false,
        }
    }
}

/// A clip placed on a track.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: Uuid,
    pub kind: ClipKind,
    /// Position on the timeline (in frames).
    pub timeline_range: TimeRange,
    /// Source range within the media file (in frames).
    pub source_range: TimeRange,
}

/// What type of media this clip represents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipKind {
    Video { path: String },
    Audio { path: String },
    Title { text: String },
}

/// A range of frames [start, end).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeRange {
    /// Inclusive start frame.
    pub start: u64,
    /// Exclusive end frame.
    pub end: u64,
}

impl TimeRange {
    pub fn new(start: u64, end: u64) -> Self {
        debug_assert!(start <= end, "start must be <= end");
        Self { start, end }
    }

    pub fn duration(&self) -> u64 {
        self.end - self.start
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }
}

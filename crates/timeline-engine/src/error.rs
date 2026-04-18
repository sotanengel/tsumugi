use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum TimelineError {
    #[error("track not found: {0}")]
    TrackNotFound(Uuid),

    #[error("clip not found: {0}")]
    ClipNotFound(Uuid),

    #[error("clip overlaps with existing clip")]
    ClipOverlap,

    #[error("track is locked")]
    TrackLocked,

    #[error("invalid time range: start {start} >= end {end}")]
    InvalidTimeRange { start: u64, end: u64 },
}

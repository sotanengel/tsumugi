pub mod error;
pub mod history;
pub mod model;
pub mod ops;

pub use error::TimelineError;
pub use history::History;
pub use model::{Clip, ClipKind, TimeRange, Timeline, Track};
pub use ops::{add_clip, add_track, make_clip, remove_clip, split_clip};

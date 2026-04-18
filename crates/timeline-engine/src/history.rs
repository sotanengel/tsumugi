use crate::model::Timeline;

/// Snapshot-based undo/redo history.
///
/// Each mutation saves a full timeline snapshot. Simple and correct
/// for Phase 0. Can be optimized to command-pattern diffs in Phase 1
/// if memory becomes a concern.
#[derive(Debug)]
pub struct History {
    undo_stack: Vec<Timeline>,
    redo_stack: Vec<Timeline>,
    max_size: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size: 100,
        }
    }

    /// Save the current timeline state before a mutation.
    /// Call this BEFORE modifying the timeline.
    pub fn save(&mut self, timeline: &Timeline) {
        self.undo_stack.push(timeline.clone());
        self.redo_stack.clear();
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    /// Undo: restore the previous timeline state.
    /// Returns the restored timeline, or None if nothing to undo.
    pub fn undo(&mut self, current: &Timeline) -> Option<Timeline> {
        let previous = self.undo_stack.pop()?;
        self.redo_stack.push(current.clone());
        Some(previous)
    }

    /// Redo: restore the next timeline state.
    /// Returns the restored timeline, or None if nothing to redo.
    pub fn redo(&mut self, current: &Timeline) -> Option<Timeline> {
        let next = self.redo_stack.pop()?;
        self.undo_stack.push(current.clone());
        Some(next)
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

import { useEffect } from "react";
import { useTimelineStore } from "../store/timeline-store";

const isMac = navigator.platform.toUpperCase().includes("MAC");

function isModKey(e: KeyboardEvent): boolean {
  return isMac ? e.metaKey : e.ctrlKey;
}

export function useKeyboardShortcuts() {
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      const state = useTimelineStore.getState();

      // Space: toggle playback
      if (e.code === "Space" && !isModKey(e)) {
        e.preventDefault();
        state.togglePlayback();
        return;
      }

      // Delete / Backspace: delete selected clip
      if ((e.code === "Delete" || e.code === "Backspace") && !isModKey(e)) {
        if (state.selectedClipId && state.selectedTrackId) {
          e.preventDefault();
          state.removeClip(state.selectedTrackId, state.selectedClipId);
          state.deselectClip();
        }
        return;
      }

      // Cmd+Z / Ctrl+Z: undo
      if (isModKey(e) && e.code === "KeyZ" && !e.shiftKey) {
        e.preventDefault();
        state.undo();
        return;
      }

      // Cmd+Shift+Z / Ctrl+Y: redo
      if ((isModKey(e) && e.code === "KeyZ" && e.shiftKey) ||
          (!isMac && e.ctrlKey && e.code === "KeyY")) {
        e.preventDefault();
        state.redo();
        return;
      }

      // Cmd+C: copy
      if (isModKey(e) && e.code === "KeyC" && !e.shiftKey) {
        if (state.selectedClipId && state.selectedTrackId) {
          e.preventDefault();
          state.copySelectedClip();
        }
        return;
      }

      // Cmd+X: cut
      if (isModKey(e) && e.code === "KeyX") {
        if (state.selectedClipId && state.selectedTrackId) {
          e.preventDefault();
          state.cutSelectedClip();
        }
        return;
      }

      // Cmd+V: paste
      if (isModKey(e) && e.code === "KeyV") {
        e.preventDefault();
        state.pasteClip();
        return;
      }
    };

    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, []);
}

import { create } from "zustand";
import type { AddClipArgs, MediaInfo, Timeline } from "@tsumugi/timeline-types";
import * as api from "./api";

interface TimelineState {
  timeline: Timeline | null;
  mediaLibrary: MediaInfo[];
  loading: boolean;
  error: string | null;

  // Actions
  loadTimeline: () => Promise<void>;
  createTimeline: (fps: number) => Promise<void>;
  addTrack: (name: string) => Promise<void>;
  removeTrack: (trackId: string) => Promise<void>;
  addClip: (args: AddClipArgs) => Promise<void>;
  removeClip: (trackId: string, clipId: string) => Promise<void>;
  splitClip: (trackId: string, clipId: string, atFrame: number) => Promise<void>;
  undo: () => Promise<void>;
  redo: () => Promise<void>;
  importMedia: (path: string) => Promise<void>;
  previewSource: string | null;
  setPreviewSource: (path: string | null) => void;
  selectedClipId: string | null;
  selectedTrackId: string | null;
  selectClip: (trackId: string, clipId: string) => void;
  deselectClip: () => void;
}

export const useTimelineStore = create<TimelineState>((set, get) => ({
  timeline: null,
  previewSource: null,
  selectedClipId: null,
  selectedTrackId: null,
  mediaLibrary: [],
  loading: false,
  error: null,

  loadTimeline: async () => {
    set({ loading: true, error: null });
    try {
      const timeline = await api.getTimeline();
      set({ timeline, loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  createTimeline: async (fps) => {
    set({ loading: true, error: null });
    try {
      const timeline = await api.createNewTimeline(fps);
      set({ timeline, loading: false, mediaLibrary: [] });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  addTrack: async (name) => {
    try {
      const timeline = await api.addTrack(name);
      set({ timeline });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  removeTrack: async (trackId) => {
    try {
      const timeline = await api.removeTrack(trackId);
      set({ timeline });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  addClip: async (args) => {
    try {
      const timeline = await api.addClip(args);
      set({ timeline });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  removeClip: async (trackId, clipId) => {
    try {
      const timeline = await api.removeClip(trackId, clipId);
      set({ timeline });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  splitClip: async (trackId, clipId, atFrame) => {
    try {
      const timeline = await api.splitClip(trackId, clipId, atFrame);
      set({ timeline });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  undo: async () => {
    try {
      const timeline = await api.undo();
      set({ timeline, error: null });
    } catch (_e) {
      // Silently ignore "nothing to undo"
    }
  },

  redo: async () => {
    try {
      const timeline = await api.redo();
      set({ timeline, error: null });
    } catch (_e) {
      // Silently ignore "nothing to redo"
    }
  },

  importMedia: async (path) => {
    try {
      const info = await api.probeMedia(path);
      set({ mediaLibrary: [...get().mediaLibrary, info] });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  setPreviewSource: (path) => {
    set({ previewSource: path });
  },

  selectClip: (trackId, clipId) => {
    const tl = get().timeline;
    if (!tl) return;
    const track = tl.tracks.find((t) => t.id === trackId);
    const clip = track?.clips.find((c) => c.id === clipId);
    set({
      selectedClipId: clipId,
      selectedTrackId: trackId,
      previewSource: clip?.path ?? get().previewSource,
    });
  },

  deselectClip: () => {
    set({ selectedClipId: null, selectedTrackId: null });
  },
}));

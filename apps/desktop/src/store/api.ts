import { invoke } from "@tauri-apps/api/core";
import type { AddClipArgs, MediaInfo, Timeline } from "@tsumugi/timeline-types";

export async function getTimeline(): Promise<Timeline> {
  return invoke<Timeline>("get_timeline");
}

export async function createNewTimeline(fps: number): Promise<Timeline> {
  return invoke<Timeline>("create_new_timeline", { fps });
}

export async function addTrack(name: string): Promise<Timeline> {
  return invoke<Timeline>("add_track_cmd", { name });
}

export async function removeTrack(trackId: string): Promise<Timeline> {
  return invoke<Timeline>("remove_track_cmd", { trackId });
}

export async function addClip(args: AddClipArgs): Promise<Timeline> {
  return invoke<Timeline>("add_clip_cmd", { args });
}

export async function removeClip(
  trackId: string,
  clipId: string,
): Promise<Timeline> {
  return invoke<Timeline>("remove_clip_cmd", { trackId, clipId });
}

export async function splitClip(
  trackId: string,
  clipId: string,
  atFrame: number,
): Promise<Timeline> {
  return invoke<Timeline>("split_clip_cmd", { trackId, clipId, atFrame });
}

export async function undo(): Promise<Timeline> {
  return invoke<Timeline>("undo_cmd");
}

export async function redo(): Promise<Timeline> {
  return invoke<Timeline>("redo_cmd");
}

export async function probeMedia(path: string): Promise<MediaInfo> {
  return invoke<MediaInfo>("probe_media", { path });
}

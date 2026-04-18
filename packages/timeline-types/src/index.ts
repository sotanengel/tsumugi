/** Matches Rust TimelineDto from commands.rs */
export interface Timeline {
  id: string;
  fps: number;
  tracks: Track[];
}

export interface Track {
  id: string;
  name: string;
  clips: Clip[];
  muted: boolean;
  locked: boolean;
}

export interface Clip {
  id: string;
  kind: ClipKind;
  path?: string;
  text?: string;
  timeline_start: number;
  timeline_end: number;
  source_start: number;
  source_end: number;
}

export type ClipKind = "video" | "audio" | "title";

/** Matches Rust MediaInfoDto from commands.rs */
export interface MediaInfo {
  path: string;
  duration_secs: number;
  format_name: string;
  has_video: boolean;
  has_audio: boolean;
  width?: number;
  height?: number;
  fps?: number;
}

/** Arguments for the add_clip_cmd Tauri command */
export interface AddClipArgs {
  track_id: string;
  kind: ClipKind;
  path?: string;
  text?: string;
  start: number;
  end: number;
}

/** Standard aspect ratios supported by Tsumugi */
export const ASPECT_RATIOS = {
  "9:16": { width: 1080, height: 1920, label: "TikTok / Reels / Shorts" },
  "1:1": { width: 1080, height: 1080, label: "Instagram Feed" },
  "4:5": { width: 1080, height: 1350, label: "Instagram Portrait" },
  "16:9": { width: 1920, height: 1080, label: "YouTube / Wedding" },
} as const;

export type AspectRatio = keyof typeof ASPECT_RATIOS;

import type { Track as TrackType } from "@tsumugi/timeline-types";
import { useTimelineStore } from "../store/timeline-store";

export const PIXELS_PER_FRAME = 2;

function TrackRow({ track }: { track: TrackType }) {
  const { removeClip, removeTrack } = useTimelineStore();

  return (
    <div className="flex border-b border-border-dim min-h-12">
      <div className="w-30 px-1.5 py-1.5 bg-bg-track border-r border-border-dim text-xs shrink-0 flex items-center justify-between">
        <div>
          <div className="font-semibold">{track.name}</div>
          <div className="text-text-dim text-[10px]">
            {track.clips.length} clip{track.clips.length !== 1 ? "s" : ""}
          </div>
        </div>
        <button
          type="button"
          onClick={() => removeTrack(track.id)}
          className="bg-transparent border-none text-text-dim cursor-pointer text-sm px-1 hover:text-error-text"
          title="Delete track"
        >
          ×
        </button>
      </div>
      <div className="flex-1 relative flex items-center">
        {track.clips.map((clip) => (
          <div
            key={clip.id}
            className="absolute h-9 rounded-sm px-1.5 py-0.5 text-[10px] overflow-hidden cursor-pointer border border-white/10"
            style={{
              left: clip.timeline_start * PIXELS_PER_FRAME,
              width: (clip.timeline_end - clip.timeline_start) * PIXELS_PER_FRAME,
              background: clip.kind === "video"
                ? "var(--color-bg-clip-video)"
                : clip.kind === "audio"
                  ? "var(--color-bg-clip-audio)"
                  : "var(--color-bg-clip-title)",
            }}
            onDoubleClick={() => removeClip(track.id, clip.id)}
            title={`${clip.kind}: ${clip.path || clip.text || ""} (double-click to remove)`}
          >
            <span>{clip.path?.split("/").pop() || clip.text || clip.kind}</span>
          </div>
        ))}
      </div>
    </div>
  );
}

export function TimelinePanel() {
  const { timeline } = useTimelineStore();

  if (!timeline) {
    return (
      <div className="p-5 text-center text-text-dim">
        Create a new project to get started
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-auto border-t border-border">
      {timeline.tracks.length === 0 && (
        <div className="p-4 text-center text-text-dim text-sm">
          Add a track to begin editing
        </div>
      )}
      {timeline.tracks.map((track) => (
        <TrackRow key={track.id} track={track} />
      ))}
    </div>
  );
}

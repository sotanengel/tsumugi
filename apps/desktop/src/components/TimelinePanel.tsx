import { useRef, useState } from "react";
import type { MediaInfo, Track as TrackType } from "@tsumugi/timeline-types";
import { useTimelineStore } from "../store/timeline-store";

export const PIXELS_PER_FRAME = 2;

function TrackRow({ track }: { track: TrackType }) {
  const { removeClip, removeTrack, addClip, timeline, selectClip, selectedClipId } = useTimelineStore();
  const [dragOver, setDragOver] = useState(false);
  const clipAreaRef = useRef<HTMLDivElement>(null);

  const handleDragOver = (e: React.DragEvent) => {
    if (e.dataTransfer.types.includes("application/tsumugi-media")) {
      e.preventDefault();
      e.dataTransfer.dropEffect = "copy";
      setDragOver(true);
    }
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setDragOver(false);
    const raw = e.dataTransfer.getData("application/tsumugi-media");
    if (!raw || !clipAreaRef.current || !timeline) return;

    const media: MediaInfo = JSON.parse(raw);
    const rect = clipAreaRef.current.getBoundingClientRect();
    const offsetX = e.clientX - rect.left;
    const startFrame = Math.max(0, Math.round(offsetX / PIXELS_PER_FRAME));
    const durationFrames = Math.round(media.duration_secs * timeline.fps);
    const endFrame = startFrame + durationFrames;

    const kind = media.has_video ? "video" : "audio";
    addClip({
      track_id: track.id,
      kind,
      path: media.path,
      start: startFrame,
      end: endFrame,
    });
  };

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
      <div
        ref={clipAreaRef}
        className={`flex-1 relative flex items-center transition-colors ${dragOver ? "bg-blue-500/10 ring-1 ring-blue-400/50" : ""}`}
        onDragOver={handleDragOver}
        onDragLeave={() => setDragOver(false)}
        onDrop={handleDrop}
      >
        {track.clips.map((clip) => (
          <div
            key={clip.id}
            className={`absolute h-9 rounded-sm px-1.5 py-0.5 text-[10px] overflow-hidden cursor-pointer border ${selectedClipId === clip.id ? "border-blue-400 ring-1 ring-blue-400/50" : "border-white/10"}`}
            style={{
              left: clip.timeline_start * PIXELS_PER_FRAME,
              width: (clip.timeline_end - clip.timeline_start) * PIXELS_PER_FRAME,
              background: clip.kind === "video"
                ? "var(--color-bg-clip-video)"
                : clip.kind === "audio"
                  ? "var(--color-bg-clip-audio)"
                  : "var(--color-bg-clip-title)",
            }}
            onClick={(e) => { e.stopPropagation(); selectClip(track.id, clip.id); }}
            onDoubleClick={() => removeClip(track.id, clip.id)}
            title={`${clip.kind}: ${clip.path || clip.text || ""}`}
          >
            <span>{clip.path?.split("/").pop() || clip.text || clip.kind}</span>
          </div>
        ))}
      </div>
    </div>
  );
}

export function TimelinePanel() {
  const { timeline, deselectClip } = useTimelineStore();

  if (!timeline) {
    return (
      <div className="p-5 text-center text-text-dim">
        Create a new project to get started
      </div>
    );
  }

  return (
    <div className="h-full overflow-auto" onClick={deselectClip}>
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

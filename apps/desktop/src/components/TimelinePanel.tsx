import type { Track as TrackType } from "@tsumugi/timeline-types";
import { useTimelineStore } from "../store/timeline-store";

function TrackRow({ track }: { track: TrackType }) {
  const { removeClip, removeTrack } = useTimelineStore();

  return (
    <div style={{
      display: "flex",
      borderBottom: "1px solid #222",
      minHeight: 48,
    }}>
      <div style={{
        width: 120,
        padding: "6px 6px",
        background: "#16213e",
        borderRight: "1px solid #222",
        fontSize: 12,
        flexShrink: 0,
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
      }}>
        <div>
          <div style={{ fontWeight: 600 }}>{track.name}</div>
          <div style={{ color: "#666", fontSize: 10 }}>
            {track.clips.length} clip{track.clips.length !== 1 ? "s" : ""}
          </div>
        </div>
        <button
          type="button"
          onClick={() => removeTrack(track.id)}
          style={{
            background: "none",
            border: "none",
            color: "#666",
            cursor: "pointer",
            fontSize: 14,
            padding: "2px 4px",
          }}
          title="Delete track"
        >
          ×
        </button>
      </div>
      <div style={{ flex: 1, position: "relative", display: "flex", alignItems: "center" }}>
        {track.clips.map((clip) => (
          <div
            key={clip.id}
            style={{
              position: "absolute",
              left: clip.timeline_start * 2,
              width: (clip.timeline_end - clip.timeline_start) * 2,
              height: 36,
              background: clip.kind === "video" ? "#0f3460" : clip.kind === "audio" ? "#1a5276" : "#4a235a",
              borderRadius: 3,
              padding: "2px 6px",
              fontSize: 10,
              overflow: "hidden",
              cursor: "pointer",
              border: "1px solid rgba(255,255,255,0.1)",
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
      <div style={{ padding: 20, textAlign: "center", color: "#555" }}>
        Create a new project to get started
      </div>
    );
  }

  return (
    <div style={{
      flex: 1,
      overflow: "auto",
      borderTop: "1px solid #333",
    }}>
      {timeline.tracks.length === 0 && (
        <div style={{ padding: 16, textAlign: "center", color: "#555", fontSize: 13 }}>
          Add a track to begin editing
        </div>
      )}
      {timeline.tracks.map((track) => (
        <TrackRow key={track.id} track={track} />
      ))}
    </div>
  );
}

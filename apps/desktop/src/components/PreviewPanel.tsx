import { useTimelineStore } from "../store/timeline-store";

export function PreviewPanel() {
  const { timeline } = useTimelineStore();

  return (
    <div style={{
      flex: 1,
      display: "flex",
      alignItems: "center",
      justifyContent: "center",
      background: "#000",
      minHeight: 300,
    }}>
      {timeline ? (
        <div style={{
          width: 480,
          height: 270,
          background: "#111",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          borderRadius: 4,
          border: "1px solid #333",
        }}>
          <span style={{ color: "#444", fontSize: 14 }}>
            Preview (16:9) — {timeline.fps}fps
          </span>
        </div>
      ) : (
        <span style={{ color: "#444" }}>No project loaded</span>
      )}
    </div>
  );
}

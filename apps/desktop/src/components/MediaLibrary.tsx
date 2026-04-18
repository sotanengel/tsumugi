import { useTimelineStore } from "../store/timeline-store";

export function MediaLibrary() {
  const { mediaLibrary } = useTimelineStore();

  return (
    <div style={{
      padding: 8,
      borderRight: "1px solid #333",
      width: 220,
      overflow: "auto",
    }}>
      <h3 style={{ margin: "0 0 8px", fontSize: 13, color: "#888" }}>
        Media ({mediaLibrary.length})
      </h3>
      {mediaLibrary.length === 0 && (
        <p style={{ color: "#555", fontSize: 12 }}>Import media to get started</p>
      )}
      {mediaLibrary.map((m) => (
        <div
          key={m.path}
          style={{
            padding: "6px 8px",
            marginBottom: 4,
            background: "#16213e",
            borderRadius: 4,
            fontSize: 12,
            cursor: "grab",
          }}
          title={m.path}
        >
          <div style={{ fontWeight: 600, overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
            {m.path.split("/").pop()}
          </div>
          <div style={{ color: "#888", marginTop: 2 }}>
            {m.duration_secs.toFixed(1)}s
            {m.width && m.height && ` · ${m.width}×${m.height}`}
          </div>
        </div>
      ))}
    </div>
  );
}

import { useTimelineStore } from "../store/timeline-store";

export function MediaLibrary() {
  const { mediaLibrary } = useTimelineStore();

  return (
    <div className="p-2 border-r border-border w-56 overflow-auto">
      <h3 className="m-0 mb-2 text-xs text-text-muted font-semibold">
        Media ({mediaLibrary.length})
      </h3>
      {mediaLibrary.length === 0 && (
        <p className="text-text-dim text-xs">Import media to get started</p>
      )}
      {mediaLibrary.map((m) => (
        <div
          key={m.path}
          className="p-1.5 mb-1 bg-bg-track rounded text-xs cursor-grab"
          title={m.path}
        >
          <div className="font-semibold truncate">
            {m.path.split("/").pop()}
          </div>
          <div className="text-text-muted mt-0.5">
            {m.duration_secs.toFixed(1)}s
            {m.width && m.height && ` · ${m.width}×${m.height}`}
          </div>
        </div>
      ))}
    </div>
  );
}

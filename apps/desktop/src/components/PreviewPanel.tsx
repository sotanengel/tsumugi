import { useTimelineStore } from "../store/timeline-store";

export function PreviewPanel() {
  const { timeline } = useTimelineStore();

  return (
    <div className="flex-1 flex items-center justify-center bg-black min-h-[300px]">
      {timeline ? (
        <div className="w-[480px] h-[270px] bg-bg-primary flex items-center justify-center rounded border border-border">
          <span className="text-text-dim text-sm">
            Preview (16:9) — {timeline.fps}fps
          </span>
        </div>
      ) : (
        <span className="text-text-dim">No project loaded</span>
      )}
    </div>
  );
}

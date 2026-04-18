import { useEffect } from "react";
import { MediaLibrary } from "./components/MediaLibrary";
import { PreviewPanel } from "./components/PreviewPanel";
import { TimelinePanel } from "./components/TimelinePanel";
import { Toolbar } from "./components/Toolbar";
import { useTimelineStore } from "./store/timeline-store";

export function App() {
  const { loadTimeline, error } = useTimelineStore();

  useEffect(() => {
    loadTimeline();
  }, [loadTimeline]);

  return (
    <div style={{
      display: "flex",
      flexDirection: "column",
      height: "100vh",
      background: "#0a0a1a",
      color: "#e0e0e0",
      fontFamily: "system-ui, -apple-system, sans-serif",
    }}>
      <Toolbar />
      {error && (
        <div style={{ padding: "4px 12px", background: "#4a0000", color: "#ff6b6b", fontSize: 12 }}>
          {error}
        </div>
      )}
      <div style={{ display: "flex", flex: 1, overflow: "hidden" }}>
        <MediaLibrary />
        <PreviewPanel />
      </div>
      <TimelinePanel />
    </div>
  );
}

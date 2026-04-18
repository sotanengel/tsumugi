import { useEffect } from "react";
import * as Tooltip from "@radix-ui/react-tooltip";
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
    <Tooltip.Provider delayDuration={300}>
      <div className="flex flex-col h-screen bg-bg-primary text-text-primary font-sans">
        <Toolbar />
        {error && (
          <div className="px-3 py-1 bg-error-bg text-error-text text-xs">
            {error}
          </div>
        )}
        <div className="flex flex-1 overflow-hidden">
          <MediaLibrary />
          <PreviewPanel />
        </div>
        <TimelinePanel />
      </div>
    </Tooltip.Provider>
  );
}

import { useEffect } from "react";
import * as Tooltip from "@radix-ui/react-tooltip";
import { MediaLibrary } from "./components/MediaLibrary";
import { PreviewPanel } from "./components/PreviewPanel";
import { SplitPane } from "./components/SplitPane";
import { TimelinePanel } from "./components/TimelinePanel";
import { Toolbar } from "./components/Toolbar";
import { useKeyboardShortcuts } from "./hooks/use-keyboard-shortcuts";
import { useTimelineStore } from "./store/timeline-store";

export function App() {
  const { loadTimeline, error } = useTimelineStore();

  useEffect(() => {
    loadTimeline();
  }, [loadTimeline]);

  useKeyboardShortcuts();

  return (
    <Tooltip.Provider delayDuration={300}>
      <div className="flex flex-col h-screen bg-bg-primary text-text-primary font-sans">
        <Toolbar />
        {error && (
          <div className="px-3 py-1 bg-error-bg text-error-text text-xs">
            {error}
          </div>
        )}
        {/* Vertical split: top (media+preview) | bottom (timeline) */}
        <SplitPane
          direction="vertical"
          initialSize={400}
          minSize={100}
          collapsible="both"
        >
          {/* Horizontal split: left (media library) | right (preview) */}
          <SplitPane
            direction="horizontal"
            initialSize={220}
            minSize={60}
            maxSize={400}
            collapsible="first"
          >
            <MediaLibrary />
            <PreviewPanel />
          </SplitPane>
          <TimelinePanel />
        </SplitPane>
      </div>
    </Tooltip.Provider>
  );
}

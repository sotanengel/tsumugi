import { useCallback, useRef, useState } from "react";

interface SplitPaneProps {
  direction: "horizontal" | "vertical";
  initialSize: number;
  minSize?: number;
  maxSize?: number;
  children: [React.ReactNode, React.ReactNode];
  collapsible?: "first" | "second" | "both";
}

export function SplitPane({
  direction,
  initialSize,
  minSize = 40,
  maxSize,
  children,
  collapsible,
}: SplitPaneProps) {
  const [size, setSize] = useState(initialSize);
  const [collapsed, setCollapsed] = useState<"first" | "second" | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const dragging = useRef(false);

  const isHorizontal = direction === "horizontal";

  const handleMouseDown = useCallback(
    (e: React.MouseEvent) => {
      e.preventDefault();
      dragging.current = true;

      const handleMouseMove = (e: MouseEvent) => {
        if (!dragging.current || !containerRef.current) return;
        const rect = containerRef.current.getBoundingClientRect();
        const pos = isHorizontal ? e.clientX - rect.left : e.clientY - rect.top;
        const total = isHorizontal ? rect.width : rect.height;
        const clamped = Math.max(minSize, Math.min(pos, maxSize ?? total - minSize));
        setSize(clamped);
        setCollapsed(null);
      };

      const handleMouseUp = () => {
        dragging.current = false;
        document.removeEventListener("mousemove", handleMouseMove);
        document.removeEventListener("mouseup", handleMouseUp);
      };

      document.addEventListener("mousemove", handleMouseMove);
      document.addEventListener("mouseup", handleMouseUp);
    },
    [isHorizontal, minSize, maxSize],
  );

  const toggleCollapse = (which: "first" | "second") => {
    if (collapsed === which) {
      setCollapsed(null);
    } else {
      setCollapsed(which);
    }
  };

  const canCollapseFirst = collapsible === "first" || collapsible === "both";
  const canCollapseSecond = collapsible === "second" || collapsible === "both";

  const firstStyle = collapsed === "first"
    ? { [isHorizontal ? "width" : "height"]: 0, overflow: "hidden" as const }
    : { [isHorizontal ? "width" : "height"]: size, flexShrink: 0 };

  const dividerCursor = isHorizontal ? "cursor-col-resize" : "cursor-row-resize";

  return (
    <div
      ref={containerRef}
      className={`flex ${isHorizontal ? "flex-row" : "flex-col"} flex-1 overflow-hidden`}
    >
      {/* First pane */}
      <div style={firstStyle} className={`overflow-auto ${collapsed === "first" ? "hidden" : ""}`}>
        {children[0]}
      </div>

      {/* Divider */}
      <div
        className={`${isHorizontal ? "w-1" : "h-1"} bg-border ${dividerCursor} hover:bg-blue-500/50 active:bg-blue-500 flex items-center justify-center shrink-0 relative group`}
        onMouseDown={handleMouseDown}
      >
        {/* Collapse buttons */}
        <div className={`absolute ${isHorizontal ? "flex-col top-1" : "flex-row left-1"} flex gap-0.5 opacity-0 group-hover:opacity-100 transition-opacity z-10`}>
          {canCollapseFirst && (
            <button
              type="button"
              onClick={() => toggleCollapse("first")}
              className="w-4 h-4 bg-bg-secondary border border-border rounded text-[8px] text-text-muted hover:text-text-primary flex items-center justify-center"
              title={collapsed === "first" ? "Expand" : "Collapse"}
            >
              {isHorizontal ? (collapsed === "first" ? ">" : "<") : (collapsed === "first" ? "v" : "^")}
            </button>
          )}
          {canCollapseSecond && (
            <button
              type="button"
              onClick={() => toggleCollapse("second")}
              className="w-4 h-4 bg-bg-secondary border border-border rounded text-[8px] text-text-muted hover:text-text-primary flex items-center justify-center"
              title={collapsed === "second" ? "Expand" : "Collapse"}
            >
              {isHorizontal ? (collapsed === "second" ? "<" : ">") : (collapsed === "second" ? "^" : "v")}
            </button>
          )}
        </div>
      </div>

      {/* Second pane */}
      <div className={`flex-1 overflow-auto ${collapsed === "second" ? "hidden" : ""}`}>
        {children[1]}
      </div>
    </div>
  );
}

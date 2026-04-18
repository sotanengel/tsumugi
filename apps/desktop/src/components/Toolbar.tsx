import * as Tooltip from "@radix-ui/react-tooltip";
import { open } from "@tauri-apps/plugin-dialog";
import { useTimelineStore } from "../store/timeline-store";

function ToolbarButton({ label, onClick }: { label: string; onClick: () => void }) {
  return (
    <Tooltip.Root>
      <Tooltip.Trigger asChild>
        <button type="button" onClick={onClick}>
          {label}
        </button>
      </Tooltip.Trigger>
      <Tooltip.Portal>
        <Tooltip.Content
          className="rounded bg-bg-track px-2 py-1 text-xs text-text-primary shadow-lg"
          sideOffset={5}
        >
          {label}
          <Tooltip.Arrow className="fill-bg-track" />
        </Tooltip.Content>
      </Tooltip.Portal>
    </Tooltip.Root>
  );
}

export function Toolbar() {
  const { addTrack, importMedia, createTimeline } = useTimelineStore();

  const handleImport = async () => {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Media",
          extensions: ["mp4", "mov", "webm", "mkv", "mp3", "wav", "aac", "png", "jpg", "jpeg", "heic"],
        },
      ],
    });
    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected];
      for (const path of paths) {
        await importMedia(path);
      }
    }
  };

  return (
    <div className="flex gap-2 px-3 py-2 border-b border-border bg-bg-secondary">
      <ToolbarButton label="New Project" onClick={() => createTimeline(30)} />
      <ToolbarButton label="Import Media" onClick={handleImport} />
      <ToolbarButton label="+ Video Track" onClick={() => addTrack("Video")} />
      <ToolbarButton label="+ Audio Track" onClick={() => addTrack("Audio")} />
    </div>
  );
}

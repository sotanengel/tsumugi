import { open } from "@tauri-apps/plugin-dialog";
import { useTimelineStore } from "../store/timeline-store";

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
    <div style={{
      display: "flex",
      gap: 8,
      padding: "8px 12px",
      borderBottom: "1px solid #333",
      background: "#1a1a2e",
    }}>
      <button type="button" onClick={() => createTimeline(30)}>New Project</button>
      <button type="button" onClick={handleImport}>Import Media</button>
      <button type="button" onClick={() => addTrack("Video")}>+ Video Track</button>
      <button type="button" onClick={() => addTrack("Audio")}>+ Audio Track</button>
    </div>
  );
}

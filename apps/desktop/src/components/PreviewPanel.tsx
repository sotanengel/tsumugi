import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useTimelineStore } from "../store/timeline-store";

export function PreviewPanel() {
  const { timeline, previewSource, playbackRequested } = useTimelineStore();
  const videoRef = useRef<HTMLVideoElement>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [videoError, setVideoError] = useState<string | null>(null);
  const [dataUrl, setDataUrl] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    setIsPlaying(false);
    setCurrentTime(0);
    setDuration(0);
    setVideoError(null);
    setDataUrl(null);

    if (!previewSource) return;

    setLoading(true);
    invoke<string>("read_file_as_data_url", { path: previewSource })
      .then((url) => {
        setDataUrl(url);
        setLoading(false);
      })
      .catch((err) => {
        setVideoError(String(err));
        setLoading(false);
      });
  }, [previewSource]);

  // Respond to keyboard shortcut playback toggle
  useEffect(() => {
    if (!playbackRequested || !videoRef.current) return;
    if (playbackRequested === "play") {
      videoRef.current.play();
      setIsPlaying(true);
    } else {
      videoRef.current.pause();
      setIsPlaying(false);
    }
    useTimelineStore.setState({ playbackRequested: null });
  }, [playbackRequested]);

  const togglePlay = () => {
    const video = videoRef.current;
    if (!video) return;
    if (video.paused) {
      video.play();
      setIsPlaying(true);
    } else {
      video.pause();
      setIsPlaying(false);
    }
  };

  const handleTimeUpdate = () => {
    if (videoRef.current) {
      setCurrentTime(videoRef.current.currentTime);
    }
  };

  const handleScrub = (e: React.ChangeEvent<HTMLInputElement>) => {
    const time = Number(e.target.value);
    if (videoRef.current) {
      videoRef.current.currentTime = time;
      setCurrentTime(time);
    }
  };

  const formatTime = (secs: number) => {
    const m = Math.floor(secs / 60);
    const s = Math.floor(secs % 60);
    return `${m}:${s.toString().padStart(2, "0")}`;
  };

  return (
    <div className="flex-1 flex flex-col items-center justify-center bg-black min-h-[300px]">
      {loading && (
        <span className="text-text-muted text-sm">Loading preview...</span>
      )}
      {videoError && (
        <span className="text-error-text text-xs px-4">{videoError}</span>
      )}
      {dataUrl ? (
        <>
          <video
            ref={videoRef}
            src={dataUrl}
            className="max-w-full max-h-[calc(100%-48px)] object-contain"
            onTimeUpdate={handleTimeUpdate}
            onLoadedMetadata={() => {
              if (videoRef.current) setDuration(videoRef.current.duration);
            }}
            onEnded={() => setIsPlaying(false)}
            onError={() => {
              setVideoError("Failed to play video. Format may not be supported.");
            }}
          />
          <div className="flex items-center gap-2 w-full px-4 py-2 bg-bg-secondary">
            <button type="button" onClick={togglePlay} className="text-sm w-8">
              {isPlaying ? "||" : "\u25B6"}
            </button>
            <input
              type="range"
              min={0}
              max={duration || 0}
              step={0.01}
              value={currentTime}
              onChange={handleScrub}
              className="flex-1 h-1 accent-blue-500"
            />
            <span className="text-xs text-text-muted w-20 text-right">
              {formatTime(currentTime)} / {formatTime(duration)}
            </span>
          </div>
        </>
      ) : !loading && !videoError ? (
        <div className="w-[480px] h-[270px] bg-bg-primary flex items-center justify-center rounded border border-border">
          <span className="text-text-dim text-sm">
            {timeline ? `Preview (16:9) — ${timeline.fps}fps` : "No project loaded"}
          </span>
        </div>
      ) : null}
    </div>
  );
}

# Architecture Overview

## Layer Diagram

```
┌───────────────────────────────────────┐
│  Frontend (React / TypeScript)        │
│  ├─ Timeline UI                       │
│  ├─ Preview (aspect overlay)          │
│  ├─ Media Library                     │
│  └─ Property Panels                   │
└─────────────┬─────────────────────────┘
              │ Tauri IPC (JSON)
┌─────────────┴─────────────────────────┐
│  Core (Rust Workspace)                │
│  ├─ timeline-engine   [pure logic]    │
│  ├─ compositor        [wgpu]          │
│  ├─ media-io          [FFmpeg]        │
│  ├─ project-store     [SQLite]        │
│  ├─ transcribe        [whisper.cpp]   │
│  └─ tsumugi-core      [integration]   │
└─────────────┬─────────────────────────┘
              │ IPC / HTTP (opt-in)
┌─────────────┴─────────────────────────┐
│  MCP Server (Node.js / TypeScript)    │
│  └─ Tool Registry → timeline-engine   │
└───────────────────────────────────────┘
```

## Crate Responsibilities

| Crate | Role | I/O |
|-------|------|-----|
| `timeline-engine` | Pure timeline logic: tracks, clips, operations | None |
| `compositor` | GPU rendering via wgpu | GPU |
| `media-io` | FFmpeg wrapper for probing and encoding | Filesystem, FFmpeg |
| `project-store` | SQLite-based project persistence | Filesystem |
| `transcribe` | Speech-to-text via whisper.cpp | Filesystem, CPU/GPU |
| `tsumugi-core` | Integration crate, re-exports all above | None |

## Design Principles

1. **Crate isolation**: communicate via traits, one-way dependency flow
2. **Pure core**: `timeline-engine` has zero I/O, fully testable
3. **Side effects at boundaries**: I/O only in `media-io`, `project-store`, `compositor`
4. **Local-first**: no network required, all processing on-device
5. **Non-destructive editing**: source files are never modified

## Data Flow

1. User imports media → `media-io::probe_file()` extracts metadata
2. User edits timeline → `timeline-engine` operations (pure functions)
3. Preview requested → `compositor` renders frame via wgpu
4. Export requested → `media-io` encodes via FFmpeg
5. Save project → `project-store` persists to SQLite

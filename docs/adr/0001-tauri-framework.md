# ADR 0001: Use Tauri 2 as Application Framework

## Status

Accepted

## Context

Tsumugi needs a cross-platform desktop framework that supports Rust for
performance-critical video processing and a web frontend for rapid UI development.

## Decision

Use Tauri 2.x with React + TypeScript frontend.

## Rationale

- **Rust backend**: native performance for video processing, GPU access via wgpu
- **Small binary**: Tauri uses the OS webview, resulting in ~10MB binaries vs ~200MB for Electron
- **Security**: Tauri's capability-based permissions limit what the frontend can access
- **Cross-platform**: macOS, Windows, Linux from single codebase
- **Active ecosystem**: Tauri 2 is stable with growing community

## Alternatives Considered

- **Electron**: Too large (~200MB), Node.js not ideal for video processing
- **Flutter**: Good for mobile, weaker desktop ecosystem, no Rust integration
- **Native (SwiftUI/WinUI)**: Per-platform effort too high for 2-person team
- **egui/iced**: Pure Rust UI, but less mature for complex UIs

## Consequences

- Frontend devs need to learn Tauri IPC patterns
- Platform-specific APIs accessed through Tauri plugins
- Web technologies for UI (tested with Vitest + Playwright)

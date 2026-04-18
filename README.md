# Tsumugi (紡)

> **紡ぐ** — Weave your story. OSS video editing for everyone.

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![CI](https://github.com/sotanengel/tsumugi/actions/workflows/ci.yml/badge.svg)](https://github.com/sotanengel/tsumugi/actions/workflows/ci.yml)
[![WIP](https://img.shields.io/badge/status-Phase_0-yellow.svg)]()

**Tsumugi** is a free, open-source, privacy-first desktop video editor built
with [Tauri 2](https://tauri.app/), Rust, and React. From TikTok shorts to
wedding movies — edit locally, no cloud required.

## The 4 Promises

1. **Your videos are yours** — no cloud upload required
2. **Free forever** — Apache 2.0, guaranteed
3. **AI is the sidekick, you're the director** — automation assists, never takes over
4. **Grows with you** — beginner to advanced, same tool

## Architecture

```
Frontend (React/TS) → Tauri IPC → Rust Core → Native libs (FFmpeg, wgpu)
                                             → MCP Server (AI integration)
```

### Rust Crates

| Crate | Purpose |
|-------|---------|
| `timeline-engine` | Pure timeline logic (no I/O) |
| `compositor` | GPU rendering (wgpu) |
| `media-io` | FFmpeg probe & encode |
| `project-store` | SQLite persistence |
| `transcribe` | Speech-to-text (Phase 2) |
| `tsumugi-core` | Integration re-exports |

## Getting Started

### Prerequisites

- Rust (stable) — see `rust-toolchain.toml`
- Node.js 22+ with pnpm 10+
- FFmpeg
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

### Setup

```bash
git clone https://github.com/sotanengel/tsumugi.git
cd tsumugi
make setup    # pnpm install + cargo check
make dev      # start Tauri dev server
```

### Commands

```bash
make lint     # run all linters
make fmt      # format all code
make test     # run all tests
make build    # production build
```

## Project Structure

```
tsumugi/
├── apps/
│   ├── desktop/           # Tauri app (React + src-tauri)
│   └── mcp-server/        # MCP server for AI integration
├── crates/                # Rust workspace
│   ├── timeline-engine/   # Pure timeline logic
│   ├── compositor/        # wgpu rendering
│   ├── media-io/          # FFmpeg wrapper
│   ├── project-store/     # SQLite persistence
│   ├── transcribe/        # whisper.cpp (Phase 2)
│   └── tsumugi-core/      # Integration crate
├── docs/                  # Architecture & ADRs
├── AGENTS.md              # AI agent working rules
├── CLAUDE.md              # Claude Code instructions
└── CONTRIBUTING.md        # How to contribute
```

## Status

**Phase 0** — Foundation (in progress)

- [x] Repository & tooling setup
- [x] CI/CD pipeline
- [x] Rust workspace with all crates
- [x] Tauri 2 app shell
- [x] timeline-engine with tests (12 passing)
- [x] project-store with SQLite schema
- [x] media-io FFmpeg probe PoC
- [x] MCP server stub
- [ ] 1080p/30fps preview (撤退ライン)
- [ ] Full Phase 0 completion → OSS public release

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions and guidelines.

## License

[Apache License 2.0](LICENSE)

---

*紡 (tsumugi) — from "to spin thread," meaning "to weave a story."*
*Videos are threads; Tsumugi helps you weave them into stories.*

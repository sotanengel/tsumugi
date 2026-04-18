# Tsumugi — Agent Instructions (Single Source of Truth)

This document is the canonical reference for all AI agent configuration files.
Tool-specific files (CLAUDE.md, GEMINI.md, etc.) are thin adapters that point here.

## Project

**Tsumugi (紡)** — OSS desktop video editor built with Tauri 2, Rust, and React.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Framework | Tauri 2.x |
| Frontend | React + TypeScript + Vite |
| UI | Radix UI + Tailwind CSS |
| State | Zustand |
| Core | Rust (multi-crate workspace) |
| Video I/O | FFmpeg |
| GPU compositing | wgpu |
| Project storage | SQLite (rusqlite) |
| MCP Server | @modelcontextprotocol/sdk (TS) |
| Linting | clippy, Biome, markdownlint, yamllint |
| Testing | cargo test, Vitest, Playwright |

## Architecture

```
Frontend (React/TS) → Tauri IPC → Rust Core (crates/*) → Native libs (FFmpeg, wgpu)
                                                       → MCP Server (Node.js)
```

### Rust Crates

- `timeline-engine` — pure logic, no I/O
- `compositor` — wgpu rendering
- `media-io` — FFmpeg wrapper
- `project-store` — SQLite persistence
- `transcribe` — whisper.cpp wrapper (Phase 2)
- `tsumugi-core` — integration crate

## Working Rules

1. **Small PRs**: 300 lines or less, one concern per PR
2. **Conventional Commits**: `feat:`, `fix:`, `chore:`, `ci:`, `docs:`, `build:`, `test:`
3. **Branch per feature**: `feat/xxx`, `fix/xxx`, `chore/xxx` → PR → squash merge
4. **No direct push to main**
5. **Tests required**: no feature PR without tests
6. **Rust stable only**: no nightly features
7. **Dependencies**: check `rust-toolchain.toml` and `.mise.toml` for pinned versions
8. **GitHub Actions**: all `uses:` must be SHA-pinned
9. **Secrets**: never commit `.env`, credentials, or API keys

## Coding Conventions

### Rust

- `rustfmt` + `clippy --deny warnings`
- Modules communicate via traits
- Dependency direction: upper → lower only
- Side effects isolated in thin wrappers; core logic is pure

### TypeScript

- Biome for formatting and linting (not ESLint/Prettier)
- Strict mode enabled
- Prefer named exports

## Testing

| Layer | Tool | Coverage target |
|-------|------|----------------|
| Rust unit | `cargo test` | 80%+ |
| Rust integration | `cargo test --test` | major flows |
| TS unit | Vitest | 70%+ |
| TS component | Vitest + Testing Library | 60%+ |
| E2E | Playwright | key scenarios |

## Commands

```bash
make setup    # Install dependencies
make lint     # Run all linters
make fmt      # Format all code
make test     # Run all tests
make dev      # Start Tauri dev server
make build    # Production build
```

## Definition of Done

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Linters pass (`make lint`)
- [ ] PR follows conventional commit format
- [ ] Documentation updated if public API changed

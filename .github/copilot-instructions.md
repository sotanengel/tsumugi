# Tsumugi — GitHub Copilot Instructions

## Project Overview

Tsumugi (紡) is an OSS desktop video editor built with Tauri 2, Rust, and React.
It targets creators aged 18–35 who want free, private, local-first video editing.

## Tech Stack

- **Backend**: Rust multi-crate workspace (timeline-engine, compositor, media-io,
  project-store, transcribe, tsumugi-core)
- **Frontend**: React + TypeScript + Vite, Radix UI + Tailwind CSS, Zustand
- **Framework**: Tauri 2.x (Rust ↔ JS via IPC)
- **Testing**: cargo test, Vitest, Playwright
- **Linting**: clippy + rustfmt (Rust), Biome (TS/JS), markdownlint, yamllint

## Coding Conventions

### Rust

- Edition 2021, stable toolchain only
- `clippy --deny warnings`, `rustfmt` formatting
- Crates communicate via traits, not concrete types
- Pure functions in core; side effects in thin wrappers
- Error types use `thiserror`

### TypeScript

- Strict mode, Biome for formatting/linting
- Named exports preferred
- React functional components only

## PR Rules

- 300 lines max per PR
- Conventional Commits: `feat:`, `fix:`, `chore:`, `ci:`, `docs:`
- Tests required for feature PRs
- GitHub Actions must use SHA-pinned references

## Project Structure

```
crates/          — Rust crates (timeline-engine, compositor, media-io, etc.)
apps/desktop/    — Tauri app (React frontend + src-tauri Rust backend)
apps/mcp-server/ — MCP server for AI agent integration
packages/        — Shared TypeScript packages
docs/            — Architecture docs and ADRs
```

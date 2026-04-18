# Tsumugi — Agent Working Rules

> Read `docs/agent-instructions.md` for the full reference (tech stack,
> architecture, coding conventions, testing targets).

## Mission

Tsumugi is an OSS desktop video editor. Your goal is to help build it while
maintaining code quality, test coverage, and small PR discipline.

## Quick Rules

1. **Read first, write second** — understand existing code before modifying.
2. **Small PRs** — 300 lines max, one concern per PR.
3. **Conventional Commits** — `feat:`, `fix:`, `chore:`, `ci:`, `docs:`, etc.
4. **Tests are mandatory** — no feature PR without accompanying tests.
5. **Rust stable only** — no nightly features or unstable APIs.
6. **SHA-pin GitHub Actions** — never use tag-only references.
7. **No secrets in code** — `.env` is gitignored; use `.env.example` for templates.
8. **Verify before done** — `make lint` and `make test` must pass.

## Crate Isolation

Rust crates communicate through traits. Dependencies flow one direction only
(upper → lower). Core logic must be pure functions; side effects live in thin
wrappers.

## File Layout

```
crates/          — Rust workspace members
apps/desktop/    — Tauri app (frontend + src-tauri)
apps/mcp-server/ — MCP server (TypeScript)
packages/        — Shared TS packages
docs/            — Documentation
scripts/         — Build and utility scripts
tests/           — E2E tests and fixtures
```

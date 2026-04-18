# Tsumugi — Gemini CLI Instructions

Read `AGENTS.md` first, then `docs/agent-instructions.md` for full context.

## Gemini-specific notes

- Always verify changes with `make lint` and `make test` before completing.
- This is a Tauri 2 + Rust + React project — check both frontend and backend.
- Rust crates are in `crates/`, Tauri backend in `apps/desktop/src-tauri/`.
- Use Conventional Commits for all commit messages.
- Do not use nightly Rust features — `rust-toolchain.toml` enforces stable.

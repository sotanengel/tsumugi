# Tsumugi — Claude Code Instructions

Read `AGENTS.md` first, then `docs/agent-instructions.md` for full context.

## Claude-specific notes

- Use `make lint` and `make test` to verify changes before marking done.
- When creating PRs, follow the template in `.github/PULL_REQUEST_TEMPLATE.md`.
- Prefer `cargo check --workspace` for quick compile verification.
- Use `cargo clippy --workspace -- -D warnings` for lint checks.
- Biome (not ESLint) handles TS/JS linting: `npx biome check .`
- Keep PR descriptions concise — bullet points preferred.

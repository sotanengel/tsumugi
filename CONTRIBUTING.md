# Contributing to Tsumugi

## Getting Started

```bash
# Clone
git clone https://github.com/sotanengel/tsumugi.git
cd tsumugi

# Install dependencies
make setup    # runs pnpm install + cargo check

# Run development server
make dev      # starts Tauri dev server
```

## Prerequisites

- Rust (stable, see `rust-toolchain.toml`)
- Node.js 22+ with pnpm 10+
- FFmpeg (for media-io features)
- System dependencies for Tauri: see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

## Development Workflow

1. Create a feature branch: `git checkout -b feat/my-feature`
2. Make changes (keep PRs under 300 lines)
3. Run checks: `make lint && make test`
4. Commit using [Conventional Commits](https://www.conventionalcommits.org/):
   `feat:`, `fix:`, `chore:`, `ci:`, `docs:`, `build:`, `test:`
5. Push and open a PR against `main`

## PR Guidelines

- One concern per PR
- 300 lines max (excluding lockfiles)
- Tests required for feature PRs
- Fill out the PR template checklist
- All CI checks must pass before merge

## Code Style

- **Rust**: `cargo fmt` + `cargo clippy --deny warnings`
- **TypeScript**: Biome (`pnpm exec biome check .`)
- **Markdown**: markdownlint
- **Commits**: Conventional Commits

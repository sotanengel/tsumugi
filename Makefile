.PHONY: setup lint fmt test test-rust test-frontend pre-commit dev build clean

## Setup — install dependencies
setup:
	pnpm install
	cargo check --workspace
	@echo "✓ Setup complete"

## Lint — run all linters
lint:
	cargo fmt --all -- --check
	cargo clippy --workspace -- -D warnings
	pnpm exec biome check .
	@echo "✓ All lints passed"

## Format — auto-format all code
fmt:
	cargo fmt --all
	pnpm exec biome check --write .

## Test — run all tests
test: test-rust test-frontend

## Test Rust
test-rust:
	cargo test --workspace

## Test Frontend
test-frontend:
	pnpm test

## Pre-commit — run pre-commit hooks
pre-commit:
	pre-commit run --all-files

## Install pre-commit hook
install-pre-commit-hook:
	pre-commit install

## Dev — start Tauri dev server
dev:
	pnpm tauri dev

## Build — production build
build:
	pnpm tauri build

## Clean — remove build artifacts
clean:
	cargo clean
	rm -rf node_modules dist

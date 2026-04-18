# Security Policy

## Supported Scope

- Application runtime (Tauri, wgpu, FFmpeg integration)
- Project file handling (`.tsumugi` SQLite files)
- MCP server (when running in HTTP mode)
- Dependency supply chain

## Reporting a Vulnerability

Please report security vulnerabilities privately via
[GitHub Security Advisories](https://github.com/sotanengel/tsumugi/security/advisories).

Do **not** open a public issue for security vulnerabilities.

## Hardening Summary

- Project files are treated as untrusted input (sanitized before use)
- FFmpeg inputs are validated before processing
- GitHub Actions use SHA-pinned references
- Dependencies follow a wait period before adoption (see `renovate.json`)
- No telemetry or network communication by default
- MCP server HTTP mode requires token authentication

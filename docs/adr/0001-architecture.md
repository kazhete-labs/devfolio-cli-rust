# ADR 0001: Rust modular CLI (parity with Go)

## Status

Accepted — 2026-08-02

## Context

Port of `devfolio-cli` for language comparison (binary size, runtime, DX) while keeping identical UX and scoring rules.

## Decision

- Single binary via `clap`
- Blocking `reqwest` + `rustls` (no OpenSSL)
- Modules: `github`, `score`, `emit`, `generate`
- Scoring regexes match the Go implementation

## Consequences

- Longer cold compile than Go
- Potentially smaller/faster release binary after optimization
- Same MVP surface: `devfolio generate <user>`

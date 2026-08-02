# devfolio-cli-rust

[![CI](https://github.com/kazhete-labs/devfolio-cli-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/kazhete-labs/devfolio-cli-rust/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/Rust-stable%20%2F%202021-DEA584?logo=rust&logoColor=white)](Cargo.toml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Go sibling](https://img.shields.io/badge/also%20available%20in-Go-00ADD8?logo=go&logoColor=white)](../devfolio-cli)

Rust port of [devfolio-cli](../devfolio-cli), feature-parity: turn any GitHub username into a **static portfolio site** and a **README quality scorecard** — one command, no SaaS.

```bash
devfolio generate octocat -o ./devfolio-out
```

## Why the Rust build

- ⚡ **Faster scoring** — ~8.5× quicker than the Go version on CPU-bound README grading (see [COMPARISON.md](../devfolio-cli/COMPARISON.md))
- 📦 **Smaller binary** — ~5.6 MB release build, statically linked TLS (rustls)
- 📊 **Same scorecard rubric** — identical weights and grades as the Go CLI
- 🖥️ **Static output** — plain HTML/CSS, deploy anywhere
- 🦀 **Single binary**, no runtime dependencies

## Install

```bash
cargo install --path .
```

Or build a release binary:

```bash
cargo build --release
./target/release/devfolio generate YOUR_USER -o ./devfolio-out
```

## 📖 User Guide

**New here? Start with the [full User Guide](docs/USER_GUIDE.md)** — install options, every flag, the scoring rubric, and troubleshooting for common errors.

## Architecture

Same hexagonal-lite layout as the Go version:

`CLI (clap) → generate → github adapter + score domain → emit HTML/MD`

ADR: [`docs/adr/0001-architecture.md`](docs/adr/0001-architecture.md)

## Development

```bash
cargo test
cargo build --release
```

## See also

- [User Guide](docs/USER_GUIDE.md) — the full walkthrough
- [Go vs Rust comparison](../devfolio-cli/COMPARISON.md) — benchmarks against the [Go sibling](../devfolio-cli)

## License

MIT — see [LICENSE](LICENSE).

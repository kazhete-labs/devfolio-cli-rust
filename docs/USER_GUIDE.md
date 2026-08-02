# User Guide — devfolio-cli-rust

Generate a static portfolio site and README quality scorecard from any public GitHub username. Feature-parity with the Go `devfolio-cli`.

## Requirements

- Rust stable (edition 2021) to build from source, **or** a release `devfolio` binary
- Network access to `api.github.com`
- Optional: `GITHUB_TOKEN` for higher API rate limits

Without a token, unauthenticated limits apply. Multi-repo README fetches can return **403** — use a token for demos.

## Install

```bash
cargo install --path .
# binary: ~./target/release/devfolio (after cargo build --release)
```

```bash
cargo build --release
./target/release/devfolio generate YOUR_USER -o ./devfolio-out
```

## Quick start

```bash
export GITHUB_TOKEN=ghp_xxxxxxxx   # PowerShell: $env:GITHUB_TOKEN="..."

devfolio generate YOUR_GITHUB_USER -o ./devfolio-out
```

| Output | Purpose |
|--------|---------|
| `devfolio-out/index.html` | Portfolio |
| `devfolio-out/scorecard.html` | HTML scorecard |
| `devfolio-out/scorecard.md` | Markdown scorecard |
| `devfolio-out/styles.css` | Styles |

## Commands

### `devfolio generate <user>`

| Flag | Default | Description |
|------|---------|-------------|
| `-o`, `--out` | `devfolio-out` | Output directory |
| `--max-repos` | `12` | Max non-fork repos |
| `--token` | env `GITHUB_TOKEN` | GitHub token |
| `--timeout` | `120` | Timeout (seconds) |

```bash
devfolio generate octocat -o ./out --max-repos 5
devfolio --version
```

## Scoring rules

Identical weights to the Go CLI (install 20, demo 15, badges/license/architecture 10 each, etc.). See the Go [USER_GUIDE](../../devfolio-cli/docs/USER_GUIDE.md) table for the full checklist.

## Troubleshooting

| Symptom | Fix |
|---------|-----|
| `403` on README | Export `GITHUB_TOKEN` |
| Slow first `cargo` | Normal cold compile; later builds are cached |
| Missing outputs | Ensure `generate` finished without error; check path `-o` |

## Develop / test

```bash
cargo test
cargo build --release
cargo run --release -- generate octocat -o ./devfolio-out --max-repos 5
```

See also: [README](../README.md), [ADR](adr/0001-architecture.md), sibling comparison in `../devfolio-cli/COMPARISON.md`.

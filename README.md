# devfolio-cli-rust

Rust port of [devfolio-cli](../devfolio-cli): turn a GitHub username into a **static portfolio** + **README scorecard**.

## User guide

Full install, flags, scoring, and troubleshooting: **[docs/USER_GUIDE.md](docs/USER_GUIDE.md)**.

## Install

```bash
cargo install --path .
```

## Demo

```bash
devfolio generate octocat -o ./devfolio-out
```

## Architecture

Same hexagonal-lite layout as the Go version:

`CLI (clap) → generate → github adapter + score domain → emit HTML/MD`

## Development

```bash
cargo test
cargo build --release
```

## License

MIT — see [LICENSE](LICENSE).

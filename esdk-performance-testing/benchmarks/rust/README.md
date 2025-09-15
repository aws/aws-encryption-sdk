# ESDK Rust Performance Benchmark

Rust implementation of the AWS Encryption SDK performance benchmark suite.

## Features

- **Throughput**: Operations per second and latency analysis (P50, P95, P99)
- **Memory**: Peak memory consumption monitoring
- **Concurrency**: Multi-threaded performance testing
- **Raw AES Keyring**: Local 256-bit AES keys (no KMS dependency)

## Prerequisites

- Rust 1.86.0+ ([rustup.rs](https://rustup.rs/))
- Build tools (Xcode CLI Tools on macOS, build-essential on Linux)

## Quick Start

```bash
# If necessary, build the ESDK and return here
cd ../../../AwsEncryptionSDK/
make polymorph_rust transpile_rust
cd ../esdk-performance-testing/benchmarks/rust/

# Build and run
cargo run --release -- --config ../../config/test-scenarios.yaml

# Quick test (requires quick_config in YAML)
cargo run --release -- --quick
```

## Configuration

### Command Line

- `--config`: Path to YAML config file (required)
- `--output`: Results output path (default: `../../results/raw-data/rust_results.json`)
- `--quick`: Quick test mode (requires `quick_config` section in YAML)

## Logging

Default: info level. Override with `RUST_LOG`:

```bash
RUST_LOG=debug ./target/release/esdk_benchmark --config config.yaml
```

## Development

```bash
# Format and lint
cargo fmt
cargo clippy -- -D warnings

# Test
cargo test

# Debug build
cargo build
RUST_LOG=debug ./target/debug/esdk_benchmark --config config.yaml
```

## Troubleshooting

- **Build issues**: `rustup update && cargo clean && cargo build --release`
- **Config issues**: Validate YAML syntax and check file permissions
- **Memory issues**: Monitor with `htop` or Activity Monitor

## License

Apache License 2.0

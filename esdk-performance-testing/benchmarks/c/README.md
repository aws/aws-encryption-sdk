# ESDK C Performance Benchmark

Performance benchmark suite for the AWS Encryption SDK for C (`aws-encryption-sdk-c`).

## Prerequisites

- CMake 3.10+
- C11 compiler (gcc or clang)
- [aws-encryption-sdk-c](https://github.com/aws/aws-encryption-sdk-c) installed
- [aws-c-common](https://github.com/awslabs/aws-c-common) installed
- [libyaml](https://github.com/yaml/libyaml) (`yaml-0.1`)
- [json-c](https://github.com/json-c/json-c)
- pthreads (included on Linux/macOS)

### macOS (Homebrew)

```bash
brew install libyaml json-c
```

### Linux (apt)

```bash
sudo apt-get install libyaml-dev libjson-c-dev
```

## Build

```bash
# Build everything (deps + benchmark)
./build.sh

# Clean all build artifacts
./build.sh --clean
```

## Run

```bash
# Full benchmark suite
./esdk_benchmark

# Quick test (reduced iterations and data sizes)
./esdk_benchmark --quick

# Custom config and output paths
./esdk_benchmark --config /path/to/config.yaml --output /path/to/results.json
```

## CLI Options

| Flag           | Description                            | Default                                 |
| -------------- | -------------------------------------- | --------------------------------------- |
| `-c, --config` | Path to test configuration YAML        | `../../config/test-scenarios.yaml`      |
| `-o, --output` | Path to output results JSON            | `../../results/raw-data/c_results.json` |
| `-q, --quick`  | Run quick test with reduced iterations | off                                     |
| `-h, --help`   | Show help message                      |                                         |

## Test Types

- **Throughput** — Measures encrypt/decrypt ops per second and latency percentiles (p50, p95, p99)
- **Memory** — Measures peak heap allocation via a tracking allocator wrapper and RSS via continuous background sampling
- **Concurrency** — Measures throughput under parallel load using pthreads

## Output

Results are written as JSON to the configured output path with the same schema used by all other language benchmarks in this repo.

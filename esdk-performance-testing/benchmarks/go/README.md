# ESDK Go Benchmark

Performance benchmark suite for the AWS Encryption SDK (ESDK) Go implementation.

## Quick Start

```bash
# Run quick benchmark
go run . --config ../../config/test-scenarios.yaml --quick

# Run full benchmark
go run . --config ../../config/test-scenarios.yaml
```

## Build

```bash
# Build release binary
go build -o esdk-benchmark .

# Run built binary
./esdk-benchmark --quick
```

## Configuration

The benchmark uses YAML configuration files. See `../../config/test-scenarios.yaml` for the full configuration format.

### Quick Mode

Quick mode runs a subset of tests with reduced iterations:

- Only runs test types specified in `quick_config.test_types`
- Uses smaller data sizes from `quick_config.data_sizes.small`
- Fewer iterations: `quick_config.iterations.measurement`

## Test Types

- **throughput**: Measures operations per second and latency
- **memory**: Measures peak memory usage during operations
- **concurrency**: Tests performance under concurrent load

## Output

Results are saved to JSON format in `../../results/raw-data/go_results.json` by default.

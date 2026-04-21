# AWS Encryption SDK JavaScript Benchmark

Performance testing suite for the AWS Encryption SDK JavaScript/Node.js implementation.

## Quick Start

```bash
# Install dependencies
npm install

# Run benchmark
node src/main.js

# Quick test (reduced iterations)
node src/main.js --quick
```

## Options

- `--config <path>` - Path to test configuration file (default: `../../config/test-scenarios.yaml`)
- `--output <path>` - Path to output results file (default: `../../results/raw-data/node_results.json`)
- `--quick` - Run with reduced iterations for faster testing

## Configuration

Edit `../../config/test-scenarios.yaml` for test parameters:

- Data sizes (small/medium/large)
- Iterations and concurrency levels

## Test Types

- **Throughput** - Measures encryption/decryption operations per second with latency percentiles
- **Memory** - Tracks heap memory usage during operations via `process.memoryUsage()`
- **Concurrency** - Tests performance under concurrent load using async `Promise.all` (single-threaded; not true OS-level parallelism like Go goroutines or Rust tokio tasks)

## Output

Results saved as JSON to `../../results/raw-data/node_results.json` with:

- Performance metrics (ops/sec, latency percentiles)
- Memory usage (peak heap, efficiency ratio)
- System information (CPU, memory, Node.js version)

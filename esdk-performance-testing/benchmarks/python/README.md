# AWS Encryption SDK Python Benchmark

Performance testing suite for the AWS Encryption SDK Python implementation.

## Quick Start

```bash
# Install dependencies
pip install -r requirements.txt

# Run benchmark
python esdk_benchmark.py

# Quick test (reduced iterations)
python esdk_benchmark.py --quick
```

## Options

- `--config` - Path to test configuration file (default: `../../config/test-scenarios.yaml`)
- `--output` - Path to output results file (default: `../../results/raw-data/python_results.json`)
- `--quick` - Run with reduced iterations for faster testing

## Configuration

Edit `../../config/test-scenarios.yaml` for test parameters:

- Data sizes (small/medium/large)
- Iterations and concurrency levels

## Test Types

- **Throughput** - Measures encryption/decryption operations per second
- **Memory** - Tracks memory usage and allocations during operations
- **Concurrency** - Tests performance under concurrent load

## Output

Results saved as JSON to `../../results/raw-data/python_results.json` with:

- Performance metrics (ops/sec, latency percentiles)
- Memory usage (peak, average, allocations, input data to memory ratio)
- System information (CPU, memory, Python version)

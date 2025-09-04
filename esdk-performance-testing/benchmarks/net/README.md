# AWS Encryption SDK .NET Benchmark

Performance testing suite for the AWS Encryption SDK .NET implementation.

## Quick Start

```bash
# Build project
dotnet build --configuration Release

# Run benchmark
dotnet run --configuration Release

# Quick test (reduced iterations)
dotnet run --configuration Release -- --quick
```

## Options

- `-c, --config` - Path to test configuration file (default: `../../config/test-scenarios.yaml`)
- `-o, --output` - Path to output results file (default: `../../results/raw-data/net_results.json`)
- `-q, --quick` - Run with reduced iterations for faster testing

## Configuration

Edit `../../config/test-scenarios.yaml` for test parameters:

- Data sizes (small/medium/large)
- Iterations and concurrency levels

## Test Types

- **Throughput** - Measures encryption/decryption operations per second
- **Memory** - Tracks memory usage and allocations during operations
- **Concurrency** - Tests performance under concurrent load

## Output

Results saved as JSON to `../../results/raw-data/net_results.json` with:

- Performance metrics (ops/sec, latency percentiles)
- Memory usage (peak, average, allocations)
- System information (CPU, memory, .NET version)

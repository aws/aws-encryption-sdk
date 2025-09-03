# ESDK .NET Performance Benchmark

This directory contains the .NET implementation of the AWS Encryption SDK (ESDK) performance benchmark suite.

## Overview

The .NET benchmark provides comprehensive performance testing for the ESDK .NET runtime, measuring:

- **Throughput**: Operations per second and data processing rates
- **Latency**: Encrypt/decrypt operation timing with percentile analysis
- **Memory Usage**: Peak memory consumption and efficiency with GC optimization
- **Concurrency**: Multi-threaded performance characteristics using async/await
- **Chunk Processing**: Efficient handling of large files through chunked processing

## Key Features

### Chunk File Reading

The .NET implementation includes advanced chunk file reading capabilities for efficient processing of large files:

- **Configurable Chunk Size**: Default 1MB, customizable via `--chunk-size` parameter
- **Memory Efficient**: Processes large files without loading entire content into memory
- **Streaming Processing**: Handles files larger than available RAM using `ReadOnlySpan<byte>`
- **Progress Tracking**: Real-time progress indication for large file operations

### Async Processing

- Built on .NET's Task-based Asynchronous Pattern (TAP)
- Concurrent task execution with proper async/await patterns
- Non-blocking operations for better resource utilization
- Configurable concurrency levels

### Advanced Metrics

- Statistical analysis using MathNet.Numerics
- Percentile calculations (P50, P95, P99)
- Memory profiling with GC integration
- Comprehensive system information collection

## Prerequisites

- **.NET SDK**: 8.0 or later (install from [dotnet.microsoft.com](https://dotnet.microsoft.com/download))
- **System Dependencies**:
  - On Linux: `libicu` for globalization
  - On macOS: No additional dependencies
  - On Windows: No additional dependencies

## Quick Start

### 1. Build and Run

```bash
# Run full benchmark suite
./run_benchmark.sh

# Run quick test (reduced iterations)
./run_benchmark.sh --quick

# Run with custom chunk size (2MB)
./run_benchmark.sh --chunk-size 2097152

# Run with verbose logging
./run_benchmark.sh --verbose
```

### 2. Manual Build

```bash
# Restore dependencies
dotnet restore

# Build in release mode (recommended for benchmarks)
dotnet build --configuration Release

# Run with custom parameters
dotnet run --configuration Release -- \
    --config ../../config/test-scenarios.yaml \
    --output ../../results/raw-data/net_results.json \
    --chunk-size 1048576 \
    --verbose
```

## Configuration

### Command Line Options

| Option          | Description                                  | Default                                   |
| --------------- | -------------------------------------------- | ----------------------------------------- |
| `-c, --config`  | Path to test configuration file              | `../../config/test-scenarios.yaml`        |
| `-o, --output`  | Path to output results file                  | `../../results/raw-data/net_results.json` |
| `-q, --quick`   | Run quick test with reduced iterations       | `false`                                   |
| `--chunk-size`  | Chunk size for large file processing (bytes) | `1048576` (1MB)                           |
| `-v, --verbose` | Enable verbose logging                       | `false`                                   |

### Build Script Options

| Option              | Description                    | Default                |
| ------------------- | ------------------------------ | ---------------------- |
| `--debug`           | Build in debug mode            | `false` (release mode) |
| `--chunk-size SIZE` | Set chunk size for large files | `1048576`              |
| `-v, --verbose`     | Enable verbose output          | `false`                |

### Configuration File

The benchmark uses the same YAML configuration as other language implementations:

```yaml
data_sizes:
  small: [1024, 5120, 10240]
  medium: [102400, 512000, 1048576]
  large: [10485760, 52428800, 104857600]

iterations:
  warmup: 5
  measurement: 10

concurrency_levels: [1, 2, 4, 8]
algorithm_suites: ["ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256"]
frame_lengths: [4096, 65536, 1048576]
```

## Chunk Processing Details

### How It Works

1. **Size Detection**: Files larger than the configured chunk size are automatically processed in chunks
2. **Sequential Processing**: Each chunk is encrypted/decrypted individually using `ReadOnlySpan<byte>`
3. **Memory Management**: Only one chunk is held in memory at a time with efficient span operations
4. **Progress Tracking**: Real-time progress updates using ShellProgressBar
5. **Result Aggregation**: Timing results are combined for accurate overall metrics

### Chunk Size Selection

Choose chunk size based on your use case:

- **Small chunks (64KB - 256KB)**: Lower memory usage, more frequent operations
- **Medium chunks (1MB - 4MB)**: Balanced performance (recommended)
- **Large chunks (8MB+)**: Higher throughput, more memory usage

### Example Usage

```bash
# Process 1GB files with 4MB chunks
./run_benchmark.sh --chunk-size 4194304

# Memory-constrained environment (256KB chunks)
./run_benchmark.sh --chunk-size 262144

# High-performance setup (16MB chunks)
./run_benchmark.sh --chunk-size 16777216
```

## Performance Optimization

### Build Optimizations

The benchmark automatically builds in release mode with optimizations:

```xml
<PropertyGroup Condition="'$(Configuration)'=='Release'">
  <Optimize>true</Optimize>
  <DebugType>portable</DebugType>
  <DebugSymbols>true</DebugSymbols>
</PropertyGroup>
```

### Runtime Optimizations

- **GC Configuration**: `SustainedLowLatency` mode for consistent performance
- **Span Usage**: `ReadOnlySpan<byte>` for zero-copy operations
- **Async Patterns**: Proper async/await usage for I/O operations
- **Memory Pooling**: Efficient memory allocation patterns

### Environment Variables

```bash
# Enable detailed GC logging
export DOTNET_GCStress=0
export DOTNET_gcServer=1

# Set custom thread pool settings
export DOTNET_ThreadPool_ForceMinWorkerThreads=8
export DOTNET_ThreadPool_ForceMaxWorkerThreads=32
```

## Output Format

Results are saved in JSON format compatible with other language implementations:

```json
{
  "metadata": {
    "language": "net",
    "timestamp": "2024-01-01T00:00:00Z",
    "dotnet_version": "8.0.0",
    "cpu_count": 8,
    "total_memory_gb": 16.0,
    "total_tests": 42,
    "chunk_size": 1048576
  },
  "results": [
    {
      "TestName": "throughput",
      "Language": "net",
      "DataSize": 1048576,
      "AlgorithmSuite": "ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256",
      "FrameLength": 65536,
      "Concurrency": 1,
      "EncryptLatencyMs": 2.5,
      "DecryptLatencyMs": 2.1,
      "EndToEndLatencyMs": 4.8,
      "OpsPerSecond": 208.33,
      "BytesPerSecond": 218453333.33,
      "P50Latency": 4.7,
      "P95Latency": 5.2,
      "P99Latency": 5.8,
      "Timestamp": "2024-01-01T00:00:00Z"
    }
  ]
}
```

## Troubleshooting

### Common Issues

1. **Build Failures**

   ```bash
   # Update .NET SDK
   dotnet --version

   # Clean and rebuild
   dotnet clean && dotnet build --configuration Release
   ```

2. **Memory Issues with Large Files**

   ```bash
   # Reduce chunk size
   ./run_benchmark.sh --chunk-size 262144

   # Monitor memory usage
   ./run_benchmark.sh --verbose
   ```

3. **Performance Issues**

   ```bash
   # Ensure release build
   ./run_benchmark.sh --debug  # Don't use this for benchmarks

   # Check GC settings
   dotnet run --configuration Release -- --verbose
   ```

### Debug Mode

For development and debugging:

```bash
# Build in debug mode
./run_benchmark.sh --debug --verbose

# Enable detailed logging
dotnet run --configuration Debug -- --verbose
```

### Profiling

For performance analysis:

```bash
# Install profiling tools
dotnet tool install --global dotnet-trace
dotnet tool install --global dotnet-counters

# Profile the benchmark
dotnet trace collect --process-id <pid> --providers Microsoft-DotNETCore-SampleProfiler
```

## Integration

### CI/CD Integration

```yaml
# GitHub Actions example
- name: Setup .NET
  uses: actions/setup-dotnet@v3
  with:
    dotnet-version: "8.0.x"

- name: Run .NET Benchmarks
  run: |
    cd aws-encryption-sdk/esdk-performance-testing/benchmarks/net
    ./run_benchmark.sh --quick

- name: Upload Results
  uses: actions/upload-artifact@v3
  with:
    name: net-benchmark-results
    path: aws-encryption-sdk/esdk-performance-testing/results/raw-data/net_results.json
```

### Automated Testing

```bash
# Run unit tests (if any)
dotnet test

# Run benchmarks with validation
dotnet run --configuration Release -- --quick --verbose
```

## Contributing

When contributing to the .NET benchmark:

1. **Code Style**: Follow .NET coding conventions and use EditorConfig
2. **Async Patterns**: Use proper async/await patterns, avoid blocking calls
3. **Memory Management**: Use spans and memory-efficient patterns
4. **Testing**: Add unit tests for new functionality
5. **Documentation**: Update this README for new features
6. **Performance**: Benchmark changes against baseline

### Development Setup

```bash
# Install development tools
dotnet tool install --global dotnet-format
dotnet tool install --global dotnet-outdated-tool

# Format code
dotnet format

# Check for outdated packages
dotnet outdated
```

## Dependencies

### Core Dependencies

- **CommandLineParser**: Command-line argument parsing
- **YamlDotNet**: YAML configuration file support
- **Newtonsoft.Json**: JSON serialization
- **ShellProgressBar**: Progress indication
- **MathNet.Numerics**: Statistical calculations

### Logging and Monitoring

- **Microsoft.Extensions.Logging**: Structured logging
- **System.Diagnostics.PerformanceCounter**: Performance monitoring
- **System.Management**: System information (Windows)

### Development Dependencies

- **.NET 8.0 SDK**: Required for building and running
- **NuGet Package Manager**: Dependency management

## License

This benchmark is part of the AWS Encryption SDK and is licensed under the Apache License 2.0.

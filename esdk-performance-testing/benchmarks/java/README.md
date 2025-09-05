# ESDK Performance Benchmark - Java

This directory contains the Java implementation of the AWS Encryption SDK (ESDK) performance benchmark suite.

## Overview

The Java benchmark provides comprehensive performance testing for the ESDK Java runtime, measuring:

- **Throughput**: Operations per second and bytes per second
- **Latency**: Encrypt, decrypt, and end-to-end timing
- **Memory Usage**: Peak memory consumption and efficiency
- **Concurrency**: Multi-threaded performance scaling
- **Statistical Analysis**: P50, P95, P99 latency percentiles

## Prerequisites

- Java 11 or higher
- Maven 3.6 or higher
- Access to AWS Encryption SDK Java libraries

## Building

```bash
# Build the project
mvn clean compile

# Create executable JAR
mvn clean package

# Run tests
mvn test
```

## Running Benchmarks

### Quick Test

```bash
# Using Maven
mvn exec:java -Dexec.mainClass="com.amazon.esdk.benchmark.ESDKBenchmark" -Dexec.args="--quick"

# Using JAR
java -jar target/esdk-benchmark.jar --quick
```

### Full Benchmark Suite

```bash
# Using Maven
mvn exec:java -Dexec.mainClass="com.amazon.esdk.benchmark.ESDKBenchmark"

# Using JAR
java -jar target/esdk-benchmark.jar
```

### Custom Configuration

```bash
# Specify custom config and output paths
java -jar target/esdk-benchmark.jar \
  --config /path/to/config.yaml \
  --output /path/to/results.json
```

## Command Line Options

- `--config, -c`: Path to test configuration file (default: `../../config/test-scenarios.yaml`)
- `--output, -o`: Path to output results file (default: `../../results/raw-data/java_results.json`)
- `--quick, -q`: Run quick test with reduced iterations
- `--help, -h`: Show help message

## Configuration

The benchmark uses a YAML configuration file to define test parameters:

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

## Output Format

Results are saved in JSON format with the following structure:

```json
{
  "metadata": {
    "language": "java",
    "timestamp": "2025-08-27T15:30:00Z",
    "javaVersion": "11.0.20",
    "cpuCount": 8,
    "totalMemoryGB": 16.0,
    "totalTests": 45
  },
  "results": [
    {
      "testName": "throughput",
      "language": "java",
      "dataSize": 1024,
      "algorithmSuite": "ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256",
      "frameLength": 4096,
      "concurrency": 1,
      "encryptLatencyMs": 0.85,
      "decryptLatencyMs": 0.72,
      "endToEndLatencyMs": 1.57,
      "opsPerSecond": 636.94,
      "bytesPerSecond": 652224.0,
      "peakMemoryMB": 0.0,
      "memoryEfficiencyRatio": 0.0,
      "p50Latency": 1.55,
      "p95Latency": 1.89,
      "p99Latency": 2.12,
      "timestamp": "2025-08-27T15:30:15Z",
      "javaVersion": "11.0.20",
      "cpuCount": 8,
      "totalMemoryGB": 16.0
    }
  ]
}
```

## Performance Tuning

### JVM Options

For optimal performance, consider these JVM options:

```bash
java -Xmx4g -Xms2g \
     -XX:+UseG1GC \
     -XX:MaxGCPauseMillis=200 \
     -XX:+UnlockExperimentalVMOptions \
     -XX:+UseJVMCICompiler \
     -jar target/esdk-benchmark.jar
```

### System Configuration

- Ensure adequate heap memory (recommended: 4GB+)
- Use SSD storage for temporary files
- Disable CPU frequency scaling for consistent results
- Close unnecessary applications during benchmarking

## Troubleshooting

### Common Issues

1. **OutOfMemoryError**: Increase heap size with `-Xmx` option
2. **Dependency conflicts**: Run `mvn dependency:tree` to check for conflicts
3. **Slow performance**: Ensure JVM warmup and disable background processes

### Debug Mode

Enable debug logging:

```bash
java -Dorg.slf4j.simpleLogger.defaultLogLevel=debug \
     -jar target/esdk-benchmark.jar
```

## Integration

### CI/CD Pipeline

Example GitHub Actions workflow:

```yaml
- name: Run Java Benchmarks
  run: |
    cd aws-encryption-sdk/esdk-performance-testing/benchmarks/java
    mvn clean package
    java -jar target/esdk-benchmark.jar --quick
```

### Automated Analysis

The benchmark integrates with the overall performance testing suite. Results can be:

- Compared across language implementations
- Tracked over time for regression detection
- Analyzed for performance optimization opportunities

## Contributing

When modifying the Java benchmark:

1. Follow Java coding standards
2. Add appropriate unit tests
3. Update documentation for new features
4. Ensure compatibility with existing configuration format
5. Test with both quick and full benchmark modes

## Dependencies

Key dependencies used in this benchmark:

- **AWS Encryption SDK**: Core encryption/decryption functionality
- **Jackson**: JSON/YAML processing
- **Commons CLI**: Command line argument parsing
- **ProgressBar**: Visual progress indication
- **SLF4J**: Logging framework
- **JUnit**: Unit testing (test scope)

## License

This benchmark suite is part of the AWS Encryption SDK project and follows the same licensing terms.

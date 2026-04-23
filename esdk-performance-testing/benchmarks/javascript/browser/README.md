# AWS Encryption SDK Browser JavaScript Benchmark

Performance testing suite for the AWS Encryption SDK Browser JavaScript implementation (`@aws-crypto/client-browser`).

## Overview

This benchmark uses the browser variant of the AWS Encryption SDK, which relies on WebCrypto APIs (`RawAesKeyringWebCrypto`). The launcher bundles the benchmark with esbuild, serves it locally, and runs it in headless Chrome via Puppeteer to collect results.

## Quick Start

```bash
# Install dependencies
npm install

# Run benchmark (bundles with esbuild, launches headless Chrome via Puppeteer)
node src/launcher.js

# Quick test (reduced iterations)
node src/launcher.js --quick
```

## Options

- `--config <path>` - Path to test configuration file (default: `../../../config/test-scenarios.yaml`)
- `--output <path>` - Path to output results file (default: `../../../results/raw-data/browser_js_results.json`)
- `--quick` - Run with reduced iterations for faster testing

## Configuration

Edit `../../config/test-scenarios.yaml` for test parameters:

- Data sizes (small/medium/large)
- Iterations and concurrency levels

## Test Types

- **Throughput** - Measures encryption/decryption operations per second with latency percentiles
- **Memory** - Tracks heap memory usage during operations via Chrome's `performance.memory` API
- **Concurrency** - Tests performance under concurrent load using async workers via `Promise.all`

## Key Differences from Node.js Benchmark

- Uses `@aws-crypto/client-browser` instead of `@aws-crypto/client-node`
- Uses `RawAesKeyringWebCrypto` instead of `RawAesKeyringNode`
- Keyring creation is async (`importCryptoKey` returns a Promise)
- Encrypt/decrypt operates on `Uint8Array` instead of `Buffer`
- Reuses the shared test harness (`tests.js`, `result-base.js`, `utils.js`) via a browser-specific platform adapter

## Output

Results saved as JSON to `../../results/raw-data/browser_js_results.json` with:

- Performance metrics (ops/sec, latency percentiles)
- Memory usage (peak heap, efficiency ratio)
- System information (CPU, memory, Node.js version)

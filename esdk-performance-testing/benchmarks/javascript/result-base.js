// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

"use strict";

/**
 * Create a BenchmarkResult object matching the Go/Rust result format.
 * This module is browser-safe — no Node-specific APIs (fs, os, path, process).
 * Node-specific defaults are only used when the APIs are available.
 *
 * @param {string} language - Language identifier for results
 * @param {object} overrides - Fields to set on the result
 * @returns {object}
 */
function createBenchmarkResult(language, overrides = {}) {
  return {
    test_name: "",
    language,
    data_size: 0,
    concurrency: 1,
    encrypt_latency_ms: 0,
    decrypt_latency_ms: 0,
    end_to_end_latency_ms: 0,
    ops_per_second: 0,
    bytes_per_second: 0,
    peak_memory_mb: 0,
    memory_efficiency_ratio: 0,
    p50_latency: 0,
    p95_latency: 0,
    p99_latency: 0,
    timestamp: new Date().toISOString().replace("T", " ").substring(0, 19),
    node_version: _nodeVersion,
    cpu_count: _cpuCount,
    total_memory_gb: _totalMemoryGb,
    runtime_note:
      "Single-threaded; concurrency is interleaved async I/O, not parallel execution",
    ...overrides,
  };
}

// Detect environment once at module load. Browser callers override these
// fields via platformOverrides so the values here are only used in Node.
const _isNode = typeof process !== "undefined" && !!process.version;
const _nodeVersion = _isNode ? process.version : undefined;
let _cpuCount = 0;
let _totalMemoryGb = 0;

if (_isNode) {
  const os = require("os");
  _cpuCount = os.cpus().length;
  _totalMemoryGb = parseFloat((os.totalmem() / 1024 ** 3).toFixed(1));
} else if (typeof navigator !== "undefined" && navigator.hardwareConcurrency) {
  _cpuCount = navigator.hardwareConcurrency;
}

module.exports = { createBenchmarkResult };

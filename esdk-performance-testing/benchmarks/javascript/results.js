// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

"use strict";

const { createBenchmarkResult } = require("./result-base");
const { average, percentile } = require("./utils");

// ---------------------------------------------------------------------------
// Result factory builder — returns factory functions bound to a language tag
// ---------------------------------------------------------------------------

function createResultFactories(language, platformOverrides = {}) {
  function createThroughputResult(timingData, dataSize, iterations, wallClockSec) {
    const sorted = [...timingData.endToEndTimes].sort((a, b) => a - b);
    const avgEncrypt = average(timingData.encryptTimes);
    const avgDecrypt = average(timingData.decryptTimes);
    const avgE2E = average(timingData.endToEndTimes);
    const opsPerSec = wallClockSec > 0 ? iterations / wallClockSec : 0;

    return createBenchmarkResult(language, {
      ...platformOverrides,
      test_name: "throughput",
      data_size: dataSize,
      encrypt_latency_ms: avgEncrypt,
      decrypt_latency_ms: avgDecrypt,
      end_to_end_latency_ms: avgE2E,
      ops_per_second: opsPerSec,
      bytes_per_second: opsPerSec * dataSize,
      p50_latency: percentile(sorted, 50),
      p95_latency: percentile(sorted, 95),
      p99_latency: percentile(sorted, 99),
    });
  }

  function createMemoryResult(dataSize, peakMemoryMb, memoryEfficiencyRatio, totalAllocationsMb) {
    return createBenchmarkResult(language, {
      ...platformOverrides,
      test_name: "memory",
      data_size: dataSize,
      peak_memory_mb: peakMemoryMb,
      memory_efficiency_ratio: memoryEfficiencyRatio,
      total_allocations_mb: totalAllocationsMb || 0,
    });
  }

  function createConcurrentResult(
    allTimes,
    totalOps,
    totalDurationSec,
    dataSize,
    concurrency,
  ) {
    const sorted = [...allTimes].sort((a, b) => a - b);
    const avgLatency = average(allTimes);
    const opsPerSec = totalDurationSec > 0 ? totalOps / totalDurationSec : 0;

    return createBenchmarkResult(language, {
      ...platformOverrides,
      test_name: "concurrent",
      data_size: dataSize,
      concurrency,
      end_to_end_latency_ms: avgLatency,
      ops_per_second: opsPerSec,
      bytes_per_second: opsPerSec * dataSize,
      p50_latency: percentile(sorted, 50),
      p95_latency: percentile(sorted, 95),
      p99_latency: percentile(sorted, 99),
    });
  }

  return { createThroughputResult, createMemoryResult, createConcurrentResult };
}

module.exports = { createResultFactories };

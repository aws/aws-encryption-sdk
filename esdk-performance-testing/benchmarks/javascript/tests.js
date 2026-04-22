// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

"use strict";

const MEMORY_TEST_ITERATIONS = 5;
// Timing constants aligned with Go's benchmark (esdk_benchmark.go).
// NOTE: setInterval-based sampling cannot collect samples while the single-threaded
// event loop is blocked by synchronous crypto work. Unlike Go (goroutine) and Rust
// (tokio::spawn), there is no way to sample memory from a separate thread in JS.
// The post-operation burst in runMemoryTest partially compensates, but peak memory
// may be undercounted compared to Go/Rust.
const GC_SETTLE_TIME_MS = 5;
const FINAL_SAMPLE_WAIT_MS = 2;

/**
 * Platform abstraction expected by the test functions.
 *
 * Node callers supply:
 *   generateTestData(size)       — returns Uint8Array
 *   nowMs()                      — returns high-resolution timestamp in ms
 *   log(msg)                     — log with timestamp
 *   getMemoryUsage()             — returns { heapUsed, heapTotal } in bytes (or null)
 *   triggerGC()                  — force GC if available (may be no-op)
 *
 * Browser callers supply the same interface backed by WebCrypto / performance.now / etc.
 */

// ---------------------------------------------------------------------------
// Throughput test
// ---------------------------------------------------------------------------

async function runThroughputTest(
  keyring,
  dataSize,
  iterations,
  warmup,
  runEncryptDecryptCycle,
  createThroughputResult,
  platform,
) {
  platform.log(
    `Running throughput test - Size: ${dataSize} bytes, Iterations: ${iterations}`,
  );

  const data = platform.generateTestData(dataSize);

  // Warmup
  for (let i = 0; i < warmup; i++) {
    await runEncryptDecryptCycle(keyring, data);
  }

  // Measurement
  const encryptTimes = [];
  const decryptTimes = [];
  const endToEndTimes = [];

  const wallClockStart = platform.nowMs();
  for (let i = 0; i < iterations; i++) {
    const start = platform.nowMs();
    const { encryptMs, decryptMs } = await runEncryptDecryptCycle(
      keyring,
      data,
    );
    if (!Number.isFinite(encryptMs) || !Number.isFinite(decryptMs)) {
      throw new Error(
        `Invalid timing data: encryptMs=${encryptMs}, decryptMs=${decryptMs}`,
      );
    }
    const e2eMs = platform.nowMs() - start;

    encryptTimes.push(encryptMs);
    decryptTimes.push(decryptMs);
    endToEndTimes.push(e2eMs);
  }
  const wallClockSec = (platform.nowMs() - wallClockStart) / 1000;

  const result = createThroughputResult(
    { encryptTimes, decryptTimes, endToEndTimes },
    dataSize,
    iterations,
    wallClockSec,
  );

  platform.log(
    `Throughput test completed - Ops/sec: ${result.ops_per_second.toFixed(2)}, MB/sec: ${(result.bytes_per_second / (1024 * 1024)).toFixed(2)}`,
  );

  return result;
}

// ---------------------------------------------------------------------------
// Memory test — continuous sampling with delta from baseline like Go/Rust
// NOTE: setInterval/setTimeout-based sampling cannot collect samples while the
// event loop is blocked by synchronous crypto work. Results may undercount peak
// memory compared to Go/Rust which use OS-level memory APIs with dedicated
// threads. The finalDeltaMB fallback partially mitigates this.
// ---------------------------------------------------------------------------

function sampleMemoryContinuously(baselineHeapBytes, platform) {
  const samples = [];
  const interval = setInterval(() => {
    const mem = platform.getMemoryUsage();
    if (!mem) return;
    const heapDelta = Math.max(0, mem.heapUsed - baselineHeapBytes);
    samples.push({ heapDeltaMB: heapDelta / (1024 * 1024) });
  }, 1);
  return {
    samples,
    stop: () => clearInterval(interval),
  };
}

async function runMemoryTest(
  keyring,
  dataSize,
  runEncryptDecryptCycle,
  createMemoryResult,
  platform,
) {
  const hasMem = !!platform.getMemoryUsage();

  if (!hasMem) {
    platform.log(
      "WARNING: Memory API not available. Memory results will be approximate.",
    );
  }

  platform.log(
    `Running memory test - Size: ${dataSize} bytes (${MEMORY_TEST_ITERATIONS} iterations, continuous sampling)`,
  );

  const data = platform.generateTestData(dataSize);

  let overallPeakDelta = 0;
  let overallPeakAllocs = 0;
  const avgDeltaValues = [];

  for (let i = 0; i < MEMORY_TEST_ITERATIONS; i++) {
    // GC + settle like Go
    platform.triggerGC();
    await new Promise((r) => setTimeout(r, GC_SETTLE_TIME_MS));

    // Baseline
    const baseline = platform.getMemoryUsage() || { heapUsed: 0, heapTotal: 0 };
    const baselineHeap = baseline.heapUsed;

    // Start continuous sampling with baseline for delta calculation
    const sampler = sampleMemoryContinuously(baselineHeap, platform);

    const opStart = platform.nowMs();
    await runEncryptDecryptCycle(keyring, data);
    const opEnd = platform.nowMs();

    sampler.stop();
    const durationSec = (opEnd - opStart) / 1000;

    // Post-operation burst: take several rapid samples to catch peak before
    // GC reclaims memory. setInterval cannot fire during sync crypto work,
    // so the continuous sampler often collects zero samples. This burst
    // compensates by sampling immediately after the operation completes.
    for (let b = 0; b < 5; b++) {
      const mem = platform.getMemoryUsage();
      if (mem) {
        const delta = Math.max(0, mem.heapUsed - baselineHeap);
        sampler.samples.push({ heapDeltaMB: delta / (1024 * 1024) });
      }
      await new Promise((r) => setTimeout(r, FINAL_SAMPLE_WAIT_MS));
    }

    // Analyze samples (all values are already deltas from baseline)
    let iterPeakDelta = 0;
    let deltaSum = 0;
    for (const s of sampler.samples) {
      if (s.heapDeltaMB > iterPeakDelta) iterPeakDelta = s.heapDeltaMB;
      deltaSum += s.heapDeltaMB;
    }
    const iterAvgDelta =
      sampler.samples.length > 0 ? deltaSum / sampler.samples.length : 0;

    // Final delta as fallback
    const afterMem = platform.getMemoryUsage() || { heapUsed: 0, heapTotal: 0 };
    const finalDeltaMB =
      Math.max(0, afterMem.heapUsed - baselineHeap) / (1024 * 1024);
    iterPeakDelta = Math.max(iterPeakDelta, finalDeltaMB);

    // Total allocs approximation: heapTotal delta from baseline
    const totalAllocsMB =
      Math.max(0, afterMem.heapTotal - baseline.heapTotal) / (1024 * 1024) +
      iterPeakDelta;

    avgDeltaValues.push(iterAvgDelta > 0 ? iterAvgDelta : finalDeltaMB);
    if (iterPeakDelta > overallPeakDelta) overallPeakDelta = iterPeakDelta;
    if (totalAllocsMB > overallPeakAllocs) overallPeakAllocs = totalAllocsMB;

    platform.log(
      `=== Iteration ${i + 1} === Peak Delta: ${iterPeakDelta.toFixed(2)} MB, Total Allocs: ${totalAllocsMB.toFixed(2)} MB, Avg Delta: ${iterAvgDelta.toFixed(2)} MB (${durationSec.toFixed(3)}s, ${sampler.samples.length} samples)`,
    );
  }

  if (avgDeltaValues.length === 0) {
    throw new Error("All memory test iterations failed");
  }

  const overallAvgDelta =
    avgDeltaValues.reduce((a, b) => a + b, 0) / avgDeltaValues.length;
  const memoryEfficiency =
    overallAvgDelta > 0 ? dataSize / (overallAvgDelta * 1024 * 1024) : 0;

  platform.log("Memory Summary:");
  platform.log(
    `- Peak Memory Delta: ${overallPeakDelta.toFixed(2)} MB (operation overhead)`,
  );
  platform.log(
    `- Average Memory Delta: ${overallAvgDelta.toFixed(2)} MB (operation overhead)`,
  );
  platform.log(
    `- Total Allocations: ${overallPeakAllocs.toFixed(2)} MB (max across all runs)`,
  );

  return createMemoryResult(
    dataSize,
    overallPeakDelta,
    memoryEfficiency,
    overallPeakAllocs,
  );
}

// ---------------------------------------------------------------------------
// Concurrency test
// NOTE: Unlike Go (goroutines on OS threads) and Rust (tokio::spawn with a
// multi-thread runtime), JavaScript's Promise.all runs all workers on a single
// thread with interleaved async I/O. The results are therefore NOT directly
// comparable to Go/Rust concurrent benchmarks. To achieve true parallelism in
// Node.js, worker_threads would be required.
// ---------------------------------------------------------------------------

async function runConcurrentTest(
  keyring,
  dataSize,
  concurrency,
  iterationsPerWorker,
  runEncryptDecryptCycle,
  createConcurrentResult,
  platform,
) {
  platform.log(
    `Running concurrent test - Size: ${dataSize} bytes, Concurrency: ${concurrency}`,
  );

  const totalOps = concurrency * iterationsPerWorker;

  // Generate data once and share across all workers, matching Go's approach
  const data = platform.generateTestData(dataSize);
  const startTime = platform.nowMs();

  const workerPromises = [];
  for (let w = 0; w < concurrency; w++) {
    workerPromises.push(
      workerFunction(
        keyring,
        data,
        iterationsPerWorker,
        runEncryptDecryptCycle,
        platform,
      ),
    );
  }

  const workerResults = await Promise.all(workerPromises);

  const totalDurationSec = (platform.nowMs() - startTime) / 1000;

  const allTimes = [];
  for (const wr of workerResults) {
    if (wr.error) {
      throw new Error(`Worker error: ${wr.error}`);
    }
    allTimes.push(...wr.times);
  }

  if (allTimes.length === 0) {
    throw new Error("No timing data collected from concurrent test");
  }

  const result = createConcurrentResult(
    allTimes,
    totalOps,
    totalDurationSec,
    dataSize,
    concurrency,
  );

  platform.log(
    `Concurrent test completed - Ops/sec: ${result.ops_per_second.toFixed(2)}, Avg latency: ${result.end_to_end_latency_ms.toFixed(2)} ms`,
  );

  return result;
}

async function workerFunction(
  keyring,
  data,
  iterations,
  runEncryptDecryptCycle,
  platform,
) {
  const times = [];

  try {
    for (let i = 0; i < iterations; i++) {
      const start = platform.nowMs();
      await runEncryptDecryptCycle(keyring, data);
      times.push(platform.nowMs() - start);
    }
    return { times, error: null };
  } catch (err) {
    return { times, error: err.message };
  }
}

module.exports = {
  runThroughputTest,
  runMemoryTest,
  runConcurrentTest,
};

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

"use strict";

const os = require("os");
const crypto = require("crypto");
const { getDataSizes, shouldRunTestType } = require("./config");
const { log_msg } = require("./log");
const {
  runThroughputTest,
  runMemoryTest,
  runConcurrentTest,
} = require("./tests");

const DEFAULT_ITERATIONS_PER_WORKER = 5;

/** Node.js platform adapter for the shared test harness. */
const nodePlatform = {
  generateTestData(size) {
    return new Uint8Array(crypto.randomBytes(size));
  },
  nowMs() {
    return Number(process.hrtime.bigint()) / 1e6;
  },
  log: log_msg,
  getMemoryUsage() {
    const m = process.memoryUsage();
    return { heapUsed: m.heapUsed, heapTotal: m.heapTotal };
  },
  triggerGC() {
    if (global.gc) global.gc();
  },
};

/**
 * Orchestrate all benchmark tests based on config.
 */
async function runAllBenchmarks(config, isQuickMode, esdk, resultFactories) {
  const keyring = await esdk.createKeyring();
  log_msg("ESDK client initialized successfully");
  log_msg(
    `Initialized ESDK Benchmark - CPU cores: ${os.cpus().length}, Memory: ${(os.totalmem() / 1024 ** 3).toFixed(1)}GB`,
  );
  log_msg("Starting comprehensive ESDK benchmark suite");

  const dataSizes = getDataSizes(config);
  const iterations = (config.iterations && config.iterations.measurement) || 10;
  const warmup = (config.iterations && config.iterations.warmup) || 5;
  const concurrencyLevels = config.concurrency_levels || [1, 2, 4];
  const iterationsPerWorker = DEFAULT_ITERATIONS_PER_WORKER;
  const results = [];

  const { createThroughputResult, createMemoryResult, createConcurrentResult } =
    resultFactories;

  // Throughput tests
  if (shouldRunTestType(config, "throughput", isQuickMode)) {
    log_msg("Running throughput tests...");
    for (const dataSize of dataSizes) {
      try {
        const result = await runThroughputTest(
          keyring,
          dataSize,
          iterations,
          warmup,
          esdk.runEncryptDecryptCycle,
          createThroughputResult,
          nodePlatform,
        );
        results.push(result);
        log_msg(
          `Throughput test completed: ${result.ops_per_second.toFixed(2)} ops/sec`,
        );
      } catch (err) {
        log_msg(`Throughput test failed: ${err.message}`);
      }
    }
  } else {
    log_msg("Skipping throughput tests (not in test_types)");
  }

  // Memory tests
  if (shouldRunTestType(config, "memory", isQuickMode)) {
    log_msg("Running memory tests...");
    for (const dataSize of dataSizes) {
      try {
        const result = await runMemoryTest(
          keyring,
          dataSize,
          esdk.runEncryptDecryptCycle,
          createMemoryResult,
          nodePlatform,
        );
        results.push(result);
        log_msg(
          `Memory test completed: ${result.peak_memory_mb.toFixed(2)} MB peak`,
        );
      } catch (err) {
        log_msg(`Memory test failed: ${err.message}`);
      }
    }
  } else {
    log_msg("Skipping memory tests (not in test_types)");
  }

  // Concurrency tests
  if (shouldRunTestType(config, "concurrency", isQuickMode)) {
    log_msg("Running concurrency tests...");
    for (const dataSize of dataSizes) {
      for (const concurrency of concurrencyLevels) {
        if (concurrency <= 1) continue;
        try {
          const result = await runConcurrentTest(
            keyring,
            dataSize,
            concurrency,
            iterationsPerWorker,
            esdk.runEncryptDecryptCycle,
            createConcurrentResult,
            nodePlatform,
          );
          results.push(result);
          log_msg(
            `Concurrent test completed: ${result.ops_per_second.toFixed(2)} ops/sec @ ${concurrency} workers`,
          );
        } catch (err) {
          log_msg(`Concurrent test failed: ${err.message}`);
        }
      }
    }
  } else {
    log_msg("Skipping concurrency tests (not in test_types)");
  }

  log_msg(`Benchmark suite completed. Total results: ${results.length}`);
  return results;
}

module.exports = {
  runAllBenchmarks,
};

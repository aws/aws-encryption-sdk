// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/**
 * Browser ESDK benchmark — runs entirely in the browser.
 * Bundled with esbuild, executed in headless Chrome via Puppeteer.
 * All results are emitted via console.log as JSON for the launcher to collect.
 *
 * Reuses the shared test harness (tests.js) and result factories (results.js)
 * via a browser-specific platform adapter.
 */

import { Buffer } from "buffer";
globalThis.Buffer = Buffer;

import {
  RawAesKeyringWebCrypto,
  buildClient,
  CommitmentPolicy,
  RawAesWrappingSuiteIdentifier,
  synchronousRandomValues,
} from "@aws-crypto/client-browser";

import { formatTimestamp } from "../../utils.js";
import {
  runThroughputTest,
  runMemoryTest,
  runConcurrentTest,
} from "../../tests.js";
import { createResultFactories } from "../../results.js";

const { encrypt, decrypt } = buildClient(
  CommitmentPolicy.REQUIRE_ENCRYPT_REQUIRE_DECRYPT,
);
const wrappingSuite =
  RawAesWrappingSuiteIdentifier.AES256_GCM_IV12_TAG16_NO_PADDING;

const LANGUAGE = "browser-js";

// ---------------------------------------------------------------------------
// Browser platform adapter
// ---------------------------------------------------------------------------

function log(msg) {
  console.log(`${formatTimestamp()} ${msg}`);
}

const browserPlatform = {
  generateTestData(size) {
    const buf = new Uint8Array(size);
    for (let offset = 0; offset < size; offset += 65536) {
      const chunk = new Uint8Array(
        buf.buffer,
        offset,
        Math.min(65536, size - offset),
      );
      crypto.getRandomValues(chunk);
    }
    return buf;
  },
  nowMs() {
    return performance.now();
  },
  log,
  getMemoryUsage() {
    // performance.memory is Chrome-only and non-standard.
    if (!performance.memory) return null;
    return {
      heapUsed: performance.memory.usedJSHeapSize,
      heapTotal: performance.memory.totalJSHeapSize,
    };
  },
  triggerGC() {
    // No reliable GC trigger in browsers
  },
};

// ---------------------------------------------------------------------------
// ESDK operations
// ---------------------------------------------------------------------------

async function createKeyring() {
  const key = synchronousRandomValues(32);
  const masterKey = await RawAesKeyringWebCrypto.importCryptoKey(
    key,
    wrappingSuite,
  );
  return new RawAesKeyringWebCrypto({
    keyName: "test-aes-256-key",
    keyNamespace: "esdk-performance-test",
    wrappingSuite,
    masterKey,
  });
}

function arraysEqual(a, b) {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) return false;
  }
  return true;
}

async function runEncryptDecryptCycle(keyring, data) {
  const ctx = { purpose: "performance-test", size: data.length.toString() };

  const encStart = performance.now();
  const { result: ciphertext } = await encrypt(keyring, data, {
    encryptionContext: ctx,
  });
  const encryptMs = performance.now() - encStart;

  const decStart = performance.now();
  const { plaintext: decrypted } = await decrypt(keyring, ciphertext);
  const decryptMs = performance.now() - decStart;

  if (!arraysEqual(new Uint8Array(decrypted), data)) {
    throw new Error("Decrypted data does not match original");
  }

  return { encryptMs, decryptMs };
}

// ---------------------------------------------------------------------------
// Result factories — delegate to shared createResultFactories with browser overrides
// ---------------------------------------------------------------------------

function createBrowserResultFactories() {
  return createResultFactories(LANGUAGE, {
    node_version: undefined,
    browser_version: navigator.userAgent,
    cpu_count: navigator.hardwareConcurrency || 0,
    total_memory_gb: 0,
    runtime_note:
      "Browser JS is single-threaded; concurrency is interleaved async I/O, not parallel execution",
  });
}

// ---------------------------------------------------------------------------
// Config — injected by launcher via window.__BENCHMARK_CONFIG__
// ---------------------------------------------------------------------------

function getConfig() {
  if (window.__BENCHMARK_CONFIG__) {
    return window.__BENCHMARK_CONFIG__;
  }
  throw new Error("No config injected. Run via the launcher (node src/launcher.js).");
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

function withTimeout(promise, ms, label) {
  if (!ms) return promise;
  return Promise.race([
    promise,
    new Promise((_, reject) =>
      setTimeout(() => reject(new Error(`${label} timed out after ${ms}ms`)), ms),
    ),
  ]);
}

async function main() {
  const config = getConfig();
  const testTimeout = config.perTestTimeoutMs || 0;

  const keyring = await createKeyring();
  log("ESDK client initialized successfully");
  log(
    `Initialized ESDK Browser Benchmark - UserAgent: ${navigator.userAgent.substring(0, 60)}`,
  );
  log("Starting comprehensive ESDK benchmark suite");

  const results = [];
  const {
    createThroughputResult,
    createMemoryResult,
    createConcurrentResult,
  } = createBrowserResultFactories();

  // Throughput
  if (config.testTypes.includes("throughput")) {
    log("Running throughput tests...");
    for (const size of config.dataSizes) {
      try {
        const result = await withTimeout(
          runThroughputTest(
            keyring,
            size,
            config.iterations,
            config.warmup,
            runEncryptDecryptCycle,
            createThroughputResult,
            browserPlatform,
          ),
          testTimeout,
          `Throughput ${size}B`,
        );
        results.push(result);
        log(
          `Throughput test completed: ${result.ops_per_second.toFixed(2)} ops/sec`,
        );
      } catch (e) {
        log(`Throughput test failed: ${e.message}`);
      }
    }
  }

  // Memory
  if (config.testTypes.includes("memory")) {
    log("Running memory tests...");
    for (const size of config.dataSizes) {
      try {
        const result = await withTimeout(
          runMemoryTest(
            keyring,
            size,
            runEncryptDecryptCycle,
            createMemoryResult,
            browserPlatform,
          ),
          testTimeout,
          `Memory ${size}B`,
        );
        results.push(result);
        log(`Memory test completed: ${result.peak_memory_mb.toFixed(2)} MB peak`);
      } catch (e) {
        log(`Memory test failed: ${e.message}`);
      }
    }
  }

  // Concurrency
  if (config.testTypes.includes("concurrency")) {
    log("Running concurrency tests...");
    for (const size of config.dataSizes) {
      for (const c of config.concurrencyLevels) {
        if (c <= 1) continue;
        try {
          const result = await withTimeout(
            runConcurrentTest(
              keyring,
              size,
              c,
              config.iterationsPerWorker || 5,
              runEncryptDecryptCycle,
              createConcurrentResult,
              browserPlatform,
            ),
            testTimeout,
            `Concurrent ${size}B @${c}`,
          );
          results.push(result);
          log(
            `Concurrent test completed: ${result.ops_per_second.toFixed(2)} ops/sec @ ${c} workers`,
          );
        } catch (e) {
          log(`Concurrent test failed: ${e.message}`);
        }
      }
    }
  }

  log(`Benchmark suite completed. Total results: ${results.length}`);

  // Emit results as JSON for Puppeteer to collect
  console.log("__BENCHMARK_RESULTS__" + JSON.stringify(results));
  window.__benchmarkDone = true;
}

main().catch((e) => {
  console.error("Benchmark failed:", e.message);
  console.log("__BENCHMARK_RESULTS__[]");
  window.__benchmarkDone = true;
});

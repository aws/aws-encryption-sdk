#!/usr/bin/env node
// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

"use strict";

/**
 * Launcher: bundles the browser benchmark with esbuild, serves it,
 * runs headless Chrome via Puppeteer, collects results.
 */

const { execSync } = require("child_process");
const fs = require("fs");
const path = require("path");
const http = require("http");
const yaml = require("js-yaml");
const puppeteer = require("puppeteer");
const { Command } = require("commander");
const { saveResults } = require("../../save-results");

const ROOT = path.resolve(__dirname, "..");
const DIST = path.join(ROOT, "dist");
const SRC = path.join(ROOT, "src");

// Per-test timeout in ms (5 minutes). Prevents a single large payload from
// consuming the entire global timeout budget.
const PER_TEST_TIMEOUT_MS = 300000;

const program = new Command();
program
  .name("esdk-browser-benchmark")
  .description(
    "ESDK Browser Performance Benchmark (headless Chrome via Puppeteer)",
  )
  .option(
    "--config <path>",
    "Path to test configuration file",
    path.resolve(SRC, "../../../../config/test-scenarios.yaml"),
  )
  .option(
    "--output <path>",
    "Path to output results file",
    path.resolve(SRC, "../../../../results/raw-data/browser_js_results.json"),
  )
  .option("--quick", "Run quick test with reduced iterations", false);
program.parse(process.argv);
const opts = program.opts();
const quick = opts.quick;
const configPath = opts.config;
const outputPath = opts.output;

/**
 * Read test-scenarios.yaml and convert to the browser config format.
 */
function loadBrowserConfig(yamlPath, isQuick) {
  const resolved = path.resolve(yamlPath);
  if (!fs.existsSync(resolved)) {
    console.warn(`Config file not found: ${resolved}, using defaults`);
    return null;
  }
  const raw = fs.readFileSync(resolved, "utf8");
  const cfg = yaml.load(raw);

  if (isQuick && cfg.quick_config) {
    const q = cfg.quick_config;
    const sizes = [];
    if (q.data_sizes) {
      for (const cat of ["small", "medium", "large"]) {
        if (Array.isArray(q.data_sizes[cat])) sizes.push(...q.data_sizes[cat]);
      }
    }
    return {
      dataSizes: sizes,
      iterations: q.iterations ? q.iterations.measurement : 3,
      warmup: q.iterations ? q.iterations.warmup : 3,
      concurrencyLevels: q.concurrency_levels || [2],
      iterationsPerWorker: cfg.iterations_per_worker || 5,
      testTypes: q.test_types || ["throughput", "memory", "concurrency"],
      perTestTimeoutMs: PER_TEST_TIMEOUT_MS,
    };
  }

  const sizes = [];
  if (cfg.data_sizes) {
    for (const cat of ["small", "medium", "large"]) {
      if (Array.isArray(cfg.data_sizes[cat]))
        sizes.push(...cfg.data_sizes[cat]);
    }
  }
  return {
    dataSizes: sizes,
    iterations: cfg.iterations ? cfg.iterations.measurement : 10,
    warmup: cfg.iterations ? cfg.iterations.warmup : 5,
    concurrencyLevels: cfg.concurrency_levels || [2, 4, 8, 16],
    iterationsPerWorker: cfg.iterations_per_worker || 5,
    testTypes: ["throughput", "memory", "concurrency"],
    perTestTimeoutMs: PER_TEST_TIMEOUT_MS,
  };
}

async function main() {
  // 1. Bundle
  console.log("Bundling browser benchmark...");
  fs.mkdirSync(DIST, { recursive: true });

  execSync(
    `npx esbuild ${path.join(SRC, "benchmark.js")} ` +
      `--bundle --format=iife --platform=browser ` +
      `--outfile=${path.join(DIST, "bundle.js")} ` +
      `--external:os ` +
      `--define:global=globalThis`,
    { cwd: ROOT, stdio: "inherit" },
  );

  // Copy HTML
  fs.copyFileSync(path.join(SRC, "index.html"), path.join(DIST, "index.html"));

  // 2. Serve
  const server = http.createServer((req, res) => {
    const urlPath = req.url.split("?")[0];
    const file = urlPath === "/" ? "/index.html" : urlPath;
    const fp = path.join(DIST, file);
    if (fs.existsSync(fp)) {
      const ext = path.extname(fp);
      const ct = ext === ".js" ? "application/javascript" : "text/html";
      res.writeHead(200, { "Content-Type": ct });
      res.end(fs.readFileSync(fp));
    } else {
      res.writeHead(404);
      res.end("Not found");
    }
  });

  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  const port = server.address().port;

  // Track connections so we can destroy them on shutdown
  const connections = new Set();
  server.on("connection", (conn) => {
    connections.add(conn);
    conn.on("close", () => connections.delete(conn));
  });
  const url = `http://127.0.0.1:${port}/${quick ? "?quick" : ""}`;
  console.log(`Serving at ${url}`);

  // 3. Launch headless Chrome
  const browser = await puppeteer.launch({
    headless: "new",
    args: [
      "--no-sandbox",
      "--enable-precise-memory-info", // enables performance.memory
      "--js-flags=--expose-gc",
    ],
  });

  const page = await browser.newPage();

  let results = [];

  const resultsPromise = new Promise((resolve) => {
    page.on("console", (msg) => {
      const text = msg.text();
      if (text.startsWith("__BENCHMARK_RESULTS__")) {
        try {
          results = JSON.parse(text.substring("__BENCHMARK_RESULTS__".length));
        } catch (e) {
          console.error("Failed to parse results:", e.message);
        }
        resolve();
      } else {
        console.log(text);
      }
    });
  });

  page.on("pageerror", (err) => console.error("Page error:", err.message));

  // Load config from YAML and inject into the page
  const browserConfig = loadBrowserConfig(configPath, quick);
  if (browserConfig) {
    await page.evaluateOnNewDocument((cfg) => {
      window.__BENCHMARK_CONFIG__ = cfg;
    }, browserConfig);
    console.log("Injected config from", path.resolve(configPath));
  }

  console.log("Running benchmark in headless Chrome...\n");
  await page.goto(url, { waitUntil: "load", timeout: 0 });

  // Wait for the benchmark to emit results (up to 10 minutes)
  await Promise.race([
    resultsPromise,
    new Promise((_, reject) =>
      setTimeout(() => reject(new Error("Benchmark timed out")), 600000),
    ),
  ]);

  await browser.close();
  for (const conn of connections) conn.destroy();
  server.close();

  // 4. Save results
  if (results.length > 0) {
    const resolvedOutput = saveResults(results, outputPath, "browser-js", {
      browser_version: results[0].browser_version || "unknown",
    });

    console.log(`\n=== ESDK Browser Benchmark Summary ===`);
    console.log(`Total tests completed: ${results.length}`);
    console.log(`Results saved to: ${resolvedOutput}`);

    const maxOps = Math.max(
      0,
      ...results
        .filter((r) => r.test_name === "throughput")
        .map((r) => r.ops_per_second),
    );
    if (maxOps > 0)
      console.log(`Maximum throughput: ${maxOps.toFixed(2)} ops/sec`);
  } else {
    console.error("No results collected!");
    process.exit(1);
  }

  process.exit(0);
}

main().catch((err) => {
  console.error("Launcher failed:", err.message);
  process.exit(1);
});

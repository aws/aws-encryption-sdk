// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

"use strict";

const { Command } = require("commander");
const { loadConfig, adjustConfigForQuickMode } = require("../../config");
const { runAllBenchmarks } = require("../../benchmark");
const { createResultFactories } = require("../../results");
const { saveResults } = require("../../save-results");
const esdk = require("./esdk");

const LANGUAGE = "javascript";
const SCRIPT_DIR = require("path").dirname(__filename);

if (!global.gc) {
  console.warn(
    "WARNING: --expose-gc not enabled. Memory test results will be less accurate.\n" +
      "Run with: node --expose-gc src/main.js (or use `npm run benchmark`)\n",
  );
}

async function main() {
  const program = new Command();
  program
    .name("esdk-node-benchmark")
    .description("ESDK Node.js Performance Benchmark")
    .option(
      "--config <path>",
      "Path to test configuration file",
      require("path").resolve(
        SCRIPT_DIR,
        "../../../../config/test-scenarios.yaml",
      ),
    )
    .option(
      "--output <path>",
      "Path to output results file",
      require("path").resolve(
        SCRIPT_DIR,
        "../../../../results/raw-data/node_results.json",
      ),
    )
    .option("--quick", "Run quick test with reduced iterations", false);

  program.parse(process.argv);
  const opts = program.opts();

  try {
    const config = loadConfig(opts.config);

    if (opts.quick) {
      adjustConfigForQuickMode(config);
    }

    const resultFactories = createResultFactories(LANGUAGE);
    const results = await runAllBenchmarks(
      config,
      opts.quick,
      esdk,
      resultFactories,
    );

    saveResults(results, opts.output, LANGUAGE);

    const resolvedOutput = require("path").resolve(opts.output);
    console.log(`\n=== ESDK Node.js Benchmark Summary ===`);
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
  } catch (err) {
    console.error(`Benchmark failed: ${err.message}`);
    process.exit(1);
  }
}

main();

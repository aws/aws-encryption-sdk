// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

"use strict";

const fs = require("fs");
const path = require("path");
const yaml = require("js-yaml");

/**
 * Load test configuration from YAML file.
 * @param {string} configPath - Path to the YAML config file
 * @returns {object} Parsed configuration
 */
function loadConfig(configPath) {
  const resolvedPath = path.resolve(configPath);
  if (!fs.existsSync(resolvedPath)) {
    throw new Error(`Config file not found: ${resolvedPath}`);
  }

  try {
    const raw = fs.readFileSync(resolvedPath, "utf8");
    return yaml.load(raw);
  } catch (err) {
    throw new Error(`Failed to parse config file: ${err.message}`);
  }
}

/**
 * Collect all data sizes from config into a flat array of numbers.
 * Values can exceed 2^31 so they are kept as standard JS numbers (safe up to 2^53).
 * @param {object} config
 * @returns {number[]}
 */
function getDataSizes(config) {
  const sizes = [];
  const dataSizes = config.data_sizes || {};
  for (const category of ["small", "medium", "large"]) {
    if (Array.isArray(dataSizes[category])) {
      for (const s of dataSizes[category]) {
        const n = Number(s);
        if (!Number.isFinite(n) || n <= 0) {
          throw new Error(`Invalid data size in ${category}: ${s}`);
        }
        sizes.push(n);
      }
    }
  }
  return sizes;
}

/**
 * Adjust config in-place for quick mode using quick_config section.
 * @param {object} config
 */
function adjustConfigForQuickMode(config) {
  const quick = config.quick_config;
  if (!quick) {
    throw new Error(
      "Quick mode requested but no quick_config found in config file",
    );
  }

  config.iterations = config.iterations || {};
  config.data_sizes = config.data_sizes || {};

  config.iterations.measurement = quick.iterations.measurement;
  config.iterations.warmup = quick.iterations.warmup;
  config.data_sizes.small = quick.data_sizes.small || [];
  config.data_sizes.medium = [];
  config.data_sizes.large = [];
  config.concurrency_levels = quick.concurrency_levels || [1, 2];
}

/**
 * Determine whether a given test type should run.
 * In quick mode, only test types listed in quick_config.test_types are executed.
 * @param {object} config
 * @param {string} testType
 * @param {boolean} isQuickMode
 * @returns {boolean}
 */
function shouldRunTestType(config, testType, isQuickMode) {
  if (isQuickMode) {
    const quick = config.quick_config;
    if (quick && Array.isArray(quick.test_types)) {
      return quick.test_types.includes(testType);
    }
  }
  return true;
}

module.exports = {
  loadConfig,
  getDataSizes,
  adjustConfigForQuickMode,
  shouldRunTestType,
};

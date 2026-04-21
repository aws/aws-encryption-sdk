// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

"use strict";

/**
 * Shared pure utilities used by both Node and browser benchmarks.
 * This module must remain free of Node-specific APIs (fs, os, process, crypto)
 * so that esbuild can bundle it for the browser.
 */

function average(arr) {
  if (!arr.length) return 0;
  return arr.reduce((a, b) => a + b, 0) / arr.length;
}

/**
 * Interpolated percentile matching Go/Rust implementation.
 * @param {number[]} sortedArr - Pre-sorted array of values
 * @param {number} p - Percentile on 0-100 scale (e.g. 50, 95, 99)
 * @returns {number}
 */
function percentile(sortedArr, p) {
  if (!sortedArr.length) return 0;
  if (p <= 0) return sortedArr[0];
  if (p >= 100) return sortedArr[sortedArr.length - 1];

  const index = (p / 100.0) * (sortedArr.length - 1);
  const lower = Math.floor(index);
  const upper = Math.ceil(index);

  if (lower === upper) return sortedArr[lower];

  const weight = index - lower;
  return sortedArr[lower] * (1 - weight) + sortedArr[upper] * weight;
}

/**
 * Format a timestamp matching Go's log.Printf format.
 * @returns {string} e.g. "2026/04/15 15:32:52"
 */
function formatTimestamp() {
  const now = new Date();
  return `${now.getFullYear()}/${String(now.getMonth() + 1).padStart(2, "0")}/${String(now.getDate()).padStart(2, "0")} ${String(now.getHours()).padStart(2, "0")}:${String(now.getMinutes()).padStart(2, "0")}:${String(now.getSeconds()).padStart(2, "0")}`;
}

module.exports = { average, percentile, formatTimestamp };

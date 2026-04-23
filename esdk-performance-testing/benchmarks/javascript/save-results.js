// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

"use strict";

const fs = require("fs");
const path = require("path");

/**
 * Save benchmark results to a JSON file.
 * Node-only — uses fs and path.
 * @param {object[]} results
 * @param {string} outputPath
 * @param {string} language
 * @param {object} [extraMetadata] - Additional metadata fields (e.g. browser_version)
 * @returns {string} Resolved absolute path of the written file
 */
function saveResults(results, outputPath, language, extraMetadata) {
  const resolvedPath = path.resolve(outputPath);
  fs.mkdirSync(path.dirname(resolvedPath), { recursive: true });

  const metadata = {
    language,
    timestamp: new Date().toISOString().replace("T", " ").substring(0, 19),
    total_tests: results.length,
  };

  if (results.length > 0) {
    metadata.node_version = results[0].node_version;
    metadata.cpu_count = results[0].cpu_count;
    metadata.total_memory_gb = results[0].total_memory_gb;
  }

  Object.assign(metadata, extraMetadata);

  // Remove undefined fields so they don't serialize as null in JSON
  for (const key of Object.keys(metadata)) {
    if (metadata[key] === undefined) delete metadata[key];
  }

  const payload = { metadata, results };
  fs.writeFileSync(resolvedPath, JSON.stringify(payload, null, 2));
  return resolvedPath;
}

module.exports = { saveResults };

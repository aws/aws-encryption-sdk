// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

"use strict";

/**
 * Node-only logging module.
 * Uses process.stdout.write which is not available in browser environments.
 * The browser benchmark defines its own log() using console.log instead.
 */

const { formatTimestamp } = require("./utils");

/**
 * Log a message with a timestamp prefix, matching Go's log.Printf format.
 * Example: log_msg("Running test - Size: 1024 bytes")
 * Output:  "2026/04/15 15:32:52 Running test - Size: 1024 bytes"
 */
function log_msg(msg) {
  process.stdout.write(`${formatTimestamp()} ${msg}\n`);
}

module.exports = { log_msg };

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

"use strict";

const crypto = require("crypto");
const {
  RawAesKeyringNode,
  buildClient,
  CommitmentPolicy,
  RawAesWrappingSuiteIdentifier,
} = require("@aws-crypto/client-node");

const { encrypt, decrypt } = buildClient(
  CommitmentPolicy.REQUIRE_ENCRYPT_REQUIRE_DECRYPT,
);

/**
 * Create a RawAesKeyringNode for benchmarking.
 * @returns {Promise<RawAesKeyringNode>}
 */
async function createKeyring() {
  const unencryptedMasterKey = crypto.randomBytes(32);
  return new RawAesKeyringNode({
    keyName: "test-aes-256-key",
    keyNamespace: "esdk-performance-test",
    unencryptedMasterKey,
    wrappingSuite:
      RawAesWrappingSuiteIdentifier.AES256_GCM_IV12_TAG16_NO_PADDING,
  });
}

/**
 * Run a single encrypt -> decrypt cycle and return timing in milliseconds.
 * @param {object} keyring
 * @param {Uint8Array} data
 * @returns {Promise<{encryptMs: number, decryptMs: number}>}
 */
async function runEncryptDecryptCycle(keyring, data) {
  const encryptionContext = {
    purpose: "performance-test",
    size: data.length.toString(),
  };

  const encStart = process.hrtime.bigint();
  const { result: ciphertext } = await encrypt(keyring, data, {
    encryptionContext,
  });
  const encEnd = process.hrtime.bigint();
  const encryptMs = Number(encEnd - encStart) / 1e6;

  const decStart = process.hrtime.bigint();
  const { plaintext: decrypted } = await decrypt(keyring, ciphertext);
  const decEnd = process.hrtime.bigint();
  const decryptMs = Number(decEnd - decStart) / 1e6;

  // decrypt() returns a Buffer; data is a Uint8Array.
  // Buffer.equals() accepts Uint8Array in Node.js, so this cross-type comparison works.
  if (!decrypted.equals(data)) {
    throw new Error("Decrypted data does not match original");
  }

  return { encryptMs, decryptMs };
}

module.exports = {
  createKeyring,
  runEncryptDecryptCycle,
};

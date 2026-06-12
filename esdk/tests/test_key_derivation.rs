// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/encrypt.md#get-the-encryption-materials
//! Key derivation requirements.
//!
//! These call `derive_keys` directly (via `__test_internals`) and assert on the
//! derived key bytes. An encrypt/decrypt round-trip cannot prove these
//! requirements: a wrong-but-consistent KDF on both sides still round-trips, so
//! it shows neither which KDF was selected nor what the KDF produced.

mod fixtures;
mod test_helpers;

use aws_esdk::__test_internals::derive_keys;
use aws_mpl_legacy::suites::{get_algorithm_suite_info, AlgorithmSuite};

// AES-256 data key length, required by the `derive_keys` length precondition.
const AES_256_KEY_LEN: usize = 32;

// AES-256 algorithm suite binary IDs (data-format wire values).
const NO_KDF_AES_256: [u8; 2] = [0x00, 0x78];
const HKDF_SHA256_AES_256: [u8; 2] = [0x01, 0x78];

fn suite(binary_id: [u8; 2]) -> &'static AlgorithmSuite {
    get_algorithm_suite_info(binary_id).expect("known algorithm suite id")
}

// A deterministic plaintext data key of the AES-256 key length.
fn sample_key() -> Vec<u8> {
    (0..AES_256_KEY_LEN)
        .map(|i| u8::try_from(i).expect("index fits in u8"))
        .collect()
}

// Identity KDF returns the plaintext data key unchanged (and no commitment key).
#[test]
fn test_identity_kdf_derived_key_equals_plaintext_key() {
    let pdk = sample_key();
    let message_id = [0u8; 16];

    let derived = derive_keys(&message_id, &pdk, suite(NO_KDF_AES_256), false).unwrap();

    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //# - If the key derivation algorithm is the [identity KDF](../framework/algorithm-suites.md#identity-kdf),
    //# then the derived data key MUST be the same as the plaintext data key.
    //
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //= type=test
    //# If the key derivation algorithm is the [identity KDF](../framework/algorithm-suites.md#identity-kdf),
    //# then the derived data key MUST be the same as the plaintext data key.
    assert_eq!(derived.data_key.as_slice(), pdk.as_slice());
    assert!(derived.commitment_key.is_none());
}

// The KDF that runs is the one named by the algorithm suite: the identity suite
// returns the key verbatim, the HKDF suite transforms it.
#[test]
fn test_kdf_algorithm_is_selected_from_suite() {
    let pdk = sample_key();
    let message_id = [7u8; 16];

    let identity = derive_keys(&message_id, &pdk, suite(NO_KDF_AES_256), false).unwrap();
    let hkdf = derive_keys(&message_id, &pdk, suite(HKDF_SHA256_AES_256), false).unwrap();

    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=Identity suite returns the key verbatim while the HKDF suite transforms it, proving the KDF follows the suite
    //# The algorithm used to derive a data key from the plaintext data key MUST be
    //# the [key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm) included in the
    //# [algorithm suite](../framework/algorithm-suites.md) defined above.
    assert_eq!(identity.data_key.as_slice(), pdk.as_slice());
    assert_ne!(hkdf.data_key.as_slice(), pdk.as_slice());
}

// Expected V1 HKDF-SHA256 derived key for the inputs in the test below, computed
// independently with OpenSSL (RFC 5869 HKDF) so this is a known-answer test rather
// than a self-consistency check:
//
//   openssl kdf -keylen 32 -binary -kdfopt digest:SHA256 \
//     -kdfopt hexkey:000102...1f                       (sample_key: bytes 0..31) \
//     -kdfopt hexsalt:<32 zero bytes>                  (salt = hash length, all zero) \
//     -kdfopt hexinfo:0178<0x01 * 16>                  (suite binary id || message id) \
//     HKDF
const EXPECTED_HKDF_SHA256_DERIVED_KEY: [u8; 32] = [
    0x46, 0x91, 0x38, 0xc3, 0x42, 0xa4, 0xb0, 0xc7, 0x48, 0x76, 0x02, 0xf7, 0xdc, 0x96, 0x7d, 0xe5,
    0x44, 0xb5, 0x0a, 0xb2, 0xba, 0xda, 0x90, 0xdc, 0x32, 0xa8, 0x03, 0xa5, 0xaf, 0x7c, 0xdb, 0x35,
];

// The end-to-end interop test vectors (PR M3) exercise this same derivation against
// known-good ciphertexts from other implementations; this unit test pins the V1 HKDF
// process directly against an independent HKDF computation.
#[test]
fn test_hkdf_derivation_process() {
    let suite = suite(HKDF_SHA256_AES_256);
    let pdk = sample_key();
    let message_id_a = [1u8; 16];
    let message_id_b = [2u8; 16];

    let a1 = derive_keys(&message_id_a, &pdk, suite, false).unwrap();
    let a2 = derive_keys(&message_id_a, &pdk, suite, false).unwrap();
    let b = derive_keys(&message_id_b, &pdk, suite, false).unwrap();

    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=Derived key matches an independent OpenSSL HKDF-SHA256 computation over the same salt, ikm, and info
    //# - If the key derivation algorithm is [HKDF](../framework/algorithm-suites.md#hkdf),
    //# the derivation process used MUST be the process described in [HKDF Encryption Key](../transitive-requirements.md#hkdf-encryption-key).
    assert_eq!(a1.data_key.as_slice(), &EXPECTED_HKDF_SHA256_DERIVED_KEY);

    // Supporting regression checks: derivation is deterministic, is bound to the
    // message id, and produces no commitment key for V1 suites.
    assert_eq!(
        a2.data_key.as_slice(),
        a1.data_key.as_slice(),
        "HKDF derivation must be deterministic"
    );
    assert_ne!(
        a1.data_key.as_slice(),
        b.data_key.as_slice(),
        "HKDF must bind the derived key to the message id"
    );
    assert!(a1.commitment_key.is_none(), "V1 HKDF suites have no commitment key");
}

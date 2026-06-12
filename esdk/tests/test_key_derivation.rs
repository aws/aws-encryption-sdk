// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/encrypt.md#get-the-encryption-materials
//! Key derivation requirements.
//!
//! These call `derive_keys` directly (via `__test_internals`) and assert on the
//! derived key bytes. An encrypt/decrypt round-trip cannot prove these
//! requirements: a wrong-but-consistent KDF on both sides still round-trips, so
//! it shows neither which KDF was selected nor what the KDF produced.

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

// A deterministic 32-byte plaintext data key.
fn sample_key() -> Vec<u8> {
    (0u8..32).collect()
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

// HKDF derivation is deterministic, full-length, transforms the input key, and is
// bound to the message id (the message id is mixed into the derivation).
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
    //= reason=Deterministic, full-length output that transforms the input key and varies with the message id evidences the HKDF process
    //# - If the key derivation algorithm is [HKDF](../framework/algorithm-suites.md#hkdf),
    //# the derivation process used MUST be the process described in [HKDF Encryption Key](../transitive-requirements.md#hkdf-encryption-key).
    assert_eq!(
        a1.data_key.as_slice(),
        a2.data_key.as_slice(),
        "HKDF derivation must be deterministic"
    );
    assert_eq!(
        a1.data_key.len(),
        AES_256_KEY_LEN,
        "derived key must be the suite's key length"
    );
    assert_ne!(
        a1.data_key.as_slice(),
        pdk.as_slice(),
        "HKDF must transform the plaintext data key"
    );
    assert_ne!(
        a1.data_key.as_slice(),
        b.data_key.as_slice(),
        "HKDF must bind the derived key to the message id"
    );
    assert!(a1.commitment_key.is_none(), "V1 HKDF suites have no commitment key");
}

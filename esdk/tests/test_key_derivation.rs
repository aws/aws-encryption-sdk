// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/encrypt.md#get-the-encryption-materials
//! and spec/client-apis/key-derivation.md.

mod fixtures;
mod test_helpers;

use aws_esdk::ErrorKind;
use aws_esdk::__test_internals::{derive_key_v2, derive_keys};
use aws_mpl_legacy::primitives::{hkdf_expand, hkdf_extract};
use aws_mpl_legacy::suites::{get_algorithm_suite_info, AlgorithmSuite};

// AES-256 data key length, required by the `derive_keys` length precondition.
const AES_256_KEY_LEN: usize = 32;

// V1 message ID length is 128 bits; V2 is 256 bits.
const V2_MESSAGE_ID_LEN: usize = 32;

// AES-256 algorithm suite binary IDs (data-format wire values).
const NO_KDF_AES_256: [u8; 2] = [0x00, 0x78];
const HKDF_SHA256_AES_256: [u8; 2] = [0x01, 0x78];
// V1 AES-256 with HKDF-SHA-384 + ECDSA-P-384. Differs from HKDF_SHA256_AES_256
// only in the HKDF hash, so it lets us test that the suite's hash function
// (not a hard-coded one) actually drives derivation.
const HKDF_SHA384_AES_256_SIG: [u8; 2] = [0x03, 0x78];
const HKDF_SHA512_COMMIT_AES_256: [u8; 2] = [0x04, 0x78];

// V2 HKDF-Expand info labels — must match the constants in src/key_derivation.rs.
const KEY_LABEL: &[u8] = b"DERIVEKEY";
const COMMIT_LABEL: &[u8] = b"COMMITKEY";

fn suite(binary_id: [u8; 2]) -> &'static AlgorithmSuite {
    get_algorithm_suite_info(binary_id).expect("known algorithm suite id")
}

// A deterministic plaintext data key of the AES-256 key length.
fn sample_key() -> Vec<u8> {
    (0..AES_256_KEY_LEN)
        .map(|i| u8::try_from(i).expect("index fits in u8"))
        .collect()
}

// These call `derive_keys` directly so the assertions observe the derived bytes; a
// round-trip can't prove which KDF ran or what it produced (both sides would agree).

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
// returns the key verbatim, the HKDF suite transforms it. The second HKDF suite
// (SHA-384) is a supporting regression check that a *different* HKDF suite yields
// different bytes; it does NOT isolate the hash function, because the two suites
// also differ in their binary ID, which is mixed into the HKDF info. The
// hash-function-from-suite requirement is proven falsifiably by the independent
// SHA-256 known-answer vector in `test_hkdf_derivation_process`.
#[test]
fn test_kdf_algorithm_is_selected_from_suite() {
    let pdk = sample_key();
    let message_id = [7u8; 16];

    let identity = derive_keys(&message_id, &pdk, suite(NO_KDF_AES_256), false).unwrap();
    let hkdf_sha256 = derive_keys(&message_id, &pdk, suite(HKDF_SHA256_AES_256), false).unwrap();
    let hkdf_sha384 =
        derive_keys(&message_id, &pdk, suite(HKDF_SHA384_AES_256_SIG), false).unwrap();

    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=Identity suite returns the key verbatim while the HKDF suite transforms it, proving the KDF follows the suite
    //# The algorithm used to derive a data key from the plaintext data key MUST be
    //# the [key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm) included in the
    //# [algorithm suite](../framework/algorithm-suites.md) defined above.
    assert_eq!(identity.data_key.as_slice(), pdk.as_slice());
    assert_ne!(hkdf_sha256.data_key.as_slice(), pdk.as_slice());

    // Supporting regression check: a different HKDF suite produces different bytes.
    // This does not isolate the hash function (the suites also differ in binary ID,
    // which is part of the HKDF info); the hash-from-suite requirement is proven by
    // the known-answer vector in `test_hkdf_derivation_process`.
    assert_ne!(hkdf_sha256.data_key.as_slice(), hkdf_sha384.data_key.as_slice());
}

// Expected V1 HKDF-SHA256 derived key for the inputs in the test below, computed
// independently with OpenSSL (RFC 5869 HKDF) so this is a known-answer test rather
// than a self-consistency check.
//
// Reproduce (copy-pasteable, OpenSSL 3.0+):
//
//   openssl kdf -keylen 32 -binary \
//     -kdfopt digest:SHA256 \
//     -kdfopt hexkey:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f \
//     -kdfopt hexsalt:0000000000000000000000000000000000000000000000000000000000000000 \
//     -kdfopt hexinfo:017801010101010101010101010101010101 \
//     HKDF | xxd -p -c 64
//
//   where:
//     hexkey  = sample_key()       = bytes 0x00..0x1f
//     hexsalt = 32 zero bytes       = salt defaults to hash length, all zero
//     hexinfo = 0178 || 0x01 * 16   = suite binary id || message_id ([1u8; 16])
const EXPECTED_HKDF_SHA256_DERIVED_KEY: [u8; 32] = [
    0x46, 0x91, 0x38, 0xc3, 0x42, 0xa4, 0xb0, 0xc7, 0x48, 0x76, 0x02, 0xf7, 0xdc, 0x96, 0x7d, 0xe5,
    0x44, 0xb5, 0x0a, 0xb2, 0xba, 0xda, 0x90, 0xdc, 0x32, 0xa8, 0x03, 0xa5, 0xaf, 0x7c, 0xdb, 0x35,
];

// The end-to-end interop test vectors exercise this same derivation against
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
    //# the derivation process used MUST be the process described in [HKDF Encryption Key](./key-derivation.md#hkdf-encryption-key).
    //
    //= spec/client-apis/key-derivation.md#hkdf-encryption-key
    //= type=test
    //= reason=The expected vector was computed with HKDF-SHA-256; no other hash would produce these bytes, proving the hash is taken from the suite
    //# - The hash function MUST be specified by the [algorithm suite key derivation settings](#algorithm-suites-encryption-key-derivation-settings).
    //
    //= spec/client-apis/key-derivation.md#hkdf-encryption-key
    //= type=test
    //= reason=The expected vector was computed with all-zero salt of hash length, no other salt would produce these bytes
    //# - If there is no salt length defined for the [algorithm suite encryption key derivation commitment setting](#algorithm-suites-encryption-key-derivation-settings),
    //# the the salt MUST be a byte sequence of 0 as long as the hash length in bytes.
    //
    //= spec/client-apis/key-derivation.md#hkdf-encryption-key
    //= type=test
    //= reason=The expected vector was computed with info = 0x0178 || 0x01*16 (binary_id || message_id); no other info would produce these bytes
    //# - If [key commitment](#key-commitment) for the [algorithm suite encryption key derivation setting](#algorithm-suites-encryption-key-derivation-settings) is False,
    //# the the input info MUST be a concatenation of the [algorithm suite ID](#algorithm-suite-id)
    //# followed by the [message ID](../data-format/message-header.md#message-id).
    assert_eq!(a1.data_key.as_slice(), &EXPECTED_HKDF_SHA256_DERIVED_KEY);

    //= spec/client-apis/key-derivation.md#hkdf-encryption-key
    //= type=test
    //# - The length of the output keying material MUST equal the [encryption key length](#encryption-key-length)
    //# specified by the [algorithm suite encryption settings](#algorithm-suites-encryption-settings).
    assert_eq!(a1.data_key.len(), AES_256_KEY_LEN);

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

// V2 HKDF (suite 0x0478, AES-256-GCM-HKDF-SHA-512-COMMIT-KEY): the encryption key
// MUST be HKDF-Expand(HKDF-Extract(salt=message_id, ikm=pdk), info=binary_id||"DERIVEKEY", L=32).
// We compute the same chain with the underlying primitives directly and compare —
// any drift in salt, ikm, info, or output length would cause a mismatch.
#[test]
fn test_v2_hkdf_derivation_process() {
    let alg_suite = suite(HKDF_SHA512_COMMIT_AES_256);
    let pdk = sample_key();
    let message_id = [0x42u8; V2_MESSAGE_ID_LEN];

    let derived = derive_keys(&message_id, &pdk, alg_suite, false).unwrap();

    // Compute the expected encryption key using the same primitives the SDK uses.
    let prk = hkdf_extract(alg_suite.kdf_digest(), &message_id, &pdk);
    let info = [&alg_suite.binary_id[..], KEY_LABEL];
    let mut expected_encrypt_key = vec![0u8; AES_256_KEY_LEN];
    hkdf_expand(&prk, &info, &mut expected_encrypt_key).expect("hkdf_expand");

    //= spec/client-apis/key-derivation.md#hkdf-encryption-key
    //= type=test
    //= reason=Re-running HKDF-Extract+Expand with salt=message_id and info=binary_id||DERIVEKEY produces the same encryption key, proving the V2 path follows the spec
    //# If an algorithm suite uses HKDF to derive the encryption key
    //# the AWS Encryption SDK MUST use HKDF with the following specifics:
    assert_eq!(derived.data_key.as_slice(), expected_encrypt_key.as_slice());

    let other_message_id = [0x99u8; V2_MESSAGE_ID_LEN];
    let other_prk = hkdf_extract(alg_suite.kdf_digest(), &other_message_id, &pdk);
    let mut wrong_salt_key = vec![0u8; AES_256_KEY_LEN];
    hkdf_expand(&other_prk, &info, &mut wrong_salt_key).expect("hkdf_expand");
    // Wrong-salt differential: regression-only coverage. The duvet citation for
    // "salt MUST be the message ID" lives on the cross-message-id test below
    // (`test_message_id_binds_derived_keys_v2`), which proves the same property
    // through the public API rather than by re-running the underlying primitive.
    assert_ne!(derived.data_key.as_slice(), wrong_salt_key.as_slice());

    let wrong_info = [&alg_suite.binary_id[..], b"WRONGLABEL"];
    let mut wrong_info_key = vec![0u8; AES_256_KEY_LEN];
    hkdf_expand(&prk, &wrong_info, &mut wrong_info_key).expect("hkdf_expand");
    //= spec/client-apis/key-derivation.md#hkdf-encryption-key
    //= type=test
    //= reason=The preceding assert_eq! pins info=binary_id||DERIVEKEY against a recomputation with the underlying HKDF primitive; this assert_ne! falsifies any other info label
    //# - If [key commitment](#key-commitment) for the [algorithm suite encryption key derivation setting](#algorithm-suites-encryption-key-derivation-settings) is True,
    //# then the input info MUST be a concatenation of the [algorithm suite ID](#algorithm-suite-id) followed by the string `DERIVEKEY` as UTF8 encoded bytes.
    assert_ne!(derived.data_key.as_slice(), wrong_info_key.as_slice());

    //= spec/client-apis/key-derivation.md#hkdf-encryption-key
    //= type=test
    //# - The length of the output keying material MUST equal the [encryption key length](#encryption-key-length)
    //# specified by the [algorithm suite encryption settings](#algorithm-suites-encryption-settings).
    assert_eq!(derived.data_key.len(), AES_256_KEY_LEN);
}

// V2 HKDF commit key derivation (suite 0x0478): the commit key MUST be
// HKDF-Expand(HKDF-Extract(salt=message_id, ikm=pdk), info="COMMITKEY", L=32).
// Using the SAME PRK as the encryption key proves the "extract once, expand twice"
// property holds and that the commit-side info is exactly "COMMITKEY".
#[test]
fn test_v2_hkdf_commit_key_derivation_process() {
    let alg_suite = suite(HKDF_SHA512_COMMIT_AES_256);
    let pdk = sample_key();
    let message_id = [0x42u8; V2_MESSAGE_ID_LEN];

    let derived = derive_keys(&message_id, &pdk, alg_suite, false).unwrap();
    let commit_key = derived
        .commitment_key
        .as_ref()
        .expect("V2 committing suite produces a commitment key");

    // Compute the expected commit key using the same primitives the SDK uses.
    let prk = hkdf_extract(alg_suite.kdf_digest(), &message_id, &pdk);
    let mut expected_commit_key = vec![0u8; AES_256_KEY_LEN];
    hkdf_expand(&prk, &[COMMIT_LABEL], &mut expected_commit_key).expect("hkdf_expand");

    //= spec/client-apis/key-derivation.md#hkdf-commit-key
    //= type=test
    //= reason=Re-running HKDF-Extract+Expand with salt=message_id and info=COMMITKEY produces the same commit key, proving the commit derivation follows the spec
    //# If an algorithm suite uses HKDF to derive the commitment key
    //# the AWS Encryption SDK MUST use HKDF with the following specifics:
    assert_eq!(commit_key.as_slice(), expected_commit_key.as_slice());

    let mut wrong_info_commit_key = vec![0u8; AES_256_KEY_LEN];
    hkdf_expand(&prk, &[b"WRONGLABEL"], &mut wrong_info_commit_key).expect("hkdf_expand");
    //= spec/client-apis/key-derivation.md#hkdf-commit-key
    //= type=test
    //= reason=The preceding assert_eq! pins info=COMMITKEY against a recomputation with the underlying HKDF primitive; this assert_ne! falsifies any other commit-key info label
    //# - The input info MUST the string `COMMITKEY` as UTF8 encoded bytes by the algorithm suite commitment settings.
    assert_ne!(commit_key.as_slice(), wrong_info_commit_key.as_slice());

    //= spec/client-apis/key-derivation.md#hkdf-commit-key
    //= type=test
    //# - The length of the output keying material MUST equal the [algorithm suite data length](#algorithm-suite-data-length)
    //# specified by the [supported algorithm suites](#supported-algorithm-suites).
    assert_eq!(commit_key.len(), AES_256_KEY_LEN);
}

// The V2 IKM-length precondition rejects a plaintext data key whose length differs
// from the suite's KDF input length. Covers encryption-key R1.4 and commit-key R2.4
// (both share the same length check at runtime). Calls `derive_key_v2` directly so
// the wrapper's debug_assert (which guards the same invariant in debug builds) does
// not preempt the runtime length check we are exercising.
#[test]
fn test_kdf_input_length_validation() {
    let alg_suite = suite(HKDF_SHA512_COMMIT_AES_256);
    let message_id = [0x42u8; V2_MESSAGE_ID_LEN];

    // Suite expects 32-byte PDK; pass 31 bytes.
    let too_short_pdk = vec![0u8; AES_256_KEY_LEN - 1];
    let result = derive_key_v2(&message_id, &too_short_pdk, alg_suite);

    //= spec/client-apis/key-derivation.md#hkdf-encryption-key
    //= type=test
    //# - The length of the input keying material MUST equal the [key derivation input length](#key-derivation-input-length)
    //# specified by the [algorithm suite encryption key derivation settings](#algorithm-suites-encryption-key-derivation-settings).
    //
    //= spec/client-apis/key-derivation.md#hkdf-commit-key
    //= type=test
    //# - The length of the input keying material MUST equal the [key derivation input length](#key-derivation-input-length)
    //# specified by the algorithm suite commit key derivation setting.
    let err = result.expect_err("wrong-length PDK must be rejected");
    assert_eq!(
        err.kind,
        ErrorKind::ValidationError,
        "wrong-length PDK must yield ErrorKind::ValidationError, got: {err:?}"
    );
    assert!(
        err.to_string().contains("does not match KDF input key length"),
        "error must name the KDF input length mismatch, got: {err}"
    );
}

// Companion to the test above. The spec is strict equality, so both sides of the
// boundary must be rejected — a length check that only fires on `<` (or only on
// `>`) would silently accept the other side. The duvet citation for the equality
// requirement lives on the under-length test above; this test is regression-only
// coverage of the over-length boundary.
#[test]
fn test_kdf_input_length_validation_too_long() {
    let alg_suite = suite(HKDF_SHA512_COMMIT_AES_256);
    let message_id = [0x42u8; V2_MESSAGE_ID_LEN];

    // Suite expects 32-byte PDK; pass 33 bytes.
    let too_long_pdk = vec![0u8; AES_256_KEY_LEN + 1];
    let result = derive_key_v2(&message_id, &too_long_pdk, alg_suite);

    let err = result.expect_err("over-length PDK must be rejected");
    assert_eq!(
        err.kind,
        ErrorKind::ValidationError,
        "over-length PDK must yield ErrorKind::ValidationError, got: {err:?}"
    );
    assert!(
        err.to_string().contains("does not match KDF input key length"),
        "error must name the KDF input length mismatch, got: {err}"
    );
}

// The message ID is mixed into the V2 derivation as the HKDF salt, so changing
// only the message ID (with all other inputs held equal) must yield a different
// encryption key AND a different commitment key. Falsifies any path that ignores
// the message ID at extract time.
#[test]
fn test_message_id_binds_derived_keys_v2() {
    let alg_suite = suite(HKDF_SHA512_COMMIT_AES_256);
    let pdk = sample_key();
    let message_id_a = [0x01u8; V2_MESSAGE_ID_LEN];
    let message_id_b = [0x02u8; V2_MESSAGE_ID_LEN];

    let a = derive_keys(&message_id_a, &pdk, alg_suite, false).unwrap();
    let b = derive_keys(&message_id_b, &pdk, alg_suite, false).unwrap();

    //= spec/client-apis/key-derivation.md#hkdf-encryption-key
    //= type=test
    //= reason=Swapping only the message ID changes the encryption key — proves the message ID is an input to derivation (its salt role is shown by the recomputation test above)
    //# - If salt length is defined for the [algorithm suite encryption key derivation commitment setting](#algorithm-suites-encryption-key-derivation-settings),
    //# the salt MUST be the [message ID](../data-format/message-header.md#message-id) with a length equal to the salt length.
    assert_ne!(a.data_key.as_slice(), b.data_key.as_slice());

    let a_commit = a.commitment_key.as_ref().unwrap();
    let b_commit = b.commitment_key.as_ref().unwrap();
    //= spec/client-apis/key-derivation.md#hkdf-commit-key
    //= type=test
    //= reason=Swapping only the 32-byte message ID changes the commit key — proves the message ID is an input to commit derivation
    //# - The salt MUST be the [message ID](../data-format/message-header.md#message-id) with a length of 256 bits.
    assert_ne!(a_commit.as_slice(), b_commit.as_slice());
}

// Helper: the kdf hash on the V2 suites equals the commitment hash by construction.
// We rely on this to compute expected vectors via the underlying primitive directly.
trait KdfDigestExt {
    fn kdf_digest(&self) -> aws_mpl_legacy::primitives::DigestAlg;
}

impl KdfDigestExt for AlgorithmSuite {
    fn kdf_digest(&self) -> aws_mpl_legacy::primitives::DigestAlg {
        match self.kdf {
            aws_mpl_legacy::suites::DerivationAlgorithm::Hkdf(h) => h.hmac,
            other => panic!("expected HKDF-based suite, got {other:?}"),
        }
    }
}

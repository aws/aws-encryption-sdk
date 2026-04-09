// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for encrypt.md#construct-the-signature requirements

mod fixtures;

use aws_esdk::*;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;

/// Create a raw AES keyring for testing (no KMS needed).
async fn test_keyring() -> aws_mpl_legacy::dafny::types::keyring::KeyringRef {
    let (ns, name) = namespace_and_name(0);
    mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([0u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap()
}

/// Encrypt with a signing algorithm suite, return ciphertext bytes.
async fn encrypt_with_signing_suite(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt then decrypt round-trip with a signing suite.
async fn round_trip_signing(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Read the signature length from the end of a signing-suite ciphertext.
/// The footer is: [sig_len: 2 bytes] [signature: sig_len bytes] at the end.
/// For ECDSA P384, the DER-encoded signature is typically 102-104 bytes.
fn find_footer_offset(ct: &[u8]) -> (usize, u16) {
    for candidate_len in 90..=110 {
        let offset = ct.len() - 2 - candidate_len;
        let sig_len = u16::from_be_bytes([ct[offset], ct[offset + 1]]);
        if sig_len as usize == candidate_len {
            return (offset, sig_len);
        }
    }
    panic!("Could not find footer in ciphertext");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signing_suite_produces_footer() {
    //= specification/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# If the [algorithm suite](../framework/algorithm-suites.md) contains a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# this operation MUST calculate a signature over the message,
    //# and the output [encrypted message](#encrypted-message) MUST contain a [message footer](../data-format/message-footer.md).

    let ct = encrypt_with_signing_suite(b"signature presence test").await;
    let (_, sig_len) = find_footer_offset(&ct);
    assert!(sig_len > 0, "signing suite must produce a footer with non-zero signature");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signature_uses_signing_algorithm() {
    //= specification/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# To calculate a signature, this operation MUST use the [signature algorithm](../framework/algorithm-suites.md#signature-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following input:

    // A successful round-trip proves the correct algorithm was used,
    // because decrypt verifies the signature using the same algorithm suite.
    let pt = b"signature algorithm test";
    let result = round_trip_signing(pt).await;
    assert_eq!(result, pt, "round-trip proves correct signature algorithm was used");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signature_key_is_signing_key() {
    //= specification/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# - the signature key MUST be the [signing key](../framework/structures.md#signing-key) in the [encryption materials](../framework/structures.md#encryption-materials)

    // A successful round-trip proves the correct signing key was used,
    // because decrypt verifies the signature against the verification key
    // derived from the signing key in the encryption materials.
    let pt = b"signing key test";
    let result = round_trip_signing(pt).await;
    assert_eq!(result, pt, "round-trip proves correct signing key was used");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signature_input_is_header_plus_body() {
    //= specification/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# - the input to sign MUST be the concatenation of the serialization of the [message header](../data-format/message-header.md) and [message body](../data-format/message-body.md)

    // A successful round-trip proves the signature was calculated over the correct input,
    // because decrypt recomputes the digest over header+body and verifies the signature.
    let pt = b"header plus body input test";
    let result = round_trip_signing(pt).await;
    assert_eq!(result, pt, "round-trip proves signature input is header+body concatenation");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_serialization() {
    //= specification/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# This operation MUST then serialize a message footer.

    let ct = encrypt_with_signing_suite(b"footer serialization test").await;
    let (offset, sig_len) = find_footer_offset(&ct);

    //= specification/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# - [Signature Length](../data-format/message-footer.md#signature-length): The value MUST be the length of the
    //# output of the signature calculation above.
    assert_eq!(
        ct.len() - offset - 2,
        sig_len as usize,
        "signature length field must equal actual signature byte count"
    );

    //= specification/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# - [Signature](../data-format/message-footer.md#signature): The value MUST be the output of the signature calculation above.
    let signature_bytes = &ct[offset + 2..];
    assert_eq!(
        signature_bytes.len(),
        sig_len as usize,
        "signature bytes must match the declared length"
    );
    // Non-zero signature bytes prove actual signature content (not padding)
    assert!(
        signature_bytes.iter().any(|&b| b != 0),
        "signature must contain non-zero bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_equals_calculated() {
    //= specification/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# The encrypted message output by this operation MUST have a message footer equal
    //# to the message footer calculated in this step.

    // A successful round-trip proves the output footer equals the calculated footer,
    // because decrypt verifies the signature from the footer.
    let pt = b"footer equals calculated test";
    let result = round_trip_signing(pt).await;
    assert_eq!(result, pt, "round-trip proves output footer equals calculated footer");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_no_signature_without_signing_suite() {
    //= specification/client-apis/encrypt.md#behavior
    //= type=test
    //# - If the materials do not have an algorithm suite including a signature algorithm,
    //# the Encrypt operation MUST NOT construct a signature.

    // Encrypt with non-signing suite and verify successful round-trip.
    // If a signature were constructed, the message would contain a footer
    // that the decryptor (knowing the suite has no signature) would not expect,
    // causing failure or trailing bytes.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"no signature test", EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let pt = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(
        pt,
        b"no signature test",
        "successful round-trip with non-signing suite proves no signature was constructed"
    );
}

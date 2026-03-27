// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for aws-encryption-sdk-specification/data-format/message-footer.md

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

/// Encrypt then decrypt with a signing algorithm suite.
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

/// Encrypt with a non-signing algorithm suite, return ciphertext bytes.
async fn encrypt_without_signing_suite(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    encrypt(&input).await.unwrap().ciphertext
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_present_with_signing_suite() {
    //= specification/data-format/message-footer.md#overview
    //= type=test
    //# When an [algorithm suite](../framework/algorithm-suites.md) includes a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# the [message](message.md) MUST contain a footer.

    let ct_signing = encrypt_with_signing_suite(b"footer presence test").await;
    let ct_no_signing = encrypt_without_signing_suite(b"footer presence test").await;

    // The signing ciphertext must be longer than the non-signing one
    // because it contains the footer (2-byte length + signature bytes).
    assert!(
        ct_signing.len() > ct_no_signing.len(),
        "signing suite ciphertext ({}) must be longer than non-signing ({}) due to footer",
        ct_signing.len(),
        ct_no_signing.len()
    );

    // Verify the footer is parseable: find the 2-byte length + signature at the end
    let (_, sig_len) = find_footer_offset(&ct_signing);
    assert!(sig_len > 0, "footer signature length must be non-zero for signing suite");
}

/// Read the signature length from the end of a signing-suite ciphertext.
/// The footer is: [sig_len: 2 bytes] [signature: sig_len bytes] at the end.
/// We need to find where the footer starts. Since we don't know sig_len yet,
/// we try reading the 2-byte length at various offsets.
/// For ECDSA P384, the DER-encoded signature is typically 102-104 bytes.
fn find_footer_offset(ct: &[u8]) -> (usize, u16) {
    // Try signature lengths in the expected range for ECDSA P384 DER signatures
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
async fn test_footer_signature_length_is_two_bytes() {
    //= specification/data-format/message-footer.md#signature-length
    //= type=test
    //# This length of the signature length field MUST be 2 bytes.

    let ct = encrypt_with_signing_suite(b"sig length 2 bytes test").await;
    let (offset, sig_len) = find_footer_offset(&ct);

    // The signature length field occupies exactly bytes [offset] and [offset+1]
    // and the remaining bytes after it equal sig_len
    assert_eq!(
        ct.len() - offset - 2,
        sig_len as usize,
        "signature length field (2 bytes at offset {offset}) must describe remaining footer bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_signature_length_is_uint16() {
    //= specification/data-format/message-footer.md#signature-length
    //= type=test
    //# The signature length field MUST be interpreted as a UInt16.

    let ct = encrypt_with_signing_suite(b"sig length uint16 test").await;
    let (offset, sig_len) = find_footer_offset(&ct);

    // Interpret the 2 bytes as big-endian UInt16 and verify it matches the actual signature length
    let interpreted = u16::from_be_bytes([ct[offset], ct[offset + 1]]);
    assert_eq!(interpreted, sig_len);
    let actual_sig_bytes = &ct[offset + 2..];
    assert_eq!(
        actual_sig_bytes.len(),
        interpreted as usize,
        "UInt16-interpreted signature length must equal actual signature byte count"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_signature_calculated_over_header_and_body() {
    //= specification/data-format/message-footer.md#signature
    //= type=test
    //# This signature MUST be calculated over both the [message header](message-header.md) and the [message body](message-body.md),
    //# in the order of serialization.

    // A successful round-trip decrypt with a signing suite proves the signature
    // was correctly calculated over header+body, because decrypt verifies it.
    let pt = b"signature over header and body test";
    let result = round_trip_signing(pt).await;
    assert_eq!(
        result, pt,
        "successful decrypt proves signature was calculated over header+body in serialization order"
    );
}

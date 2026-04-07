// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/client-apis/decrypt.md#verify-the-signature

mod fixtures;

use aws_esdk::*;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;

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

/// Find the footer offset in a signing-suite ciphertext.
/// Footer format: [sig_len: 2 bytes big-endian] [signature: sig_len bytes] at the end.
fn find_footer_offset(ct: &[u8]) -> usize {
    for candidate_len in 90..=110 {
        let offset = ct.len() - 2 - candidate_len;
        let sig_len = u16::from_be_bytes([ct[offset], ct[offset + 1]]);
        if sig_len as usize == candidate_len {
            return offset;
        }
    }
    panic!("Could not find footer in ciphertext");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_verify_signature_round_trip_signing_suite() {
    //= specification/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //# If the algorithm suite has a signature algorithm,
    //# the Decrypt operation MUST verify the message footer using the specified signature algorithm.

    //= specification/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //# After deserializing the body, the Decrypt operation MUST deserialize the next encrypted message bytes
    //# as the [message footer](../data-format/message-footer.md).

    //= specification/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //# The order for message footer deserialization MUST conform to the [Message Footer](../data-format/message-footer.md) specification.

    //= specification/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# The order for message footer serialization MUST conform to the [Message Footer](../data-format/message-footer.md) specification.

    //= specification/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //# Once the message footer is deserialized, the Decrypt operation MUST use the
    //# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm)
    //# from the [algorithm suite](../framework/algorithm-suites.md) in the decryption materials to
    //# verify the encrypted message, with the following inputs:

    //= specification/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //# - The verification key MUST be the [verification key](../framework/structures.md#verification-key)
    //# in the decryption materials.

    //= specification/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //# - The input to verify MUST be the concatenation of the serialization of the
    //# [message header](../data-format/message-header.md) and [message body](../data-format/message-body.md).

    let keyring = test_keyring().await;
    let plaintext = b"signature verification round trip test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(
        result.plaintext, plaintext,
        "successful round-trip with signing suite proves footer deserialized, signature algorithm used, verification key used, and input was header+body"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_verify_signature_fails_on_tampered_footer() {
    //= specification/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //# If this verification is not successful, this operation MUST immediately halt and fail.

    let keyring = test_keyring().await;
    let plaintext = b"tamper footer test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Tamper with a signature byte in the footer (last byte of ciphertext is part of the signature)
    let footer_offset = find_footer_offset(&ct);
    ct[footer_offset + 3] ^= 0xFF;

    let dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    assert!(
        result.is_err(),
        "decrypt must fail when footer signature bytes are tampered"
    );
}

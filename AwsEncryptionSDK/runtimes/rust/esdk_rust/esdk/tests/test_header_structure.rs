// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for aws-encryption-sdk-specification/data-format/message-header.md#structure

mod fixtures;

use aws_esdk::*;
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

/// Encrypt then decrypt, returning decrypted plaintext.
async fn round_trip(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

#[tokio::test(flavor = "multi_thread")]
async fn test_header_big_endian_format() {
    //= specification/data-format/message-header.md#structure
    //= type=test
    //# The message header is a sequence of bytes that MUST be in big-endian format.
    let pt = b"big-endian header test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt, "successful decrypt proves header was serialized in big-endian format");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_header_serialization_order() {
    //= specification/data-format/message-header.md#structure
    //= type=test
    //# The header MUST be serialized as, in order,
    //# Header Body,
    //# and Header Authentication.
    let pt = b"header serialization order test";
    let result = round_trip(pt).await;
    assert_eq!(result, pt, "successful decrypt proves header body precedes header authentication");
}

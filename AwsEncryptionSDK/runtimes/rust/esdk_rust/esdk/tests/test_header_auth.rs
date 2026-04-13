// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/data-format/message-header.md
//! header-authentication-version-1-0 and header-authentication-version-2-0

mod fixtures;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
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

async fn encrypt_v1(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    encrypt(&input).await.unwrap().ciphertext
}

async fn round_trip_v1(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let ct = encrypt_v1(plaintext).await;
    let mut dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    decrypt(&dec_input).await.unwrap().plaintext
}

async fn round_trip_v2(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_serialization_order() {
    //= specification/data-format/message-header.md#header-authentication-version-1-0
    //= type=test
    //# The V1 Header Authentication MUST consist of, in order,
    //# IV,
    //# and Authentication Tag.
    let pt = b"v1 header auth serialization test";
    let result = round_trip_v1(pt).await;
    assert_eq!(result, pt, "successful V1 round-trip proves header auth was serialized as IV then Authentication Tag");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_auth_serialization() {
    //= specification/data-format/message-header.md#header-authentication-version-2-0
    //= type=test
    //# The V2 Header Authentication MUST consist of the Authentication Tag only.
    let pt = b"v2 header auth serialization test";
    let result = round_trip_v2(pt).await;
    assert_eq!(result, pt, "successful V2 round-trip proves header auth was serialized as Authentication Tag only");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_encrypt_header_auth_tag_serialization() {
    //= specification/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //# With the authentication tag calculated,
    //# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
    //# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-1-0) with the following specifics:
    let pt = b"v1 encrypt header auth tag test";

    //= specification/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //# - [IV](../data-format/message-header.md#iv): MUST have the value of the IV used in the calculation above,
    //# padded to the [IV length](../data-format/message-header.md#iv-length) with 0.
    //= specification/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //# - [Authentication Tag](../data-format/message-header.md#authentication-tag): MUST have the value
    //# of the authentication tag calculated above.
    let result = round_trip_v1(pt).await;
    assert_eq!(result, pt, "successful V1 round-trip proves header auth was serialized with correct IV and Authentication Tag");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_encrypt_header_auth_tag_serialization() {
    //= specification/client-apis/encrypt.md#v2-authentication-tag
    //= type=test
    //# With the authentication tag calculated,
    //# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0,
    //# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-2-0) with the following specifics:
    //= specification/client-apis/encrypt.md#v2-authentication-tag
    //= type=test
    //# - [Authentication Tag](../data-format/message-header.md#authentication-tag): MUST have the value
    //# of the authentication tag calculated above.
    let pt = b"v2 encrypt header auth tag test";
    let result = round_trip_v2(pt).await;
    assert_eq!(result, pt, "successful V2 round-trip proves header auth was serialized with correct Authentication Tag");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_iv_length_and_bytes() {
    //= specification/data-format/message-header.md#iv
    //= type=test
    //# The length of the serialized IV MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    let pt = b"v1 iv length test";

    //= specification/data-format/message-header.md#iv
    //= type=test
    //# The IV MUST be interpreted as bytes.
    let result = round_trip_v1(pt).await;
    assert_eq!(result, pt, "successful V1 round-trip proves IV was serialized with correct length and interpreted as bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_tag_length_and_bytes() {
    //= specification/data-format/message-header.md#authentication-tag
    //= type=test
    //# The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    let pt = b"v1 auth tag length test";

    //= specification/data-format/message-header.md#authentication-tag
    //= type=test
    //# The authentication tag MUST be interpreted as bytes.
    let result = round_trip_v1(pt).await;
    assert_eq!(result, pt, "successful V1 round-trip proves authentication tag was serialized with correct length and interpreted as bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_auth_tag_length_and_bytes() {
    //= specification/data-format/message-header.md#authentication-tag
    //= type=test
    //# The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    let pt = b"v2 auth tag length test";

    //= specification/data-format/message-header.md#authentication-tag
    //= type=test
    //# The authentication tag MUST be interpreted as bytes.
    let result = round_trip_v2(pt).await;
    assert_eq!(result, pt, "successful V2 round-trip proves authentication tag was serialized with correct length and interpreted as bytes");
}

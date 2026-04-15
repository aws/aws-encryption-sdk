// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod fixtures;
use aws_esdk::*;

// THIS IS AN INCORRECTLY SERIALIZED CIPHERTEXT PRODUCED BY
// THE ESDK .NET V4.0.0
// This message was constructed with a zeroed 32 byte AES Key
// using v4.0.0 of the Encryption SDK for .NET which incorrectly
// serializes the message header making messages unreadable in
// other implementations and making this version unable to
// read other implementation's messages.

const ESDK_NET_V400_MESSAGE: &[u8] = &[
    2, 5, 120, 238, 5, 239, 107, 129, 136, 211, 103, 75, 18, 140, 11, 74, 26, 191, 92, 27, 202,
    170, 33, 28, 9, 117, 252, 29, 29, 92, 213, 21, 231, 172, 234, 0, 95, 0, 1, 0, 21, 97, 119, 115,
    45, 99, 114, 121, 112, 116, 111, 45, 112, 117, 98, 108, 105, 99, 45, 107, 101, 121, 0, 68, 65,
    119, 102, 117, 103, 90, 99, 107, 57, 116, 100, 53, 104, 78, 108, 49, 78, 108, 75, 111, 47, 104,
    105, 114, 53, 85, 47, 48, 81, 109, 98, 73, 111, 107, 79, 72, 81, 87, 97, 72, 83, 43, 115, 117,
    119, 75, 73, 77, 82, 76, 99, 67, 80, 49, 54, 55, 56, 43, 49, 82, 75, 49, 48, 82, 101, 119, 61,
    61, 0, 1, 0, 21, 83, 111, 109, 101, 32, 109, 97, 110, 97, 103, 101, 100, 32, 114, 97, 119, 32,
    107, 101, 121, 115, 0, 47, 77, 121, 32, 50, 53, 54, 45, 98, 105, 116, 32, 65, 69, 83, 32, 119,
    114, 97, 112, 112, 105, 110, 103, 32, 107, 101, 121, 0, 0, 0, 128, 0, 0, 0, 12, 229, 254, 197,
    205, 110, 124, 222, 48, 217, 121, 252, 11, 0, 48, 64, 60, 232, 232, 76, 229, 15, 118, 224, 152,
    79, 93, 113, 166, 255, 172, 255, 148, 185, 150, 195, 179, 78, 52, 186, 38, 216, 48, 118, 45,
    113, 204, 71, 102, 116, 148, 199, 109, 178, 19, 2, 203, 150, 201, 65, 32, 199, 180, 2, 0, 0,
    16, 0, 67, 72, 208, 112, 230, 137, 188, 187, 0, 28, 183, 198, 192, 45, 248, 108, 2, 129, 34,
    42, 59, 155, 70, 117, 182, 216, 239, 27, 210, 78, 62, 104, 181, 247, 141, 50, 133, 42, 72, 200,
    185, 57, 20, 49, 193, 240, 171, 140, 255, 255, 255, 255, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 1, 0, 0, 0, 15, 67, 37, 106, 11, 15, 23, 78, 239, 208, 185, 4, 36, 182, 9, 63, 62, 83,
    97, 42, 250, 252, 185, 165, 14, 182, 231, 83, 176, 227, 191, 92, 0, 103, 48, 101, 2, 49, 0,
    193, 152, 7, 169, 197, 137, 244, 88, 9, 1, 6, 56, 96, 13, 220, 201, 56, 16, 50, 68, 70, 36,
    174, 38, 14, 241, 207, 11, 139, 154, 166, 224, 191, 20, 12, 175, 56, 117, 183, 120, 119, 228,
    173, 130, 71, 110, 211, 189, 2, 48, 99, 98, 250, 36, 53, 182, 2, 204, 198, 55, 150, 51, 159,
    101, 231, 34, 42, 30, 57, 204, 88, 114, 138, 94, 12, 79, 52, 71, 178, 34, 61, 246, 55, 163,
    145, 95, 80, 61, 85, 143, 32, 0, 98, 20, 88, 251, 204, 5,
];

#[tokio::test(flavor = "multi_thread")]
async fn test_net_retry_flag() {
    let key_namespace = "Some managed raw keys";
    let key_name = "My 256-bit AES wrapping key";
    let expected_message: &[u8] = &[
        84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 46,
    ];

    let raw_aes_keyring = mpl()
        .create_raw_aes_keyring()
        .key_namespace(key_namespace)
        .key_name(key_name)
        .wrapping_key(aws_smithy_types::Blob::new([0; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();

    // Attempt to decrypt the v4.0.0 message without the retry flag and expect
    // decryption to fail
    let ec = EncryptionContext::new();
    let mut input = DecryptInput::with_legacy_keyring(ESDK_NET_V400_MESSAGE, ec, raw_aes_keyring);
    input.net_v4_retry_policy = NetV400RetryPolicy::ForbidRetry;

    let expect_failure = decrypt(&input).await;
    if expect_failure.is_ok() {
        panic!("Expected decryption to fail without retry flag");
    }

    // Decrypt v4.0.0 message with the default configuration which is to retry
    // and expect decryption to pass
    input.net_v4_retry_policy = NetV400RetryPolicy::AllowRetry;
    let expect_success = decrypt(&input).await.unwrap();
    assert!(expect_success.plaintext == expected_message);
}

#[test]
fn test_encrypt_input_default_commitment_policy() {
    //= specification/client-apis/client.md#initialization
    //= type=test
    //# If no [commitment policy](#commitment-policy) is provided the default MUST be [REQUIRE_ENCRYPT_REQUIRE_DECRYPT](../framework/algorithm-suites.md#require_encrypt_require_decrypt).
    let input = EncryptInput::default();
    assert_eq!(
        input.commitment_policy,
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt
    );
}

#[test]
fn test_decrypt_input_default_commitment_policy() {
    //= specification/client-apis/client.md#initialization
    //= type=test
    //# If no [commitment policy](#commitment-policy) is provided the default MUST be [REQUIRE_ENCRYPT_REQUIRE_DECRYPT](../framework/algorithm-suites.md#require_encrypt_require_decrypt).
    let input = DecryptInput::default();
    assert_eq!(
        input.commitment_policy,
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt
    );
}

#[test]
fn test_encrypt_input_default_max_edks_is_none() {
    //= specification/client-apis/client.md#initialization
    //= type=test
    //# If no [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys) is provided
    //# the default MUST result in no limit on the number of encrypted data keys (aside from the limit imposed by the [message format](../format/message-header.md)).
    let input = EncryptInput::default();
    assert!(input.max_encrypted_data_keys.is_none());
}

#[test]
fn test_decrypt_input_default_max_edks_is_none() {
    //= specification/client-apis/client.md#initialization
    //= type=test
    //# If no [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys) is provided
    //# the default MUST result in no limit on the number of encrypted data keys (aside from the limit imposed by the [message format](../format/message-header.md)).
    let input = DecryptInput::default();
    assert!(input.max_encrypted_data_keys.is_none());
}

#[test]
fn test_encrypt_input_custom_commitment_policy() {
    //= specification/client-apis/client.md#initialization
    //= type=test
    //# - On client initialization,
    //# the caller MUST have the option to provide a [commitment policy](#commitment-policy).
    let mut input = EncryptInput::default();
    input.commitment_policy =
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    assert_eq!(
        input.commitment_policy,
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt
    );
}

#[test]
fn test_encrypt_input_custom_max_edks() {
    //= specification/client-apis/client.md#initialization
    //= type=test
    //# - On client initialization,
    //# the caller MUST have the option to provide a [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys).
    let mut input = EncryptInput::default();
    input.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(5).unwrap());
    assert_eq!(input.max_encrypted_data_keys.unwrap().get(), 5);
}

#[test]
fn test_encrypt_input_accepts_plaintext() {
    //= specification/client-apis/encrypt.md#input
    //= type=test
    //# - The input to the Encrypt operation MUST accept a required [plaintext](#plaintext) argument.

    //= specification/client-apis/encrypt.md#plaintext
    //= type=test
    //# This MUST be a sequence of bytes.
    let plaintext = b"hello world";
    let mut input = EncryptInput::default();
    input.plaintext = plaintext;
    assert_eq!(input.plaintext, plaintext);
}

#[test]
fn test_encrypt_input_accepts_cmm_and_keyring() {
    //= specification/client-apis/encrypt.md#input
    //= type=test
    //# - The input to the Encrypt operation MUST accept a [cryptographic Materials Manager (CMM)](../framework/cmm-interface.md) and a [keyring](../framework/keyring-interface.md) argument.
    let input = EncryptInput::default();
    // source field exists and is Option<MaterialSource>, accepting CMM or keyring
    assert!(input.source.is_none());
}

#[test]
fn test_encrypt_input_accepts_optional_algorithm_suite() {
    //= specification/client-apis/encrypt.md#input
    //= type=test
    //# - The input to the Encrypt operation MUST accept an optional [Algorithm Suite](#algorithm-suite) argument.
    let input = EncryptInput::default();
    assert!(input.algorithm_suite_id.is_none());
}

#[test]
fn test_encrypt_input_accepts_optional_encryption_context() {
    //= specification/client-apis/encrypt.md#input
    //= type=test
    //# - The input to the Encrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument.
    let input = EncryptInput::default();
    assert!(input.encryption_context.is_empty());
}

#[test]
fn test_encrypt_input_accepts_optional_frame_length() {
    //= specification/client-apis/encrypt.md#input
    //= type=test
    //# - The input to the Encrypt operation MUST accept an optional [Frame Length](#frame-length) argument.
    let mut input = EncryptInput::default();
    input.frame_length = FrameLength::new(8192).unwrap();
    assert_eq!(input.frame_length.0.get(), 8192);
}

#[test]
fn test_decrypt_input_accepts_encrypted_message() {
    //= specification/client-apis/decrypt.md#input
    //= type=test
    //# - The input to the Decrypt operation MUST accept a required [Encrypted Message](#encrypted-message) argument.

    //= specification/client-apis/decrypt.md#encrypted-message
    //= type=test
    //# The input encrypted message MUST be a sequence of bytes in the
    //# [message format](../data-format/message.md) specified by the AWS Encryption SDK.
    let ciphertext = b"fake ciphertext";
    let mut input = DecryptInput::default();
    input.ciphertext = ciphertext;
    assert_eq!(input.ciphertext, ciphertext);
}

#[test]
fn test_decrypt_input_accepts_cmm_and_keyring() {
    //= specification/client-apis/decrypt.md#input
    //= type=test
    //# - The input to the Decrypt operation MUST accept an optional [Cryptographic Materials Manager (CMM)](../framework/cmm-interface.md) argument.
    let input = DecryptInput::default();
    // source field exists and is Option<MaterialSource>, accepting CMM or keyring
    assert!(input.source.is_none());
}

#[test]
fn test_decrypt_input_accepts_optional_encryption_context() {
    //= specification/client-apis/decrypt.md#input
    //= type=test
    //# - The input to the Decrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument.
    let input = DecryptInput::default();
    assert!(input.encryption_context.is_empty());
}

#[test]
fn test_frame_length_rejects_zero() {
    //= specification/client-apis/encrypt.md#frame-length
    //= type=test
    //# This value MUST be greater than 0 and MUST NOT exceed the value 2^32 - 1.
    assert!(FrameLength::new(0).is_err());
}

#[test]
fn test_frame_length_accepts_max_u32() {
    //= specification/client-apis/encrypt.md#frame-length
    //= type=test
    //# This value MUST be greater than 0 and MUST NOT exceed the value 2^32 - 1.
    let fl = FrameLength::new(u32::MAX).unwrap();
    assert_eq!(fl.0.get(), u32::MAX);
}

#[test]
fn test_frame_length_default_is_4096() {
    //= specification/client-apis/encrypt.md#frame-length
    //= type=test
    //# This value MUST default to 4096 bytes.
    assert_eq!(FrameLength::default().0.get(), 4096);
}

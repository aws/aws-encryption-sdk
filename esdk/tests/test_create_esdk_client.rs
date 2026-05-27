// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use aws_esdk::{
    decrypt, encrypt, mpl, DecryptInput, EncryptInput, EncryptionContext, ErrorKind, FrameLength,
    NetV400RetryPolicy,
};

mod fixtures;
mod test_helpers;

use test_helpers::test_keyring;

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

// ESDK .NET v4.0.0 incorrectly serialized headers. This test proves our
// retry logic: decrypting the v4.0.0 message fails without the retry flag
// (CryptographicError from header auth mismatch) and succeeds with it.
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

    let err = decrypt(&input).await.expect_err("Expected decryption to fail without retry flag");
    assert!(
        matches!(err.kind, ErrorKind::CryptographicError),
        "expected CryptographicError, got {:?}",
        err.kind
    );

    // Decrypt v4.0.0 message with the default configuration which is to retry
    // and expect decryption to pass
    input.net_v4_retry_policy = NetV400RetryPolicy::AllowRetry;
    let expect_success = decrypt(&input).await.unwrap();
    assert_eq!(expect_success.plaintext, expected_message);
}

#[test]
fn test_default_commitment_policy() {
    //= spec/client-apis/client.md#initialization
    //= type=test
    //= reason=EncryptInput::default() and DecryptInput::default() both return RequireEncryptRequireDecrypt
    //# If no [commitment policy](#commitment-policy) is provided the default MUST be [REQUIRE_ENCRYPT_REQUIRE_DECRYPT](../framework/algorithm-suites.md#require_encrypt_require_decrypt).
    let expected =
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    assert_eq!(
        EncryptInput::default().commitment_policy,
        expected,
        "EncryptInput default commitment policy"
    );
    assert_eq!(
        DecryptInput::default().commitment_policy,
        expected,
        "DecryptInput default commitment policy"
    );
}

#[test]
fn test_default_max_encrypted_data_keys_is_none() {
    //= spec/client-apis/client.md#initialization
    //= type=test
    //# If no [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys) is provided
    //# the default MUST result in no limit on the number of encrypted data keys (aside from the limit imposed by the [message format](../format/message-header.md)).
    assert!(
        EncryptInput::default().max_encrypted_data_keys.is_none(),
        "EncryptInput default max_encrypted_data_keys must be None (no limit)"
    );
    assert!(
        DecryptInput::default().max_encrypted_data_keys.is_none(),
        "DecryptInput default max_encrypted_data_keys must be None (no limit)"
    );
}

#[test]
fn test_frame_length_rejects_zero() {
    //= spec/client-apis/encrypt.md#frame-length
    //= type=test
    //# This value MUST be greater than 0 and MUST NOT exceed the value 2^32 - 1.
    let err = FrameLength::new(0).expect_err("FrameLength::new(0) must fail");
    assert!(
        matches!(err.kind, ErrorKind::ValidationError),
        "expected ValidationError, got {:?}",
        err.kind
    );
}

#[test]
fn test_frame_length_accepts_max_u32() {
    //= spec/data-format/message-body.md#framed-data
    //= type=test
    //= reason=FrameLength::new(u32::MAX) succeeds, proving the implementation accepts the maximum allowed value
    //# - The total bytes allowed in a single frame MUST be less than or equal to `2^32 - 1`.
    let fl = FrameLength::new(u32::MAX).unwrap();
    assert_eq!(fl.0.get(), u32::MAX);
}

#[test]
fn test_frame_length_default_is_4096() {
    //= spec/client-apis/encrypt.md#frame-length
    //= type=test
    //# This value MUST default to 4096 bytes.
    assert_eq!(FrameLength::default().0.get(), 4096);
}

// Round-trip a fully-populated EncryptInput and DecryptInput through
// encrypt() and decrypt(). Each populated optional field is proven
// "accepted" by the operation returning Ok(_).
#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt_accepts_all_optional_inputs() {
    let keyring = test_keyring().await;
    let mut ec = EncryptionContext::new();
    ec.insert("greet".to_string(), "hello".to_string());
    let plaintext = b"round-trip plaintext";

    let mut encrypt_input =
        EncryptInput::with_legacy_keyring(plaintext, ec.clone(), keyring.clone());

    //= spec/client-apis/encrypt.md#input
    //= type=test
    //= reason=encrypt_input.algorithm_suite_id is set to Some(non-default V1 suite); encrypt() succeeds
    //# - Encrypt operation input MUST accept an optional [Algorithm Suite](#algorithm-suite) argument.
    encrypt_input.algorithm_suite_id =
        Some(aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);

    //= spec/client-apis/client.md#initialization
    //= type=test
    //= reason=encrypt_input.commitment_policy is set non-default; encrypt() honors it
    //# - On client initialization,
    //# the caller MUST have the option to provide a [commitment policy](#commitment-policy).
    //
    //= spec/client-apis/client.md#commitment-policy
    //= type=test
    //= reason=commitment_policy is an EsdkCommitmentPolicy from aws_mpl_legacy; encrypt() honors it
    //# The AWS Encryption SDK MUST use the ESDK [commitment policies](../framework/commitment-policy.md) defined in the Material Providers Library.
    encrypt_input.commitment_policy =
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;

    //= spec/client-apis/encrypt.md#input
    //= type=test
    //= reason=encrypt_input.frame_length is set to a non-default 1024; encrypt() succeeds
    //# - Encrypt operation input MUST accept an optional [Frame Length](#frame-length) argument.
    encrypt_input.frame_length = FrameLength::new(1024).unwrap();

    //= spec/client-apis/client.md#initialization
    //= type=test
    //= reason=encrypt_input.max_encrypted_data_keys is set to Some(5); encrypt() succeeds
    //# - On client initialization,
    //# the caller MUST have the option to provide a [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys).
    encrypt_input.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(5).unwrap());

    //= spec/client-apis/encrypt.md#input
    //= type=test
    //= reason=encrypt() returns Ok with plaintext populated, proving the required plaintext argument is accepted
    //# - Encrypt operation input MUST accept a required [plaintext](#plaintext) argument.
    //
    //= spec/client-apis/encrypt.md#input
    //= type=test
    //= reason=encrypt_input.encryption_context is populated; encrypt() succeeds
    //# - Encrypt operation input MUST accept an optional [Encryption Context](#encryption-context) argument.
    //
    //= spec/client-apis/encrypt.md#input
    //= type=test
    //= reason=encrypt_input.source is Some(MaterialSource::LegacyKeyring(...)); encrypt() succeeds
    //# - Encrypt operation input MUST accept an optional [cryptographic Materials Manager (CMM)](../framework/cmm-interface.md) argument.
    //
    //= spec/client-apis/encrypt.md#input
    //= type=test
    //= reason=encrypt_input.source is Some(MaterialSource::LegacyKeyring(...)); encrypt() succeeds
    //# - Encrypt operation input MUST accept an optional [keyring](../framework/keyring-interface.md) argument.
    let encrypt_output = encrypt(&encrypt_input)
        .await
        .expect("encrypt must accept a fully-populated EncryptInput");

    //= spec/client-apis/decrypt.md#input
    //= type=test
    //= reason=decrypt_input.ciphertext is the &[u8] produced by encrypt(); decrypt() succeeds
    //# - Decrypt operation input MUST accept a required [Encrypted Message](#encrypted-message) argument.
    //
    //= spec/client-apis/decrypt.md#encrypted-message
    //= type=test
    //= reason=ciphertext is the byte-sequence ESDK message; decrypt() accepts and round-trips it
    //# The input encrypted message MUST be a sequence of bytes in the
    //# [message format](../data-format/message.md) specified by the AWS Encryption SDK.
    //
    //= spec/client-apis/decrypt.md#input
    //= type=test
    //= reason=decrypt_input.encryption_context is populated; decrypt() succeeds
    //# - Decrypt operation input MUST accept an optional [Encryption Context](#encryption-context) argument.
    //
    //= spec/client-apis/decrypt.md#input
    //= type=test
    //= reason=decrypt_input.source is Some(MaterialSource::LegacyKeyring(...)); decrypt() succeeds
    //# - Decrypt operation input MUST accept an optional [Cryptographic Materials Manager (CMM)](../framework/cmm-interface.md) argument.
    //
    //= spec/client-apis/decrypt.md#input
    //= type=test
    //= reason=decrypt_input.source is Some(MaterialSource::LegacyKeyring(...)); decrypt() succeeds
    //# - Decrypt operation input MUST accept an optional [Keyring](../framework/keyring-interface.md) argument.
    let mut decrypt_input =
        DecryptInput::with_legacy_keyring(&encrypt_output.ciphertext, ec.clone(), keyring);
    decrypt_input.commitment_policy =
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    decrypt_input.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(5).unwrap());

    let decrypt_output = decrypt(&decrypt_input)
        .await
        .expect("decrypt must accept a fully-populated DecryptInput");

    assert_eq!(
        decrypt_output.plaintext, plaintext,
        "round-trip plaintext must match"
    );
    assert_eq!(
        decrypt_output
            .encryption_context
            .get("greet")
            .map(String::as_str),
        Some("hello"),
        "encrypt/decrypt must honor the encryption context input"
    );
}

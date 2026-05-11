// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for message-header.md: supported-versions, supported-types, version, type, content-type

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_version_values() {
    //= spec/data-format/message-header.md#supported-versions
    //= type=test
    //# The supported versions MUST be:
    //
    //= spec/data-format/message-header.md#supported-versions
    //= type=test
    //# - Hex value `01` MUST be version 1.0
    let ct_v1 = encrypt_v1(b"v1 version test").await;
    assert_eq!(ct_v1[0], 0x01, "V1 ciphertext must start with version byte 0x01");

    //= spec/data-format/message-header.md#supported-versions
    //= type=test
    //# - Hex value `02` MUST be version 2.0
    let ct_v2 = encrypt_v2(b"v2 version test").await;
    assert_eq!(ct_v2[0], 0x02, "V2 ciphertext must start with version byte 0x02");

    // Round-trip proves both version formats produce valid, decryptable messages.
    let pt_v1 = round_trip_v1(b"v1 version test", EncryptionContext::new()).await;
    assert_eq!(pt_v1, b"v1 version test");
    let pt_v2 = round_trip_v2(b"v2 version test", EncryptionContext::new()).await;
    assert_eq!(pt_v2, b"v2 version test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_type_customer_aed_value() {
    let ct = encrypt_v1(b"type test").await;
    //= spec/data-format/message-header.md#type
    //= type=test
    //# The length of the serialized type field MUST be 1 byte.
    //
    //= spec/data-format/message-header.md#supported-types
    //= type=test
    //# - `80` MUST be Customer Authenticated Encrypted Data
    assert_eq!(
        ct[1], 0x80,
        "V1 ciphertext must have type byte 0x80 at offset 1"
    );

    // Round-trip proves the type field is correctly processed.
    let pt = round_trip_v1(b"type test", EncryptionContext::new()).await;
    assert_eq!(pt, b"type test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_content_type_framed_value() {
    let keyring = test_keyring().await;
    for version in VERSIONS {
        let ct = encrypt_with_version(b"content type framed", version, keyring.clone()).await;
        let offset = match version {
            Version::V1 => parse_v1_trailing_offsets(&ct).0,
            Version::V2 => content_type_offset_v2(&ct),
        };
        //= spec/data-format/message-header.md#content-type
        //= type=test
        //# The length of the serialized content type field MUST be 1 byte.
        assert_eq!(ct[offset], 0x02, "{version:?}: framed content type must be 0x02");
    }

    // Round-trip proves framed messages are processed correctly.
    let pt = round_trip(b"content type framed").await;
    assert_eq!(pt, b"content type framed");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_content_type_nonframed_value() {
    // The ESDK cannot produce nonframed messages, so we use external test vectors.
    for version in VERSIONS {
        let ct = external_nonframed_ct(version);
        let offset = match version {
            Version::V1 => parse_v1_trailing_offsets(ct).0,
            Version::V2 => content_type_offset_v2(ct),
        };
        //= spec/data-format/message-header.md#supported-content-types
        //= type=test
        //# The supported content types MUST be:
        assert_eq!(ct[offset], 0x01, "{version:?}: nonframed content type must be 0x01");

        // Round-trip decrypt proves the nonframed message is processed correctly end-to-end.
        let pt = decrypt_external_nonframed_vector(version).await;
        assert!(!pt.is_empty(), "{version:?}: nonframed decrypt must produce plaintext");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_content_type_invalid_value_rejected() {
    let keyring = test_keyring().await;
    for version in VERSIONS {
        let mut ct = encrypt_with_version(b"test", version, keyring.clone()).await;
        let offset = match version {
            Version::V1 => parse_v1_trailing_offsets(&ct).0,
            Version::V2 => content_type_offset_v2(&ct),
        };
        ct[offset] = 0x00;

        let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone());
        //= spec/data-format/message-header.md#content-type
        //= type=test
        //= reason=Tampering a single byte at the content type offset changes the parsed value, proving the field is 1 byte.
        //# The length of the serialized content type field MUST be 1 byte.
        //
        //= spec/data-format/message-header.md#supported-content-types
        //= type=test
        //= reason=Negative test asserts that there are no other supported content types
        //# The supported content types MUST be:
        let err = decrypt(&dec_input).await.unwrap_err();
        assert!(matches!(err.kind, ErrorKind::SerializationError), "{version:?}: expected SerializationError, got {:?}", err.kind);
        let msg = format!("{err}");
        assert!(
            msg.contains("content type"),
            "{version:?}: invalid content type 0x00 must be rejected with content type error, got: {msg}"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_base64_input_rejected() {
    let keyring = test_keyring().await;
    let mut ct = encrypt_v2(b"base64 test").await;
    ct[0] = 0x41;

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let err = decrypt(&dec_input).await.unwrap_err();
    assert!(matches!(err.kind, ErrorKind::SerializationError), "expected SerializationError, got {:?}", err.kind);
    let msg = format!("{err}");
    //= spec/client-apis/decrypt.md#encrypted-message-format
    //= type=test
    //# To make diagnosing this mistake easier, implementations SHOULD detect the first two bytes of the Base64 encoding of any supported message [versions](../data-format/message-header.md#version)
    //# and [types](../data-format/message-header.md#type)
    //# and fail with a more specific error message.
    assert!(
        msg.contains("Base64"),
        "Base64-like input must produce a Base64-specific error, got: {msg}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unsupported_type_rejected_v1() {
    let keyring = test_keyring().await;
    let mut ct = encrypt_v1(b"type test").await;
    ct[1] = 0x00;

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    //= spec/data-format/message-header.md#supported-types
    //= type=test
    //= reason=Tampering the type byte at offset 1 to an unsupported value proves the field is validated and only the specified types are accepted.
    //# The supported types MUST be:
    let err = decrypt(&dec_input).await.unwrap_err();
    assert!(matches!(err.kind, ErrorKind::SerializationError), "expected SerializationError, got {:?}", err.kind);
    let msg = format!("{err}");
    assert!(
        msg.contains("message type"),
        "unsupported type 0x00 must be rejected with type error, got: {msg}"
    );
}

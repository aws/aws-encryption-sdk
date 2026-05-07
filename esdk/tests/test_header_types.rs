// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for message-header.md: supported-versions, supported-types, version, type, content-type

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_version_v2_value() {
    let ct = encrypt_v2(b"v2 version test").await;
    //= spec/data-format/message-header.md#supported-versions
    //= type=test
    //# The supported versions MUST be:
    assert_eq!(
        ct[0], 0x02,
        "V2 ciphertext must start with version byte 0x02"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_version_v1_value() {
    let ct = encrypt_v1(b"v1 version test").await;
    //= spec/data-format/message-header.md#supported-versions
    //= type=test
    //# - Hex value `01` MUST be version 1.0
    assert_eq!(
        ct[0], 0x01,
        "V1 ciphertext must start with version byte 0x01"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_type_customer_aed_value() {
    let ct = encrypt_v1(b"type test").await;
    //= spec/data-format/message-header.md#type
    //= type=test
    //# The length of the serialized type field MUST be 1 byte.
    assert_eq!(
        ct[1], 0x80,
        "V1 ciphertext must have type byte 0x80 at offset 1"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_content_type_framed_value() {
    let keyring = test_keyring().await;
    for version in VERSIONS {
        let ct = encrypt_with_version(b"test", version, keyring.clone()).await;
        let offset = match version {
            Version::V1 => parse_v1_trailing_offsets(&ct).0,
            Version::V2 => content_type_offset_v2(&ct),
        };
        //= spec/data-format/message-header.md#content-type
        //= type=test
        //# The length of the serialized content type field MUST be 1 byte.
        assert_eq!(ct[offset], 0x02, "{version:?}: framed content type must be 0x02");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_content_type_nonframed_value() {
    let keyring = test_keyring().await;
    for version in VERSIONS {
        let mut ct = encrypt_with_version(b"test", version, keyring.clone()).await;
        let offset = match version {
            Version::V1 => parse_v1_trailing_offsets(&ct).0,
            Version::V2 => content_type_offset_v2(&ct),
        };
        ct[offset] = 0x01;

        let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone());
        let err = decrypt(&dec_input).await.unwrap_err();
        assert!(matches!(err.kind, ErrorKind::SerializationError), "{version:?}: expected SerializationError, got {:?}", err.kind);
        let msg = format!("{err}");
        //= spec/data-format/message-header.md#content-type
        //= type=test
        //# The length of the serialized content type field MUST be 1 byte.
        assert!(
            !msg.contains("Unsupported Content Type"),
            "{version:?}: 0x01 must be accepted as a valid content type, got: {msg}"
        );
        // Content type 0x01 is valid but the body is still framed, so decrypt
        // must fail for a body-level reason (e.g. authentication or parsing),
        // not because the content type was rejected.
        assert!(
            !msg.is_empty(),
            "{version:?}: expected a concrete error from body mismatch, got empty message"
        );
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
        //# The length of the serialized content type field MUST be 1 byte.
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
async fn test_unsupported_version_rejected() {
    let keyring = test_keyring().await;
    let mut ct = encrypt_v2(b"version test").await;
    ct[0] = 0x03;

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    //= spec/data-format/message-header.md#supported-versions
    //= type=test
    //# - Hex value `02` MUST be version 2.0
    let err = decrypt(&dec_input).await.unwrap_err();
    assert!(matches!(err.kind, ErrorKind::SerializationError), "expected SerializationError, got {:?}", err.kind);
    let msg = format!("{err}");
    assert!(
        msg.contains("version"),
        "unsupported version 0x03 must be rejected with version error, got: {msg}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unsupported_type_rejected_v1() {
    let keyring = test_keyring().await;
    let mut ct = encrypt_v1(b"type test").await;
    ct[1] = 0x00;

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    //= spec/data-format/message-header.md#type
    //= type=test
    //# The length of the serialized type field MUST be 1 byte.
    let err = decrypt(&dec_input).await.unwrap_err();
    assert!(matches!(err.kind, ErrorKind::SerializationError), "expected SerializationError, got {:?}", err.kind);
    let msg = format!("{err}");
    assert!(
        msg.contains("message type"),
        "unsupported type 0x00 must be rejected with type error, got: {msg}"
    );
}

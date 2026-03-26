// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/data-format/message-header.md#supported-content-types

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

/// Find the content type byte offset in a V2 ciphertext.
/// V2 header: Version(1) + AlgSuiteID(2) + MessageID(32) + AAD(variable) + EDKs(variable) + ContentType(1).
fn content_type_offset(ct: &[u8]) -> usize {
    let mut pos: usize = 1 + 2 + 32; // skip Version, AlgSuiteID, MessageID

    // AAD: 2-byte length, then if non-zero: 2-byte kv_count + aad_byte_len bytes
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += 2 + aad_byte_len;
    }

    // EDKs: 2-byte count, then for each: provider_id(2+len) + provider_info(2+len) + ciphertext(2+len)
    let edk_count = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    for _ in 0..edk_count {
        let pid_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pid_len;
        let pinfo_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pinfo_len;
        let ct_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + ct_len;
    }

    pos
}

#[tokio::test(flavor = "multi_thread")]
async fn test_content_type_framed_value() {
    //= specification/data-format/message-header.md#supported-content-types
    //= type=test
    //# The supported content types MUST be:
    //= specification/data-format/message-header.md#supported-content-types
    //= type=test
    //# - `02` for [Framed](message-body.md#framed-data)
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(b"test", EncryptionContext::new(), keyring);
    let ct = encrypt(&input).await.unwrap().ciphertext;

    let offset = content_type_offset(&ct);
    assert_eq!(ct[offset], 0x02, "framed content type must be 0x02");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_content_type_nonframed_value() {
    //= specification/data-format/message-header.md#supported-content-types
    //= type=test
    //# - `01` for [Non-Framed](message-body.md#non-framed-data)
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(b"test", EncryptionContext::new(), keyring.clone());
    let mut ct = encrypt(&input).await.unwrap().ciphertext;

    // Set content type byte to 0x01 (NonFramed). Decryption will fail because
    // the body format doesn't match, but the error must NOT be "Unsupported Content Type" —
    // proving 0x01 is a supported content type value.
    let offset = content_type_offset(&ct);
    ct[offset] = 0x01;

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let err = decrypt(&dec_input).await.unwrap_err();
    let msg = format!("{err}");
    assert!(
        !msg.contains("Unsupported Content Type"),
        "0x01 must be accepted as a valid content type, got: {msg}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_content_type_invalid_value_rejected() {
    //= specification/data-format/message-header.md#content-type
    //= type=test
    //# The value (hex) of this field MUST be a value that exists in the following table:
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(b"test", EncryptionContext::new(), keyring.clone());
    let mut ct = encrypt(&input).await.unwrap().ciphertext;

    let offset = content_type_offset(&ct);
    ct[offset] = 0x00; // invalid content type

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    assert!(decrypt(&dec_input).await.is_err(), "invalid content type 0x00 must be rejected");
}

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for encrypt.md#authentication-tag

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

/// V1 header auth layout: IV(12) || Tag(16) immediately after the header body.
/// Returns (header_body_end, iv_offset, tag_offset).
fn v1_header_auth_offsets(ct: &[u8]) -> (usize, usize, usize) {
    assert_eq!(ct[0], 0x01, "must be V1 message");
    let (_, _, _, frame_length_offset) = parse_v1_trailing_offsets(ct);
    let header_body_end = frame_length_offset + 4;
    (header_body_end, header_body_end, header_body_end + IV_LEN)
}

/// V2 header auth layout: Tag(16) immediately after the header body (no IV on wire).
/// Returns (header_body_end, tag_offset).
fn v2_header_auth_offsets(ct: &[u8]) -> (usize, usize) {
    let fields = parse_v2_header_field_offsets(ct);
    let header_body_end = fields.last().expect("must have header fields").2;
    (header_body_end, header_body_end)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_has_iv_then_tag() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# The encrypted message output by the Encrypt operation MUST have a message header equal
    //# to the message header calculated in this step.
    let pt = b"raw byte v1 auth tag";
    let ct = encrypt_v1(pt).await;
    let (_, iv_offset, tag_offset) = v1_header_auth_offsets(&ct);

    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# - The IV MUST have a value of 0.
    let iv_bytes = &ct[iv_offset..iv_offset + IV_LEN];
    assert_eq!(iv_bytes.len(), IV_LEN, "V1 header auth IV must be {IV_LEN} bytes");
    assert!(
        iv_bytes.iter().all(|&b| b == 0),
        "V1 header auth IV must be all zeros (IV=0 padded to IV length)"
    );

    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# After serializing the message header body,
    //# this operation MUST calculate an [authentication tag](../data-format/message-header.md#authentication-tag)
    //# over the message header body.
    let tag_bytes = &ct[tag_offset..tag_offset + TAG_LEN];
    assert_eq!(tag_bytes.len(), TAG_LEN, "V1 header auth tag must be {TAG_LEN} bytes");
    assert!(
        tag_bytes.iter().any(|&b| b != 0),
        "V1 header auth tag must not be all zeros"
    );

    // Round-trip cross-check: independent decrypt validates the same tag.
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V1 round-trip with auth tag verification");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_auth_has_tag_only() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# After serializing the message header body,
    //# this operation MUST calculate an [authentication tag](../data-format/message-header.md#authentication-tag)
    //# over the message header body.
    let pt = b"raw byte v2 auth tag";
    let ct = encrypt_v2(pt).await;
    let (header_body_end, tag_offset) = v2_header_auth_offsets(&ct);

    let tag_bytes = &ct[tag_offset..tag_offset + TAG_LEN];
    assert_eq!(tag_bytes.len(), TAG_LEN, "V2 header auth tag must be {TAG_LEN} bytes");
    assert!(
        tag_bytes.iter().any(|&b| b != 0),
        "V2 header auth tag must not be all zeros"
    );

    // V2 has NO IV on the wire — body starts right after the tag.
    let after_tag = header_body_end + TAG_LEN;
    let next_4 = u32::from_be_bytes([
        ct[after_tag],
        ct[after_tag + 1],
        ct[after_tag + 2],
        ct[after_tag + 3],
    ]);
    assert!(
        next_4 == 1 || next_4 == 0xFFFF_FFFF,
        "V2: bytes after auth tag must be body start (seq=1 or endframe), got {next_4:#010X}"
    );

    // Round-trip cross-check: independent decrypt validates the same tag.
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V2 round-trip with auth tag verification");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_present_with_required_ec_v1() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=with a non-empty required EC the encryption-context-to-only-authenticate is non-empty, exercising the AAD-concatenation and required-EC-filtering code paths; the tag bytes prove an AEAD output was produced for this scenario and the round-trip proves encrypt and decrypt agree on the AAD construction
    //# - The AAD MUST be the concatenation of the serialized [message header body](../data-format/message-header.md#header-body)
    //# and the serialization of encryption context to only authenticate.
    let ec = std::collections::HashMap::from([
        ("test-public-key".to_string(), "testval".to_string()),
        ("user-key".to_string(), "user-val".to_string()),
    ]);
    let pt = b"v1 required ec with auth tag";
    let ct = encrypt_v1_with_ec(pt, ec.clone()).await;

    // Raw-byte: auth tag is present at the expected offset with the expected length.
    let (_, _, tag_offset) = v1_header_auth_offsets(&ct);
    let tag_bytes = &ct[tag_offset..tag_offset + TAG_LEN];
    assert_eq!(tag_bytes.len(), TAG_LEN, "V1 auth tag must be {TAG_LEN} bytes");
    assert!(
        tag_bytes.iter().any(|&b| b != 0),
        "V1 auth tag must be a real AEAD output (not all zeros)"
    );

    // Round-trip cross-check: decrypt with the same EC validates the AAD construction.
    let result = round_trip_v1(pt, ec).await;
    assert_eq!(result, pt, "V1 round-trip with non-empty EC");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_present_with_required_ec_v2() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=with a non-empty required EC the encryption-context-to-only-authenticate is non-empty, exercising the AAD-concatenation and required-EC-filtering code paths; the tag bytes prove an AEAD output was produced for this scenario and the round-trip proves encrypt and decrypt agree on the AAD construction
    //# - The AAD MUST be the concatenation of the serialized [message header body](../data-format/message-header.md#header-body)
    //# and the serialization of encryption context to only authenticate.
    let ec = std::collections::HashMap::from([
        ("test-public-key".to_string(), "testval".to_string()),
        ("user-key".to_string(), "user-val".to_string()),
    ]);
    let pt = b"v2 required ec with auth tag";
    let ct = encrypt_no_sign_with_ec(pt, ec.clone(), Version::V2).await;

    // Raw-byte: auth tag is present at the expected offset with the expected length.
    let (_, tag_offset) = v2_header_auth_offsets(&ct);
    let tag_bytes = &ct[tag_offset..tag_offset + TAG_LEN];
    assert_eq!(tag_bytes.len(), TAG_LEN, "V2 auth tag must be {TAG_LEN} bytes");
    assert!(
        tag_bytes.iter().any(|&b| b != 0),
        "V2 auth tag must be a real AEAD output (not all zeros)"
    );

    // Round-trip cross-check: decrypt with the same EC validates the AAD construction.
    let result = round_trip_v2(pt, ec).await;
    assert_eq!(result, pt, "V2 round-trip with non-empty EC");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_tampered_header_fails_decrypt_v1() {
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# If this tag verification fails, this operation MUST immediately halt and fail.
    let mut ct = encrypt_v1(b"v1 tamper test").await;
    // Tamper with a byte in the header body area (after version byte).
    assert!(ct.len() > 10, "ciphertext must be long enough to tamper");
    ct[5] ^= 0xFF;
    let keyring = test_keyring().await;
    let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy =
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let err = decrypt(&dec_input)
        .await
        .expect_err("V1 tampered header must fail decryption");
    assert!(
        matches!(err.kind, ErrorKind::CryptographicError),
        "V1: expected CryptographicError, got {:?}",
        err.kind
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_tampered_header_fails_decrypt_v2() {
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# If this tag verification fails, this operation MUST immediately halt and fail.
    let mut ct = encrypt_v2(b"v2 tamper test").await;
    // Tamper with a byte in the header body area (after version byte).
    assert!(ct.len() > 10, "ciphertext must be long enough to tamper");
    ct[5] ^= 0xFF;
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let err = decrypt(&dec_input)
        .await
        .expect_err("V2 tampered header must fail decryption");
    assert!(
        matches!(err.kind, ErrorKind::ValidationError),
        "V2: expected ValidationError, got {:?}",
        err.kind
    );
}

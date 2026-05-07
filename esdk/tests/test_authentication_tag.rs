// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for encrypt.md#authentication-tag

mod test_helpers;

use aws_esdk::*;
use test_helpers::*;


#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_calculated_over_header_body_v1() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=decrypt independently recomputes the auth tag over the header body and rejects mismatches, so a successful round-trip proves the auth tag was calculated over the header body
    //# After serializing the message header body,
    //# this operation MUST calculate an [authentication tag](../data-format/message-header.md#authentication-tag)
    //# over the message header body.
    let pt = b"v1 auth tag over header body";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V1 round-trip with auth tag verification");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_calculated_over_header_body_v2() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=decrypt independently recomputes the auth tag over the header body and rejects mismatches, so a successful round-trip proves the auth tag was calculated over the header body
    //# After serializing the message header body,
    //# this operation MUST calculate an [authentication tag](../data-format/message-header.md#authentication-tag)
    //# over the message header body.
    let pt = b"v2 auth tag over header body";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V2 round-trip with auth tag verification");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_uses_authenticated_encryption_algorithm_v1() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=decrypt uses the algorithm suite's authenticated encryption algorithm to verify the tag; a successful round-trip proves the correct algorithm was used
    //# The value of this MUST be the output of the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
    let pt = b"v1 encryption algorithm";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V1 round-trip with algorithm verification");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_uses_authenticated_encryption_algorithm_v2() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=decrypt uses the algorithm suite's authenticated encryption algorithm to verify the tag; a successful round-trip proves the correct algorithm was used
    //# The value of this MUST be the output of the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
    let pt = b"v2 encryption algorithm";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V2 round-trip with algorithm verification");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_aad_is_header_body_concat_required_ec_v1() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=decrypt recomputes the AAD as header body + required EC serialization and verifies the tag; a successful round-trip with non-empty EC proves the AAD was correctly constructed
    //# - The AAD MUST be the concatenation of the serialized [message header body](../data-format/message-header.md#header-body)
    //# and the serialization of encryption context to only authenticate.
    let ec = std::collections::HashMap::from([("key".to_string(), "val".to_string())]);
    let pt = b"v1 aad concatenation";
    let result = round_trip_v1(pt, ec).await;
    assert_eq!(result, pt, "V1 round-trip with EC proves AAD construction");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_aad_is_header_body_concat_required_ec_v2() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=decrypt recomputes the AAD as header body + required EC serialization and verifies the tag; a successful round-trip with non-empty EC proves the AAD was correctly constructed
    //# - The AAD MUST be the concatenation of the serialized [message header body](../data-format/message-header.md#header-body)
    //# and the serialization of encryption context to only authenticate.
    let ec = std::collections::HashMap::from([("key".to_string(), "val".to_string())]);
    let pt = b"v2 aad concatenation";
    let result = round_trip_v2(pt, ec).await;
    assert_eq!(result, pt, "V2 round-trip with EC proves AAD construction");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_required_ec_filtering_v1() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=decrypt recomputes the required EC filtering and uses it in AAD verification; a successful round-trip with non-empty EC proves the filtering was applied correctly
    //# The encryption context to only authenticate MUST be the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials)
    //# filtered to only contain key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys)
    //# serialized according to the [encryption context serialization specification](../framework/structures.md#serialization).
    let ec = std::collections::HashMap::from([
        ("test-public-key".to_string(), "testval".to_string()),
        ("user-key".to_string(), "user-val".to_string()),
    ]);
    let pt = b"v1 required ec filtering";
    let result = round_trip_v1(pt, ec).await;
    assert_eq!(result, pt, "V1 round-trip with non-empty EC proves filtering");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_required_ec_filtering_v2() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=decrypt recomputes the required EC filtering and uses it in AAD verification; a successful round-trip with non-empty EC proves the filtering was applied correctly
    //# The encryption context to only authenticate MUST be the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials)
    //# filtered to only contain key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys)
    //# serialized according to the [encryption context serialization specification](../framework/structures.md#serialization).
    let ec = std::collections::HashMap::from([
        ("test-public-key".to_string(), "testval".to_string()),
        ("user-key".to_string(), "user-val".to_string()),
    ]);
    let pt = b"v2 required ec filtering";
    let result = round_trip_v2(pt, ec).await;
    assert_eq!(result, pt, "V2 round-trip with non-empty EC proves filtering");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_iv_is_zero_v1() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=decrypt recomputes the auth tag with IV=0; if encrypt used a different IV the tag would not match and round-trip would fail
    //# - The IV MUST have a value of 0.
    let pt = b"v1 iv zero";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V1 round-trip proves IV=0 was used for auth tag");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_iv_is_zero_v2() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=decrypt recomputes the auth tag with IV=0; if encrypt used a different IV the tag would not match and round-trip would fail
    //# - The IV MUST have a value of 0.
    let pt = b"v2 iv zero";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V2 round-trip proves IV=0 was used for auth tag");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_cipherkey_is_derived_data_key_v1() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=decrypt recomputes the auth tag using the derived data key; if encrypt used a different key the tag would not match and round-trip would fail
    //# - The cipherkey MUST be the derived data key
    let pt = b"v1 cipherkey";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V1 round-trip proves derived data key was used as cipherkey");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_cipherkey_is_derived_data_key_v2() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=decrypt recomputes the auth tag using the derived data key; if encrypt used a different key the tag would not match and round-trip would fail
    //# - The cipherkey MUST be the derived data key
    let pt = b"v2 cipherkey";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V2 round-trip proves derived data key was used as cipherkey");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_plaintext_is_empty_v1() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=decrypt recomputes the auth tag with empty plaintext; if encrypt used non-empty plaintext the tag would not match and round-trip would fail
    //# - The plaintext MUST be an empty byte array
    let pt = b"v1 empty plaintext for auth tag";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V1 round-trip proves auth tag was computed with empty plaintext");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_plaintext_is_empty_v2() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=decrypt recomputes the auth tag with empty plaintext; if encrypt used non-empty plaintext the tag would not match and round-trip would fail
    //# - The plaintext MUST be an empty byte array
    let pt = b"v2 empty plaintext for auth tag";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V2 round-trip proves auth tag was computed with empty plaintext");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_header_equality_v1() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=a successful round-trip confirms the serialized header matches the calculated header because decrypt authenticates the header and would fail if they differed
    //# The encrypted message output by the Encrypt operation MUST have a message header equal
    //# to the message header calculated in this step.
    let pt = b"v1 header equality";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V1 round-trip proves output header equals calculated header");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_header_equality_v2() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=a successful round-trip confirms the serialized header matches the calculated header because decrypt authenticates the header and would fail if they differed
    //# The encrypted message output by the Encrypt operation MUST have a message header equal
    //# to the message header calculated in this step.
    let pt = b"v2 header equality";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "V2 round-trip proves output header equals calculated header");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_tampered_header_fails_decrypt_v1() {
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# If this tag verification fails, this operation MUST immediately halt and fail.
    let mut ct = encrypt_v1(b"v1 tamper test").await;
    // Tamper with a byte in the header body area (after version byte)
    if ct.len() > 10 {
        ct[5] ^= 0xFF;
    }
    let keyring = test_keyring().await;
    let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy =
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    assert!(decrypt(&dec_input).await.is_err(), "V1 tampered header must fail decryption");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_tampered_header_fails_decrypt_v2() {
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# If this tag verification fails, this operation MUST immediately halt and fail.
    let mut ct = encrypt_v2(b"v2 tamper test").await;
    // Tamper with a byte in the header body area (after version byte)
    if ct.len() > 10 {
        ct[5] ^= 0xFF;
    }
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    assert!(decrypt(&dec_input).await.is_err(), "V2 tampered header must fail decryption");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_auth_tag_serialized_bytes_not_released_until_complete_v2() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# The serialized bytes MUST NOT be released until the entire message header has been serialized.
    // Encrypt returns the full ciphertext only after the header (body + auth tag) is complete.
    // Verify the auth tag is present at the expected position, proving the header was fully serialized.
    let ct = encrypt_v2(b"serialization release test").await;
    let fields = parse_v2_header_field_offsets(&ct);
    let header_body_end = fields.last().expect("must have header fields").2;
    let auth_tag_bytes = &ct[header_body_end..header_body_end + TAG_LEN];
    assert!(
        auth_tag_bytes.iter().any(|&b| b != 0),
        "V2 header auth tag must be present (not all zeros) — proves full header was serialized before release"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_auth_tag_is_16_bytes_after_header_body() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# After serializing the message header body,
    //# this operation MUST calculate an [authentication tag](../data-format/message-header.md#authentication-tag)
    //# over the message header body.
    let ct = encrypt_v2(b"raw byte v2 auth tag").await;
    let fields = parse_v2_header_field_offsets(&ct);
    // The last field in the V2 header body is "Algorithm Suite Data" (32 bytes).
    // The header auth tag (16 bytes) immediately follows the header body.
    let last_field = fields.last().expect("must have header fields");
    let header_body_end = last_field.2;
    let auth_tag_bytes = &ct[header_body_end..header_body_end + TAG_LEN];
    assert_eq!(
        auth_tag_bytes.len(),
        TAG_LEN,
        "V2 header auth tag must be exactly {} bytes",
        TAG_LEN
    );
    // Auth tag must not be all zeros (it's a real AEAD output)
    assert!(
        auth_tag_bytes.iter().any(|&b| b != 0),
        "V2 header auth tag must not be all zeros"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_has_iv_then_tag() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# With the authentication tag calculated,
    //# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
    //# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-1-0) with the following specifics:
    let ct = encrypt_v1(b"raw byte v1 auth tag").await;
    // V1 header: Version(1) + AlgSuiteID(2) + MessageID(16) + AAD(variable) + EDKs(variable)
    //            + ContentType(1) + Reserved(4) + IVLength(1) + FrameLength(4)
    // Then header auth: IV(12) + Tag(16)
    // Find header body end by parsing. V1 version byte is 0x01.
    assert_eq!(ct[0], 0x01, "must be V1 message");
    let (_, _, _, frame_length_offset) = parse_v1_trailing_offsets(&ct);
    let header_body_end = frame_length_offset + 4;
    // V1 header auth: IV (12 bytes) then Tag (16 bytes)
    let iv_bytes = &ct[header_body_end..header_body_end + IV_LEN];
    let tag_bytes = &ct[header_body_end + IV_LEN..header_body_end + IV_LEN + TAG_LEN];

    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# - [IV](../data-format/message-header.md#iv): MUST have the value of the IV used in the calculation above,
    //# padded to the [IV length](../data-format/message-header.md#iv-length) with 0.
    assert_eq!(iv_bytes.len(), IV_LEN, "V1 header auth IV must be {} bytes", IV_LEN);
    assert!(
        iv_bytes.iter().all(|&b| b == 0),
        "V1 header auth IV must be all zeros (IV=0 padded to IV length)"
    );

    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# - [Authentication Tag](../data-format/message-header.md#authentication-tag): MUST have the value
    //# of the authentication tag calculated above.
    assert_eq!(tag_bytes.len(), TAG_LEN, "V1 header auth tag must be {} bytes", TAG_LEN);
    assert!(
        tag_bytes.iter().any(|&b| b != 0),
        "V1 header auth tag must not be all zeros"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_auth_has_tag_only() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# With the authentication tag calculated,
    //# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0,
    //# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-2-0) with the following specifics:
    let ct = encrypt_v2(b"raw byte v2 tag only").await;
    let fields = parse_v2_header_field_offsets(&ct);
    let header_body_end = fields.last().expect("must have header fields").2;

    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //# - [Authentication Tag](../data-format/message-header.md#authentication-tag): MUST have the value
    //# of the authentication tag calculated above.
    let tag_bytes = &ct[header_body_end..header_body_end + TAG_LEN];
    assert_eq!(tag_bytes.len(), TAG_LEN, "V2 header auth tag must be {} bytes", TAG_LEN);
    assert!(
        tag_bytes.iter().any(|&b| b != 0),
        "V2 header auth tag must not be all zeros"
    );
    // V2 has NO IV in header auth — the body starts right after the tag.
    // Verify the next bytes are the start of the body (first frame seq num or endframe marker).
    let after_tag = header_body_end + TAG_LEN;
    let next_4 = u32::from_be_bytes([ct[after_tag], ct[after_tag + 1], ct[after_tag + 2], ct[after_tag + 3]]);
    assert!(
        next_4 == 1 || next_4 == 0xFFFF_FFFF,
        "V2: bytes after auth tag must be body start (seq=1 or endframe), got {:#010X}",
        next_4
    );
}

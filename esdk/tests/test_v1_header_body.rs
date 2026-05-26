// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/encrypt.md#v1-header

#[allow(clippy::duplicate_mod)]
mod fixtures;
mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

/// V1 header fixed-offset fields.
/// Layout: Version(1) | Type(1) | AlgSuiteID(2) | MessageID(16) | AAD(variable) | ...
struct V1FixedFields<'a> {
    /// Version byte at offset 0.
    version: u8,
    /// Type byte at offset 1.
    msg_type: u8,
    /// Algorithm Suite ID (big-endian u16) at offset 2..4.
    suite_id: u16,
    /// Message ID (16 bytes) at offset 4..20.
    message_id: &'a [u8],
}

impl<'a> V1FixedFields<'a> {
    /// Parse the fixed-offset fields from a V1 ciphertext.
    fn parse(ct: &'a [u8]) -> Self {
        Self {
            version: ct[0],
            msg_type: ct[1],
            suite_id: u16::from_be_bytes([ct[2], ct[3]]),
            message_id: &ct[4..20],
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_version() {
    let pt = b"test v1 header version";
    let ct = encrypt_v1_with_ec(pt, EncryptionContext::new()).await;
    let hdr = V1FixedFields::parse(&ct);
    //= spec/client-apis/encrypt.md#v1-header
    //= type=test
    //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0,
    //# the remaining header fields MUST be serialized according to the
    //# [Header Body Version 1.0](../data-format/message-header.md#header-body-version-10) specification:
    //
    //= spec/client-apis/encrypt.md#v1-header
    //= type=test
    //# - MUST serialize the [Version](../data-format/message-header.md#version).
    //# The value MUST correspond to [1.0](../data-format/message-header.md#supported-versions).
    //
    //= spec/data-format/message-header.md#header-body-version-1-0
    //= type=test
    //# The value of the `Version` field MUST be `01` in the Version 1.0 header body.
    assert_eq!(hdr.version, 0x01, "Version field must be 0x01 for V1");
    // Round-trip proves the V1 message is valid end-to-end.
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_type() {
    let ct = encrypt_v1_with_ec(b"type test", EncryptionContext::new()).await;
    let hdr = V1FixedFields::parse(&ct);
    //= spec/client-apis/encrypt.md#v1-header
    //= type=test
    //# - MUST serialize the [Type](../data-format/message-header.md#type).
    //# The value MUST correspond to [Customer Authenticated Encrypted Data](../data-format/message-header.md#supported-types).
    assert_eq!(hdr.msg_type, 0x80, "Type field must be 0x80 (Customer AED)");
    // Round-trip proves the type field is correctly processed.
    let result = round_trip_v1(b"type test", EncryptionContext::new()).await;
    assert_eq!(result, b"type test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_algorithm_suite_id() {
    let ct = encrypt_v1_with_ec(b"suite test", EncryptionContext::new()).await;
    let hdr = V1FixedFields::parse(&ct);

    //= spec/client-apis/encrypt.md#v1-header
    //= type=test
    //# - MUST serialize the [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id).
    //# The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.
    //
    //= spec/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //# The value (hex) of this field MUST be a value that exists in the
    //# [Supported Algorithm Suites](../framework/algorithm-suites.md#supported-algorithm-suites) table.
    //
    //= spec/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
    //
    //= spec/data-format/message-header.md#algorithm-suite-id
    //= type=test
    //# The length of the serialized algorithm suite ID field MUST be 2 bytes.
    assert_eq!(
        hdr.suite_id, 0x0178,
        "Algorithm Suite ID must match the suite used"
    );
    // Round-trip proves the algorithm suite ID is correctly processed.
    let result = round_trip_v1(b"suite test", EncryptionContext::new()).await;
    assert_eq!(result, b"suite test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_message_id() {
    //= spec/client-apis/encrypt.md#v1-header
    //= type=test
    //# - MUST serialize the [Message ID](../data-format/message-header.md#message-id).
    //# The process used to generate this identifier MUST use a good source of randomness
    //# to make the chance of duplicate identifiers negligible.
    let ct1 = encrypt_v1_with_ec(b"msg id test", EncryptionContext::new()).await;
    let ct2 = encrypt_v1_with_ec(b"msg id test", EncryptionContext::new()).await;
    let hdr1 = V1FixedFields::parse(&ct1);
    let hdr2 = V1FixedFields::parse(&ct2);

    //= spec/data-format/message-header.md#message-id
    //= type=test
    //# The length of the serialized message ID MUST be 16 bytes for [version 1.0](#header-body-version-10) headers.
    assert_eq!(hdr1.message_id.len(), 16, "V1 message ID must be 16 bytes");

    //= spec/data-format/message-header.md#message-id
    //= type=test
    //# While implementations cannot guarantee complete uniqueness,
    //# implementations MUST use a good source of randomness when generating message IDs in order to make
    //# the chance of duplicate IDs negligible.
    assert_ne!(hdr1.message_id, hdr2.message_id, "Message IDs must be unique (random)");
    // Round-trip proves the message ID is correctly processed.
    let result = round_trip_v1(b"msg id test", EncryptionContext::new()).await;
    assert_eq!(result, b"msg id test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_aad() {
    //= spec/client-apis/encrypt.md#v1-header
    //= type=test
    //# - MUST serialize the [AAD](../data-format/message-header.md#aad).
    //# The value MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials),
    //# and this serialization MUST NOT contain any key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).

    // Set up an encryption context with two keys: one that will be "required" (excluded from header)
    // and one that will be written to the header normally.
    let ec = std::collections::HashMap::from([
        ("written_key".to_string(), "written_val".to_string()),
        ("required_key".to_string(), "required_val".to_string()),
    ]);
    let required_ec_keys = vec!["required_key".to_string()];

    let keyring = test_keyring().await;
    let default_cmm = mpl()
        .create_default_cryptographic_materials_manager()
        .keyring(keyring.clone())
        .send()
        .await
        .unwrap();
    let req_cmm = mpl()
        .create_required_encryption_context_cmm()
        .underlying_cmm(default_cmm)
        .required_encryption_context_keys(required_ec_keys)
        .send()
        .await
        .unwrap();

    let pt = b"aad required ec test";
    let mut enc_input = EncryptInput::with_legacy_cmm(pt, ec.clone(), req_cmm);
    enc_input.algorithm_suite_id = Some(aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // The AAD section of the header is at offset 20 (after Version+Type+AlgSuiteID+MessageID).
    // Search the raw header bytes for the required key — it MUST NOT appear.
    let required_key_bytes = b"required_key";
    let written_key_bytes = b"written_key";

    // Find the AAD section and check within it
    let aad_byte_len = u16::from_be_bytes([ct[20], ct[21]]) as usize;
    let aad_section = &ct[20..20 + 2 + aad_byte_len];

    // The written key MUST be present in the AAD
    assert!(
        aad_section.windows(written_key_bytes.len()).any(|w| w == written_key_bytes),
        "written_key must appear in the header AAD"
    );

    // The required key MUST NOT be present in the AAD
    assert!(
        !aad_section.windows(required_key_bytes.len()).any(|w| w == required_key_bytes),
        "required_key must NOT appear in the header AAD"
    );

    // Round-trip proves the message is still valid when required EC is supplied on decrypt
    let mut dec_input = DecryptInput::with_legacy_keyring(
        &ct,
        std::collections::HashMap::from([("required_key".to_string(), "required_val".to_string())]),
        keyring,
    );
    dec_input.commitment_policy = aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt, "round-trip with required EC proves correctness");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_encrypted_data_keys() {
    let pt = b"edk test";
    let ct = encrypt_v1_with_ec(pt, EncryptionContext::new()).await;
    // Parse past AAD to find EDK count
    let edk_section = parse_edk_section(&ct, Version::V1);
    //= spec/client-apis/encrypt.md#v1-header
    //= type=test
    //# - MUST serialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
    //# The value MUST be the serialization of the
    //# [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).
    assert!(edk_section.edk_count >= 1, "must have at least one EDK");
    assert!(!edk_section.edks[0].edk.is_empty(), "EDK ciphertext must be non-empty");
    // Round-trip proves EDKs are correctly serialized (decrypt uses them to recover the data key)
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(
        result, pt,
        "round-trip proves EDKs serialized correctly (decrypt uses them)"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_content_type() {
    let ct = encrypt_v1_with_ec(b"content type test", EncryptionContext::new()).await;
    let (ct_offset, _, _, _) = parse_v1_trailing_offsets(&ct);
    //= spec/client-apis/encrypt.md#v1-header
    //= type=test
    //# - MUST serialize the [Content Type](../data-format/message-header.md#content-type).
    //# The value MUST be [02](../data-format/message-header.md#supported-content-types).
    assert_eq!(ct[ct_offset], 0x02, "Content Type must be 0x02 (framed)");
    // Round-trip proves the content type is correctly processed.
    let result = round_trip_v1(b"content type test", EncryptionContext::new()).await;
    assert_eq!(result, b"content type test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_reserved() {
    let ct = encrypt_v1_with_ec(b"reserved test", EncryptionContext::new()).await;
    let (_, reserved_offset, _, _) = parse_v1_trailing_offsets(&ct);
    //= spec/client-apis/encrypt.md#v1-header
    //= type=test
    //# - MUST serialize the [Reserved](../data-format/message-header.md#reserved).
    //
    //= spec/data-format/message-header.md#reserved
    //= type=test
    //# A reserved sequence of 4 bytes
    //# that MUST have the value (hex) of `00 00 00 00`.
    assert_eq!(
        &ct[reserved_offset..reserved_offset + 4],
        &[0x00, 0x00, 0x00, 0x00],
        "Reserved bytes must be serialized as 00 00 00 00"
    );
    // Round-trip proves the reserved field is correctly processed.
    let result = round_trip_v1(b"reserved test", EncryptionContext::new()).await;
    assert_eq!(result, b"reserved test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_iv_length() {
    let ct = encrypt_v1_with_ec(b"iv length test", EncryptionContext::new()).await;
    let (_, _, iv_length_offset, frame_length_offset) = parse_v1_trailing_offsets(&ct);
    //= spec/client-apis/encrypt.md#v1-header
    //= type=test
    //# - MUST serialize the [IV Length](../data-format/message-header.md#iv-length).
    //# The value MUST match the [IV length](../framework/algorithm-suites.md#iv-length)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md).
    //
    //= spec/data-format/message-header.md#iv-length
    //= type=test
    //# The IV length MUST be interpreted as a UInt8.
    //
    //= spec/data-format/message-header.md#iv-length
    //= type=test
    //# This value MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the
    //# [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    // AlgAes256GcmIv12Tag16HkdfSha256 has IV length 12
    assert_eq!(
        ct[iv_length_offset], 12,
        "IV Length must be 12, matching the algorithm suite (serialized as UInt8)"
    );
    //= spec/data-format/message-header.md#iv-length
    //= type=test
    //# The length of the serialized IV length field MUST be 1 byte.
    assert_eq!(
        frame_length_offset - iv_length_offset,
        1,
        "IV length field must be exactly 1 byte"
    );
    // Round-trip proves the IV length is correctly processed.
    let result = round_trip_v1(b"iv length test", EncryptionContext::new()).await;
    assert_eq!(result, b"iv length test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_frame_length() {
    let ct = encrypt_v1_with_ec(b"frame length test", EncryptionContext::new()).await;
    let (_, _, iv_length_offset, frame_length_offset) = parse_v1_trailing_offsets(&ct);
    //= spec/data-format/message-header.md#frame-length
    //= type=test
    //# The frame length MUST be interpreted as a UInt32.
    // Frame length field is 4 bytes (UInt32), immediately after IV length
    assert_eq!(
        frame_length_offset - iv_length_offset,
        1,
        "frame length immediately follows IV length (confirming IV length is 1 byte)"
    );
    let frame_length = u32::from_be_bytes([
        ct[frame_length_offset],
        ct[frame_length_offset + 1],
        ct[frame_length_offset + 2],
        ct[frame_length_offset + 3],
    ]);
    //= spec/client-apis/encrypt.md#v1-header
    //= type=test
    //# - MUST serialize the [Frame Length](../data-format/message-header.md#frame-length).
    //# The value MUST be the value of the frame size determined above.
    assert_eq!(frame_length, 4096, "Frame Length must be the default 4096 (serialized as UInt32)");
    // Round-trip proves the frame length is correctly processed.
    let result = round_trip_v1(b"frame length test", EncryptionContext::new()).await;
    assert_eq!(result, b"frame length test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_serialization_order() {
    let ct = encrypt_v1_with_ec(b"order test", EncryptionContext::new()).await;
    let hdr = V1FixedFields::parse(&ct);

    //= spec/client-apis/encrypt.md#v1-header
    //= type=test
    //# The serialization order MUST follow the [Header Body Version 1.0](../data-format/message-header.md#header-body-version-10) specification.
    //
    //= spec/data-format/message-header.md#header-body-version-1-0
    //= type=test
    //# The V1 Header Body MUST consist of, in order,
    //# Version,
    //# Type,
    //# Algorithm Suite ID,
    //# Message ID,
    //# AAD,
    //# Encrypted Data Keys,
    //# Content Type,
    //# Reserved,
    //# IV Length,
    //# and Frame Length.
    // Fixed-offset fields
    assert_eq!(hdr.version, 0x01, "offset 0: Version");
    assert_eq!(hdr.msg_type, 0x80, "offset 1: Type");
    assert_eq!(hdr.suite_id, 0x0178, "offset 2-3: Algorithm Suite ID");
    assert_eq!(hdr.message_id.len(), 16, "offset 4-19: Message ID (16 bytes)");
    // Variable-length fields (AAD, EDKs) followed by trailing fixed fields
    let (ct_offset, reserved_offset, iv_length_offset, frame_length_offset) =
        parse_v1_trailing_offsets(&ct);
    // Verify trailing fields appear in order with correct sizes
    assert!(ct_offset > 20, "Content Type must follow AAD and EDKs");
    assert_eq!(ct[ct_offset], 0x02, "Content Type = 0x02 (framed)");
    assert_eq!(reserved_offset, ct_offset + 1, "Reserved immediately follows Content Type");
    assert_eq!(
        &ct[reserved_offset..reserved_offset + 4],
        &[0x00, 0x00, 0x00, 0x00],
        "Reserved = 00 00 00 00"
    );
    assert_eq!(iv_length_offset, reserved_offset + 4, "IV Length immediately follows Reserved");
    assert_eq!(ct[iv_length_offset], 12, "IV Length = 12");
    assert_eq!(frame_length_offset, iv_length_offset + 1, "Frame Length immediately follows IV Length");
    let frame_length = u32::from_be_bytes([
        ct[frame_length_offset],
        ct[frame_length_offset + 1],
        ct[frame_length_offset + 2],
        ct[frame_length_offset + 3],
    ]);
    assert_eq!(frame_length, 4096, "Frame Length = 4096");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_version_field_is_1_byte() {
    let ct = encrypt_v1_with_ec(b"version 1 byte test", EncryptionContext::new()).await;
    //= spec/data-format/message-header.md#version
    //= type=test
    //# The length of the serialized version field MUST be 1 byte.
    // Version is at offset 0, Type is at offset 1 — proving version is exactly 1 byte
    assert_eq!(ct[0], 0x01, "version byte must be 0x01");
    assert_eq!(
        ct[1], 0x80,
        "type byte immediately follows at offset 1, proving version is 1 byte"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_type_field_is_1_byte() {
    let ct = encrypt_v1_with_ec(b"type 1 byte test", EncryptionContext::new()).await;
    // Type is at offset 1, Algorithm Suite ID starts at offset 2 — proving type is exactly 1 byte
    assert_eq!(ct[1], 0x80, "type byte must be 0x80");
    let suite_id = u16::from_be_bytes([ct[2], ct[3]]);
    assert_eq!(
        suite_id, 0x0178,
        "algorithm suite ID immediately follows at offset 2, proving type is 1 byte"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_content_type_field_is_1_byte() {
    let ct = encrypt_v1_with_ec(b"content type 1 byte test", EncryptionContext::new()).await;
    let (ct_offset, reserved_offset, _, _) = parse_v1_trailing_offsets(&ct);
    // Content type is 1 byte, reserved immediately follows
    assert_eq!(
        reserved_offset - ct_offset,
        1,
        "content type field must be exactly 1 byte"
    );
    assert_eq!(ct[ct_offset], 0x02, "content type must be 0x02 (framed)");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_reserved_field_is_4_bytes() {
    let ct = encrypt_v1_with_ec(b"reserved 4 bytes test", EncryptionContext::new()).await;
    let (_, reserved_offset, iv_length_offset, _) = parse_v1_trailing_offsets(&ct);
    //= spec/data-format/message-header.md#reserved
    //= type=test
    //# The length of the serialized reserved field MUST be 4 bytes.
    assert_eq!(
        iv_length_offset - reserved_offset,
        4,
        "reserved field must be exactly 4 bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_reserved_bytes_tampered_rejected() {
    let keyring = test_keyring().await;
    let mut ct = encrypt_v1_with_ec(b"reserved tamper test", EncryptionContext::new()).await;
    let (_, reserved_offset, _, _) = parse_v1_trailing_offsets(&ct);

    // Tamper: set reserved bytes to non-zero
    ct[reserved_offset] = 0x01;

    let mut dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy =
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;

    //= spec/data-format/message-header.md#reserved
    //= type=test
    //= reason=Tampering the reserved bytes to a non-zero value and verifying decrypt rejects it proves the reserved-bytes validation is enforced on the read path.
    //# A reserved sequence of 4 bytes
    //# that MUST have the value (hex) of `00 00 00 00`.
    let err = decrypt(&dec_input)
        .await
        .expect_err("decrypt must reject tampered reserved bytes");
    assert!(
        matches!(err.kind, aws_esdk::ErrorKind::SerializationError),
        "expected SerializationError, got: {} ({:?})",
        err.message, err.kind
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_iv_length_mismatch_rejected() {
    let keyring = test_keyring().await;
    let mut ct = encrypt_v1_with_ec(b"iv length tamper test", EncryptionContext::new()).await;
    let (_, _, iv_length_offset, _) = parse_v1_trailing_offsets(&ct);

    // The correct IV length for the V1 signing suite is 12. Set it to 11.
    assert_eq!(ct[iv_length_offset], 12, "baseline: IV length should be 12");
    ct[iv_length_offset] = 11;

    let mut dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy =
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;

    //= spec/data-format/message-header.md#iv-length
    //= type=test
    //= reason=Tampering the IV length byte to a value that does not match the algorithm suite and verifying decrypt rejects it proves the IV-length validation is enforced on the read path.
    //# This value MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the
    //# [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    let err = decrypt(&dec_input)
        .await
        .expect_err("decrypt must reject mismatched IV length");
    assert!(
        matches!(err.kind, aws_esdk::ErrorKind::SerializationError),
        "expected SerializationError, got: {} ({:?})",
        err.message, err.kind
    );
}

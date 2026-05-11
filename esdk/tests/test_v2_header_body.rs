// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for V2 header body serialization (spec/client-apis/encrypt.md#v2-header)

mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

/// V2 header fixed-offset fields.
/// Layout: Version(1) | AlgSuiteID(2) | MessageID(32) | AAD(variable) | ...
struct V2FixedFields<'a> {
    /// Version byte at offset 0.
    version: u8,
    /// Algorithm Suite ID (big-endian u16) at offset 1..3.
    suite_id: u16,
    /// Message ID (32 bytes) at offset 3..35.
    message_id: &'a [u8],
}

impl<'a> V2FixedFields<'a> {
    /// Parse the fixed-offset fields from a V2 ciphertext.
    fn parse(ct: &'a [u8]) -> Self {
        Self {
            version: ct[0],
            suite_id: u16::from_be_bytes([ct[1], ct[2]]),
            message_id: &ct[3..35],
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_version() {
    let pt = b"test v2 header version";
    let ct = encrypt_default(pt).await.ciphertext;
    let hdr = V2FixedFields::parse(&ct);
    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0,
    //# the remaining header fields MUST be serialized according to the
    //# [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification:
    //
    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //# - MUST serialize the [Version](../data-format/message-header.md#version).
    //# The value MUST correspond to [2.0](../data-format/message-header.md#supported-versions).
    //
    //= spec/data-format/message-header.md#header-body-version-2-0
    //= type=test
    //# The value of the `Version` field MUST be `02` in the Version 2.0 header body.
    assert_eq!(hdr.version, 0x02, "Version field must be 0x02 for V2");
    // Round-trip proves the V2 message is valid end-to-end.
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_algorithm_suite_id() {
    let ct = encrypt_default(b"suite test").await.ciphertext;
    let hdr = V2FixedFields::parse(&ct);

    //= spec/client-apis/encrypt.md#v2-header
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
    //# The length of the serialized algorithm suite ID field MUST be 2 bytes.
    assert_eq!(
        hdr.suite_id, 0x0578,
        "Algorithm Suite ID must match the default V2 committing suite"
    );
    // Round-trip proves the algorithm suite ID is correctly processed.
    let result = round_trip_v2(b"suite test", EncryptionContext::new()).await;
    assert_eq!(result, b"suite test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_message_id() {
    let ct1 = encrypt_default(b"msg id v2 test").await.ciphertext;
    let ct2 = encrypt_default(b"msg id v2 test").await.ciphertext;
    let hdr1 = V2FixedFields::parse(&ct1);
    let hdr2 = V2FixedFields::parse(&ct2);

    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //# - MUST serialize the [Message ID](../data-format/message-header.md#message-id).
    //# The process used to generate this identifier MUST use a good source of randomness
    //# to make the chance of duplicate identifiers negligible.
    //
    //= spec/data-format/message-header.md#message-id
    //= type=test
    //# The length of the serialized message ID MUST be 32 bytes for [version 2.0](#header-body-version-20) headers.
    assert_eq!(hdr1.message_id.len(), 32, "V2 Message ID must be 32 bytes");

    //= spec/data-format/message-header.md#message-id
    //= type=test
    //= reason=two encryptions of the same plaintext produce different message IDs, confirming randomness
    //# While implementations cannot guarantee complete uniqueness,
    //# implementations MUST use a good source of randomness when generating message IDs in order to make
    //# the chance of duplicate IDs negligible.
    assert_ne!(hdr1.message_id, hdr2.message_id, "V2 Message IDs must be unique (random)");
    // Round-trip proves the message ID is correctly processed.
    let result = round_trip_v2(b"msg id v2 test", EncryptionContext::new()).await;
    assert_eq!(result, b"msg id v2 test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_aad() {
    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //# - MUST serialize the [AAD](../data-format/message-header.md#aad).
    //# The value MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials),
    //# and this serialization MUST NOT contain any key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).

    // Set up an encryption context with two keys: one "required" (excluded from header)
    // and one written to the header normally.
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

    let pt = b"aad required ec test v2";
    let enc_input = EncryptInput::with_legacy_cmm(pt, ec.clone(), req_cmm);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // AAD starts at offset 35 (after Version(1) + AlgSuiteID(2) + MessageID(32))
    let aad_byte_len = u16::from_be_bytes([ct[35], ct[36]]) as usize;
    let aad_section = &ct[35..35 + 2 + aad_byte_len];

    let required_key_bytes = b"required_key";
    let written_key_bytes = b"written_key";

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
    let dec_input = DecryptInput::with_legacy_keyring(
        &ct,
        std::collections::HashMap::from([("required_key".to_string(), "required_val".to_string())]),
        keyring,
    );
    let result = decrypt(&dec_input).await.unwrap();
    assert_eq!(result.plaintext, pt, "round-trip with required EC proves correctness");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_encrypted_data_keys() {
    let pt = b"edk test";
    let ct = encrypt_default(pt).await.ciphertext;
    let edk_section = parse_edk_section(&ct, Version::V2);
    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //# - MUST serialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
    //# The value MUST be the serialization of the
    //# [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).
    assert!(edk_section.edk_count >= 1, "must have at least one EDK");
    assert!(!edk_section.edks[0].edk.is_empty(), "EDK ciphertext must be non-empty");
    // Round-trip proves EDKs are correctly serialized (decrypt uses them to recover the data key)
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "round-trip proves EDKs serialized correctly");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_content_type() {
    let ct = encrypt_default(b"content type test").await.ciphertext;
    let offset = content_type_offset_v2(&ct);
    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //# - MUST serialize the [Content Type](../data-format/message-header.md#content-type).
    //# The value MUST be [02](../data-format/message-header.md#supported-content-types).
    assert_eq!(ct[offset], 0x02, "V2 content type must be 0x02 (framed)");
    // Round-trip proves the content type is correctly processed.
    let result = round_trip_v2(b"content type test", EncryptionContext::new()).await;
    assert_eq!(result, b"content type test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_frame_length() {
    let ct = encrypt_default(b"frame length test").await.ciphertext;
    let (_, _, fl_offset) = parse_header_offsets(&ct);
    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //# - MUST serialize the [Frame Length](../data-format/message-header.md#frame-length).
    //# The value MUST be the value of the frame size determined above.
    //
    //= spec/data-format/message-header.md#frame-length
    //= type=test
    //# The frame length MUST be interpreted as a UInt32.
    let frame_len = u32::from_be_bytes([
        ct[fl_offset], ct[fl_offset + 1], ct[fl_offset + 2], ct[fl_offset + 3],
    ]);
    assert_eq!(frame_len, 4096, "Frame length must be the default 4096 (serialized as UInt32)");
    // Round-trip proves the frame length is correctly processed.
    let result = round_trip_v2(b"frame length test", EncryptionContext::new()).await;
    assert_eq!(result, b"frame length test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_algorithm_suite_data() {
    let ct = encrypt_default(b"suite data test").await.ciphertext;
    let fields = parse_v2_header_field_offsets(&ct);
    let (_, start, end) = fields
        .iter()
        .find(|(n, _, _)| *n == "Algorithm Suite Data")
        .unwrap();

    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //= reason=verifies the algorithm suite data (commit key) is 32 bytes in the raw V2 header and round-trip proves correctness
    //# - MUST serialize the [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data).
    //# The value MUST be the value of the [commit key](../framework/algorithm-suites.md#commit-key)
    //# derived according to the [algorithm suites commit key derivation settings](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
    assert_eq!(end - start, 32, "Algorithm Suite Data must be 32 bytes for committing suites");
    // Round-trip proves algorithm suite data (commit key) is correct.
    let result = round_trip_v2(b"suite data test", EncryptionContext::new()).await;
    assert_eq!(result, b"suite data test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_serialization_order() {
    let ct = encrypt_default(b"order test").await.ciphertext;
    let fields = parse_v2_header_field_offsets(&ct);

    //= spec/client-apis/encrypt.md#v2-header
    //= type=test
    //# The serialization order MUST follow the [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification.
    //
    //= spec/data-format/message-header.md#header-body-version-2-0
    //= type=test
    //# The V2 Header Body MUST consist of, in order,
    //# Version,
    //# Algorithm Suite ID,
    //# Message ID,
    //# AAD,
    //# Encrypted Data Keys,
    //# Content Type,
    //# Frame Length,
    //# and Algorithm Suite Data.
    let expected_order = [
        "Version",
        "Algorithm Suite ID",
        "Message ID",
        "AAD",
        "Encrypted Data Keys",
        "Content Type",
        "Frame Length",
        "Algorithm Suite Data",
    ];

    assert_eq!(fields.len(), expected_order.len(), "V2 header must have exactly 8 fields");

    for (i, (name, start, end)) in fields.iter().enumerate() {
        assert_eq!(*name, expected_order[i], "field {i} should be '{}'", expected_order[i]);
        assert!(start < end, "field '{}' has zero or negative length", name);
        if i > 0 {
            let (_, _, prev_end) = fields[i - 1];
            assert_eq!(
                *start, prev_end,
                "field '{}' does not immediately follow '{}'",
                name, fields[i - 1].0
            );
        }
    }
}

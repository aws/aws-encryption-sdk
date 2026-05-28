// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/decrypt.md#parse-the-header
//! and spec/client-apis/decrypt.md#behavior

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use fixtures::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_unsupported_version_rejected() {
    let keyring = test_keyring().await;
    let plaintext = b"unsupported version test";

    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Byte 0 is the version field — set to an unsupported value
    let mut tampered_ct = valid_ct.clone();
    tampered_ct[0] = 0xFF;

    let valid_input = DecryptInput::from_encrypt(&valid_ct, &enc_input);
    let tampered_input = DecryptInput::from_encrypt(&tampered_ct, &enc_input);

    //= spec/client-apis/decrypt.md#parse-the-header
    //= type=test
    //= reason=Supported version byte → Ok; unsupported value 0xFF → SerializationError
    //# The value MUST be a [supported version](../data-format/message-header.md#supported-versions).
    assert!(decrypt(&valid_input).await.is_ok(), "supported version must decrypt");
    assert_eq!(
        decrypt(&tampered_input).await.unwrap_err().kind,
        ErrorKind::SerializationError,
        "unsupported version byte must produce SerializationError"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unsupported_content_type_v1_rejected() {
    let keyring = test_keyring().await;
    let plaintext = b"unsupported content type v1 test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // V1 header layout with empty encryption context:
    // version(1) + type(1) + alg_suite(2) + message_id(16) = 20 bytes fixed
    // AAD section with empty EC: key_value_pairs_length(2) = 0x0000 → 2 bytes total
    // EDK section: edk_count(2) + each EDK
    let mut pos: usize = 20; // after version + type + alg_suite + message_id
    let aad_len = u16::from_be_bytes([valid_ct[pos], valid_ct[pos + 1]]) as usize;
    pos += 2;
    if aad_len > 0 {
        pos += aad_len;
    }
    let edk_count = u16::from_be_bytes([valid_ct[pos], valid_ct[pos + 1]]) as usize;
    pos += 2;
    for _ in 0..edk_count {
        let provider_id_len = u16::from_be_bytes([valid_ct[pos], valid_ct[pos + 1]]) as usize;
        pos += 2 + provider_id_len;
        let provider_info_len = u16::from_be_bytes([valid_ct[pos], valid_ct[pos + 1]]) as usize;
        pos += 2 + provider_info_len;
        let edk_len = u16::from_be_bytes([valid_ct[pos], valid_ct[pos + 1]]) as usize;
        pos += 2 + edk_len;
    }
    // pos now points to the content type byte
    let original_content_type = valid_ct[pos];
    assert!(
        original_content_type == 1 || original_content_type == 2,
        "sanity check: content type should be 1 or 2, got {original_content_type}"
    );

    let mut tampered_ct = valid_ct.clone();
    tampered_ct[pos] = 0xFF; // unsupported content type

    let mut valid_input = DecryptInput::from_encrypt(&valid_ct, &enc_input);
    valid_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let mut tampered_input = DecryptInput::from_encrypt(&tampered_ct, &enc_input);
    tampered_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=Supported content type byte → Ok; unsupported value 0xFF → SerializationError
    //# The value MUST be a [supported content type](../data-format/message-header.md#supported-content-types).
    assert!(decrypt(&valid_input).await.is_ok(), "supported content type must decrypt");
    assert_eq!(
        decrypt(&tampered_input).await.unwrap_err().kind,
        ErrorKind::SerializationError,
        "unsupported V1 content type must produce SerializationError"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unsupported_content_type_v2_rejected() {
    let keyring = test_keyring().await;
    let plaintext = b"unsupported content type v2 test";

    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // V2 header layout with empty encryption context:
    // version(1) + alg_suite(2) + message_id(32) = 35 bytes fixed
    let mut pos: usize = 35;
    let aad_len = u16::from_be_bytes([valid_ct[pos], valid_ct[pos + 1]]) as usize;
    pos += 2;
    if aad_len > 0 {
        pos += aad_len;
    }
    let edk_count = u16::from_be_bytes([valid_ct[pos], valid_ct[pos + 1]]) as usize;
    pos += 2;
    for _ in 0..edk_count {
        let provider_id_len = u16::from_be_bytes([valid_ct[pos], valid_ct[pos + 1]]) as usize;
        pos += 2 + provider_id_len;
        let provider_info_len = u16::from_be_bytes([valid_ct[pos], valid_ct[pos + 1]]) as usize;
        pos += 2 + provider_info_len;
        let edk_len = u16::from_be_bytes([valid_ct[pos], valid_ct[pos + 1]]) as usize;
        pos += 2 + edk_len;
    }
    let original_content_type = valid_ct[pos];
    assert!(
        original_content_type == 1 || original_content_type == 2,
        "sanity check: content type should be 1 or 2, got {original_content_type}"
    );

    let mut tampered_ct = valid_ct.clone();
    tampered_ct[pos] = 0xFF; // unsupported content type

    let valid_input = DecryptInput::from_encrypt(&valid_ct, &enc_input);
    let tampered_input = DecryptInput::from_encrypt(&tampered_ct, &enc_input);

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=Supported content type byte → Ok; unsupported value 0xFF → SerializationError
    //# The value MUST be a [supported content type](../data-format/message-header.md#supported-content-types).
    assert!(decrypt(&valid_input).await.is_ok(), "supported content type must decrypt");
    assert_eq!(
        decrypt(&tampered_input).await.unwrap_err().kind,
        ErrorKind::SerializationError,
        "unsupported V2 content type must produce SerializationError"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unsupported_type_rejected() {
    let keyring = test_keyring().await;
    let plaintext = b"unsupported type test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // V1 layout: byte 0 = version (0x01), byte 1 = type (0x80)
    assert_eq!(valid_ct[1], 0x80, "sanity check: V1 type should be 0x80");

    let mut tampered_ct = valid_ct.clone();
    tampered_ct[1] = 0xFF; // unsupported type

    let mut valid_input = DecryptInput::from_encrypt(&valid_ct, &enc_input);
    valid_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let mut tampered_input = DecryptInput::from_encrypt(&tampered_ct, &enc_input);
    tampered_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=Supported type byte (0x80) → Ok; unsupported value 0xFF → SerializationError
    //# The value MUST be a [supported type](../data-format/message-header.md#supported-types).
    assert!(decrypt(&valid_input).await.is_ok(), "supported type must decrypt");
    assert_eq!(
        decrypt(&tampered_input).await.unwrap_err().kind,
        ErrorKind::SerializationError,
        "unsupported V1 type byte must produce SerializationError"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_max_encrypted_data_keys_enforcement() {
    // Create two keyrings and a multi-keyring to produce 2 EDKs
    let keyring1 = test_keyring().await;
    let (ns2, name2) = namespace_and_name(1);
    let keyring2 = mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns2)
        .key_name(name2)
        .wrapping_key(aws_smithy_types::Blob::new([1u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();
    let multi_keyring = mpl()
        .create_multi_keyring()
        .generator(keyring1.clone())
        .child_keyrings(vec![keyring2])
        .send()
        .await
        .unwrap();

    let plaintext = b"max edk decrypt test";
    let enc_input = EncryptInput::with_legacy_keyring(
        plaintext,
        EncryptionContext::new(),
        multi_keyring.clone(),
    );
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Decrypt with max_encrypted_data_keys=1, but message has 2 EDKs → must fail
    let mut dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), multi_keyring);
    dec_input.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(1).unwrap());
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when EDK count exceeds max_encrypted_data_keys");
    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=2 EDKs on wire + max=1 → SerializationError; proves the limit halts deserialization
    //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys)
    //# deserialized from the [message header](../data-format/message-header.md)
    //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md),
    //# then as soon as that can be determined during deserializing
    //# decrypt MUST process no more bytes and yield an error.
    assert_eq!(err.kind, ErrorKind::SerializationError, "got: {err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_no_header_info_released_before_verification() {
    // Tamper with the header auth tag so header verification fails.
    // The non-streaming decrypt must return an error with no partial output.
    let keyring = test_keyring().await;
    let plaintext = b"no header info before verification";

    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let mut tampered_ct = valid_ct.clone();
    tampered_ct[10] ^= 0xFF;

    let valid_input = DecryptInput::from_encrypt(&valid_ct, &enc_input);
    let tampered_input = DecryptInput::from_encrypt(&tampered_ct, &enc_input);

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=Untampered ct decrypts (Ok with output); tampered header byte → ValidationError with no DecryptOutput, so nothing released
    //# Until the [header is verified](#verify-the-header), this operation MUST NOT
    //# release any parsed information from the header.
    assert!(decrypt(&valid_input).await.is_ok(), "valid ct must decrypt");
    assert_eq!(
        decrypt(&tampered_input).await.unwrap_err().kind,
        ErrorKind::ValidationError,
        "tampered header must produce ValidationError, no output"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_tag_deserialized_and_verified() {
    // Tamper the V1 header auth tag to prove it was deserialized and used for verification.
    let keyring = test_keyring().await;
    let plaintext = b"v1 header auth tag tamper test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // V1 header auth: IV(12) + Tag(16) at the end of the header, before the body.
    // Tamper the last byte of the header (part of the auth tag).
    let body_start = find_body_start(&valid_ct, 4096).expect("body start");
    let auth_tag_last_byte = body_start - 1;
    let mut tampered_ct = valid_ct.clone();
    tampered_ct[auth_tag_last_byte] ^= 0xFF;

    let mut valid_input = DecryptInput::from_encrypt(&valid_ct, &enc_input);
    valid_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let mut tampered_input = DecryptInput::from_encrypt(&tampered_ct, &enc_input);
    tampered_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=Untampered V1 message decrypts; tampered V1 auth tag byte → CryptographicError, proving the tag was deserialized and used
    //# - MUST deserialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
    assert!(decrypt(&valid_input).await.is_ok(), "valid V1 ct must decrypt");
    assert_eq!(
        decrypt(&tampered_input).await.unwrap_err().kind,
        ErrorKind::CryptographicError,
        "tampered V1 auth tag must produce CryptographicError"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_iv_deserialized_and_used() {
    // Tamper the V1 header auth IV to prove it was deserialized and used for verification.
    let keyring = test_keyring().await;
    let plaintext = b"v1 header auth iv tamper test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // V1 header auth IV is 12 bytes immediately before the 16-byte auth tag, which
    // is immediately before the body. Tamper its first byte.
    let body_start = find_body_start(&valid_ct, 4096).expect("body start");
    let iv_start = body_start - 16 - 12; // 16 bytes tag + 12 bytes IV before body
    let mut tampered_ct = valid_ct.clone();
    tampered_ct[iv_start] ^= 0xFF;

    let mut valid_input = DecryptInput::from_encrypt(&valid_ct, &enc_input);
    valid_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let mut tampered_input = DecryptInput::from_encrypt(&tampered_ct, &enc_input);
    tampered_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=Untampered V1 message decrypts; tampered V1 header auth IV byte → CryptographicError, proving IV was deserialized and used
    //# - MUST deserialize the [IV](../data-format/message-header.md#iv).
    assert!(decrypt(&valid_input).await.is_ok(), "valid V1 ct must decrypt");
    assert_eq!(
        decrypt(&tampered_input).await.unwrap_err().kind,
        ErrorKind::CryptographicError,
        "tampered V1 header auth IV must produce CryptographicError"
    );
}


#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_body_fields_parsed_from_wire() {
    // Drive the production deserialization path (read_header_body) on a known
    // V1 ciphertext, then use independent on-wire field offsets
    // (parse_v1_trailing_offsets) to assert each field's bytes/length/value at
    // the spec-defined location. Each annotation is placed immediately above
    // the assertion that proves that specific field was deserialized:
    //
    //   - For fields with public accessors (`message_id`, `algorithm_suite`),
    //     the parsed value is asserted to equal the wire bytes at the field's
    //     offset.
    //   - For other fields, the wire bytes at the spec-defined offset are
    //     asserted to have the expected length/value, AND `read_header_body`
    //     returned Ok (consuming exactly those bytes in order). If the
    //     implementation skipped a field, downstream offsets would be
    //     misaligned and read_header_body would fail or yield wrong values
    //     for the public-accessor fields.
    use aws_esdk::__test_internals::*;

    let keyring = test_keyring().await;
    let plaintext = b"v1 fields parse test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Parse the V1 header body via the production deserialization function.
    let mut cursor = std::io::Cursor::new(ct.as_slice());
    let mut raw = Vec::new();
    let header_body =
        read_header_body(&mut cursor, None, &mut raw).expect("V1 header body must parse");
    assert_eq!(
        header_body.algorithm_suite().message_version,
        1,
        "expected V1 body"
    );

    // Independent on-wire field offsets for V1 trailing fields.
    let (content_type_offset, reserved_offset, iv_length_offset, frame_length_offset) =
        parse_v1_trailing_offsets(&ct);

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=Wire byte 1 = 0x80 (Customer AED); read_header_body consumed it as Type
    //# - MUST deserialize the [Type](../data-format/message-header.md#type).
    assert_eq!(ct[1], 0x80, "V1 type byte must be 0x80 (Customer AED)");

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=Parsed alg suite binary_id equals wire bytes 2..4
    //# - MUST deserialize the [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id).
    assert_eq!(
        header_body.algorithm_suite().binary_id,
        [ct[2], ct[3]],
        "parsed alg suite binary_id equals wire bytes"
    );

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=V1 message_id is 16 wire bytes (4..20); pub accessor returns those bytes
    //# - MUST deserialize the [Message ID](../data-format/message-header.md#message-id).
    assert_eq!(
        header_body.message_id().len(),
        16,
        "V1 message_id is 16 bytes"
    );
    assert_eq!(
        header_body.message_id(),
        &ct[4..20],
        "parsed message_id equals wire bytes 4..20"
    );

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=AAD length field at offset 20 declares N; content_type_offset = 22 + N
    //# - MUST deserialize the [AAD](../data-format/message-header.md#aad).
    let aad_declared_len = u16::from_be_bytes([ct[20], ct[21]]) as usize;
    let aad_section_len = if aad_declared_len > 0 {
        2 + aad_declared_len
    } else {
        2
    };
    assert!(
        20 + aad_section_len <= content_type_offset,
        "AAD section bytes (length 2 + declared {aad_declared_len}) consumed before EDKs and trailing fields"
    );

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=EDK count and bodies consumed; content_type_offset is past EDK section
    //# - MUST deserialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
    let edk_section_start = 20 + aad_section_len;
    let edk_count_on_wire = u16::from_be_bytes([ct[edk_section_start], ct[edk_section_start + 1]]);
    assert_eq!(edk_count_on_wire, 1, "single keyring produces 1 EDK on wire");
    assert!(
        content_type_offset > edk_section_start,
        "EDK section bytes consumed before content_type"
    );

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=Content type byte at trailing offset = 0x02 (Framed)
    //# - MUST deserialize the [Content Type](../data-format/message-header.md#content-type).
    assert_eq!(
        ct[content_type_offset], 0x02,
        "default content type is Framed (0x02)"
    );

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=Reserved is 4 zero bytes at expected offset (per spec)
    //# - MUST deserialize the [Reserved](../data-format/message-header.md#reserved).
    assert_eq!(
        &ct[reserved_offset..reserved_offset + 4],
        &[0u8; 4],
        "Reserved must be 4 zero bytes"
    );

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=IV Length is 1 byte at expected offset, value=12 for AES-GCM suite
    //# - MUST deserialize the [IV Length](../data-format/message-header.md#iv-length).
    assert_eq!(
        ct[iv_length_offset], 12,
        "V1 IV Length byte must equal AES-GCM IV length (12)"
    );

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //= type=test
    //= reason=Frame length is 4 wire bytes BE = 4096 (default)
    //# - MUST deserialize the [Frame Length](../data-format/message-header.md#frame-length).
    let fl_value = u32::from_be_bytes([
        ct[frame_length_offset],
        ct[frame_length_offset + 1],
        ct[frame_length_offset + 2],
        ct[frame_length_offset + 3],
    ]);
    assert_eq!(fl_value, 4096, "default frame length on wire = 4096");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_body_fields_parsed_from_wire() {
    // Drive the production deserialization path (read_header_body and
    // read_header_auth_tag) on a known V2 ciphertext, then use independent
    // on-wire field offsets (parse_v2_header_field_offsets) to assert each
    // field's bytes/length/value at the spec-defined location. Each
    // annotation is placed immediately above the assertion that proves
    // that specific field was deserialized.
    use aws_esdk::__test_internals::*;

    let keyring = test_keyring().await;
    let plaintext = b"v2 fields parse test";

    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Parse the V2 header body via the production deserialization function.
    let mut cursor = std::io::Cursor::new(ct.as_slice());
    let mut raw = Vec::new();
    let header_body =
        read_header_body(&mut cursor, None, &mut raw).expect("V2 header body must parse");
    assert_eq!(
        header_body.algorithm_suite().message_version,
        2,
        "expected V2 body"
    );

    // Independent on-wire field offsets for V2 header.
    let fields = parse_v2_header_field_offsets(&ct);
    let get = |name: &'static str| -> (usize, usize) {
        let (_, s, e) = fields
            .iter()
            .find(|(n, _, _)| *n == name)
            .unwrap_or_else(|| panic!("V2 header must contain field {name}"));
        (*s, *e)
    };

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=Parsed alg suite binary_id equals on-wire bytes; read_header_body consumed them
    //# - MUST deserialize the [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id).
    let (suite_start, suite_end) = get("Algorithm Suite ID");
    assert_eq!(suite_end - suite_start, 2, "Algorithm Suite ID is 2 bytes");
    assert_eq!(
        header_body.algorithm_suite().binary_id,
        [ct[suite_start], ct[suite_start + 1]],
        "parsed alg suite binary_id equals wire bytes"
    );

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=V2 message_id is 32 wire bytes; pub accessor returns those bytes
    //# - MUST deserialize the [Message ID](../data-format/message-header.md#message-id).
    let (mid_start, mid_end) = get("Message ID");
    assert_eq!(mid_end - mid_start, 32, "V2 message_id is 32 bytes");
    assert_eq!(
        header_body.message_id(),
        &ct[mid_start..mid_end],
        "parsed message_id equals wire bytes"
    );

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=AAD section length on wire equals 2-byte length field + declared bytes
    //# - MUST deserialize the [AAD](../data-format/message-header.md#aad).
    let (aad_start, aad_end) = get("AAD");
    let aad_declared = u16::from_be_bytes([ct[aad_start], ct[aad_start + 1]]) as usize;
    assert_eq!(
        aad_end - aad_start,
        2 + aad_declared,
        "AAD on-wire span = 2 (length field) + declared bytes"
    );

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=EDK count read at expected offset = 1 (single keyring)
    //# - MUST deserialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
    let (edk_start, _) = get("Encrypted Data Keys");
    let edk_count_on_wire = u16::from_be_bytes([ct[edk_start], ct[edk_start + 1]]);
    assert_eq!(edk_count_on_wire, 1, "single keyring produces 1 EDK on wire");

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=Content type byte at expected offset = 0x02 (Framed)
    //# - MUST deserialize the [Content Type](../data-format/message-header.md#content-type).
    let (ct_start, ct_end) = get("Content Type");
    assert_eq!(ct_end - ct_start, 1, "content type is 1 byte on wire");
    assert_eq!(ct[ct_start], 0x02, "default content type is Framed (0x02)");

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=Frame length is 4 wire bytes BE = 4096 (default)
    //# - MUST deserialize the [Frame Length](../data-format/message-header.md#frame-length).
    let (fl_start, fl_end) = get("Frame Length");
    assert_eq!(fl_end - fl_start, 4, "frame length is 4 bytes");
    let fl_value = u32::from_be_bytes([
        ct[fl_start],
        ct[fl_start + 1],
        ct[fl_start + 2],
        ct[fl_start + 3],
    ]);
    assert_eq!(fl_value, 4096, "default frame length = 4096");

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=Algorithm Suite Data is 32 wire bytes (V2 commit key field)
    //# - MUST deserialize the [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data).
    let (sd_start, sd_end) = get("Algorithm Suite Data");
    assert_eq!(
        sd_end - sd_start,
        32,
        "V2 Algorithm Suite Data is 32 bytes"
    );

    // Now tamper the V2 auth tag byte (immediately after Algorithm Suite Data)
    // and confirm decrypt fails with CryptographicError — direct proof that
    // the implementation deserialized the auth tag from the wire and used it
    // for header verification.
    let auth_tag_offset = sd_end;
    let auth_tag_end = auth_tag_offset + 16;
    assert!(
        auth_tag_end <= ct.len(),
        "auth tag bytes are present in wire after Algorithm Suite Data"
    );
    let mut tampered_ct = ct.clone();
    tampered_ct[auth_tag_offset] ^= 0xFF;

    let valid_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone());
    let tampered_input =
        DecryptInput::with_legacy_keyring(&tampered_ct, EncryptionContext::new(), keyring);

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=test
    //= reason=Untampered ct decrypts; tampered V2 auth tag byte → CryptographicError, proving the auth tag was deserialized and used
    //# - MUST deserialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
    assert!(decrypt(&valid_input).await.is_ok(), "valid V2 ct must decrypt");
    assert_eq!(
        decrypt(&tampered_input).await.unwrap_err().kind,
        ErrorKind::CryptographicError,
        "tampered V2 auth tag must produce CryptographicError"
    );
}

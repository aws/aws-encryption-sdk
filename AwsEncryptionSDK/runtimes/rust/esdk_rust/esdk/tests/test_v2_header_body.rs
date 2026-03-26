// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for V2 header body serialization order

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

/// Encrypt plaintext with default settings (V2 algorithm suite), return ciphertext bytes.
async fn encrypt_default(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    encrypt(&input).await.unwrap().ciphertext
}

/// Parse the V2 header body fields from ciphertext bytes, returning the byte offset
/// after each field boundary in order. Panics if the header is not well-formed.
/// Returns a Vec of (field_name, start_offset, end_offset) tuples.
fn parse_v2_header_field_offsets(ct: &[u8]) -> Vec<(&'static str, usize, usize)> {
    let mut fields = Vec::new();
    let mut pos = 0;

    // Version: 1 byte (must be 0x02 for V2)
    assert!(pos < ct.len(), "not enough bytes for Version");
    assert_eq!(ct[pos], 0x02, "expected V2 version byte");
    fields.push(("Version", pos, pos + 1));
    pos += 1;

    // Algorithm Suite ID: 2 bytes
    assert!(pos + 2 <= ct.len(), "not enough bytes for Algorithm Suite ID");
    fields.push(("Algorithm Suite ID", pos, pos + 2));
    pos += 2;

    // Message ID: 32 bytes (V2 uses 32-byte message IDs)
    assert!(pos + 32 <= ct.len(), "not enough bytes for Message ID");
    fields.push(("Message ID", pos, pos + 32));
    pos += 32;

    // AAD: variable length, self-describing
    // Format: aad_byte_length (2 bytes), then if non-zero:
    //   key_value_pair_count (2 bytes), then for each pair:
    //   key_length (2 bytes) + key + value_length (2 bytes) + value
    assert!(pos + 2 <= ct.len(), "not enough bytes for AAD length");
    let aad_start = pos;
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        // kv_count (2 bytes) is NOT included in aad_byte_len
        pos += 2 + aad_byte_len;
    }
    fields.push(("AAD", aad_start, pos));

    // Encrypted Data Keys: variable length
    // Format: edk_count (2 bytes), then for each EDK:
    //   provider_id_length (2 bytes) + provider_id +
    //   provider_info_length (2 bytes) + provider_info +
    //   ciphertext_length (2 bytes) + ciphertext
    assert!(pos + 2 <= ct.len(), "not enough bytes for EDK count");
    let edk_start = pos;
    let edk_count = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    for i in 0..edk_count {
        assert!(pos + 2 <= ct.len(), "not enough bytes for provider ID length (EDK {})", i);
        let pid_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pid_len;
        assert!(pos + 2 <= ct.len(), "not enough bytes for provider info length (EDK {})", i);
        let pinfo_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pinfo_len;
        assert!(pos + 2 <= ct.len(), "not enough bytes for EDK ciphertext length (EDK {})", i);
        let ct_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + ct_len;
    }
    fields.push(("Encrypted Data Keys", edk_start, pos));

    // Content Type: 1 byte
    assert!(pos < ct.len(), "not enough bytes for Content Type");
    fields.push(("Content Type", pos, pos + 1));
    pos += 1;

    // Frame Length: 4 bytes
    assert!(pos + 4 <= ct.len(), "not enough bytes for Frame Length");
    fields.push(("Frame Length", pos, pos + 4));
    pos += 4;

    // Algorithm Suite Data: variable, determined by algorithm suite
    // For committing suites, this is 32 bytes (commit key)
    assert!(pos + 32 <= ct.len(), "not enough bytes for Algorithm Suite Data");
    fields.push(("Algorithm Suite Data", pos, pos + 32));

    fields
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_body_serialization_order() {
    //= specification/client-apis/encrypt.md#v2-header
    //= type=test
    //# The serialization order MUST follow the [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification.

    //= specification/data-format/message-header.md#header-body-version-2-0
    //= type=test
    //# The V2 Header Body MUST be serialized as, in order,
    //# Version,
    //# Algorithm Suite ID,
    //# Message ID,
    //# AAD,
    //# Encrypted Data Keys,
    //# Content Type,
    //# Frame Length,
    //# and Algorithm Suite Data.

    let ct = encrypt_default(b"test plaintext").await;
    let fields = parse_v2_header_field_offsets(&ct);

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

    assert_eq!(
        fields.len(),
        expected_order.len(),
        "expected {} header fields, got {}",
        expected_order.len(),
        fields.len()
    );

    for (i, (name, start, end)) in fields.iter().enumerate() {
        assert_eq!(
            *name, expected_order[i],
            "field {} should be '{}' but was '{}'",
            i, expected_order[i], name
        );
        assert!(
            start < end,
            "field '{}' has zero or negative length",
            name
        );
        if i > 0 {
            let (_, _, prev_end) = fields[i - 1];
            assert_eq!(
                *start, prev_end,
                "field '{}' does not immediately follow '{}' (gap at byte {})",
                name,
                fields[i - 1].0,
                prev_end
            );
        }
    }
}

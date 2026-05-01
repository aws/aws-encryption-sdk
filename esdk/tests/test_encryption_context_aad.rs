// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/data-format/message-header.md#aad,
//! #key-value-pairs-length, and #key-value-pairs

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use fixtures::*;
use test_helpers::*;

/// V1 header AAD offset: Version(1) + Type(1) + AlgSuiteID(2) + MessageID(16) = 20.
const V1_AAD_OFFSET: usize = 20;
/// V2 header AAD offset: Version(1) + AlgSuiteID(2) + MessageID(32) = 35.
const V2_AAD_OFFSET: usize = 35;

fn aad_offset(version: Version) -> usize {
    match version {
        Version::V1 => V1_AAD_OFFSET,
        Version::V2 => V2_AAD_OFFSET,
    }
}

/// Assert that every (key, value) pair in `expected` is present in `actual`.
/// Used to verify the encryption context survives the round trip intact,
/// while ignoring any keys the SDK may add (e.g. `aws-crypto-public-key`
/// for signing suites — not used by these tests, but the check is defensive).
fn assert_ec_contains(actual: &EncryptionContext, expected: &EncryptionContext, version: Version) {
    for (k, v) in expected {
        assert_eq!(
            actual.get(k),
            Some(v),
            "{version:?}: decrypted EC missing or mismatched for key {k:?}"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_aad_serialization_order() {
    for version in VERSIONS {
        let ec = small_encryption_context(SmallEncryptionContextVariation::AB);
        let pt = b"aad serialization order";
        let ct = encrypt_no_sign_with_ec(pt, ec.clone(), version).await;
        let off = aad_offset(version);

        //= specification/data-format/message-header.md#aad
        //= type=test
        //# The AAD MUST consist of, in order,
        //# Key Value Pairs Length,
        //# and Key Value Pairs.

        // Primary assertion: the encrypt path lays out KVP Length first, followed by KVP data.
        let kvp_len = u16::from_be_bytes([ct[off], ct[off + 1]]) as usize;
        assert!(kvp_len > 0, "{version:?}: non-empty EC must have non-zero KVP length");
        // KVP data follows immediately after the 2-byte length field (count is first).
        let kvp_count_offset = off + 2;
        let kvp_count =
            u16::from_be_bytes([ct[kvp_count_offset], ct[kvp_count_offset + 1]]) as usize;
        assert_eq!(kvp_count, 2, "{version:?}: AB encryption context has 2 key-value pairs");

        // Cross-check: the decrypt path recovers the same encryption context, which is
        // only possible if the on-wire ordering agreed with the spec on both sides.
        let dec = decrypt_with_version(&ct, version).await;
        assert_ec_contains(&dec.encryption_context, &ec, version);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_aad_key_value_pairs_length_field_size() {
    for version in VERSIONS {
        let ec = small_encryption_context(SmallEncryptionContextVariation::A);
        let pt = b"kvp length field size";
        let ct = encrypt_no_sign_with_ec(pt, ec.clone(), version).await;
        let off = aad_offset(version);

        //= specification/data-format/message-header.md#key-value-pairs-length
        //= type=test
        //# The length of the serialized key value pairs length field MUST be 2 bytes.

        // The KVP length field occupies exactly 2 bytes at [off..off+2].
        let kvp_len = u16::from_be_bytes([ct[off], ct[off + 1]]) as usize;
        // For "A" (keyA=valA): key_len(2) + key(4) + val_len(2) + val(4) = 12 bytes of pair data.
        assert_eq!(kvp_len, 12, "{version:?}: KVP length for single pair keyA=valA must be 12");

        // Cross-check: the decrypted EC matches what we encrypted, confirming the 2-byte
        // length field was parsed correctly on the decrypt side too.
        let dec = decrypt_with_version(&ct, version).await;
        assert_ec_contains(&dec.encryption_context, &ec, version);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_aad_key_value_pairs_length_uint16() {
    for version in VERSIONS {
        let ec = small_encryption_context(SmallEncryptionContextVariation::A);
        let pt = b"kvp length uint16";
        let ct = encrypt_no_sign_with_ec(pt, ec.clone(), version).await;
        let off = aad_offset(version);

        //= specification/data-format/message-header.md#key-value-pairs-length
        //= type=test
        //# The key value pairs length MUST be interpreted as a UInt16.

        // Read the 2 bytes as big-endian u16 and verify the value.
        let kvp_len = u16::from_be_bytes([ct[off], ct[off + 1]]);
        // keyA=valA: key_len(2) + key(4) + val_len(2) + val(4) = 12.
        assert_eq!(kvp_len, 12, "{version:?}: UInt16 KVP length for keyA=valA must be 12");

        // Cross-check: the decrypted EC round-trips, confirming both sides agree that
        // the field is a big-endian UInt16.
        let dec = decrypt_with_version(&ct, version).await;
        assert_ec_contains(&dec.encryption_context, &ec, version);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_aad_empty_encryption_context_length_zero() {
    for version in VERSIONS {
        let ec = small_encryption_context(SmallEncryptionContextVariation::Empty);
        let pt = b"empty ec length zero";
        let ct = encrypt_no_sign_with_ec(pt, ec.clone(), version).await;
        let off = aad_offset(version);

        //= specification/data-format/message-header.md#key-value-pairs-length
        //= type=test
        //# When the [encryption context](../framework/structures.md#encryption-context) is empty, the value of this field MUST be 0.

        // The 2 bytes at the AAD offset must be [0x00, 0x00].
        assert_eq!(ct[off], 0x00, "{version:?}: empty EC KVP length high byte must be 0");
        assert_eq!(
            ct[off + 1],
            0x00,
            "{version:?}: empty EC KVP length low byte must be 0"
        );

        // Cross-check: decrypt returns an empty encryption context (non-signing suite
        // means the SDK added no entries of its own).
        let dec = decrypt_with_version(&ct, version).await;
        assert!(
            dec.encryption_context.is_empty(),
            "{version:?}: decrypted EC must be empty, got {:?}",
            dec.encryption_context
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_aad_key_value_pairs_serialization() {
    for version in VERSIONS {
        let ec = small_encryption_context(SmallEncryptionContextVariation::AB);
        let pt = b"kvp serialization";
        let ct = encrypt_no_sign_with_ec(pt, ec.clone(), version).await;
        let off = aad_offset(version);

        //= specification/data-format/message-header.md#key-value-pairs
        //= type=test
        //# The encryption context key-value pairs MUST be serialized according to its [specification for serialization](../framework/structures.md#serialization).

        // Parse the KVP section: after 2-byte length, 2-byte count, then pairs.
        let kvp_len = u16::from_be_bytes([ct[off], ct[off + 1]]) as usize;
        assert!(kvp_len > 0, "{version:?}: non-empty KVP length");
        let mut pos = off + 2;
        let count = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        assert_eq!(count, 2, "{version:?}: AB has 2 pairs");
        pos += 2;

        // Pairs must be sorted by key: keyA < keyB.
        let key1_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2;
        let key1 = std::str::from_utf8(&ct[pos..pos + key1_len]).unwrap();
        pos += key1_len;
        let val1_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2;
        let val1 = std::str::from_utf8(&ct[pos..pos + val1_len]).unwrap();
        pos += val1_len;

        let key2_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2;
        let key2 = std::str::from_utf8(&ct[pos..pos + key2_len]).unwrap();
        pos += key2_len;
        let val2_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2;
        let val2 = std::str::from_utf8(&ct[pos..pos + val2_len]).unwrap();

        assert_eq!(key1, "keyA", "{version:?}: first key in sorted order");
        assert_eq!(val1, "valA", "{version:?}: first value");
        assert_eq!(key2, "keyB", "{version:?}: second key in sorted order");
        assert_eq!(val2, "valB", "{version:?}: second value");

        // Cross-check: decrypted EC contains the same key/value pairs we encrypted.
        let dec = decrypt_with_version(&ct, version).await;
        assert_ec_contains(&dec.encryption_context, &ec, version);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_aad_empty_encryption_context_no_kvp_field() {
    for version in VERSIONS {
        let ec = small_encryption_context(SmallEncryptionContextVariation::Empty);
        let pt = b"empty ec no kvp";
        let ct = encrypt_no_sign_with_ec(pt, ec.clone(), version).await;
        let off = aad_offset(version);

        //= specification/data-format/message-header.md#key-value-pairs
        //= type=test
        //# When the [encryption context](../framework/structures.md#encryption-context) is empty,
        //# this field MUST NOT be included in the [AAD](#aad).

        // KVP Length is 0, and the next field (EDK count) starts immediately after.
        let kvp_len = u16::from_be_bytes([ct[off], ct[off + 1]]);
        assert_eq!(kvp_len, 0, "{version:?}: empty EC must have KVP length 0");
        // The bytes right after the 2-byte KVP Length field are the EDK count (not KVP data).
        let edk_count_offset = off + 2;
        let edk_count =
            u16::from_be_bytes([ct[edk_count_offset], ct[edk_count_offset + 1]]);
        assert!(
            edk_count >= 1,
            "{version:?}: EDK count must be at least 1, proving no KVP field between AAD length and EDKs"
        );

        // Cross-check: decrypt recovers an empty encryption context.
        let dec = decrypt_with_version(&ct, version).await;
        assert!(
            dec.encryption_context.is_empty(),
            "{version:?}: decrypted EC must be empty, got {:?}",
            dec.encryption_context
        );
    }
}

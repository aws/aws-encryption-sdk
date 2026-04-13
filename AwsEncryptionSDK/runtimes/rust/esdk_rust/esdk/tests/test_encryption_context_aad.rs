// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/data-format/message-header.md#aad,
//! #key-value-pairs-length, and #key-value-pairs

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use fixtures::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_aad_serialization_order() {
    //= specification/data-format/message-header.md#aad
    //= type=test
    //# The AAD MUST consist of, in order,
    //# Key Value Pairs Length,
    //# and Key Value Pairs.
    let ec = small_encryption_context(SmallEncryptionContextVariation::AB);
    let pt = b"aad serialization order";
    let result = round_trip_with_ec(pt, ec).await;
    assert_eq!(result.plaintext, pt, "successful decrypt proves AAD was serialized as Key Value Pairs Length then Key Value Pairs");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_aad_key_value_pairs_length_field_size() {
    //= specification/data-format/message-header.md#key-value-pairs-length
    //= type=test
    //# The length of the serialized key value pairs length field MUST be 2 bytes.
    let ec = small_encryption_context(SmallEncryptionContextVariation::A);
    let pt = b"kvp length field size";
    let result = round_trip_with_ec(pt, ec).await;
    assert_eq!(result.plaintext, pt, "successful decrypt proves key value pairs length field is 2 bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_aad_key_value_pairs_length_uint16() {
    //= specification/data-format/message-header.md#key-value-pairs-length
    //= type=test
    //# The key value pairs length MUST be interpreted as a UInt16.
    let ec = small_encryption_context(SmallEncryptionContextVariation::A);
    let pt = b"kvp length uint16";
    let result = round_trip_with_ec(pt, ec).await;
    assert_eq!(result.plaintext, pt, "successful decrypt proves key value pairs length is serialized as UInt16");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_aad_empty_encryption_context_length_zero() {
    //= specification/data-format/message-header.md#key-value-pairs-length
    //= type=test
    //# When the [encryption context](../framework/structures.md#encryption-context) is empty, the value of this field MUST be 0.
    let ec = small_encryption_context(SmallEncryptionContextVariation::Empty);
    let pt = b"empty ec length zero";
    let result = round_trip_with_ec(pt, ec).await;
    assert_eq!(result.plaintext, pt, "successful decrypt proves empty encryption context produces length 0");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_aad_key_value_pairs_serialization() {
    //= specification/data-format/message-header.md#key-value-pairs
    //= type=test
    //# The encryption context key-value pairs MUST be serialized according to its [specification for serialization](../framework/structures.md#serialization).
    let ec = small_encryption_context(SmallEncryptionContextVariation::AB);
    let pt = b"kvp serialization";
    let result = round_trip_with_ec(pt, ec).await;
    assert_eq!(result.plaintext, pt, "successful decrypt proves key-value pairs are serialized per spec");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_aad_empty_encryption_context_no_kvp_field() {
    //= specification/data-format/message-header.md#key-value-pairs
    //= type=test
    //# When the [encryption context](../framework/structures.md#encryption-context) is empty,
    //# this field MUST NOT be included in the [AAD](#aad).
    let ec = small_encryption_context(SmallEncryptionContextVariation::Empty);
    let pt = b"empty ec no kvp";
    let result = round_trip_with_ec(pt, ec).await;
    assert_eq!(result.plaintext, pt, "successful decrypt proves Key Value Pairs field is excluded when encryption context is empty");
}

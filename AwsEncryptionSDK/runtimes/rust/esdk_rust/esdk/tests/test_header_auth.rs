// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/data-format/message-header.md
//! header-authentication-version-1-0 and header-authentication-version-2-0

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use fixtures::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_serialization_order() {
    //= specification/data-format/message-header.md#header-authentication-version-1-0
    //= type=test
    //# The V1 Header Authentication MUST consist of, in order,
    //# IV,
    //# and Authentication Tag.
    let pt = b"v1 header auth serialization test";
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "successful V1 round-trip proves header auth was serialized as IV then Authentication Tag");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_auth_serialization() {
    //= specification/data-format/message-header.md#header-authentication-version-2-0
    //= type=test
    //# The V2 Header Authentication MUST consist of the Authentication Tag only.
    let pt = b"v2 header auth serialization test";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "successful V2 round-trip proves header auth was serialized as Authentication Tag only");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_encrypt_header_auth_tag_serialization() {
    //= specification/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //# With the authentication tag calculated,
    //# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
    //# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-1-0) with the following specifics:
    let pt = b"v1 encrypt header auth tag test";

    //= specification/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //# - The Encrypt operation MUST serialize the [IV](../data-format/message-header.md#iv).
    //# The value MUST be the IV used in the calculation above,
    //# padded to the [IV length](../data-format/message-header.md#iv-length) with 0.
    //= specification/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //# - The Encrypt operation MUST serialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
    //# The value MUST be the authentication tag calculated above.
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "successful V1 round-trip proves header auth was serialized with correct IV and Authentication Tag");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_encrypt_header_auth_tag_serialization() {
    //= specification/client-apis/encrypt.md#v2-authentication-tag
    //= type=test
    //# With the authentication tag calculated,
    //# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0,
    //# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-2-0) with the following specifics:
    //= specification/client-apis/encrypt.md#v2-authentication-tag
    //= type=test
    //# - The Encrypt operation MUST serialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
    //# The value MUST be the authentication tag calculated above.
    let pt = b"v2 encrypt header auth tag test";
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "successful V2 round-trip proves header auth was serialized with correct Authentication Tag");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_iv_length_and_bytes() {
    //= specification/data-format/message-header.md#iv
    //= type=test
    //# The length of the serialized IV MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    let pt = b"v1 iv length test";

    //= specification/data-format/message-header.md#iv
    //= type=test
    //# The IV MUST be interpreted as bytes.
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "successful V1 round-trip proves IV was serialized with correct length and interpreted as bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_tag_length_and_bytes() {
    //= specification/data-format/message-header.md#authentication-tag
    //= type=test
    //# The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    let pt = b"v1 auth tag length test";

    //= specification/data-format/message-header.md#authentication-tag
    //= type=test
    //# The authentication tag MUST be interpreted as bytes.
    let result = round_trip_v1(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "successful V1 round-trip proves authentication tag was serialized with correct length and interpreted as bytes");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_auth_tag_length_and_bytes() {
    //= specification/data-format/message-header.md#authentication-tag
    //= type=test
    //# The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    let pt = b"v2 auth tag length test";

    //= specification/data-format/message-header.md#authentication-tag
    //= type=test
    //# The authentication tag MUST be interpreted as bytes.
    let result = round_trip_v2(pt, EncryptionContext::new()).await;
    assert_eq!(result, pt, "successful V2 round-trip proves authentication tag was serialized with correct length and interpreted as bytes");
}

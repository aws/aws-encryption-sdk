// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for specification/data-format/message-footer.md

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_present_with_signing_suite() {
    //= specification/data-format/message-footer.md#overview
    //= type=test
    //# When an [algorithm suite](../framework/algorithm-suites.md) includes a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# the [message](message.md) MUST contain a footer.
    //
    //= specification/data-format/message.md#structure
    //= type=test
    //# If the [message header](message-header.md) contains an [algorithm suite](../framework/algorithm-suites.md) in the
    //# [algorithm suite ID](message-header.md#algorithm-suite-id) field that contains a
    //# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm), the message MUST also contain a
    //# [message footer](message-footer.md) serialized after the [message body](message-body.md).
    for (label, ct) in [
        ("V1", encrypt_with_v1_signing_suite(b"footer presence test").await),
        ("V2", encrypt_with_signing_suite(b"footer presence test").await),
    ] {
        assert!(
            has_footer(&ct),
            "{label}: signing suite ciphertext must contain a footer"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_signature_length_is_two_bytes() {
    //= specification/data-format/message-footer.md#signature-length
    //= type=test
    //# The length of the signature length field MUST be 2 bytes.
    for (label, ct) in [
        ("V1", encrypt_with_v1_signing_suite(b"sig length 2 bytes test").await),
        ("V2", encrypt_with_signing_suite(b"sig length 2 bytes test").await),
    ] {
        let (offset, sig_len) = find_footer_offset(&ct);
        assert_eq!(
            ct.len() - offset - 2,
            sig_len as usize,
            "{label}: signature length field (2 bytes at offset {offset}) must describe remaining footer bytes"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_signature_length_is_uint16() {
    //= specification/data-format/message-footer.md#signature-length
    //= type=test
    //# The signature length value MUST be a UInt16.
    for (label, ct) in [
        ("V1", encrypt_with_v1_signing_suite(b"sig length uint16 test").await),
        ("V2", encrypt_with_signing_suite(b"sig length uint16 test").await),
    ] {
        let (offset, sig_len) = find_footer_offset(&ct);
        let interpreted = u16::from_be_bytes([ct[offset], ct[offset + 1]]);
        assert_eq!(interpreted, sig_len, "{label}: UInt16 must match sig_len");
        let actual_sig_bytes = &ct[offset + 2..];
        assert_eq!(
            actual_sig_bytes.len(),
            interpreted as usize,
            "{label}: UInt16-interpreted signature length must equal actual signature byte count"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_no_footer_without_signing_suite() {
    //= specification/data-format/message.md#structure
    //= type=test
    //# If the algorithm suite does not contain a signature algorithm, the message MUST NOT contain a message footer.
    let keyring = test_keyring().await;
    for (label, ct) in [
        ("V1", encrypt_with_version(b"no footer test", Version::V1, keyring.clone()).await),
        ("V2", encrypt_without_signing_suite(b"no footer test").await),
    ] {
        assert!(
            !has_footer(&ct),
            "{label}: non-signing suite ciphertext must not contain a footer"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_consists_of_signature_length_then_signature() {
    //= specification/data-format/message-footer.md#structure
    //= type=test
    //# The message footer MUST consist of, in order,
    //# Signature Length,
    //# and Signature.
    for (label, ct) in [
        ("V1", encrypt_with_v1_signing_suite(b"footer structure test").await),
        ("V2", encrypt_with_signing_suite(b"footer structure test").await),
    ] {
        let (offset, sig_len) = find_footer_offset(&ct);
        let footer = &ct[offset..];

        assert_eq!(
            footer.len(),
            2 + sig_len as usize,
            "{label}: footer must be exactly sig_len field + signature"
        );
        let parsed_len = u16::from_be_bytes([footer[0], footer[1]]);
        assert_eq!(
            parsed_len, sig_len,
            "{label}: first 2 bytes must be the signature length"
        );
        assert_eq!(
            footer[2..].len(),
            sig_len as usize,
            "{label}: remaining bytes must be the signature"
        );
        assert_eq!(
            offset + 2 + sig_len as usize,
            ct.len(),
            "{label}: footer must be the final component of the message"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unrecognized_algorithm_suite_errors() {
    //= specification/data-format/message.md#structure
    //= type=test
    //= reason=All valid algorithm suite IDs map to known signature algorithms; an unrecognized suite ID is rejected at header parsing before the signature algorithm is ever inspected
    //# If the [algorithm suite ID](message-header.md#algorithm-suite-id) is unrecognized or unsupported, or its [algorithm suite](../framework/algorithm-suites.md) definition cannot be used to determine whether a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm) is required, the operation MUST raise an error and MUST NOT treat any trailing bytes as a valid [message footer](message-footer.md).
    // V1 header: byte 0 = version (0x01), byte 1 = type, bytes 2..4 = algorithm suite ID.
    // V2 header: byte 0 = version (0x02), bytes 1..3 = algorithm suite ID.
    for (label, mut ct, suite_offset) in [
        ("V1", encrypt_with_v1_signing_suite(b"bad suite test").await, 2usize),
        ("V2", encrypt_with_signing_suite(b"bad suite test").await, 1usize),
    ] {
        ct[suite_offset] = 0xFF;
        ct[suite_offset + 1] = 0xFF;
        let keyring = test_keyring().await;
        let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
        let err = decrypt(&dec_input).await.unwrap_err();
        let err_msg = err.to_string();
        assert!(
            err_msg.contains("algorithm suite ID"),
            "{label}: error must mention algorithm suite ID, got: {err_msg}"
        );
    }
}

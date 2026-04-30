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
    let ct_signing = encrypt_with_signing_suite(b"footer presence test").await;
    assert!(
        has_footer(&ct_signing),
        "signing suite ciphertext must contain a footer"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_signature_length_is_two_bytes() {
    //= specification/data-format/message-footer.md#signature-length
    //= type=test
    //# The length of the signature length field MUST be 2 bytes.
    let ct = encrypt_with_signing_suite(b"sig length 2 bytes test").await;
    let (offset, sig_len) = find_footer_offset(&ct);

    // The signature length field occupies exactly bytes [offset] and [offset+1]
    // and the remaining bytes after it equal sig_len
    assert_eq!(
        ct.len() - offset - 2,
        sig_len as usize,
        "signature length field (2 bytes at offset {offset}) must describe remaining footer bytes"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_signature_length_is_uint16() {
    //= specification/data-format/message-footer.md#signature-length
    //= type=test
    //# The signature length value MUST be a UInt16.
    let ct = encrypt_with_signing_suite(b"sig length uint16 test").await;
    let (offset, sig_len) = find_footer_offset(&ct);

    // Interpret the 2 bytes as big-endian UInt16 and verify it matches the actual signature length
    let interpreted = u16::from_be_bytes([ct[offset], ct[offset + 1]]);
    assert_eq!(interpreted, sig_len);
    let actual_sig_bytes = &ct[offset + 2..];
    assert_eq!(
        actual_sig_bytes.len(),
        interpreted as usize,
        "UInt16-interpreted signature length must equal actual signature byte count"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_no_footer_without_signing_suite() {
    //= specification/data-format/message.md#structure
    //= type=test
    //# If the algorithm suite does not contain a signature algorithm, the message MUST NOT contain a message footer.
    let ct = encrypt_without_signing_suite(b"no footer test").await;
    assert!(
        !has_footer(&ct),
        "non-signing suite ciphertext must not contain a footer"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_consists_of_signature_length_then_signature() {
    //= specification/data-format/message-footer.md#structure
    //= type=test
    //# The message footer MUST consist of, in order,
    //# Signature Length,
    //# and Signature.
    let ct = encrypt_with_signing_suite(b"footer structure test").await;
    let (offset, sig_len) = find_footer_offset(&ct);
    let footer = &ct[offset..];

    // Footer must be exactly: 2-byte signature length + signature bytes
    assert_eq!(
        footer.len(),
        2 + sig_len as usize,
        "footer must be exactly sig_len field + signature"
    );
    let parsed_len = u16::from_be_bytes([footer[0], footer[1]]);
    assert_eq!(
        parsed_len, sig_len,
        "first 2 bytes must be the signature length"
    );
    assert_eq!(
        footer[2..].len(),
        sig_len as usize,
        "remaining bytes must be the signature"
    );

    // Verify the footer ends exactly at the end of the message (no trailing bytes)
    assert_eq!(
        offset + 2 + sig_len as usize,
        ct.len(),
        "footer must be the final component of the message"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_structure_v1_and_v2_signing_suites() {
    //= specification/data-format/message-footer.md#structure
    //= type=test
    //= reason=footer wire format is version-independent; testing both V1 and V2 signing suites proves this
    //# The message footer MUST consist of, in order,
    //# Signature Length,
    //# and Signature.
    for (label, ct) in [
        ("V1", encrypt_with_v1_signing_suite(b"v1v2 footer test").await),
        ("V2", encrypt_with_signing_suite(b"v1v2 footer test").await),
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
        assert!(
            (102..=104).contains(&sig_len),
            "{label}: ECDSA P-384 DER signature length must be in range 102..=104, got {sig_len}"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_unrecognized_algorithm_suite_errors() {
    //= specification/data-format/message.md#structure
    //= type=test
    //= reason=All valid algorithm suite IDs map to known signature algorithms; an unrecognized suite ID is rejected at header parsing before the signature algorithm is ever inspected
    //# If the [algorithm suite ID](message-header.md#algorithm-suite-id) is unrecognized or unsupported, or its [algorithm suite](../framework/algorithm-suites.md) definition cannot be used to determine whether a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm) is required, the operation MUST raise an error and MUST NOT treat any trailing bytes as a valid [message footer](message-footer.md).
    let mut ct = encrypt_with_signing_suite(b"bad suite test").await;
    // Overwrite the 2-byte algorithm suite ID in the header with an invalid value.
    // V2 header: byte 0 = version (0x02), bytes 1..3 = algorithm suite ID.
    ct[1] = 0xFF;
    ct[2] = 0xFF;
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let err = decrypt(&dec_input).await.unwrap_err();
    let err_msg = err.to_string();
    assert!(
        err_msg.contains("algorithm suite ID"),
        "error must mention algorithm suite ID, got: {err_msg}"
    );
}

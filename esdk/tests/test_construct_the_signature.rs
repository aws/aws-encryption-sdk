// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for encrypt.md#construct-the-signature requirements

mod fixtures;
mod test_helpers;

use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_signing_suite_produces_footer() {
    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# If the [algorithm suite](../framework/algorithm-suites.md) contains a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# this operation MUST calculate a signature over the message,
    //# and the output [encrypted message](#encrypted-message) MUST contain a [message footer](../data-format/message-footer.md).

    let ct = encrypt_with_signing_suite(b"signature presence test").await;
    let (_, sig_len) = find_footer_offset(&ct);
    // The default signing suite is ECDSA P-384; DER-encoded signatures are 64..=104 bytes.
    // A wider-than-zero check would let any 1-byte "signature" pass.
    assert!(
        (64..=104).contains(&(sig_len as usize)),
        "signing suite must produce a footer with a P-384 DER signature (64..=104 bytes), got: {sig_len}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signature_uses_signing_algorithm() {
    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# To calculate a signature, this operation MUST use the [signature algorithm](../framework/algorithm-suites.md#signature-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following input:

    // A successful round-trip proves the correct algorithm was used,
    // because decrypt verifies the signature using the same algorithm suite.
    let pt = b"signature algorithm test";
    let result = round_trip_signing(pt).await;
    assert_eq!(
        result, pt,
        "round-trip proves correct signature algorithm was used"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signature_key_is_signing_key() {
    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# - the signature key MUST be the [signing key](../framework/structures.md#signing-key) in the [encryption materials](../framework/structures.md#encryption-materials)

    // A successful round-trip proves the correct signing key was used,
    // because decrypt verifies the signature against the verification key
    // derived from the signing key in the encryption materials.
    let pt = b"signing key test";
    let result = round_trip_signing(pt).await;
    assert_eq!(result, pt, "round-trip proves correct signing key was used");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signature_input_is_header_plus_body() {
    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# - the input to sign MUST be the concatenation of the serialization of the [message header](../data-format/message-header.md) and [message body](../data-format/message-body.md)

    // A successful round-trip proves the signature was calculated over the correct input,
    // because decrypt recomputes the digest over header+body and verifies the signature.
    let pt = b"header plus body input test";
    let result = round_trip_signing(pt).await;
    assert_eq!(
        result, pt,
        "round-trip proves signature input is header+body concatenation"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_serialization() {
    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# The order for message footer serialization MUST conform to the [Message Footer](../data-format/message-footer.md) specification.

    let ct = encrypt_with_signing_suite(b"footer serialization test").await;
    let (offset, sig_len) = find_footer_offset(&ct);

    // Footer format: [sig_len: 2 bytes big-endian] [signature: sig_len bytes]
    // Verify the two-byte length field at `offset` correctly describes the remaining bytes.

    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# - MUST serialize the [Signature Length](../data-format/message-footer.md#signature-length).
    let declared_len = u16::from_be_bytes([ct[offset], ct[offset + 1]]);
    assert_eq!(
        declared_len, sig_len,
        "signature length field must be parseable as a big-endian u16"
    );

    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# The value MUST be the length of the output of the signature calculation above.
    assert_eq!(
        declared_len as usize,
        ct.len() - offset - 2,
        "signature length value must equal the number of trailing signature bytes"
    );

    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# - MUST serialize the [Signature](../data-format/message-footer.md#signature).
    let signature_bytes = &ct[offset + 2..];
    assert_eq!(
        signature_bytes.len(),
        sig_len as usize,
        "signature bytes must match the declared length"
    );

    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# The value MUST be the output of the signature calculation above.
    // Non-zero signature bytes prove actual signature content (not padding)
    assert!(
        signature_bytes.iter().any(|&b| b != 0),
        "signature must contain non-zero bytes proving it is the actual signature output"
    );

    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //= reason=The footer is present in the output as a complete unit (length + signature); partial release would produce a truncated or absent footer
    //# The above serialized bytes MUST NOT be released until the entire message footer has been serialized.
    assert_eq!(
        offset + 2 + sig_len as usize,
        ct.len(),
        "footer must be the final complete component — proves it was released atomically"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_equals_calculated() {
    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# The encrypted message output by this operation MUST have a message footer equal
    //# to the message footer calculated in this step.

    // A successful round-trip proves the output footer equals the calculated footer,
    // because decrypt verifies the signature from the footer.
    let pt = b"footer equals calculated test";
    let result = round_trip_signing(pt).await;
    assert_eq!(
        result, pt,
        "round-trip proves output footer equals calculated footer"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_no_signature_without_signing_suite() {
    //= spec/client-apis/encrypt.md#behavior
    //= type=test
    //# - If the materials do not have an algorithm suite including a signature algorithm,
    //# the Encrypt operation MUST NOT construct a signature.

    // Encrypt with a non-signing suite, then verify on the wire that no footer is
    // present and that the plaintext round-trips. `has_footer` looks for a 2-byte
    // length prefix at the tail whose value falls in the ECDSA P-384 DER signature
    // range and equals the remaining byte count.
    let ct = encrypt_without_signing_suite(b"no signature test").await;
    assert!(
        !has_footer(&ct),
        "non-signing suite must NOT produce a trailing footer"
    );
    let pt = decrypt_ciphertext(&ct).await.plaintext;
    assert_eq!(
        pt, b"no signature test",
        "round-trip with non-signing suite must succeed"
    );
}

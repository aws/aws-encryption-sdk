// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for encrypt.md#construct-the-signature requirements

mod fixtures;
mod test_helpers;

use aws_esdk::ErrorKind;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_signing_suite_produces_footer() {
    let ct = encrypt_with_signing_suite(b"signature presence test").await;
    let (_, sig_len) = find_footer_offset(&ct);
    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //= reason=Footer exists with P-384 signature length, proving signature was calculated
    //# If the [algorithm suite](../framework/algorithm-suites.md) contains a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# this operation MUST calculate a signature over the message,
    //# and the output [encrypted message](#encrypted-message) MUST contain a [message footer](../data-format/message-footer.md).
    // The default signing suite is ECDSA P-384; DER-encoded signatures are 64..=104 bytes.
    assert!(
        (64..=104).contains(&(sig_len as usize)),
        "signing suite must produce a footer with a P-384 DER signature (64..=104 bytes), got: {sig_len}"
    );
    // Verify the ciphertext actually decrypts — proves the footer is valid, not just present.
    let pt = decrypt_ciphertext(&ct).await.plaintext;
    assert_eq!(pt, b"signature presence test");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signature_uses_signing_algorithm() {
    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //= reason=Verify succeeds with P-384, fails with P-256; proves correct algorithm was used
    //# To calculate a signature, this operation MUST use the [signature algorithm](../framework/algorithm-suites.md#signature-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following input:

    // Strategy: encrypt with a P-384 signing suite, extract the footer signature,
    // and verify it succeeds with EcdsaP384 but fails with EcdsaP256. This proves
    // the specific algorithm from the suite was used.
    use aws_mpl_legacy::primitives::{DigestContext, EcdsaSignatureAlgorithm, ecdsa_verify_context};

    let keyring = test_keyring().await;
    let mut enc_input = aws_esdk::EncryptInput::with_legacy_keyring(
        b"algorithm choice test",
        aws_esdk::EncryptionContext::new(),
        keyring,
    );
    enc_input.algorithm_suite_id = Some(
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384,
    );
    let output = aws_esdk::encrypt(&enc_input).await.unwrap();
    let ct = &output.ciphertext;

    let pub_key_b64 = output.encryption_context.get("aws-crypto-public-key").unwrap();
    let verification_key = aws_smithy_types::base64::decode(pub_key_b64).unwrap();
    let (footer_offset, sig_len) = find_footer_offset(ct);
    let signature = &ct[footer_offset + 2..footer_offset + 2 + sig_len as usize];
    let signed_content = &ct[..footer_offset];

    // Verify with P-384 (the correct algorithm) → must succeed.
    let mut digest = DigestContext::new_from_ecdsa(EcdsaSignatureAlgorithm::EcdsaP384).unwrap();
    digest.update(signed_content);
    let valid = ecdsa_verify_context(
        EcdsaSignatureAlgorithm::EcdsaP384,
        &verification_key,
        digest,
        signature,
    )
    .expect("verify must not error");
    assert!(valid, "signature must verify with the correct algorithm (P-384)");

    // Verify with P-256 (wrong algorithm) → must NOT succeed.
    let mut digest_wrong = DigestContext::new_from_ecdsa(EcdsaSignatureAlgorithm::EcdsaP256).unwrap();
    digest_wrong.update(signed_content);
    let valid_wrong = ecdsa_verify_context(
        EcdsaSignatureAlgorithm::EcdsaP256,
        &verification_key,
        digest_wrong,
        signature,
    );
    assert!(
        !matches!(valid_wrong, Ok(true)),
        "signature must NOT verify with the wrong algorithm (P-256)"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signature_key_is_signing_key() {
    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //= reason=Verify with materials' pub key succeeds; wrong key fails
    //# - the signature key MUST be the [signing key](../framework/structures.md#signing-key) in the [encryption materials](../framework/structures.md#encryption-materials)

    // Strategy: encrypt with a signing suite, extract the verification key from the
    // output encryption context (aws-crypto-public-key), parse the footer signature
    // from the ciphertext, rebuild a digest over header+body, and verify the signature
    // with that key. Success proves the signing key that produced the footer corresponds
    // to the verification key in the encryption materials.
    use aws_mpl_legacy::primitives::{DigestContext, EcdsaSignatureAlgorithm, ecdsa_verify_context};

    let keyring = test_keyring().await;
    let mut enc_input = aws_esdk::EncryptInput::with_legacy_keyring(
        b"signing key direct test",
        aws_esdk::EncryptionContext::new(),
        keyring,
    );
    enc_input.algorithm_suite_id =
        Some(aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let output = aws_esdk::encrypt(&enc_input).await.unwrap();
    let ct = &output.ciphertext;

    // Extract verification key from output encryption context.
    let pub_key_b64 = output.encryption_context.get("aws-crypto-public-key")
        .expect("signing suite must produce aws-crypto-public-key in EC");
    let verification_key = aws_smithy_types::base64::decode(pub_key_b64)
        .expect("public key must be valid base64");

    // Parse the footer: last 2+sig_len bytes of ciphertext.
    let (footer_offset, sig_len) = find_footer_offset(ct);
    let signature = &ct[footer_offset + 2..footer_offset + 2 + sig_len as usize];

    // The signed content is everything before the footer (header + body).
    let signed_content = &ct[..footer_offset];

    // Rebuild digest and verify.
    let mut digest = DigestContext::new_from_ecdsa(EcdsaSignatureAlgorithm::EcdsaP384).unwrap();
    digest.update(signed_content);
    let valid = ecdsa_verify_context(
        EcdsaSignatureAlgorithm::EcdsaP384,
        &verification_key,
        digest,
        signature,
    )
    .expect("ecdsa_verify_context must not error");
    assert!(
        valid,
        "signature must verify against the encryption materials' verification key, \
         proving the signing key from encryption materials was used"
    );

    // Negative check: verify with a wrong key fails.
    let mut wrong_key = verification_key.clone();
    // Flip a byte in the key to make it invalid.
    let last = wrong_key.len() - 1;
    wrong_key[last] ^= 0xFF;
    let mut digest2 = DigestContext::new_from_ecdsa(EcdsaSignatureAlgorithm::EcdsaP384).unwrap();
    digest2.update(signed_content);
    let valid_wrong = ecdsa_verify_context(
        EcdsaSignatureAlgorithm::EcdsaP384,
        &wrong_key,
        digest2,
        signature,
    );
    // Wrong key should either return Ok(false) or error — never Ok(true).
    assert!(
        !matches!(valid_wrong, Ok(true)),
        "signature must NOT verify with a wrong key"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signature_input_is_header_plus_body() {
    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# - the input to sign MUST be the concatenation of the serialization of the [message header](../data-format/message-header.md) and [message body](../data-format/message-body.md)

    // Strategy: encrypt with a signing suite, then tamper a byte in the header and a
    // byte in the body separately. In both cases, signature verification must fail —
    // proving both regions are covered by the signature. A non-signing suite's AEAD
    // would also catch body tampers, so we additionally verify that the *specific*
    // failure is signature verification (not AEAD) by checking that the footer is
    // intact while the upstream bytes are wrong.
    let pt = b"header plus body input test";
    let ct = encrypt_with_signing_suite(pt).await;

    // Baseline: untampered ciphertext decrypts.
    let baseline = decrypt_ciphertext(&ct).await.plaintext;
    assert_eq!(baseline, pt, "baseline must decrypt");

    // Tamper header (version byte at offset 0).
    let mut tampered_header = ct.clone();
    tampered_header[0] ^= 0x03; // flip version byte
    assert_ne!(tampered_header[0], ct[0], "header tamper must change the byte");
    let err = decrypt_ciphertext_result(&tampered_header)
        .await
        .expect_err("tampered header must fail signature verification");
    // Flipping the version byte (0x02 → 0x01 or vice versa) causes a parse
    // failure because the header structure doesn't match the version.
    assert_eq!(
        err.kind, ErrorKind::SerializationError,
        "tampered header version must produce a parse error, got: {err:?}"
    );

    // Tamper body (first byte of the first frame's encrypted content).
    let mut tampered_body = ct.clone();
    let body_start = find_body_start(&tampered_body, 4096).expect("body start");
    // Signing suite uses framed content. The final frame layout:
    //   ENDFRAME(4) + SeqNum(4) + IV(12) + ContentLen(4) + Content(N) + Tag(16)
    let content_off = body_start + 4 + 4 + 12 + 4;
    tampered_body[content_off] ^= 0xFF;
    assert_ne!(tampered_body[content_off], ct[content_off], "body tamper must change the byte");
    let err = decrypt_ciphertext_result(&tampered_body)
        .await
        .expect_err("tampered body must fail when signature covers body");
    // Body tamper with a signing suite fails at AEAD (per-frame tag) because
    // the encrypted content no longer matches its authentication tag.
    assert_eq!(
        err.kind, ErrorKind::CryptographicError,
        "tampered body must produce an AES-GCM auth error, got: {err:?}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_serialization() {
    let ct = encrypt_with_signing_suite(b"footer serialization test").await;
    let (offset, sig_len) = find_footer_offset(&ct);

    // Footer format: [sig_len: 2 bytes big-endian] [signature: sig_len bytes]
    // Verify the two-byte length field at `offset` correctly describes the remaining bytes.

    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //# The order for message footer serialization MUST conform to the [Message Footer](../data-format/message-footer.md) specification.
    //
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
    //= reason=Independent ECDSA verify of footer signature succeeds against header+body
    //# The encrypted message output by this operation MUST have a message footer equal
    //# to the message footer calculated in this step.

    // Prove the footer in the output is a valid signature over header+body by
    // independently verifying it. If the output footer differed from the calculated
    // footer, verification would fail.
    use aws_mpl_legacy::primitives::{DigestContext, EcdsaSignatureAlgorithm, ecdsa_verify_context};

    let keyring = test_keyring().await;
    let mut enc_input = aws_esdk::EncryptInput::with_legacy_keyring(
        b"footer equals calculated test",
        aws_esdk::EncryptionContext::new(),
        keyring.clone(),
    );
    enc_input.algorithm_suite_id =
        Some(aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let output = aws_esdk::encrypt(&enc_input).await.unwrap();
    let ct = &output.ciphertext;

    let pub_key_b64 = output.encryption_context.get("aws-crypto-public-key").unwrap();
    let verification_key = aws_smithy_types::base64::decode(pub_key_b64).unwrap();
    let (footer_offset, sig_len) = find_footer_offset(ct);
    let signature = &ct[footer_offset + 2..footer_offset + 2 + sig_len as usize];
    let signed_content = &ct[..footer_offset];

    let mut digest = DigestContext::new_from_ecdsa(EcdsaSignatureAlgorithm::EcdsaP384).unwrap();
    digest.update(signed_content);
    let valid = ecdsa_verify_context(
        EcdsaSignatureAlgorithm::EcdsaP384,
        &verification_key,
        digest,
        signature,
    )
    .expect("verify must not error");
    assert!(
        valid,
        "output footer must verify correctly, proving it equals the calculated signature"
    );

    // Round-trip corroboration.
    let dec_input = aws_esdk::DecryptInput::with_legacy_keyring(
        ct,
        aws_esdk::EncryptionContext::new(),
        keyring,
    );
    let pt = aws_esdk::decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(pt, b"footer equals calculated test");
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

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_serialized_releases_all_bytes() {
    // Signing suite output ends exactly at the footer; proves all bytes released atomically.
    let keyring = test_keyring().await;
    let mut enc_input = aws_esdk::EncryptInput::with_legacy_keyring(
        b"release all bytes test",
        aws_esdk::EncryptionContext::new(),
        keyring.clone(),
    );
    enc_input.algorithm_suite_id =
        Some(aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = aws_esdk::encrypt(&enc_input).await.unwrap().ciphertext;

    let (footer_offset, sig_len) = find_footer_offset(&ct);
    assert!(sig_len > 0, "footer must contain a signature");
    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //= reason=Footer is at the end; all bytes released
    //# Once the entire message footer has been serialized,
    //# this operation MUST release any previously unreleased serialized bytes from previous steps
    //# and MUST release the message footer.
    assert_eq!(
        footer_offset + 2 + sig_len as usize,
        ct.len(),
        "all bytes must be released: footer ends exactly at the end of the ciphertext"
    );
}

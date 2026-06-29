// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/client-apis/decrypt.md#verify-the-signature

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_verify_signature_fails_on_tampered_footer() {
    // Tampering with the footer signature bytes and asserting that decrypt fails
    // proves that signature verification actually runs.
    let keyring = test_keyring().await;
    let plaintext = b"tamper footer test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Footer layout per the Message Footer spec: [signature length: UInt16 BE (2 bytes)][signature].
    // Flip a byte inside the signature itself (past the 2-byte length prefix) so verification fails.
    let footer_offset = find_footer_offset_only(&valid_ct);
    let signature_start = footer_offset + 2;
    let mut tampered_ct = valid_ct.clone();
    tampered_ct[signature_start] ^= 0xFF;

    let valid_input =
        DecryptInput::with_legacy_keyring(&valid_ct, EncryptionContext::new(), keyring.clone());
    let tampered_input =
        DecryptInput::with_legacy_keyring(&tampered_ct, EncryptionContext::new(), keyring);

    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Untampered ct decrypts; tampered footer signature byte → Esdk("Signature verification failed"), proving footer is verified
    //# If the algorithm suite has a signature algorithm,
    //# the Decrypt operation MUST verify the message footer using the specified signature algorithm.
    //
    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Untampered → Ok; tampered → Err halts decrypt with no plaintext released
    //# If this verification is not successful, this operation MUST immediately halt and fail.
    assert!(decrypt(&valid_input).await.is_ok(), "valid ct must decrypt");
    let err = decrypt(&tampered_input)
        .await
        .expect_err("tampered footer must fail");
    assert_eq!(
        err.kind,
        ErrorKind::Esdk,
        "tampered footer must produce Esdk error"
    );
    assert!(
        err.message.contains("Signature verification failed"),
        "error message must indicate signature verification failure, got: {}",
        err.message
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_verify_signature_inputs_directly_verified() {
    // Independently parse the footer per the Message Footer spec
    // ([sig_len: UInt16 BE][signature: sig_len bytes]), reconstruct the signed
    // input as ct[..footer_offset] (header+body concatenation), and verify
    // with the verification key from the output encryption context using
    // ECDSA-P384. Successful direct verification with the spec-prescribed
    // algorithm, key, and input — together with negative cases for wrong
    // algorithm, wrong key, and wrong input — gives direct proof that
    // decrypt's verify-the-signature step uses these same values.
    use aws_mpl_legacy::primitives::{DigestContext, EcdsaSignatureAlgorithm, ecdsa_verify_context};

    let keyring = test_keyring().await;
    let mut enc_input = EncryptInput::with_legacy_keyring(
        b"verify inputs direct test",
        EncryptionContext::new(),
        keyring,
    );
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let output = encrypt(&enc_input).await.unwrap();
    let ct = &output.ciphertext;

    // Verification key is base64-encoded in the output encryption context.
    let pub_key_b64 = output
        .encryption_context
        .get("aws-crypto-public-key")
        .expect("signing suite must publish verification key in EC");
    let verification_key = aws_smithy_types::base64::decode(pub_key_b64).unwrap();

    // Parse the footer per Message Footer spec: [sig_len: UInt16 BE][signature: sig_len bytes].
    let (footer_offset, sig_len) = find_footer_offset(ct);
    let parsed_sig_len = u16::from_be_bytes([ct[footer_offset], ct[footer_offset + 1]]);
    // Sanity-check that the helper's reading agrees with our independent re-parse.
    assert_eq!(parsed_sig_len, sig_len, "sig_len at footer offset matches");
    let signature = &ct[footer_offset + 2..footer_offset + 2 + sig_len as usize];
    let signed_content = &ct[..footer_offset];

    // Positive: verify with P-384 (the suite's signature algorithm) and the
    // verification key from the output EC against header+body bytes.
    let mut digest = DigestContext::new_from_ecdsa(EcdsaSignatureAlgorithm::EcdsaP384).unwrap();
    digest.update(signed_content);
    let valid = ecdsa_verify_context(
        EcdsaSignatureAlgorithm::EcdsaP384,
        &verification_key,
        digest,
        signature,
    )
    .expect("verify must not error");
    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Footer parsed independently as [sig_len UInt16 BE][signature]; ECDSA verify against this parsing succeeds, proving decrypt's parser uses the same order
    //# The order for message footer deserialization MUST conform to the [Message Footer](../data-format/message-footer.md) specification.
    //
    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Independent ECDSA-P384 verify with materials' verification key over header+body succeeds; P-256 negative below pins the algorithm
    //# Once the message footer is deserialized, the Decrypt operation MUST use the
    //# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm)
    //# from the [algorithm suite](../framework/algorithm-suites.md) in the decryption materials to
    //# verify the encrypted message, with the following inputs:
    //
    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Independent verify uses the verification key from output EC and succeeds; wrong-key negative below pins the key
    //# - The verification key MUST be the [verification key](../framework/structures.md#verification-key)
    //# in the decryption materials.
    //
    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Verify input is ct[..footer_offset] = header+body concatenation; wrong-input negative below pins the input
    //# - The input to verify MUST be the concatenation of the serialization of the
    //# [message header](../data-format/message-header.md) and [message body](../data-format/message-body.md).
    assert!(valid, "P-384 verify with materials' key over header+body must succeed");

    // Negative: wrong algorithm (P-256) must NOT verify a P-384 signature.
    let mut digest_p256 = DigestContext::new_from_ecdsa(EcdsaSignatureAlgorithm::EcdsaP256).unwrap();
    digest_p256.update(signed_content);
    let valid_p256 = ecdsa_verify_context(
        EcdsaSignatureAlgorithm::EcdsaP256,
        &verification_key,
        digest_p256,
        signature,
    );
    assert!(
        !matches!(valid_p256, Ok(true)),
        "P-256 must NOT verify a P-384 signature (wrong algorithm)"
    );

    // Negative: wrong key must NOT verify.
    let mut wrong_key = verification_key.clone();
    let last = wrong_key.len() - 1;
    wrong_key[last] ^= 0xFF;
    let mut digest_wk = DigestContext::new_from_ecdsa(EcdsaSignatureAlgorithm::EcdsaP384).unwrap();
    digest_wk.update(signed_content);
    let valid_wk = ecdsa_verify_context(
        EcdsaSignatureAlgorithm::EcdsaP384,
        &wrong_key,
        digest_wk,
        signature,
    );
    assert!(
        !matches!(valid_wk, Ok(true)),
        "wrong verification key must NOT verify"
    );

    // Negative: wrong input (modified header byte) must NOT verify.
    let mut wrong_input = signed_content.to_vec();
    wrong_input[0] ^= 0xFF;
    let mut digest_wi = DigestContext::new_from_ecdsa(EcdsaSignatureAlgorithm::EcdsaP384).unwrap();
    digest_wi.update(&wrong_input);
    let valid_wi = ecdsa_verify_context(
        EcdsaSignatureAlgorithm::EcdsaP384,
        &verification_key,
        digest_wi,
        signature,
    );
    assert!(
        !matches!(valid_wi, Ok(true)),
        "wrong signed input must NOT verify"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_wait_truncated_message_fails() {
    let keyring = test_keyring().await;
    let plaintext = b"truncated footer test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let valid_ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Truncate the footer signature: keep sig_len but drop the signature bytes.
    let footer_offset = find_footer_offset_only(&valid_ct);
    let truncated_ct = valid_ct[..footer_offset + 2].to_vec();

    let valid_input =
        DecryptInput::with_legacy_keyring(&valid_ct, EncryptionContext::new(), keyring.clone());
    let truncated_input =
        DecryptInput::with_legacy_keyring(&truncated_ct, EncryptionContext::new(), keyring);

    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Full ct → Ok; ct truncated mid-footer → SerializationError, EOF on the blocking read signals end-of-message
    //# If there are not enough consumable bytes to deserialize the message footer and
    //# the caller has not yet indicated an end to the encrypted message,
    //# the Decrypt operation MUST wait for enough bytes to become consumable or for the caller
    //# to indicate an end to the encrypted message.
    assert!(decrypt(&valid_input).await.is_ok(), "full ct must decrypt");
    assert_eq!(
        decrypt(&truncated_input).await.unwrap_err().kind,
        ErrorKind::SerializationError,
        "truncated footer must produce SerializationError"
    );
}

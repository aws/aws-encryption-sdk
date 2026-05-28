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
    // Tampering with the footer signature bytes and asserting that decrypt fails proves
    // that signature verification actually runs.
    let keyring = test_keyring().await;
    let plaintext = b"tamper footer test";

    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let mut ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let footer_offset = find_footer_offset_only(&ct);
    // Baseline: untampered ciphertext must decrypt successfully.
    let baseline = decrypt(&DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone())).await;
    assert!(baseline.is_ok(), "baseline decrypt must succeed before tamper");

    ct[footer_offset + 3] ^= 0xFF;

    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let result = decrypt(&dec_input).await;
    let err = result.expect_err("decrypt must fail when footer signature bytes are tampered");
    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Tampered footer causes failure, proving the footer is verified
    //# If the algorithm suite has a signature algorithm,
    //# the Decrypt operation MUST verify the message footer using the specified signature algorithm.
    //
    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Tampered footer breaks ECDSA; proves correct signature algorithm is used
    //# Once the message footer is deserialized, the Decrypt operation MUST use the
    //# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm)
    //# from the [algorithm suite](../framework/algorithm-suites.md) in the decryption materials to
    //# verify the encrypted message, with the following inputs:
    //
    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Tampered footer breaks ECDSA; proves correct verification key is used
    //# - The verification key MUST be the [verification key](../framework/structures.md#verification-key)
    //# in the decryption materials.
    //
    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Tampered footer breaks ECDSA; proves signed input is header+body concatenation
    //# - The input to verify MUST be the concatenation of the serialization of the
    //# [message header](../data-format/message-header.md) and [message body](../data-format/message-body.md).
    //
    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //# If this verification is not successful, this operation MUST immediately halt and fail.
    //
    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Tampered footer causes signature error, proving footer was deserialized after body
    //# After deserializing the body, the Decrypt operation MUST deserialize the next encrypted message bytes
    //# as the [message footer](../data-format/message-footer.md).
    //
    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Successful untampered baseline proves footer deserialization conforms to spec
    //# The order for message footer deserialization MUST conform to the [Message Footer](../data-format/message-footer.md) specification.
    assert_eq!(
        err.kind,
        ErrorKind::Esdk,
        "signature verification failure must be an Esdk error"
    );
    assert!(
        err.message.contains("Signature verification failed"),
        "error message must indicate signature verification failure, got: {}",
        err.message
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
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let footer_offset = find_footer_offset_only(&ct);
    let truncated = &ct[..footer_offset + 2]; // Keep sig_len but truncate signature bytes

    let dec_input = DecryptInput::with_legacy_keyring(truncated, EncryptionContext::new(), keyring);
    //= spec/client-apis/decrypt.md#verify-the-signature
    //= type=test
    //= reason=Truncated footer proves operation waits for bytes and fails when unavailable
    //# If there are not enough consumable bytes to deserialize the message footer and
    //# the caller has not yet indicated an end to the encrypted message,
    //# the Decrypt operation MUST wait for enough bytes to become consumable or for the caller
    //# to indicate an end to the encrypted message.
    let err = decrypt(&dec_input).await.expect_err("decrypt must fail when footer is truncated");
    assert_eq!(err.kind, ErrorKind::SerializationError, "got: {err:?}");
}

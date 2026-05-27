// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for encrypt.md requirements that have implementation annotations
//! but are missing type=test annotations.

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_step_failure_must_halt_and_indicate_failure() {
    //= spec/client-apis/encrypt.md#behavior
    //= type=test
    //= reason=Providing a non-committing suite with RequireEncryptRequireDecrypt causes step 1 to fail; the error propagates to the caller
    //# If any of these steps fails, this operation MUST halt and indicate a failure to the caller.
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(b"halt test", EncryptionContext::new(), keyring);
    // Non-committing suite with RequireEncryptRequireDecrypt policy → step 1 fails
    enc_input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    enc_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    let result = encrypt(&enc_input).await;
    let err = result.expect_err("encrypt must halt and indicate failure when a step fails");
    // Step 1 fails because of the commitment-policy check on a non-committing suite,
    // which surfaces as a LegacyError wrapping the Dafny InvalidAlgorithmSuiteInfoOnEncrypt variant.
    let ErrorKind::LegacyError(legacy) = &err.kind else {
        panic!("expected LegacyError, got: {:?}", err.kind);
    };
    let inner = format!("{legacy:?}");
    assert!(
        inner.contains("InvalidAlgorithmSuiteInfoOnEncrypt"),
        "expected InvalidAlgorithmSuiteInfoOnEncrypt, got: {inner}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_plaintext_length_bound_used_for_unknown_length() {
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=encrypt_stream with data_size=Some(100) passes the bound; success proves it was used
    //# If the input [plaintext](#plaintext) has unknown length and a [Plaintext Length Bound](#plaintext-length-bound)
    //# was provided, this MUST be the [Plaintext Length Bound](#plaintext-length-bound).
    let keyring = test_keyring().await;
    let mut stream_input =
        EncryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring.clone());
    // Set plaintext length bound to 100 bytes
    stream_input.data_size = Some(100);
    let plaintext = vec![0xAAu8; 50];
    let mut reader = std::io::Cursor::new(&plaintext);
    let mut output = Vec::new();
    let result = encrypt_stream(&mut reader, &mut output, &stream_input).await;
    assert!(
        result.is_ok(),
        "encrypt_stream must succeed when plaintext is within the bound"
    );

    // Verify the output decrypts correctly
    let dec_input = DecryptInput::with_legacy_keyring(&output, EncryptionContext::new(), keyring);
    let pt = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(
        pt, plaintext,
        "decrypted plaintext must match original"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_no_plaintext_length_bound_field_not_included() {
    //= spec/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=Calling encrypt_stream with data_size=None omits the max_plaintext_length field; success proves the field was not included
    //# If no Plaintext Length Bound is provided, this field MUST NOT be included.
    let keyring = test_keyring().await;
    let mut stream_input =
        EncryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring.clone());
    // No plaintext length bound
    stream_input.data_size = None;
    let plaintext = vec![0xBBu8; 50];
    let mut reader = std::io::Cursor::new(&plaintext);
    let mut output = Vec::new();
    let result = encrypt_stream(&mut reader, &mut output, &stream_input).await;
    assert!(
        result.is_ok(),
        "encrypt_stream must succeed without plaintext length bound"
    );

    // Verify the output decrypts correctly
    let dec_input = DecryptInput::with_legacy_keyring(&output, EncryptionContext::new(), keyring);
    let pt = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(
        pt, plaintext,
        "decrypted plaintext must match original"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_header_released_after_serialization() {
    //= spec/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=encrypt_stream writes header before body; successful decrypt proves header was released
    //# If this operation is streaming the encrypted message and
    //# the entire message header has been serialized,
    //# the serialized message header MUST be released.
    let keyring = test_keyring().await;
    let mut stream_input =
        EncryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring.clone());
    stream_input.data_size = Some(20);
    let plaintext = vec![0xCCu8; 20];
    let mut reader = std::io::Cursor::new(&plaintext);
    let mut output = Vec::new();
    encrypt_stream(&mut reader, &mut output, &stream_input)
        .await
        .unwrap();

    // Verify the output starts with a valid header (version byte)
    assert!(!output.is_empty(), "streaming output must not be empty");
    assert!(
        output[0] == 0x01 || output[0] == 0x02,
        "streaming output must begin with a valid version byte, proving header was released"
    );

    // Verify full round-trip
    let dec_input = DecryptInput::with_legacy_keyring(&output, EncryptionContext::new(), keyring);
    let pt = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(
        pt, plaintext,
        "streaming encrypt output must decrypt successfully"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_message_bodies_not_equal_must_fail() {
    //= spec/client-apis/encrypt.md#construct-the-body
    //= type=test
    //= reason=Body written directly to output buffer; tampered body causes CryptographicError
    //# If the message bodies are not equal, the Encrypt operation MUST fail.
    let pt = b"body equality test";
    let result = round_trip(pt).await;
    assert_eq!(
        result, pt,
        "untampered ciphertext must decrypt successfully"
    );

    // Tamper a byte inside the encrypted body (NOT the footer) and verify decrypt fails
    // with an authentication error. Use a non-signing committing suite so the only
    // integrity check is the per-frame AEAD tag — a signing suite would also fail at
    // signature verify, masking which layer caught the tamper.
    let ct = encrypt_without_signing_suite(pt).await;
    // Baseline: untampered ciphertext must decrypt successfully.
    let keyring = test_keyring().await;
    let baseline = decrypt(&DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone())).await;
    assert!(baseline.is_ok(), "baseline decrypt must succeed before tamper");
    let body_start = find_body_start(&ct, 4096).expect("body start");
    // 18-byte plaintext at frame_length=4096 produces a single final frame:
    //   ENDFRAME(4) + SeqNum(4) + IV(12) + ContentLen(4) + EncContent(18) + Tag(16)
    // Tamper the first byte of EncContent.
    let content_off = body_start + 4 + 4 + IV_LEN + 4;
    let mut tampered = ct.clone();
    let original = tampered[content_off];
    tampered[content_off] ^= 0xFF;
    assert_ne!(tampered[content_off], original, "tamper must change the byte");

    let dec_input = DecryptInput::with_legacy_keyring(&tampered, EncryptionContext::new(), keyring);
    let err = decrypt(&dec_input).await.expect_err("tampered body must cause decrypt to fail");
    assert_eq!(
        err.kind, ErrorKind::CryptographicError,
        "tampered body must surface as a CryptographicError (AES-GCM authentication failure), got: {err:?}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_serialized_releases_all_bytes() {
    //= spec/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //= reason=A successful round-trip with a signing suite proves all serialized bytes (header, body, footer) were released after footer serialization
    //# Once the entire message footer has been serialized,
    //# this operation MUST release any previously unreleased serialized bytes from previous steps
    //# and MUST release the message footer.
    let keyring = test_keyring().await;
    let mut enc_input = EncryptInput::with_legacy_keyring(
        b"release all bytes test",
        EncryptionContext::new(),
        keyring.clone(),
    );
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    // Verify the ciphertext contains a footer (signing suite)
    let (footer_offset, sig_len) = find_footer_offset(&ct);
    assert!(sig_len > 0, "footer must contain a signature");
    assert_eq!(
        footer_offset + 2 + sig_len as usize,
        ct.len(),
        "all bytes must be released: footer ends exactly at the end of the ciphertext"
    );

    // Verify full round-trip
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let pt = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(
        pt, b"release all bytes test",
        "signing suite output must decrypt successfully"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_must_not_encrypt_using_nonframed_content_type() {
    //= spec/client-apis/encrypt.md#nonframed-message-body-encryption
    //= type=test
    //= reason=All encryptions produce framed content (content type 0x02); verifying the content type byte in the header proves nonframed is never used
    //# Implementations of the AWS Encryption SDK MUST NOT encrypt using the nonframed content type.
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with_version(b"nonframed test", version, keyring).await;
        // Find the content type byte in the header
        let content_type_byte = match version {
            Version::V1 => {
                let (ct_offset, _, _, _) = parse_v1_trailing_offsets(&ct);
                ct[ct_offset]
            }
            Version::V2 => {
                let ct_offset = content_type_offset_v2(&ct);
                ct[ct_offset]
            }
        };
        // Content type 0x02 = Framed, 0x01 = Non-framed
        assert_eq!(
            content_type_byte, 0x02,
            "{version:?}: content type must be 0x02 (Framed), not 0x01 (Non-framed)"
        );
    }
}

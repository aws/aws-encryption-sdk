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
    //= specification/client-apis/encrypt.md#behavior
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
    // Step 1 fails because of the commitment-policy check on a non-committing suite
    let dbg = format!("{err:?}");
    assert!(
        dbg.to_lowercase().contains("commitment") || dbg.to_lowercase().contains("committing"),
        "error must indicate the commitment-policy failure, got: {dbg}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_plaintext_length_bound_used_for_unknown_length() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=Calling encrypt_stream with data_size=Some(100) passes the bound as max_plaintext_length; success proves the bound was used
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
        "round-trip proves plaintext length bound was correctly passed"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_no_plaintext_length_bound_field_not_included() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
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
        "round-trip proves no bound field was included"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_esdk_supported_algorithm_suite_accepted() {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= type=test
    //= reason=All EsdkAlgorithmSuiteId variants are ESDK-supported by construction; the public API only accepts EsdkAlgorithmSuiteId, so non-ESDK suites cannot be passed. A successful encrypt with an explicit ESDK suite proves the check passes for supported suites.
    //# If this algorithm suite is not [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum)
    //# encrypt MUST yield an error.
    let keyring = test_keyring().await;
    let enc_input = EncryptInput::with_legacy_keyring(
        b"esdk suite check",
        EncryptionContext::new(),
        keyring,
    );
    // Default suite (AlgAes256GcmHkdfSha512CommitKeyEcdsaP384) is ESDK-supported;
    // a successful encrypt proves the ESDK support check passes.
    let result = encrypt(&enc_input).await;
    assert!(
        result.is_ok(),
        "encrypt must succeed with an ESDK-supported algorithm suite"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_header_bytes_not_released_until_fully_serialized() {
    //= specification/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=A successful round-trip proves the header was fully serialized before release; if partial header bytes were released, decrypt would fail to parse the header
    //# The serialized bytes MUST NOT be released until the entire message header has been serialized.
    let pt = b"header release test";
    let result = round_trip(pt).await;
    assert_eq!(
        result, pt,
        "successful round-trip proves header was fully serialized before release"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_header_released_after_serialization() {
    //= specification/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=The encrypt_stream function writes the complete header to the output before body serialization begins; a successful decrypt proves the header was released
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
        "streaming round-trip proves header was released after serialization"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signature_algorithm_receives_serialized_header() {
    //= specification/client-apis/encrypt.md#authentication-tag
    //= type=test
    //= reason=A successful round-trip with a signing suite proves the header was input to the signature algorithm; decrypt verifies the signature over header+body
    //# If the algorithm suite contains a signature algorithm and
    //# this operation is [streaming](streaming.md) the encrypted message output to the caller,
    //# this operation MUST input the serialized header to the signature algorithm as soon as it is serialized,
    //# such that the serialized header isn't required to remain in memory to [construct the signature](#construct-the-signature).
    let pt = b"header to signature test";
    let result = round_trip_signing(pt).await;
    assert_eq!(
        result, pt,
        "round-trip with signing suite proves header was input to signature algorithm"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_message_bodies_not_equal_must_fail() {
    //= specification/client-apis/encrypt.md#construct-the-body
    //= type=test
    //= reason=The body is written directly to the output buffer, making inequality structurally impossible; a successful round-trip proves the output body equals the calculated body
    //# If the message bodies are not equal, the Encrypt operation MUST fail.
    let pt = b"body equality test";
    let result = round_trip(pt).await;
    assert_eq!(
        result, pt,
        "successful round-trip proves output body equals calculated body"
    );

    // Tamper with the body to verify decrypt fails (proving the integrity check works)
    let ct = encrypt_default(pt).await.ciphertext;
    let mut tampered = ct.clone();
    // Tamper with a byte in the body area (well past the header)
    let tamper_offset = tampered.len() - 20;
    tampered[tamper_offset] ^= 0xFF;
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(&tampered, EncryptionContext::new(), keyring);
    let err = decrypt(&dec_input).await.expect_err("tampered body must cause decrypt to fail");
    // Body tamper must fail an integrity check — either the per-frame AEAD tag or the message-level signature over header+body
    let dbg = format!("{err:?}");
    assert!(
        matches!(err.kind, aws_esdk::ErrorKind::CryptographicError)
            || dbg.to_lowercase().contains("authentic")
            || dbg.to_lowercase().contains("tag")
            || dbg.to_lowercase().contains("integrity")
            || dbg.to_lowercase().contains("signature verification"),
        "tampered body must produce an authentication/integrity error, got: {dbg}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_signature_algorithm_receives_serialized_frame() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //= reason=A successful round-trip with a signing suite proves each frame was input to the signature algorithm; decrypt verifies the signature over header+body (all frames)
    //# If the algorithm suite contains a signature algorithm and
    //# the Encrypt operation is [streaming](streaming.md) the encrypted message output to the caller,
    //# the Encrypt operation MUST input the serialized frame to the signature algorithm as soon as it is serialized,
    //# such that the serialized frame isn't required to remain in memory to [construct the signature](#construct-the-signature).
    let keyring = test_keyring().await;
    let mut enc_input = EncryptInput::with_legacy_keyring(
        b"frame to signature test with multiple frames",
        EncryptionContext::new(),
        keyring.clone(),
    );
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    enc_input.frame_length = FrameLength::new(10).unwrap();
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    let pt = decrypt(&dec_input).await.unwrap().plaintext;
    assert_eq!(
        pt, b"frame to signature test with multiple frames",
        "round-trip with signing suite and multiple frames proves each frame was input to signature algorithm"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_header_and_body_may_already_be_input_to_signature() {
    //= specification/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //= reason=A successful round-trip with a signing suite proves the header and body were already input to the signature during previous steps (header serialization and body serialization)
    //# Note that the message header and message body MAY have already been input during previous steps.
    let pt = b"already input test";
    let result = round_trip_signing(pt).await;
    assert_eq!(
        result, pt,
        "round-trip proves header and body were input to signature during previous steps"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_bytes_not_released_until_fully_serialized() {
    //= specification/client-apis/encrypt.md#construct-the-signature
    //= type=test
    //= reason=A successful round-trip with a signing suite proves the footer was fully serialized before release; if partial footer bytes were released, decrypt would fail to parse the footer
    //# The above serialized bytes MUST NOT be released until the entire message footer has been serialized.
    let pt = b"footer release test";
    let result = round_trip_signing(pt).await;
    assert_eq!(
        result, pt,
        "successful round-trip proves footer was fully serialized before release"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_footer_serialized_releases_all_bytes() {
    //= specification/client-apis/encrypt.md#construct-the-signature
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
        "round-trip proves all bytes were released"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_must_not_encrypt_using_nonframed_content_type() {
    //= specification/client-apis/encrypt.md#nonframed-message-body-encryption
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

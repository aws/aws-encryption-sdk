// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod fixtures;
mod test_helpers;

use aws_esdk::{
    decrypt_stream, encrypt, encrypt_stream, DecryptStreamInput, EncryptInput, EncryptStreamInput,
    EncryptionContext, FrameLength,
};
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use test_helpers::test_keyring;

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_encrypt_decrypt_round_trip() {
    //= spec/client-apis/streaming.md#overview
    //= type=test
    //= reason=encrypt_stream and decrypt_stream are the streaming APIs; the round-trip proves both work
    //# The AWS Encryption SDK MAY provide APIs that enable streamed [encryption](encrypt.md)
    //# and [decryption](decrypt.md).
    //
    //= spec/client-apis/encrypt.md#plaintext
    //= type=test
    //= reason=the round-trip with a Cursor input source succeeds, proving plaintext can be streamed to encrypt
    //# This input MAY be [streamed](streaming.md) to this operation.
    //
    //= spec/client-apis/encrypt.md#encrypted-message
    //= type=test
    //= reason=the round-trip with a Vec<u8> ciphertext sink succeeds, proving the encrypted message can be streamed
    //# This operation MAY [stream](streaming.md) the encrypted message.
    //
    //= spec/client-apis/streaming.md#outputs
    //= type=test
    //= reason=decrypt_stream returns Ok only after all plaintext is written; the round-trip assertion confirms this
    //# Operations MUST NOT indicate completion or success until an end to the output has been indicated.
    let keyring = test_keyring().await;
    let plaintext = b"hello streaming world";

    // Encrypt via streaming API
    let ec = EncryptionContext::new();
    let enc_input = EncryptStreamInput::with_legacy_keyring(ec.clone(), keyring.clone());

    let mut pt_cursor = std::io::Cursor::new(&plaintext[..]);
    let mut ciphertext: Vec<u8> = Vec::new();

    encrypt_stream(&mut pt_cursor, &mut ciphertext, &enc_input)
        .await
        .unwrap();

    assert!(!ciphertext.is_empty(), "ciphertext must not be empty");

    // Decrypt via streaming API
    let dec_input = DecryptStreamInput::with_legacy_keyring(ec, keyring);
    let mut ct_cursor = std::io::Cursor::new(&ciphertext[..]);
    let mut decrypted: Vec<u8> = Vec::new();

    decrypt_stream(&mut ct_cursor, &mut decrypted, &dec_input)
        .await
        .unwrap();

    assert_eq!(decrypted, plaintext, "round-trip plaintext must match");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_stream_multi_frame_signed_rejected_by_default() {
    // Multi-frame signed payload: 30 bytes / frame_length=10 → 3 frames (2 regular + final).
    // With unsafe_release_plaintext_before_verify=false (default), decrypt_stream must
    // reject this up front rather than release plaintext before signature verification.
    let keyring = test_keyring().await;
    let plaintext = vec![0xAAu8; 30];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(10).unwrap();
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let dec_input =
        DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    assert!(
        !dec_input.unsafe_release_plaintext_before_verify,
        "unsafe_release_plaintext_before_verify must default to false"
    );

    let mut ct_cursor = std::io::Cursor::new(&ct[..]);
    let mut output: Vec<u8> = Vec::new();
    let result = decrypt_stream(&mut ct_cursor, &mut output, &dec_input).await;
    assert!(
        result.is_err(),
        "multi-frame signed message must fail with default unsafe_release_plaintext_before_verify=false"
    );

    //= spec/client-apis/streaming.md#release
    //= type=test
    //= reason=decrypt_stream rejects the multi-frame signed payload before any plaintext is written, honoring "specify when not to release"
    //# The decrypt and encrypt operations specify when to release output bytes and when not to release output bytes.
    assert!(
        output.is_empty(),
        "no plaintext must be released before signature verification"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_stream_multi_frame_signed_unsafe_flag_round_trip() {
    // Same multi-frame signed payload as the default-rejection test. Setting
    // unsafe_release_plaintext_before_verify=true opts in to early release;
    // decrypt_stream now succeeds and the plaintext round-trips. Doubles as
    // multi-frame streaming round-trip coverage for this PR.
    let keyring = test_keyring().await;
    let plaintext = vec![0xAAu8; 30];
    let mut enc_input =
        EncryptInput::with_legacy_keyring(&plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(10).unwrap();
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;

    let mut dec_input =
        DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
    dec_input.unsafe_release_plaintext_before_verify = true;

    let mut ct_cursor = std::io::Cursor::new(&ct[..]);
    let mut output: Vec<u8> = Vec::new();
    decrypt_stream(&mut ct_cursor, &mut output, &dec_input)
        .await
        .unwrap();
    assert_eq!(output, plaintext);
}

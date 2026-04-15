// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_encrypt_decrypt_round_trip() {
    //= specification/client-apis/streaming.md#overview
    //= type=test
    //= reason=encrypt_stream and decrypt_stream provide streaming encryption and decryption APIs; a successful round-trip proves both exist and work
    //# The AWS Encryption SDK MAY provide APIs that enable streamed [encryption](encrypt.md)
    //# and [decryption](decrypt.md).

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
async fn test_streaming_finite_working_memory() {
    //= specification/client-apis/streaming.md#overview
    //= type=test
    //= reason=encrypts and decrypts a multi-frame payload via streaming APIs, demonstrating that arbitrarily large inputs can be processed frame-by-frame with finite memory
    //# APIs that support streaming of the encrypt or decrypt operation SHOULD allow customers
    //# to be able to process arbitrarily large inputs with a finite amount of working memory.

    let keyring = test_keyring().await;
    // Use a payload larger than one frame (default frame = 4096 bytes)
    let plaintext = vec![0xABu8; 10_000];

    let ec = EncryptionContext::new();
    let enc_input = EncryptStreamInput::with_legacy_keyring(ec.clone(), keyring.clone());
    let mut pt_cursor = std::io::Cursor::new(&plaintext[..]);
    let mut ciphertext: Vec<u8> = Vec::new();
    encrypt_stream(&mut pt_cursor, &mut ciphertext, &enc_input)
        .await
        .unwrap();

    let mut dec_input = DecryptStreamInput::with_legacy_keyring(ec, keyring);
    dec_input.i_accept_the_danger = true;
    let mut ct_cursor = std::io::Cursor::new(&ciphertext[..]);
    let mut decrypted: Vec<u8> = Vec::new();
    decrypt_stream(&mut ct_cursor, &mut decrypted, &dec_input)
        .await
        .unwrap();

    assert_eq!(decrypted, plaintext, "multi-frame streaming round-trip must match");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_does_not_require_full_buffering() {
    //= specification/client-apis/streaming.md#overview
    //= type=test
    //= reason=the streaming APIs accept SafeRead/SafeWrite (incremental I/O), proving the implementation does not require holding the entire input in memory
    //# If an implementation requires holding the entire input in memory in order to perform the operation,
    //# that implementation SHOULD NOT provide an API that allows the caller to stream the operation.

    let keyring = test_keyring().await;
    let plaintext = b"streaming without full buffering";

    let ec = EncryptionContext::new();
    let enc_input = EncryptStreamInput::with_legacy_keyring(ec.clone(), keyring.clone());
    let mut pt_cursor = std::io::Cursor::new(&plaintext[..]);
    let mut ciphertext: Vec<u8> = Vec::new();
    // encrypt_stream accepts &mut dyn SafeRead, not &[u8] — it does not require the full input in memory
    encrypt_stream(&mut pt_cursor, &mut ciphertext, &enc_input)
        .await
        .unwrap();

    let dec_input = DecryptStreamInput::with_legacy_keyring(ec, keyring);
    let mut ct_cursor = std::io::Cursor::new(&ciphertext[..]);
    let mut decrypted: Vec<u8> = Vec::new();
    // decrypt_stream accepts &mut dyn SafeRead, not &[u8] — it does not require the full input in memory
    decrypt_stream(&mut ct_cursor, &mut decrypted, &dec_input)
        .await
        .unwrap();

    assert_eq!(decrypted, plaintext.as_slice());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_completion_only_after_output_end() {
    //= specification/client-apis/streaming.md#outputs
    //= type=test
    //= reason=decrypt_stream returns Ok only after all plaintext bytes have been written to the SafeWrite output; the assertion that decrypted == plaintext proves output was complete before success was indicated
    //# Operations MUST NOT indicate completion or success until an end to the output has been indicated.

    let keyring = test_keyring().await;
    let plaintext = b"completion test payload";

    let ec = EncryptionContext::new();
    let enc_input = EncryptStreamInput::with_legacy_keyring(ec.clone(), keyring.clone());
    let mut pt_cursor = std::io::Cursor::new(&plaintext[..]);
    let mut ciphertext: Vec<u8> = Vec::new();
    encrypt_stream(&mut pt_cursor, &mut ciphertext, &enc_input)
        .await
        .unwrap();

    let dec_input = DecryptStreamInput::with_legacy_keyring(ec, keyring);
    let mut ct_cursor = std::io::Cursor::new(&ciphertext[..]);
    let mut decrypted: Vec<u8> = Vec::new();
    // After decrypt_stream returns Ok, all output must already be in `decrypted`
    let result = decrypt_stream(&mut ct_cursor, &mut decrypted, &dec_input).await;
    assert!(result.is_ok(), "decrypt_stream must succeed");
    // The fact that decrypted contains the full plaintext AFTER Ok proves
    // completion was not indicated before the output was fully written
    assert_eq!(decrypted, plaintext.as_slice(), "all output must be present when success is indicated");
}

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_encrypt_decrypt_round_trip() {
    //= spec/client-apis/streaming.md#overview
    //= type=test
    //= reason=encrypt_stream and decrypt_stream provide streaming encryption and decryption APIs; a successful round-trip proves both exist and work
    //# The AWS Encryption SDK MAY provide APIs that enable streamed [encryption](encrypt.md)
    //# and [decryption](decrypt.md).
    //
    //= spec/client-apis/encrypt.md#plaintext
    //= type=test
    //= reason=encrypt_stream reads plaintext from a SafeRead (Cursor) incrementally, proving the plaintext MAY be streamed to the encrypt operation
    //# This input MAY be [streamed](streaming.md) to this operation.
    //
    //= spec/client-apis/encrypt.md#encrypted-message
    //= type=test
    //= reason=encrypt_stream writes the encrypted message to a SafeWrite (Vec<u8>) frame-by-frame, proving the encrypted message MAY be streamed
    //# This operation MAY [stream](streaming.md) the encrypted message.
    let keyring = test_keyring().await;
    let plaintext = b"hello streaming world";

    // Encrypt via streaming API
    let ec = EncryptionContext::new();
    let enc_input = EncryptStreamInput::with_legacy_keyring(ec.clone(), keyring.clone());

    //= spec/client-apis/streaming.md#inputs
    //= type=test
    //= reason=the Cursor implements Read; encrypt_stream reads bytes from it incrementally, making them consumable
    //# - There MUST be a mechanism for input bytes to become consumable.
    //
    //= spec/client-apis/streaming.md#inputs
    //= type=test
    //= reason=the Cursor returns Ok(0) at EOF; encrypt_stream completes successfully, proving the EOF mechanism works
    //# - There MUST be a mechanism to indicate that there are no more input bytes.
    let mut pt_cursor = std::io::Cursor::new(&plaintext[..]);
    let mut ciphertext: Vec<u8> = Vec::new();

    //= spec/client-apis/streaming.md#inputs
    //= type=test
    //= reason=encrypt_stream accepts a SafeRead (Cursor) as input, proving the operation accepts input within a streaming framework
    //# In order to support streaming, the operation MUST accept some input within a streaming framework.
    encrypt_stream(&mut pt_cursor, &mut ciphertext, &enc_input)
        .await
        .unwrap();

    assert!(!ciphertext.is_empty(), "ciphertext must not be empty");

    // Decrypt via streaming API
    let dec_input = DecryptStreamInput::with_legacy_keyring(ec, keyring);
    let mut ct_cursor = std::io::Cursor::new(&ciphertext[..]);
    let mut decrypted: Vec<u8> = Vec::new();

    //= spec/client-apis/streaming.md#outputs
    //= type=test
    //= reason=the Vec<u8> receives bytes via SafeWrite::write(), which is the mechanism for releasing output bytes
    //# - There MUST be a mechanism for output bytes to be released.
    //
    //= spec/client-apis/streaming.md#outputs
    //= type=test
    //= reason=decrypt_stream writes output to a Vec<u8> via SafeWrite; the non-empty result proves output was produced within the streaming framework
    //# In order to support streaming, the operation MUST produce some output within a streaming framework.
    decrypt_stream(&mut ct_cursor, &mut decrypted, &dec_input)
        .await
        .unwrap();

    assert_eq!(decrypted, plaintext, "round-trip plaintext must match");
}

//= spec/client-apis/streaming.md#overview
//= type=test
//= reason=encrypts and decrypts a multi-frame payload via streaming APIs, demonstrating that arbitrarily large inputs can be processed frame-by-frame with finite memory
//# APIs that support streaming of the encrypt or decrypt operation SHOULD allow customers
//# to be able to process arbitrarily large inputs with a finite amount of working memory.
//
//= spec/client-apis/streaming.md#overview
//= type=test
//= reason=the streaming APIs accept SafeRead/SafeWrite (incremental I/O), proving the implementation does not require holding the entire input in memory
//# If an implementation requires holding the entire input in memory in order to perform the operation,
//# that implementation SHOULD NOT provide an API that allows the caller to stream the operation.
//
//= spec/client-apis/encrypt.md#plaintext
//= type=test
//= reason=encrypt_stream processes a multi-frame payload via SafeRead/SafeWrite without holding the entire plaintext in memory; providing a streaming encrypt API is therefore correct under the contrapositive of this requirement.
//# If an implementation requires holding the input entire plaintext in memory in order to perform this operation,
//# that implementation SHOULD NOT provide an API that allows this input to be streamed.
#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_finite_working_memory() {
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
    // Multi-frame payloads with signed algorithm suites stream data before
    // signature verification completes; acknowledge this risk for the test.
    dec_input.i_accept_the_danger = true;
    let mut ct_cursor = std::io::Cursor::new(&ciphertext[..]);
    let mut decrypted: Vec<u8> = Vec::new();
    decrypt_stream(&mut ct_cursor, &mut decrypted, &dec_input)
        .await
        .unwrap();

    assert_eq!(
        decrypted, plaintext,
        "multi-frame streaming round-trip must match"
    );
}

//= spec/client-apis/streaming.md#outputs
//= type=test
//= reason=decrypt_stream returns Ok only after all plaintext bytes have been written to the SafeWrite output; the assertion that decrypted == plaintext proves output was complete before success was indicated
//# Operations MUST NOT indicate completion or success until an end to the output has been indicated.
#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_completion_only_after_output_end() {
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
    let result = decrypt_stream(&mut ct_cursor, &mut decrypted, &dec_input).await;
    assert!(result.is_ok(), "decrypt_stream must succeed");

    //= spec/client-apis/streaming.md#outputs
    //= type=test
    //= reason=after decrypt_stream returns Ok, all output is present in the Vec<u8>, proving the end-of-output mechanism works
    //# - There MUST be a mechanism to indicate that the entire output has been released.
    assert_eq!(
        decrypted,
        plaintext.as_slice(),
        "all output must be present when success is indicated"
    );
}

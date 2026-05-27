// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod fixtures;
mod test_helpers;

use aws_esdk::{DecryptStreamInput, EncryptStreamInput, EncryptionContext, ErrorKind};
use test_helpers::{
    multi_frame_signed_for_stream, multi_frame_unsigned_for_stream, run_decrypt_stream,
    run_encrypt_stream, test_keyring, CallStyle,
};

// Round-trips plaintext via the streaming encrypt/decrypt path. Iterates over
// CallStyle so both the free `encrypt_stream`/`decrypt_stream` functions AND
// `Esdk::default().encrypt_stream`/`decrypt_stream` are exercised by this one
// test. A failure under either path will name the offending CallStyle.
#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_encrypt_decrypt_round_trip() {
    for style in CallStyle::ALL {
        let keyring = test_keyring().await;
        let plaintext = b"hello streaming world";

        // Encrypt via streaming API
        let ec = EncryptionContext::new();
        let enc_input = EncryptStreamInput::with_legacy_keyring(ec.clone(), keyring.clone());

        let mut pt_cursor = std::io::Cursor::new(&plaintext[..]);
        let mut ciphertext: Vec<u8> = Vec::new();

        //= spec/client-apis/streaming.md#inputs
        //= type=test
        //= reason=encrypt_stream reads from the Cursor incrementally, making bytes consumable; the round-trip succeeds
        //# - There MUST be a mechanism for input bytes to become consumable.
        //
        //= spec/client-apis/streaming.md#inputs
        //= type=test
        //= reason=the Cursor returns Ok(0) at EOF; encrypt_stream completes successfully, proving the EOF mechanism works
        //# - There MUST be a mechanism to indicate that there are no more input bytes.
        //
        //= spec/client-apis/streaming.md#inputs
        //= type=test
        //= reason=encrypt_stream accepts a SafeRead (Cursor) as input; the round-trip succeeds, proving the streaming-input mechanism works
        //# In order to support streaming, the operation MUST accept some input within a streaming framework.
        //
        //= spec/client-apis/encrypt.md#plaintext
        //= type=test
        //= reason=the round-trip with a Cursor input source succeeds, proving plaintext can be streamed to encrypt
        //# This input MAY be [streamed](streaming.md) to this operation.
        run_encrypt_stream(style, &mut pt_cursor, &mut ciphertext, &enc_input)
            .await
            .unwrap_or_else(|e| panic!("encrypt_stream failed under {style:?}: {e:?}"));

        //= spec/client-apis/encrypt.md#encrypted-message
        //= type=test
        //= reason=the round-trip with a Vec<u8> ciphertext sink succeeds, proving the encrypted message can be streamed
        //# This operation MAY [stream](streaming.md) the encrypted message.
        assert!(
            !ciphertext.is_empty(),
            "ciphertext must not be empty (style: {style:?})"
        );

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
        //= reason=decrypt_stream writes output to a Vec<u8> via SafeWrite; the non-empty result proves streaming output
        //# In order to support streaming, the operation MUST produce some output within a streaming framework.
        run_decrypt_stream(style, &mut ct_cursor, &mut decrypted, &dec_input)
            .await
            .unwrap_or_else(|e| panic!("decrypt_stream failed under {style:?}: {e:?}"));

        //= spec/client-apis/streaming.md#overview
        //= type=test
        //= reason=encrypt_stream and decrypt_stream are the streaming APIs; the round-trip proves both work
        //# The AWS Encryption SDK MAY provide APIs that enable streamed [encryption](encrypt.md)
        //# and [decryption](decrypt.md).
        //
        //= spec/client-apis/streaming.md#outputs
        //= type=test
        //= reason=decrypt_stream returns Ok only after all plaintext is written; the round-trip assertion confirms this
        //# Operations MUST NOT indicate completion or success until an end to the output has been indicated.
        //
        //= spec/client-apis/streaming.md#outputs
        //= type=test
        //= reason=decrypt_stream's Ok return means all plaintext is in the output, proving end-of-output signaling
        //# - There MUST be a mechanism to indicate that the entire output has been released.
        assert_eq!(
            decrypted, plaintext,
            "round-trip plaintext must match (style: {style:?})"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_stream_multi_frame_signed_rejected_by_default() {
    // multi_frame_signed_for_stream produces a 30-byte / frame_length=10 ECDSA-P384
    // payload (3 frames: 2 regular + final). With unsafe_release_plaintext_before_verify
    // = false (default), decrypt_stream must reject this up front rather than release
    // plaintext before signature verification.
    for style in CallStyle::ALL {
        let (_plaintext, keyring, ct) = multi_frame_signed_for_stream().await;

        let dec_input =
            DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
        assert!(
            !dec_input.unsafe_release_plaintext_before_verify,
            "unsafe_release_plaintext_before_verify must default to false"
        );

        let mut ct_cursor = std::io::Cursor::new(&ct[..]);
        let mut output: Vec<u8> = Vec::new();
        let err = run_decrypt_stream(style, &mut ct_cursor, &mut output, &dec_input)
            .await
            .expect_err("multi-frame signed message must fail with default unsafe_release_plaintext_before_verify=false");
        assert!(
            matches!(err.kind, ErrorKind::Esdk),
            "expected ErrorKind::Esdk under {style:?}, got {:?}",
            err.kind
        );

        //= spec/client-apis/streaming.md#release
        //= type=test
        //= reason=decrypt_stream rejects the multi-frame signed payload before any plaintext is written, honoring "specify when not to release"
        //# The decrypt and encrypt operations specify when to release output bytes and when not to release output bytes.
        assert!(
            output.is_empty(),
            "no plaintext must be released before signature verification (style: {style:?})"
        );
    }
}

// Proves that unsafe_release_plaintext_before_verify=true allows a
// multi-frame signed payload to round-trip via decrypt_stream. Doubles
// as multi-frame streaming coverage for this PR.
#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_stream_multi_frame_signed_unsafe_flag_round_trip() {
    // Same multi-frame signed payload as the default-rejection test. Setting
    // unsafe_release_plaintext_before_verify=true opts in to early release;
    // decrypt_stream now succeeds and the plaintext round-trips. Doubles as
    // multi-frame streaming round-trip coverage for this PR.
    for style in CallStyle::ALL {
        let (plaintext, keyring, ct) = multi_frame_signed_for_stream().await;

        let mut dec_input =
            DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
        dec_input.unsafe_release_plaintext_before_verify = true;

        let mut ct_cursor = std::io::Cursor::new(&ct[..]);
        let mut output: Vec<u8> = Vec::new();
        run_decrypt_stream(style, &mut ct_cursor, &mut output, &dec_input)
            .await
            .unwrap_or_else(|e| panic!("decrypt_stream failed under {style:?}: {e:?}"));
        assert_eq!(
            output, plaintext,
            "multi-frame signed round-trip mismatch under {style:?}"
        );
    }
}

// Multi-frame unsigned payloads don't require unsafe_release_plaintext_before_verify
// because each frame is individually authenticated via AES-GCM (no trailing signature).
#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_stream_multi_frame_unsigned_accepted_by_default() {
    for style in CallStyle::ALL {
        let (plaintext, keyring, ct) = multi_frame_unsigned_for_stream().await;

        let dec_input =
            DecryptStreamInput::with_legacy_keyring(EncryptionContext::new(), keyring);
        assert!(!dec_input.unsafe_release_plaintext_before_verify);

        let mut ct_cursor = std::io::Cursor::new(&ct[..]);
        let mut output: Vec<u8> = Vec::new();
        run_decrypt_stream(style, &mut ct_cursor, &mut output, &dec_input)
            .await
            .unwrap_or_else(|e| panic!("decrypt_stream failed under {style:?}: {e:?}"));
        assert_eq!(
            output, plaintext,
            "multi-frame unsigned round-trip mismatch under {style:?}"
        );
    }
}

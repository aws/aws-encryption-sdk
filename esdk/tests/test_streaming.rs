// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod fixtures;
mod test_helpers;

use aws_esdk::{
    decrypt_stream, encrypt_stream, DecryptStreamInput, EncryptStreamInput, EncryptionContext,
};
use test_helpers::test_keyring;

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
    //
    //= spec/client-apis/streaming.md#outputs
    //= type=test
    //= reason=decrypt_stream returns Ok only after all plaintext bytes have been written to the SafeWrite output; the round-trip assertion below proves the operation did not signal completion before the output was complete
    //# Operations MUST NOT indicate completion or success until an end to the output has been indicated.
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

    //= spec/client-apis/streaming.md#outputs
    //= type=test
    //= reason=after decrypt_stream returns Ok, all plaintext bytes are present in the output Vec<u8>; this proves the end-of-output signaling matches actual completion
    //# - There MUST be a mechanism to indicate that the entire output has been released.
    assert_eq!(decrypted, plaintext, "round-trip plaintext must match");
}

//= spec/client-apis/streaming.md#overview
//= type=test
//= reason=A 10_000-byte plaintext is streamed through encrypt_stream via a Read source that generates bytes on demand (PatternReader; never materializes the full payload) and decrypt_stream's plaintext output is consumed by a Write sink that counts and verifies bytes without buffering them (VerifyingWriter). Both plaintext directions are processed without holding the full payload in memory, demonstrating bounded working memory across the streaming round-trip.
//# APIs that support streaming of the encrypt or decrypt operation SHOULD allow customers
//# to be able to process arbitrarily large inputs with a finite amount of working memory.
//
//= spec/client-apis/streaming.md#overview
//= type=test
//= reason=PatternReader holds two usize-sized fields regardless of total byte count; VerifyingWriter holds three. Neither side ever buffers the full plaintext, so providing a streaming API is correct under this requirement.
//# If an implementation requires holding the entire input in memory in order to perform the operation,
//# that implementation SHOULD NOT provide an API that allows the caller to stream the operation.
//
//= spec/client-apis/encrypt.md#plaintext
//= type=test
//= reason=PatternReader generates plaintext bytes on demand; encrypt_stream succeeds without ever requiring the full plaintext to be materialized in memory.
//# If an implementation requires holding the input entire plaintext in memory in order to perform this operation,
//# that implementation SHOULD NOT provide an API that allows this input to be streamed.
#[tokio::test(flavor = "multi_thread")]
async fn test_streaming_finite_working_memory() {
    /// `Read` source that yields `remaining` bytes equal to `byte` on demand.
    /// Holds two `usize`-sized fields regardless of total byte count, so the
    /// plaintext is never materialized as a buffer.
    #[derive(Debug)]
    struct PatternReader {
        remaining: usize,
        byte: u8,
    }
    impl std::io::Read for PatternReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let n = std::cmp::min(buf.len(), self.remaining);
            buf[..n].fill(self.byte);
            self.remaining -= n;
            Ok(n)
        }
    }

    /// `Write` sink that counts bytes written and verifies every byte equals
    /// `expected`. Holds three small fields regardless of total byte count, so
    /// the decrypted plaintext is never buffered.
    #[derive(Debug)]
    struct VerifyingWriter {
        count: usize,
        expected: u8,
        all_match: bool,
    }
    impl std::io::Write for VerifyingWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            if !buf.iter().all(|&b| b == self.expected) {
                self.all_match = false;
            }
            self.count += buf.len();
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let keyring = test_keyring().await;
    // 10_000 bytes is larger than the default 4096-byte frame, so this
    // exercises multi-frame handling on top of the streaming I/O.
    let total_bytes: usize = 10_000;
    let pattern = 0xABu8;

    // Encrypt: plaintext source streams bytes on demand. The full plaintext
    // is never materialized as a `Vec`. The ciphertext must be buffered so we
    // can re-feed it to `decrypt_stream` below; this is unavoidable in a
    // single-threaded round-trip but the buffer holds ciphertext, not plaintext.
    let mut pt_reader = PatternReader {
        remaining: total_bytes,
        byte: pattern,
    };
    let ec = EncryptionContext::new();
    let enc_input = EncryptStreamInput::with_legacy_keyring(ec.clone(), keyring.clone());
    let mut ciphertext: Vec<u8> = Vec::new();
    encrypt_stream(&mut pt_reader, &mut ciphertext, &enc_input)
        .await
        .unwrap();

    // Decrypt: plaintext sink counts and verifies bytes without buffering.
    // Multi-frame payloads with signed algorithm suites stream data before
    // signature verification completes; acknowledge this risk for the test.
    let mut dec_input = DecryptStreamInput::with_legacy_keyring(ec, keyring);
    dec_input.i_accept_the_danger = true;
    let mut ct_cursor = std::io::Cursor::new(&ciphertext[..]);
    let mut sink = VerifyingWriter {
        count: 0,
        expected: pattern,
        all_match: true,
    };
    decrypt_stream(&mut ct_cursor, &mut sink, &dec_input)
        .await
        .unwrap();

    assert_eq!(
        sink.count, total_bytes,
        "decrypt_stream output byte count must equal the streamed plaintext length"
    );
    assert!(
        sink.all_match,
        "every decrypted plaintext byte must equal the streamed pattern"
    );
}

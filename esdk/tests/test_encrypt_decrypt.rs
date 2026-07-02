// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

mod fixtures;
mod test_helpers;

use aws_esdk::{DecryptInput, EncryptInput, EncryptionContext, ErrorKind};
use test_helpers::{kms_keyring, run_decrypt, run_encrypt, CallStyle};

// Round-trips plaintext via encrypt/decrypt. Iterates over CallStyle so both
// the free `encrypt`/`decrypt` functions AND `Esdk::default().encrypt`/`decrypt`
// are exercised by this one test. A failure under either path will name the
// offending CallStyle.
#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt() {
    for style in CallStyle::ALL {
        let kms_keyring = kms_keyring().await;
        let plaintext = b"hello esdk";
        let ec = EncryptionContext::new();
        let encrypt_input = EncryptInput::with_legacy_keyring(plaintext, ec, kms_keyring);

        //= spec/client-apis/client.md#encrypt
        //= type=test
        //# The AWS Encryption SDK Client MUST provide an [encrypt](./encrypt.md#input) function
        //# that adheres to [encrypt](./encrypt.md).
        let encrypt_output = run_encrypt(style, &encrypt_input)
            .await
            .unwrap_or_else(|e| panic!("encrypt failed under {style:?}: {e:?}"));
        let esdk_ciphertext = encrypt_output.ciphertext;

        let decrypt_input = DecryptInput::from_encrypt(&esdk_ciphertext, &encrypt_input);

        //= spec/client-apis/client.md#decrypt
        //= type=test
        //# The AWS Encryption SDK Client MUST provide an [decrypt](./decrypt.md#input) function
        //# that adheres to [decrypt](./decrypt.md).
        let decrypt_output = run_decrypt(style, &decrypt_input)
            .await
            .unwrap_or_else(|e| panic!("decrypt failed under {style:?}: {e:?}"));

        assert_eq!(
            decrypt_output.plaintext, plaintext,
            "round-trip plaintext must match (style: {style:?})"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bad_decrypt_input() {
    for style in CallStyle::ALL {
        let kms_keyring = kms_keyring().await;
        let ec = EncryptionContext::new();
        let encrypt_input =
            EncryptInput::with_legacy_keyring(b"hello esdk", ec.clone(), kms_keyring.clone());
        let esdk_ciphertext = run_encrypt(style, &encrypt_input)
            .await
            .unwrap_or_else(|e| panic!("encrypt failed under {style:?}: {e:?}"))
            .ciphertext;
        let mut decrypt_input =
            DecryptInput::with_legacy_keyring(&esdk_ciphertext, ec, kms_keyring);

        decrypt_input.source = None;
        let bad_decrypt_output = run_decrypt(style, &decrypt_input).await;

        //= spec/client-apis/decrypt.md#input
        //= type=test
        //# The Decrypt operation MUST validate that exactly one of a keyring or CMM was provided by the caller.
        //
        //= spec/client-apis/decrypt.md#input
        //= type=test
        //# If the caller does not provide exactly one of a keyring or CMM, the Decrypt operation MUST fail.
        let err = bad_decrypt_output
            .expect_err("decrypt must fail when source = None");
        assert!(
            matches!(err.kind, ErrorKind::ValidationError),
            "expected ValidationError under {style:?}, got {:?}",
            err.kind
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt_short() {
    for style in CallStyle::ALL {
        let kms_keyring = kms_keyring().await;
        let plaintext = b"hello esdk";
        let ec = EncryptionContext::new();
        let encrypt_input = EncryptInput::with_legacy_keyring(plaintext, ec, kms_keyring);
        let encrypt_output = run_encrypt(style, &encrypt_input)
            .await
            .unwrap_or_else(|e| panic!("encrypt failed under {style:?}: {e:?}"));
        let esdk_ciphertext = encrypt_output.ciphertext;
        let cipher_len: usize = esdk_ciphertext.len();
        let mut decrypt_input =
            DecryptInput::from_encrypt(&esdk_ciphertext[..cipher_len], &encrypt_input);

        // Truncate the last byte of the ciphertext. For the default signing suite this
        // chops off the final byte of the footer signature, so footer deserialization
        // fails with UnexpectedEof inside read_footer (mapped to SerializationError by ser_io).
        decrypt_input.ciphertext = &esdk_ciphertext[..cipher_len - 1];
        //= spec/client-apis/decrypt.md#behavior
        //= type=test
        //= reason=Truncating the ciphertext's last byte makes footer deserialization fail; decrypt MUST halt with an error
        //# - If all bytes have been provided and this operation
        //# is unable to complete the above steps with the consumable encrypted message bytes,
        //# this operation MUST halt and indicate a failure to the caller.
        let err = run_decrypt(style, &decrypt_input)
            .await
            .expect_err("truncated ciphertext must fail decrypt");
        assert_eq!(
            err.kind,
            ErrorKind::SerializationError,
            "truncated ciphertext must produce a SerializationError under {style:?}, got: {:?} ({})",
            err.kind, err.message
        );
    }
}

// Proves that an encryption context provided at encrypt time round-trips:
// the (key, value) pair appears in DecryptOutput.encryption_context.
#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt_ec() {
    for style in CallStyle::ALL {
        let kms_keyring = kms_keyring().await;
        let plaintext = b"hello esdk";
        let encryption_context =
            std::collections::HashMap::from([("stuff".to_string(), "junk".to_string())]);
        let encrypt_input = EncryptInput::with_legacy_keyring(
            plaintext,
            encryption_context,
            kms_keyring.clone(),
        );
        let encrypt_output = run_encrypt(style, &encrypt_input)
            .await
            .unwrap_or_else(|e| panic!("encrypt failed under {style:?}: {e:?}"));
        let esdk_ciphertext = encrypt_output.ciphertext;
        let ec = EncryptionContext::new();
        let decrypt_input =
            DecryptInput::with_legacy_keyring(&esdk_ciphertext, ec, kms_keyring);
        let decrypt_output = run_decrypt(style, &decrypt_input)
            .await
            .unwrap_or_else(|e| panic!("decrypt failed under {style:?}: {e:?}"));

        assert_eq!(decrypt_output.plaintext, plaintext);
        assert_eq!(
            decrypt_output
                .encryption_context
                .get("stuff")
                .map(String::as_str),
            Some("junk"),
            "input EC pair (stuff, junk) must round-trip to the decrypt output under {style:?}, got: {:?}",
            decrypt_output.encryption_context
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt_bad_ec() {
    for style in CallStyle::ALL {
        let kms_keyring = kms_keyring().await;
        let plaintext = b"hello esdk";
        let encryption_context = std::collections::HashMap::from([(
            "aws-crypto-stuff".to_string(),
            "junk".to_string(),
        )]);
        let encrypt_input =
            EncryptInput::with_legacy_keyring(plaintext, encryption_context, kms_keyring);
        let encrypt_output = run_encrypt(style, &encrypt_input).await;

        //= spec/client-apis/encrypt.md#encryption-context
        //= type=test
        //# If the input encryption context contains any entries with a key beginning with `aws-crypto-`,
        //# the encryption operation MUST fail.
        let err = encrypt_output.expect_err("encrypt must fail for aws-crypto- EC key");
        assert_eq!(
            err.kind,
            ErrorKind::ValidationError,
            "expected ValidationError for reserved 'aws-crypto-' prefix under {style:?}, got: {:?} ({})",
            err.kind, err.message
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bad_encrypt_input() {
    for style in CallStyle::ALL {
        let kms_keyring = kms_keyring().await;
        let plaintext = b"hello esdk";
        let ec = EncryptionContext::new();
        let mut encrypt_input = EncryptInput::with_legacy_keyring(plaintext, ec, kms_keyring);
        encrypt_input.source = None;
        let encrypt_output = run_encrypt(style, &encrypt_input).await;

        //= spec/client-apis/encrypt.md#input
        //= type=test
        //# The Encrypt operation MUST validate that exactly one keyring or CMM was provided by the caller.
        //
        //= spec/client-apis/encrypt.md#input
        //= type=test
        //# If the caller does not provide exactly one of a keyring or CMM, the Encrypt operation MUST fail.
        let err = encrypt_output.expect_err("encrypt must fail when source = None");
        assert!(
            matches!(err.kind, ErrorKind::ValidationError),
            "expected ValidationError under {style:?}, got {:?}",
            err.kind
        );
    }
}

// Sweeps frame_length from 4 (multi-frame) up to plaintext.len() (single
// full frame), round-tripping at each length to confirm encrypt+decrypt
// agree across both frame-count regimes — under both CallStyle paths.
#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt_varied_frame_lengths() {
    for style in CallStyle::ALL {
        let kms_keyring = kms_keyring().await;
        let plaintext = "0123456789abcdef".as_bytes();

        let ec = EncryptionContext::new();
        let mut encrypt_input =
            EncryptInput::with_legacy_keyring(plaintext, ec.clone(), kms_keyring.clone());
        // Sweep frame_length from 4 (multi-frame) up to plaintext.len() (single full frame),
        // round-tripping at each length to confirm encrypt+decrypt agree across both regimes.
        for i in 4..=plaintext.len() {
            encrypt_input.frame_length.0 =
                std::num::NonZeroU32::new(u32::try_from(i).expect("frame length fits u32"))
                    .unwrap();
            let encrypt_output = run_encrypt(style, &encrypt_input)
                .await
                .unwrap_or_else(|e| panic!("encrypt failed under {style:?} (frame_len={i}): {e:?}"));
            let esdk_ciphertext = encrypt_output.ciphertext;
            let decrypt_input = DecryptInput::with_legacy_keyring(
                &esdk_ciphertext,
                ec.clone(),
                kms_keyring.clone(),
            );
            let decrypt_output = run_decrypt(style, &decrypt_input)
                .await
                .unwrap_or_else(|e| panic!("decrypt failed under {style:?} (frame_len={i}): {e:?}"));
            assert_eq!(
                decrypt_output.plaintext, plaintext,
                "round-trip mismatch under {style:?} at frame_len={i}"
            );
        }
    }
}

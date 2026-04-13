// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for decrypt.md#encrypted-message-format — Base64 detection

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use fixtures::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_rejects_base64_encoded_input() {
    // Construct input starting with 0x41 ('A') — the first byte of Base64-encoded
    // version 0x01 type 0x80 (per the spec table: "01 80" → "AY..." → "41 59...")
    //= specification/client-apis/decrypt.md#encrypted-message-format
    //= type=test
    //# To make diagnosing this mistake easier, implementations SHOULD detect the first two bytes of the Base64 encoding of any supported message [versions](../data-format/message-header.md#version-1)
    //# and [types](../data-format/message-header.md#type)
    //# and fail with a more specific error message.
    let fake_b64_input: Vec<u8> = {
        let mut v = vec![0x41, 0x59]; // 'A', 'Y'
        v.extend_from_slice(&[0u8; 100]); // padding
        v
    };

    let keyring = test_keyring().await;
    let dec_input =
        DecryptInput::with_legacy_keyring(&fake_b64_input, EncryptionContext::new(), keyring);
    let err = decrypt(&dec_input).await.unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("Base64"),
        "Error message should mention Base64, got: {msg}"
    );
}

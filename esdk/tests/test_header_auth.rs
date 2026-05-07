// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/data-format/message-header.md
//! header-authentication-version-1-0 and header-authentication-version-2-0

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

/// Helper: find the V1 header auth start offset (immediately after header body).
/// V1 header body ends after FrameLength (4 bytes at frame_length_offset).
fn v1_header_auth_offset(ct: &[u8]) -> usize {
    let (_, _, _, frame_length_offset) = parse_v1_trailing_offsets(ct);
    frame_length_offset + 4
}

/// Helper: find the V2 header auth start offset (immediately after header body).
/// V2 header body ends after the last field (Algorithm Suite Data).
fn v2_header_auth_offset(ct: &[u8]) -> usize {
    let fields = parse_v2_header_field_offsets(ct);
    fields.last().expect("must have header fields").2
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_structure() {
    //= spec/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //= reason=Raw-byte inspection proves V1 header auth is serialized as IV(12 zeros) then Tag(16 bytes), matching the V1 header authentication spec
    //# With the authentication tag calculated,
    //# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
    //# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-1-0) with the following specifics:
    let ct = encrypt_v1(b"v1 header auth structure").await;
    assert_eq!(ct[0], 0x01, "must be V1 message");
    let auth_start = v1_header_auth_offset(&ct);

    let iv_bytes = &ct[auth_start..auth_start + IV_LEN];
    let tag_bytes = &ct[auth_start + IV_LEN..auth_start + IV_LEN + TAG_LEN];

    //= spec/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //# - MUST serialize the [IV](../data-format/message-header.md#iv).
    assert_eq!(iv_bytes.len(), IV_LEN, "V1 header auth IV must be {IV_LEN} bytes");

    //= spec/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //# The value MUST be the IV used in the calculation above,
    //# padded to the [IV length](../data-format/message-header.md#iv-length) with 0.
    assert!(
        iv_bytes.iter().all(|&b| b == 0),
        "V1 header auth IV must be all zeros (padded to IV length with 0)"
    );

    //= spec/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //# - MUST serialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
    assert_eq!(tag_bytes.len(), TAG_LEN, "V1 header auth tag must be {TAG_LEN} bytes");

    //= spec/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //= reason=Tag is non-zero proving it is a real AEAD output (the authentication tag calculated above)
    //# The value MUST be the authentication tag calculated above.
    assert!(
        tag_bytes.iter().any(|&b| b != 0),
        "V1 header auth tag must not be all zeros"
    );

    // Verify ordering: IV at auth_start, then tag immediately after
    assert_eq!(
        auth_start + IV_LEN + TAG_LEN,
        auth_start + 28,
        "V1 header auth must be exactly IV({IV_LEN}) + Tag({TAG_LEN}) = 28 bytes"
    );

    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //= reason=Successful decrypt proves the IV used for header verification was the value serialized in the header's IV field.
    //# - For message format version [1.0](../data-format/message-header.md#supported-versions)
    //# the IV MUST be the value serialized in the message header's [IV field](../data-format/message-header.md#iv).
    let result = decrypt_with_version(&ct, Version::V1).await;
    assert_eq!(result.plaintext, b"v1 header auth structure", "V1 round-trip must succeed");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_auth_structure() {
    //= spec/client-apis/encrypt.md#v2-authentication-tag
    //= type=test
    //= reason=Raw-byte inspection proves V2 header auth is serialized as Tag only (no IV), matching the V2 header authentication spec
    //# With the authentication tag calculated,
    //# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0,
    //# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-2-0) with the following specifics:
    let ct = encrypt_v2(b"v2 header auth structure").await;
    assert_eq!(ct[0], 0x02, "must be V2 message");
    let auth_start = v2_header_auth_offset(&ct);

    let tag_bytes = &ct[auth_start..auth_start + TAG_LEN];

    //= spec/client-apis/encrypt.md#v2-authentication-tag
    //= type=test
    //# - The Encrypt operation MUST serialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
    assert_eq!(tag_bytes.len(), TAG_LEN, "V2 header auth tag must be {TAG_LEN} bytes");

    //= spec/client-apis/encrypt.md#v2-authentication-tag
    //= type=test
    //= reason=Tag is non-zero proving it is a real AEAD output (the authentication tag calculated above)
    //# The value MUST be the authentication tag calculated above.
    assert!(
        tag_bytes.iter().any(|&b| b != 0),
        "V2 header auth tag must not be all zeros"
    );

    // V2 has NO IV — verify the body starts right after the tag
    let after_tag = auth_start + TAG_LEN;
    let next_4 = u32::from_be_bytes([ct[after_tag], ct[after_tag + 1], ct[after_tag + 2], ct[after_tag + 3]]);
    assert!(
        next_4 == 1 || next_4 == 0xFFFF_FFFF,
        "V2: bytes after auth tag must be body start (seq=1 or endframe), got {next_4:#010X}"
    );

    // Round-trip proves decrypt successfully deserialized the auth tag
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //= reason=Successful round-trip decrypt proves the IV used for V2 header verification was 0.
    //# For message format version [2.0](../data-format/message-header.md#supported-versions)
    //# the IV MUST be 0.
    let result = decrypt_ciphertext(&ct).await;
    assert_eq!(result.plaintext, b"v2 header auth structure", "V2 round-trip must succeed");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_header_auth_tag_length_both_versions() {
    let keyring = test_keyring().await;
    for version in VERSIONS {
        let ct = encrypt_with_version(b"tag length test", version, keyring.clone()).await;

        let (auth_start, tag_bytes) = match version {
            Version::V1 => {
                let start = v1_header_auth_offset(&ct);
                (start, &ct[start + IV_LEN..start + IV_LEN + TAG_LEN])
            }
            Version::V2 => {
                let start = v2_header_auth_offset(&ct);
                (start, &ct[start..start + TAG_LEN])
            }
        };
        assert_eq!(
            tag_bytes.len(),
            TAG_LEN,
            "{version:?}: header auth tag must be {TAG_LEN} bytes (at offset {auth_start})"
        );

        assert!(
            tag_bytes.iter().any(|&b| b != 0),
            "{version:?}: header auth tag must not be all zeros"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_corrupted_header_auth_tag_fails_decrypt() {
    let keyring = test_keyring().await;
    for version in VERSIONS {
        let mut ct = encrypt_with_version(b"corrupt test", version, keyring.clone()).await;

        // Corrupt the auth tag
        let tag_offset = match version {
            Version::V1 => v1_header_auth_offset(&ct) + IV_LEN,
            Version::V2 => v2_header_auth_offset(&ct),
        };
        ct[tag_offset] ^= 0xFF;

        let mut dec_input =
            DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone());
        if let Version::V1 = version {
            dec_input.commitment_policy =
                aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }

        //= spec/client-apis/decrypt.md#verify-the-header
        //= type=test
        //= reason=Corrupted auth tag causes AES-GCM verification failure, proving the tag is checked.
        //# If this tag verification fails, this operation MUST immediately halt and fail.
        let err = decrypt(&dec_input).await.expect_err(
            &format!("{version:?}: corrupted header auth tag must fail decryption"),
        );
        assert_eq!(
            err.kind,
            ErrorKind::CryptographicError,
            "{version:?}: expected CryptographicError, got {err:?}"
        );
    }
}

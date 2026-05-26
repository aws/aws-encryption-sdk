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

    //= spec/data-format/message-header.md#header-authentication-version-1-0
    //= type=test
    //# The V1 Header Authentication MUST consist of, in order,
    //# IV,
    //# and Authentication Tag.
    let iv_bytes = &ct[auth_start..auth_start + IV_LEN];
    let tag_bytes = &ct[auth_start + IV_LEN..auth_start + IV_LEN + TAG_LEN];

    //= spec/data-format/message-header.md#iv
    //= type=test
    //= reason=Test grabs IV_LEN bytes and asserts they are all 0
    //# The length of the serialized IV MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    //
    //= spec/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //# - MUST serialize the [IV](../data-format/message-header.md#iv).
    // 
    //= spec/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //# The value MUST be the IV used in the calculation above,
    //# padded to the [IV length](../data-format/message-header.md#iv-length) with 0.
    //
    //= spec/client-apis/decrypt.md#verify-the-header
    //= type=test
    //# - For message format version [1.0](../data-format/message-header.md#supported-versions)
    //# the IV MUST be the value serialized in the message header's [IV field](../data-format/message-header.md#iv).
    assert!(
        iv_bytes.iter().all(|&b| b == 0),
        "V1 header auth IV must be all zeros (padded to IV length with 0)"
    );

    //= spec/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //# - MUST serialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
    //
    //= spec/data-format/message-header.md#authentication-tag
    //= type=test
    //# The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    assert!(
        tag_bytes.iter().any(|&b| b != 0),
        "V1 header auth tag must not be all zeros"
    );

    //= spec/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //= reason=Validated on decrypt. If this weren't the auth tag, decrypt would fail.
    //# The value MUST be the authentication tag calculated above.
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

    //= spec/data-format/message-header.md#header-authentication-version-2-0
    //= type=test
    //# The V2 Header Authentication MUST consist of the Authentication Tag only.
    let tag_bytes = &ct[auth_start..auth_start + TAG_LEN];

    //= spec/client-apis/encrypt.md#v2-authentication-tag
    //= type=test
    //# - The Encrypt operation MUST serialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
    //
    //= spec/data-format/message-header.md#authentication-tag
    //= type=test
    //# The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
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

    //= spec/client-apis/encrypt.md#v2-authentication-tag
    //= type=test
    //= reason=Validated on decrypt. If this weren't the auth tag, decrypt would fail.
    //# The value MUST be the authentication tag calculated above.
    let result = decrypt_ciphertext(&ct).await;
    assert_eq!(result.plaintext, b"v2 header auth structure", "V2 round-trip must succeed");
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

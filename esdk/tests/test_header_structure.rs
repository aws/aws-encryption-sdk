// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for spec/data-format/message-header.md

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use test_helpers::*;

//= spec/data-format/message-header.md#structure
//= type=test
//# The message header is a sequence of bytes that MUST be in big-endian format.
#[tokio::test(flavor = "multi_thread")]
async fn test_header_big_endian_format() {
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with_version(b"big-endian header test", version, keyring).await;
        // The Algorithm Suite ID is a multi-byte field at a known offset.
        // V1: offset 2..4 (after Version + Type), AlgAes256GcmIv12Tag16HkdfSha256 = 0x0178
        // V2: offset 1..3 (after Version), AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 = 0x0578
        let (offset, expected_hi, expected_lo) = match version {
            Version::V1 => (2, 0x01u8, 0x78u8),
            Version::V2 => (1, 0x05u8, 0x78u8),
        };
        assert_eq!(
            ct[offset], expected_hi,
            "{version:?}: Algorithm Suite ID high byte must be big-endian"
        );
        assert_eq!(
            ct[offset + 1], expected_lo,
            "{version:?}: Algorithm Suite ID low byte must be big-endian"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_header_serialization_order() {
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with_version(b"header serialization order test", version, keyring).await;
        // Find where header body ends and header auth begins.
        // V1: header auth = IV(iv_len) + auth_tag(16). IV length from header.
        // V2: header auth = auth_tag(16) only (IV is implicit zeros).
        let header_auth_offset = match version {
            Version::V1 => {
                let (_, _, _, fl_off) = parse_v1_trailing_offsets(&ct);
                fl_off + 4 // body ends after frame length
            }
            Version::V2 => {
                let fields = parse_v2_header_field_offsets(&ct);
                let (_, _, end) = fields.last().expect("no fields parsed");
                *end // body ends after Algorithm Suite Data
            }
        };
        let auth_tag_len: usize = 16;
        let header_auth_size = match version {
            Version::V1 => {
                let iv_len = ct[parse_v1_trailing_offsets(&ct).2] as usize;
                iv_len + auth_tag_len
            }
            Version::V2 => auth_tag_len,
        };
        // Verify header auth bytes exist immediately after header body
        assert!(
            ct.len() > header_auth_offset + header_auth_size,
            "{version:?}: header authentication must follow header body"
        );
        // For V2, verify the auth tag is non-zero (a valid GMAC tag)
        if let Version::V2 = version {
            let tag = &ct[header_auth_offset..header_auth_offset + auth_tag_len];
            assert!(
                tag.iter().any(|&b| b != 0),
                "{version:?}: header auth tag must be non-zero"
            );
        }
    }
}

//= spec/data-format/message-header.md#encrypted-data-key-count
//= type=test
//# This value MUST be greater than 0.
#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_key_count_greater_than_zero() {
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with_version(b"edk count test", version, keyring.clone()).await;
        let offset = edk_count_offset(&ct, version);
        let edk_count = u16::from_be_bytes([ct[offset], ct[offset + 1]]);
        assert!(
            edk_count > 0,
            "{version:?}: encrypted data key count must be greater than 0, got {edk_count}"
        );

        // Negative test: set EDK count to 0 and verify decryption fails
        let mut tampered = ct.clone();
        tampered[offset] = 0;
        tampered[offset + 1] = 0;
        let mut dec_input = DecryptInput::with_legacy_keyring(
            &tampered,
            EncryptionContext::new(),
            keyring.clone(),
        );
        if let Version::V1 = version {
            dec_input.commitment_policy =
                aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }
        assert!(
            decrypt(&dec_input).await.is_err(),
            "{version:?}: EDK count of 0 must be rejected"
        );
    }
}

//= spec/data-format/message-header.md#algorithm-suite-data
//= type=test
//# The length of the suite data field MUST be equal to the [Algorithm Suite Data Length](../framework/algorithm-suites.md#algorithm-suite-data-length) value
//# of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
#[tokio::test(flavor = "multi_thread")]
async fn test_suite_data_length_matches_algorithm_suite() {
    let ct = encrypt_default(b"suite data length test")
        .await
        .ciphertext;
    let fields = parse_v2_header_field_offsets(&ct);
    let (_, start, end) = fields
        .iter()
        .find(|(name, _, _)| *name == "Algorithm Suite Data")
        .expect("Algorithm Suite Data field not found");
    // Default V2 committing suite has 32-byte algorithm suite data (key commitment).
    assert_eq!(
        end - start,
        32,
        "algorithm suite data length must be 32 bytes for the default V2 committing suite"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_suite_data_interpreted_as_bytes() {
    let ct = encrypt_default(b"suite data bytes test")
        .await
        .ciphertext;
    let fields = parse_v2_header_field_offsets(&ct);
    let (_, start, end) = fields
        .iter()
        .find(|(name, _, _)| *name == "Algorithm Suite Data")
        .expect("Algorithm Suite Data field not found");
    let suite_data = &ct[*start..*end];
    assert_eq!(suite_data.len(), 32, "suite data must be exactly 32 bytes");
    // Key commitment is a cryptographic hash — it should not be valid UTF-8 text.
    // Verify it has byte values outside printable ASCII range.
    let non_ascii_count = suite_data
        .iter()
        .filter(|&&b| !(0x20..=0x7E).contains(&b))
        .count();
    assert!(
        non_ascii_count > 0,
        "suite data should contain non-ASCII bytes (raw key commitment, not a string)"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_frame_length_field_is_4_bytes() {
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with_version(b"frame length 4 bytes test", version, keyring).await;
        let frame_length_offset = match version {
            Version::V1 => {
                let (_, _, _, fl_off) = parse_v1_trailing_offsets(&ct);
                fl_off
            }
            Version::V2 => {
                let (_, _, fl_off) = parse_header_offsets(&ct);
                fl_off
            }
        };
        // Parse the 4-byte field and verify it contains the expected default frame length.
        // This proves the field is exactly 4 bytes: if it were shorter or longer,
        // the parsed value would not match 4096.
        let frame_length = u32::from_be_bytes([
            ct[frame_length_offset],
            ct[frame_length_offset + 1],
            ct[frame_length_offset + 2],
            ct[frame_length_offset + 3],
        ]);
        assert_eq!(
            frame_length, 4096,
            "{version:?}: frame length field must be 4 bytes encoding default value 4096"
        );
    }
}

//= spec/data-format/message-header.md#frame-length
//= type=test
//# When the [content type](#content-type) is non-framed, the value of this field MUST be 0.
#[tokio::test(flavor = "multi_thread")]
async fn test_nonframed_frame_length_must_be_zero() {
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct =
            encrypt_with_version(b"frame length test", version, keyring.clone()).await;

        let (content_type_offset, frame_length_offset) = match version {
            Version::V1 => {
                let (ct_off, _, _, fl_off) = parse_v1_trailing_offsets(&ct);
                (ct_off, fl_off)
            }
            Version::V2 => {
                let (_, ct_off, fl_off) = parse_header_offsets(&ct);
                (ct_off, fl_off)
            }
        };

        // Set content type to NonFramed (0x01) and frame length to a non-zero value
        let mut tampered = ct.clone();
        tampered[content_type_offset] = 0x01;
        tampered[frame_length_offset] = 0x00;
        tampered[frame_length_offset + 1] = 0x00;
        tampered[frame_length_offset + 2] = 0x10;
        tampered[frame_length_offset + 3] = 0x00;

        let mut dec_input = DecryptInput::with_legacy_keyring(
            &tampered,
            EncryptionContext::new(),
            keyring.clone(),
        );
        if let Version::V1 = version {
            dec_input.commitment_policy =
                aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }
        assert!(
            decrypt(&dec_input).await.is_err(),
            "{version:?}: nonframed content with non-zero frame length must be rejected"
        );
    }
}

// Negative test for the `read_header_body` sanity check that framed content
// must carry a non-zero Frame Length. The spec does not explicitly require this
// (it only requires Frame Length = 0 when content type is non-framed), but a
// framed frame length of 0 is a degenerate wire format that the implementation
// rejects during header deserialization, before header-auth verification.
#[tokio::test(flavor = "multi_thread")]
async fn test_framed_frame_length_must_be_positive() {
    for version in VERSIONS {
        let keyring = test_keyring().await;
        let ct = encrypt_with_version(b"framed zero frame length test", version, keyring.clone())
            .await;

        let (content_type_offset, frame_length_offset) = match version {
            Version::V1 => {
                let (ct_off, _, _, fl_off) = parse_v1_trailing_offsets(&ct);
                (ct_off, fl_off)
            }
            Version::V2 => {
                let (_, ct_off, fl_off) = parse_header_offsets(&ct);
                (ct_off, fl_off)
            }
        };

        // Sanity-check that the fixture is framed content (ContentType = 0x02).
        assert_eq!(
            ct[content_type_offset], 0x02,
            "{version:?}: fixture should be framed content"
        );

        // Tamper: set frame length to 0 while leaving content type framed.
        let mut tampered = ct.clone();
        tampered[frame_length_offset] = 0x00;
        tampered[frame_length_offset + 1] = 0x00;
        tampered[frame_length_offset + 2] = 0x00;
        tampered[frame_length_offset + 3] = 0x00;

        let mut dec_input =
            DecryptInput::with_legacy_keyring(&tampered, EncryptionContext::new(), keyring.clone());
        if let Version::V1 = version {
            dec_input.commitment_policy =
                aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }
        assert!(
            decrypt(&dec_input).await.is_err(),
            "{version:?}: framed content with zero frame length must be rejected"
        );
    }
}
